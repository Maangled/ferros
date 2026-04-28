# S3 Agent Center — Contracts

---

## Contracts owned by S3

| Contract | Type | Location | Status |
|----------|------|----------|--------|
| `Agent` trait | Rust trait | `crates/ferros-agents/src/agent.rs` | 🟡 Pre-G3 runtime-facing scaffold created |
| `AgentManifest` type | Rust type + JSON | `crates/ferros-agents/src/manifest.rs` | 🟡 Pre-G3 scaffold created |
| `AgentRegistry` trait | Rust trait | `crates/ferros-agents/src/registry.rs` | 🟡 Pre-G3 scaffold created |
| IPC bus transport abstraction | Rust traits + endpoint types | `crates/ferros-agents/src/bus.rs` | 🟡 Transport-only scaffold created; adapters remain open |
| `EchoAgent` + `TimerAgent` | Rust types | `crates/ferros-agents/src/reference.rs` | 🟡 Convergence slice created |
| `LocalAgentApi` + local `ferros agent` CLI | Rust API + Rust bin + temp-file state surface | `crates/ferros-node/src/lib.rs`, `crates/ferros-node/src/bin/ferros.rs` | 🟡 First broader local-only wrapper/API slice landed as `LocalAgentApi`; broader remote transport and privileged writes remain open |
| Read-first JSON/RPC API (for S5 web shell) | JSON/RPC contract types + local host handler | `crates/ferros-agents/src/rpc.rs`, `crates/ferros-node/src/lib.rs` | 🟡 Local shell-host read surface landed, including aggregated `agent.snapshot`; broader remote transport and privileged writes remain open |

---

## Contracts consumed by S3

| Contract | Source | Purpose |
|----------|--------|---------|
| `ProfileId` | S2 | Agents are authorized by profile |
| `CapabilityGrant` | S2 | Grants required per agent manifest |
| Executor interface / runtime traits | S4 | Agents run inside the `ferros-runtime` executor |

## Current boundary notes

- `AgentManifest` now stores `CapabilityRequirement` entries, not `CapabilityGrant`. The manifest declares what an agent needs; S2 grants remain runtime authorization inputs.
- The pre-G3 `Agent` trait now keeps lifecycle host-agnostic while also exposing message-handling and polling hooks that the `ferros-node demo` host can drive without freezing a higher-level RPC contract.
- The current `ferros-node demo` path now hardens that host seam into reusable in-memory host methods on `DemoRuntime`: `reference_host()` bootstraps the reference grants, registry, and agents, `run_reference_demo_cycle()` exercises the deterministic host path, and persisted CLI-state replay now reuses the same host bootstrap. This stays below any published lifecycle/write wrapper or broader S4 host guarantee.
- `InMemoryAgentRegistry` uses `BTreeMap` ordering so `list()` stays deterministic and avoids the unordered `String`-key registry shape rejected by ADR-018.
- `BusTransport` is transport-scoped only: it binds and connects `BusEndpoint` values, while channels exchange opaque `Vec<u8>` payloads so S4 hosts can layer sockets, named pipes, or future local transports without freezing a wire format yet.
- The current local operator surface is intentionally local-only: `LocalAgentApi` in `crates/ferros-node/src/lib.rs` now publishes the first broader typed `list | describe | run | stop | logs` wrapper/API slice above CLI formatting while still routing through the internal `LocalAgentController`, rebuilding the reference runtime per invocation, and persisting minimal status/log state in the temp directory. It should not be treated as the post-G3 JSON/RPC or remote-control contract.
- The landed `LocalAgentApi` seam now also preserves richer deny detail on local denied lifecycle attempts: the denied `run` path carries typed missing-capability detail in the local error path while the CLI and persisted deny-log summary text stay stable on the same local state path.
- The first post-G3 JSON/RPC surface is intentionally read-first: `agent.list`, `agent.describe`, `agent.snapshot`, `grant.list`, and `denyLog.list` now exist as typed JSON/RPC request and response shapes in `crates/ferros-agents/src/rpc.rs`, with a local host handler in `crates/ferros-node/src/lib.rs` backed by the current runtime, persisted grant state, and deny-log state.
- Focused `ferros-node` tests now lock the current read-first error-envelope behavior for unsupported JSON-RPC version, missing `agentName` on `agent.describe`, unknown method names, and unknown agents, including a live `POST /rpc` socket smoke through the localhost shell host.
- The read-first JSON/RPC surface is currently served only through the local shell host and does not yet publish a broader remote transport contract, health endpoints, subscriptions, or privileged write actions. Grant creation/revoke flows remain shell intents plus future post-read contract work rather than part of this read-first contract, and S4 restart/reload semantics remain unpublished/open on this boundary.
- Current deny-by-default evidence now spans manifest authorization results plus a dedicated local lifecycle/log harness around the `ferros` agent state path: denied lifecycle attempts persist `denied-start:*` entries into CLI state, `ferros agent logs` exposes them, and `denyLog.list` can observe them through the read-first local shell host. Broader remote or write-side policy/log harness surfaces remain open.
- The first local-only lifecycle/write seam is now implementation-backed through the current `ferros` CLI/state path, and the first broader local-only wrapper/API slice is now landed above it: `LocalAgentApi` returns typed local agent list, detail, lifecycle, and log results above CLI formatting, while `ferros agent run` / `stop` still mutate only the persisted local state path and `agent.describe` / `agent.snapshot` still provide stable read-after-write observation on that same path. This still does not publish any remote-control contract.
- `EchoAgent` and `TimerAgent` are intentionally in-crate convergence fixtures for G3 prep; they can be split into dedicated crates later if that improves packaging or release boundaries.

