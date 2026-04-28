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
- Keep pairing notes at the level of consumed S2 consumer-boundary assumptions plus the S3/S4 seams still missing before any authoritative pairing flow, `ferros-hub` scaffold, or HA bridge plan is honest.
- Map each unchecked G4 evidence item to one upstream seam and one S7-owned proof point before any `ferros-hub` crate or HA bridge work is proposed.

---

## Operator evidence surface

The operator-facing evidence surface for hub bring-up and status is read-only. It is the operator observation surface for D1 demo preparation and G4 evidence collection, not a consumer hub UI.

**Source of truth:** `docs/hub/pack-b-bring-up-worksheet.md` (derived from `docs/hub/reference-hardware.md`, which remains authoritative).

### Surface definition

| Field | Source | Notes |
|-------|--------|-------|
| Device target | `docs/hub/d1-target-inventory.md` (planned) | Primary D1 target name and form factor |
| Firmware-spike milestone reached | HARDWARE-QUEUE wave results | Which of the four milestones (boot / identify / accept-grant / report-state) has been reached |
| Profile init status | `ferros profile init` on target device | Whether `ferros profile init` and `ferros profile show` have run successfully on the target |
| HA entity registered (or named stand-in) | S7 bridge runway notes | Which entity or stand-in is registered; must be named if a stand-in is used |
| Consent flow visible | localhost shell deny-log | Whether deny-by-default enforcement is demonstrable to the operator |
| Power-cycle status | D1 evidence capture | Whether reboot-safe FERROS-side state has been demonstrated |

**Constraints:**
- Read-only. The operator views bring-up state; this surface does not mutate it.
- Does not constitute D1 evidence. Evidence is documented in `docs/gates/D1.md` when all D1 criteria are met simultaneously.
- Does not introduce new JSON/RPC routes. All reads come from the existing S3 read-first contract (`agent.snapshot`, `agent.list`, `denyLog.list`) and local CLI output.
- HA bridge protocol details, pairing handshake order, and HA fork internals are not invented here.

---

## HA bridge onramp mapping (ADR-023)

HA entity discovery through the FERROS bridge is an onramp event under ADR-023 (Onramp Policy). This note documents how HA entity registration maps to the ADR-023 framing.

**Mapping:**
- HA entities discovered by the bridge arrive as **proposed FERROS material**, not as canonical FERROS profile or capability-grant state.
- A discovered entity must route through the S5 onramp consent surface before it becomes canonical FERROS state (profile attribute, persisted capability grant, or sealed progress evidence).
- Passively running the HA bridge does not constitute consent. Consent requires an explicit user accept action through the S5 onramp surface.
- The accept event must be auditable through the S3/S4 audit-log surface.

**What this note does NOT constrain:**
- Bridge protocol details (pairing handshake order, HA entity schema, HA wire format) remain S7-owned decisions. ADR-023 establishes the consent invariant; it does not specify how the bridge works internally.
- The onramp consent surface implementation is S5-owned; S7 produces the proposed-material item and hands it to the staging area; S7 does not own the accept/reject UX.
- No new S7 protocol constraints are introduced by this mapping note.

**ADR reference:** `docs/adr/ADR-023-onramp-policy.md` (Accepted). The consumer-awareness note for S7 is appended to that ADR.

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
- S2 has now published the consumer-boundary answers S7 can plan around: `ProfileId` is derived from the persisted local verifying key, and `CapabilityGrant` state is usable only when the signed envelope verifies, binds locally, and is not revoked.
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

## S7 handoff assumptions from published S2 consumer boundaries

This handoff is provisional and consumer-boundary only. It records what S7 may assume now from S2 and what still remains open before any authoritative pairing flow, `ferros-hub` scaffold, or Home Assistant bridge plan is honest.

