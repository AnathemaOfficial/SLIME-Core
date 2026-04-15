# Full-Stack Conformance — Gate / Shield / AB / SLIME / Actuator

**Status:** Integration rules (non-canon)
**Purpose:** Define the cross-layer conformance contract between all SYF components.
**Authority:** Each component's `specs/` directory is the sole canon. This document governs integration only.

---

## 1) Roles (non-overlapping)

```
Client/Agent
    ↓ request
SYF-Gate ─────── admission verdict (ALLOW | DENY), fail-closed
    ↓
SYF-Shield ───── irreversibility coupling (EP) + non-regenerative capacity
    ↓
AB-S ─────────── composition: resolve_action → Ok(Effect) | Err(Impossibility)
    ↓
SLIME ────────── impossibility membrane: binary verdict, 32B egress ABI
    ↓
Actuator ─────── effect execution boundary (owns egress socket)
```

| Component | Role | Knows about |
|---|---|---|
| **SYF-Gate** | Admission / bounds / invariant check | Input structure, signal, limits |
| **SYF-Shield** | Capacity accounting / EP coupling | Capacity, progression, exhaustion |
| **AB-S** | Law composition (Gate × Shield) | Budget, domain, action topology |
| **SLIME** | Binary membrane (AUTHORIZED / IMPOSSIBLE) | Frame ABI, socket, fail-closed |
| **Actuator** | Effect execution | 32B frame decoding, idempotence |

---

## 2) Invariant Ownership Map

```
Invariant                        Enforced by      Mechanism
─────────────────────────────── ──────────────── ─────────────────────────
Fail-closed on ambiguity         Gate + SLIME     DENY / silence on error
Identity ≠ Capacity              Gate             Structural (no auth = allow)
Bounds non-configurable          Gate             Hard-coded constants
Non-regenerative capacity        Shield + AB      Private inners, crate-only mutation
EP = first partial irreversible  Shield           Typestate (Active → Sealed)
Token linearity (single-use)     Shield           Non-Copy/Non-Clone phantom marker
Budget non-pilotable             AB               Private fields, compile-fail tests
Binary verdict only              SLIME            No reason codes in egress
32B ABI fixed                    SLIME            Compile-time struct size
Socket ownership by actuator     SLIME + Actuator Actuator = server, SLIME = client
Monotonicity (action space ↓)    Shield + AB      Capacity only decreases, never increases
```

---

## 2b) Implementation Notes (added 2026-04-15, post-tordeur)

The following implementation patterns were established during the SAFA
adversarial audit cycle and are recorded here for conformance guidance:

**Per-agent workspace isolation:** In multi-agent deployments (SAFA P3),
each agent operates in an isolated subdirectory (`workspace_root/{agent_id}`).
Shell intents SHOULD use `{{agent_workspace}}` (resolved at runtime) rather
than `{{workspace_root}}` (global, shared). Intents using the global
workspace MUST emit a boot-time warning.

**TOCTOU re-validation on file operations:** After `create_dir_all()`,
implementations MUST re-canonicalize the created parent and re-verify it
remains under the effective workspace root. This closes the
intermediate-directory symlink-swap race between validation and creation.

**Platform scope:** Structural guarantees in §2 assume Unix (Linux). On
Windows, `verify_no_symlinks` is a no-op and `shell_exec` is unavailable.
See `ARCHITECTURE_SECURITY_MODEL.md` §10 for details.

---

## 3) Oracle / Feedback Rules (hard requirements)

### R-1: Reason codes are audit-only
If Gate produces `reason_code`, it **MUST NOT** be observable by any agent/client.
- **Allowed:** internal logs, operator audit, offline reports.
- **Forbidden:** returning reason codes to the requestor / agent.

### R-2: Public interface is binary
Externally observable outcomes must remain:
- `AUTHORIZED` (frame emitted), or
- `IMPOSSIBLE` (no frame / silence).

### R-3: No adaptive retry loops
No component may expose a signal that helps an agent search the boundary.
Retries without state change must be inert (same inputs → same verdict).

---

## 4) Sealed Inputs (non-tunable by integrators)

### R-4: Capacity / Budget / Progression are sealed
Any capacity-like input must be:
- **Non-writable** by integrators (private inners, getters only),
- Mutated only inside the law (`pub(crate)` consume/tick),
- Protected by compile-fail tests preventing field access.

### R-5: context_min is local-sealed
If a monotone counter is used, it **MUST** be generated locally at the membrane/actuator boundary.
Never provided by an agent/client. Never required as an external oracle.

### R-6: Signal source = TCB
Signal fields (`r_local`, `quantified_flow`, `quantified_entropy`, `observed_cadence`)
must come from a trusted provider (membrane/actuator), not from the agent.

---

## 5) Domain Identity (no truncation)

### R-7: Domain IDs are u64 end-to-end
All domain identifiers that reach SLIME egress **MUST** be representable as `u64`.
No truncation to `u32`/`u16` is permitted anywhere in the pipeline.

---

## 6) Observability (defense-in-depth) without feedback

Observability is allowed only as **forensics / audit**, never as input to the law.

**Allowed:**
- Actuator-side decoding logs (32B frames)
- Rate counters, parsing error counters
- Read-only dashboards from logs
- System metrics outside SLIME (CPU/RAM/network)

**Forbidden:**
- Auto-tuning capacity/budget based on metrics
- Changing allow/deny logic based on dashboards
- Exposing internal reasons to agents
- Any feedback path from observability → law evaluation

---

## 7) Minimal Compliance Checks

Before sealing any integration:

- [ ] `specs/` of each repo unchanged (canon intact)
- [ ] AB, Gate, Shield have `API_SURFACE_AUDIT.md` and compile-fail tests for sealed inners
- [ ] SLIME egress ABI remains exactly 32 bytes LE: `(u64 domain_id, u64 magnitude, u128 actuation_token)`
- [ ] Actuator owns socket; SLIME is client-only; fail-closed validated
- [ ] No reason codes in any externally observable output
- [ ] No `u16`/`u32` truncation on domain IDs
- [ ] Signal/context_min sourced from TCB, not agent

---

## Cross-references

- **SYF-Gate:** [github.com/AnathemaOfficial/SYF-Gate](https://github.com/AnathemaOfficial/SYF-Gate) — `specs/`, `ref/`, `docs/API_SURFACE_AUDIT.md`
- **SYF-Shield:** [github.com/AnathemaOfficial/SYF-Shield](https://github.com/AnathemaOfficial/SYF-Shield) — `specs/`, `pom/`, `docs/API_SURFACE_AUDIT.md`
- **Anathema-Breaker:** [github.com/AnathemaOfficial/Anathema-Breaker](https://github.com/AnathemaOfficial/Anathema-Breaker) — `specs/`, `src/pom/`, `tests/ui/`
- **SLIME:** this repo — `specs/`, `CONFORMANCE.md` (internal matrix)

---

**END — FULL-STACK CONFORMANCE**
