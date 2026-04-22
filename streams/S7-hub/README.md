# S7 — Smart-Home Hub

**Stream:** S7  
**Status:** ⬜ Blocked on G3  
**Gate:** G4 — this stream owns the launch gate

---

## Mission

`ferros-hub` is the actual launch vehicle. Launch is not a website event or a crates.io publish — it is this binary running on a Raspberry Pi or home server, paired with a real profile, with at least one Home Assistant entity registered through the agent center and consent enforced. See `LAUNCH.md` for the precise definition.

---

## Scope

- `ferros-hub` crate/binary:
  - Target persona: smart-home hub, AI edge device, home server.
  - Wraps `ferros-node` (S4) and `ferros-agents` (S3) into a single deployable binary.
  - Pairing flow: device profile (Ed25519, device-bound) + user profile → signed capability grants.
  - Reboot-safe storage: profile and grants persist across power cycles.
- Home Assistant integration (fork: `Maangled/home-assistant`):
  - FERROS HA custom component: registers agents and devices via the agent center.
  - FERROS-managed agents appear as HA entities with consent gates.
  - Consent deny is visible in the HA dashboard.
- Target platforms: `x86_64-unknown-linux-gnu` (home server), `aarch64-unknown-linux-gnu` (Pi / edge).
- Reference hardware recipe: `docs/hub/reference-hardware.md`.

---

## Out of scope

- The Home Assistant core itself (fork is a separate repo).
- Multi-node / distributed case (post-launch, `v0.3.0+`).
- Embedded targets below Linux (`thumbv7em-none-eabi`) — post-launch.

---

## Dependencies

- **S4 (G3 must be green):** `ferros-runtime` must be stable before hub wraps it.
- **S3 (G3 must be green):** Agent registry and spawn lifecycle must be functional.
- **S6:** `botgen-rust` harvest patterns may inform hub agent design.

---

## What this stream blocks

- **Launch (G4).** Nothing else blocks G4 except S7.

---

## Definition of done (G4 — launch)

- [ ] `ferros-hub` binary compiles for `aarch64-unknown-linux-gnu`.
- [ ] Runs on a Raspberry Pi or home server (not CI, not QEMU, not developer laptop).
- [ ] Device profile created via `ferros profile init` on the target device.
- [ ] `ferros agent list` shows at least one registered HA-bridge agent.
- [ ] At least one real HA entity registered and visible in HA dashboard — not mocked.
- [ ] Consent enforced: ungranted capability request blocked at bus level, logged, visible in HA UI or `ferros agent logs`.
- [ ] Hub survives a full power cycle: profile loads, agents re-register, HA entity restored.
- [ ] At least one independent private-beta install confirmed (not the primary developer's machine).
- [ ] `docs/hub/reference-hardware.md` documents the tested hardware.

---

## Likely crates / files

| Path | Role |
|------|------|
| `crates/ferros-hub/` | Hub binary crate |
| `crates/ferros-hub/src/pairing.rs` | Device pairing flow |
| `crates/ferros-hub/src/ha_bridge.rs` | Home Assistant bridge agent |
| `docs/hub/reference-hardware.md` | Hardware recipe |
| `docs/hub/install.md` | Install script and instructions |

---

## Immediate next steps

1. Write `docs/hub/reference-hardware.md` hardware recipe (can begin before G3).
2. Design pairing flow: device profile + user profile → grants (align with S2 API).
3. Scaffold `crates/ferros-hub/` (after G3).
4. Implement HA bridge agent referencing S3's `Agent` trait.
5. Implement aarch64 cross-compilation target in CI.
6. Power cycle test on real hardware.
