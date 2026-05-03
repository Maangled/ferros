# REENTRY-PHASE0-ORCH Coordination Note

Status: Active
Date: 2026-05-03
Authority: docs/orchestration/LOCAL-DRIVER.md

## Mission
Execute the first bounded orchestration segment for the amended Ferros re-entry initiative using parallel owner lanes and one serial truth-sync lane.

## Initiative Truth Lock
- Track A is existing D1/G4 hardware-evidence queue work only.
- Track B is ADR-025 research and disposition only.
- ADR-025 remains Proposed and non-binding until accepted.
- Phase 0 is a hard stop before physical hardware execution.
- Findings templates are non-evidence until filled from real operator-attended sessions.

## Strict Dependency Mode
This segment runs under a temporary strict policy tighter than ADR-021:
- No npm packages.
- No npm manifests or lockfiles.
- No Cargo dependency additions.
- No Cargo.lock drift.
- No new Cargo workspace members unless explicitly pre-approved as local-only non-dependency structure.

## Gate Truth Snapshot (copied from STATUS)
- G1: closed.
- G2: closed.
- G3: closed.
- D1: active runway, not closed.
- G4: active, not closed.
- No physical-device evidence claim.
- No real Home Assistant proof claim.
- No consent acceptance proof claim.
- No D1 closure claim.
- No G4 closure claim.

## Track Separation Rule
Track A may advance queue-defined hardware evidence only after Phase 0 prerequisites are satisfied. Track B may produce research and disposition artifacts that inform ADR-025 status. Track B artifacts do not authorize hardware execution, gate movement, or ADR-025 promotion by themselves.

## Segment Stop Conditions
Stop-clean or stop-escalate if any of the following occurs:
- Validation failure that triage cannot repair.
- Missing required hardware/operator facts for Track A execution.
- Any wording drift that implies unsupported claims.
- Any strict dependency violation.
- Any change requiring frozen-schema/gate-close/destructive action.

## Phase Entry/Exit Criteria
### Phase 0 Entry
- Coordination note exists.
- Owner lanes scoped and non-overlapping.

### Phase 0 Exit
- All required placeholders identified and tracked.
- Blocker note published for unresolved hardware/operator facts.
- ADR-025 guardrail scoreboard scaffold created.
- Guardrail draft notes for checks 1-7 created as research-only.
- Dependency audit completed.
- Claim red-team summary completed.
- Serial truth-sync run-log entry appended.

## Claims Added
- Segment authority and boundaries were recorded.
- Strict dependency mode was recorded.
- Current gate truth was recorded as snapshot.

## Claims Explicitly Not Added
- No gate movement.
- No ADR-025 promotion.
- No physical-world execution claim.
- No Home Assistant proof claim.
- No D1 or G4 closure claim.

## HANDOFF CARD
- Lane ID: A0
- Status: complete
- Files read: STATUS.md; docs/adr/ADR-025-dual-root-hardware-runway.md; docs/adr/ADR-021-dependency-admission-policy.md; docs/adr/ADR-022-decision-program-governance.md; docs/orchestration/HARDWARE-QUEUE.md
- Files changed: docs/orchestration/REENTRY-PHASE0-COORDINATION.md
- Evidence produced: Coordination authority note for REENTRY-PHASE0-ORCH
- Claims added: Segment scope, strict dependency mode, gate truth snapshot, phase criteria
- Claims explicitly not added: Gate movement, ADR-025 promotion, hardware evidence
- Validation: markdown structure check and scope check
- Residual risks: Track A remains blocked until operator-specific placeholders are resolved
- Next safe follow-up, if any: execute A1 readiness audit and maintain claim ceilings
