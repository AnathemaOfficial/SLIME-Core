use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::os::unix::net::UnixStream;
use std::process;
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

//
// -------------------- Hardening Constants (Phase 2) --------------------
//

const MAX_HEADER_BYTES: usize = 8 * 1024; // 8KB
const MAX_BODY_BYTES: usize = 64 * 1024; // 64KB
const READ_TIMEOUT_SECS: u64 = 2;

// -------------------- Types --------------------
//

#[derive(Clone, Copy)]
struct AuthorizedEffect {
    domain_id: u64,
    magnitude: u64,
    actuation_token: u128,
}

struct ActionRequest {
    domain_id: u64,
    magnitude: u64,
}

//
// -------------------- Ingress Read (Hardened) --------------------
//

fn read_http_body_hardened(stream: &mut TcpStream) -> Option<Vec<u8>> {
    // Slow-loris defense: bounded read time
    let _ = stream.set_read_timeout(Some(Duration::from_secs(READ_TIMEOUT_SECS)));

    let mut buf = Vec::<u8>::new();
    let mut tmp = [0u8; 1024];

    // 1) Read headers up to MAX_HEADER_BYTES, stop at \r\n\r\n
    let mut header_end = None;
    while buf.len() < MAX_HEADER_BYTES {
        let n = stream.read(&mut tmp).ok()?;
        if n == 0 {
            return None;
        }
        buf.extend_from_slice(&tmp[..n]);

        // Detect end of headers
        if let Some(pos) = buf.windows(4).position(|w| w == b"\r\n\r\n") {
            header_end = Some(pos + 4);
            break;
        }
    }

    // Header too big or never terminated
    let header_end = header_end?;
    if header_end >= MAX_HEADER_BYTES {
        return None;
    }

    // 2) Parse Content-Length (required)
    let header_text = std::str::from_utf8(&buf[..header_end]).ok()?;
    let content_length = header_text
        .lines()
        .find(|l| l.to_ascii_lowercase().starts_with("content-length:"))
        .and_then(|l| l.splitn(2, ':').nth(1))
        .and_then(|v| v.trim().parse::<usize>().ok())?;

    if content_length > MAX_BODY_BYTES {
        return None;
    }

    // 3) Read exactly content_length bytes of body
    let mut body = Vec::with_capacity(content_length);
    let already_read = &buf[header_end..];
    let preloaded = already_read.len().min(content_length);
    body.extend_from_slice(&already_read[..preloaded]);
    while body.len() < content_length {
        let remaining = content_length - body.len();
        let chunk = remaining.min(tmp.len());
        let n = stream.read(&mut tmp[..chunk]).ok()?;
        if n == 0 {
            return None;
        }
        body.extend_from_slice(&tmp[..n]);

    }

    Some(body)
}

//
// -------------------- Request Parse (Dummy) --------------------
//

fn parse_request(body: &[u8]) -> Option<ActionRequest> {
    let text = std::str::from_utf8(body).ok()?;

    // extremely dumb parse (dummy)
    // expects something like: {"domain":"test","magnitude":10,...}
    let domain = if let Some(p) = text.find("\"domain\"") {
        let s = &text[p..];
        let q1 = s.find('"')?;
        let s2 = &s[q1 + 1..];
        let q2 = s2.find('"')?;
        let s3 = &s2[q2 + 1..];
        let q3 = s3.find('"')?;
        let s4 = &s3[q3 + 1..];
        let q4 = s4.find('"')?;
        &s4[..q4]
    } else {
        "test"
    };

    let magnitude = if let Some(p) = text.find("\"magnitude\":") {
        let s = &text[p + 12..];
        s.trim_start()
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<u64>()
            .unwrap_or(0)
    } else {
        return None;
    };

    let domain_id = fnv1a_64(domain.as_bytes());

    Some(ActionRequest {
        domain_id,
        magnitude,
    })
}

