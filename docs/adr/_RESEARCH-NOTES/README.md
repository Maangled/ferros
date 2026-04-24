# FERROS Research Notes

Research notes preserve investigation before a decision is stable enough for a full ADR.

Use this lane when the goal is to gather evidence, compare models, or hold cross-domain context in-repo without pretending the decision is already frozen.

---

## When to Use a Research Note

Create a research note when one or more of the following is true:

- the question materially affects future implementation but the evidence is incomplete
- multiple domains intersect and a premature ADR would hide unresolved trade-offs
- the work needs durable repo context but should not yet constrain streams as a binding decision
- the topic depends on external research, governance precedent, or long-horizon design exploration

Examples:

- ACC object-to-card projection before the first live shell slice
- governance or voting-mechanism comparison before any committed structure exists
- post-quantum or trustless-proof posture before the platform has a real migration plan

---

## Naming Pattern

Use the filename pattern:

`RN-YYYY-MM-short-topic.md`

Example:

`RN-2026-04-acc-card-deck-projection.md`

---

## Status Values

| Status | Meaning |
|--------|---------|
| Draft | Early note; structure may still change materially |
| Exploring | Active investigation with real evidence gathering underway |
| Ready for ADR | Evidence is strong enough that a formal decision record should be written |
| Rejected | The line of inquiry was examined and intentionally not promoted |
| Archived | Historical note retained for context after supersession or obsolescence |

---

## Suggested Structure

```markdown
# RN-YYYY-MM — [Short title]

**Status:** Draft | Exploring | Ready for ADR | Rejected | Archived
**Date:** YYYY-MM-DD
**Stream sponsor:** S1 / S2 / S3 / S4 / S5 / S6 / S7 / S8 / Cross-cutting
**Domain tags:** ...
**Primary evidence basis:** Implementation proof | Formal or analytical proof | Research or precedent proof | Operational proof

## Question

## Why this matters now

## Inputs and related records

## Findings

## Open questions

## Promotion criteria

## References
```

---

## Promotion Rule

Promote a research note into an ADR when:

- the main question is no longer exploratory
- the evidence basis is explicit and reviewable
- the result is intended to constrain streams, gates, contracts, or governance

If a note never reaches that point, keep it as research history or archive it explicitly.
