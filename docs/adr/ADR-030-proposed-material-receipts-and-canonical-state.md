# ADR-030 - Proposed material, receipts, and canonical state

**Status:** Draft  
**Date:** 2026-05-04  
**Stream:** Cross-cutting / S5 / S7  
**Deciders:** Maangled  
**Domain tags:** architecture / UX doctrine / policy / cross-cutting  
**Primary evidence basis:** Research or precedent proof

_See [ADR-022-decision-program-governance.md](./ADR-022-decision-program-governance.md), [ADR-023-onramp-policy.md](./ADR-023-onramp-policy.md), [ADR-029-human-operator-session-orchestration-and-evidence-flow.md](./ADR-029-human-operator-session-orchestration-and-evidence-flow.md), and [../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md](../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md). The discovery packet at `/home/homelab001/apps/FERROS-PDF-DISCOVERY-NOTE.md` is planning input, not authority._

---

## Context

FERROS already distinguishes between observed onramp material and canonical local state, but the current repo language is still fragmented across runway summaries, findings packets, shell surfaces, and draft UX planning. The next UX lane needs one explicit lifecycle for inbound or generated material so Profile, ACC, Home-Hub, Forge, and Arena all present the same state model without implying hidden writes or automatic promotion into canonical state.

---

## Decision

**If accepted, FERROS will standardize the staged-state lifecycle as: proposed -> inspected -> accepted or rejected -> receipt emitted -> canonical state updated only after explicit accept.**

This ADR draft scopes the lifecycle and the language, not the final endpoint or schema details.

---

## Rationale

This keeps ADR-023 visible at the surface level instead of leaving the invariant buried in backend behavior.

### Options considered

| Option | Summary | Reason not chosen (if rejected) |
|--------|---------|--------------------------------|
| Option A (draft target) | One shared staged-state lifecycle across surfaces | - |
| Option B | Let each surface describe staged material differently | Rejected because it will make consent, receipts, and operator evidence harder to reason about |
| Option C | Allow some imported or generated material to bypass staged state | Rejected because it weakens the current FERROS consent and lineage posture |

---

## Consequences

**Positive:**
- Shared language for proposed material, receipts, and canonical state.
- Cleaner operator-session instructions and evidence checks.
- Home-Hub and Arena can surface honest blocked states without pretending missing backend seams are complete.

**Negative / trade-offs:**
- Adds another explicit state model that must stay aligned with shell copy and adapter seams.
- Receipt and lineage fields may need new backend support before the full model can be rendered consistently.

---

## Compliance

- Do not describe staged material as canonical before explicit acceptance.
- Do not emit receipts that imply canonical promotion when the result is still rehearsal, runway, or blocked.
- Revisit this ADR if a future surface needs more than one staged-state family.

---

## Implementation Evidence

- Current groundwork: [ADR-023-onramp-policy.md](./ADR-023-onramp-policy.md), [../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md](../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md), and [../orchestration/OPERATOR-SESSION-PATTERN.md](../orchestration/OPERATOR-SESSION-PATTERN.md).
- Existing display-only observation path: the localhost runway route and shell summary surfaces referenced by `streams/S5-ux/README.md`.

---

## Deferred Scope or Open Research

- Deferred: exact receipt schema, receipt persistence, and receipt query endpoints.
- Deferred: whether proposed material from Arena, Forge, and Home-Hub share one artifact model or sibling models.
- Deferred: canonical-state promotion mechanics once endpoint support exists.

---

## References

- [ADR-023-onramp-policy.md](./ADR-023-onramp-policy.md)
- [ADR-029-human-operator-session-orchestration-and-evidence-flow.md](./ADR-029-human-operator-session-orchestration-and-evidence-flow.md)
- [../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md](../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md)
