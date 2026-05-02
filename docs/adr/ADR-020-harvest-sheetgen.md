# ADR-020 — Harvest sheetgen-rust for ferros-data Invariants and Tooling Discipline

**Status:** Accepted  
**Date:** 2026-04-23  
**Stream:** S6  
**Deciders:** FERROS stream coordination / S6 harvest lane

---

## Context

`Refractov/sheetgen-rust` contains the strongest prior art for a future `ferros-data` layer: migration-first relational design, database-enforced invariants, JSONB materialized snapshots, z-ordering constraints, command-catalog tooling, and schema-derived testing ideas. The audit in `.tmp/sg-r.md` also shows the cost of letting too many schema surfaces coexist: sheetgen-rust carries Rust domain types, SQL migrations, and a JSONC logical schema document that drift from one another. FERROS needs the strong invariant patterns without inheriting the drift and plumbing debt.

---

## Decision

**FERROS will harvest sheetgen-rust as the prior-art basis for `ferros-data`, with SQL migrations as the authoritative source for relational invariants. Rust types, tests, and tooling must be derived from or mechanically checked against that source. FERROS will not maintain a third handwritten logical-schema document that can drift from code and DDL.**

Additional decisions:

- Adopt database-enforced invariants such as polymorphic-parent `CHECK` constraints and versioned JSONB snapshots.
- Adapt ordering, cache, asset-storage, and command-catalog patterns into FERROS-owned primitives.
- Discard manual enum handlers, handwritten dynamic SQL builders, runtime mutation of committed contract files, and premature stringification of typed values.

---

## Rationale

The strongest lesson from sheetgen-rust is not any single utility type. It is that relational invariants belong in the database, but tooling must keep every other representation aligned with that truth. The repo demonstrates valuable patterns for snapshots, ordering, cache invalidation, and schema-aware command tooling, while also showing how manual duplication creates long-term drift. Choosing migration-first authority for `ferros-data` lets FERROS preserve correctness-critical constraints where they are enforceable and keep higher-layer code honest.

### Options considered

| Option | Summary | Reason not chosen (if rejected) |
|--------|---------|--------------------------------|
| Migration-first `ferros-data` with checked or generated dependents (chosen) | SQL migrations own relational invariants; Rust types and tooling are derived or mechanically checked | — |
| Code-first Rust structs with generated migrations | Rust types own the model and SQL is emitted from code | Rejected because FERROS is likely to need advanced Postgres invariants that are clearer and safer to express directly in DDL |
| Three parallel schema sources | Keep Rust structs, SQL migrations, and a handwritten logical-spec document in sync manually | Rejected because sheetgen-rust shows that this drifts quickly and creates tooling debt |

---

## Consequences

**Positive:**
- Future `ferros-data` work has a clear authority boundary before code is written.
- FERROS can adopt strong database invariants early instead of treating them as optional ORM details.
- S8 tooling now has a concrete rule: generate or validate contracts from owned descriptors, never write source-controlled contract files at runtime.
- S6 can separate useful data-layer patterns from sheetgen-rust's manual plumbing and anti-patterns.

**Negative / trade-offs:**
- FERROS will need stronger migration and database-test discipline early.
- Contributors cannot treat ad hoc Rust model changes as authoritative if the relational invariant lives in migrations.
- Some tooling work moves earlier because parity checks between migrations, Rust types, and external contracts become mandatory.

---

## Adopt / Adapt / Reference / Discard

### Adopt

- Migration-first schema discipline for relational invariants.
- Versioned JSONB snapshot or read-model pattern with indexed latest-version retrieval.
- Dual validation for polymorphic parents: authoritative database `CHECK` constraint plus application-side pre-validation for better error messages.
- A shared `revision_base` concept for versioned entities, including identity, lineage, and timestamps.
- Ordered multi-level composition hierarchies when the domain genuinely has layered authored surfaces, for example `workspace -> project -> sheet -> titleblock -> viewport -> view -> drawing -> annotation`.
- Child-owned visibility and cropping semantics derived from explicit parentage rather than inferred from loose application state.
- Exact-one-parent polymorphism enforced in DDL with `CHECK` constraints and partial indexes when one logical entity may attach to several parent kinds but must belong to only one at a time.

### Adapt

- Z-ordering invariants into a reusable `ferros-data` ordering primitive, using sheetgen-rust's per-parent unique indexes and type-banded ordering as strong prior art while still allowing FERROS to choose a simpler owned allocator.
- Command registry and contract-generation ideas into S8 tooling, but use dedicated tooling or `xtask` flows instead of startup-time source rewrites.
- Redis JSON cache and invalidation patterns behind FERROS-owned abstractions.
- Blob-storage pathing and placeholder-then-replace upload workflows behind a `BlobStorage` trait.
- Mixins and feature-gated schema composition into FERROS-owned code generation or checked descriptors, not a drifting handwritten JSONC file.

### Reference

- Domain and storage model separation as an architecture boundary, while avoiding sheetgen-rust's type duplication and string-based workarounds.
- PL/pgSQL text-to-enum conversion helpers as a fallback approach when derive-based enum mapping is unavailable.
- Schema-derived integration-test generation as an S8 tooling idea worth reviving with typed descriptors.

### Discard

- Manual `tokio-postgres` enum handlers where derive-based support exists.
- Handwritten dynamic SQL update builders with manually tracked parameter indexes.
- Runtime writes to committed contract files such as `definitions.yaml`.
- Premature stringification of timestamps or enums in storage models.
- Any approach that leaves three independently maintained schema sources in play.
- Flattened or JSON-only hierarchy models that hide parent-child invariants from the database when the domain actually needs enforceable nesting rules.

---

## Downstream Implications

### S6 Ecosystem Harvest

- This ADR closes the first sheetgen-rust audit pass.
- `crates/ferros-data/` should start only after its initial migration authority and invariant boundaries are consistent with this ADR.

### S8 Docs / Governance / Tooling

- S8 should treat this ADR as the rule for future contract-generation or command-catalog tooling.
- Generated contract documents must come from a dedicated tool path and must never be mutated as a side effect of normal runtime startup.

### Future ferros-data work

- The first `ferros-data` slice should prove migration authority, at least one database-enforced invariant, and one parity or smoke check that the Rust layer matches the owned schema.
- Ordering, cache, snapshot, and blob-storage helpers should be introduced as explicit FERROS abstractions, not pasted from sheetgen-rust repositories.

---

## Compliance

- If FERROS introduces a handwritten logical schema file for relational data, revisit this ADR.
- If a future `ferros-data` slice chooses code-first schema generation instead of migration-first authority, revisit this ADR explicitly.
- If S8 tooling starts writing source-controlled contract files during runtime, treat that as non-compliant with this ADR.

---

## References

- `.tmp/sg-r.md`
- [ADR-013](c:\Users\mosle\OneDrive\Desktop\GPTs\ferros\ferros\docs\adr\ADR-013-legacy-integration-strategy.md)
- [streams/S6-harvest/README.md](c:\Users\mosle\OneDrive\Desktop\GPTs\ferros\ferros\streams\S6-harvest\README.md)
- [streams/S8-docs/README.md](c:\Users\mosle\OneDrive\Desktop\GPTs\ferros\ferros\streams\S8-docs\README.md)