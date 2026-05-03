# DOC-BATCH-2026-05-03-HOMEHUB-LOCAL-01

Status: stop-clean closeout
Segment: `REENTRY-HOMEHUB-LOCAL-ORCH-01`
Track: code
Date: 2026-05-03

## Segment Summary
This bounded repo-local segment corrected the near-term Track A critical path toward local `ferros` profile and `ferros-hub` bring-up on `homelab001`, added a local bring-up runbook and findings template, added a LAN-device onboarding planning note that does not depend on Home Assistant, updated ADR-025 research for Home Assistant de-emphasis, confirmed dependency hygiene, and stopped cleanly after repo-local prep because operator-attended command output is still required before findings can be filled honestly.

## Completed Lanes
- `A0` local policy amendment and critical-path correction
- `A1` local profile and hub runbook
- `A2` LAN-device onboarding scout
- `A3` findings-template re-scope
- `B1` ADR-025 impact note
- `D1` dependency and artifact hygiene review
- `R1` claim red-team review
- `T1` serial truth-sync and run-log closeout

## Evidence Produced
- `cargo test -p ferros-node run_dispatches_profile_init_and_show_with_explicit_path`
- `cargo run -p ferros-hub -- summary`
- `cargo run -p ferros-hub -- prove-bridge`
- `cargo xtask hub-runway --keep-artifacts`
- clean `get_errors` on all touched docs in this segment

## Claims Added
- homelab001 local bring-up is the active repo-local Track A priority.
- exact repo-backed commands and capture paths now exist for local profile and hub bring-up.
- a local findings template exists for homelab001 local bring-up.
- LAN-device onboarding is planned as local observation-first proposed material.
- ADR-025 research now records that Home Assistant is optional and deferred as separate-host proof.

## Claims Explicitly Not Added
- No separate-host Home Assistant proof.
- No device control.
- No Matter support.
- No packet-inspection or deep network telemetry.
- No D1 closure.
- No G4 closure.
- No launch-readiness claim.
- No ADR-025 promotion.

## Truth Surfaces Touched
- docs/orchestration/REENTRY-HOMEHUB-LOCAL-ORCH-01.md
- docs/orchestration/REENTRY-HOMEHUB-LOCAL-DEPENDENCY-HYGIENE.md
- docs/orchestration/REENTRY-HOMEHUB-LOCAL-CLAIM-REDTEAM.md
- docs/orchestration/WAVE-RUN-LOG.md
- docs/orchestration/doc-batches/DOC-BATCH-2026-05-03-HOMEHUB-LOCAL-01.md

## Next Queued Segment
`REENTRY-HOMEHUB-LOCAL-FINDINGS-01`

This next segment should fill [ferros/docs/hardware/findings/FINDINGS-homelab001-local-bringup.md](ferros/docs/hardware/findings/FINDINGS-homelab001-local-bringup.md) after operator-attended command output is captured on `homelab001`.