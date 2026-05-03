# REENTRY-HOMEHUB-LOCAL-ORCH-01 Coordination Note

Status: Active
Date: 2026-05-03
Authority: docs/orchestration/LOCAL-DRIVER.md

## Mission
Run the next bounded repo-local re-entry segment for homelab001 local bring-up.

## Critical-Path Correction
- Track A critical path is now local `ferros` profile bring-up, local `ferros-hub` bring-up, and LAN-device onboarding planning on `homelab001`.
- Home Assistant is optional, onramp-oriented, and lower priority than profile or hub bring-up.
- Same-machine Home Assistant on `homelab001` is allowed as temporary local rehearsal if it helps hub or onramp work.
- Separate-host Home Assistant validation is deferred and is not a blocker for the current segment.
- Separate-host Home Assistant proof, G4 proof, launch proof, and isolated DUT-only hard power-cut proof are not produced in this segment.

## Local Facts Locked For This Segment
- Active DUT and local hub machine: `homelab001`
- Operator: `Maangled`
- Repo path: `/home/homelab001/apps/ferros`
- Local state path: `/home/homelab001/apps/ferros/.local-state`
- Local artifact path: `/home/homelab001/apps/ferros/.local-artifacts`
- Temporary Home Assistant mode: co-located on `homelab001` if used
- Separate-host Home Assistant validation: deferred
- Primary device interest: local LAN devices, Matter-capable devices if later implemented, and semi-smart mirrors as observed network devices

## Segment Lane Plan
1. `A0` local policy amendment and critical-path correction
2. `A1` local profile and hub runbook
3. `A2` LAN device onboarding scout
4. `A3` findings-template re-scope for homelab001 local bring-up
5. `B1` ADR-025 impact note for Home Assistant de-emphasis
6. `D1` dependency and artifact hygiene confirmation
7. `R1` claim red-team review after owner lanes land
8. `T1` serial truth-sync and run-log closeout

## Claim Ceiling
- No D1 closure.
- No G4 closure.
- No real separate-host Home Assistant proof.
- No launch-readiness claim.
- No Matter support claim unless separately implemented and proven.
- No LAN-device control claim unless separately implemented and proven.
- No packet-inspection or network-telemetry claim beyond explicit local observation paths.

## Expected Segment Outcome
- Repo-backed local bring-up instructions exist for `homelab001`.
- A local-only findings template exists for profile or hub bring-up without requiring separate-host Home Assistant.
- A first-pass LAN-device onboarding plan exists without inventing device control or Matter support.
- ADR-025 research notes reflect that Home Assistant is optional and deferred as separate-host proof.
- Dependency hygiene and claim boundaries remain explicit.

## HANDOFF CARD
- Lane ID: A0
- Status: complete
- Files read: docs/orchestration/LOCAL-DRIVER.md; docs/orchestration/BATCH-MODE.md; docs/orchestration/HARDWARE-QUEUE.md; docs/orchestration/SYSTEM-QUEUE.md; docs/orchestration/WAVE-QUEUE.md; docs/orchestration/WAVE-RUN-LOG.md; docs/orchestration/REENTRY-PHASE0-HARDWARE-READINESS-AUDIT.md; docs/hardware/pack-b-session-01-plan.md; docs/hardware/pack-b-session-01-command-map.md
- Files changed: docs/orchestration/REENTRY-HOMEHUB-LOCAL-ORCH-01.md
- Evidence produced: repo-backed segment plan and critical-path correction
- Claims added: homelab001 local bring-up is the active near-term Track A priority; Home Assistant is optional and deferred for separate-host proof
- Claims explicitly not added: gate movement, hardware evidence, separate-host Home Assistant proof, launch proof
- Validation: markdown structure and scope review
- Residual risks: real profile or hub session results still need to be captured in findings
- Next safe follow-up, if any: land the local bring-up runbook and findings template