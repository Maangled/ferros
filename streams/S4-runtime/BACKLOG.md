# S4 Runtime / OS Core — Backlog

---

## Now

- [ ] Design capability and consent primitive types against the upcoming S2 frozen boundaries
- [ ] Design policy engine interface

## Next

- [ ] Extend the existing `crates/ferros-core/` foundation crate beyond the marker-only slice
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

No upstream G1 blocker remains. Real `CapabilityGrant` wiring still depends on G2; keep runtime and node implementation shallow until the S2 type is frozen.
