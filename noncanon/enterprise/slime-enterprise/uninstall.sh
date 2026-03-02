#!/usr/bin/env bash
set -euo pipefail

BIN_DIR="/usr/local/bin"
UNIT_DIR="/etc/systemd/system"

echo "[1/6] stop/disable"
sudo systemctl stop slime.service actuator.service 2>/dev/null || true
sudo systemctl disable slime.service actuator.service 2>/dev/null || true

echo "[2/6] remove units"
sudo rm -f "$UNIT_DIR/slime.service" "$UNIT_DIR/actuator.service"
sudo systemctl daemon-reload

echo "[3/6] remove binaries"
sudo rm -f "$BIN_DIR/slime-runner" "$BIN_DIR/actuator-min"

echo "[4/6] runtime cleanup (best-effort)"
sudo rm -f /run/slime/egress.sock 2>/dev/null || true
sudo rmdir /run/slime 2>/dev/null || true

echo "[5/6] users/groups (optional, commented)"
# sudo userdel slime 2>/dev/null || true
# sudo userdel actuator 2>/dev/null || true
# sudo groupdel slime-actuator 2>/dev/null || true

echo "[6/6] done"
