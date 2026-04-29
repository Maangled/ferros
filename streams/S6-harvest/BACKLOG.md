# S6 Ecosystem Harvest — Backlog

---

## Now

- [x] Audit `Maangled/botgen-rust` — agent spawning patterns (P0: unblocks S3)
- [x] Audit `Maangled/workpace-rust` — workspace/session model (P0: informs S5)
- [x] Audit `Maangled/sheetgen-rust` — data generation primitives (P1)

## Next

- [x] Write ADR-018: harvest `botgen-rust` → agent spawning patterns
- [x] Write ADR-019: harvest `workpace-rust` → agent center UX shell
- [x] Write ADR-020: harvest `sheetgen-rust` → `ferros-data`
- [x] Scaffold `crates/ferros-data/` with a standalone ADR-020-aligned migration-first crate shell
- [x] Admit `crates/ferros-data/` to the root workspace
- [x] Verify `ferros-data` compiles under the FERROS workspace
- [x] Publish the typed local onramp proposal boundary in `ferros-data` with bounded local-only validation/write helpers for downstream hub consumers
- [x] Hand off the local onramp proposal boundary to the current S7/S4/S5 rehearsal packet without reopening ADR-020 or inventing a second proposed-material model

## Later

- [ ] Git subtree integration for `sheetgen-rust` primitives (if license permits)
- [ ] Extract workpace session model into S5 Phase B scaffold

## Blocked

No active blocker on the `ferros-data` workspace-admission slice. Downstream consumer wiring remains intentionally out of scope for S6.
