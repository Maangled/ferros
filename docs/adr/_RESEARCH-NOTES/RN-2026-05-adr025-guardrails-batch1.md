# RN-2026-05 ADR-025 Guardrails Batch 1 (Checks 1-3)

Status: Draft
Scope: Research-only
Constraint: ADR-025 remains Proposed and non-binding.

## Check 1. Lane Sufficiency
Question:
- Are S1-S8 sufficient to describe each hardware family without hidden governance gaps?

Evidence reviewed:
- docs/adr/ADR-025-dual-root-hardware-runway.md (required checks and lane model)
- docs/hardware/d1-target-inventory.md (Pack B x86_64 primary, Pack C companion)
- docs/research/S4-no-std-target-matrix.md (target constraints)
- docs/research/S7-d1-bring-up-checklist.md (evidence paths)

Decision pressure:
- Lane model must be specific enough for execution but not heavy enough to stall constrained boards.

Risks:
- Lane ambiguity can cause ownership overlap between stream and hardware-root docs.
- Constrained targets may require collapsed lane semantics.

Recommendation:
- Adjust.

ADR text impact:
- Keep S1-S8 baseline but add explicit per-family lane compression guidance in deferred scope.

D1/G4 claim impact:
- None. Structural clarification only.

Unresolved evidence:
- Need one per-family table showing required vs optional lanes before acceptance.

## Check 2. S9 Necessity
Question:
- Is S9 needed as a separate service lane, or should it be folded into S8 governance?

Evidence reviewed:
- docs/adr/ADR-025-dual-root-hardware-runway.md (S9 section)
- docs/adr/ADR-022-decision-program-governance.md (decision program and research lane)
- docs/orchestration/LOCAL-DRIVER.md (bounded recursion and serial truth surfaces)

Decision pressure:
- S9 must provide unique operational value and avoid duplicating S8 functions.

Risks:
- Redundant governance lane increases review burden.
- Unclear lane ownership could create competing truth-sync paths.

Recommendation:
- Adjust.

ADR text impact:
- Keep S9 provisional and require one explicit non-redundancy criterion before acceptance.

D1/G4 claim impact:
- None. Governance shape only.

Unresolved evidence:
- Need a concrete runbook example where S9 decisions differ from normal S8 truth-sync.

## Check 3. Fastest/FERROS Separation
Question:
- Does dual-root separation prevent overclaim and preserve delivery cadence?

Evidence reviewed:
- docs/adr/ADR-025-dual-root-hardware-runway.md (root meanings and handoff rules)
- docs/hardware/d1-target-inventory.md (current Pack B-first selection)
- docs/hardware/pack-b-session-01-plan.md (placeholder-based execution lock)
- docs/hardware/pack-b-session-01-command-map.md (operator mapping)

Decision pressure:
- Need a clean partition where Track A hardware evidence does not imply ADR acceptance.

Risks:
- Fastest-side findings can be misread as FERROS-root proof.
- Handoff docs may drift without explicit claim ceilings.

Recommendation:
- Accept with enforcement language.

ADR text impact:
- Add stronger non-claim language tying Fastest evidence to bounded claim classes.

D1/G4 claim impact:
- No direct movement. Improves claim discipline.

Unresolved evidence:
- Need one completed handoff packet after a real session to test boundary language.

## Research Disclaimer
This note is research-only and does not promote ADR-025, close any gate, or add physical hardware claims.

## HANDOFF CARD
- Lane ID: B2
- Status: complete
- Files read: docs/adr/ADR-025-dual-root-hardware-runway.md; docs/hardware/d1-target-inventory.md; docs/hardware/pack-b-session-01-plan.md; docs/hardware/pack-b-session-01-command-map.md; docs/research/S4-no-std-target-matrix.md; docs/research/S7-d1-bring-up-checklist.md; docs/orchestration/LOCAL-DRIVER.md
- Files changed: docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-guardrails-batch1.md
- Evidence produced: draft outputs for checks 1-3
- Claims added: research recommendations for checks 1-3
- Claims explicitly not added: ADR promotion, gate movement, hardware execution claims
- Validation: source-reference consistency pass
- Residual risks: unresolved evidence remains for lane compression and S9 non-redundancy
- Next safe follow-up, if any: complete checks 4-7 and run red-team synthesis
