# S7 — Smart-Home Hub

**Stream:** S7  
**Status:** 🟡 Runway mode; G4 is active and implementation is no longer blocked on G3  
**Gate:** G4 — this stream owns the launch gate

---

## Mission

`ferros-hub` is the launch vehicle. Launch is not a website event or a crates.io publish — it is a real hub binary running on a Raspberry Pi or home server, with a real device profile, at least one real Home Assistant entity registered through the agent center, and consent enforced end-to-end. See `LAUNCH.md` for the authoritative definition.

Current S7 work is still runway work. The stream can prepare hardware, deployment assumptions, and evidence collection now, but it should not claim a running hub, a satisfied G4 checklist, or an authoritative pairing handshake before the relevant S2 consumer surfaces and S3/S4 runtime seams are concrete in implementation.

---

## Scope

- `ferros-hub` crate/binary:
  - Target personas: smart-home hub, AI edge device, home server.
  - Wraps `ferros-node` (S4) and `ferros-agents` (S3) into a single deployable binary.
  - Pairing runway: document the device-profile, operator-approval, persistence, revocation, and deny-observability constraints that must eventually compose around the stable S2 consumer surfaces (`ProfileId`, `CapabilityGrant`) and yield signed capability grants. The exact handshake remains provisional until the hub exists and the S3/S4 seams are real.
  - Reboot-safe storage: profile and grants must persist across restart and full power cycle.
- Home Assistant integration (fork: `Maangled/home-assistant`):
  - FERROS HA custom component registers agents and devices via the agent center.
  - FERROS-managed agents appear as HA entities with consent gates.
  - Consent denials must be observable in the HA dashboard or `ferros agent logs` once implemented.
- Target platforms: `x86_64-unknown-linux-gnu` (home server) and `aarch64-unknown-linux-gnu` (Pi / edge).
- Reference hardware runway: `docs/hub/reference-hardware.md`.

---

## Current lane

- Treat `docs/hub/reference-hardware.md` as the hardware recipe authority for this wave and keep it aligned to the evidence G4 will eventually require.
- Use the Pack B `x86_64` lane as the first bring-up target unless real hardware availability forces a Pi-first pass, because it improves shell access, log capture, rollback, and restart debugging while staying launch-valid.
- Keep the first Home Assistant lab topology separate from the device under test so later restart and deny-observability evidence can be attributed cleanly.
- Keep pairing notes at the level of constraints and open questions bound to S2 consumer surfaces plus the S3/S4 seams that will eventually enforce them.
- Map each unchecked G4 evidence item to one upstream seam and one S7-owned proof point before any `ferros-hub` crate or HA bridge work is proposed.

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
- **S2:** `ProfileId` and `CapabilityGrant` are stable consumer surfaces for runway planning, so S7 should consume them rather than redefine signing, approval, or issuance semantics.
- **S6:** Harvest patterns may inform hub agent design later.

---

## Pairing posture (runway only)

- S7 currently treats pairing as a bounded consumer-side design problem, not a stream-local protocol that can be ratified from planning docs alone.
- S2 gives S7 stable names to plan around today: `ProfileId` and `CapabilityGrant`.
- S7 can document hub obligations now: operator approval checkpoints, signed-grant persistence, revocation expectations, restart and power-cycle survival, and consent-deny observability.
- S7 should not freeze handshake order, signing ceremony details, or authoritative grant semantics just because G3 is closed; the real hub-facing S3/S4 seams are still not concrete enough to lock the protocol details.

---

## Runway pairing checkpoint map

This map is runway-only. It binds provisional pairing checkpoints to the current S2 consumer surfaces plus the S3 registry/list/log surfaces and S4 runtime policy, deny logging, and restart seams. It does not ratify handshake order, grant-issuance ceremony, or an authoritative pairing protocol.

