# DOC-BATCH-2026-05-03-HARDWARE-ADR025-PROOF-01

Status: stop-clean closeout
Segment: `HARDWARE-ADR025-PROOF-01`
Track: hardware
Date: 2026-05-03

## Segment Summary
This user-directed hardware-track serial run drained the currently safe hardware proof packets on `homelab001`: the docs-only firmware-spike target plan, the first Pack B `x86_64` profile-baseline session, and the narrow ADR-025 proof note that maps that baseline into framework terms. The run stopped before the reboot-dependent and separate-host-dependent waves so the queue state and claim ceiling stayed honest.

## Completed Lanes
- `HARDWARE-2026-04-27-02` firmware-spike target plan for `homelab001`
- `HARDWARE-2026-04-30-05` first Pack B `x86_64` physical baseline on `homelab001`
- `HARDWARE-2026-05-03-08` ADR-025 proof note for the completed Pack B baseline

## Blocked Lanes
- `HARDWARE-2026-04-30-06` requires an operator-approved reboot window on `homelab001` and was not triggered from the current agent-run session.
- `HARDWARE-2026-04-30-07` requires a real separate Pack C Home Assistant host on the same LAN and remains blocked in co-located mode.

## Evidence Produced
- `docs/hardware/firmware-spikes/homelab001/README.md`
- `docs/hardware/findings/FINDINGS-pack-b-session-01-profile-baseline.md`
- `docs/hardware/adr25-proof/pack-b-session-01-x86-proof.md`

## Claims Added
- The chosen D1 device now has a concrete firmware-spike milestone plan anchored to `homelab001`.
- The repo now contains the first findings-backed Pack B `x86_64` physical baseline for `ferros profile init` and `ferros profile show` on the named DUT.
- ADR-025 now has a narrow x86_64 operational proof note tied to real Pack B findings rather than only research notes.

## Claims Explicitly Not Added
- No D1 closure.
- No G4 closure.
- No Home Assistant proof.
- No reboot-safe or full power-cycle survival proof.
- No separate-host Pack C visibility proof.
- No Pi, Jetson, or ESP32 family proof.
- No FERROS-native runtime proof.

## Truth Surfaces Touched
- `docs/hardware/firmware-spikes/homelab001/README.md`
- `docs/hardware/findings/FINDINGS-pack-b-session-01-profile-baseline.md`
- `docs/hardware/adr25-proof/pack-b-session-01-x86-proof.md`
- `docs/orchestration/HARDWARE-QUEUE.md`
- `docs/orchestration/WAVE-RUN-LOG.md`
- `docs/orchestration/doc-batches/DOC-BATCH-2026-05-03-HARDWARE-ADR025-PROOF-01.md`

## Next Queued Segment
`HARDWARE-2026-04-30-06` is the next queue item, but only once an operator-approved reboot window exists. `HARDWARE-2026-04-30-07` remains blocked until a separate Pack C Home Assistant host exists on the same LAN.