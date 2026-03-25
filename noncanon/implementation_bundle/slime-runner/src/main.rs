use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
#[cfg(unix)]
use std::os::unix::net::UnixStream;
use std::process;
#[cfg(unix)]
use std::sync::{Mutex, OnceLock};
#[cfg(all(not(unix), feature = "integration_demo"))]
use std::sync::OnceLock;
use std::time::Duration;

// ---------------------------------------------------------------------------
// Conditional resolver: real AB-S engine or stub
// ---------------------------------------------------------------------------

#[cfg(feature = "real_ab")]
compile_error!(
    "Feature `real_ab` is reserved for private enterprise wiring and is not available in the public slime-runner checkout"
);

#[cfg(not(feature = "stub_ab"))]
compile_error!("The public slime-runner checkout requires the default `stub_ab` resolver");

// Stub resolver (default for open-source builds)
#[cfg(feature = "stub_ab")]
mod stub_resolver {
    //! Reference-only action resolver — NOT the real law engine.
    //!
    //! Simple capacity check: known domain + magnitude ≤ capacity → AUTHORIZED.
    //! The real engine (Anathema-Breaker) uses formal typestate topology
    //! (RZ → EP → IZ) and is not included in the open-source distribution.

    #[derive(Clone, Copy)]
    pub struct Domain(pub u16);

    #[derive(Clone, Copy)]
    pub struct Magnitude(pub u32);

    #[derive(Clone, Copy)]
    pub struct Capacity(pub u32);

    #[derive(Clone, Copy)]
    #[allow(dead_code)]
    pub struct Progression(pub u32);

    #[allow(dead_code)]
    pub struct Budget {
        pub capacity: Capacity,
        pub progression: Progression,
    }

    /// Stub resolver: magnitude ≤ capacity → AUTHORIZED, else IMPOSSIBLE.
    /// Budget is decremented on success (fresh per request in V1).
    pub fn resolve(domain: Domain, magnitude: Magnitude, budget: &mut Budget) -> Option<u32> {
        let _ = domain; // domain validity already checked by resolve_domain()
        if magnitude.0 > budget.capacity.0 {
            return None;
        }
        budget.capacity = Capacity(budget.capacity.0.saturating_sub(magnitude.0));
        Some(magnitude.0)
    }
}

#[cfg(feature = "stub_ab")]
use stub_resolver::{Budget, Capacity, Domain, Magnitude, Progression};

//
// -------------------- Hardening Constants (Phase 2) --------------------
//

const MAX_HEADER_BYTES: usize = 8 * 1024;
const MAX_BODY_BYTES: usize = 64 * 1024;
const READ_TIMEOUT_SECS: u64 = 2;

//
// -------------------- CoreSpec Constants (Phase 6.3) --------------------
// Compile-time law. No runtime configuration. No env vars.
// Change these constants = produce a different binary = different CoreSpec.
//

/// Domain mapping table — sealed at compile time.
/// Unknown domains are structurally impossible.
const DOMAIN_TABLE: &[(&str, u16)] = &[
    ("test", 0),
    ("payment", 1),
    ("deploy", 2),
    ("db_prod", 3),
];

/// Budget constants — fresh budget per request (V1 statelessness).
/// No state persists between requests.
const CORESPEC_CAPACITY: u32 = 10_000;
const CORESPEC_PROGRESSION: u32 = 1;

// -------------------- Types --------------------

#[derive(Clone, Copy)]
#[cfg_attr(not(unix), allow(dead_code))]
struct AuthorizedEffect {
    domain_id: u64,
    magnitude: u64,
    actuation_token: u128,
}

struct ActionRequest {
    domain: [u8; 64],
    domain_len: usize,
    magnitude: u64,
}

//
// -------------------- Domain Resolution (Phase 6.3) --------------------
//

fn resolve_domain(name: &str) -> Option<Domain> {
    for &(key, id) in DOMAIN_TABLE {
        if key == name {
            return Some(Domain(id));
        }
    }
    None
}

fn domain_to_egress_id(d: Domain) -> u64 {
    d.0 as u64
}

//
// -------------------- Law Resolution Wrapper --------------------
//

/// Resolve an action through the public stub law engine.
/// Returns the applied magnitude on AUTHORIZED, or None on IMPOSSIBLE.
fn resolve_law(domain: Domain, magnitude: Magnitude, budget: &mut Budget) -> Option<u32> {
    stub_resolver::resolve(domain, magnitude, budget)
}

//
// -------------------- Ingress Read (Hardened) --------------------
//

