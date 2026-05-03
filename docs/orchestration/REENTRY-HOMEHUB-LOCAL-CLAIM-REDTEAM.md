# REENTRY-HOMEHUB-LOCAL-ORCH-01 Claim Red-Team Summary

Status: complete
Date: 2026-05-03
Role: R1 risk and claim rationalizer

## Inputs Reviewed
- docs/orchestration/REENTRY-HOMEHUB-LOCAL-ORCH-01.md
- docs/hardware/homelab001-local-bringup-runbook.md
- docs/hub/local-lan-device-onboarding.md
- docs/hardware/findings/FINDINGS-homelab001-local-bringup.md
- docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-homehub-local-critical-path.md
- docs/orchestration/REENTRY-HOMEHUB-LOCAL-DEPENDENCY-HYGIENE.md
- docs/orchestration/REENTRY-PHASE0-HARDWARE-READINESS-AUDIT.md
- docs/orchestration/HARDWARE-QUEUE.md
- STATUS.md
- docs/adr/ADR-025-dual-root-hardware-runway.md

## Verdict
Current segment wording stays inside the requested claim ceiling after the local bridge-agent-name truth-sync.

## Red-Team Checks
- Same-machine Home Assistant is described only as optional local rehearsal.
- Profile or hub local bring-up is not described as D1 or G4 proof.
- LAN-device planning does not claim device control, Matter support, or deep network telemetry.
- ADR-025 remains Proposed and non-binding.
- Local command validation is not described as physical-device or launch proof.

## Required Follow-Up Edits
- None.

## Claims Added
- homelab001 local bring-up is the active repo-local priority surface.
- exact local runbook commands and capture paths now exist.
- a local-only findings template exists for profile or hub bring-up.
- LAN-device onboarding is framed as local observation-first proposed material.
- ADR-025 research now reflects Home Assistant de-emphasis without promoting the ADR.

## Claims Explicitly Not Added
- No separate-host Home Assistant proof.
- No device-control claim.
- No Matter-support claim.
- No packet-inspection or deep-telemetry claim.
- No D1 closure.
- No G4 closure.
- No launch-readiness claim.
- No ADR-025 promotion.

## HANDOFF CARD
- Lane ID: R1
- Status: complete
- Files read: all owner-lane outputs for this segment plus STATUS.md, HARDWARE-QUEUE.md, and ADR-025
- Files changed: docs/orchestration/REENTRY-HOMEHUB-LOCAL-CLAIM-REDTEAM.md
- Evidence produced: claim red-team summary for the local homehub segment
- Claims added: claim-boundary confirmation for current wording
- Claims explicitly not added: gate movement, separate-host Home Assistant proof, device control, Matter support, ADR promotion
- Validation: cross-document claim consistency review
- Residual risks: operator-attended findings are still required before any evidence-carrying follow-up segment
- Next safe follow-up, if any: serial truth-sync and stop-clean handoff for findings fill

## Addendum — REENTRY-HOMEHUB-LOCAL-FINDINGS-01

Status: complete
Date: 2026-05-03
Role: R1 risk and claim rationalizer

### Inputs Reviewed
- docs/hardware/findings/FINDINGS-homelab001-local-bringup.md
- docs/hardware/homelab001-local-bringup-runbook.md
- .local-artifacts/reentry-homehub-local-01/command-exit-codes.tsv
- .local-artifacts/reentry-homehub-local-01/host-facts.txt
- .local-artifacts/reentry-homehub-local-01/agent-list.txt
- .local-artifacts/reentry-homehub-local-01/agent-describe.txt

### Verdict
The filled findings remain inside the requested claim ceiling. The segment records actual local outputs, preserves the `ha-local-bridge` visibility mismatch as a failure, and keeps the passive LAN note within read-only neighbor-cache observation.

### Red-Team Checks
- Local command success is not described as D1 or G4 proof.
- The `ha-local-bridge` mismatch is documented as unresolved rather than normalized away.
- No Home Assistant process interaction is framed as separate-host proof.
- The passive LAN note is limited to `ip neigh show` cache visibility and does not claim service discovery, device control, Matter support, or deep telemetry.
- `deny-demo` output is described only as local-only, non-evidentiary behavior.

### Required Follow-Up Edits
- None.

### Claims Added
- Actual homelab001 local bring-up outputs are now captured in the findings packet.
- The copied `.tmp/hub` artifact set and command exit ledger are recorded.
- The bridge-agent visibility discrepancy is documented as an unresolved local finding.
- Passive host facts and neighbor-cache observation are recorded as read-only context.

### Claims Explicitly Not Added
- No separate-host Home Assistant proof.
- No device-control claim.
- No Matter-support claim.
- No packet-inspection or deep-telemetry claim.
- No D1 closure.
- No G4 closure.
- No launch-readiness claim.

## HANDOFF CARD — REENTRY-HOMEHUB-LOCAL-FINDINGS-01
- Lane ID: R1
- Status: complete
- Files read: docs/hardware/findings/FINDINGS-homelab001-local-bringup.md; docs/hardware/homelab001-local-bringup-runbook.md; .local-artifacts/reentry-homehub-local-01/command-exit-codes.tsv; .local-artifacts/reentry-homehub-local-01/host-facts.txt; .local-artifacts/reentry-homehub-local-01/agent-list.txt; .local-artifacts/reentry-homehub-local-01/agent-describe.txt
- Files changed: docs/orchestration/REENTRY-HOMEHUB-LOCAL-CLAIM-REDTEAM.md
- Evidence produced: claim-boundary review addendum for the findings execution segment
- Claims added: the findings wording is confirmed honest against the captured outputs
- Claims explicitly not added: D1 or G4 movement, Home Assistant proof, device control, Matter support, launch readiness
- Validation: cross-check between findings wording and raw capture artifacts
- Residual risks: the bridge-agent visibility mismatch remains unresolved
- Next safe follow-up, if any: queue `REENTRY-HOMEHUB-LOCAL-AGENT-VISIBILITY-01`