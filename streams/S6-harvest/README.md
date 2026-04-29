# S6 — Ecosystem Harvest

**Stream:** S6  
**Status:** 🟡 Active  
**Gate:** Rolling (feeds S3 and S7; no single gate owned by S6)

---

## Mission

FERROS is not starting from zero. `sheetgen-rust`, `botgen-rust`, `workpace-rust`, and the Home Assistant fork contain exercised architectural patterns and operational lessons that may belong in FERROS as first-class primitives. This stream makes the harvest **explicit work** so rewriting-from-scratch does not masquerade as "new work," and so downstream implementation streams consume reviewed ADRs instead of raw prior-art code.

---

## Scope

For each source repository, decide per module or subsystem: **adopt / adapt / reference / discard**. Produce a harvest ADR. Extract only stable primitives into the FERROS workspace after the ADR is accepted.

| Source repo | Target in FERROS | Pattern to harvest |
|-------------|------------------|--------------------|
| `Maangled/botgen-rust` | `ferros-agents` (S3) / S4 prep | Agent lifecycle, registry shape, work queue, materialization from descriptions |
| `Maangled/workpace-rust` | S5 Phase B web shell | Workspace/session model; UX shell patterns |
| `Maangled/sheetgen-rust` | `ferros-data` crate | Data/sheet generation primitives; schema discipline |
| `Maangled/home-assistant` | S7 reference only | Integration semantics, entity model, and operational patterns; not direct code lift |

## External Prior-Art Policy

- S6 is the only stream that reads legacy repos as raw input.
- S2, S3, S4, S5, and S7 should consume S6 ADRs and accepted conclusions, not mine the old repos directly during implementation.
- Mechanical plumbing may be adopted with attribution when it does not affect FERROS invariants: CI shapes, build scripts, Fly or Docker skeletons, and similar undifferentiated scaffolding.
- Type-level, policy-level, identity, grant, and consent code must be adapted or rewritten under FERROS invariants, not bulk-ported.

---

## Out of scope

- Bulk-porting prior repos into FERROS without an ADR decision.
- Letting legacy repo abstractions directly define S2 identity, S3 agent, or S4 runtime contracts.
- Implementing the agent center itself (S3).
- Implementing the UX shell itself (S5).
- Rewriting the Home Assistant fork (S7).

---

## Dependencies

- **S1:** Cargo workspace exists; crate extraction is no longer blocked on G1.
- **S3 / S4 traits:** Harvested agent patterns must conform to FERROS-owned interfaces rather than legacy shapes.

---

## What this stream blocks

- **S3 / S4:** ADR-018 is now the handoff surface for registry, lifecycle, queue, and materialization decisions; downstream streams should consume it instead of mining `botgen-rust` directly.
- **S5:** ADR-019 is now the handoff surface for workspace/session, typed IPC, and shell-delivery conclusions; any later extraction remains follow-on work rather than fresh ADR authoring.
- **S7:** Home Assistant prior art remains reference-scoped; the local onramp rehearsal packet now consumes the `ferros-data` proposal boundary instead of inventing a second proposed-material model, and hub work should continue consuming S6 conclusions rather than the legacy fork directly.

---

## Current completion baseline

- [x] ADR-018, ADR-019, and ADR-020 are accepted and record the adopt/adapt/reference/discard verdicts for `botgen-rust`, `workpace-rust`, and `sheetgen-rust`.
- [x] `crates/ferros-data/` is admitted to the root Cargo workspace as the ADR-020-aligned scaffold.
- [x] `ferros-data` now owns the typed local onramp proposal boundary consumed by the current local hub/shell/harness rehearsal packet.
- [ ] Git provenance and attribution still need to be preserved where later code lift is warranted, especially for any deeper `sheetgen-rust` extraction.
- [ ] Downstream implementation streams still need to consume these accepted decisions in their own lanes; that wiring remains outside S6.

---

## Likely crates / files

| Path | Role |
|------|------|
| `crates/ferros-data/` | ADR-020-aligned `sheetgen-rust` scaffold admitted to the root workspace; now hosts local-push and local-onramp proposal primitives |
| `docs/adr/ADR-018-harvest-botgen.md` | Accepted harvest ADR for `botgen-rust` |
| `docs/adr/ADR-019-harvest-workpace.md` | Accepted harvest ADR for `workpace-rust` |
| `docs/adr/ADR-020-harvest-sheetgen.md` | Accepted harvest ADR for `sheetgen-rust` |

---

## Current next work

1. Preserve provenance for any later lifted `sheetgen-rust` primitives, including history-carrying approaches such as `git subtree` when the slice and license make that worthwhile.
2. Keep `ferros-data` aligned with ADR-020 as a migration-first scaffold and current local-only primitive host now that it is a root workspace member.
3. Hand off ADR-018, ADR-019, and ADR-020 as the approved prior-art boundary for S3, S4, S5, and S7; downstream consumer wiring remains with those owning streams.
