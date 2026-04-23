# S3 Agent Center — Contracts

---

## Contracts owned by S3

| Contract | Type | Location | Status |
|----------|------|----------|--------|
| `Agent` trait | Rust trait | `crates/ferros-agents/src/agent.rs` | 🟡 Pre-G3 runtime-facing scaffold created |
| `AgentManifest` type | Rust type + JSON | `crates/ferros-agents/src/manifest.rs` | 🟡 Pre-G3 scaffold created |
| `AgentRegistry` trait | Rust trait | `crates/ferros-agents/src/registry.rs` | 🟡 Pre-G3 scaffold created |
| IPC bus transport abstraction | Rust traits + endpoint types | `crates/ferros-agents/src/bus.rs` | 🟡 Pre-G3 scaffold created |
| `EchoAgent` + `TimerAgent` | Rust types | `crates/ferros-agents/src/reference.rs` | 🟡 Convergence slice created |
| JSON/RPC API (for S5 web shell) | HTTP+JSON spec | TBD | ⬜ Post-G3 |

---

## Contracts consumed by S3

| Contract | Source | Purpose |
|----------|--------|---------|
| `ProfileId` | S2 | Agents are authorized by profile |
| `CapabilityGrant` | S2 | Grants required per agent manifest |
| Executor interface / runtime traits | S4 | Agents run inside the `ferros-runtime` executor |

## Pre-G3 boundary notes

- `AgentManifest` now stores `CapabilityRequirement` entries, not `CapabilityGrant`. The manifest declares what an agent needs; S2 grants remain runtime authorization inputs.
- The pre-G3 `Agent` trait now keeps lifecycle host-agnostic while also exposing message-handling and polling hooks that the `ferros-node demo` host can drive without freezing a higher-level RPC contract.
- `InMemoryAgentRegistry` uses `BTreeMap` ordering so `list()` stays deterministic and avoids the unordered `String`-key registry shape rejected by ADR-018.
- `BusTransport` is transport-scoped only: it binds and connects `BusEndpoint` values, while channels exchange opaque `Vec<u8>` payloads so S4 hosts can layer sockets, named pipes, or future local transports without freezing a wire format yet.
- `EchoAgent` and `TimerAgent` are intentionally in-crate convergence fixtures for G3 prep; they can be split into dedicated crates later if that improves packaging or release boundaries.

---

## Downstream consumers

| Stream | What it consumes |
|--------|-----------------|
| S4 Runtime | `Agent` trait — runtime hosts agents via this interface |
| S5 UX | JSON/RPC API — agent center web shell talks to S3 over this |
| S7 Hub | `AgentRegistry` — hub registers HA-bridge agents through S3 |
