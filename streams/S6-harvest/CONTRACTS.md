# S6 Ecosystem Harvest — Contracts

---

## Contracts owned by S6

S6 is a harvest and migration stream. It produces ADRs and extracted crates; it does not own runtime contracts.

| Contract | Type | Location | Status |
|----------|------|----------|--------|
| ADR-018: botgen harvest decision | ADR | `docs/adr/ADR-018-harvest-botgen.md` | ⬜ Not yet written |
| ADR-019: workpace harvest decision | ADR | `docs/adr/ADR-019-harvest-workpace.md` | ⬜ Not yet written |
| ADR-020: sheetgen harvest decision | ADR | `docs/adr/ADR-020-harvest-sheetgen.md` | ⬜ Not yet written |
| `ferros-data` crate public API | Rust types | `crates/ferros-data/` | ⬜ Not yet created |

S6 ADRs are the approved boundary by which external prior art enters FERROS. Downstream streams should consume the ADR decisions, not raw legacy repositories, unless a later governance change says otherwise.

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
| S3 Agent Center | ADR-backed `botgen-rust` decisions inform `Agent` trait implementation |
| S4 Runtime | ADR-backed `botgen-rust` decisions inform policy and lifecycle prep |
| S5 UX | ADR-backed `workpace-rust` decisions inform Phase B web shell design |
| S7 Hub | Reference-scoped Home Assistant and harvested data decisions inform hub architecture |
