# ADR-014: Three-Layer Decomposition

## Status
Proposed — applies when Rust code lands

## Context

sheetgen-rust organizes its source into three layers:

- `src/domain/` — business rules, types, validation logic
- `src/storage/` — persistence adapters (PostgreSQL repositories)
- `src/api/` — HTTP surface, request/response mapping

This separation proved effective: domain logic remained testable without a database,
storage adapters could be swapped without touching business rules, and the API layer
stayed thin.

FERROS Phase 0 uses a monolith (`docs/personal-profile.html` + `FerrosCore.js`). The
three-layer split does not apply to this phase. When Rust code begins — whether for the
OS layer, a contract validator, or the first `ferros-core` crate — this ADR defines the
canonical decomposition.

## Decision

Adopt the three-layer pattern as the standard FERROS Rust decomposition:

| Layer | Crate / Directory | Equivalent in sheetgen-rust |
|-------|------------------|-----------------------------|
| Contracts + domain logic | `ferros-core/` | `src/domain/` |
| Persistence adapters | `ferros-storage/` | `src/storage/` |
| UI surfaces + API endpoints | `ferros-surfaces/` | `src/api/` |

### `ferros-core/`

The shared Rust core. Modelled on botgen-rust's `core/shared/` crate, which provides:

- `prelude.rs` — common re-exports
- `types.rs` — shared type definitions
- `macros.rs` — derive macros for common behavior
- `error.rs` — unified error types
- `config.rs` — configuration management

`ferros-core/` will hold: JSON schema validators (generated from C1–C10), domain types,
the agent host protocol (C8 runtime host contract), and the audit record enforcer (C7).

### `ferros-storage/`

Persistence adapters. Phase 0 uses localStorage/IndexedDB (JS). In Rust phases, this
layer wraps whatever storage backend is active (CoW filesystem, SQLite embedded, etc.)
without leaking storage details into `ferros-core/`.

### `ferros-surfaces/`

HTML surfaces and, eventually, native UI surfaces. Depends on `ferros-core/`; has no
direct dependency on `ferros-storage/` (storage access goes through `ferros-core/`
contracts).

### Dependency rule

```
ferros-surfaces → ferros-core
ferros-storage  → ferros-core
ferros-surfaces ⛔ ferros-storage  (no direct cross-dependency)
```

This mirrors botgen-rust's `core/` → `services/` → `bots/` layering, which enforced no
cyclic dependencies across its three tiers.

## Consequences

### Positive

- Domain logic in `ferros-core/` is testable without any UI or storage infrastructure.
- Storage adapters can be replaced (localStorage → CoW filesystem → cloud relay) without
  touching domain types or surface code.
- The layer boundary is the right place to enforce schema validation: `ferros-core/`
  validates; `ferros-storage/` persists; `ferros-surfaces/` renders.

### Negative

- This ADR is future-facing. Phase 0 does not use it. Agents must not create a Rust
  workspace prematurely to satisfy this ADR.
- The three-crate split adds initial boilerplate for the first Rust contributor. The
  tradeoff is worth it for the long-term layering discipline.

### Activation condition

This ADR is Proposed. It becomes Accepted when the first Rust source file lands in the
repository. At that point, the three-layer structure defined here is binding. The wave
entry condition for L4 (Wave 1) is the trigger: when Wave 1 begins and Rust code is
introduced, this ADR transitions to Accepted simultaneously.

## Related

- [ADR-013](./ADR-013-legacy-integration-strategy.md) — overall legacy integration
  strategy; sheetgen-rust L4 in the tracking table
- [ADR-0001](./ADR-0001-start-new-do-not-fork.md) — Rust from scratch; no forking of
  predecessor codebases
