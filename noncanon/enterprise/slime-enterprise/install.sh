#!/usr/bin/env bash
set -euo pipefail

BIN_DIR="/usr/local/bin"
UNIT_DIR="/etc/systemd/system"

echo "[1/9] Users/groups"
sudo groupadd -f slime-actuator
id -u slime >/dev/null 2>&1 || sudo useradd -r -s /usr/sbin/nologin -g slime-actuator slime
id -u actuator >/dev/null 2>&1 || sudo useradd -r -s /usr/sbin/nologin -g slime-actuator actuator

echo "[2/9] Install binaries"
sudo install -m 0755 bin/slime-runner "$BIN_DIR/slime-runner"
sudo install -m 0755 bin/actuator-min "$BIN_DIR/actuator-min"

echo "[3/9] Install systemd units"
sudo install -m 0644 systemd/actuator.service "$UNIT_DIR/actuator.service"
sudo install -m 0644 systemd/slime.service "$UNIT_DIR/slime.service"

echo "[4/9] daemon-reload"
sudo systemctl daemon-reload

echo "[5/9] enable"
sudo systemctl enable actuator.service
sudo systemctl enable slime.service

echo "[6/9] start actuator"
sudo systemctl restart actuator.service

echo "[7/9] start slime"
sudo systemctl restart slime.service

echo "[8/9] status (proof)"
sudo systemctl --no-pager status actuator.service || true
sudo systemctl --no-pager status slime.service || true

echo "[9/9] sanity: socket"
ls -l /run/slime/egress.sock 2>/dev/null || true

echo "OK: installed"
