# S7 Smart-Home Hub — Contracts

---

## Contracts owned by S7

| Contract | Type | Location | Status |
|----------|------|----------|--------|
| `HubConfig` type (device profile, port, HA endpoint) | Rust type | `crates/ferros-hub/` | ⬜ Not yet created |
| HA bridge agent manifest | `AgentManifest` JSON | `crates/ferros-hub/src/ha_bridge.rs` | ⬜ Not yet created |
| Pairing protocol | Convention + code | `crates/ferros-hub/src/pairing.rs` | ⬜ Not yet created |
| Reference hardware spec | Doc | `docs/hub/reference-hardware.md` | ⬜ Not yet written |

---

## Contracts consumed by S7

| Contract | Source | Purpose |
|----------|--------|---------|
| `ProfileId`, `CapabilityGrant` | S2 | Pairing flow issues grants; profile binds to device |
| `Agent` trait, `AgentRegistry` | S3 | Hub registers HA-bridge agents through S3 |
| `ferros-runtime` executor + bus | S4 | Hub wraps the runtime as its execution environment |
| `ferros-data` primitives | S6 | Optional: data generation for hub agents |

---

## Notes on the HA fork

The `Maangled/home-assistant` fork is a separate repository. S7 adds a FERROS custom component to that fork that bridges the HA device/entity model to the FERROS agent center. Changes to the HA fork are tracked in that repo; S7 documents the integration interface here.
