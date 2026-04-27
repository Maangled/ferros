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
- [x] Record a local `cargo check -p ferros-core --target thumbv7em-none-eabi --no-default-features` proof and configure CI to enforce the same check
- [x] Scaffold `crates/ferros-runtime/` — executor + in-process bus boundary
- [x] Scaffold `crates/ferros-node/` binary
- [x] `ferros demo` subcommand
- [x] Use the current concrete S2 `CapabilityGrant` type in the convergence demo path
- [x] Land S4-owned docs-only seam classification for S7 runway planning against the current policy surface and the narrow published reload boundary while broader durable hub-facing restart/re-registration semantics remain unpublished
- [ ] Harden the current in-memory host path beyond the convergence demo
- [x] Publish the narrow docs-only S7 reload boundary: validated local profile/grant reload plus fixed reference-runtime state replay are published today, while broader durable hub-facing restart/re-registration semantics remain unpublished
- [x] Add focused `ferros-node` and `ferros-profile` tests that lock the published reload boundary without widening into durable hub-facing restart or re-registration semantics
- [x] Extract the first internal local host-controller surface above the current CLI/state path without publishing a broader lifecycle/write wrapper/API or remote-control contract
- [x] Support the first broader local-only lifecycle/write wrapper/API slice above the extracted controller seam without publishing remote transport, remote-control semantics, or broader S4 restart/reload semantics
- [ ] Keep S4 support narrow while the next S3 follow-up exposes richer local deny-reason introspection on the landed `LocalAgentApi` seam without widening into remote transport or broader restart/reload publication

## Later

- [ ] Multi-threaded executor option
- [ ] Persistent bus message log (for audit)

## Post-G3 constraints

G3 is now closed. The next S4 work is broader `no_std` hardening, host/runtime hardening beyond the in-memory demo, and the narrow runtime support required by the first post-G3 S3 contract wave; keep runtime and node changes shallow unless they are directly needed by those owned slices.
