# S8 Glossary And Doc-Batch Template Cleanup

Status: Batch F research handoff. This records terminology and template improvements to apply later without changing orchestration policy in this lane.

## Glossary Candidates

| Term | Proposed meaning |
|------|------------------|
| Batch Mode | Explicit queue-consuming mode that processes one track until its declared scope or a stop condition is reached |
| Lane | A bounded owner/anchor work unit inside a wave or batch |
| Width-8 | Current maximum repo-editing lane ceiling after the recorded proof runs; not a requirement to force eight unsafe lanes |
| Shared truth surface | Status, gate, queue, run-log, contract overview, CI, root manifest, or doc-batch file that should land in serial reconciliation |
| Frozen surface | A contract or evidence surface that cannot move without explicit scope, such as S2 schemas or closed gate wording |
| Evidence claim | A statement that a gate condition has been proven by a specific run, session, or artifact |
| Runway | Prep or rehearsal work that makes evidence possible but does not itself close a gate |
| Hardware window | Human-named device/session context required before hardware-track items can execute |
| Substrate refinement | Orchestration-policy cleanup rather than product-feature work |
| Conditional pass | Batch result where declared work landed but a named validation hook or review ambiguity remains |

## Doc-Batch Summary Template Notes

Future doc-batch summaries should keep this shape:

1. Batch metadata: date, track, theme, lane ceiling, and declared waves.
2. Lane table: wave ID, owning stream, anchor files, result, and validation.
3. Gatekeeper summary: pass/fail/conditional plus named ambiguities.
4. Frozen-surface check: schemas, gate evidence, ADR-024, and read-first contract surfaces.
5. Queue result: Ready/In Progress/Done state after close.
6. Hardware-window check.
7. Next-batch handoff.

## Cleanup Candidates

- Add these glossary entries to `GLOSSARY.md` only in a later S8 docs wave.
- Add a reusable doc-batch summary skeleton only if repetition starts creating real closeout risk.
- Keep policy wording in `LOCAL-DRIVER.md` and `BATCH-MODE.md` as the authority; this research note is not policy.

## Stop Lines

- Do not change the width-8 ceiling from this note.
- Do not rewrite orchestration policy.
- Do not edit closed gate evidence.
- Do not treat glossary cleanup as contributor issue seeding.
