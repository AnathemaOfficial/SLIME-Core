# T03 — Egress Size Integrity

## Purpose

Attempt to observe or coerce SLIME into emitting anything other than exactly 32 bytes.

## Setup

Enterprise Appliance v0.1 installed.

Ensure actuator-min is running.

## Steps

1) Restart actuator in foreground logging mode (optional diagnostic)

```bash
sudo systemctl restart actuator.service
sudo journalctl -u actuator.service -f
```

2) Trigger a valid action (domain-specific trigger).

3) Observe actuator logs.

Each event must log exactly:
- 64 hex characters (32 bytes)

Example:
```
00112233aabbccdd...
```

4) Attempt to inject extra payload via client tampering (if applicable to environment).

## Expected

- Every authorized event = exactly 32 bytes.
- No 31-byte or 33-byte payload.
- No framing or metadata.
- No JSON or structured wrapper.

Any deviation = failure of invariant.

## Proof artifacts

Capture:
- actuator log output
- byte length confirmation (manual hex length count or script)
