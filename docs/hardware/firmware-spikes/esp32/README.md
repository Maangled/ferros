# ESP32 Family Baseline

> Planning note only for the non-x86 family baseline packet. This file does not constitute physical ESP32 evidence, D1 evidence, G4 evidence, Home Assistant proof, or FERROS-native OS proof.

## Target summary

| Field | Value | Note |
|-------|-------|------|
| Family | `ESP32` | Current compressed peripheral family |
| Lane role | `FERROS-compatible ecosystem device, not a FERROS OS target` | Follows the compressed-lane rule set |
| Practical posture | `firmware-first peripheral family` | Parent-node control plane remains required |
| Preferred target shape | `ESP32 development board or narrow-purpose peripheral node` | Exact board or peripheral role remains unnamed tonight |
| Control plane | `x86_64 Fastest on homelab001` | Owns early identity, storage, and truth surfaces |
| Current evidence state | `docs-only baseline packet only` | No board session has happened |

## Claim ceiling

- This file is a docs-only compressed baseline.
- `S2` and `S6` stay federated to the x86_64 control plane.
- `S3` and `S5` stay optional and must not be silently upgraded into mandatory OS-style lanes.
- No physical board run, no Home Assistant proof, no D1 movement, no G4 movement, and no FERROS-native OS claim are added here.

## Lane baseline

| Lane | Status | Baseline note |
|------|--------|---------------|
| `S1` | `required` | Firmware boot and toolchain are unavoidable. |
| `S2` | `federated` | Persistent identity remains anchored in the parent node. |
| `S3` | `optional` | A single-purpose peripheral may not need a general agent registry. |
| `S4` | `required` | The runtime or event-loop model is the core technical question. |
| `S5` | `optional` | Physical I/O may exist, but not every board needs a distinct UX lane. |
| `S6` | `federated` | Storage and artifact retention should not be assumed locally. |
| `S7` | `required` | The realistic role is peripheral or bridge-facing device behavior. |
| `S8` | `required` | Evidence and claim ceilings still need an explicit ESP32 surface. |
| `S9` | `provisional` | Allowed only when a board-specific ignition or handoff packet is actually needed. |

## Milestone plan

| Milestone | Expected deliverable | Toolchain or wiring needed | Known unknowns |
|-----------|----------------------|----------------------------|----------------|
| `boot` | One ESP32 family board builds and boots the chosen firmware baseline with a repeatable flash path. | ESP32 board, firmware toolchain, serial or USB flash path, and parent-node observation surface. | Exact board type, SDK version, and flash discipline remain unnamed tonight. |
| `identify` | The board presents only a peripheral-specific identity to the x86_64 control plane rather than claiming independent profile ownership. | Parent-node control plane plus explicit peripheral registration or stand-in note later. | The exact registration boundary remains open. |
| `accept-grant` | Capability proposals and consent remain parent-node owned while the ESP32 consumes only the granted boundary needed for its narrow device role. | Existing x86_64 onramp and consent surfaces plus later peripheral-routing notes. | No ESP32-side consent execution is proven yet. |
| `report-state` | One narrow peripheral event or state report may later be surfaced through the parent node without implying a full HA or OS-grade runtime claim. | Peripheral firmware plus parent-node logging or bridge observation surface. | No real board output or HA-facing observation exists yet. |

## Current execution posture

| Milestone | Current posture | Why |
|-----------|-----------------|-----|
| `boot` | Defined only | No named ESP32 board is under test tonight |
| `identify` | Defined only | Control-plane attribution exists, but not board execution |
| `accept-grant` | Deferred | Consent remains parent-node owned for the compressed posture |
| `report-state` | Deferred | No board runtime or bridge observation exists yet |

## Sources held constant for this baseline

- `docs/adr/ADR-025-dual-root-hardware-runway.md`
- `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-family-lane-profiles.md`
- `docs/streams/STREAM-E-CORE-OS.md`
- `docs/research/S4-no-std-target-matrix.md`