fn read_http_body_hardened(stream: &mut TcpStream) -> Option<Vec<u8>> {
    let _ = stream.set_read_timeout(Some(Duration::from_secs(READ_TIMEOUT_SECS)));

    let mut buf = Vec::<u8>::new();
    let mut tmp = [0u8; 1024];

    let mut header_end = None;
    while buf.len() < MAX_HEADER_BYTES {
        let n = stream.read(&mut tmp).ok()?;
        if n == 0 {
            return None;
        }
        buf.extend_from_slice(&tmp[..n]);
        if let Some(pos) = buf.windows(4).position(|w| w == b"\r\n\r\n") {
            header_end = Some(pos + 4);
            break;
        }
    }

    let header_end = header_end?;
    if header_end >= MAX_HEADER_BYTES {
        return None;
    }

    let header_text = std::str::from_utf8(&buf[..header_end]).ok()?;
    let content_length = header_text
        .lines()
        .find(|l| l.to_ascii_lowercase().starts_with("content-length:"))
        .and_then(|l| l.split_once(':').map(|x| x.1))
        .and_then(|v| v.trim().parse::<usize>().ok())?;

    if content_length > MAX_BODY_BYTES {
        return None;
    }

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
// -------------------- Request Parse --------------------
//

fn parse_request(body: &[u8]) -> Option<ActionRequest> {
    let text = std::str::from_utf8(body).ok()?;

    let domain_str = {
        let p = text.find("\"domain\"")?;
        let s = &text[p..];
        let q1 = s.find('"')?;
        let s2 = &s[q1 + 1..];
        let q2 = s2.find('"')?;
        let s3 = &s2[q2 + 1..];
        let q3 = s3.find('"')?;
        let s4 = &s3[q3 + 1..];
        let q4 = s4.find('"')?;
        &s4[..q4]
    };

    let magnitude = {
        let p = text.find("\"magnitude\":")?;
        let s = &text[p + 12..];
        s.trim_start()
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<u64>()
            .ok()?
    };

    let mut domain = [0u8; 64];
    let domain_len = domain_str.len().min(64);
    domain[..domain_len].copy_from_slice(&domain_str.as_bytes()[..domain_len]);

    Some(ActionRequest {
        domain,
        domain_len,
        magnitude,
    })
}

//
// -------------------- Egress (CANON v0) --------------------
//

#[cfg(unix)]
mod egress {
    use super::*;

    const SOCKET_PATH: &str = "/run/slime/egress.sock";
    static STREAM: OnceLock<Mutex<UnixStream>> = OnceLock::new();

    pub fn init_fail_closed() {
        let s = UnixStream::connect(SOCKET_PATH).unwrap_or_else(|_| {
            process::exit(1);
        });
        let _ = STREAM.set(Mutex::new(s));
    }

    pub fn apply(effect: AuthorizedEffect) {
        let stream = STREAM.get();
        if stream.is_none() {
            process::exit(1);
        }
        let mut guard = stream.unwrap().lock().unwrap();

        let mut buf = [0u8; 32];
        buf[0..8].copy_from_slice(&effect.domain_id.to_le_bytes());
        buf[8..16].copy_from_slice(&effect.magnitude.to_le_bytes());
        buf[16..32].copy_from_slice(&effect.actuation_token.to_le_bytes());

        if guard.write_all(&buf).is_err() {
            let s = UnixStream::connect(SOCKET_PATH).unwrap_or_else(|_| process::exit(1));
            *guard = s;
            if guard.write_all(&buf).is_err() {
                process::exit(1);
            }
        }
    }
}

#[cfg(not(unix))]
mod egress {
    use super::*;

    #[cfg(feature = "integration_demo")]
    static DEMO_EGRESS_FILE: OnceLock<String> = OnceLock::new();

    #[cfg(feature = "integration_demo")]
    fn encode_effect(effect: AuthorizedEffect) -> [u8; 32] {
        let mut buf = [0u8; 32];
        buf[0..8].copy_from_slice(&effect.domain_id.to_le_bytes());
        buf[8..16].copy_from_slice(&effect.magnitude.to_le_bytes());
        buf[16..32].copy_from_slice(&effect.actuation_token.to_le_bytes());
        buf
    }

    #[cfg(feature = "integration_demo")]
    pub fn init_fail_closed() {
        let path = std::env::var("SLIME_DEMO_EGRESS_FILE").unwrap_or_else(|_| {
            eprintln!("integration_demo requires SLIME_DEMO_EGRESS_FILE");
            process::exit(1);
        });

        std::fs::File::create(&path).unwrap_or_else(|_| {
            eprintln!("integration_demo could not create demo egress file");
            process::exit(1);
        });

        let _ = DEMO_EGRESS_FILE.set(path);
    }

    #[cfg(feature = "integration_demo")]
    pub fn apply(effect: AuthorizedEffect) {
        let path = DEMO_EGRESS_FILE.get().unwrap_or_else(|| {
            process::exit(1);
        });

        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)
            .unwrap_or_else(|_| process::exit(1));

        let buf = encode_effect(effect);
        if file.write_all(&buf).is_err() {
            process::exit(1);
        }
    }

    #[cfg(not(feature = "integration_demo"))]
    pub fn init_fail_closed() {
        eprintln!("slime-runner requires a Unix target for egress socket support");
        process::exit(1);
    }

    #[cfg(not(feature = "integration_demo"))]
    pub fn apply(_effect: AuthorizedEffect) {
        process::exit(1);
    }
}

