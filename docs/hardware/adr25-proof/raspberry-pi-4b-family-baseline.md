# Raspberry Pi 4B ADR-025 Family Baseline Note

Status: docs-only family baseline note
Scope: Raspberry Pi 4B Pack A baseline only
Authority: ADR-022 plus the accepted ADR-025 framework and current family-profile research note
Constraint: this note does not create physical Raspberry Pi evidence or move D1 or G4

## Evidence inputs

- `docs/adr/ADR-025-dual-root-hardware-runway.md`
- `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-family-lane-profiles.md`
- `docs/hub/reference-hardware.md`
- `docs/hardware/d1-target-inventory.md`
- `docs/hardware/firmware-spikes/raspberry-pi-4b/README.md`

## What this wave establishes

- The repo now has a concrete docs-only Pack A baseline for Raspberry Pi family work.
- Pi Fastest posture is now written down as Linux-first with `S2` and `S6` federated upward to x86_64.
- Pi FERROS-side boot and native-runtime work remain explicitly deferred rather than silently missing.

## ADR-025 mapping

| ADR-025 proof dimension | Result from this baseline note | Bound imposed by this note |
|-------------------------|--------------------------------|----------------------------|
| Check 1 — lane sufficiency | Positive at the docs-only level: Pi now has an explicit required or federated lane map rather than a vague fallback label. | This is not physical Pi execution or family-level operational closure. |
| Check 5 — control-plane witness boundary | x86_64 is explicitly named as the early identity and storage owner for Pi work. | No cross-family claim upgrade is earned from that control-plane role alone. |
| Check 6 — claim-boundary enforcement | The baseline note keeps Pi at planning posture and repeats the surviving non-claims. | No D1, G4, HA, or native-runtime claim is added here. |

## Surviving non-claims

- No physical Raspberry Pi run.
- No Home Assistant proof.
- No D1 or G4 movement.
- No independent Pi identity or storage ownership yet.
- No FERROS-native boot or runtime proof.

## Closure impact

- Safe closure movement earned now: none.
- Honest advancement earned now: the queue now has a concrete Pi family baseline packet that can later route into a real Pack A board session without improvising lane ownership.
- Next proof edge: a named Raspberry Pi board plus one real baseline findings packet with explicit x86_64 attribution for any federated surfaces.