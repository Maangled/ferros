# S4 Runtime / OS Core — Backlog

---

## Now

- [x] Design capability and consent primitive types against the upcoming S2 frozen boundaries
- [x] Design policy engine interface using ADR-018 lifecycle, task-history, and deny-by-default implications as prior art constraints
- [x] Converge the current runtime path with S2's concrete `CapabilityGrant` via `CapabilityGrantView`

## Next

- [x] Extend the existing `crates/ferros-core/` foundation crate beyond the marker-only slice
- [x] Implement capability + consent types
- [x] Implement policy engine with deny-by-default logic
- [x] Unit tests: 10+ grant/deny scenarios
- [x] Property tests for policy engine (`proptest`)
- [x] Validate the current `ferros-core` std/no_std boundary with a host-side `cargo check -p ferros-core --no-default-features`
- [x] Scaffold `crates/ferros-runtime/` — executor + in-process bus boundary
- [x] Scaffold `crates/ferros-node/` binary
- [x] `ferros demo` subcommand
- [x] Use the current concrete S2 `CapabilityGrant` type in the convergence demo path
- [ ] Harden the current in-memory host path beyond the convergence demo

## Later

- [ ] Extend the current host-side `--no-default-features` slice into CI-backed embedded-target `no_std` validation for `ferros-core`
- [ ] Multi-threaded executor option
- [ ] Persistent bus message log (for audit)

## Blocked

No upstream G1 blocker remains. G2 contract freeze, further hardening, and a recorded green hosted CI run reference for the newly wired workflow steps still remain before G3 can advance; keep runtime and node implementation shallow until the S2 type is frozen.
