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

- **S3:** `botgen-rust` harvest ADR should complete before S3 implementation hardens around a registry or lifecycle shape.
- **S5:** `workpace-rust` harvest ADR should complete before the Phase B web shell hardens around a workspace/session model.
- **S7:** Home Assistant prior art and pairing-related patterns should be reference-scoped before hub implementation hardens.

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
| `docs/adr/ADR-018-harvest-botgen.md` | Harvest ADR for `botgen-rust` |
| `docs/adr/ADR-019-harvest-workpace.md` | Harvest ADR for `workpace-rust` |
| `docs/adr/ADR-020-harvest-sheetgen.md` | Harvest ADR for `sheetgen-rust` |

---

## Immediate next steps

1. Audit `Maangled/botgen-rust` first — identify agent lifecycle, registry, work queue, and materialization patterns; compare them to FERROS S3 and S4 seams.
2. Write ADR-018 for `botgen-rust` with explicit adopt/adapt/reference/discard verdicts.
3. Audit `Maangled/workpace-rust` second — identify workspace/session model patterns for S5 Phase B.
4. Write ADR-019 for `workpace-rust`.
5. Audit `Maangled/sheetgen-rust` third — identify data and schema-discipline primitives for `ferros-data`.
6. Write ADR-020 for `sheetgen-rust`, then decide whether code extraction is warranted.
