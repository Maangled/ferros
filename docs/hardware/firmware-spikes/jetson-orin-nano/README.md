# Jetson Orin Nano Family Baseline

> Planning note only for the non-x86 family baseline packet. This file does not constitute physical Jetson evidence, D1 evidence, G4 evidence, Home Assistant proof, or FERROS-native OS proof.

## Target summary

| Field | Value | Note |
|-------|-------|------|
| Family | `Jetson Orin Nano` | Current specialized edge family |
| Lane role | `ADR-025 migration family with vendor-image dependency` | Not a first D1 closure lane |
| Practical posture | `Linux-first specialized edge family` | GPU or vendor-image reality remains part of the truth boundary |
| Preferred target shape | `Jetson Orin Nano dev kit with vendor-supported Linux image and stable LAN` | Exact board still to be named later |
| Control plane | `x86_64 Fastest on homelab001` | Owns early identity, storage, and truth surfaces |
| Current evidence state | `docs-only baseline packet only` | No physical Jetson session has happened |

## Claim ceiling

- This file is a docs-only baseline.
- `S2` and `S6` stay federated to the x86_64 control plane during early Jetson runway work.
- Vendor-image or vendor-boot-chain dependency remains an explicit part of the current posture.
- No physical-device run, no Home Assistant proof, no D1 movement, no G4 movement, and no FERROS-native OS claim are added here.

## Lane baseline

| Lane | Status | Baseline note |
|------|--------|---------------|
| `S1` | `required` | Boot and board prep stay real work, but native boot remains deferred. |
| `S2` | `federated` | Early identity remains anchored on x86_64. |
| `S3` | `required` | Inference or coordinator agents are credible Jetson work. |
| `S4` | `required` | Runtime posture matters, but native runtime proof is not claimed. |
| `S5` | `required` | Operator or model-observation UI remains plausible here. |
| `S6` | `federated` | Early artifacts and caches stay above the board. |
| `S7` | `required` | Jetson may later serve hub or edge-inference bridge roles. |
| `S8` | `required` | Evidence and claim discipline stay mandatory. |
| `S9` | `provisional` | Allowed only for appliance-split or reroute proposals. |

## Milestone plan

| Milestone | Expected deliverable | Toolchain or wiring needed | Known unknowns |
|-----------|----------------------|----------------------------|----------------|
| `boot` | One Jetson board boots a vendor-supported Linux image with stable LAN reachability and local recovery path. | Jetson Orin Nano dev kit, vendor image, stable PSU, network, and console fallback. | Exact board revision, image version, and recovery path remain unnamed tonight. |
| `identify` | The Jetson presents a stable family-specific identity to the x86_64 control plane without claiming local canonical profile ownership. | One Jetson board plus x86_64 control-plane reachability and explicit attribution in findings. | The long-term balance between local and federated identity remains open. |
| `accept-grant` | Capability proposals and operator-visible consent remain routed through x86_64 during early Jetson work while the board consumes only the granted boundary. | Existing x86_64 onramp and consent surfaces plus later Jetson-side mirror notes. | No real Jetson-side consent execution is proven yet. |
| `report-state` | One bounded hub, edge-inference, or stand-in output may later be surfaced through Jetson without implying independent HA proof. | Jetson-side runtime plus later control-plane or HA observation surface. | No real Jetson runtime or HA-facing observation exists yet. |

## Current execution posture

| Milestone | Current posture | Why |
|-----------|-----------------|-----|
| `boot` | Defined only | No named Jetson board is under test tonight |
| `identify` | Defined only | Control-plane attribution exists, but not board execution |
| `accept-grant` | Deferred | Consent remains x86_64-owned for the early Jetson posture |
| `report-state` | Deferred | No Jetson-side runtime or HA observation exists yet |

## Sources held constant for this baseline

- `docs/adr/ADR-025-dual-root-hardware-runway.md`
- `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-family-lane-profiles.md`
- `docs/hub/reference-hardware.md`
- `docs/hardware/d1-target-inventory.md`