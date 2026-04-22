<img width="1199" height="349" alt="SLIME logo 2" src="https://github.com/user-attachments/assets/9ff555b7-0b0d-468d-9758-1f0e81788b4d" />

# SLIME-Core

**Systemic Law Invariant Machine Environnement**

**SLIME-Core** is the canonical kernel of the SLIME model.

It defines a **structural law layer** where certain classes of effects become **unrepresentable by construction**.

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

> It defines systems where certain behaviors cannot be expressed at all.

---

## What SLIME-Core Is

SLIME-Core is a **canonical kernel** that encodes:

- structural constraints  
- invariant-preserving transformations  
- representability boundaries  
- impossibility conditions  

It operates at the level of:

> **what can exist in a system — not what a system chooses to do**

---

## Repository Structure


specs/
Canonical law-layer specifications
(source of truth, documentation-only)

noncanon/
Reference executable runner
(standalone, exploratory, non-authoritative)

root/
Documentation and canonical entry points


### Notes

- `specs/` is the canonical source of truth  
- `noncanon/` exists for experimentation and testing  
- the repository does not define a runtime system  

---

## What SLIME-Core Is NOT

SLIME-Core does **not**:

- execute actions  
- evaluate policy  
- expose APIs  
- integrate with external systems  
- define workflows  
- encode product semantics  

It is not:
- an application layer  
- a runtime environment  
- a product component  

---

## Relationship to SAFA

SLIME-Core and SAFA are **complementary foundations**.

- **SLIME-Core** defines what is structurally representable  
- **SAFA** evaluates what is allowed  

In simplified terms:

- SLIME-Core → **what can exist**  
- SAFA → **what is allowed**  

Both are:
- composable  
- product-agnostic  
- independently usable  

See: https://github.com/AnathemaOfficial/SAFA

---

## Reference Runner (`noncanon/`)

The `noncanon/` directory provides a minimal executable model:

- enables experimentation with the kernel  
- demonstrates evaluation mechanics  
- supports exploration and testing  

It is:
- illustrative  
- incomplete  
- non-authoritative  

The canonical model remains in `specs/`.

---

## Doctrinal Integrity

SLIME-Core follows strict principles:

- **Structural, not behavioral**  
  It defines constraints, not decisions  

- **Impossibility over prevention**  
  Effects are removed at the level of representation  

- **Canonical over runtime**  
  Specification is primary; execution is secondary  

- **Product isolation**  
  No product or provider semantics exist here  

> If a system enforces behavior, it is not SLIME.  
> If a system makes behavior impossible to express, it may be.

---

## For Developers & Researchers

SLIME-Core is useful for:

- designing systems with strict invariants  
- reasoning about effect boundaries  
- exploring structural constraint models  
- building higher-level systems on top of canonical guarantees  

Suggested approach:

1. Read `specs/` carefully  
2. Use `noncanon/` for experimentation  
3. Treat execution as illustrative, not authoritative  

---

## Status

Active development.

SLIME-Core is a foundational model and may evolve as its formalization improves.

---

## Philosophy

> Do not restrict behavior.  
> Remove the ability to express it.
---

## License

Apache License 2.0 — see [LICENSE](LICENSE).
