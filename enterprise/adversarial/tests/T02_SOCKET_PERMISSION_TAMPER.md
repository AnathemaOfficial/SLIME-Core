# T02 — Socket Permission Tamper

## Purpose

Attempt to modify the Unix socket permissions to allow broader access.
Verify that SLIME + actuator ownership model resists unintended execution.

## Setup

Enterprise Appliance v0.1 installed and running.

## Steps

1) Confirm current state

```bash
systemctl is-active actuator.service slime.service
ls -l /run/slime/egress.sock
```

2) Attempt to modify socket permissions

```bash
sudo chmod 777 /run/slime/egress.sock
ls -l /run/slime/egress.sock
```

3) Restart actuator

```bash
sudo systemctl restart actuator.service
ls -l /run/slime/egress.sock
```

## Expected

- Even if manually changed, the socket should be recreated with:
  `srw-rw----` (0660)
- Owner must remain:
  `actuator:slime-actuator`
- SLIME must continue functioning normally.

## Proof artifacts

Capture:
- permission before tamper
- permission after tamper
- permission after restart
