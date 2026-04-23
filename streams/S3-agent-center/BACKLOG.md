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
- [x] Implement spawn/stop lifecycle with deny-by-default
- [x] IPC bus abstraction (Unix domain sockets / named pipes)
- [x] `echo` reference agent
- [x] `timer` reference agent
- [ ] CLI: `ferros agent list | describe | run | stop | logs`
- [x] Harness: verify deny-by-default (ungranted capability → denied + logged)
- [ ] Harden `ferros-node demo` into a reusable runtime-host integration layer

## Later

- [ ] Named pipe transport for Windows support
- [ ] JSON/RPC layer for S5 Phase B web shell
- [ ] Agent capability introspection (describe why denied)
- [ ] Agent manifest versioning

## Blocked

- Implementation blocked on G2 (needs `ProfileId`, `CapabilityGrant` types from S2).
- Executor interface depends on S4 publishing trait stubs.