---

## First broader lifecycle/write wrapper/API slice

The minimum local-only slice below is landed, and the first broader local-only wrapper/API slice above it is now also landed as `LocalAgentApi` in `crates/ferros-node/src/lib.rs`. The table below defines what is now published while still keeping richer remote observation/control, privileged UX, and S4 restart/reload semantics unpublished/open at this boundary.

| Required element | Landed surface | Landed semantics | Still not implied |
|------------------|----------------|------------------|-------------------|
| Local host/write seam | `LocalAgentApi` over `LocalAgentController`, `DemoRuntime::reference_host()`, and `run_reference_demo_cycle()` in `crates/ferros-node` | The first wrapper/API slice stays local-only, sits above CLI formatting, and reuses the current in-memory host path instead of defining a new remote host surface | No remote transport, broader host/lifecycle contract, or S4 restart/reload semantics are published |
| Local CLI/state path | current `ferros agent` CLI behavior plus persisted local state path | Landed locally: every write attempt still goes through the current local path and remains deny-by-default on each lifecycle/write attempt, while `LocalAgentApi` now exposes typed local list/detail/lifecycle/log results and typed local deny detail above that path | No privileged UX, grant-write, or bridge-control contract is published |
| Read-after-write observation | local CLI inspection plus read-first JSON/RPC methods (`agent.list`, `agent.describe`, `agent.snapshot`, `grant.list`, `denyLog.list`) | Landed locally: allowed and denied write attempts remain observable through stable read-after-write checks on the current local/read-first surfaces after `LocalAgentApi` calls or CLI calls on the same path | No richer remote observation/control, subscriptions, or streaming contract is published |
| Evidence bar | focused `local_agent_api_`, `agent_cli_`, `agent_read_rpc_`, and `shell_listener_posts_json_rpc_` coverage | Landed locally: focused coverage proves the deny-by-default write path and the resulting read-after-write observation on the current local-only seam and the first broader local wrapper/API slice | No G4 evidence or broader remote-control validation surface is implied |

---

## Read-first JSON/RPC methods

| Method | Result kind | Backing surface | Notes |
|--------|-------------|-----------------|-------|
| `agent.list` | `agentList` | `DemoRuntime::agent_records()` | Returns deterministic agent name, version, and current status rows |
| `agent.describe` | `agentDetail` | `DemoRuntime::describe_agent()` | Requires `agentName`; returns required capabilities alongside status |
| `agent.snapshot` | `agentSnapshot` | `DemoRuntime::agent_records()` + `LocalProfileStore::load_local_profile()` + persisted CLI state | Returns detailed agent records, profile-selected grant state, and deny-log observation in one read-only response; optional `agentName` filters the agent detail and deny-log portions while unknown agents keep the current not-found envelope |
| `grant.list` | `grantList` | `LocalProfileStore::load_local_profile()` | Reads signed grant state from the default or requested local profile path; missing profile state degrades to an empty list |
| `denyLog.list` | `denyLog` | persisted CLI state log entries | Returns only deny-related entries, with optional `agentName` filtering |

### Contract shape notes

- Requests use JSON-RPC `2.0` envelope fields: `jsonrpc`, `id`, `method`, and optional `params`.
- The current `params` surface is intentionally narrow: `agentName` for `agent.describe`, `agent.snapshot`, and `denyLog.list`, plus optional `profilePath` for `grant.list` and `agent.snapshot`.
- Responses use JSON-RPC `2.0` `result` and `error` envelopes. Unknown methods and invalid params return JSON-RPC error codes; unknown agents return an S3-owned custom error code for now.
- This contract is the stable read boundary for S5 shell work. It does not freeze the transport adapter yet.

---

## First local-only lifecycle/write JSON/RPC slice

The table below defines the local-only lifecycle/write JSON/RPC surface that is now landed on the current localhost shell host without widening into browser control, remote transport, grant writes, or broader restart/reload claims.

