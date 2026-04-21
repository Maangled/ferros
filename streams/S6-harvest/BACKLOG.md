# S6 Ecosystem Harvest — Backlog

---

## Now

- [ ] Audit `Maangled/botgen-rust` — agent spawning patterns (P0: unblocks S3)
- [ ] Audit `Maangled/workpace-rust` — workspace/session model (P0: informs S5)
- [ ] Audit `Maangled/sheetgen-rust` — data generation primitives (P1)

## Next

- [ ] Write ADR-0017: harvest `botgen-rust` → agent spawning patterns
- [ ] Write ADR-0018: harvest `workpace-rust` → agent center UX shell
- [ ] Write ADR-0016: harvest `sheetgen-rust` → `ferros-data`
- [ ] Scaffold `crates/ferros-data/` with harvested sheetgen primitives (after G1)
- [ ] Verify harvested code compiles under FERROS workspace

## Later

- [ ] Git subtree integration for `sheetgen-rust` primitives (if license permits)
- [ ] Extract workpace session model into S5 Phase B scaffold

## Blocked

- Crate extraction blocked on G1 (Cargo workspace must exist).
- Audits and ADR writing can proceed immediately.
