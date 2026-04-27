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
- Every write attempt on `LocalAgentApi` remains deny-by-default and is backed by focused local wrapper, CLI, lifecycle/log, JSON/RPC, and shell-host test coverage on the same local path; denied local `run` attempts now preserve typed missing-capability detail while keeping the CLI and deny-log summaries stable.
- This still does not publish remote transport, richer remote observation/control, privileged UX claims, grant writes, bridge-control choreography, or S4 restart/reload semantics.

---

## First local-only lifecycle/write JSON/RPC slice

The first write-side JSON/RPC follow-up above the landed `LocalAgentApi` slice is now implementation-backed and still stays local-only on the current localhost shell host:

- The current localhost shell host now accepts `agent.run` and `agent.stop` JSON/RPC methods; no remote adapter, auth model, or broader transport guarantee is implied.
- Those write methods route through `LocalAgentApi` on the same persisted local state path already used by the local CLI and current wrapper instead of inventing a second write path.
- Read-after-write observation stays on the current read-first methods, especially `agent.describe`, `agent.snapshot`, and `denyLog.list`, so the landed write slice still reuses the existing observation path.
- Denied writes keep the same local deny behavior: the persisted deny-log summaries stay stable on the local seam, while the JSON/RPC write path returns a local-only authorization error envelope rather than publishing a broader remote-control contract.
- Browser control, privileged UX, grant writes, bridge-control choreography, richer remote observation/control, and broader S4 restart/reload semantics stay unpublished until a later code-backed follow-up exists.

---

## Immediate next steps

1. Keep `LocalAgentApi`, the current local CLI, the landed localhost `agent.run` / `agent.stop` JSON/RPC methods, and the current read-first inspection methods aligned on one local state path.
2. Keep the shell UI itself observation-only until an explicit S5-owned follow-up decides how, or whether, it should drive the landed local-only write methods.
3. Keep browser control, privileged UX, grant writes, bridge-control choreography, richer remote observation/control, and broader S4 restart/reload semantics unpublished until later code-backed follow-up work exists.
