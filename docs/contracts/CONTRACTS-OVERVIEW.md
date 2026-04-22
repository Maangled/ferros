# FERROS — Contracts Overview

> This document is the index of all cross-stream contracts in FERROS. A contract is any interface that two or more streams depend on: a Rust trait, a JSON Schema, a CLI API, or a protocol specification.

---

## What counts as a contract

- A **Rust trait** in `crates/ferros-core/` or `crates/ferros-agents/` that is implemented by one stream and called by another.
- A **JSON Schema** in `schemas/` that defines a data format used by more than one stream.
- A **CLI API** (subcommand + arguments) that other streams depend on being stable.
- A **wire protocol** (IPC bus message format, JSON/RPC API) that crosses stream boundaries.

What does not count as a contract:
- Internal types used only within one crate.
- Implementation details not visible at a crate boundary.
- Documentation conventions (those are in `streams/SN-*/CONTRACTS.md`).

---

## Contract ownership rules

1. **One owner per contract.** The owning stream is the only stream that may change a contract's definition. Consuming streams may request changes via issue or PR, but the owner merges.
2. **Frozen schemas are immutable.** Once a schema version is frozen (e.g., `profile.v0.json`), it must not be mutated in place. New versions get a new file (`profile.v1.json`) with explicit migration rules.
3. **Trait changes require downstream sign-off.** If a trait in `ferros-core/` changes its signature, the owning stream must update all downstream impls in the same PR, or provide a deprecation shim.

---

## Stream A freeze protocol (current)

This repository also operates a Stream A contract set (C1-C10) used by the Phase/Wave tracker in `docs/progress/PROGRESS.md`.

### Freeze target

- **Target:** `v1.0.0` contract freeze for C1-C10
- **Scope:** JSON schemas, contract docs, fixture semantics, and harness expectations
- **Authority:** `docs/contracts/manifest.json` (`governance.streamAContractFreeze`)

### Trigger rule

Freeze executes when Wave 2 consumer-surface threshold is met:

1. Read Tier 3 (`S1-S4`) in `docs/progress/PROGRESS.md`.
2. Count completed surfaces.
3. Execute freeze when completed surfaces >= 3.

### Execution checklist

1. Set `governance.streamAContractFreeze.status` to active.
2. Record `manifestFrozenAt` with the commit SHA where freeze is declared.
3. Add evidence reference in `evidenceRef` (closure doc or progress evidence note).
4. Keep frozen versions immutable; all future evolution follows ADR-012.

### Evolution after freeze

- New additive fields: next minor version and fixtures/harness updates.
- Breaking field changes: next major version with explicit migration artifacts.
- No in-place edits to frozen versions.

---

## Contract index

### Rust traits

| Trait | Owner | Crate | Consumers | Status |
|-------|-------|-------|-----------|--------|
| `Agent` | S3 | `ferros-agents` | S4 (executor), S7 (hub) | ⬜ Not yet defined |
| `AgentRegistry` | S3 | `ferros-agents` | S5 (web shell), S7 (hub) | ⬜ Not yet defined |
| `PolicyEngine` | S4 | `ferros-core` | S3 (spawn gate), S7 (hub consent) | ⬜ Not yet defined |
| `Executor` | S4 | `ferros-runtime` | S3 (agent hosting) | ⬜ Not yet defined |

### JSON Schemas

| Schema | Owner | Path | Consumers | Status |
|--------|-------|------|-----------|--------|
| Profile | S2 | `schemas/profile.v0.json` | S3, S7 | ⬜ Not yet created |
| CapabilityGrant | S2 | `schemas/capability-grant.v0.json` | S3, S4, S7 | ⬜ Not yet created |
| AgentManifest | S3 | `schemas/agent-manifest.v0.json` | S4, S5, S7 | ⬜ Not yet created |

### CLI APIs

| Command | Owner | Consumers | Status |
|---------|-------|-----------|--------|
| `ferros profile init\|show\|export\|import\|grant\|revoke` | S2 | S7 (pairing scripts), S8 (docs) | ⬜ Not yet implemented |
| `ferros agent list\|describe\|run\|stop\|logs` | S3 | S5 (web shell), S7 (hub admin) | ⬜ Not yet implemented |

### IPC / RPC

| Protocol | Owner | Consumers | Status |
|----------|-------|-----------|--------|
| In-process bus (Unix socket / named pipe) | S4 | S3 (agent routing) | ⬜ Not yet defined |
| JSON/RPC HTTP API (agent center) | S3 | S5 Phase B (web shell) | ⬜ Not yet defined |

---

## How to add a new contract

1. Identify which stream **owns** the contract.
2. Add a row to the appropriate table above.
3. Add the contract to the owning stream's `CONTRACTS.md`.
4. If the contract is a new schema, create the file in `schemas/` under the owning stream's responsibility.
5. Open a PR tagged with the owning stream label.

---

## Schema versioning

Schema files follow the pattern `schemas/{name}.v{N}.json`. Version `0` schemas are pre-freeze drafts. Version `0` schemas are frozen at the corresponding gate (G2 for profile schemas). After freeze:

- Add optional fields → new version `v{N+1}`.
- Remove or rename fields → new version `v{N+1}` with a migration ADR.
- Never mutate a frozen version in place.

Downstream consumers must pin the version they target and update when they choose to migrate.
