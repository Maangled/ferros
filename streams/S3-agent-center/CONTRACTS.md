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
| Local `ferros agent` CLI | Rust bin + temp-file state surface | `crates/ferros-node/src/bin/ferros.rs`, `crates/ferros-node/src/lib.rs` | 🟡 Thin local wrapper landed; reusable API/RPC remains open |
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
- The current `ferros agent` surface is intentionally local-only: it rebuilds the reference runtime per invocation and persists minimal status/log state in the temp directory, so it should not be treated as the post-G3 JSON/RPC contract.
- The first post-G3 JSON/RPC surface is intentionally read-first: `agent.list`, `agent.describe`, `agent.snapshot`, `grant.list`, and `denyLog.list` now exist as typed JSON/RPC request and response shapes in `crates/ferros-agents/src/rpc.rs`, with a local host handler in `crates/ferros-node/src/lib.rs` backed by the current runtime, persisted grant state, and deny-log state.
- Focused `ferros-node` tests now lock the current read-first error-envelope behavior for unsupported JSON-RPC version, missing `agentName` on `agent.describe`, unknown method names, and unknown agents, including a live `POST /rpc` socket smoke through the localhost shell host.
- The read-first JSON/RPC surface is currently served only through the local shell host and does not yet publish a broader remote transport contract, health endpoints, subscriptions, or privileged write actions. Grant creation/revoke flows remain shell intents plus future post-read contract work rather than part of this read-first contract, and S4 restart/reload semantics remain unpublished/open on this boundary.
- Current deny-by-default evidence now spans manifest authorization results plus a dedicated local lifecycle/log harness around the `ferros` agent state path: denied lifecycle attempts persist `denied-start:*` entries into CLI state, `ferros agent logs` exposes them, and `denyLog.list` can observe them through the read-first local shell host. Broader remote or write-side policy/log harness surfaces remain open.
- The first local-only lifecycle/write seam is now implementation-backed through the current `ferros` CLI/state path: `ferros agent run` and `ferros agent stop` mutate only the persisted local state path, while `agent.describe` and `agent.snapshot` provide stable read-after-write observation of those local lifecycle transitions on that same path. This still does not publish a broader lifecycle/write wrapper/API or any remote-control contract.
- `EchoAgent` and `TimerAgent` are intentionally in-crate convergence fixtures for G3 prep; they can be split into dedicated crates later if that improves packaging or release boundaries.

---

## Minimum first lifecycle/write entry bar

The minimum local-only slice below is now landed. It remains below any broader lifecycle/write wrapper, richer remote observation/control surface, or privileged UX claim; S4 restart/reload semantics also remain unpublished/open at this boundary.

| Required element | Current seam to reuse | Minimum semantics | Still not implied |
|------------------|-----------------------|-------------------|-------------------|
| Local host/write seam | `DemoRuntime::reference_host()` and `run_reference_demo_cycle()` in `crates/ferros-node` | The first wrapper/API slice stays local-only and reuses the current in-memory host path instead of defining a new remote host surface | No remote transport, broader host/lifecycle contract, or S4 restart/reload semantics are published |
| Local CLI/state path | current `ferros agent` CLI behavior plus persisted local state path | Landed locally: every write attempt goes through the current local path and remains deny-by-default on each lifecycle/write attempt | No privileged UX, grant-write, or bridge-control contract is published |
| Read-after-write observation | local CLI inspection plus read-first JSON/RPC methods (`agent.list`, `agent.describe`, `agent.snapshot`, `grant.list`, `denyLog.list`) | Landed locally: allowed and denied write attempts are observable through stable read-after-write checks on the current local/read-first surfaces | No richer remote observation/control, subscriptions, or streaming contract is published |
| Evidence bar | dedicated deny-by-default lifecycle/log harness | Landed locally: focused harness coverage proves the deny-by-default write path and the resulting read-after-write observation on the current local-only seam | No G4 evidence or broader remote-control validation surface is implied |

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

### Still unpublished before bridge control flows are honest

| Surface area | Current status |
|--------------|----------------|
| Hub-facing lifecycle wrapper | Unpublished. No S3 contract yet defines a hub-owned wrapper for long-running bridge lifecycle, wrapper-owned status transitions, or an authoritative bridge control surface. |
| Richer remote observation wrapper | Unpublished. No S3 contract yet defines remote inspection transport, health/subscription/log-streaming, or HA-facing observation wrappers beyond the current local/read-first surfaces. |
| Remote control or write contract | Unpublished. No S3 contract yet defines privileged lifecycle actions, grant writes, or bridge-control sequencing on top of the current registry and read-first inspection surfaces. |

- This slice keeps the dependency lock narrow: S7 may plan against `AgentRegistry` plus local/read-first inspection surfaces only.
- This slice does not publish pairing choreography, bridge-control choreography, schema changes, or any G4 evidence surface.
