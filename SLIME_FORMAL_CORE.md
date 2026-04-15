# SLIME v0 — Formal Core

This document defines the canonical formal structure of SLIME v0.

It is not descriptive.
It is not philosophical.
It is not operational guidance.

It defines the structural invariant model.

---

## 1. Domain Definitions

Let:

- A = set of all possible ActionRequests
- E = set of all physically realizable effects
- ∅ = non-event (no effect)
- I : A → {true, false} = invariant predicate embedded in AB-S
- Φ : E → World = actuation mapping

SLIME governs only the mapping from A to E ∪ {∅}.

SLIME does not govern identity, intent, computation, or upstream logic.

---

## 2. Core Evaluation Function

Define the SLIME evaluation function:

S : A → E ∪ {∅}

Such that:

∀ a ∈ A:

- if I(a) = true  → S(a) = e ∈ E
- if I(a) = false → S(a) = ∅

No third state exists.

S(a) ∉ {error, partial, retry, fallback}

---

## 3. Non-Event Axiom

If S(a) = ∅:

- no semantic explanation is emitted
- no feedback signal is produced
- no gradient is exposed
- no adaptation interface exists

The absence of effect is terminal.

---

## 4. Attempt / Effect Separation

Upstream systems may generate any a ∈ A.

SLIME imposes no **semantic** constraint on attempt generation.

SLIME governs only effect manifestation.

Attempts are semantically unconstrained.
Effects are bounded.

> **Clarification (2026-04-15):** §6 specifies that malformed `a ∉ A`
> are rejected at ingress. This is a **format** constraint (schema
> enforcement), not a semantic constraint. The distinction is:
> format constraints ensure `a ∈ A` (well-formed); semantic constraints
> would evaluate whether `a` *should* be authorized. SLIME enforces the
> former; AB-S enforces the latter.

---

## 5. Invariant Immutability

The predicate I is:

- sealed
- non-configurable
- non-adaptive
- non-interpretable at runtime

No runtime function modifies I.

All human authority exists only at invariant genesis.

---

## 6. Interface Closure

Ingress accepts only a ∈ A conforming to a strict schema.

Malformed or undefined a ∉ A are rejected.

Egress applies Φ(e) only if e ∈ E.

If Φ(e) cannot be executed (actuator failure), the result is ∅.

No partial effect is allowed.

> **Three-valued observation note (2026-04-15):** The formal model
> defines two observable states: `S(a) = e` (AUTHORIZED) and `S(a) = ∅`
> (IMPOSSIBLE). However, an authorized effect whose actuation fails
> (e.g., egress socket disconnected) produces a third observable case:
> authorization granted, effect dropped. From the **external observer's**
> perspective this is indistinguishable from IMPOSSIBLE — the binary
> interface is preserved. From the **internal audit** perspective it is
> a distinct failure mode. Implementations MUST log actuation failures
> as audit events distinct from impossibilities.

---

## 7. Runtime Assumption

All guarantees assume:

- sealed execution environment
- integrity of AB-S core
- integrity of runtime appliance

If runtime integrity is violated,
the invariant guarantee is void.

---

## 8. Exclusion Statement

SLIME does not define:

- identity management
- permission systems
- policy engines
- adaptive control
- behavioral alignment
- process frameworks

SLIME defines only the mapping:

A → {E ∪ ∅}

under invariant I.

---

## Canonical Statement

SLIME v0 is a sealed invariant mapping from action request space to effect space.

Authorized effects manifest.
All other requests resolve to non-existence.

---


