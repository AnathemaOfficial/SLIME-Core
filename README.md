# SLIME-Core

**SLIME-Core** is the canonical kernel of the SLIME model.

It defines a **structural law layer** where certain classes of effects become **unrepresentable** by construction.

This repository exists to make that law:
- explicit
- inspectable
- verifiable

---

## Canonical Reading Notice

SLIME-Core is frequently misinterpreted.

It is **not**:

- a firewall
- a security tool
- a policy engine
- an AI alignment system
- a monitoring or observability layer
- a runtime system

SLIME-Core does **not control behavior**.

It defines a system in which certain behaviors **cannot be expressed at all**.

> SLIME does not block effects.
> It removes their representability.

---

## What SLIME-Core Is

SLIME-Core is a **canonical kernel** that encodes:

- structural constraints
- invariant-preserving transformations
- representability boundaries
- impossibility conditions

It operates at the level of:

> **what can exist in a system — not what a system chooses to do**

This makes it suitable as a foundation for:

- formal reasoning
- system design under constraints
- verification of effect boundaries
- composition with higher-level systems

---

## Repository Structure

```
specs/
  Canonical law-layer specifications
  (documentation only, no executable code)

noncanon/
  Reference executable runner
  - standalone compilation
  - stub resolver
  - demonstrative, not authoritative

root/
  Documentation and canonical entry points
```

### Notes

- `specs/` is the source of truth for the model
- `noncanon/` exists to test and explore the model
- the root contains no runtime system

---

## What SLIME-Core Is NOT

SLIME-Core does **not**:

- execute actions
- evaluate policies
- expose APIs
- integrate with external systems
- implement user workflows
- encode domain or product semantics

It is **not** an application layer.

It is **not** an execution environment.

---

## Relationship to SAFA

SLIME-Core and SAFA are **complementary foundations**.

- **SLIME-Core** defines what is structurally representable
- **SAFA** evaluates whether an action is authorized

In simplified terms:

- SLIME-Core → **what can exist**
- SAFA → **what is allowed**

SAFA operates at the policy layer, while SLIME-Core operates at the structural layer.

Both are designed to be:
- composable
- product-agnostic
- independently usable

See: https://github.com/AnathemaOfficial/SAFA

---

## Reference Runner (`noncanon/`)

The `noncanon/` directory contains a minimal executable reference:

- demonstrates how the model can be evaluated
- allows experimentation with the kernel
- is intentionally incomplete and non-authoritative

It exists for:

- testing
- exploration
- educational purposes

It does **not** define the canonical model.

---

## Doctrinal Integrity

SLIME-Core follows strict principles:

- **Structural, not behavioral**
  The model defines constraints, not decisions

- **Impossibility over prevention**
  Effects are removed at the level of representation

- **Canonical over runtime**
  The specification is primary; execution is secondary

- **Product isolation**
  No product, provider, or workflow semantics exist here

> If a system enforces behavior, it is not SLIME.
> If a system makes certain behaviors impossible to express, it may be.

---

## For Developers & Researchers

You may find SLIME-Core useful if you are:

- designing systems with strict invariants
- exploring formal constraint models
- working on safe composition of effects
- building higher-level systems that require structural guarantees

Recommended approach:

1. Read the `specs/` directory carefully
2. Use `noncanon/` to experiment
3. Treat execution as illustrative, not authoritative

---

## Building the Reference Runner

```bash
cd noncanon/implementation_bundle/slime-runner
cargo build
```

This compiles with the default `stub_ab` feature — a standalone capacity-check resolver that demonstrates the SLIME interface. `serde` / `serde_json` are the only external dependencies (used for safe JSON parsing).

**Runtime expectations:**

- The runner binds to `127.0.0.1:8080`
- The egress socket at `/run/slime/egress.sock` must be present at startup (fail-closed)
- On non-Unix targets, use the `integration_demo` feature for file-based egress
- See `CONFORMANCE.md` for intentional divergences between the reference runner and the canonical specification

**Related repositories:**

