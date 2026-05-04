# ADR-034 - Arena non-evidentiary runtime boundary

**Status:** Draft  
**Date:** 2026-05-04  
**Stream:** S5 / Cross-cutting  
**Deciders:** Maangled  
**Domain tags:** architecture / UX doctrine / policy / research / cross-cutting  
**Primary evidence basis:** Research or precedent proof

_See [ADR-016-arena-export-target.md](./ADR-016-arena-export-target.md), [ADR-023-onramp-policy.md](./ADR-023-onramp-policy.md), and [../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md](../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md)._ 

---

## Context

Arena is the natural home for runtime preview, simulation, and result staging, but the current UX plan needs a hard boundary so Arena output does not silently become profile state, progression, or launch-facing evidence. The discovery note reinforces the usefulness of simulation language, but only if FERROS keeps it explicitly non-evidentiary until a later acceptance step occurs.

---

## Decision

**If accepted, FERROS will treat Arena output as staged, sandboxed, and non-evidentiary by default; promotion into Profile, progression, or canonical records will require an explicit accept step and a receipt.**

This draft does not yet define the final result artifact or promotion endpoint.

---

## Rationale

The UX should make Arena useful now without letting playful or experimental runtime output masquerade as trusted state.

### Options considered

| Option | Summary | Reason not chosen (if rejected) |
|--------|---------|--------------------------------|
| Option A (draft target) | Keep Arena output staged and non-evidentiary until accepted | - |
| Option B | Let Arena update progression or profile state directly | Rejected because it bypasses the staged-state and consent model |
| Option C | Avoid showing Arena results until full backend promotion exists | Rejected because staged-result UX is still useful and generates concrete queue work |

---

## Consequences

**Positive:**
- Arena can be built honestly before its full backend path exists.
- Operators can test Arena preview without confusing it with canonical state.
- Result-receipt work becomes an explicit queue instead of an implicit promise.

**Negative / trade-offs:**
- Requires some duplicated state language between Arena and Profile until shared receipt flows exist.
- Some users may expect progression updates that the current boundary intentionally blocks.

---

## Compliance

- Do not label Arena results as canonical or verified by default.
- Do not let Arena write directly into profile, grants, or progression without an explicit accepted boundary.
- Revisit this ADR if Arena later becomes a trusted canonical workflow instead of a sandbox/runtime lane.

---

## Implementation Evidence

- Current policy seed: [../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md](../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md).
- Existing surface doctrine: [ADR-016-arena-export-target.md](./ADR-016-arena-export-target.md).

---

## Deferred Scope or Open Research

- Deferred: exact result artifact shape and promotion path.
- Deferred: whether Arena and Forge share one artifact receipt model.
- Deferred: how non-evidentiary badges become visible in shell chrome.

---

## References

- [ADR-016-arena-export-target.md](./ADR-016-arena-export-target.md)
- [ADR-023-onramp-policy.md](./ADR-023-onramp-policy.md)
- [../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md](../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md)
