# S6 Ecosystem Harvest — Contracts

---

## Contracts owned by S6

S6 is a harvest and migration stream. It produces ADRs and extracted crates; it does not own runtime contracts.

| Contract | Type | Location | Status |
|----------|------|----------|--------|
| ADR-0016: sheetgen harvest decision | ADR | `docs/adr/ADR-0016-harvest-sheetgen.md` | ⬜ Not yet written |
| ADR-0017: botgen harvest decision | ADR | `docs/adr/ADR-0017-harvest-botgen.md` | ⬜ Not yet written |
| ADR-0018: workpace harvest decision | ADR | `docs/adr/ADR-0018-harvest-workpace.md` | ⬜ Not yet written |
| `ferros-data` crate public API | Rust types | `crates/ferros-data/` | ⬜ Not yet created |

---

## Contracts consumed by S6

| Contract | Source | Purpose |
|----------|--------|---------|
| `Agent` trait | S3/S4 | Harvested botgen patterns must conform to this |
| Cargo workspace | S1 | Extracted crates added to workspace |

---

## Downstream consumers

| Stream | What it consumes |
|--------|-----------------|
| S3 Agent Center | `botgen-rust` patterns inform `Agent` trait implementation |
| S5 UX | `workpace-rust` patterns inform Phase B web shell design |
| S7 Hub | Harvested data primitives may be used by hub agents |