| Checkpoint | Current seam map | What stays open |
|------------|------------------|-----------------|
| bootstrap | Use S2 `ProfileId` as the device-side identity surface S7 plans around; use the S4 restart seam as the boundary that will eventually prove whether bootstrap state is present before and after restart. | Who creates the initial device state, what first-start approval path exists, and the exact bootstrap order remain provisional. |
| grant check | Use S2 `CapabilityGrant` as the grant currency; use S4 runtime policy as the enforcement seam; use the S3 registry/list surfaces as the place S7 expects to observe whether the bridge agent is present when checks begin. | The exact point where registration, approval, and grant issuance must occur remains open. |
| deny visibility | Use S4 deny logging as the source of truth for rejected capability use; use the S3 log surface as the FERROS-side inspection path while HA-facing visibility remains an integration outcome. | The final operator-visible split between HA UI and FERROS logs remains open. |
| persistence | Treat S2 `ProfileId` and `CapabilityGrant` state as the material that must survive clean restart and full power cycle; use the S4 restart seam as the boundary S7 plans around for reload. | Storage ownership, on-disk layout, and durability choreography remain open until the real hub exists. |
| revocation | Treat revoked S2 `CapabilityGrant` state as something S4 runtime policy and deny logging must reflect once implementation exists. | Revocation propagation, fan-out, and operator workflow remain provisional. |
| re-registration | Use the S3 registry/list surfaces as the checkpoint for the bridge agent returning after restart; use the S4 restart seam as the host/runtime recovery boundary. | Reconnect order, refresh behavior, and the exact relationship between re-registration and HA recovery remain open. |

---

## S2 consumer-boundary questions S7 needs answered before naming an authoritative pairing flow

These queue-ready S2-facing questions are the next step after the landed six-checkpoint map. They keep S7 in runway mode, do not freeze handshake order, and do not redefine `ProfileId` or `CapabilityGrant` semantics from S7 docs.

| Checkpoint | S2 consumer-boundary question S7 still needs answered | Why S7 needs the answer |
|------------|--------------------------------------------------------|--------------------------|
| bootstrap | What minimum `ProfileId`-bound device state may S7 assume already exists and is durable before the hub attempts any first-start pairing action? | S7 needs a stable consumer-side boundary for initial device identity without inventing a hub-local bootstrap ceremony. |
| grant check | What consumer-visible condition tells S7 it is valid to treat a `CapabilityGrant` as present for bridge-agent exposure and runtime grant checks? | S7 needs to know what it can gate on without defining its own approval or grant-issuance order. |
| deny visibility | Which denied `CapabilityGrant` conditions should remain distinguishable at the consumer boundary when S7 surfaces a rejected action to operators? | S7 needs to route deny evidence to HA UI or `ferros agent logs` without changing S2 grant semantics. |
| persistence | What persisted `ProfileId` or `CapabilityGrant` material may S7 rely on after restart and full power cycle, and what freshness boundary should S7 expect before reusing it? | S7 needs to plan durable storage and reload checks without claiming storage ownership or a final on-disk model. |
| revocation | What consumer-visible revocation state or signal should cause S7 to stop treating a previously accepted `CapabilityGrant` as usable? | S7 needs a concrete upstream boundary for revocation handling instead of inventing stream-local revocation rules. |
| re-registration | After restart, what S2-bound condition lets S7 treat a returning bridge agent as still operating under the same `ProfileId` and grant context versus requiring a new approval path? | S7 needs to know what can survive restart before it names any authoritative re-registration flow. |

---

## What this stream blocks

- **Launch (G4).** S7 owns the launch gate, and the remaining blocker is real hardware and hub evidence.

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
2. Select the exact first `x86_64` Pack B device and one fallback `aarch64` Pack A device for bring-up.
3. Route the S2 consumer-boundary question list above to S2 and record the answers before S7 names any authoritative pairing flow.
4. Keep the G4 evidence map tied to the exact S2 consumer dependencies and S3/S4 seams that must land before an implementation plan is honest.
5. Prepare the post-G3 design handoff for `ferros-hub` without scaffolding the crate or bridge in this wave.
