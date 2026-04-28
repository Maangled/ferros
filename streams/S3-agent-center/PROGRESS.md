# S3 Agent Center — Progress

Reverse-chronological. Append a dated entry at the top per session.

---

## 2026-04-28 - S5 lifecycle bar consumed the existing local RPC seam

- Truth-synced S3 owner docs after S5 landed the browser lifecycle control bar on top of the existing local-only `agent.run` / `agent.stop` methods.
- Kept S3 unchanged: no new JSON/RPC methods, no grant-write contract, no remote transport, no richer observation path, and no broader browser-control publication beyond S5's local selected-agent bar.
- Focused validation for the consumed seam remained green with `cargo test -p ferros-node agent_write_rpc_` and `cargo test -p ferros-node shell_listener_posts_json_rpc_`.

## 2026-04-26 — Minimum local-only lifecycle/write JSON/RPC slice landed on the localhost shell host

- Extended `crates/ferros-agents/src/rpc.rs` and `crates/ferros-node/src/lib.rs` so the current localhost shell host now accepts local-only `agent.run` and `agent.stop` JSON/RPC methods above the landed `LocalAgentApi` seam.
- Kept the slice narrow: the write methods still route through `LocalAgentApi` on the same persisted local state path, `agent.describe`, `agent.snapshot`, and `denyLog.list` remain the read-after-write observation path, and browser control plus broader remote/write claims remain unpublished.
- Focused validation passed with `cargo test -p ferros-node agent_write_rpc_`, `cargo test -p ferros-node shell_listener_posts_json_rpc_`, `cargo test -p ferros-node agent_read_rpc_`, and `cargo test -p ferros-agents`.

## 2026-04-26 — Minimum first local write JSON/RPC entry bar defined above `LocalAgentApi`

- Clarified the S3 owner docs so the next honest write-side publication above the landed local-only `LocalAgentApi` seam is only a local-only JSON/RPC lifecycle slice on the current localhost shell host, not browser control or a broader remote-control contract.
- Defined that minimum next bar narrowly: only `agent.run` and `agent.stop` may move first, they must still route through `LocalAgentApi` on the same persisted local state path, and the current read-first `agent.describe`, `agent.snapshot`, and `denyLog.list` methods remain the read-after-write observation path.
- Kept browser control, grant writes, bridge-control choreography, richer remote observation/control, and broader S4 restart/reload semantics unpublished until a later code-backed follow-up exists.

## 2026-04-26 — Local deny-reason introspection landed on the `LocalAgentApi` seam

- Extended `crates/ferros-node/src/lib.rs` so the local-only `LocalAgentApi` surface now preserves typed missing-capability detail on denied `run` attempts instead of flattening the local deny path to summary text only.
- Kept the slice local-only and narrow: the CLI still renders the same denial summary, the persisted deny-log text stays stable, and the read-first JSON/RPC plus localhost shell observation surfaces remain unchanged.
- Focused validation passed with `cargo test -p ferros-node local_agent_api_` and `cargo test -p ferros-node agent_cli_`.

## 2026-04-26 — First broader local-only wrapper/API slice landed above the CLI formatter

- Added `LocalAgentApi` in `crates/ferros-node/src/lib.rs` so the first broader lifecycle/write wrapper/API slice above the current local-only seam now exists as a typed local `list | describe | run | stop | logs` surface instead of only CLI-formatted lines.
- Kept the landed slice local-only and narrow: `LocalAgentApi` still reuses the current local state path, the internal `LocalAgentController`, the current read-first JSON/RPC observation surfaces, and the deny-by-default lifecycle/log evidence rather than publishing remote transport, richer remote observation/control, privileged UX, grant writes, bridge-control choreography, or S4 restart/reload semantics.
- Focused validation passed with `cargo test -p ferros-node local_agent_api_`, `cargo test -p ferros-node agent_cli_`, `cargo test -p ferros-node agent_read_rpc_`, and `cargo test -p ferros-node shell_listener_posts_json_rpc_`.

## 2026-04-26 — Internal local host-controller surface extracted above the CLI parser

- Refactored `crates/ferros-node/src/lib.rs` so the current local `ferros agent` lifecycle/log path now routes through an internal `LocalAgentController` above argv parsing instead of keeping that controller logic flattened in the CLI execution function.
- Kept the extraction below any published broader lifecycle/write wrapper/API or remote-control contract: the local state-path seam, read-first JSON/RPC observation surfaces, and deny-by-default evidence stay the same.
- Focused validation passed with `cargo test -p ferros-node agent_cli_`, `cargo test -p ferros-node agent_read_rpc_`, and `cargo test -p ferros-node shell_listener_posts_json_rpc_`.

## 2026-04-26 — Broader wrapper/API entry bar fixed above the local-only seam

- Clarified the S3 owner docs so the landed local-only `ferros agent run` / `ferros agent stop` seam is explicit fixed input rather than the publishable broader wrapper/API slice.
- Defined the minimum first broader lifecycle/write wrapper/API entry bar above that seam: reuse the current local host and CLI/state path, preserve read-first observation through `agent.list`, `agent.describe`, `agent.snapshot`, `grant.list`, and `denyLog.list`, and keep deny-by-default lifecycle/log evidence on the same local path.
- Kept remote transport, richer remote observation/control, privileged UX, grant writes, bridge-control choreography, and S4 restart/reload semantics unpublished.

## 2026-04-25 — First local-only lifecycle/write seam landed through the CLI/state path

- Added focused `crates/ferros-node/src/lib.rs` coverage that drives `ferros agent run` and `ferros agent stop` through the existing local state path, then proves `agent.describe` and `agent.snapshot` observe the resulting running and stopped state on the same path.
- Kept the seam local-only and below any broader wrapper/API claim: the landed slice reuses the current CLI/state path, current read-first inspection surfaces, and the dedicated deny-by-default lifecycle/log harness instead of publishing remote transport, richer remote observation/control, privileged UX, grant writes, bridge-control choreography, or S4 restart/reload semantics.
- Validation passed with focused `cargo test -p ferros-node agent_read_rpc_observes_cli_lifecycle_state_after_local_run_and_stop`, `cargo test -p ferros-node agent_cli_`, and `cargo test -p ferros-node agent_read_rpc_`.

## 2026-04-25 — Denied lifecycle/log harness landed on the local state path

- Added focused `crates/ferros-node/src/lib.rs` harness coverage that drives a denied `ferros agent run echo` attempt through the local state path, proves the agent stays registered, persists the `denied-start:echo missing agent.echo` evidence, and exposes that evidence through both `ferros agent logs` and `denyLog.list`.
- Adjusted the local CLI state-path execution so denied lifecycle attempts persist their runtime log evidence before returning the authorization error.
- Kept the slice local-only and evidence-backed: no lifecycle/write wrapper APIs, no richer remote observation/control transport, and no S5 privileged-flow work.
- Validation passed with focused `cargo test -p ferros-node agent_cli_denied_run_` and `cargo test -p ferros-node agent_read_rpc_exposes_denied_lifecycle_`.

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
