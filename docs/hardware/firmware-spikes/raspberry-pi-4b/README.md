# Raspberry Pi 4B Family Baseline

> Planning note only for the non-x86 family baseline packet. This file does not constitute physical Raspberry Pi evidence, D1 evidence, G4 evidence, Home Assistant proof, or FERROS-native OS proof.

## Target summary

| Field | Value | Note |
|-------|-------|------|
| Family | `Raspberry Pi 4B` | Current Pack A migration family |
| Lane role | `secondary D1 candidate and ADR-025 migration family` | Follows the x86_64-first path rather than replacing it |
| Practical posture | `Linux-first edge migration family` | Matches the family-lane profile note |
| Preferred target shape | `Raspberry Pi 4B or Pi 5 with 64-bit Linux and persistent SSD when possible` | Physical board still to be named later |
| Control plane | `x86_64 Fastest on homelab001` | Owns early identity, storage, and truth surfaces |
| Current evidence state | `docs-only baseline packet only` | No physical Pi session has happened |

## Claim ceiling

- This file is a docs-only baseline.
- `S2` and `S6` stay federated to the x86_64 control plane during early Pi runway work.
- Pi FERROS-side boot and native-runtime work remain deferred behind x86_64 FERROS progress.
- No physical-device run, no Home Assistant proof, no D1 movement, no G4 movement, and no FERROS-native OS claim are added here.

## Lane baseline

| Lane | Status | Baseline note |
|------|--------|---------------|
| `S1` | `required` | Linux boot and board prep are first-class Pi work. |
| `S2` | `federated` | Early identity remains anchored on the x86_64 control plane. |
| `S3` | `required` | Pi may host local room or kiosk agents. |
| `S4` | `required` | Linux runtime now; native runtime later. |
| `S5` | `required` | Pi remains a plausible display or kiosk surface. |
| `S6` | `federated` | Early artifacts and ledger writes stay on x86_64. |
| `S7` | `required` | Pi remains a credible Home Assistant-facing edge node. |
| `S8` | `required` | Findings and claim ceilings still need an explicit Pi surface. |
| `S9` | `provisional` | Allowed only for reroute or appliance-split proposals. |

## Milestone plan

| Milestone | Expected deliverable | Toolchain or wiring needed | Known unknowns |
|-----------|----------------------|----------------------------|----------------|
| `boot` | One Raspberry Pi family board boots stable 64-bit Linux with boring LAN reachability and a writable local cache path if needed. | Raspberry Pi 4B or Pi 5, stable PSU, persistent storage, network access, and local console fallback. | Exact board, storage medium, and power arrangement remain unnamed tonight. |
| `identify` | The Pi can present a stable family-specific identity to the x86_64 control plane without claiming local canonical profile ownership. | One Pi board plus x86_64 control-plane reachability and explicit control-plane attribution in findings. | The exact enrollment route and the point where identity can become local later remain open. |
| `accept-grant` | Capability proposals and operator-visible consent remain routed through the x86_64 control plane while the Pi consumes only the granted boundary. | Existing x86_64 onramp and consent surfaces plus Pi-side mirror notes later. | No real Pi-side consent execution is proven yet. |
| `report-state` | One bounded edge or kiosk state report may later be surfaced through the Pi lane without implying independent HA proof. | Pi-side runtime plus later Pack C or stand-in observation surface. | No real Pi-side HA entity or dashboard observation exists yet. |

## Current execution posture

| Milestone | Current posture | Why |
|-----------|-----------------|-----|
| `boot` | Defined only | No named Pi board is under test tonight |
| `identify` | Defined only | Control-plane attribution exists, but not board execution |
| `accept-grant` | Deferred | Consent remains x86_64-owned for the early Pi posture |
| `report-state` | Deferred | No Pi-side runtime or HA observation exists yet |

## Sources held constant for this baseline

- `docs/adr/ADR-025-dual-root-hardware-runway.md`
- `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-family-lane-profiles.md`
- `docs/hub/reference-hardware.md`
- `docs/hardware/d1-target-inventory.md`