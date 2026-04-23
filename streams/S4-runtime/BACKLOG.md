# S4 Runtime / OS Core — Backlog

---

## Now

- [x] Design capability and consent primitive types against the upcoming S2 frozen boundaries
- [x] Design policy engine interface using ADR-018 lifecycle, task-history, and deny-by-default implications as prior art constraints
- [ ] Converge the local `CapabilityGrantView` adapter with S2's concrete `CapabilityGrant` after G2 closes

## Next

- [x] Extend the existing `crates/ferros-core/` foundation crate beyond the marker-only slice
- [x] Implement capability + consent types
- [x] Implement policy engine with deny-by-default logic
- [x] Unit tests: 10+ grant/deny scenarios
- [x] Scaffold `crates/ferros-runtime/` — executor + in-process bus boundary
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
