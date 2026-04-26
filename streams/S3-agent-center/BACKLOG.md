# S3 Agent Center — Backlog

---

## Now

- [x] Sketch pre-G3 `Agent` trait interface (host-agnostic until S4 executor stubs land)
- [x] Review ADR-018 for `botgen-rust`
- [x] Review ADR-019 for `workpace-rust`

## Next

- [x] Promote `crates/ferros-agents/` scaffold into the root workspace once G2/S4 unblockers are ready
- [x] Implement `AgentManifest` with capability declarations
- [x] Implement registry: register / deregister / list / describe
- [x] Converge `start()` / `stop()` with S4 executor host traits
- [x] Implement spawn/stop lifecycle with deny-by-default authorization paths
- [x] IPC bus transport abstraction and endpoint kinds
- [x] `echo` reference agent
- [x] `timer` reference agent
- [x] Thin local CLI: `ferros agent list | describe | run | stop | logs`
- [x] Focused deny-by-default coverage across manifest authorization and demo/runtime denial logging
- [x] Land S3-owned docs-only seam classification for S7 runway planning against current registry plus local/read-first inspection surfaces
- [x] Publish the first hub-facing wrapper boundary for S7: lock what is currently honest on `AgentRegistry` plus local/read-first inspection surfaces while keeping hub-facing lifecycle-wrapper and richer remote observation/control unpublished
- [x] Land the narrow read-only wrapper/API slice: aggregated `agent.snapshot` JSON/RPC observation over current agent detail, grant-state, and deny-log sources
- [x] Harden `ferros-node demo` into a reusable runtime-host integration layer
- [ ] Expand deny-by-default evidence from the current manifest/runtime assertions into a dedicated lifecycle/log harness
- [ ] Publish the next S3 follow-up only when a real lifecycle/write wrapper/API slice exists: define the hub-facing lifecycle wrapper and any richer remote observation/control contract without inventing pairing choreography, bridge-control sequencing, or S4 restart/reload semantics

## Later

- [ ] Concrete Unix domain socket and named pipe adapters for the bus boundary
- [ ] Lifecycle/write JSON/RPC follow-up for S5 Phase B web shell once a real privileged wrapper slice exists
- [ ] Agent capability introspection (describe why denied)
- [ ] Agent manifest versioning

## Blocked

- Final G3 contract freeze still depends on G2 locking `ProfileId` and `CapabilityGrant`.
- Broader lifecycle/write host/API convergence still depends on S4 stabilizing shared execution traits.
