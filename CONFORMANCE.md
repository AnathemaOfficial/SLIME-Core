# SLIME v0 — Conformance Matrix

**Status:** Reference (non-canon)
**Purpose:** Clarify intentional divergences between canon specifications, the slime-runner harness, and the enterprise deployment.

---

## Authority Rule

> **Canon = specs/ exclusively.**
> Noncanon code is a harness or deployment tool.
> If runner or deploy diverges from specs/, the runner/deploy is wrong — not the spec.

---

## Conformance Table

| Aspect | Canon (specs/) | slime-runner (noncanon) | Enterprise Deploy (noncanon) |
|---|---|---|---|
| **Ingress: format errors** | 400/413/500 with `error` + `message` fields | Always returns HTTP 200 + `IMPOSSIBLE` (flattened) | Same as runner |
| **Ingress: impossibility** | HTTP 200 + `{"status":"IMPOSSIBLE"}` | HTTP 200 + `{"status":"IMPOSSIBLE"}` | Same |
| **Ingress: payload (base64)** | Required field, max 64KB decoded, passed to AB-S | Ignored (parser reads `domain` + `magnitude` only) | Same as runner |
| **AB-S Core** | Sealed, opaque, compile-time law, non-inspectable | Public checkout uses the `stub_ab` reference resolver. Private enterprise wiring may swap in the real AB-S engine, but that path is not shipped in this manifest. | Private deployment may embed the real AB-S engine |
| **Egress: ABI** | 32 bytes LE: u64 + u64 + u128 | 32 bytes LE: u64 + u64 + u128 | Same |
| **Egress: socket ownership** | Actuator owns socket (server/listener); SLIME connects as client | SLIME connects as client (fail-closed if absent) | `actuator.service` creates socket; `slime.service` requires it |
| **Egress: socket path** | `/run/slime/egress.sock` (hardcoded) | `/run/slime/egress.sock` | Same |
| **Egress: socket perms** | `0660`, owner `actuator`, group `slime-actuator` | Best-effort `0660` by actuator-min | Actuator creates socket; systemd `RuntimeDirectory` ensures `/run/slime` exists; permissions enforced by actuator + unit config |
| **Domain normalization** | `domain` string → `u64` domain_id (R-7: no truncation) | Static compile-time table: string → `Domain(u16)`. Unknown domains → IMPOSSIBLE. No hash. | Same as runner |
| **Saturation states** | SATURATED, then SEALED (terminal) | Not modeled (per-request budget prevents cross-request depletion) | Not modeled |
| **Backpressure** | Kernel buffer fills, writes block, no bypass | Same (inherited from OS) | Same |
| **Dashboard** | N/A (out of law scope) | Not implemented | Read-only on port 8081 if deployed (`noncanon/enterprise/dashboard`) |
| **Fail-closed boot** | If socket absent at startup, SLIME exits | Exits with code 1 | `ExecStartPre` polls for socket, fails after timeout |

---

## Intentional Divergences

The following divergences are **intentional** and expected in the noncanon harness:

1. **Flattened error handling** — The runner returns `IMPOSSIBLE` for both format errors and true impossibilities. Canon distinguishes these (4xx vs 200). This simplification is acceptable in a test harness but must not be treated as conformant behavior.

2. **No payload processing** — The runner ignores the `payload` field entirely. Canon requires base64 decoding and size validation before passing to AB-S.

3. **No saturation/sealed states** — The runner does not model cumulative capacity exhaustion across requests. Canon defines terminal SEALED state when the system can no longer authorize actions. The runner uses a fresh per-request Budget, so capacity accounting exists within a single request but no cross-request depletion occurs.

4. **Domain table vs hash** — Canon allows `u64` domain identifiers via stable hash or compile-time table (R-7: no truncation). The runner uses a static compile-time table mapping domain strings to `Domain(u16)`. This is a deliberate choice: table-based resolution is more auditable than hash-based. The mapping is sealed at compile time and unknown domains are structurally impossible.

5. **Magnitude zero rejection** — Canon specifies magnitude range `1..2^64-1`. The runner additionally rejects magnitudes exceeding `u32::MAX` because the stub resolver uses `Magnitude(u32)`. Canon allows the full `u64` range.

6. **Actuation token generation** — Canon states the `actuation_token` field carries authorization metadata and the actuator bridge must verify authenticity. The runner generates a non-zero token from a monotonic counter XOR'd with domain/magnitude. This is NOT cryptographic — a real deployment should use HMAC or similar.

7. **Egress write failure** — Canon says "No retry or recovery mechanism exists." As of v0.4.0, the runner complies: write failure exits the process immediately (fail-closed). Prior versions (≤ v0.3.x) silently reconnected and retried once — this was a spec violation.

8. **AUTHORIZED response missing effect_id** — Canon specifies `{"status":"AUTHORIZED","effect_id":"<UUID>"}`. The runner returns only `{"status":"AUTHORIZED"}` — no `effect_id`. This is an intentional simplification in the harness.

9. **Content-Type enforcement** — Canon does not mandate `Content-Type: application/json`. As of v0.4.0, the runner enforces it. Requests without `application/json` content type are rejected.

10. **integration_demo env var** — V1_INVARIANTS §2 states "MUST NOT read environment variables." The `integration_demo` feature reads `SLIME_DEMO_EGRESS_FILE` from the environment. This is acknowledged as a noncanon divergence for demo purposes only.

---

## Enterprise-Only Resolution Notes

The following item is resolved only in private enterprise wiring, not in the
public `slime-runner` checkout:

1. **Stub AB-S** — The public runner still uses `stub_ab` by default to
demonstrate interface form. Private enterprise wiring may delegate authorization
to the real Anathema-Breaker core, but that dependency path is intentionally not
present in the public manifest.

---

## Deployment Warning: Runner != Full Canon

The public `slime-runner` harness validates interface form with `stub_ab` and
does not implement the full canon specification.

**Remaining gaps:**

- Public checkout does not ship the private real AB-S dependency path
- Ignores payload entirely (no validation, no size check)
- Flattens all errors to IMPOSSIBLE (no distinction between format errors and true impossibilities)
- No cross-request saturation model (no SEALED terminal state)
- No FirePlank-Guard binary integrity verification

**To achieve full SLIME canon compliance:**

1. Provide private enterprise wiring for the real AB-S engine outside the public manifest
2. Implement full ingress validation (payload base64, size limits, HTTP status codes per canon)
3. Deploy FirePlank-Guard (ACTUATOR_TCB.md) for binary integrity verification
4. Verify conformance against `specs/` - not against the runner

---

## How to Read This Document

- If you are **auditing SLIME**, use `specs/` as the sole authority.
- If you are **testing the public harness**, expect the divergences listed above and the default `stub_ab` resolver.
- If you need MB01-MB05 or real AB-S wiring, that is private enterprise validation outside the default public path.
- If you are **deploying enterprise**, the systemd model in `noncanon/enterprise/` is the reference.
- If you are **evaluating security posture**, do not treat the public harness as proof of enterprise AB-S integration. Full conformance additionally requires payload validation, private engine wiring, and FirePlank-Guard integrity.

**No divergence listed here modifies the canon.**

---

**END — SLIME v0 CONFORMANCE MATRIX**
