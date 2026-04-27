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

- **S4 UX hooks:** The landed local CLI plus reusable in-memory demo host are the current runtime UX while broader lifecycle/write host surfaces remain open.
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
- [x] Broader deny-by-default lifecycle/log harness coverage for final G3 closure.

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

## First broader lifecycle/write wrapper/API slice

The first broader slice above the current local-only lifecycle/write seam is now landed as `LocalAgentApi` in `crates/ferros-node/src/lib.rs`. It stays local-only and keeps the surface narrow:

- `LocalAgentApi` reuses the current code-backed local seams in `ferros-node`: `DemoRuntime::reference_host()`, `run_reference_demo_cycle()`, the persisted local state path, and the internal `LocalAgentController`, instead of inventing a second write path or a remote host surface.
- The published inspection boundary stays read-first: local CLI inspection plus the current JSON/RPC read methods (`agent.list`, `agent.describe`, `agent.snapshot`, `grant.list`, `denyLog.list`) continue to provide stable local read-after-write observation of the landed wrapper/API slice; S4 restart/reload semantics remain unpublished/open at this boundary.
- Every write attempt on `LocalAgentApi` remains deny-by-default and is backed by focused local wrapper, CLI, lifecycle/log, JSON/RPC, and shell-host test coverage on the same local path.
- This still does not publish remote transport, richer remote observation/control, privileged UX claims, grant writes, bridge-control choreography, or S4 restart/reload semantics.

---

## Immediate next steps

1. Keep `LocalAgentApi`, the current local CLI, and the current read-first inspection surfaces aligned on one local state path until a richer code-backed follow-up actually lands; S4 restart/reload semantics remain unpublished/open there.
2. Make the next implementation wave only the narrowest follow-up above the landed `LocalAgentApi` slice, without publishing remote transport, richer remote observation/control, privileged UX, grant writes, bridge-control choreography, or S4 restart/reload semantics early.
3. Freeze the post-G2 S3 contracts only after that next surface exists and the S2 and S4 dependency surfaces settle.
