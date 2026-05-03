# RN-2026-05 ADR-025 Guardrail Scoreboard (Draft)

Status: Draft
Scope: Research-only
Authority: ADR-022
Constraint: ADR-025 remains Proposed and non-binding.

## Check 1. Lane Sufficiency
Question:
- Are S1-S8 sufficient to describe per-hardware work without hidden gaps?

Evidence reviewed:
- docs/adr/ADR-025-dual-root-hardware-runway.md
- docs/orchestration/HARDWARE-QUEUE.md
- streams/S1-foundation/README.md
- streams/S7-hub/PROGRESS.md

Decision pressure:
- Need clear lane ownership before accepting dual-root policy.

Risks:
- Lane overlap can blur gate claims.

Recommendation:
- Pending

ADR text impact:
- Pending

D1/G4 claim impact:
- No direct gate movement; structure-only.

Unresolved evidence:
- Needs concrete lane ownership examples per board family.

## Check 2. S9 Necessity
Question:
- Is S9 needed as a separate ignition lane versus S8 governance extension?

Evidence reviewed:
- docs/adr/ADR-025-dual-root-hardware-runway.md
- docs/adr/ADR-022-decision-program-governance.md
- docs/orchestration/LOCAL-DRIVER.md

Decision pressure:
- Must avoid adding a lane with ambiguous authority.

Risks:
- Duplicate governance loops and unclear stop conditions.

Recommendation:
- Pending

ADR text impact:
- Pending

D1/G4 claim impact:
- No direct gate movement.

Unresolved evidence:
- Need operational examples showing S9 adds non-redundant value.

## Check 3. Fastest/FERROS Separation
Question:
- Does dual-root separation reduce overclaim risk while preserving delivery velocity?

Evidence reviewed:
- docs/adr/ADR-025-dual-root-hardware-runway.md
- docs/hardware/d1-target-inventory.md
- docs/orchestration/HARDWARE-QUEUE.md

Decision pressure:
- Must keep Track A evidence and Track B architecture disposition decoupled.

Risks:
- Premature binding of FERROS-root authority.

Recommendation:
- Pending

ADR text impact:
- Pending

D1/G4 claim impact:
- No direct gate movement.

Unresolved evidence:
- Need proof that handoff artifacts are enough to prevent claim leakage.

## Check 4. Embedded-Device Compression
Question:
- Should constrained targets like ESP32 collapse lane count or lane content?

Evidence reviewed:
- docs/adr/ADR-025-dual-root-hardware-runway.md
- docs/research/S4-no-std-target-matrix.md
- docs/research/S1-boot-sequence-d1-target.md

Decision pressure:
- Lane model must remain practical on constrained targets.

Risks:
- Over-scaffolding and stalled execution.

Recommendation:
- Pending

ADR text impact:
- Pending

D1/G4 claim impact:
- No direct gate movement.

Unresolved evidence:
- Need explicit ESP32 lane-shape proposal and tradeoffs.

## Check 5. Server-Control-Plane
Question:
- Can x86_64 Fastest safely serve as control-plane for Pi/Jetson/ESP32 findings?

Evidence reviewed:
- docs/adr/ADR-025-dual-root-hardware-runway.md
- docs/hardware/d1-target-inventory.md
- docs/research/S7-d1-bring-up-checklist.md

Decision pressure:
- Multi-board coordination depends on this assumption.

Risks:
- Control-plane overreach can misstate hardware evidence.

Recommendation:
- Pending

ADR text impact:
- Pending

D1/G4 claim impact:
- No direct gate movement.

Unresolved evidence:
- Need one complete evidence routing example from DUT to control-plane ledger.

## Check 6. Claim-Boundary
Question:
- Does the model prevent accidental claims beyond current evidence?

Evidence reviewed:
- docs/adr/ADR-025-dual-root-hardware-runway.md
- STATUS.md
- docs/orchestration/HARDWARE-QUEUE.md
- docs/gates/D1.md

Decision pressure:
- Claim safety is mandatory before any status promotion.

Risks:
- Language drift can imply D1/G4 closure without evidence.

Recommendation:
- Pending

ADR text impact:
- Pending

D1/G4 claim impact:
- Must remain no movement until evidence exists.

Unresolved evidence:
- Need red-team wording tests tied to run-log and findings outputs.

## Check 7. Agent-Permission
Question:
- Is read-wide/write-narrow operationally enforceable across roots and lanes?

Evidence reviewed:
- docs/adr/ADR-025-dual-root-hardware-runway.md
- docs/orchestration/LOCAL-DRIVER.md
- docs/adr/ADR-022-decision-program-governance.md

Decision pressure:
- Permission model is core to safe parallel execution.

Risks:
- Unenforced write boundaries can create conflicting truth surfaces.

Recommendation:
- Pending

ADR text impact:
- Pending

D1/G4 claim impact:
- No direct gate movement.

Unresolved evidence:
- Need enforcement mechanism definition (policy, tooling, or CI gate).

## Research Disclaimer
This scoreboard is research-only and does not change ADR status, gate status, or hardware evidence claims.

## HANDOFF CARD
- Lane ID: B1
- Status: complete
- Files read: docs/adr/ADR-025-dual-root-hardware-runway.md; docs/adr/ADR-022-decision-program-governance.md; docs/adr/_INDEX.md; docs/adr/_ROADMAP.md
- Files changed: docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-guardrail-scoreboard.md
- Evidence produced: seven-check scoreboard scaffold with required slots
- Claims added: research scaffold exists
- Claims explicitly not added: ADR-025 promotion, gate movement, binding architecture
- Validation: template completeness review for checks 1-7
- Residual risks: recommendations remain pending until batch notes are completed
- Next safe follow-up, if any: complete draft recommendations in batch1 and batch2 notes
