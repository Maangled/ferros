# S3 Agent Center — Progress

Reverse-chronological. Append a dated entry at the top per session.

---

## 2026-04-25 — Demo host hardened into a reusable in-memory host layer

- Added reusable `DemoRuntime::reference_host()` and `DemoRuntime::run_reference_demo_cycle()` methods so the current reference-agent host path is no longer only a one-off top-level demo script.
- Reused that same host bootstrap for persisted CLI-state replay, keeping the in-memory demo, shell/RPC host, and local CLI reconstruction paths aligned on one reference host surface.
- Kept the slice below any published lifecycle/write wrapper or broader S4 runtime-host guarantee.
- Validation passed with `cargo test -p ferros-node demo_`.

## 2026-04-24 — Read-first JSON-RPC error paths locked with focused coverage

- Added focused `crates/ferros-node/src/lib.rs` tests for unsupported JSON-RPC version, missing `agentName` on `agent.describe`, unknown agent lookup, and unknown method names against the existing read-first handler.
- Added a listener-level `POST /rpc` smoke that proves the live localhost shell host returns the same structured JSON-RPC invalid-params envelope over TCP, not just through direct handler calls.
- Kept the contract unchanged: no new methods, no write actions, no transport expansion, and no privileged flows.
- Validation passed with `cargo test -p ferros-node agent_read_rpc_` and `cargo test -p ferros-node shell_listener_posts_json_rpc_`.

## 2026-04-23 — Thin local `ferros agent` CLI landed

- Added `crates/ferros-node/src/bin/ferros.rs` and the matching `AgentCliCommand` execution path for `agent list | describe | run | stop | logs`.
- Kept the surface intentionally local: each invocation rebuilds the in-process reference runtime and persists only minimal status/log state under the temp directory instead of freezing JSON/RPC early.
- Added focused CLI tests covering list plus run/describe/stop/logs lifecycle behavior against the `echo` and `timer` reference agents.
- Kept the remaining G3 gap focused on reusable host/API hardening, broader log/harness expansion, and post-G2 contract freeze.

## 2026-04-23 — Reference agents converged on `ferros-node demo`

- Added in-crate `EchoAgent` and `TimerAgent` reference agents under `crates/ferros-agents/src/reference.rs`.
- Extended the pre-G3 `Agent` trait with message-handling and polling hooks so the reference agents can exercise a real host path without freezing a higher-level wire protocol.
- Verified a deterministic `ferros-node demo` path that registers both agents, echoes a payload, emits a timer tick, and logs one deny-by-default rejection using the current real `CapabilityGrant` type.
- Kept the remaining G3 gap focused on reusable host/API hardening, log and harness expansion, and post-G2 contract hardening.

## 2026-04-23 — Bus transport boundary scaffolded

- Added `crates/ferros-agents/src/bus.rs` with host-agnostic `BusTransport`, `BusListener`, and `BusChannel` traits plus `BusEndpoint` and `BusTransportKind` value types for local IPC addressing.
- Kept the boundary transport-focused by using opaque byte payloads, leaving concrete Unix domain socket and named pipe implementations for later host work.
- Hardened manifest and registry coverage with profile-scoped deny-by-default authorization and deregister lifecycle tests.
- Validated the crate with focused `cargo test` against `crates/ferros-agents/Cargo.toml`.

## 2026-04-23 — Pre-G3 contract scaffold landed

- Added a standalone `crates/ferros-agents/` crate without touching the root workspace manifest.
- Defined a pre-G3 `Agent` trait, `AgentManifest`, `CapabilityRequirement`, and `AgentRegistry` boundary against the current S2 `ProfileId` and `CapabilityGrant` types.
- Added an `InMemoryAgentRegistry` with deterministic `BTreeMap` ordering plus manifest authorization helpers that deny by default when required capabilities are missing.
- Validated the scaffold with a manifest-scoped `cargo test` run.

## 2026-04-21 — Stream scaffolded

- Stream directory and planning documents created.
- Blocked on G2 (S2 Profile & Identity).
- `Agent` trait interface can be sketched in parallel with S2/S4 work.
- S6 harvest ADRs for `botgen-rust` and `workpace-rust` should be completed before S3 implementation begins.
