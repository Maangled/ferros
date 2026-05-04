# Jetson Orin Nano ADR-025 Family Baseline Note

Status: docs-only family baseline note
Scope: Jetson Orin Nano baseline only
Authority: ADR-022 plus the accepted ADR-025 framework and current family-profile research note
Constraint: this note does not create physical Jetson evidence or move D1 or G4

## Evidence inputs

- `docs/adr/ADR-025-dual-root-hardware-runway.md`
- `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-family-lane-profiles.md`
- `docs/hub/reference-hardware.md`
- `docs/hardware/firmware-spikes/jetson-orin-nano/README.md`

## What this wave establishes

- The repo now has a concrete docs-only Jetson family baseline instead of only a passing mention in the hardware-root discussion.
- Jetson Fastest posture is now written down as Linux-first with federated identity and storage plus explicit vendor-image dependency.
- Jetson FERROS-side boot and runtime ownership remain deferred behind x86_64 FERROS progress.

## ADR-025 mapping

| ADR-025 proof dimension | Result from this baseline note | Bound imposed by this note |
|-------------------------|--------------------------------|----------------------------|
| Check 1 — lane sufficiency | Positive at the docs-only level: Jetson now has an explicit lane baseline rather than a generic future family label. | This is not physical Jetson execution or family-level operational closure. |
| Check 5 — control-plane witness boundary | x86_64 is explicitly named as the early identity and storage owner for Jetson work. | No cross-family claim upgrade is earned from that control-plane role alone. |
| Check 6 — claim-boundary enforcement | The baseline note keeps Jetson at planning posture and preserves vendor-image and non-claim language. | No D1, G4, HA, or native-runtime claim is added here. |

## Surviving non-claims

- No physical Jetson run.
- No Home Assistant proof.
- No D1 or G4 movement.
- No independent Jetson identity or storage ownership yet.
- No FERROS-native boot or runtime proof.

## Closure impact

- Safe closure movement earned now: none.
- Honest advancement earned now: the queue now has a concrete Jetson family baseline packet that can later become a real board session without inventing lane ownership on the fly.
- Next proof edge: a named Jetson board plus one real baseline findings packet with explicit control-plane attribution and vendor-image notes.