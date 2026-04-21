# S7 Smart-Home Hub — Backlog

---

## Now

- [ ] Write `docs/hub/reference-hardware.md` (can begin before G3)
- [ ] Design pairing flow (device profile + user profile → capability grants)
- [ ] Identify target Pi / home server hardware for first private-beta install

## Next (after G3)

- [ ] Scaffold `crates/ferros-hub/`
- [ ] Implement pairing flow using S2 profile API
- [ ] Implement HA bridge agent using S3 `Agent` trait
- [ ] Wire `ferros-hub` to `ferros-runtime` (S4) and `ferros-agents` (S3)
- [ ] aarch64 cross-compilation target in CI
- [ ] Smoke test on x86_64 home server first

## Later

- [ ] Power cycle test on aarch64 / Raspberry Pi
- [ ] `docs/hub/install.md` install script
- [ ] At least one independent private-beta install
- [ ] Verify G4 checklist (see `docs/gates/G4.md`)

## Blocked

- Implementation blocked on G3.
- HA custom component depends on `Maangled/home-assistant` fork being available.
