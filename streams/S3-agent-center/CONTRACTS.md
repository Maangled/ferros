# S3 Agent Center — Contracts

---

## Contracts owned by S3

| Contract | Type | Location | Status |
|----------|------|----------|--------|
| `Agent` trait | Rust trait | `crates/ferros-agents/src/agent.rs` | 🟡 Pre-G3 scaffold created |
| `AgentManifest` type | Rust type + JSON | `crates/ferros-agents/src/manifest.rs` | 🟡 Pre-G3 scaffold created |
| `AgentRegistry` trait | Rust trait | `crates/ferros-agents/src/registry.rs` | 🟡 Pre-G3 scaffold created |
| IPC bus transport abstraction | Rust trait | `crates/ferros-agents/src/bus.rs` | ⬜ Not yet created |
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
- The pre-G3 `Agent` trait keeps `start()` and `stop()` host-agnostic so S3 can freeze lifecycle vocabulary before S4 publishes executor host traits.
- `InMemoryAgentRegistry` uses `BTreeMap` ordering so `list()` stays deterministic and avoids the unordered `String`-key registry shape rejected by ADR-018.

---

## Downstream consumers

| Stream | What it consumes |
|--------|-----------------|
| S4 Runtime | `Agent` trait — runtime hosts agents via this interface |
| S5 UX | JSON/RPC API — agent center web shell talks to S3 over this |
| S7 Hub | `AgentRegistry` — hub registers HA-bridge agents through S3 |
