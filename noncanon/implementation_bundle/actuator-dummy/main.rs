// ============================================================================
// NONCANON / LEGACY IMPLEMENTATION BUNDLE
//
// This file is NOT the canonical SLIME v0 runner nor the enterprise actuator.
//
// Canonical references:
// - Runner (hardened): noncanon/implementation_bundle/slime-runner/
// - Enterprise actuator (minimal): enterprise/actuator-min/
//
// This legacy bundle is kept for historical / illustration purposes only.
// Do NOT use it as a security reference.
// ============================================================================
//
// NOTE:
// - The canonical SLIME v0 state is sealed via tags (e.g. slime-v0-hardening-phase2b).
// - Known issues in legacy code are addressed in the canonical runner/actuator.
// ============================================================================
// =============================================================================
// SLIME v0 — Runner (Linux std)
// Canon: CANON.md / ARCHITECTURE.md / INGRESS_API_SPEC.md / EGRESS_SOCKET_SPEC.md
// Constraints: zero external deps, non-semantic logs, fail-closed egress
// =============================================================================
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::os::unix::net::UnixStream;
use std::thread;
use std::time::Duration;

// -----------------------------------------------------------------------------
// 1. CORE TYPES (mirrored from slime-core no_std)
// -----------------------------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ActionRequest {
    pub domain_id: u64,
    pub magnitude: u64,
    pub payload_ptr: *const u8,
    pub payload_len: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AuthorizedEffect {
    pub domain_id: u64,      // 8 bytes
    pub magnitude: u64,      // 8 bytes
    pub actuation_token: u128, // 16 bytes → TOTAL 32 bytes
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AB_S_Verdict {
    pub is_ok: u8,       // 0 = false, 1 = true (stable ABI)
    pub pad: [u8; 7],    // padding for 8-byte alignment
    pub payload: [u8; 32],
}

// -----------------------------------------------------------------------------
// 2. FNV-1a 64-bit (inline, zero deps)
// -----------------------------------------------------------------------------
fn fnv1a_64(s: &str) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in s.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

// -----------------------------------------------------------------------------
// 3. INGRESS — HTTP/1.1 parser strict (schema-locked exact)
// -----------------------------------------------------------------------------
mod ingress {
    use super::*;

    pub fn start() {
        let listener = TcpListener::bind("127.0.0.1:8080").expect("ingress bind failed");
        eprintln!("{{\"event\":\"ingress_started\"}}");

        for stream in listener.incoming() {
            if let Ok(stream) = stream {
                handle_request(stream);
            }
        }
    }

    fn handle_request(mut stream: TcpStream) {
        // Buffer size 16384 bytes (headers + body)
        let mut buf = [0u8; 16384];
        let n = match stream.read(&mut buf) {
            Ok(n) if n > 0 => n,
            _ => return,
        };

        // Step 1: extract Content-Length
        let content_length = match parse_content_length(&buf[..n]) {
            Some(cl) if cl <= 16384 - 512 => cl,
            _ => {
                let _ = stream.write_all(b"HTTP/1.1 400 Bad Request\r\nContent-Length: 64\r\n\r\n{\"error\":\"invalid_request\",\"message\":\"missing Content-Length\"}");
                return;
            }
        };

        // Step 2: extract body
        let body_start = match memmem(&buf[..n], b"\r\n\r\n") {
            Some(pos) if pos + 4 + content_length <= n => pos + 4,
            _ => {
                let _ = stream.write_all(b"HTTP/1.1 400 Bad Request\r\nContent-Length: 53\r\n\r\n{\"error\":\"invalid_request\",\"message\":\"missing body\"}");
                return;
            }
        };
        let body = &buf[body_start..body_start + content_length];

        // Step 3: parse exact schema
        let (domain, magnitude, payload_b64) = match parse_exact_schema(body) {
            Ok(parsed) => parsed,
            Err(_) => {
                let _ = stream.write_all(b"HTTP/1.1 400 Bad Request\r\nContent-Length: 56\r\n\r\n{\"error\":\"invalid_request\",\"message\":\"invalid schema\"}");
                return;
            }
        };

        let domain_id = fnv1a_64(domain) & 0xFFFF_FFFF;

        // Base64 decode (cap 64KB)
        let payload = match base64_decode(payload_b64) {
            Some(p) if p.len() <= 65536 => p,
            _ => {
                let _ = stream.write_all(b"HTTP/1.1 413 Payload Too Large\r\nContent-Length: 70\r\n\r\n{\"error\":\"payload_too_large\",\"message\":\"exceeds 65536 bytes\"}");
                return;
            }
        };

        let action = ActionRequest {
            domain_id,
            magnitude,
            payload_ptr: payload.as_ptr(),
            payload_len: payload.len(),
        };

        // --- AB-S CORE (SEALED) ---
        let verdict = unsafe { ab_s_phase_7_resolve(action) };

        if verdict.is_ok != 0 {
            // Safety: ABI guarantees payload contains valid AuthorizedEffect if is_ok == 1
            let effect = unsafe { core::ptr::read(verdict.payload.as_ptr() as *const AuthorizedEffect) };
            eprintln!("{{\"event\":\"authorized\"}}");
            egress::apply(effect);
            let _ = stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 27\r\n\r\n{\"status\":\"AUTHORIZED\"}");
        } else {
            eprintln!("{{\"event\":\"impossible\"}}");
            let _ = stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 27\r\n\r\n{\"status\":\"IMPOSSIBLE\"}");
        }
    }

    fn parse_content_length(req: &[u8]) -> Option<usize> {
        let req_str = std::str::from_utf8(req).ok()?;
        for line in req_str.lines() {
            if line.starts_with("Content-Length:") {
                return line["Content-Length:".len()..].trim().parse().ok();
            }
        }
        None
    }

    fn memmem(haystack: &[u8], needle: &[u8]) -> Option<usize> {
        haystack.windows(needle.len()).position(|window| window == needle)
    }

    // Parse exact schema with separator validators
    fn parse_exact_schema(body: &[u8]) -> Result<(&str, u64, &str), ()> {
        let body_str = std::str::from_utf8(body).map_err(|_| ())?;
        
        // State 0: {"domain":"
        if !body_str.starts_with(r#"{"domain":""#) { return Err(()); }
        let mut pos = r#"{"domain":""#.len();
        
        // State 1: domain (ascii until ")
        let domain_end = body_str[pos..].find('"').ok_or(())?;
        let domain = &body_str[pos..pos + domain_end];
        if !domain.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') { return Err(()); }
        pos += domain_end;
        if body_str.as_bytes().get(pos) != Some(&b'"') { return Err(()); }
        pos += 1;
        if body_str.as_bytes().get(pos) != Some(&b',') { return Err(()); }
        pos += 1;
        
        // State 2: "magnitude":
        if !body_str[pos..].starts_with(r#""magnitude":"#) { return Err(()); }
        pos += r#""magnitude":"#.len();
        
        // State 3: magnitude (u64)
        let magnitude_end = body_str[pos..].find(',').ok_or(())?;
        let magnitude_str = &body_str[pos..pos + magnitude_end];
        let magnitude = magnitude_str.parse::<u64>().map_err(|_| ())?;
        pos += magnitude_end;
        if body_str.as_bytes().get(pos) != Some(&b',') { return Err(()); }
        pos += 1;
        
        // State 4: "payload":"
        if !body_str[pos..].starts_with(r#""payload":""#) { return Err(()); }
        pos += r#""payload":""#.len();
        
        // State 5: payload (base64 until ")
        let payload_end = body_str[pos..].find('"').ok_or(())?;
        let payload_b64 = &body_str[pos..pos + payload_end];
        pos += payload_end;
        if body_str.as_bytes().get(pos) != Some(&b'"') { return Err(()); }
        pos += 1;
        if body_str.as_bytes().get(pos) != Some(&b'}') { return Err(()); }
        pos += 1;
        
        // State 6: exact end
        if pos != body_str.len() { return Err(()); }
        
        Ok((domain, magnitude, payload_b64))
    }

    // Base64 RFC 4648 strict
    fn base64_decode(input: &str) -> Option<Vec<u8>> {
        const TABLE: [u8; 64] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        
        // Validation: length must be multiple of 4
        if input.len() % 4 != 0 {
            return None;
        }
        
        // Empty string is valid base64 for zero bytes
        if input.is_empty() {
            return Some(Vec::new());
        }
        
        // Count padding
        let padding = input.chars().rev().take_while(|c| *c == '=').count();
        if padding > 2 {
            return None; // RFC 4648: max 2 padding chars
        }
        
        let decoded_len = (input.len() / 4) * 3 - padding;
        if decoded_len > 65536 {
            return None; // Cap 64KB
        }
        
        let mut output = Vec::with_capacity(decoded_len);
        let mut buf: u32 = 0;
        let mut buf_bits = 0;
        
        for (i, c) in input.chars().enumerate() {
            if c == '=' {
                // Padding only at end
                if i < input.len() - padding {
                    return None;
                }
                continue;
            }
            
            let val = TABLE.iter().position(|&b| b == c as u8)?;
            buf = (buf << 6) | (val as u32);
            buf_bits += 6;
            
            if buf_bits >= 8 {
                output.push((buf >> (buf_bits - 8)) as u8);
                buf_bits -= 8;
            }
        }
        
        if output.len() != decoded_len {
            return None;
        }
        
        Some(output)
    }
}

extern "C" {
    fn ab_s_phase_7_resolve(req: ActionRequest) -> AB_S_Verdict;
}

// -----------------------------------------------------------------------------
// 4. EGRESS — UnixStream CLIENT (fail-closed)
// -----------------------------------------------------------------------------
mod egress {
    use super::*;

    const SOCKET_PATH: &str = "/run/slime/egress.sock";

    pub fn apply(effect: AuthorizedEffect) {
        static INIT: std::sync::Once = std::sync::Once::new();
        static mut STREAM: Option<UnixStream> = None;

        INIT.call_once(|| {
            match UnixStream::connect(SOCKET_PATH) {
                Ok(stream) => unsafe { STREAM = Some(stream) },
                Err(_) => {
                    eprintln!("{{\"event\":\"egress_init_failed\"}}");
                    std::process::exit(1);
                }
            }
        });

        // Explicit LE 32 bytes write
        let mut bytes = [0u8; 32];
        bytes[0..8].copy_from_slice(&effect.domain_id.to_le_bytes());
        bytes[8..16].copy_from_slice(&effect.magnitude.to_le_bytes());
        bytes[16..32].copy_from_slice(&effect.actuation_token.to_le_bytes());

        if let Some(stream) = unsafe { STREAM.as_mut() } {
            if stream.write_all(&bytes).is_err() {
                eprintln!("{{\"event\":\"egress_write_failed\"}}");
                // SILENT DROP — no retry
            }
        }
    }
}

// -----------------------------------------------------------------------------
// 5. DASHBOARD — HTTP read-only (HTML + health separated)
// -----------------------------------------------------------------------------
mod dashboard {
    use super::*;

    const DASHBOARD_HTML: &str = include_str!("../resources/dashboard.html");

    pub fn start() {
        let listener = TcpListener::bind("127.0.0.1:8081").expect("dashboard bind failed");
        eprintln!("{{\"event\":\"dashboard_started\"}}");

        for stream in listener.incoming() {
            if let Ok(stream) = stream {
                handle_request(stream);
            }
        }
    }

    fn handle_request(mut stream: TcpStream) {
        let mut buf = [0u8; 1024];
        let n = match stream.read(&mut buf) {
            Ok(n) if n > 0 => n,
            _ => return,
        };

        let req_line = match std::str::from_utf8(&buf[..n]) {
            Ok(s) => s.lines().next().unwrap_or(""),
            Err(_) => "",
        };

        if req_line.starts_with("GET / HTTP/1.1") {
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
                DASHBOARD_HTML.len(),
                DASHBOARD_HTML
            );
            let _ = stream.write_all(response.as_bytes());
        } else if req_line.starts_with("GET /health HTTP/1.1") {
            let _ = stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 15\r\n\r\n{\"status\":\"ok\"}");
        } else {
            let _ = stream.write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n");
        }
    }
}

// -----------------------------------------------------------------------------
// 6. ENTRY POINT — monotone pipeline
// -----------------------------------------------------------------------------
fn main() {
    thread::spawn(|| ingress::start());
    thread::spawn(|| dashboard::start());

    loop {
        thread::sleep(Duration::from_secs(86400));
    }
}
