# DOC-BATCH-2026-05-03-HARDWARE-NONX86-FAMILY-BASELINES-01

Status: stop-clean closeout
Segment: `HARDWARE-NONX86-FAMILY-BASELINES-01`
Track: hardware
Date: 2026-05-03

## Segment Summary
This user-directed hardware-track serial run queued and completed the first docs-only non-x86 family baselines under the accepted ADR-025 framework: Raspberry Pi 4B, Jetson Orin Nano, and ESP32. Each family now has a concrete baseline packet with target summary, lane posture, control-plane attribution, and explicit non-claims. The run stayed docs-only and did not claim any physical board execution.

## Completed Lanes
- `HARDWARE-2026-05-03-09` Raspberry Pi 4B docs-only family baseline
- `HARDWARE-2026-05-03-10` Jetson Orin Nano docs-only family baseline
- `HARDWARE-2026-05-03-11` ESP32 docs-only compressed family baseline

## Blocked Lanes
- `HARDWARE-2026-04-30-06` remains blocked on an operator-approved reboot window on `homelab001`.
- `HARDWARE-2026-04-30-07` remains blocked on a real separate Pack C Home Assistant host on the same LAN.

## Evidence Produced
- `docs/hardware/firmware-spikes/raspberry-pi-4b/README.md`
- `docs/hardware/adr25-proof/raspberry-pi-4b-family-baseline.md`
- `docs/hardware/firmware-spikes/jetson-orin-nano/README.md`
- `docs/hardware/adr25-proof/jetson-orin-nano-family-baseline.md`
- `docs/hardware/firmware-spikes/esp32/README.md`
- `docs/hardware/adr25-proof/esp32-family-baseline.md`

## Claims Added
- Raspberry Pi 4B now has an explicit docs-only Pack A baseline with federated identity and storage boundaries.
- Jetson Orin Nano now has an explicit docs-only family baseline with vendor-image dependency and federated identity and storage boundaries.
- ESP32 now has an explicit docs-only compressed peripheral baseline with active, federated, and optional lanes called out directly.

## Claims Explicitly Not Added
- No physical-device evidence for Pi, Jetson, or ESP32.
- No Home Assistant proof.
- No D1 closure.
- No G4 closure.
- No FERROS-native OS proof.
- No claim that non-x86 families are ready to bypass x86_64 control-plane attribution.

## Truth Surfaces Touched
- `docs/hardware/firmware-spikes/raspberry-pi-4b/README.md`
- `docs/hardware/adr25-proof/raspberry-pi-4b-family-baseline.md`
- `docs/hardware/firmware-spikes/jetson-orin-nano/README.md`
- `docs/hardware/adr25-proof/jetson-orin-nano-family-baseline.md`
- `docs/hardware/firmware-spikes/esp32/README.md`
- `docs/hardware/adr25-proof/esp32-family-baseline.md`
- `docs/orchestration/HARDWARE-QUEUE.md`
- `docs/orchestration/WAVE-RUN-LOG.md`
- `docs/orchestration/doc-batches/DOC-BATCH-2026-05-03-HARDWARE-NONX86-FAMILY-BASELINES-01.md`

## Next Queued Segment
No new Ready hardware items were left behind by this batch. The next real hardware moves remain the already-blocked `HARDWARE-2026-04-30-06` reboot-boundary packet and `HARDWARE-2026-04-30-07` separate-host HA packet, plus any future physical Pi, Jetson, or ESP32 board session once named hardware exists.