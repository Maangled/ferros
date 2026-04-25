# S7 Smart-Home Hub — Backlog

---

## Now

- [ ] Keep `docs/hub/reference-hardware.md` current as the hardware recipe authority for candidate hardware, storage/network assumptions, and G4 evidence fields
- [ ] Select one `aarch64` candidate and one `x86_64` candidate for the first on-device sessions; treat the Pack B `x86_64` lane as the first bring-up target unless hardware reality forces Pi-first
- [x] Define the first Home Assistant lab topology for hardware bring-up and consent-deny observation around one device under test plus one separate HA host
- [x] Define the first Home Assistant bridge runway contract at one bridge agent, one real entity, runtime-enforced capability checks, operator-visible deny traces, and restart-safe FERROS state
- [x] Land the runway-only six-checkpoint pairing map bounded by S2 consumer surfaces (`ProfileId`, `CapabilityGrant`) and the current S3/S4 seams
- [x] Land the queue-ready S2 consumer-boundary question list for bootstrap, grant check, deny visibility, persistence, revocation, and re-registration without freezing protocol steps
- [x] Consume the published S2 consumer-boundary answers into an S7-owned handoff/assumptions slice without freezing handshake order, storage layout, or re-registration choreography

## Next runway

- [ ] Turn the bridge runway contract into a DUT-side evidence checklist once a real hub/bridge path exists; keep the first proof at one registered bridge agent and one real entity
- [ ] Turn the published S2 consumer-boundary handoff into an S7-owned seam brief keyed to the exact S3 registry/list/log and S4 restart/policy APIs still needed before any authoritative pairing flow, `ferros-hub` scaffold, or HA bridge plan is honest
- [ ] Keep that seam brief docs-only and non-implementation until the concrete S3/S4 APIs exist
- [ ] Recheck runway pairing notes against `docs/hub/reference-hardware.md`, `streams/S7-hub/CONTRACTS.md`, `STATUS.md`, and `docs/gates/G4.md` whenever S7 wording moves, S2 consumer-boundary rules change, or new S3/S4 seam details publish

## Later runway prep

- [ ] Prepare the first evidence-capture worksheet for on-device sessions without claiming G4 evidence yet
- [ ] Outline the operator notes needed for power-cycle validation and consent-deny observation
- [ ] Prepare the eventual `docs/hub/install.md` doc scope as a placeholder only, not runnable install instructions
- [ ] Queue the G4 evidence handoff points that must be satisfied before any launch claim

## Blocked / deferred in current G4 runway

- Runtime and hub implementation work remain out of scope in the active G4 runway: no `crates/ferros-hub/` scaffold, no pairing implementation, no HA bridge, no runtime wiring, and no G4 evidence claims in this pass.
- The first honest G4 evidence is blocked on a real `ferros-hub` binary and HA bridge existing on physical hardware.
- HA custom component work depends on the `Maangled/home-assistant` fork being available.

