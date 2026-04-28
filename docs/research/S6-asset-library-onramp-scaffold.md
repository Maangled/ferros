# S6 Asset-Library Onramp Scaffold Boundary

Status: Batch F research handoff. This sketches the asset/library boundary as proposed material under ADR-023 and ADR-024 constraints.

## Fixed Inputs

- ADR-020 accepted the migration-first `ferros-data` direction for future data primitives.
- ADR-023 accepted the onramp policy: inbound material is proposed until an operator accepts it.
- ADR-024 remains Proposed. No chain or legal-substrate decision is ratified here.
- S6 remains the route for prior-art harvest; other streams should consume accepted S6 conclusions rather than raw legacy repo shapes.

## Scaffold Boundary

The first asset-library scaffold should be treated as an onramp source, not as canonical state:

| Layer | Boundary |
|-------|----------|
| Proposed asset | Imported or generated candidate material with source attribution |
| Review surface | S5 onramp consent surface shows proposed fields and accept/reject controls |
| Accepted record | FERROS-owned state written only after explicit operator acceptance |
| Data storage | Future `ferros-data` work may provide migration-backed storage once scoped |
| Audit trail | Local accepted/rejected decision record; no chain finality claim |

## What S6 Owns

- Provenance and harvest discipline for any future asset-library prior art.
- ADR-020 alignment if `ferros-data` becomes the storage home.
- Attribution notes when code or schema ideas are adapted from prior repositories.

## What Other Streams Own

| Stream | Ownership |
|--------|-----------|
| S5 | Operator review and consent surface |
| S7 | Any HA-origin material entering as proposed onramp items |
| S8 | Contributor-facing glossary and docs navigation |
| System track | ADR-024 ratification or legal/counsel decisions |

## Next Implementation Questions

1. Does the first asset proposal use static fixtures, local files, or generated sample material?
2. Does accepted material need `ferros-data` storage immediately, or can the first pass stay fixture-backed?
3. Which source attribution fields are mandatory before an operator can accept material?
4. What local audit shape is sufficient before any ledger substrate is ratified?

## Stop Lines

- Do not ratify ADR-024 from this scaffold.
- Do not claim legal finality, chain anchoring, or public ledger readiness.
- Do not import prior-art code without attribution and an explicit implementation wave.
- Do not bypass ADR-023 quarantine by writing proposed material directly into canonical state.
