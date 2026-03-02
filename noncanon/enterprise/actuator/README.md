# SLIME Actuator (Phase 4.2)

This is an **external** actuator-side decoder/logger for SLIME.

## Invariants respected
- SLIME remains unchanged.
- Egress ABI is fixed: **exactly 32 bytes, little-endian**:
  - `u64 domain_id` (LE)
  - `u64 magnitude` (LE)
  - `u128 actuation_token` (LE)
- No reason codes.
- No feedback channel.
- Observability is actuator-side only (append-only log).

## Paths (fixed)
- Socket (server/owner): `/run/slime/egress.sock`
- Log file (append-only): `enterprise/actuator/logs/events.log`

## Build + Run
From repo root:

```bash
cd enterprise/actuator
cargo build --release
sudo ./target/release/slime-actuator
You should see:

listening on /run/slime/egress.sock

logging to enterprise/actuator/logs/events.log

Log format

One line per egress event:

<unix_ms> domain=<u64> magnitude=<u64> token=0x<32-hex>

Example:
1739999999999 domain=1 magnitude=100 token=0x0000...abcd

Notes

In production/systemd, socket ownership and permissions should be handled by unit config.

This program sets socket perms to 0660 best-effort, but does not attempt chown.


---

## 4) Build / run quick test
```bash
cd enterprise/actuator
cargo build --release
sudo ./target/release/slime-actuator
