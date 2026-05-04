# ADR-031 - Evidence badges and security-rating claims

**Status:** Draft  
**Date:** 2026-05-04  
**Stream:** Cross-cutting / S5 / S8  
**Deciders:** Maangled  
**Domain tags:** governance / policy / UX doctrine / security / cross-cutting  
**Primary evidence basis:** Research or precedent proof

_See [ADR-022-decision-program-governance.md](./ADR-022-decision-program-governance.md), [ADR-029-human-operator-session-orchestration-and-evidence-flow.md](./ADR-029-human-operator-session-orchestration-and-evidence-flow.md), and [../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md](../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md). The external discovery note is motif input only._

---

## Context

The upcoming shell work wants visible evidence badges, local-only badges, non-evidentiary markers, and status-like rails, but FERROS cannot allow vague security or verification language to drift into UX copy without named proof. The paper-era idea of a generic security rating is useful only if translated into a bounded evidence-badge system backed by concrete sources.

---

## Decision

**If accepted, FERROS will allow only evidence badges backed by named proof surfaces and will reject unsupported security-rating or readiness claims in UI copy.**

This draft scopes allowed badge families and proof expectations rather than freezing the exact visual system.

---

## Rationale

The UI needs a way to show state clearly without over-claiming trust, safety, or launch readiness.

### Options considered

| Option | Summary | Reason not chosen (if rejected) |
|--------|---------|--------------------------------|
| Option A (draft target) | Restrict badges to named evidence-backed labels | - |
| Option B | Allow soft security or verification labels as design language | Rejected because it invites unsupported trust claims |
| Option C | Ban all badges and state labels beyond raw text | Rejected because operators still need clear state shorthand |

---

## Consequences

**Positive:**
- UI claims stay tied to tests, receipts, findings, or artifacts.
- Operator sessions have a clear sanity-check rule for badge language.
- Launch and module-lane language stays honest.

**Negative / trade-offs:**
- Designers and docs work must maintain a proof map for badge labels.
- Some visually appealing labels from prior art will remain unavailable.

---

## Compliance

- A badge must point to a named proof surface, test, receipt, artifact, or findings record.
- Placeholder badges must be labeled as placeholder, rehearsal, runway, blocked, or non-evidentiary when appropriate.
- Revisit this ADR if FERROS later needs a richer assurance-tier vocabulary beyond direct proof labels.

---

## Implementation Evidence

- Current seed language exists in [../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md](../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md) and the discovery note.
- Operator-session review path is defined in [ADR-029-human-operator-session-orchestration-and-evidence-flow.md](./ADR-029-human-operator-session-orchestration-and-evidence-flow.md).

---

## Deferred Scope or Open Research

- Deferred: exact badge taxonomy and iconography.
- Deferred: whether badges should encode severity, provenance, or freshness separately.
- Deferred: which badges become reusable shared modules vs. one-off labels.

---

## References

- [ADR-029-human-operator-session-orchestration-and-evidence-flow.md](./ADR-029-human-operator-session-orchestration-and-evidence-flow.md)
- [../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md](../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md)
