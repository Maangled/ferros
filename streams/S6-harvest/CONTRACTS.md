# S6 Ecosystem Harvest — Contracts

---

## Contracts owned by S6

S6 is a harvest and migration stream. It produces ADRs and extracted crates; it does not own runtime contracts.

| Contract | Type | Location | Status |
|----------|------|----------|--------|
| ADR-018: botgen harvest decision | ADR | `docs/adr/ADR-018-harvest-botgen.md` | ✅ Accepted |
| ADR-019: workpace harvest decision | ADR | `docs/adr/ADR-019-harvest-workpace.md` | ✅ Accepted |
| ADR-020: sheetgen harvest decision | ADR | `docs/adr/ADR-020-harvest-sheetgen.md` | ✅ Accepted |
| Local Onramp Proposal | Rust boundary + local schema | `crates/ferros-data/src/lib.rs`, `schemas/onramp-proposal.schema.json` | 🟡 Published as an S6-owned local-only proposal boundary for the current S7 or S4 or S5 rehearsal packet; not a canonical mutation, remote transport, or gate-closing contract |
| Local Onramp Decision Receipt | Rust boundary + local schema | `crates/ferros-data/src/lib.rs`, `schemas/onramp-decision-rehearsal.schema.json` | 🟡 Published as an S6-owned local-only decision-receipt boundary for the current S7 or S4 or S5 rehearsal packet; not an accept or reject transport, canonical mutation, remote transport, or gate-closing contract |

S6 ADRs are the approved boundary by which external prior art enters FERROS. Downstream streams should consume the ADR decisions, not raw legacy repositories, unless a later governance change says otherwise.

S6 does not claim the whole `ferros-data` public API as a cross-stream contract. The owned extracted surfaces are the local-only `LocalOnrampProposal` and `LocalOnrampDecisionReceipt` boundaries above; the shared local-runway guardrail layer in `ferros-data` remains explanatory support for those seams rather than a separate top-level contract surface.

---

## Contracts consumed by S6

| Contract | Source | Purpose |
|----------|--------|---------|
| `Agent` trait | S3/S4 | Harvested botgen patterns must conform to this |
| Cargo workspace | S1 | Hosts extracted crates and validates narrow admission slices |

---

## Downstream consumers

| Stream | What it consumes |
|--------|-----------------|
| S3 Agent Center | ADR-backed `botgen-rust` decisions inform `Agent` trait implementation |
| S4 Runtime | ADR-backed `botgen-rust` decisions inform policy and lifecycle prep |
| S5 UX | ADR-backed `workpace-rust` decisions inform Phase B web shell design |
| S7 Hub | Reference-scoped Home Assistant and harvested data decisions inform hub architecture |
