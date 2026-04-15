# Known Limitations

**Status:** Living document
**Updated:** 2026-04-15 (post-tordeur audit)

---

## Platform-Specific

| ID | Limitation | Impact | Mitigation |
|---|---|---|---|
| P-01 | `verify_no_symlinks` is a no-op on Windows | Workspace containment not enforced | Deploy on Unix (Linux) |
| P-02 | `shell_exec` requires `#[cfg(unix)]` | Shell actuator unavailable on Windows | Deploy on Unix |
| P-03 | `setpgid` process group isolation is Unix-only | Shell commands may leak to parent group on non-Unix | Deploy on Unix |

## Network Security

| ID | Limitation | Impact | Mitigation |
|---|---|---|---|
| N-01 | IPv6 ULA (`fc00::/7`) not classified as private | HTTP actuator SSRF to internal IPv6 services | Network-level allowlist or IPv4-only actuator |
| N-02 | IPv6 link-local (`fe80::/10`) not classified as private | Same as N-01 | Same as N-01 |
| N-03 | NAT64 prefix (`64:ff9b::/96`) not blocked | Potential cloud metadata access via NAT64 | Same as N-01 |

## Specification

| ID | Limitation | Impact | Mitigation |
|---|---|---|---|
| S-01 | slime-runner is single-threaded | Trivially DoS-able (one slow connection blocks all) | Use SAFA for production (async, concurrent) |
| S-02 | slime-runner uses `stub_ab` (not real AB-S) | No formal law resolution, just capacity check | Use enterprise wiring for real AB-S |
| S-03 | Actuation token in runner is non-cryptographic | Token forgery possible on egress socket | Use HMAC or signed tokens in production |

## Formal Model

| ID | Limitation | Impact | Mitigation |
|---|---|---|---|
| F-01 | Three-valued internal state (authorized+dropped) | Audit distinction between IMPOSSIBLE and actuation failure | Log actuation failures as distinct audit events |
| F-02 | Monotonicity is per-agent, not global | One agent's exhaustion doesn't constrain others | Documented in ARCHITECTURE_SECURITY_MODEL.md §4 |

---

**END — KNOWN LIMITATIONS**
