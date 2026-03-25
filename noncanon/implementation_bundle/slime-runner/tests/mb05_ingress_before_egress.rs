#![cfg(all(feature = "private_validation", unix))]

// M-B05 — Boot Ordering Invariant (Structural)
//
// Proves: ingress does not bind/accept before egress validation succeeds.
// This is a structural test — it observes the consequence of correct boot ordering.
//
// Separated from T-E06 (Layer A) by Amendment A-02:
//   M-B05 = structural invariant (does ingress bind at all?)
//   T-E06 = runtime adversarial race (can a request sneak through during boot?)
//
// V1_INVARIANTS ref: §7.2 (Fail-Closed — ingress MUST NOT open unless
//                          all initialization completes successfully)

use std::net::{SocketAddr, TcpStream};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

fn try_connect(addr: SocketAddr, timeout: Duration) -> bool {
    let start = Instant::now();
    while start.elapsed() < timeout {
        if TcpStream::connect_timeout(&addr, Duration::from_millis(50)).is_ok() {
            return true;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    false
}

#[test]
#[ignore = "requires enterprise CoreSpec artifact and Unix runtime semantics"]
fn mb05_ingress_must_not_bind_when_egress_missing() {
    // Ensure egress socket is absent before spawning
    let _ = std::fs::remove_file("/run/slime/egress.sock");

    let mut child = Command::new("target/release/slime-core-enterprise")
        .env_clear()
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("M-B05: failed to spawn core binary");

    // Allow boot sequence to begin before probing ingress
    std::thread::sleep(Duration::from_millis(50));

    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let connected = try_connect(addr, Duration::from_millis(750));

    let status = child.try_wait().expect("M-B05: try_wait failed");
    let exited = status.is_some();

    // Ingress MUST NOT have accepted connections
    assert!(
        !connected,
        "M-B05: ingress bound and accepted connection even though egress was missing — invariant violated"
    );

    // Preferred outcome: exit(1) cleanly
    if exited {
        let code = status.unwrap().code().unwrap_or(-1);
        assert_eq!(
            code, 1,
            "M-B05: expected exit(1) on fail-closed boot, got exit({})",
            code
        );
    } else {
        // Ingress refused (no connection) but process still running — also acceptable
        // Clean up
        let _ = child.kill();
    }
}
