# S7 Smart-Home Hub — Contracts

---

## Contracts owned or staged by S7

S7 owns deployment-facing hub surfaces. It does not get to redefine S2 profile or capability-grant semantics from stream-local planning docs.

| Contract | Type | Location | Status |
|----------|------|----------|--------|
| `HubConfig` type (device profile, port, HA endpoint) | Rust type | `crates/ferros-hub/` | ⬜ Not yet created |
| HA bridge agent manifest | `AgentManifest` JSON | `crates/ferros-hub/src/ha_bridge.rs` | ⬜ Not yet created |
| Pairing constraints and open questions | Stream-local planning notes | `streams/S7-hub/*.md` | 🟨 Runway only; authoritative protocol intentionally deferred |
| Reference hardware runway and evidence template | Doc | `docs/hub/reference-hardware.md` | 🟨 In progress |

---

## Contracts consumed by S7

| Contract | Source | Purpose |
|----------|--------|---------|
| `ProfileId`, `CapabilityGrant` | S2 | Pairing flow issues grants; profile binds to device |
| `Agent` trait, `AgentRegistry` | S3 | Hub registers HA-bridge agents through S3 |
| `ferros-runtime` executor + bus | S4 | Hub wraps the runtime as its execution environment |
| `ferros-data` primitives | S6 | Optional: data generation for hub agents |

---

## Pairing boundary note

The future hub pairing flow must end in signed, reboot-survivable, revocable capability grants because launch requires those properties in practice. That does not mean the exact handshake should be frozen in these docs yet. Until `ferros-hub` exists and the S2/S3/S4 implementation seams are concrete, S7 should track constraints, storage needs, and validation expectations rather than pretend the protocol is already settled.

---

## Notes on the HA fork

The `Maangled/home-assistant` fork is a separate repository. S7 adds a FERROS custom component to that fork that bridges the HA device/entity model to the FERROS agent center. Changes to the HA fork are tracked in that repo; S7 documents the integration interface here.
