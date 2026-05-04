# ESP32 ADR-025 Family Baseline Note

Status: docs-only family baseline note
Scope: ESP32 compressed peripheral baseline only
Authority: ADR-022 plus the accepted ADR-025 framework and current family-profile research note
Constraint: this note does not create physical ESP32 evidence or move D1 or G4

## Evidence inputs

- `docs/adr/ADR-025-dual-root-hardware-runway.md`
- `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-family-lane-profiles.md`
- `docs/streams/STREAM-E-CORE-OS.md`
- `docs/research/S4-no-std-target-matrix.md`
- `docs/hardware/firmware-spikes/esp32/README.md`

## What this wave establishes

- The repo now has a concrete docs-only ESP32 baseline instead of treating the peripheral family as an unshaped footnote.
- ESP32 compression is now explicit: `S1`, `S4`, `S7`, and `S8` are active; `S2` and `S6` are federated upward; `S3` and `S5` remain optional.
- ESP32 stays outside any FERROS-native OS overclaim.

## ADR-025 mapping

| ADR-025 proof dimension | Result from this baseline note | Bound imposed by this note |
|-------------------------|--------------------------------|----------------------------|
| Check 1 — lane sufficiency | Positive at the docs-only level: ESP32 now has an explicit minimum lane profile rather than an implied omission. | This is not physical ESP32 execution or family-level operational closure. |
| Check 4 — embedded-device compression | Positive at the docs-only level: the compressed profile is now written as active, federated, and optional lanes rather than hand-waving. | This does not prove the firmware or event-loop design works on hardware. |
| Check 6 — claim-boundary enforcement | The baseline note keeps ESP32 in peripheral posture and repeats the surviving non-claims. | No D1, G4, HA, or FERROS-native OS claim is added here. |

## Surviving non-claims

- No physical ESP32 run.
- No Home Assistant proof.
- No D1 or G4 movement.
- No independent ESP32 identity or storage ownership yet.
- No FERROS-native OS proof.

## Closure impact

- Safe closure movement earned now: none.
- Honest advancement earned now: the queue now has a concrete compressed-family baseline packet that can later route into a real peripheral board session without inventing lane ownership on the fly.
- Next proof edge: a named ESP32 board plus one real firmware or peripheral findings packet with explicit parent-node attribution.