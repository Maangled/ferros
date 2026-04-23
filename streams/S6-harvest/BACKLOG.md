# S6 Ecosystem Harvest — Backlog

---

## Now

- [ ] Audit `Maangled/botgen-rust` — agent spawning patterns (P0: unblocks S3)
- [ ] Audit `Maangled/workpace-rust` — workspace/session model (P0: informs S5)
- [ ] Audit `Maangled/sheetgen-rust` — data generation primitives (P1)

## Next

- [ ] Write ADR-018: harvest `botgen-rust` → agent spawning patterns
- [ ] Write ADR-019: harvest `workpace-rust` → agent center UX shell
- [ ] Write ADR-020: harvest `sheetgen-rust` → `ferros-data`
- [ ] Scaffold `crates/ferros-data/` with harvested sheetgen primitives once the sheetgen harvest direction is fixed
- [ ] Verify harvested code compiles under FERROS workspace

## Later

- [ ] Git subtree integration for `sheetgen-rust` primitives (if license permits)
- [ ] Extract workpace session model into S5 Phase B scaffold

## Blocked

No upstream G1 blocker remains. Audits and ADR writing are active now; crate extraction should wait until the harvest direction is settled.
