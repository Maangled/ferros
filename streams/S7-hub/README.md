# S7 — Smart-Home Hub

**Stream:** S7  
**Status:** ⬜ Runway mode; implementation blocked on G3  
**Gate:** G4 — this stream owns the launch gate

---

## Mission

`ferros-hub` is the launch vehicle. Launch is not a website event or a crates.io publish — it is a real hub binary running on a Raspberry Pi or home server, with a real device profile, at least one real Home Assistant entity registered through the agent center, and consent enforced end-to-end. See `LAUNCH.md` for the authoritative definition.

Current S7 work is still runway work. The stream can prepare hardware, deployment assumptions, and evidence collection now, but it should not claim a running hub, a satisfied G4 checklist, or an authoritative pairing handshake before the relevant S2/S3/S4 implementation surfaces exist.

---

## Scope

- `ferros-hub` crate/binary:
  - Target personas: smart-home hub, AI edge device, home server.
  - Wraps `ferros-node` (S4) and `ferros-agents` (S3) into a single deployable binary.
  - Pairing runway: identify the device-profile, approval, persistence, and revocation constraints that must eventually produce signed capability grants. The exact handshake remains provisional until implementation work starts.
  - Reboot-safe storage: profile and grants must persist across restart and full power cycle.
- Home Assistant integration (fork: `Maangled/home-assistant`):
  - FERROS HA custom component registers agents and devices via the agent center.
  - FERROS-managed agents appear as HA entities with consent gates.
  - Consent denials must be observable in the HA dashboard or `ferros agent logs` once implemented.
- Target platforms: `x86_64-unknown-linux-gnu` (home server) and `aarch64-unknown-linux-gnu` (Pi / edge).
- Reference hardware runway: `docs/hub/reference-hardware.md`.

---

## Current lane

- Expand the hardware runway doc so the first on-device sessions collect the evidence G4 will eventually require.
- Decide the first physical hardware targets and Home Assistant topology.
- Keep pairing notes at the level of constraints and open questions until the actual hub crate and S2/S3/S4 boundaries are ready.

---

## Out of scope

- The Home Assistant core itself (the fork is a separate repo).
- Multi-node / distributed hub behavior (`v0.3.0+`).
- Embedded targets below Linux (`thumbv7em-none-eabi`).
- Ratifying S2 identity or capability-grant semantics from stream-local docs.

---

## Dependencies

- **S4 (G3 must be green):** `ferros-runtime` must be stable before hub wraps it.
- **S3 (G3 must be green):** Agent registry and spawn lifecycle must be functional.
- **S2:** Profile and capability-grant types are the pairing-flow currency, so S7 should consume them rather than redefine them.
- **S6:** Harvest patterns may inform hub agent design later.

---

## What this stream blocks

- **Launch (G4).** S7 owns the launch gate, but G4 still depends on G3 closing and on real hardware evidence.

---

## Definition of done (G4 — launch)

- [ ] `ferros-hub` cross-compiles for the chosen home-hardware target (`aarch64-unknown-linux-gnu` or `x86_64-unknown-linux-gnu`).
- [ ] Runs on a physical Raspberry Pi or home server, not CI, not QEMU, and not a developer laptop.
- [ ] Device profile created via `ferros profile init` on the target device and still loads after restart.
- [ ] `ferros agent list` shows at least one registered HA-bridge agent on the running hub.
- [ ] At least one real Home Assistant entity is registered through the FERROS agent center and visible in the HA dashboard.
- [ ] Consent is enforced: an ungranted capability request is denied at the bus level, logged, and visible in HA UI or `ferros agent logs`.
- [ ] Hub survives a full power cycle: profile loads, agents re-register, and the HA entity is restored without manual intervention.
- [ ] At least one independent private-beta install is confirmed outside the primary developer's machine.
- [ ] `docs/hub/reference-hardware.md` records the tested hardware, and `LAUNCH.md` is updated with the confirmed install date and hardware specification.

---

## Likely crates / files

| Path | Role | Status |
|------|------|--------|
| `crates/ferros-hub/` | Hub binary crate | ⬜ Not created yet |
| `crates/ferros-hub/src/pairing.rs` | Pairing implementation surface | ⬜ Deferred until implementation |
| `crates/ferros-hub/src/ha_bridge.rs` | Home Assistant bridge agent | ⬜ Deferred until implementation |
| `docs/hub/reference-hardware.md` | Hardware runway and evidence prep | 🟨 Active |
| `docs/hub/install.md` | Install script and instructions | ⬜ Future |

---

## Immediate next steps

1. Keep `docs/hub/reference-hardware.md` current with the chosen runway hardware, topology assumptions, and evidence fields.
2. Select the first physical `aarch64` and `x86_64` candidates for bring-up.
3. Record pairing constraints and open questions without freezing the final handshake before implementation work starts.
4. Scaffold `crates/ferros-hub/` after G3 closes.
5. Implement the HA bridge agent against S3's `Agent` trait and S4 runtime once the crate exists.
6. Run the first real hardware power-cycle validation before claiming any G4 progress.
