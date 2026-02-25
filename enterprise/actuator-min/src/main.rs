use std::fs;
use std::io::{Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::UnixListener;
use std::path::Path;

const SOCK_PATH: &str = "/run/slime/egress.sock";
const RUN_DIR: &str = "/run/slime";
const EVENT_LOG: &str = "/var/log/slime-actuator/events.log";

fn main() -> std::io::Result<()> {
    // Ensure /run/slime exists
    fs::create_dir_all(RUN_DIR)?;
    fs::set_permissions(RUN_DIR, fs::Permissions::from_mode(0o755))?;

    // Remove stale socket
    if Path::new(SOCK_PATH).exists() {
        let _ = fs::remove_file(SOCK_PATH);
    }

    // Bind socket (actuator owns it)
    let listener = UnixListener::bind(SOCK_PATH)?;
    fs::set_permissions(SOCK_PATH, fs::Permissions::from_mode(0o660))?;

    // Ensure log dir exists (best-effort)
    if let Some(parent) = Path::new(EVENT_LOG).parent() {
        let _ = fs::create_dir_all(parent);
        let _ = fs::set_permissions(parent, fs::Permissions::from_mode(0o750));
    }

    eprintln!("actuator-min: listening on {SOCK_PATH}");

    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                let mut buf = [0u8; 32];
                if let Err(e) = s.read_exact(&mut buf) {
                    eprintln!("actuator-min: read_exact failed: {e}");
                    continue;
                }

                // Hex encode 32 bytes
                let mut hex = String::with_capacity(64);
                for b in buf {
                    hex.push_str(&format!("{:02x}", b));
                }

                // Log line (no feedback to SLIME)
                let line = format!("{hex}\n");
                eprint!("actuator-min event: {}", line);

                if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(EVENT_LOG) {
                    let _ = f.write_all(line.as_bytes());
                }
            }
            Err(e) => eprintln!("actuator-min: accept error: {e}"),
        }
    }

    Ok(())
}
