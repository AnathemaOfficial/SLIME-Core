#![cfg(all(feature = "private_validation", unix))]

// M-B03 — Reproducible Build
//
// Proves: three consecutive clean builds from the same source produce identical hashes.
// Artifact determinism is a requirement of the trust model.
//
// Prerequisites for true bit-for-bit reproducibility (document in SECURITY_INVARIANTS_TESTED.md):
//   - Toolchain pinned via rust-toolchain.toml
//   - Cargo.lock versioned
//   - SOURCE_DATE_EPOCH set in build_corespec.sh
//   - Consistent linker (lld vs bfd can differ)
//
// V1_INVARIANTS ref: §5 (Trust Model — reproducible builds)

use std::process::Command;

fn run(cmd: &mut Command) -> (i32, String) {
    let out = cmd.output().expect("run command");
    let code = out.status.code().unwrap_or(-1);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    (code, format!("{}\n{}", stdout, stderr))
}

fn extract_hash(output: &str) -> String {
    output
        .lines()
        .find(|l| !l.trim().is_empty())
        .unwrap_or("")
        .trim()
        .to_string()
}

#[test]
#[ignore = "requires bash scripts and enterprise CoreSpec build pipeline"]
fn mb03_build_repro_three_times_same_hash_enterprise() {
    // Log toolchain for CI traceability
    if let Ok(out) = Command::new("rustc").arg("--version").output() {
        println!(
            "M-B03 toolchain: {}",
            String::from_utf8_lossy(&out.stdout).trim()
        );
    }
    if let Ok(out) = Command::new("rustup").args(["show", "active-toolchain"]).output() {
        println!(
            "M-B03 active toolchain: {}",
            String::from_utf8_lossy(&out.stdout).trim()
        );
    }

    // Build 1
    let (c1, o1) = run(
        Command::new("bash")
            .arg("scripts/build_corespec.sh")
            .arg("enterprise"),
    );
    assert_eq!(c1, 0, "M-B03: build 1 failed:\n{}", o1);

    let (c_h1, h1o) = run(
        Command::new("bash")
            .arg("scripts/hash_artifact.sh")
            .arg("target/release/slime-core-enterprise"),
    );
    assert_eq!(c_h1, 0, "M-B03: hash 1 failed:\n{}", h1o);
    let h1 = extract_hash(&h1o);
    println!("M-B03 build 1 hash: {}", h1);

    // Clean + Build 2
    let _ = run(Command::new("cargo").arg("clean"));
    let (c2, o2) = run(
        Command::new("bash")
            .arg("scripts/build_corespec.sh")
            .arg("enterprise"),
    );
    assert_eq!(c2, 0, "M-B03: build 2 failed:\n{}", o2);

    let (c_h2, h2o) = run(
        Command::new("bash")
            .arg("scripts/hash_artifact.sh")
            .arg("target/release/slime-core-enterprise"),
    );
    assert_eq!(c_h2, 0, "M-B03: hash 2 failed:\n{}", h2o);
    let h2 = extract_hash(&h2o);
    println!("M-B03 build 2 hash: {}", h2);

    // Clean + Build 3
    let _ = run(Command::new("cargo").arg("clean"));
    let (c3, o3) = run(
        Command::new("bash")
            .arg("scripts/build_corespec.sh")
            .arg("enterprise"),
    );
    assert_eq!(c3, 0, "M-B03: build 3 failed:\n{}", o3);

    let (c_h3, h3o) = run(
        Command::new("bash")
            .arg("scripts/hash_artifact.sh")
            .arg("target/release/slime-core-enterprise"),
    );
    assert_eq!(c_h3, 0, "M-B03: hash 3 failed:\n{}", h3o);
    let h3 = extract_hash(&h3o);
    println!("M-B03 build 3 hash: {}", h3);

    assert_eq!(
        h1, h2,
        "M-B03: build 1 and 2 differ: {} vs {}",
        h1, h2
    );
    assert_eq!(
        h2, h3,
        "M-B03: build 2 and 3 differ: {} vs {}",
        h2, h3
    );
}
