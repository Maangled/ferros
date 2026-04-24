# S7 Smart-Home Hub — Backlog

---

## Now

- [ ] Keep `docs/hub/reference-hardware.md` current with candidate hardware, storage/network assumptions, and G4 evidence fields
- [ ] Select one `aarch64` candidate and one `x86_64` candidate for the first on-device sessions
- [ ] Define the first Home Assistant lab topology for hardware bring-up
- [ ] Capture pairing constraints and open questions without freezing the final handshake semantics

## Next (after G3)

- [ ] Scaffold `crates/ferros-hub/`
- [ ] Turn runway pairing notes into an implementation plan once S2/S3/S4 boundaries are real
- [ ] Implement pairing flow using S2 profile and capability-grant APIs
- [ ] Implement HA bridge agent using S3 `Agent` trait
- [ ] Wire `ferros-hub` to `ferros-runtime` (S4) and `ferros-agents` (S3)
- [ ] Add cross-compilation coverage for the chosen home-hardware target in CI
- [ ] Smoke test on the easier first physical target before attempting broader hardware coverage

## Later

- [ ] First real hardware bring-up on Raspberry Pi or home server
- [ ] Verify restart and full power-cycle survival on the chosen launch candidate
- [ ] `docs/hub/install.md` install script
- [ ] At least one independent private-beta install
- [ ] Update `LAUNCH.md` with confirmed install date and hardware specification when evidence is real
- [ ] Verify the full G4 checklist (see `docs/gates/G4.md`)

## Blocked

- Runtime implementation is blocked on G3 (S3 + S4 convergence).
- The first honest G4 evidence is blocked on a real `ferros-hub` binary and HA bridge existing on physical hardware.
- HA custom component work depends on the `Maangled/home-assistant` fork being available.