- [SAFA](https://github.com/AnathemaOfficial/SAFA) — complementary public foundation (policy layer for autonomous agents and LLM tool-use)
- [SLIME-Enterprise](https://github.com/AnathemaOfficial/SLIME-Enterprise) — enterprise actuator membrane (private)

---

## Foundational Context

SLIME is part of a broader invariant-driven architecture:

- [SYF-Core](https://github.com/AnathemaOfficial/SYF-Core) — mathematical invariant layer
- [SYF-Minimal](https://github.com/AnathemaOfficial/SYF-Minimal) — minimal canonical invariant spec
- [SYF-Lab](https://github.com/AnathemaOfficial/SYF-Lab) — experimental environment
- [SYF-Gate](https://github.com/AnathemaOfficial/SYF-Gate) — admission verdict
- [SYF-Shield](https://github.com/AnathemaOfficial/SYF-Shield) — irreversibility + capacity

These are not extensions of SLIME, but parallel primitives built around invariant enforcement.

### Cross-layer references

- `FULL_STACK_CONFORMANCE.md` — cross-layer integration rules (Gate / Shield / AB / SLIME / Actuator)
- `ARCHITECTURE_SECURITY_MODEL.md` — full structural security model (monotonicity, irreversibility, binary enforcement, model limitations)
- `SAFA_ADAPTER_MAPPING.md` — how SLIME-Core concepts map to SAFA's actual API surface
- `KNOWN_LIMITATIONS.md` — platform-specific and known security limitations

### Formal specification

The structural model of SLIME v0 is formally defined in:

- `SLIME_FORMAL_CORE.md` — invariant mapping model (A → E ∪ ∅) in purely formal terms, without narrative, policy, or operational framing

---

# SLIME v0 — Canonical Specification

**Systemic Law Invariant Machine Environnement**
**Sealed Execution Environment for Non-Negotiable Action Limits**

The following sections preserve the foundational specification of SLIME v0 as established during the initial design work. They describe the model at the level of modules, inputs, outputs, and sealed enforcement, complementing the developer-facing overview above.

---

## What SLIME Is

**SLIME v0** is a **sealed execution environment** that enforces **non-negotiable action limits** independent of intelligence, intent, configuration, or code quality.

It is designed to sit **in front of an existing system**, intercept actions, and allow only those that are **physically authorized** by a sealed engine.

SLIME does not decide.
SLIME does not interpret.
SLIME applies a law that has already been decided.

---

## What SLIME Is Not

SLIME is **not**:

- an AI system
- a governance framework
- a policy engine
- a rule engine
- a monitoring tool
- a simulator
- a safety checklist
- a configuration layer
- a retry or fallback system

SLIME does not optimize behavior.
SLIME does not correct systems.
SLIME does not explain decisions.

---

## Architecture Overview

```
Existing System
      ↓
   SLIME v0
      ↓
    World
        ↓
   (AB-S sealed internally)
```

- All actions pass through SLIME or do not occur.
- The enforcement engine (**AB-S**) is embedded inside SLIME and is **never directly accessible**.
- SLIME is the **only surface visible** to users or operators.

---

## Core Principle

SLIME enforces **impossibility**, not policy.

If an action is not authorized, it does not fail —
**it does not happen**.

There is no retry.
There is no override.
There is no exception.

---

## SLIME v0 Modules

SLIME v0 consists of **four fixed modules**.

### 1. Ingress

- Accepts **declarative ActionRequests** only
- No logic
- No branching
- No heuristics

Ingress does not validate correctness or intent.

---

### 2. AB-S Core (Embedded)

- Fully sealed
- No configuration
- No inspection
- No repair
- No interface

AB-S outputs only:

- `OK(AuthorizedEffect)`
- `Err(Impossibility)`

---

### 3. Egress

- Maps authorized effects to **mechanical actuation**
- Fail-closed by design
- No retries
- No fallback paths
- No simulated success

If actuation cannot occur, **nothing occurs**.

---

### 4. Dashboard (Read-Only)

- Observational only
- Displays execution facts without semantic explanation
- No controls
- No tuning
- No influence on execution

The dashboard never feeds back into SLIME.

---

## Inputs

SLIME v0 accepts **only**:

- Declarative ActionRequests from upstream systems
- Verdicts from the embedded AB-S engine

SLIME explicitly rejects:

- human overrides
- configuration flags
- confidence scores
- external metrics
- adaptive parameters

---

## Outputs

SLIME v0 produces only:

- **Authorized physical effect**, or
- **Non-event**

`Err(Impossibility)` is not an error.
It is a terminal state.

No signal is produced for learning or optimization.

---

## Security Model

SLIME achieves security through **structural impossibility**.

- No feedback → no learning
- No learning → no circumvention
- No configuration → no drift

SLIME does not protect the engine.
SLIME prevents access to the engine entirely.

---

## Intended Use

SLIME v0 allows an organization to:

- plug SLIME in front of an existing system
- redirect actions through it
- observe what is blocked
- enforce hard limits without modifying internal code
- test sealed enforcement in real environments

SLIME is suitable for:

- autonomous systems
- industrial control
- financial actuation
- high-risk automation
- AI containment at the actuation boundary

---

## Versioning & Scope

- **SLIME v0 is intentionally minimal**
- Nothing may be inserted inside SLIME v0
- Future systems may wrap or deploy SLIME
- The core remains sealed

---

## Common Misinterpretations

SLIME is not:

- a firewall
- a security policy engine
- an AI alignment system
- a monitoring system
- a cloud boundary service

If you read it that way, you are applying the wrong mental model.

---

## Canonical Statement

> **SLIME applies a law that cannot be negotiated.
> It exposes no controls, offers no explanations, and allows no exceptions.
> What passes through SLIME is physically authorized — everything else does not exist.**

---

## Status

Active development.

SLIME-Core is a foundational model and may evolve as its formalization improves.

---

## Philosophy

> Do not restrict behavior.
> Remove the ability to express it.

SLIME-Core is an attempt to encode that idea precisely.

---

## License

Apache License 2.0 — see [LICENSE](LICENSE).
