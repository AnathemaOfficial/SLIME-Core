
<img width="1411" height="548" alt="SLIME logo 2" src="https://github.com/user-attachments/assets/4086e224-4e02-499a-8c1b-2a506aa44ad5" />

---

⚠️ Before reading this repository:

1. Read `INTERPRETATION_GUIDE.md`
2. Read `INTEGRATION_PRIMER.md`

If you interpret SLIME as a firewall, security tool, policy engine,
AI alignment system, cloud boundary, or monitoring framework,
you are using the wrong mental model.

---

# ⚠️ Canonical Reading Notice

This repository must be read as a description of a structural law,
not as an AI safety product or control system.

SLIME does not constrain intelligence.
SLIME does not guide behavior.
SLIME does not prevent harm.

SLIME removes the representability of certain effects.

Any interpretation of SLIME as a behavioral, ethical, probabilistic,
or any process-layer control system is incorrect.

---

## Repository Structure

The root directory and `specs/` define the canonical law-layer.

All executable code resides under `noncanon/`. The reference runner compiles standalone with the default stub resolver.

Anything outside `noncanon/` is documentation-only and defines no runtime behavior.

---

## Foundational Context

SLIME is part of a broader invariant-driven architecture:

- SYF-Core (mathematical invariant layer) https://github.com/AnathemaOfficial/SYF-Core
- SYF-Minimal (minimal canonical invariant spec) https://github.com/AnathemaOfficial/SYF-Minimal
- SYF-Lab (experimental environment) https://github.com/AnathemaOfficial/SYF-Lab
- SYF-Gate (admission verdict) https://github.com/AnathemaOfficial/SYF-Gate
- SYF-Shield (irreversibility + capacity) https://github.com/AnathemaOfficial/SYF-Shield

These are not extensions of SLIME,
but parallel primitives built around invariant enforcement.

See `FULL_STACK_CONFORMANCE.md` for cross-layer integration rules (Gate / Shield / AB / SLIME / Actuator).

See `ARCHITECTURE_SECURITY_MODEL.md` for the full structural security model (monotonicity, irreversibility, binary enforcement, model limitations).

---

## Formal Specification

The structural model of SLIME v0 is formally defined in:

- SLIME_FORMAL_CORE.md

This document defines the invariant mapping model (A → E ∪ ∅)
in purely formal terms, without narrative, policy, or operational framing.

---

# **SLIME v0**

**Systemic Law Invariant Machine Environnement**
**Sealed Execution Environment for Non-Negotiable Action Limits**

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

* an AI system
* a governance framework
* a policy engine
* a rule engine
* a monitoring tool
* a simulator
* a safety checklist
* a configuration layer
* a retry or fallback system

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

* All actions pass through SLIME or do not occur.
* The enforcement engine (**AB-S**) is embedded inside SLIME and is **never directly accessible**.
* SLIME is the **only surface visible** to users or operators.

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

* Accepts **declarative ActionRequests** only
* No logic
* No branching
* No heuristics

Ingress does not validate correctness or intent.

---

### 2. AB-S Core (Embedded)

* Fully sealed
* No configuration
* No inspection
* No repair
* No interface

AB-S outputs only:

* `OK(AuthorizedEffect)`
* `Err(Impossibility)`

---

### 3. Egress

* Maps authorized effects to **mechanical actuation**
* Fail-closed by design
* No retries
* No fallback paths
* No simulated success

If actuation cannot occur, **nothing occurs**.

---

### 4. Dashboard (Read-Only)

* Observational only
* Displays execution facts without semantic explanation
* No controls
* No tuning
* No influence on execution

The dashboard never feeds back into SLIME.

---

## Inputs

SLIME v0 accepts **only**:

* Declarative ActionRequests from upstream systems
* Verdicts from the embedded AB-S engine

SLIME explicitly rejects:

* human overrides
* configuration flags
* confidence scores
* external metrics
* adaptive parameters

---

## Outputs

SLIME v0 produces only:

* **Authorized physical effect**, or
* **Non-event**

`Err(Impossibility)` is not an error.
It is a terminal state.

No signal is produced for learning or optimization.

---

## Security Model

SLIME achieves security through **structural impossibility**.

* No feedback → no learning
* No learning → no circumvention
* No configuration → no drift

SLIME does not protect the engine.
SLIME prevents access to the engine entirely.

---

## Intended Use

SLIME v0 allows an organization to:

* plug SLIME in front of an existing system
* redirect actions through it
* observe what is blocked
* enforce hard limits without modifying internal code
* test sealed enforcement in real environments

SLIME is suitable for:

* autonomous systems
* industrial control
* financial actuation
* high-risk automation
* AI containment at the actuation boundary

---

## Versioning & Scope

* **SLIME v0 is intentionally minimal**
* Nothing may be inserted inside SLIME v0
* Future systems may wrap or deploy SLIME
* The core remains sealed

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

## Building the Reference Runner

```bash
cd noncanon/implementation_bundle/slime-runner
cargo build
```

This compiles with the default `stub_ab` feature — a standalone capacity-check
resolver that demonstrates the SLIME interface without external dependencies.

For enterprise deployments with the real law engine, see
[SLIME Enterprise](https://github.com/AnathemaOfficial/SLIME-Enterprise) (private).

---

## Status

**SLIME v0 — CANON / SEALED**

Development proceeds **only** by extension *around* SLIME, never inside it.

---

## License

Apache 2.0 — see [LICENSE](LICENSE).

