# RN-2026-05 ADR-025 Guardrails Batch 2 (Checks 4-7)

Status: Draft
Scope: Research-only
Constraint: ADR-025 remains Proposed and non-binding.

## Check 4. Embedded-Device Compression
Question:
- Should embedded targets (especially ESP32) use compressed lane structure?

Evidence reviewed:
- docs/adr/ADR-025-dual-root-hardware-runway.md
- docs/research/S4-no-std-target-matrix.md
- docs/research/S1-boot-sequence-d1-target.md

Decision pressure:
- Full lane set may not be practical for constrained targets.

Risks:
- Over-scaffolded docs with no executable path.
- Misleading parity assumptions between server-class and embedded targets.

Recommendation:
- Adjust.

ADR text impact:
- Add explicit compressed profile for constrained targets with required minimum lanes.

D1/G4 claim impact:
- None. D1/G4 still depend on real evidence.

Unresolved evidence:
- Need target-specific feasibility notes for ESP32 Fastest and FERROS roots.

## Check 5. Server-Control-Plane
Question:
- Can x86_64 Fastest act as control-plane for multi-board evidence routing?

Evidence reviewed:
- docs/adr/ADR-025-dual-root-hardware-runway.md
- docs/hardware/d1-target-inventory.md
- docs/research/S7-d1-bring-up-checklist.md

Decision pressure:
- Control-plane assumption affects how findings are aggregated and reviewed.

Risks:
- Over-centralization can blur source-of-evidence boundaries.
- Control-plane wording may imply remote orchestration claims not yet proven.

Recommendation:
- Accept with caveats.

ADR text impact:
- Keep x86_64 control-plane concept but require explicit source-attribution and claim ceilings in findings.

D1/G4 claim impact:
- None directly. Improves evidence attribution quality.

Unresolved evidence:
- Need one real Pack B/Pack C evidence route demonstrating attribution chain.

## Check 6. Claim-Boundary
Question:
- Does ADR-025 language adequately prevent unsupported claims?

Evidence reviewed:
- docs/adr/ADR-025-dual-root-hardware-runway.md
- STATUS.md
- docs/gates/D1.md
- docs/orchestration/HARDWARE-QUEUE.md

Decision pressure:
- Claim safety must be mechanically enforceable at run-log and findings level.

Risks:
- Ambiguous language could imply D1/G4 or HA proof.
- Research artifacts may be misread as implementation proof.

Recommendation:
- Adjust.

ADR text impact:
- Strengthen compliance section with explicit prohibited claims until evidence artifacts exist.

D1/G4 claim impact:
- Keeps both gates open until required evidence is present.

Unresolved evidence:
- Need claim red-team checklist integrated into orchestration outputs.

## Check 7. Agent-Permission
Question:
- Is read-wide/write-narrow enforceable across hardware roots and lanes?

Evidence reviewed:
- docs/adr/ADR-025-dual-root-hardware-runway.md
- docs/orchestration/LOCAL-DRIVER.md
- docs/adr/ADR-022-decision-program-governance.md

Decision pressure:
- Parallel lane safety depends on enforceable write boundaries.

Risks:
- Policy-only permissions without tooling checks can drift.
- Cross-lane edits to truth surfaces may bypass intended controls.

Recommendation:
- Adjust.

ADR text impact:
- Add an enforcement section requiring lane packet restrictions and serial truth-sync authority.

D1/G4 claim impact:
- None directly. Improves governance and claim safety.

Unresolved evidence:
- Need one batch run proving non-overlap enforcement under lane packets.

## Research Disclaimer
This note is research-only and does not promote ADR-025, close any gate, or claim hardware evidence.

## HANDOFF CARD
- Lane ID: B3
- Status: complete
- Files read: docs/adr/ADR-025-dual-root-hardware-runway.md; docs/hardware/d1-target-inventory.md; docs/research/S4-no-std-target-matrix.md; docs/research/S1-boot-sequence-d1-target.md; docs/research/S7-power-cycle-recovery-protocol.md; docs/adr/ADR-023-onramp-policy.md; docs/adr/ADR-024-ledger-substrate.md
- Files changed: docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-guardrails-batch2.md
- Evidence produced: draft outputs for checks 4-7
- Claims added: research recommendations for checks 4-7
- Claims explicitly not added: ADR promotion, gate movement, hardware execution claims
- Validation: source-reference consistency pass
- Residual risks: enforcement evidence is still missing
- Next safe follow-up, if any: run R1 claim red-team and disposition pre-check
