# FERROS Doctrine

> This document states the root evaluation rubric for FERROS decisions.
> ADRs, research notes, gates, and stream plans should cite it when a choice materially affects ownership, consent, portability, auditability, or trust.

---

## Root Thesis

FERROS exists to increase user ownership and digital rights over identity, automation, data, and local computing.

Every material decision in FERROS should be evaluated against whether it increases, preserves, or compromises:

- local sovereignty over state and execution
- explicit consent before privileged action
- inspectability and portability of core surfaces and records
- durable, versioned contracts instead of ambient convention
- auditability of meaningful state transitions
- minimal dependency and custody assumptions

---

## Core Principles

### 1. Local Sovereignty First

The default FERROS posture is local control of identity, state, and automation. Networked, hosted, or on-chain systems may extend the platform, but they do not define the baseline user right.

### 2. Consent Before Capability

FERROS treats privileged action as something that must be explicitly armed, reviewed, and auditable. Convenience is not a sufficient reason to bypass a consent boundary.

### 3. Inspectable and Portable Surfaces

Users should be able to inspect core artifacts, move their data, and understand the seams between shell, runtime, profile, and contracts without opaque vendor-specific machinery.

### 4. Contracts Over Guesswork

When two streams or layers depend on each other, the dependency should be recorded as an explicit contract, schema, trait, protocol, or documented surface rather than inferred from one implementation.

### 5. Auditable State Transitions

Changes that matter should leave a reviewable trail. FERROS should prefer signed records, explicit status changes, reproducible tests, and append-only evidence over implicit mutation.

### 6. Minimal Ambient Trust

Dependencies, hosted services, custody layers, and governance assumptions should stay as small and explicit as possible. If trust must be introduced, it should be named and justified.

---

## Proof Expectations

FERROS does not use one proof style for every domain. The right evidence depends on the kind of claim being made.

| Evidence basis | Typical use | What counts as proof |
|---------------|-------------|----------------------|
| Implementation proof | Runtime, crate, schema, CLI, harness, CI claims | Running code, focused tests, fixtures, harness passes, CI evidence |
| Formal or analytical proof | Protocols, invariants, cryptographic or structural claims | Mathematical argument, schema logic, benchmark or invariant analysis |
| Research or precedent proof | Governance, economics, pedagogy, biomedical, jurisdiction, ecosystem strategy | Credible external research, standards, peer-reviewed or durable references |
| Operational proof | Deployment, real-hardware, launch, contributor-scale, governance operation | Recorded runbooks, field tests, sign-off surfaces, hosted or real-device evidence |

Every new ADR or research note should declare which evidence basis it primarily relies on and what evidence is still missing.

---

## How to Use This Document

When writing an ADR or research note:

1. Name which doctrine principles are most affected.
2. State the primary evidence basis for the record.
3. If the evidence is incomplete, keep the record in the research lane or mark the decision as proposed instead of pretending the question is settled.

When reviewing a change:

1. Ask whether the change strengthens or weakens user ownership and digital rights.
2. Check whether the claimed benefits are supported by the right kind of proof for that domain.
3. Prefer explicit trade-offs over hidden erosion of sovereignty, auditability, or portability.

---

## Related Records

- [docs/adr/ADR-0001-start-new-do-not-fork.md](docs/adr/ADR-0001-start-new-do-not-fork.md)
- [docs/adr/ADR-002-smart-contract-boundaries.md](docs/adr/ADR-002-smart-contract-boundaries.md)
- [docs/adr/ADR-007-single-file-system.md](docs/adr/ADR-007-single-file-system.md)
- [docs/adr/ADR-010-cards-and-decks-nomenclature.md](docs/adr/ADR-010-cards-and-decks-nomenclature.md)
- [docs/adr/ADR-015-universal-parametric-authoring-workbench.md](docs/adr/ADR-015-universal-parametric-authoring-workbench.md)
- [docs/adr/ADR-021-dependency-admission-policy.md](docs/adr/ADR-021-dependency-admission-policy.md)
