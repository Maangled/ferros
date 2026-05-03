# RN-2026-05 ADR-025 S9 Provisional Service Packet

Status: Draft
Scope: Research-only
Constraint: ADR-025 remains Proposed and non-binding.

## Question
- Does `S9` provide a non-redundant function distinct from `S8` truth-sync?
- What should a provisional ignition packet be allowed to do during the x86_64 overlay pilot?

## Evidence Reviewed
- `docs/adr/ADR-025-dual-root-hardware-runway.md`
- `docs/orchestration/LOCAL-DRIVER.md`
- `docs/orchestration/BATCH-MODE.md`
- `docs/orchestration/ADR-025-X86-OVERLAY-PILOT-01.md`
- `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-guardrails-batch1.md`
- `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-guardrails-batch2.md`

## Concrete Non-Redundant S9 Function
`S8` records what became true inside the current authority model.

`S9` proposes what should reload, reroute, or be re-scoped because something new was learned.

In the overlay pilot, the minimum honest `S9` function is:
- detect a finding or architecture constraint that current `S1-S8` truth-sync alone does not route,
- classify whether the implication belongs to `Fastest`, `FERROS`, or both,
- produce a handoff or reload packet for later queued work,
- stay proposal-only and non-destructive.

## Allowed Triggers, Inputs, And Outputs

### Allowed Triggers
- a completed `Fastest` finding that implies a new `FERROS` research question,
- a completed `FERROS` research finding that implies a new `Fastest` boundary or abstraction,
- a cross-root discrepancy that needs explicit routing,
- a change in lane shape or sequencing that should be proposed but not yet enforced.

### Allowed Inputs
- existing findings or research notes,
- run-log and queue context,
- current lane-map and metadata-translation notes,
- current `S1-S8` authority surfaces.

### Allowed Outputs
- reload notes,
- reroute notes,
- handoff packets,
- board-specific ignition prompts,
- proposed future queue items.

### Forbidden Outputs
- direct edits to live S1-S8 owner surfaces outside the declared S9 packet,
- truth-sync replacement,
- queue authority replacement,
- gate movement,
- hardware or Home Assistant proof claims,
- background-autonomy claims.

## Authority Boundary Versus S8

| Surface | S8 | S9 |
|---------|----|----|
| Primary role | truth-sync and claim ceiling | reload, reroute, and handoff proposal |
| Writes | shared truth surfaces and research bookkeeping | ignition notes and handoff packets only |
| Decision scope | what is true now | what should be reconsidered next |
| Gate authority | may record gate truth, not move it alone | may not move or imply gates |
| Queue authority | may reconcile queues as truth surfaces | may only propose queued follow-up |

Operational rule:
- `S9` may observe and propose across roots.
- `S8` remains the serial authority for shared truth.

## Example Packet Flow
- `Fastest` finding: x86_64 local hub restart behavior depends on current Linux-host assumptions.
- `S8` records the finding in the current truth surfaces.
- `S9` emits a handoff packet that says: this should reload `x86_64/FERROS` runtime research around native supervisor primitives and reload `x86_64/Fastest` storage-boundary planning so restart assumptions stay abstracted.
- The next actual edit or implementation work still happens through normal queued waves.

## Recommendation
- Keep `S9` provisional.
- Allow `S9` only as an observation-and-proposal service lane during the overlay pilot.
- Require every `S9` packet to cite the finding or note that triggered it and to state what `S8` did not already answer.
- Keep `S9` outputs as inputs to later queued work rather than live authority changes.

## ADR Text Impact
- Clarify in ADR-025 that `S9` is distinct from `S8` because it routes discoveries between roots while `S8` maintains shared truth.
- Clarify that `S9` outputs are proposal packets, not direct lane edits or gate actions.
- Clarify that one non-redundant S9 packet is required before any discussion of making `S9` permanent.

## D1/G4 Claim Impact
- No D1 movement.
- No G4 movement.
- No Home Assistant proof claim.
- No physical-device evidence claim.
- No FERROS-native OS claim.

## Research Disclaimer
This note defines a provisional S9 service packet for the overlay pilot. It does not promote ADR-025, authorize new hardware-root authority, replace S8 truth-sync, or imply continuous autonomous lane execution.

## HANDOFF CARD
- Lane ID: O2
- Status: complete
- Files read: `docs/adr/ADR-025-dual-root-hardware-runway.md`; `docs/orchestration/LOCAL-DRIVER.md`; `docs/orchestration/BATCH-MODE.md`; `docs/orchestration/ADR-025-X86-OVERLAY-PILOT-01.md`; `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-guardrails-batch1.md`; `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-guardrails-batch2.md`
- Files changed: `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-s9-provisional-service-packet.md`
- Evidence produced: provisional S9 packet note with explicit non-redundancy and authority-boundary language
- Claims added: candidate S9 function for reload and reroute proposals
- Claims explicitly not added: ADR promotion, stream retirement, gate movement, hardware evidence, background autonomy
- Validation: source-reference consistency review against ADR-025 and current orchestration docs
- Residual risks: practical S9 packet enforcement and queue integration remain unproven
- Next safe follow-up, if any: land the metadata translation note and then close out the overlay pilot batch honestly