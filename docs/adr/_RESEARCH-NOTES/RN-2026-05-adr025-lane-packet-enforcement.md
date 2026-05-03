# RN-2026-05 ADR-025 Lane-Packet Enforcement

Status: Draft
Scope: Research-only
Constraint: ADR-025 remains Proposed and non-binding.

## Question
- What is the minimum read-wide or write-narrow rule set needed for ADR-025 lane packets?
- How should serial truth-sync work when a wave fans out into multiple implementation or review lanes?

## Evidence Reviewed
- `docs/adr/ADR-025-dual-root-hardware-runway.md`
- `docs/orchestration/LOCAL-DRIVER.md`
- `docs/orchestration/BATCH-MODE.md`
- `docs/orchestration/SYSTEM-QUEUE.md`
- `docs/orchestration/WAVE-RUN-LOG.md`

## Minimum Enforcement Model
The minimum honest model is policy-first, not tooling-first.

Rules:
- lane packets may read any shared-truth surface needed for context
- each shared-truth surface may have only one declared write-owning lane inside a wave
- implementation or harness lanes should avoid shared-truth writes unless that write is their declared slice
- truth-sync writes must happen after implementation or harness lanes land, not concurrently with them
- queue or run-log housekeeping remains exempt operational bookkeeping, but even bookkeeping should be emitted by one serial truth-sync pass where practical

This is read-wide or write-narrow because context remains broad while mutation stays intentionally narrow.

## Shared-Truth Surfaces Covered By The Rule
- `STATUS.md`
- gate docs under `docs/gates/`
- queue files under `docs/orchestration/`
- `docs/orchestration/WAVE-RUN-LOG.md`
- doc-batch files under `docs/orchestration/doc-batches/`
- root or cross-stream governance docs such as ADRs or `docs/contracts/CONTRACTS-OVERVIEW.md`

## Serial Truth-Sync Rule
When a wave uses multiple internal lanes, the safe order is:

1. implementation lanes land
2. harness or validator lanes confirm the touched slice
3. one truth-sync lane reconciles shared surfaces

The truth-sync lane may read all prior lane outputs, but it should be the only lane mutating shared-truth surfaces in that final reconciliation pass.

## Example Batch Plan

| Lane | Role | Writes | Serial relationship |
|------|------|--------|---------------------|
| `I1` | implementation | ADR text or research note anchors only | parallel-safe with `H1` and `D1` |
| `H1` | harness or validator | no shared-truth writes | parallel-safe with `I1` and `D1` |
| `D1` | docs-owner | one declared owner doc such as `LOCAL-DRIVER.md` | parallel-safe with `I1` and `H1` |
| `T1` | truth-sync | queue, run-log, doc-batch, or other shared-truth surfaces | serial-after `I1`, `H1`, and `D1` |

The important property is not the lane names. It is that `T1` is the only writer for shared-truth reconciliation.

## Recommendation
- Resolve guardrail check 7 as `adjust`: read-wide or write-narrow is enforceable as a governance rule now, even before tooling exists, if lane plans declare a single writer for every shared-truth surface they touch.
- Keep the enforcement claim at the policy layer for now. Do not imply CI or runtime checks that do not exist.
- Treat shared-truth write overlap as a diff-overrun or review failure under current Batch Mode, not as a new autonomous permission system.

## ADR Text Impact
- Clarify that ADR-025 lane packets inherit read-wide context but must declare narrow write ownership.
- Clarify that truth-sync remains serial authority even when multiple implementation lanes exist.
- Clarify that current enforcement is a documented orchestration rule, not a runtime permission engine.

## D1/G4 Claim Impact
- No D1 movement.
- No G4 movement.
- No Home Assistant proof claim.
- No physical-device evidence claim.

## Research Disclaimer
This note documents the minimum governance rule set only. It does not create runtime policy enforcement, CI gates, or autonomous lane scheduling.

## HANDOFF CARD
- Lane ID: O7
- Status: complete
- Files read: `docs/adr/ADR-025-dual-root-hardware-runway.md`; `docs/orchestration/LOCAL-DRIVER.md`; `docs/orchestration/BATCH-MODE.md`; `docs/orchestration/SYSTEM-QUEUE.md`; `docs/orchestration/WAVE-RUN-LOG.md`
- Files changed: `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-lane-packet-enforcement.md`
- Evidence produced: minimum lane-packet enforcement note
- Claims added: a policy-level read-wide or write-narrow rule now exists for ADR-025 discussion
- Claims explicitly not added: runtime enforcement, ADR promotion, D1 closure, G4 closure, hardware proof
- Validation: source-reference consistency review against current orchestration docs
- Residual risks: policy is documented, but manual reviewer discipline is still required until tooling exists
- Next safe follow-up, if any: record the final ADR-025 disposition against the documented evidence chain