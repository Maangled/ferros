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
- [ ] Harden `ferros-node demo` into a reusable runtime-host integration layer
- [ ] Publish the first hub-facing wrapper boundary for S7: decide what stays on `AgentRegistry` plus local/read-first inspection surfaces and what additional lifecycle or remote-observation contract must exist before bridge control flows are honest

## Later

- [ ] Concrete Unix domain socket and named pipe adapters for the bus boundary
- [ ] JSON/RPC layer for S5 Phase B web shell
- [ ] Agent capability introspection (describe why denied)
- [ ] Agent manifest versioning

## Blocked

- Final G3 contract freeze still depends on G2 locking `ProfileId` and `CapabilityGrant`.
- Reusable runtime-host/API convergence still depends on S4 stabilizing shared execution traits.
