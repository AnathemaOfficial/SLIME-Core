# V1_INVARIANTS.md

**Status:** SEALED — Phase 0 Lock — Model B / CoreSpec
**Scope:** SLIME v1 — Compile-Time Law Specialization
**Supersedes:** All prior V1_INVARIANTS drafts

---

## 1. Core Identity

### 1.1 Law Ontology

The Core IS the law.

SLIME v1 introduces **CoreSpec**, defined as:

> A CoreSpec is a compile-time instantiation of Core law. It is not a
> parameterization of a generic runtime binary. It is a distinct sealed
> binary identity.

A CoreSpec:

- SHALL produce a distinct binary.
- SHALL NOT exist as a runtime artifact.
- SHALL NOT be loadable, swappable, or injectable at runtime.
- SHALL NOT share mutable runtime identity with any other CoreSpec.

Each binary produced from a CoreSpec IS that binary's law.
There is no separation between the law and its carrier.

### 1.2 Phase 1 Vigilance Point

> **WARNING:** Never reintroduce a generic Core layer with parameterized
> includes. This pattern silently reverts to Model A (runtime injection)
> disguised as compile-time specialization.
>
> Phase 1 implementation MUST verify that each CoreSpec produces a
> monolithic, non-decomposable binary with no shared generic runtime layer.

---

## 2. Zero Runtime Configuration

The Core:

- MUST NOT read configuration files at runtime.
- MUST NOT accept environment variables that modify policy.
- MUST NOT accept command-line flags that modify policy.
- MUST NOT load external CoreSpec artifacts.
- MUST NOT mmap or parse policy files at boot.

Policy identity is fully determined at compile time.

---

## 3. Statelessness

The Core:

- MUST NOT maintain mutable policy state.
- MUST NOT store historical decision state.
- MUST NOT implement cumulative or replay memory.
- MUST NOT implement rate-limiting state.

The Core MAY contain immutable compile-time constants.

Immutable compile-time constants are not considered mutable state.
This is not a relaxation of v0 statelessness. It is its correct
interpretation for a compiled system.

---

## 4. CoreSpec Model

A CoreSpec:

- MUST be embedded at compile-time or link-time.
- MUST NOT exist as a separate runtime file.
- MUST NOT require runtime attestation.
- MUST NOT require a trust-root oracle.
- MUST NOT be selected via build environment variable or feature flag.

**CoreSpec selection MUST be fully determined by immutable source files
under version control.**

**Build scripts MUST NOT alter CoreSpec identity based on environment
variables, feature flags, or external build-time state.**

**The resulting binary identity SHALL be derivable solely from
version-controlled source content.**

Shared implementation crates across CoreSpec variants are permitted.
They MUST NOT contain mutable statics, policy-adjacent logic, or any
runtime state that would establish shared identity between distinct
CoreSpec binaries.

---

## 5. Trust Model

SLIME v1 SHALL NOT implement a runtime trust-root mechanism.

Integrity is guaranteed by:

- Reproducible builds.
- Binary hash verification.
- Release tag verification.

If binary integrity is compromised post-build, it is outside the Core
threat model.

**Build toolchain integrity is assumed.** Toolchain compromise is outside
the Core threat model.

**Build authority is outside Core scope.** The entity controlling
compile-time source controls law identity. This is the sole remaining
policy change mechanism. It is a deployment model concern, not a Core
invariant.

---

## 6. Domain Mapping

Domain mapping:

- MUST be determined at compile time.
- MUST be fixed-size and immutable.
- MUST NOT expand dynamically.
- MUST NOT allocate memory based on domain cardinality.
- MUST NOT accept runtime domain registration.

If deterministic hashing is used:

- It MUST be compile-time generated.
- It MUST NOT depend on runtime input.

### 6.1 Platform Scope (added 2026-04-15)

The Core SHOULD target Unix (Linux) for production deployments.

On non-Unix platforms:
- `shell_exec` actuator capabilities MAY be unavailable.
- Filesystem symlink protection MAY be a no-op.
- Process group isolation (`setpgid`) is unavailable.

