# S4 — Runtime / OS Core

**Stream:** S4  
**Status:** ⬜ Blocked on G1  
**Gate:** G3 (jointly with S3)

---

## Mission

Build the "OS" layer: the capability and consent primitives, the in-process message bus, the executor, and the binary that hosts everything. This stream runs in parallel with S2/S3 via trait stubs and converges when S2 profile types land.

---

## Scope

- `ferros-core` crate:
  - Capability and consent primitives (types for capabilities, policy rules, decisions).
  - Message envelope type (sender, recipient, capability, payload, nonce).
  - Policy engine: evaluate a `CapabilityGrant` against a request → allow/deny.
  - `no_std` feature flag on `ferros-core` (opt-in) for future embedded targets.
- `ferros-runtime` crate:
  - In-process executor (single-threaded to start; multi-threaded opt-in later).
  - In-process message bus: route messages between hosted agents.
  - Deny-by-default: all capability requests go through the policy engine.
- `ferros-node` binary:
  - Hosts `ferros-runtime` + `ferros-agents` (S3) in a single process.
  - `ferros-node demo` subcommand: starts the runtime, registers the two reference agents, runs a deny-by-default capability check, prints results, exits.

---

## Out of scope

- Identity / keypair management (S2).
- Agent registry UI or CLI (S3).
- Web shell (S5).
- Home Assistant integration (S7).

---

## Dependencies

- **S1 (G1 must be green):** Cargo workspace and CI.
- **S2 (converges at G2):** Policy engine needs `CapabilityGrant` type. Use stub until S2 lands.

---

## What this stream blocks

- **S3 runtime hooks:** The executor interface (`Agent` trait, bus protocol) is owned here.
- **S7 Hub:** `ferros-hub` wraps `ferros-node` and depends on the runtime being stable.

---

## Definition of done (G3, jointly with S3)

- [ ] `ferros-core` and `ferros-runtime` build and pass `cargo test`.
- [ ] `ferros-node demo` runs deterministically: registers reference agents, proves deny-by-default, exits 0.
- [ ] 10+ unit tests covering capability grant/deny scenarios.
- [ ] Property tests (via `proptest` or `quickcheck`) for the policy engine.
- [ ] `no_std` feature compiles without `std` for `ferros-core`.

---

## Likely crates / files

| Path | Role |
|------|------|
| `crates/ferros-core/` | Capability + consent primitives |
| `crates/ferros-core/src/capability.rs` | Capability type + policy engine |
| `crates/ferros-core/src/message.rs` | Message envelope |
| `crates/ferros-runtime/` | Executor + in-process bus |
| `crates/ferros-runtime/src/executor.rs` | Task executor |
| `crates/ferros-runtime/src/bus.rs` | In-process message bus |
| `crates/ferros-node/` | Binary crate |
| `crates/ferros-node/src/demo.rs` | `demo` subcommand |

---

## Immediate next steps

1. Scaffold `crates/ferros-core/` with stub types (can start before G2 using placeholder grant type).
2. Implement policy engine with unit tests.
3. Scaffold `crates/ferros-runtime/` with executor and bus.
4. Scaffold `crates/ferros-node/` binary with `demo` subcommand.
5. Replace stub grant type with S2's `CapabilityGrant` once G2 closes.
6. Property tests for policy engine.
