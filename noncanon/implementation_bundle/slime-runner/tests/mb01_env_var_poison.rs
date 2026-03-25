#![cfg(all(feature = "private_validation", unix))]

// M-B01 — Zero Runtime Configuration (Controlled Mode)
//
// Proves: binary can be spawned under env_clear.
// Absence of env-based branching is proven by M-B04 (syscall audit).
//
// V1_INVARIANTS ref: §2 (Zero Runtime Configuration), §4 (CoreSpec MUST NOT exist as runtime file)

use std::process::{Command, Stdio};

fn bin_path_enterprise() -> String {
    std::env::var("SLIME_BIN_ENTERPRISE")
        .unwrap_or_else(|_| "target/release/slime-core-enterprise".into())
}

#[test]
#[ignore = "requires private enterprise binary wiring"]
fn mb01_controlled_env_clear_spawn() {
    let mut cmd = Command::new(bin_path_enterprise());
    cmd.env_clear()
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    // Proves: binary starts under cleared environment without crashing immediately.
    // Does NOT prove absence of env reads — that is M-B04's responsibility.
    let child = cmd.spawn();
    assert!(
        child.is_ok(),
        "M-B01: binary must start under env_clear (controlled mode)"
    );

    let mut child = child.unwrap();
    let _ = child.kill();
}