fn fnv1a_64(data: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in data {
        hash ^= *b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

//
// -------------------- Egress (CANON v0) --------------------
//

mod egress {
    use super::*;

    // Canonical, non-configurable path (SLIME v0)
    const SOCKET_PATH: &str = "/run/slime/egress.sock";

    // Store a single connected stream for the process lifetime.
    // - Boot-time: connect must succeed or SLIME terminates (fail-closed hard)
    // - Runtime: write failures are dropped silently (best-effort)
    static STREAM: OnceLock<Mutex<UnixStream>> = OnceLock::new();

    pub fn init_fail_closed() {
        let s = UnixStream::connect(SOCKET_PATH).unwrap_or_else(|_| {
            // No logs, no retries: if SLIME cannot actuate, it must not run.
            process::exit(1);
        });

        let _ = STREAM.set(Mutex::new(s));
    }

    pub fn apply(effect: AuthorizedEffect) {
        let stream = STREAM.get();
        if stream.is_none() {
            // Defensive: init is a boot prerequisite. If not initialized, fail-closed.
            process::exit(1);
        }
        let mut guard = stream.unwrap().lock().unwrap();

        // Serialize exact 32 bytes (LE): u64 + u64 + u128
        let mut buf = [0u8; 32];
        buf[0..8].copy_from_slice(&effect.domain_id.to_le_bytes());
        buf[8..16].copy_from_slice(&effect.magnitude.to_le_bytes());
        buf[16..32].copy_from_slice(&effect.actuation_token.to_le_bytes());

        // Best-effort write. Any error is a silent drop (no feedback channel).
        let _ = guard.write_all(&buf);
    }
}

//
// -------------------- Ingress (Dummy HTTP, Hardened + RL gate) --------------------
//

mod ingress {
    use super::*;
    const AUTHORIZED_STATUS: &[u8] = b"{\"status\":\"AUTHORIZED\"}";
    const IMPOSSIBLE_STATUS: &[u8] = b"{\"status\":\"IMPOSSIBLE\"}";

    fn write_status_response(stream: &mut TcpStream, status: &[u8]) {
        let header = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n", status.len());
        let _ = stream.write_all(header.as_bytes());
        let _ = stream.write_all(status);
    }

    pub fn start() {
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

        for conn in listener.incoming() {
            if let Ok(stream) = conn {
                handle(stream);
            }
        }
    }

    fn handle(mut stream: TcpStream) {
        // Hardened read: headers capped, body capped, timeout enforced, Content-Length required
        let body = match crate::read_http_body_hardened(&mut stream) {
            Some(b) => b,
            None => {
                write_status_response(&mut stream, IMPOSSIBLE_STATUS);
                return;
            }
        };

        // parse
        let req = match crate::parse_request(&body) {
            Some(r) => r,
            None => {
                write_status_response(&mut stream, IMPOSSIBLE_STATUS);
                return;
            }
        };

        // decision: always AUTHORIZED for dummy
        let effect = AuthorizedEffect {
            domain_id: req.domain_id,
            magnitude: req.magnitude,
            actuation_token: 0xABCD_EF01_2345_6789_ABCD_EF01_2345_6789u128,
        };

        crate::egress::apply(effect);

        write_status_response(&mut stream, AUTHORIZED_STATUS);
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::io::{Read, Write};
        use std::net::{TcpListener, TcpStream};

        #[test]
        fn invalid_request_returns_impossible() {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let addr = listener.local_addr().unwrap();

            let t = std::thread::spawn(move || {
                let (stream, _) = listener.accept().unwrap();
                handle(stream);
            });

            let mut client = TcpStream::connect(addr).unwrap();
            // Invalid by schema: required fields like "magnitude" are missing.
            let _ = client.write_all(b"POST / HTTP/1.1\r\nContent-Length: 2\r\n\r\n{}");
            let _ = client.shutdown(std::net::Shutdown::Write);

            let mut resp = Vec::new();
            let _ = client.read_to_end(&mut resp);
            t.join().unwrap();

            let text = String::from_utf8(resp).unwrap();
            assert!(text.contains("{\"status\":\"IMPOSSIBLE\"}"));
        }
    }
}

//
// -------------------- Main --------------------
//

fn main() {
    // Canon prerequisite: actuator socket must exist and be connectable at boot.
    crate::egress::init_fail_closed();

    ingress::start();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::net::{TcpListener, TcpStream};

    #[test]
    fn read_http_body_hardened_accepts_preloaded_body_bytes() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        let t = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let body = read_http_body_hardened(&mut stream).unwrap();
            assert_eq!(body, br#"{"domain":"t","magnitude":1}"#);
        });

        let mut client = TcpStream::connect(addr).unwrap();
        let req =
            b"POST / HTTP/1.1\r\nContent-Length: 28\r\n\r\n{\"domain\":\"t\",\"magnitude\":1}";
        let _ = client.write_all(req);
        let _ = client.shutdown(std::net::Shutdown::Write);

        t.join().unwrap();
    }
}