| Checkpoint | What S7 may assume now from S2 | What stays open before any authoritative pairing flow, `ferros-hub` scaffold, or Home Assistant bridge plan is honest |
|------------|--------------------------------|----------------------------------------------------------------------------------------------------------|
| bootstrap | Durable identity bootstrap is honest only when S2 can reload a locally persisted `KeyPair`; `ProfileId` is derived from that Ed25519 verifying key, and bootstrap identity alone does not imply any `CapabilityGrant` exists yet. | Who creates the first-start state, what operator approval path exists, and the exact bootstrap order remain open. |
| grant check | Treat a capability as present only when a persisted signed grant envelope verifies, binds to the local `ProfileId`, matches the local signer public key, and is not revoked. | Where that verified grant state first composes with the S3 registry/list path and the S4 runtime policy seam remains open until real APIs exist. |
| deny visibility | At the S2 boundary, deny causes stay limited to missing grant state, revoked grant state, or invalid/mismatched signed grant material. | How S4 deny logging preserves those causes and how S7 exposes them through FERROS logs or HA-facing surfaces remains open. |
| persistence | After restart or power cycle, only local profile and grant state that reloads and passes S2 local validation is reusable; current filesystem-first persistence is implementation evidence, not a published on-disk contract. | Storage ownership, on-disk layout, and restart choreography remain open. |
| revocation | A previously accepted grant becomes unusable when the signed envelope carries `revoked_at` and `revocation_reason`, has been re-signed, still verifies, and therefore reads as revoked. | How revocation propagates through S4 runtime policy, S3 observation surfaces, and operator recovery remains open. |
| re-registration | Treat a returning bridge agent as the same identity and grant context only when reload yields the same `ProfileId` from the persisted key and the relevant signed grants still verify, match that identity and signer, and remain active. | The exact re-registration choreography, restart order, and HA recovery path remain open. |

Outcome: S7 now has a provisional consumer-boundary handoff from S2, not an authoritative pairing flow or implementation plan.

---

## S7 seam brief before hub planning is honest

This seam brief stays docs-only. It names the exact current S3 and S4 surfaces S7 can point at today, what S7 may honestly assume from them, and what still remains unpublished before any authoritative pairing flow, `ferros-hub` scaffold, or Home Assistant bridge plan is honest.

| Upstream seam | Exact current surface S7 can name | What S7 may assume now | What still remains unpublished or open |
|---------------|-----------------------------------|------------------------|----------------------------------------|
| S3 registration surface | `AgentRegistry::register`, `AgentRegistry::deregister`, `AgentRegistry::list`, `AgentRegistry::describe` | The first HA bridge slice must be legible as one registered agent that can be listed and described through the same S3-owned registration boundary as other agents. | No hub-owned registration wrapper or lifecycle contract exists yet, so S7 cannot claim a final bridge registration flow or a multi-agent topology. |
| S3 operator inspection surface | local `ferros agent list`, `ferros agent describe`, `ferros agent logs`; read-first `agent.list`, `agent.describe`, `grant.list`, `denyLog.list` | The first bridge evidence slice can plan around these local inspection paths for agent presence, grant-state reads, and deny-log observation. | No published remote or HA-facing observation contract exists yet, and S3 still treats transport serving and privileged writes as open work. |
| S4 consent policy surface | `CapabilityRequest`, `CapabilityGrantView`, `PolicyEngine::evaluate`, `DenyByDefaultPolicy`, `PolicyDecision`, `PolicyDenialReason` | Grant checks and deny outcomes must ultimately compose through these S4 policy types and reasons instead of any S7-local grant logic. | The exact hub wrapper around policy evaluation and the operator-facing deny-propagation path are not yet published as S4-owned hub seams. |
| S4 restart and reload surface | No published hub-facing restart API; nearest current reload seams are `runtime_with_state(state_path)`, `CliState::load(state_path)`, and `LocalProfileStore::load_local_profile(path)` | Restart honesty currently means S7 can only trust state that reloads through those existing local validation paths. | There is still no stable runtime restart or re-registration contract for hubs, so S7 cannot freeze reboot choreography or treat current node-local helpers as the final seam. |

- S7 may now plan against the named S3 and S4 surfaces above, but it must still escalate missing hub-facing contracts to the owning streams instead of filling the gaps with stream-local protocol rules.
- The next honest runway step is to treat the returned S3 and S4 seam classifications as dependency locks and keep S7 docs aligned as upstream wrapper and restart contracts change.

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
3. Keep the returned S3 and S4 seam classifications as upstream dependency locks and keep S7 docs aligned as upstream wrapper and restart contracts change.
4. Keep the G4 evidence map tied to the exact S2 consumer dependencies plus the S3/S4 seams named in that brief and any upstream answers that follow.
5. Keep that follow-up docs-only and non-implementation: no `crates/ferros-hub/` scaffold and no Home Assistant bridge internals.
