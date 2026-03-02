#!/usr/bin/env bash
set -euo pipefail

BODY='{"domain":"test","magnitude":10,"payload":""}'

echo "[demo] POST /action body=$BODY"
code=$(curl -sS -o /dev/null -w "%{http_code}" -X POST http://127.0.0.1:8080/action \
  -H 'Content-Type: application/json' \
  -d "$BODY")

echo "[demo] http_code=$code"
echo "[demo] last_event:"
tail -n 1 /data/repos/SLIME/enterprise/actuator/logs/events.log || true
