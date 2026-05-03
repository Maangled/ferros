# DOC-BATCH-2026-05-03-HOMEHUB-LOCAL-FINDINGS-01

Status: continue
Segment: `REENTRY-HOMEHUB-LOCAL-FINDINGS-01`
Track: code
Date: 2026-05-03

## Segment Summary
This bounded local-execution segment ran the approved repo-local bring-up commands on `homelab001` under explicit operator authorization from `Maangled`, filled the local findings packet from actual captured outputs, recorded a bridge-agent visibility mismatch between `ferros-hub summary` and the `ferros agent` CLI, added a passive LAN neighbor-cache note, and kept all claim ceilings intact.

## Completed Lanes
- `F0` runbook execution authorization check
- `F1` local profile and hub command capture
- `F2` findings fill from captured output
- `F3` passive host and neighbor-cache observation note
- `D1` dependency and artifact hygiene review
- `R1` claim red-team review
- `T1` serial truth-sync and closeout

## Blocked Lanes
- None. The `ha-local-bridge` visibility mismatch is recorded as a non-blocking finding rather than a blocked lane.

## Evidence Produced
- `.local-artifacts/reentry-homehub-local-01/xtask-hub-runway.txt`
- `.local-artifacts/reentry-homehub-local-01/profile-init.txt`
- `.local-artifacts/reentry-homehub-local-01/profile-show.txt`
- `.local-artifacts/reentry-homehub-local-01/hub-summary.txt`
- `.local-artifacts/reentry-homehub-local-01/hub-prove-bridge.txt`
- `.local-artifacts/reentry-homehub-local-01/agent-list.txt`
- `.local-artifacts/reentry-homehub-local-01/agent-describe.txt`
- `.local-artifacts/reentry-homehub-local-01/hub-deny-demo.txt`
- `.local-artifacts/reentry-homehub-local-01/host-facts.txt`
- `.local-artifacts/reentry-homehub-local-01/command-exit-codes.tsv`
- `.local-artifacts/reentry-homehub-local-01/copied-hub-artifacts.txt`
- `.local-artifacts/reentry-homehub-local-01/simulated-local-bridge-artifact.json`
- `.local-artifacts/reentry-homehub-local-01/local-hub-state-snapshot.json`
- `.local-artifacts/reentry-homehub-local-01/local-onramp-proposal.json`
- `.local-artifacts/reentry-homehub-local-01/local-onramp-decision-receipt.json`

## Claims Added
- Actual local `ferros` profile init and show outputs are captured for `homelab001`.
- Actual local `ferros-hub` summary, prove-bridge, and deny-demo outputs are captured.
- The `.tmp/hub` JSON artifact set was copied into ignored local artifact storage and referenced in findings.
- The bridge-agent visibility discrepancy is now documented as a real local finding.
- Passive host and LAN neighbor-cache observations are captured as read-only context.

## Claims Explicitly Not Added
- No separate-host Home Assistant proof.
- No real bridge registration proof beyond local simulated rehearsal output.
- No device control.
- No Matter support.
- No packet-inspection or deep network telemetry.
- No D1 closure.
- No G4 closure.
- No launch-readiness claim.

## Truth Surfaces Touched
- docs/hardware/findings/FINDINGS-homelab001-local-bringup.md
- docs/orchestration/REENTRY-HOMEHUB-LOCAL-DEPENDENCY-HYGIENE.md
- docs/orchestration/REENTRY-HOMEHUB-LOCAL-CLAIM-REDTEAM.md
- docs/orchestration/WAVE-RUN-LOG.md
- docs/orchestration/doc-batches/DOC-BATCH-2026-05-03-HOMEHUB-LOCAL-FINDINGS-01.md

## Next Queued Segment
`REENTRY-HOMEHUB-LOCAL-AGENT-VISIBILITY-01`

This next segment should explain why `ferros-hub summary` reports `ha-local-bridge@0.1.0` while `ferros agent list` surfaces only `echo` and `timer` and `ferros agent describe ha-local-bridge` returns `unknown agent`.