# S7 Smart-Home Hub — Backlog

---

## Now

- [ ] Keep `docs/hub/reference-hardware.md` current as the hardware recipe authority for candidate hardware, storage/network assumptions, and G4 evidence fields
- [ ] Select one `aarch64` candidate and one `x86_64` candidate for the first on-device sessions
- [ ] Define the first Home Assistant lab topology for hardware bring-up and consent-deny observation
- [ ] Capture pairing constraints and open questions bounded by S2 consumer surfaces (`ProfileId`, `CapabilityGrant`) and S3/S4 runtime seams
- [ ] Record grant-aware design expectations for persistence, revocation, and deny visibility without freezing protocol steps

## Next runway

- [ ] Write the list of S2 consumer questions that must be answered before S7 names an authoritative pairing flow
- [ ] Map the provisional pairing checkpoints to the likely S3/S4 seams: bootstrap, grant check, persistence, revocation, and re-registration after restart
- [ ] Draft a post-G3 pairing/design handoff that can become an implementation plan once real hub and runtime APIs exist
- [ ] Recheck runway pairing notes against `docs/hub/reference-hardware.md`, `STATUS.md`, and `docs/gates/G4.md` whenever S7 wording moves

## Later runway prep

- [ ] Prepare the first evidence-capture worksheet for on-device sessions without claiming G4 evidence yet
- [ ] Outline the operator notes needed for power-cycle validation and consent-deny observation
- [ ] Prepare the eventual `docs/hub/install.md` doc scope as a placeholder only, not runnable install instructions
- [ ] Queue the G4 evidence handoff points that must be satisfied before any launch claim

## Blocked / deferred until G3

- Runtime and hub implementation work stay deferred until G3 closes and the S3/S4 seams are concrete: no `crates/ferros-hub/` scaffold, no pairing implementation, no HA bridge, no runtime wiring, and no G4 evidence claims in this wave.
- The first honest G4 evidence is blocked on a real `ferros-hub` binary and HA bridge existing on physical hardware.
- HA custom component work depends on the `Maangled/home-assistant` fork being available.
