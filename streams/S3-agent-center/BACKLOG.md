# S3 Agent Center — Backlog

---

## Now

- [x] Sketch pre-G3 `Agent` trait interface (host-agnostic until S4 executor stubs land)
- [x] Review ADR-018 for `botgen-rust`
- [ ] Review ADR-019 for `workpace-rust`

## Next

- [ ] Promote `crates/ferros-agents/` scaffold into the root workspace once G2/S4 unblockers are ready
- [ ] Implement `AgentManifest` with capability declarations
- [ ] Implement registry: register / deregister / list / describe
- [ ] Converge `start()` / `stop()` with S4 executor host traits
- [ ] Implement spawn/stop lifecycle with deny-by-default
- [x] IPC bus abstraction (Unix domain sockets / named pipes)
- [ ] `echo` reference agent
- [ ] `timer` reference agent
- [ ] CLI: `ferros agent list | describe | run | stop | logs`
- [ ] Harness: verify deny-by-default (ungranted capability → denied + logged)

## Later

- [ ] Named pipe transport for Windows support
- [ ] JSON/RPC layer for S5 Phase B web shell
- [ ] Agent capability introspection (describe why denied)
- [ ] Agent manifest versioning

## Blocked

- Implementation blocked on G2 (needs `ProfileId`, `CapabilityGrant` types from S2).
- Executor interface depends on S4 publishing trait stubs.
