use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::UnixListener;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

const SOCKET_PATH: &str = "/run/slime/egress.sock";
const LOG_PATH: &str = "/data/repos/SLIME/enterprise/actuator/logs/events.log";

fn now_unix_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0)
}

fn parse_frame_le(frame: &[u8; 32]) -> (u64, u64, u128) {
    let mut a = [0u8; 8];
    let mut b = [0u8; 8];
    let mut c = [0u8; 16];

    a.copy_from_slice(&frame[0..8]);
    b.copy_from_slice(&frame[8..16]);
    c.copy_from_slice(&frame[16..32]);

    let domain_id = u64::from_le_bytes(a);
    let magnitude = u64::from_le_bytes(b);
    let token = u128::from_le_bytes(c);

    (domain_id, magnitude, token)
}

fn ensure_parent_dirs() -> std::io::Result<()> {
    if let Some(parent) = Path::new(LOG_PATH).parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

fn open_log_append() -> std::io::Result<std::fs::File> {
    ensure_parent_dirs()?;
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_PATH)
}

fn main() -> std::io::Result<()> {
    // Actuator owns the socket. Remove stale socket file if present.
    let sock_path = Path::new(SOCKET_PATH);
    if sock_path.exists() {
        // Only unlink if it's a socket file path we control.
        let _ = fs::remove_file(sock_path);
    }

    // Ensure /run/slime exists (systemd usually does this via RuntimeDirectory=).
    if let Some(parent) = sock_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let listener = UnixListener::bind(SOCKET_PATH)?;

    // Best-effort: set socket perms to 0660 (ownership is handled by systemd in prod).
    let _ = fs::set_permissions(SOCKET_PATH, fs::Permissions::from_mode(0o660));

    eprintln!("[actuator] listening on {}", SOCKET_PATH);
    eprintln!("[actuator] logging to {}", LOG_PATH);

    loop {
        let (mut stream, _addr) = match listener.accept() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("[actuator] accept error: {}", e);
                continue;
            }
        };

        eprintln!("[actuator] client connected");

        loop {
            let mut frame = [0u8; 32];
            match stream.read_exact(&mut frame) {
                Ok(()) => {
                    let (domain_id, magnitude, token) = parse_frame_le(&frame);
                    let ts_ms = now_unix_ms();

                    // Append-only, one line per event, no semantics beyond the ABI fields.
                    let mut f = match open_log_append() {
                        Ok(x) => x,
                        Err(e) => {
                            eprintln!("[actuator] log open error: {}", e);
                            continue;
                        }
                    };

                    // Format: unix_ms domain_id magnitude token_hex
                    // token_hex is 32 hex chars (u128).
                    let line = format!(
                        "{} domain={} magnitude={} token=0x{:032x}\n",
                        ts_ms, domain_id, magnitude, token
                    );

                    if let Err(e) = f.write_all(line.as_bytes()) {
                        eprintln!("[actuator] log write error: {}", e);
                    }
                }
                Err(e) => {
                    // EOF or broken pipe => client disconnected; return to accept loop.
                    eprintln!("[actuator] client disconnected: {}", e);
                    break;
                }
            }
        }
    }
}
