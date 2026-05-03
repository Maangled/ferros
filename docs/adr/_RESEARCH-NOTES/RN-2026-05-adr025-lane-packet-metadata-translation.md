# RN-2026-05 ADR-025 Lane-Packet Metadata Translation

Status: Draft
Scope: Research-only
Constraint: ADR-025 remains Proposed and non-binding.

## Question
- How do the current queue metadata fields map into ADR-025 lane packets so the overlay pilot preserves the current scheduling and truth-sync discipline?

## Evidence Reviewed
- `docs/orchestration/LOCAL-DRIVER.md`
- `docs/orchestration/BATCH-MODE.md`
- `docs/orchestration/WAVE-QUEUE.md`
- `docs/orchestration/SYSTEM-QUEUE.md`
- `docs/orchestration/HARDWARE-QUEUE.md`
- `docs/adr/ADR-025-dual-root-hardware-runway.md`
- `docs/orchestration/ADR-025-X86-OVERLAY-PILOT-01.md`

## One-To-One Mappings

### `size`
- Current meaning: bounded wave scope (`S` versus `L`).
- Pilot translation: lane packet scope remains `S` or `L` with the same practical meaning.
- Pilot verdict: one-to-one.

### `parallel-safe-with`
- Current meaning: explicit non-overlap and parallel safety.
- Pilot translation: lane packet parallel-safety declaration remains explicit and should continue to list named compatible packets.
- Pilot verdict: one-to-one inside the same live authority model.

### `serial-after`
- Current meaning: explicit sequencing dependency.
- Pilot translation: lane packet dependency remains explicit and should keep the same semantics.
- Pilot verdict: one-to-one for normal waves; `S9` reroute proposals may need an additional handoff trigger concept but should not replace `serial-after` for ordinary work.

### `solo`
- Current meaning: wave must run alone because it touches truth surfaces or credibility-sensitive work.
- Pilot translation: lane packet isolation flag remains required for shared-truth, gate, or schema-sensitive work.
- Pilot verdict: one-to-one.

### `track`
- Current meaning: `code`, `system`, or `hardware` queue ownership.
- Pilot translation: lane packets still need a track because the current queue files remain the schedulable authority.
- Pilot verdict: one-to-one.

## Non-Equivalent Behavior
- ADR-025 adds root and hardware-family context on top of current `track`, but that context should remain descriptive during the pilot rather than becoming a new scheduling authority.
- `S9` reroute packets may depend on findings or handoffs rather than only on linear predecessor waves.
- If ADR-025 later introduces root-level shared surfaces such as `HANDOFFS.md`, those surfaces will need bookkeeping treatment similar to current queue and run-log files.

## Enforcement Gaps
- The current queue schema does not yet carry explicit `hardware_family` or `root` fields.
- The current stop-condition model does not define dedicated ignition-lane handoff semantics.
- The current bookkeeping exemption list does not yet account for future root-level shared surfaces because those surfaces do not exist yet.
- Read-wide or write-narrow policy is still governance language; tooling or validator enforcement remains to be proven.

## Recommendation
- Keep the current queue metadata model intact during the overlay pilot.
- Treat root and hardware-family attributes as descriptive research-note content until a later governance pass decides whether they should become queue fields.
- If ADR-025 advances later, add root-aware metadata only after explicit validator and bookkeeping rules are written.
- Use this note to constrain future migration rather than to mutate Batch Mode now.

## ADR Text Impact
- Clarify that ADR-025 overlay packets inherit current queue semantics rather than replacing them during the pilot.
- Clarify that any future root-aware metadata requires explicit validator and bookkeeping rules before becoming authoritative.
- Clarify that `S9` handoff triggers are additive and do not eliminate ordinary queue sequencing.

## D1/G4 Claim Impact
- No D1 movement.
- No G4 movement.
- No Home Assistant proof claim.
- No physical-device evidence claim.

## Research Disclaimer
This note translates current queue metadata into an ADR-025 pilot framing only. It does not change Batch Mode, queue authority, validator behavior, or gate truth.

## HANDOFF CARD
- Lane ID: O3
- Status: complete
- Files read: `docs/orchestration/LOCAL-DRIVER.md`; `docs/orchestration/BATCH-MODE.md`; `docs/orchestration/WAVE-QUEUE.md`; `docs/orchestration/SYSTEM-QUEUE.md`; `docs/orchestration/HARDWARE-QUEUE.md`; `docs/adr/ADR-025-dual-root-hardware-runway.md`; `docs/orchestration/ADR-025-X86-OVERLAY-PILOT-01.md`
- Files changed: `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-lane-packet-metadata-translation.md`
- Evidence produced: metadata translation note for the ADR-025 overlay pilot
- Claims added: current queue metadata can be carried forward unchanged during the pilot while root-aware fields remain deferred
- Claims explicitly not added: Batch Mode mutation, new queue authority, ADR promotion, gate movement
- Validation: source-reference consistency review against current queue and orchestration docs
- Residual risks: root-aware metadata and S9 handoff semantics remain deferred governance questions
- Next safe follow-up, if any: close out the four-wave pilot packet and record the preserved non-claims