Implementations MUST document which structural guarantees are
platform-dependent and MUST NOT claim full containment on platforms
where OS-level primitives are absent.

### 6.2 Domain Confidentiality Statement

Domain mapping is NOT considered confidential.

Binary inspection may reveal domain identifiers.

Policy opacity is not a Core security property.

Security relies on structural impossibility, not secrecy.

---

## 7. Fail-Closed Semantics

### 7.1 Build-Time

If any compile-time invariant is violated:

- Build MUST fail.
- Binary MUST NOT be produced.

### 7.2 Runtime

The system MUST exit(1) before ingress bind under any of the following
conditions:

- ABI version mismatch.
- Internal assertion failure.
- Ingress socket bind failure.
- Required OS capability missing.
- Memory initialization failure.

No degraded mode SHALL exist.
No fallback CoreSpec SHALL exist.
No partial initialization SHALL be permitted.

Ingress MUST NOT open unless all initialization steps complete
successfully and in full.

---

## 8. Timing Discipline

SLIME v1 SHALL NOT claim strict constant-time execution.

Decision logic MUST be:

- Bounded.
- Free of unbounded loops.
- Free of input-size-dependent branching.
- Free of dynamic memory growth during decision.

All loop bounds MUST be compile-time derivable from CoreSpec constants
(e.g., domain table size). No bound may depend on runtime input or
runtime state.

Timing uniformity beyond bounded determinism is not guaranteed and is
not claimed.

---

## 9. Anti-Replay Boundary

Core:

- MUST NOT track actuation tokens.
- MUST NOT persist action identifiers.
- MUST NOT enforce rate limits.

Actuator remains solely responsible for replay prevention.

No feedback channel from actuator to Core SHALL exist.

---

## 10. Non-Goals

SLIME v1 SHALL NOT:

- Act as an interpretive control layer.
- Load policies dynamically.
- Support runtime CoreSpec switching.
- Provide rule editing.
- Provide trust-root rotation.
- Provide distributed consensus.
- Provide adaptive behavior.
- Claim policy opacity as a security property.

---

## 11. Invariant Preservation

SLIME v1 MUST preserve all v0 invariants without relaxation.

If any compile-time specialization violates a v0 invariant:

- The build MUST fail.
- The CoreSpec MUST NOT be produced.

---

## 12. Existential Guard Clause

If compile-time specialization cannot be implemented without:

- Introducing runtime policy state, or
- Introducing trust oracles, or
- Introducing mutable configuration channels,

**THEN SLIME v1 SHALL NOT proceed.**

---

## 13. Phase 2 Forward Flag

Any orchestration mechanism for multi-binary SLIME deployments
(binary registry, deployment manifest, multi-CoreSpec coordination)
MUST be treated as a potential runtime control reentry point.

Such mechanisms MUST be reviewed against the full v0 + v1 invariant set
before Phase 2 locks.

Core remains sealed. Orchestration lives outside Core. If orchestration
requires Core modification, it is rejected.

---

## Phase 0 Entry Record

| Condition | Source | Status |
|---|---|---|
| CoreSpec defined as distinct law instantiation | R-01 | Satisfied |
| Build authority acknowledged as out-of-scope | R-02 | Satisfied |
| Statelessness boundary clarified | R-03 | Satisfied |
| Runtime exit(1) triggers enumerated | R-04 | Satisfied |
| Domain confidentiality position stated | R-05 | Satisfied |
| Timing claims bounded, not constant-time | R-06 | Satisfied |
| Existential guard clause present | R-07 | Satisfied |
| Build toolchain assumption stated | R-08 | Satisfied |
| Phase 2 runtime-control reentry flag added | R-09 | Satisfied |

---

## Phase 1 Entry Condition

Verify monolithic binary output:

- No generic runtime layer shared across CoreSpec binaries.
- No parameterized includes that vary at build invocation.
- No shared Core infrastructure containing mutable state.
- Adversarial test suite (Option C) executed and cleared before
  implementation spec is written.

---

🧊 **End of Phase 0 — CoreSpec Definition — SEALED**
