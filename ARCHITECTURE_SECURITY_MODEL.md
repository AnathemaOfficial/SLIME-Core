# Structural Security Model

> From policy enforcement to structural impossibility

## 1. Structural vs Policy Security

Most software security systems attempt to control behavior using policies, permissions, and runtime checks.

These approaches share a common weakness:
they rely on interpreting rules at execution time.

This introduces complexity, policy drift, and exploitable feedback signals.

The SYF stack adopts a different approach.

Instead of deciding whether an action is allowed, the system **removes the representability of unauthorized effects**.

If the authorization signal does not exist, the action cannot occur.

## 2. The SYF Execution Stack

The architecture separates decision, constraint, and execution across independent layers.

```
SYF-Core
   ↓
SYF-Gate
   ↓
SYF-Shield
   ↓
Anathema-Breaker
   ↓
SLIME
   ↓
Actuator
```

Each component enforces a distinct invariant.

| Layer             | Responsibility                  |
|-------------------|----------------------------------|
| SYF-Core          | Mathematical invariants          |
| SYF-Gate          | Admission decision               |
| SYF-Shield        | Irreversible capacity consumption|
| Anathema-Breaker  | Deterministic law resolution     |
| SLIME             | Binary execution membrane        |
| Actuator          | Mechanical effect execution      |

No layer overlaps another's role.

## 3. Binary Enforcement

SLIME enforces the final boundary between decision and effect.

External behavior is strictly binary:

```
AUTHORIZED  →  frame emitted  →  actuator executes
IMPOSSIBLE  →  no frame       →  actuator remains silent
```

No explanations, error codes, or semantic feedback are exposed.

This removes the possibility of **authorization oracles**.

## 4. Monotonicity of the Action Space

The system forms a **monotone capability machine**.

Each action consumes capacity or reduces the space of possible actions:

```
possible_actions(t+1) ⊆ possible_actions(t)
```

This property prevents:

- Privilege escalation
- Adaptive probing
- Amplification through repeated attempts
- Strategic optimization by attackers

The system cannot accumulate new power over time.

**Multi-agent scope note:** In deployments with per-agent budgets
(e.g., SAFA P3 Agent Containment), monotonicity holds **per agent**.
One agent exhausting its capacity does not reduce the action space of
another agent. The global property is: no agent can increase its own
action space over time.

## 5. Irreversibility via Shield

SYF-Shield couples irreversible effects with capacity consumption.

Once the **Engagement Point (EP)** is crossed:

- Effect becomes partially irreversible
- Capacity is consumed atomically

Impossible states such as "engaged but not progressed" **cannot occur**.

The typestate system enforces this at compile time:
`Shield<Active>` → `Shield<Sealed>` with structurally absent methods on `Sealed`.

## 6. Separation of Decision and Execution

The actuator does not re-evaluate authorization. The signal itself **is** the authorization.

This strict separation removes the possibility of policy reinterpretation during execution.

SLIME transports a fixed 32-byte signal:

| Field             | Type   |
|-------------------|--------|
| `domain_id`       | `u64`  |
| `magnitude`       | `u64`  |
| `actuation_token` | `u128` |

No metadata. No semantic payload. This eliminates parsing exploits and interpretation ambiguity.

## 7. Integration Boundary (Where the System Can Still Fail)

Like any formal model, the system can only constrain what it represents.

The remaining theoretical risk is **unmodeled environmental effects**:

```
effect ∉ system state space
```

These must be mitigated at the integration boundary:

- **Primitive definition**: ensuring that domain actions map to real-world effects without gaps
- **Actuator design**: ensuring the actuator cannot produce effects beyond what the signal authorizes
- **Dimension coverage**: if a new dimension of effect exists outside the model, it is unconstrained

This is not a flaw — it is the boundary of any formal system. The key is that the boundary is **explicit and auditable**.

## 8. Model Limitations (Unmodeled Dimensions)

Symbiote analysis identified the concept of **primitive overreach**: when the actuator's real-world capability exceeds what the domain model represents.

Example: a "transfer" primitive might also trigger notifications, logs, or side-effects that are outside the capacity model.

Mitigation: primitives must be **minimal and atomic**. Each primitive should map to exactly one constrained effect.

## 9. Why Audits Become Simpler

Traditional systems require auditing complex policy interactions.

In SYF, auditors only need to verify a small set of invariants:

- Capacity never increases
- The binary membrane is preserved
- The 32-byte ABI remains intact
- No semantic feedback reaches agents
- The actuator executes only when a signal exists

Once these invariants hold, **entire classes of exploits become structurally impossible**.

**Policy hash scope:** Implementations such as SAFA expose a policy bundle
hash covering domain configuration, intent mappings, and per-agent
capabilities. Deployment metadata (bind host, workspace path, log settings)
is intentionally excluded from the hash — an operator moving the daemon
between hosts should not flip every existing manifest hash. Operators
requiring deployment-field attestation should pin it out-of-band
(infrastructure-as-code, immutable image, etc.).

## Summary

SYF replaces policy-based security with **structural law enforcement**.

Instead of controlling behavior dynamically, the architecture defines a system where unauthorized actions **cannot be represented**.

Security emerges from the structure itself.

---

## 10. Platform Scope (added 2026-04-15)

The structural guarantees in this document assume a **Unix (Linux) deployment
target**. Platform-specific limitations:

### 10.1 Windows

- **Symlink protection:** `verify_no_symlinks()` in SAFA's `WorkspacePath`
  is a no-op on Windows (`#[cfg(unix)]`). The TOCTOU/symlink race
  described in §7 is **not closed** on Windows. Production deployments
  SHOULD target Unix.
- **Shell actuator:** `shell_exec` uses `libc::setpgid` for process group
  isolation and is compiled with `#[cfg(unix)]` only. On non-unix targets,
  `shell_exec` actions return `ServiceUnavailable`.

### 10.2 IPv6 SSRF (known limitation)

SAFA's HTTP actuator (`is_private_ip`) classifies IPv4 private ranges
correctly but does NOT block all IPv6 internal addresses:

- `fc00::/7` (Unique Local Addresses) — **not blocked**
- `fe80::/10` (Link-Local) — **not blocked**
- `64:ff9b::/96` (NAT64) — **not blocked**

In deployments where the HTTP actuator reaches IPv6 internal services,
an attacker-controlled domain payload could trigger SSRF via these
address families. Mitigation: deploy a network-level allowlist or
restrict the actuator to IPv4-only.

> This limitation is tracked in SAFA's `THREAT_MODEL.md` and
> `KNOWN_ISSUES_P1.md` (C1 for symlinks, HTTP-IPv6 for SSRF).

---

*See also: [FULL_STACK_CONFORMANCE.md](./FULL_STACK_CONFORMANCE.md) for the cross-layer integration contract.*