| Landed element | Landed surface | Landed semantics | Still not implied |
|----------------|----------------|------------------|-------------------|
| Host/transport scope | current localhost shell host only | The first write-side JSON/RPC slice stays local-only on the same host path currently used by the shell and the read methods | No remote transport, auth model, or browser-control claim is published |
| Write methods | `agent.run` and `agent.stop` routed through `LocalAgentApi` on the same local state path | The landed write methods reuse the current code-backed local lifecycle seam instead of inventing a second write path or a broader privileged surface | No grant writes, bridge-control choreography, or broader privileged UX contract is published |
| Read-after-write observation | current read-first `agent.describe`, `agent.snapshot`, and `denyLog.list` methods | Allowed and denied writes remain observable through the already-landed read path and stable deny-log summaries | No richer remote observation/control, subscriptions, or streaming contract is published |
| Deny behavior | current local deny-detail path plus stable CLI / deny-log summaries on denied writes, with a local-only authorization error envelope on the JSON/RPC path | The write slice stays aligned to the landed local-only deny behavior and does not claim a broader remote-control error model | No broader shared write-side error contract or remote-control guarantee is published |
| Evidence bar | focused local JSON/RPC lifecycle coverage plus localhost shell-host coverage for allowed and denied writes on the same path | The landed write-side RPC surface is still local-only, deny-by-default, and observable through the landed read path | No broader remote-control validation surface or G4 evidence is implied |

---

## S5 Phase B lifecycle control bar — S3 consumer-awareness

S5 has defined the minimum consent-gated browser-issued lifecycle control bar above the staged shell-intent copy. When that bar lands as code, S5 will consume `agent.run` and `agent.stop` from the current landed local-only lifecycle/write JSON/RPC slice. No new S3 RPC methods are implied by S5's stated bar definition. The read-after-write observation path (`agent.describe`, `agent.snapshot`, `denyLog.list`) is unchanged. Grant/revoke actions and broader browser control are out of scope for this bar.

---

## Downstream consumers

| Stream | What it consumes |
|--------|-----------------|
| S4 Runtime | `Agent` trait — runtime hosts agents via this interface |
| S5 UX | Read-first JSON/RPC API — current agent center web shell boundary for list, describe, aggregated snapshot, grant-state, and deny-log views |
| S7 Hub | `AgentRegistry` plus local/read-first inspection surfaces — runway planning only for one bridge agent at the shared registry boundary |

## First S7 hub-facing wrapper boundary

This is the first S3-owned hub-facing wrapper-boundary note for S7 runway planning. It publishes only what is currently sufficient on `AgentRegistry` plus local/read-first inspection surfaces and does not turn those surfaces into a final hub contract.

### Currently sufficient now

| Surface | Current published truth | What S7 may treat as sufficient now |
|---------|-------------------------|-------------------------------------|
| `AgentRegistry::register` and `AgentRegistry::deregister` | Shared registry entry/exit seam only | Enough to plan one bridge agent entering and leaving the shared registry boundary |
| `AgentRegistry::list` and `AgentRegistry::describe` | Shared registry inspection seam only | Enough to require that the first bridge slice be listable and describable, including required-capability inspection |
| local `ferros agent list`, `ferros agent describe`, and `ferros agent logs` | Thin local, read-first operator wrappers only | Enough for on-device bridge presence/detail plus deny/lifecycle observation |
| read-first `agent.list`, `agent.describe`, `agent.snapshot`, `grant.list`, and `denyLog.list` | Typed read-only inspection shapes on the current local host/shell path only | Enough for runway evidence planning and shell-side inspection without freezing remote control |

## First shell-intent consumer boundary

The table below defines the next honest S5 consumer publication above the landed local-only lifecycle/write JSON-RPC slice without turning that backend capability into real browser control.

| Consumer element | Minimum next S5 surface | Why this is the honest next bar | Still not implied |
|------------------|-------------------------|---------------------------------|-------------------|
| Shell scope | current localhost shell only | Keeps the staged shell intent tied to the same localhost host and local state path already used by the landed backend slice | No remote transport, auth model, or broader browser-control claim is published |
| Lifecycle intent | selected-agent copy and read-only slot affordances for `agent.run` / `agent.stop` | Lets S5 show where local lifecycle intent belongs without sending write RPC from the browser or inventing a second control path | No browser-issued write action, consent-resolution flow, or grant/revoke contract is published |
| Read-after-intent observation | manual refresh plus current `agent.snapshot`, `agent.describe`, and `denyLog.list` observation | Keeps S5 on the landed read-after-write path instead of inventing subscriptions, push state, or a second observation seam | No streaming, subscription, or richer remote observation/control contract is published |
| Consent and audit copy | existing shell consent/audit slot as visible ownership only | Preserves the future home for privileged actions without claiming that the shell can yet submit or resolve them | No privileged UX, grant-write, or bridge-control choreography contract is published |

### Still unpublished before bridge control flows are honest

| Surface area | Current status |
|--------------|----------------|
| Hub-facing lifecycle wrapper | Unpublished. No S3 contract yet defines a hub-owned wrapper for long-running bridge lifecycle, wrapper-owned status transitions, or an authoritative bridge control surface. |
| Richer remote observation wrapper | Unpublished. No S3 contract yet defines remote inspection transport, health/subscription/log-streaming, or HA-facing observation wrappers beyond the current local/read-first surfaces. |
| Remote control or write contract | Unpublished. No S3 contract yet defines privileged lifecycle actions, grant writes, or bridge-control sequencing on top of the current registry and read-first inspection surfaces. |

- This slice keeps the dependency lock narrow: S7 may plan against `AgentRegistry` plus local/read-first inspection surfaces only.
- This slice does not publish pairing choreography, bridge-control choreography, schema changes, or any G4 evidence surface.
