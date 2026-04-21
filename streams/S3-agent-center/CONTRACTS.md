# S3 Agent Center — Contracts

---

## Contracts owned by S3

| Contract | Type | Location | Status |
|----------|------|----------|--------|
| `Agent` trait | Rust trait | `crates/ferros-agents/src/agent.rs` | ⬜ Not yet created |
| `AgentManifest` type | Rust type + JSON | `crates/ferros-agents/src/manifest.rs` | ⬜ Not yet created |
| `AgentRegistry` trait | Rust trait | `crates/ferros-agents/src/registry.rs` | ⬜ Not yet created |
| IPC bus transport abstraction | Rust trait | `crates/ferros-agents/src/bus.rs` | ⬜ Not yet created |
| JSON/RPC API (for S5 web shell) | HTTP+JSON spec | TBD | ⬜ Post-G3 |

---

## Contracts consumed by S3

| Contract | Source | Purpose |
|----------|--------|---------|
| `ProfileId` | S2 | Agents are authorized by profile |
| `CapabilityGrant` | S2 | Grants required per agent manifest |
| Executor interface / runtime traits | S4 | Agents run inside the `ferros-runtime` executor |

---

## Downstream consumers

| Stream | What it consumes |
|--------|-----------------|
| S4 Runtime | `Agent` trait — runtime hosts agents via this interface |
| S5 UX | JSON/RPC API — agent center web shell talks to S3 over this |
| S7 Hub | `AgentRegistry` — hub registers HA-bridge agents through S3 |
