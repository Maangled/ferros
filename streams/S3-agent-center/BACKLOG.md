# S3 Agent Center — Backlog

---

## Now

- [ ] Sketch `Agent` trait interface (align with S4 on executor contract)
- [ ] Review S6 harvest ADRs for `botgen-rust` and `workpace-rust`

## Next

- [ ] Scaffold `crates/ferros-agents/` (after G2)
- [ ] Implement `AgentManifest` with capability declarations
- [ ] Implement registry: register / deregister / list / describe
- [ ] Implement spawn/stop lifecycle with deny-by-default
- [ ] IPC bus abstraction (Unix domain sockets / named pipes)
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
