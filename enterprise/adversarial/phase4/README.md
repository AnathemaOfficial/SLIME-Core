# SLIME — Phase 4.4 Adversarial Suite (Ingress)

Goal: attempt to force non-canonical behavior.
Observation is external only:
- actuator log: /data/repos/SLIME/enterprise/actuator/logs/events.log
- dashboard: http://127.0.0.1:8081/

Invariant: invalid requests must not produce egress effects.

Run order:
1) T01_invalid_json.sh
2) T02_missing_fields.sh
3) T03_wrong_types.sh
4) T04_oversize_body.sh
5) T05_flood.sh
