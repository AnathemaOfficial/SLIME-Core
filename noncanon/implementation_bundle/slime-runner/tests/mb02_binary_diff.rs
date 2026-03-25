#![cfg(all(feature = "private_validation", unix))]

// M-B02 — CoreSpec Binary Identity
//
// Proves: enterprise and agent CoreSpecs produce distinct binary artifacts.
// Compile-time specialization is real — not a shared binary + config.
//
// V1_INVARIANTS ref: §1.1 (Distinct CoreSpec identity)

use blake3::Hasher;
use std::{fs, path::Path};

fn hash_file(p: &Path) -> String {
    let data = fs::read(p).expect("read binary");
    let mut h = Hasher::new();
    h.update(&data);
    h.finalize().to_hex().to_string()
}

#[test]
#[ignore = "requires enterprise and agent CoreSpec artifacts built outside the public harness"]
fn mb02_binary_diff_enterprise_vs_agent() {
    let ent = Path::new("target/release/slime-core-enterprise");
    let agt = Path::new("target/release/slime-core-agent");

    assert!(
        ent.exists(),
        "M-B02: enterprise binary missing — build it first (scripts/build_corespec.sh enterprise)"
    );
    assert!(
        agt.exists(),
        "M-B02: agent binary missing — build it first (scripts/build_corespec.sh agent)"
    );

    let h_enterprise = hash_file(ent);
    let h_agent = hash_file(agt);

    assert_ne!(
        h_enterprise, h_agent,
        "M-B02: enterprise and agent must be distinct artifacts — hashes must differ"
    );
}
