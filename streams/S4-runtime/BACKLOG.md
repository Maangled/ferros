# S4 Runtime / OS Core — Backlog

---

## Now

- [ ] Design capability and consent primitive types (can precede G1 as a sketch)
- [ ] Design policy engine interface

## Next

- [ ] Scaffold `crates/ferros-core/` (after G1)
- [ ] Implement capability + consent types
- [ ] Implement policy engine with deny-by-default logic
- [ ] Unit tests: 10+ grant/deny scenarios
- [ ] Scaffold `crates/ferros-runtime/` — executor + in-process bus
- [ ] Scaffold `crates/ferros-node/` binary
- [ ] `ferros-node demo` subcommand
- [ ] Replace stub grant type with S2 `CapabilityGrant` (after G2)

## Later

- [ ] Property tests for policy engine (`proptest` or `quickcheck`)
- [ ] `no_std` feature on `ferros-core` (needed for embedded target)
- [ ] Multi-threaded executor option
- [ ] Persistent bus message log (for audit)

## Blocked

- Crate creation blocked on G1 (Cargo workspace).
- `CapabilityGrant` type dependency on G2 (use stub until then).