//
// -------------------- Ingress --------------------
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
        for stream in listener.incoming().flatten() {
            handle(stream);
        }
    }

    fn handle(mut stream: TcpStream) {
        let body = match crate::read_http_body_hardened(&mut stream) {
            Some(b) => b,
            None => {
                write_status_response(&mut stream, IMPOSSIBLE_STATUS);
                return;
            }
        };

        let req = match crate::parse_request(&body) {
            Some(r) => r,
            None => {
                write_status_response(&mut stream, IMPOSSIBLE_STATUS);
                return;
            }
        };

        // -- Law Resolution -----------------------------------------------
        //
        // 1. Resolve domain via sealed compile-time table
        let domain_str = std::str::from_utf8(&req.domain[..req.domain_len]).unwrap_or("");
        let domain = match crate::resolve_domain(domain_str) {
            Some(d) => d,
            None => {
                write_status_response(&mut stream, IMPOSSIBLE_STATUS);
                return;
            }
        };

        // 2. Validate magnitude fits u32 (AB-S uses Magnitude(u32))
        if req.magnitude == 0 || req.magnitude > u32::MAX as u64 {
            write_status_response(&mut stream, IMPOSSIBLE_STATUS);
            return;
        }
        let magnitude = Magnitude(req.magnitude as u32);

        // 3. Fresh budget per request (V1 statelessness)
        let mut budget = Budget {
            capacity: Capacity(CORESPEC_CAPACITY),
            progression: Progression(CORESPEC_PROGRESSION),
        };

        // 4. Resolve through selected law engine (real AB-S or stub)
        match crate::resolve_law(domain, magnitude, &mut budget) {
            Some(applied_mag) => {
                let authorized = AuthorizedEffect {
                    domain_id: crate::domain_to_egress_id(domain),
                    magnitude: applied_mag as u64,
                    actuation_token: 0u128,
                };
                crate::egress::apply(authorized);
                write_status_response(&mut stream, AUTHORIZED_STATUS);
            }
            None => {
                write_status_response(&mut stream, IMPOSSIBLE_STATUS);
            }
        }
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
            let _ = client.write_all(b"POST / HTTP/1.1\r\nContent-Length: 2\r\n\r\n{}");
            let _ = client.shutdown(std::net::Shutdown::Write);

            let mut resp = Vec::new();
            let _ = client.read_to_end(&mut resp);
            t.join().unwrap();

            let text = String::from_utf8(resp).unwrap();
            assert!(text.contains("{\"status\":\"IMPOSSIBLE\"}"));
        }

        #[test]
        fn zero_magnitude_returns_impossible() {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let addr = listener.local_addr().unwrap();

            let t = std::thread::spawn(move || {
                let (stream, _) = listener.accept().unwrap();
                handle(stream);
            });

            let mut client = TcpStream::connect(addr).unwrap();
            let body = br#"{"domain":"test","magnitude":0}"#;
            let req = format!(
                "POST / HTTP/1.1\r\nContent-Length: {}\r\n\r\n{}",
                body.len(),
                std::str::from_utf8(body).unwrap()
            );
            let _ = client.write_all(req.as_bytes());
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
    crate::egress::init_fail_closed();
    ingress::start();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::net::TcpStream;

    #[test]
    fn read_http_body_hardened_accepts_preloaded_body_bytes() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
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

    #[test]
    fn resolve_domain_known() {
        assert!(resolve_domain("test").is_some());
        assert!(resolve_domain("payment").is_some());
        assert!(resolve_domain("deploy").is_some());
        assert!(resolve_domain("db_prod").is_some());
    }

    #[test]
    fn resolve_domain_unknown() {
        assert!(resolve_domain("unknown").is_none());
        assert!(resolve_domain("").is_none());
        assert!(resolve_domain("PAYMENT").is_none());
    }

    #[test]
    fn read_http_body_hardened_rejects_oversized_content_length() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        let t = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let body = read_http_body_hardened(&mut stream);
            assert!(body.is_none());
        });

        let mut client = TcpStream::connect(addr).unwrap();
        let req = b"POST / HTTP/1.1\r\nContent-Length: 70000\r\n\r\n";
        let _ = client.write_all(req);
        let _ = client.shutdown(std::net::Shutdown::Write);

        t.join().unwrap();
    }

    #[test]
    fn read_http_body_hardened_rejects_missing_content_length() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        let t = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let body = read_http_body_hardened(&mut stream);
            assert!(body.is_none());
        });

        let mut client = TcpStream::connect(addr).unwrap();
        let req = b"POST / HTTP/1.1\r\nHost: localhost\r\n\r\n{}";
        let _ = client.write_all(req);
        let _ = client.shutdown(std::net::Shutdown::Write);

        t.join().unwrap();
    }

    #[test]
    fn read_http_body_hardened_rejects_incomplete_body() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        let t = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let body = read_http_body_hardened(&mut stream);
            assert!(body.is_none());
        });

        let mut client = TcpStream::connect(addr).unwrap();
        let req = b"POST / HTTP/1.1\r\nContent-Length: 20\r\n\r\n{}";
        let _ = client.write_all(req);
        let _ = client.shutdown(std::net::Shutdown::Write);

        t.join().unwrap();
    }
}
