#![cfg(all(feature = "private_validation", unix))]

// M-B04 — No Runtime Policy Load (Critical)
//
// Proves: CoreSpec binary does not open, mmap, or parse any policy/profile
// file at boot. Model B is real — policy is code, not a file.
//
// This test is CRITICAL. If M-B04 fails, Phase 1 cannot lock.
//
// Allowlist strategy (JARVIS decision):
//   System loader paths are allowlisted to avoid false positives from libc/linker.
//   Application paths (target/, Cargo.lock) are NOT allowlisted — the binary
//   must not open those during boot under any condition.
//
// V1_INVARIANTS ref: §2 (MUST NOT mmap/parse policy files), §4 (CoreSpec MUST NOT exist as runtime file)

use std::{fs, process::Command};

// Patterns that must NOT appear in file-related syscall lines.
// "config" and "dotenv" excluded: too generic, triggers false positives
// on system paths (/etc/ld.so.conf, /proc/self/exe, etc.)
const BAD_PATTERNS: &[&str] = &[
    "profile",
    "policy",
    "corespec",
    ".json",
    ".toml",
    ".yaml",
    ".yml",
];

// Conservative allowlist: known-safe system loader paths only.
// NOT included: target/, Cargo.lock, or any application path.
const SYSTEM_ALLOWLIST: &[&str] = &[
    "/proc/",
    "/sys/",
    "/dev/",
    "/usr/lib",
    "/usr/lib64",
    "/lib",
    "/lib64",
    "/etc/ld.so",
    "ld.so.cache",
];

#[test]
#[ignore = "requires strace and enterprise CoreSpec artifact"]
fn mb04_no_profile_open_strace_boot() {
    let out = Command::new("bash")
        .arg("scripts/run_strace_boot.sh")
        .arg("target/release/slime-core-enterprise")
        .arg("/tmp/slime_strace_boot.log")
        .output()
        .expect("M-B04: failed to run strace script");

    // strace may write multiple files with -ff: /tmp/slime_strace_boot.log.<pid>
    let mut logs: Vec<String> = vec![];
    for entry in fs::read_dir("/tmp").expect("M-B04: cannot read /tmp") {
        let entry = entry.expect("dir entry");
        let p = entry.path();
        if let Some(name) = p.file_name().and_then(|s| s.to_str()) {
            if name.starts_with("slime_strace_boot.log") {
                if let Ok(content) = fs::read_to_string(&p) {
                    logs.push(content);
                }
            }
        }
    }
    assert!(!logs.is_empty(), "M-B04: no strace logs found — check strace availability");

    let hay = logs.join("\n");

    // Match line by line to avoid false positives from unrelated syscall output.
    // Only inspect lines that contain a file-related syscall invocation.
    for line in hay.lines() {
        let lower = line.to_lowercase();
        let is_file_syscall = lower.contains("openat(")
            || lower.contains("open(\"")
            || lower.contains("mmap(");

        if !is_file_syscall {
            continue;
        }

        // Skip known-safe system loader lines
        if SYSTEM_ALLOWLIST.iter().any(|allowed| lower.contains(allowed)) {
            continue;
        }

        // Flag any remaining line that contains a forbidden pattern
        for pat in BAD_PATTERNS {
            if lower.contains(pat) {
                panic!(
                    "M-B04: syscall line contains forbidden pattern '{}' — possible runtime policy load:\n  {}",
                    pat, line
                );
            }
        }
    }

    assert!(
        !hay.is_empty(),
        "M-B04: strace log is empty — binary may not have started"
    );
}
