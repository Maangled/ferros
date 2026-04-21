# S6 — Ecosystem Harvest

**Stream:** S6  
**Status:** ⬜ Blocked on G1  
**Gate:** Rolling (feeds S3 and S7; no single gate owned by S6)

---

## Mission

FERROS is not starting from zero. `sheetgen-rust`, `botgen-rust`, and `workpace-rust` contain exercised architectural patterns that belong in FERROS as first-class primitives. This stream makes the harvest **explicit work** so rewriting-from-scratch does not masquerade as "new work."

---

## Scope

For each source repository, decide per module: **lift / rewrite / discard**. Produce a migration ADR. Extract stable primitives into the FERROS workspace.

| Source repo | Target in FERROS | Pattern to harvest |
|-------------|------------------|--------------------|
| `Maangled/sheetgen-rust` | `ferros-data` crate | Data/sheet generation primitives |
| `Maangled/botgen-rust` | `ferros-agents` (S3) | Bot materialization from descriptions; agent spawning patterns |
| `Maangled/workpace-rust` | S5 Phase B web shell | Workspace/session model; UX shell patterns |

---

## Out of scope

- Implementing the agent center itself (S3).
- Implementing the UX shell itself (S5).
- Rewriting the Home Assistant fork (S7).

---

## Dependencies

- **S1 (G1):** Cargo workspace must exist to add harvested crates.
- **S4 traits:** Harvested agent patterns must conform to S4's executor contract.

---

## What this stream blocks

- **S3:** `botgen-rust` harvest ADR should complete before S3 implementation begins.
- **S7:** `workpace-rust` patterns inform the hub session model.

---

## Definition of done (per source repo)

For each of `sheetgen-rust`, `botgen-rust`, `workpace-rust`:

- [ ] ADR written: what was audited, what was harvested, what was discarded, and why.
- [ ] Harvested code compiles under the FERROS workspace.
- [ ] Harvested code is relicensed or attribution is recorded in the ADR.
- [ ] Git provenance preserved where reasonable (`git subtree` or attribution in ADR).

---

## Likely crates / files

| Path | Role |
|------|------|
| `crates/ferros-data/` | Primitives from `sheetgen-rust` |
| `docs/adr/ADR-0016-harvest-sheetgen.md` | Harvest ADR for `sheetgen-rust` |
| `docs/adr/ADR-0017-harvest-botgen.md` | Harvest ADR for `botgen-rust` |
| `docs/adr/ADR-0018-harvest-workpace.md` | Harvest ADR for `workpace-rust` |

---

## Immediate next steps

1. Audit `Maangled/sheetgen-rust` — list modules; decide lift/rewrite/discard per module.
2. Audit `Maangled/botgen-rust` — identify agent spawning patterns; compare to S3 `Agent` trait.
3. Audit `Maangled/workpace-rust` — identify workspace/session model; compare to S5 Phase B design.
4. Write ADR-0016 (sheetgen), ADR-0017 (botgen), ADR-0018 (workpace).
5. Begin extraction of stable primitives into `ferros-data`.
