# DOC-BATCH-2026-05-03-ADR025-X86-OVERLAY-01

Status: stop-clean closeout
Segment: `ADR-025-X86-OVERLAY-PILOT-01`
Track: system
Date: 2026-05-03

## Segment Summary
This bounded system-track segment completed the first four ADR-025 x86_64 overlay pilot waves: the authority lock, the x86_64 Fastest or FERROS lane map, the provisional S9 service-packet note, and the queue-metadata translation note. The segment kept ADR-025 Proposed, preserved the live S1-S8 plus gate plus queue substrate, and turned the pilot from chat-only intent into queued and executed repo surfaces.

## Completed Lanes
- `O0` overlay coordination lock
- `O1` x86_64 Fastest or FERROS lane map
- `O2` provisional S9 ignition non-redundancy packet
- `O3` lane-packet metadata translation note

## Blocked Lanes
- None. The overlay pilot surfaced unresolved governance questions, but none blocked honest completion of the declared four-wave packet.

## Evidence Produced
- `docs/orchestration/ADR-025-X86-OVERLAY-PILOT-01.md`
- `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-x86-overlay-lane-map.md`
- `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-s9-provisional-service-packet.md`
- `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-lane-packet-metadata-translation.md`

## Claims Added
- The x86_64 ADR-025 overlay pilot now exists as an explicit repo-backed packet.
- The current S1-S8 stream model has an explicit candidate crosswalk into `x86_64/Fastest` and `x86_64/FERROS` lanes.
- `S9` now has a provisional non-redundant role definition as a reload or reroute proposal lane distinct from `S8` truth-sync.
- Current queue metadata now has an explicit translation note for the overlay pilot without mutating live Batch Mode semantics.

## Claims Explicitly Not Added
- No ADR-025 promotion.
- No S1-S8 retirement.
- No queue-authority replacement.
- No D1 closure.
- No G4 closure.
- No Home Assistant proof.
- No physical-device evidence.
- No FERROS-native OS claim.
- No background-autonomy claim.

## Truth Surfaces Touched
- `docs/orchestration/ADR-025-X86-OVERLAY-PILOT-01.md`
- `docs/orchestration/SYSTEM-QUEUE.md`
- `docs/orchestration/WAVE-RUN-LOG.md`
- `docs/orchestration/doc-batches/DOC-BATCH-2026-05-03-ADR025-X86-OVERLAY-01.md`

## Next Queued Segment
None in `docs/orchestration/SYSTEM-QUEUE.md`.

The next safe follow-up is a new queued disposition or implementation packet if the user wants to move from overlay pilot notes into either validator enforcement or root-surface scaffolding.