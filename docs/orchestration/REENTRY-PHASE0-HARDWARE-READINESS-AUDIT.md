# REENTRY-PHASE0 Track A Hardware Readiness Audit

Status: complete (planning only, amended)
Date: 2026-05-03
Queue reference: HARDWARE-2026-04-30-05

## Scope
This audit checks Phase 0 readiness for Track A without running any hardware commands.

## Policy Amendment (Operator-Directed)
For immediate local bring-up on `homelab001`:
- Same-machine Home Assistant is allowed for Phase 0 local rehearsal and profile or hub work.
- Separate-machine HA validation is deferred to a later hardware-validation segment.
- Profile bring-up, hub visibility, and local device onboarding are prioritized over HA bridge proof.
- Hard DUT-only power-cut evidence is deferred when DUT and HA are co-located.

This amendment changes readiness sequencing only and does not add D1 or G4 closure claims.

## Resolved Local Facts
- Named Pack B DUT identity: `homelab001`
- Named HA host identity (temporary co-located): `homelab001`
- Named operator identity: `Maangled`
- Operator station identity: `homelab001` local shell
- DUT repo path: `/home/homelab001/apps/ferros`
- Persistent state path: `/home/homelab001/apps/ferros/.local-state`
- Artifact capture root path: `/home/homelab001/apps/ferros/.local-artifacts`
- Network note: `homelab001` on `192.168.50.234/24` (`enp4s0`)

## Exact Unresolved Placeholder Checklist
Source: docs/hardware/pack-b-session-01-plan.md
- [x] Pack B DUT name
- [x] Pack C HA host name
- [x] Operator station
- [x] Operator
- [x] DUT repo path
- [x] Persistent state path
- [x] Network note
- [ ] DUT-only power-cut method (deferred for co-located host mode)
- [x] Artifact capture root

Source: docs/hardware/pack-b-session-01-command-map.md
- [x] DUT repo path placeholder in command prefixes
- [x] Pack B DUT name placeholders in profile commands
- [ ] Agent name placeholder in describe/logs

## Operator-Ready Evidence Checklist for HARDWARE-2026-04-30-05
Required before execution:
- [x] Named Pack B x86_64 DUT is fixed.
- [x] Named Pack C HA host is fixed (co-location allowed under this amendment).
- [x] Named operator and operator station are fixed.
- [x] DUT repo path is fixed and accessible.
- [x] Artifact capture root is fixed and writable.
- [ ] Operator confirms attended session window.

Required in findings record:
- [ ] Session date and operator identity.
- [ ] Named DUT and host identifiers.
- [ ] Exact command transcript or equivalent command list.
- [ ] Profile init result and path.
- [ ] Profile show result summary.
- [ ] Failure notes and remaining gaps.

## Blocker Note
Track A profile or hub local execution is no longer blocked by identity placeholders.
Separate-host HA validation and DUT-only hard power-cut claims remain deferred.

Missing required facts:
- Operator-confirmed attended session window
- Concrete DUT-only power-cut method for a future separated-host validation segment

## Findings Template Non-Evidence Confirmation
The following files are templates only and remain non-evidence until real operator-attended sessions fill them:
- docs/hardware/findings/FINDINGS-pack-b-session-01-profile-baseline.md
- docs/hardware/findings/FINDINGS-pack-b-session-02-handoff-mirror.md
- docs/hardware/findings/FINDINGS-pack-c-session-01-ha-visibility.md

## Claims Added
- Exact unresolved placeholder inventory for Phase 0
- Explicit Track A blocker statement
- Operator-ready evidence checklist for HARDWARE-2026-04-30-05
- Operator-directed local-only policy amendment and resolved identity facts

## Claims Explicitly Not Added
- No hardware execution claim
- No physical-device evidence claim
- No Home Assistant proof claim
- No D1/G4 closure claim
- No separate-host HA validation claim

## HANDOFF CARD
- Lane ID: A1
- Status: complete
- Files read: docs/hardware/pack-b-session-01-plan.md; docs/hardware/pack-b-session-01-command-map.md; docs/hardware/findings/FINDINGS-pack-b-session-01-profile-baseline.md; docs/hardware/findings/FINDINGS-pack-b-session-02-handoff-mirror.md; docs/hardware/findings/FINDINGS-pack-c-session-01-ha-visibility.md; docs/orchestration/HARDWARE-QUEUE.md
- Files changed: docs/orchestration/REENTRY-PHASE0-HARDWARE-READINESS-AUDIT.md
- Evidence produced: Phase 0 readiness audit and blocker note
- Claims added: Placeholder checklist and operator evidence checklist
- Claims explicitly not added: hardware execution or gate movement
- Validation: anchor-to-checklist mapping review
- Residual risks: co-located HA mode cannot support isolated DUT-only power-cut proof; separate-host validation remains pending
- Next safe follow-up, if any: run local profile or hub bring-up on `homelab001` with artifacts captured under `.local-artifacts`, then defer HA bridge proof and hard power-cut to a later segment
