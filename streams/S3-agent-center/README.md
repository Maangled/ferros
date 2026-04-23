# S3 — Agent Center

**Stream:** S3  
**Status:** ⬜ Blocked on G2  
**Gate:** G3

---

## Mission

Build the coordination surface that lets users register, inspect, authorize, and control agents. The agent center is the primary user-facing power interface — the moment it works with two reference agents on a real runtime, FERROS has a usable system.

---

## Scope

- `ferros-agents` crate:
  - `Agent` trait: `id()`, `capabilities()`, `start()`, `stop()`, `status()`.
  - `AgentManifest` format: name, version, required capabilities (referencing `ProfileId`).
  - Registry: register, deregister, list, describe agents by name.
  - Spawn/stop lifecycle with deny-by-default capability check.
  - Local IPC bus (Unix domain sockets / named pipes; transport abstracted).
- CLI subcommands: `ferros agent list | describe | run | stop | logs`.
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

- **S4 UX hooks:** The agent center CLI is the UX for the runtime.
- **S5 Phase B:** The local web shell talks to `ferros-agents` over JSON/RPC.
- **Contributor onboarding:** Bringing a second contributor on becomes leverage-positive once S3 is functional.

---

## Definition of done (G3, jointly with S4)

- [ ] `ferros-agents` crate builds and passes `cargo test`.
- [ ] `echo` agent: registers, receives a capability grant, spawns, echoes messages, stops.
- [ ] `timer` agent: registers, spawns, fires periodic events, stops.
- [ ] Deny-by-default verified: ungranted capability request from either agent is rejected and logged.
- [ ] `ferros agent list | describe | run | stop | logs` all functional against the reference agents.
- [ ] IPC bus transport abstraction in place (concrete: Unix domain sockets / named pipes).

---

## Likely crates / files

| Path | Role |
|------|------|
| `crates/ferros-agents/` | Agent center crate |
| `crates/ferros-agents/src/agent.rs` | `Agent` trait |
| `crates/ferros-agents/src/manifest.rs` | `AgentManifest` |
| `crates/ferros-agents/src/registry.rs` | Registry |
| `crates/ferros-agents/src/bus.rs` | IPC bus abstraction |
| `agents/echo/` | Reference agent |
| `agents/timer/` | Reference agent |

---

## Immediate next steps

1. Review accepted S6 harvest ADRs for `botgen-rust` and `workpace-rust` before hardening interfaces.
2. Define the `Agent` trait with S4 alignment on executor interface.
3. Scaffold `crates/ferros-agents/` crate (after G2).
4. Implement registry and lifecycle.
5. Implement `echo` agent.
6. Wire CLI subcommands.
7. Verify deny-by-default with a harness.
