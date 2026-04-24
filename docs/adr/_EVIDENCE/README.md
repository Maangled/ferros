# FERROS ADR Evidence Lane

This directory is reserved for additive proof records that support ADRs or research notes when the evidence outgrows a simple references list.

Use it sparingly. Most ADRs should still keep their most important evidence links in the body of the record itself.

---

## When to Add a Separate Evidence Record

Add a dedicated evidence file only when one or more of the following is true:

- the proof set spans multiple PRs, tests, harnesses, CI runs, or operational records
- the evidence is changing faster than the decision text
- a research note or ADR needs a neutral place to aggregate proof links without rewriting the record body repeatedly

---

## Suggested Naming

- `ADR-XXX.evidence.md`
- `RN-YYYY-MM-short-topic.evidence.md`

Keep evidence records human-readable. They should summarize the proof surface and link to tests, CI runs, harnesses, or external references.

---

## Minimum Structure

```markdown
# ADR-XXX Evidence

## Scope

## Implementation proofs

## Formal or analytical proofs

## Operational proofs

## Open evidence gaps
```

Evidence files should not become a second decision record. They support an ADR or research note; they do not replace it.
