# S3 — Agent Center

**Stream:** S3  
**Status:** 🟡 Convergence active; final closure still depends on G2  
**Gate:** G3

---

## Mission

Build the coordination surface that lets users register, inspect, authorize, and control agents. The agent center is the primary user-facing power interface — the moment it works with two reference agents on a real runtime, FERROS has a usable system.

---

## Scope

- `ferros-agents` crate:
  - `Agent` trait: `id()`, `capabilities()`, `start()`, `stop()`, `status()`, message handling, periodic polling.
  - `AgentManifest` format: name, version, required capabilities (referencing `ProfileId`).
  - Registry: register, deregister, list, describe agents by name.
  - Spawn/stop lifecycle with deny-by-default authorization against the current manifest/runtime inputs.
  - Local IPC transport boundary (`BusTransport`, `BusListener`, `BusChannel`, `BusEndpoint`); concrete socket/pipe adapters remain open.
- Thin local CLI surface in `crates/ferros-node`: `ferros agent list | describe | run | stop | logs`.
- Two reference agents: `echo` and `timer`.
- Borrow patterns from `botgen-rust` and `workpace-rust` only through accepted S6 harvest ADRs. S3 should not mine the legacy repos directly during implementation.

---

## Out of scope

- Runtime executor or consent bus implementation (S4).
- Web shell for the agent center (S5 Phase B).
- Home Assistant agent integration (S7).

---

## Dependencies

- **S2 (G2 must be green):** `ProfileId` and `CapabilityGrant` types must be frozen. Agents reference a profile and declare required grants.
- **S4 (runtime traits):** The executor interface for agents running inside `ferros-runtime` is owned by S4. S3 depends on the trait stubs being available.

---

## What this stream blocks

- **S4 UX hooks:** The landed local CLI is the current runtime UX while reusable host/API surfaces remain open.
- **S5 Phase B:** The local web shell talks to `ferros-agents` over JSON/RPC.
- **Contributor onboarding:** Bringing a second contributor on becomes leverage-positive once S3 is functional.

---

## Definition of done (G3, jointly with S4)

- [ ] `ferros-agents` crate builds and passes `cargo test`.
- [x] `echo` agent: registers, receives a capability grant, spawns, echoes messages, stops.
- [x] `timer` agent: registers, spawns, fires periodic events, stops.
- [x] Current deny-by-default evidence exists in manifest authorization coverage plus demo/runtime denial-log assertions.
- [x] Thin local `ferros agent list | describe | run | stop | logs` wrapper is functional against the reference agents.
- [x] IPC bus transport abstraction is in place; concrete socket/pipe adapters remain open.
- [ ] Broader deny-by-default lifecycle/log harness coverage for final G3 closure.

---

## Likely crates / files

| Path | Role |
|------|------|
| `crates/ferros-agents/` | Agent center crate |
| `crates/ferros-agents/src/agent.rs` | `Agent` trait |
| `crates/ferros-agents/src/manifest.rs` | `AgentManifest` |
| `crates/ferros-agents/src/registry.rs` | Registry |
| `crates/ferros-agents/src/bus.rs` | IPC bus abstraction |
| `crates/ferros-agents/src/reference.rs` | `echo` + `timer` reference agents |
| `crates/ferros-node/` | Convergence demo host + local `ferros` CLI wrapper |

---

## Immediate next steps

1. Harden the `ferros-node demo` path into a reusable runtime-host integration surface.
2. Expand deny-by-default evidence from the current manifest/runtime assertions into a dedicated lifecycle/log harness.
3. Keep the current CLI local-only until the post-G3 JSON/RPC boundary for S5 is designed and accepted.
4. Freeze the post-G2 S3 contracts once the S2 and S4 dependency surfaces settle.
