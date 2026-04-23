# FERROS Wave Queue

This queue feeds the local driver pattern. Process one wave per invocation unless the user explicitly requests a batch.

## Ready

### WAVE-2026-04-23-01

- Title: Freeze `CapabilityGrant` signing and verification evidence
- Status: ready
- Priority: P0
- Gate: G2
- Owning streams: S2 primary, S8 truth-sync if docs move
- Goal: Finish the shared contract, harness, and gate-truth evidence for the now-landed signed and verifiable `CapabilityGrant` path without widening beyond the frozen stripped-payload signing rule.
- Anchor files: `schemas/capability-grant.v0.json`, `docs/contracts/CONTRACTS-OVERVIEW.md`, `tools/generate-harness-constants.ps1`, `harnesses/_constants.js`, `harnesses/ferros-contract-validator.html`, `docs/gates/G2.md`
- Validation: `cargo test -p ferros-profile`; confirm shared contract and harness surfaces match `schemas/capability-grant.v0.json`
- Constraints: Keep the slice inside G2 evidence. Do not start the full profile CLI here. Limit follow-up to contract, harness, and gate truth-sync for the signed `CapabilityGrant` boundary.
- Last update: 2026-04-23

### WAVE-2026-04-23-02

- Title: Add S4 policy property tests
- Status: ready
- Priority: P1
- Gate: G3
- Owning streams: S4 primary
- Goal: Add property tests for `DenyByDefaultPolicy` that prove deny-by-default invariants across grant ordering and profile/capability mismatches without widening into unrelated runtime work.
- Anchor files: `crates/ferros-core/src/capability.rs`, `crates/ferros-core/tests/capability_policy.rs`, `crates/ferros-core/Cargo.toml`
- Validation: `cargo test -p ferros-core`
- Constraints: Keep the slice focused on policy properties and test dependencies. Do not claim full embedded readiness or broader `no_std` completion.
- Last update: 2026-04-23

### WAVE-2026-04-23-03

- Title: Land the minimal S2 profile CLI slice
- Status: ready
- Priority: P2
- Gate: G2
- Owning streams: S2 primary, S3 consumer awareness if contracts shift
- Goal: Ship the smallest useful `ferros profile init | show` path with filesystem-backed storage, using the already-landed `ProfileStore` as the persistence boundary.
- Anchor files: `crates/ferros-profile/src/lib.rs`, `crates/ferros-node/src/bin/ferros.rs`, `crates/ferros-node/src/lib.rs`, `docs/gates/G2.md`
- Validation: `cargo test -p ferros-profile -p ferros-node`
- Constraints: Keep the slice to `init` and `show` unless the implementation naturally supports one more subcommand with test coverage. Do not widen into import/export or signing in the same wave.
- Last update: 2026-04-23

## In Progress

None.

## Blocked

### WAVE-2026-04-23-B01

- Title: Start S5 local web shell against JSON/RPC
- Status: blocked
- Priority: P3
- Gate: G3
- Owning streams: S5 primary, S3 dependency
- Goal: Begin the local agent-center web shell implementation.
- Anchor files: `site/`, `crates/ferros-agents/`, `streams/S5-ux/`
- Validation: UI acceptance plus S3 API validation once the surface exists
- Constraints: Blocked until the S3 JSON/RPC API exists and G3 is materially closer.
- Last update: 2026-04-23

## Done

### WAVE-2026-04-23-D01

- Title: Propagate shared revocation semantics through S2 and S3
- Status: done
- Priority: P0
- Gate: G2/G3 boundary hygiene
- Owning streams: S2, S3, S4
- Goal: Ensure revoked grants no longer authorize work through the shared grant and manifest boundary.
- Anchor files: `crates/ferros-profile/src/lib.rs`, `crates/ferros-agents/src/manifest.rs`
- Validation: `cargo test -p ferros-profile -p ferros-agents`; `cargo test -p ferros-node`
- Constraints: Keep the fix at the shared boundary, not only the node adapter.
- Last update: 2026-04-23
