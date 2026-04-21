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
