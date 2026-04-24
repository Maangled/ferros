# FERROS Wave Queue

This queue feeds the local driver pattern. Process one wave per invocation unless the user explicitly requests a batch.

## Ready

### WAVE-2026-04-23-08

- Title: Start S7 pairing and hardware design pack
- Status: ready
- Priority: P1
- Gate: G4 runway
- Owning streams: S7 primary, S8 truth-sync if docs move, S2 consumer awareness if pairing contract wording shifts
- Goal: Keep the S7 runway moving by finishing the reference hardware recipe and documenting the current pairing constraints, open questions, and grant-aware design posture without freezing authoritative pairing semantics yet.
- Anchor files: `streams/S7-hub/README.md`, `streams/S7-hub/BACKLOG.md`, `docs/hub/reference-hardware.md`, `docs/hub/`
- Validation: editor diagnostics on touched docs; verify the pairing and hardware docs stay consistent with the current S7 README and backlog boundaries
- Constraints: Keep the slice to S7 design and documentation runway. Keep pairing notes provisional, do not scaffold `crates/ferros-hub/`, do not start the HA bridge, and do not claim G4 evidence in this wave.
- Last update: 2026-04-24

### WAVE-2026-04-23-09

- Title: Execute S5 Phase A archive and link-hygiene pack
- Status: ready
- Priority: P1
- Gate: post-G3 runway
- Owning streams: S5 primary, S8 truth-sync if archive surfaces move
- Goal: Verify inbound links, archive the inactive top-level HTML prototypes to `docs/legacy/`, and keep the real `site/` surface clean for the later local shell without starting localhost UI work yet.
- Anchor files: `site/index.html`, `streams/S5-ux/DOCS-HTML-PROTOTYPE-AUDIT.md`, `docs/`, `docs/legacy/`
- Validation: editor diagnostics on touched files; grep inbound references before moving any prototype files so active links are not broken
- Constraints: Keep `docs/agent-command-center.html` and `docs/forge-workbench.html` active. Do not start the S5 Phase B local web shell in this wave.
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

### WAVE-2026-04-24-02

- Title: Close G2 with the remaining profile CLI evidence
- Status: done
- Priority: P0
- Gate: G2
- Owning streams: S2 primary, S8 truth-sync after landing, S1 support only if a repo-backed Linux proof surface needs to move
- Goal: Finish the only remaining G2 blocker by landing a repo-backed `ferros profile export`, `import`, `grant`, and `revoke` path, including the minimum local persistence boundary for key material and signed grant state, without widening the frozen published v0 contracts or changing downstream S3, S4, or S7 consumer boundaries.
- Anchor files: `crates/ferros-profile/src/lib.rs`, `crates/ferros-node/src/lib.rs`, `crates/ferros-node/src/bin/ferros.rs`, `schemas/profile.v0.json`, `schemas/capability-grant.v0.json`, `docs/gates/G2.md`, `STATUS.md`
- Validation: `cargo test -p ferros-profile -p ferros-node`; repo-backed real-binary proof that `ferros profile init`, `grant`, `export`, `import`, `revoke`, and `show` succeed against real files and preserve the frozen `profile.v0.json` and `capability-grant.v0.json` boundaries
- Constraints: Keep `profile.v0.json` frozen as the unsigned published v0 consumer contract. Keep `SignedProfileDocument` Rust-local at v0. Do not mutate `capability-grant.v0.json`. Do not widen S3 or S4 runtime and manifest contracts, S7 pairing semantics, optional passphrase wrap, or post-G2 UX work. If a new on-disk bundle format is needed, keep it local to the CLI and store surface rather than publishing a new shared schema.
- Last update: 2026-04-24

### WAVE-2026-04-24-01

- Title: Freeze profile.v0 and settle the signed-profile v0 boundary
- Status: done
- Priority: P0
- Gate: G2
- Owning streams: S2 primary, S8 truth-sync after landing, S3 and S4 consumer awareness only if the published contract wording shifts
- Goal: Convert the current `schemas/profile.v0.json` freeze candidate into the actual frozen v0 contract by deciding whether `SignedProfileDocument` stays Rust-local at v0, landing only the minimal schema and parity changes required for freeze, and propagating that final contract through shared validation and truth surfaces without widening into the remaining profile CLI verbs.
- Anchor files: `schemas/profile.v0.json`, `crates/ferros-profile/src/lib.rs`, `schemas/fixtures/profile-valid.json`, `schemas/fixtures/signed-profile-valid.json`, `tools/generate-harness-constants.ps1`, `harnesses/_constants.js`, `harnesses/ferros-contract-validator.html`, `streams/S2-profile/CONTRACTS.md`, `docs/gates/G2.md`, `docs/contracts/CONTRACTS-OVERVIEW.md`, `STATUS.md`
- Validation: `cargo test -p ferros-profile`; if fixture or schema coverage changes, regenerate harness constants and confirm `harnesses/ferros-contract-validator.html` still accepts the frozen profile fixture set; confirm editor diagnostics are clean on touched S2 and truth-sync files
- Constraints: Keep the slice inside profile.v0 freeze semantics and freeze evidence. Do not start `ferros profile export | import | grant | revoke` in this wave. Do not mutate `schemas/capability-grant.v0.json`. Do not publish a separate signed-profile schema unless S2 can prove the unsigned profile.v0 contract cannot be frozen cleanly without it.
- Last update: 2026-04-24

### WAVE-2026-04-23-07

- Title: Tighten G3 evidence and CI demo proof
- Status: done
- Priority: P0
- Gate: G3
- Owning streams: S3 primary, S4 primary, S8 truth-sync if gate or status surfaces move
- Goal: Sync G3-facing docs to the already-landed S4 property tests and add a repo-backed CI proof for `cargo run --bin ferros -- demo` without widening into JSON/RPC or reusable host work.
- Anchor files: `.github/workflows/ci.yml`, `docs/gates/G3.md`, `STATUS.md`, `streams/S4-runtime/BACKLOG.md`, `docs/contracts/CONTRACTS-OVERVIEW.md`
- Validation: `cargo test -p ferros-core -p ferros-runtime -p ferros-agents -p ferros-node`; `cargo check -p ferros-core --no-default-features`; `cargo run --bin ferros -- demo`
- Constraints: Keep the slice inside G3 evidence, CI proof, and truth-sync. Do not start S3 JSON/RPC, reusable host work, or S5 shell implementation in this wave.
- Last update: 2026-04-23

### WAVE-2026-04-23-06

- Title: Land `KeyPair` and signed profile round-trip evidence
- Status: done
- Priority: P0
- Gate: G2
- Owning streams: S2 primary, S8 truth-sync if gate or contract docs move
- Goal: Add the first S2-owned key-material surface plus a signed profile round-trip path so `ferros-profile` can create a fresh profile, serialize it, sign it, verify it, and prove the contract with focused tests and fixtures without widening into the remaining profile CLI verbs.
- Anchor files: `crates/ferros-profile/src/lib.rs`, `crates/ferros-profile/Cargo.toml`, `schemas/profile.v0.json`, `schemas/fixtures/`, `docs/gates/G2.md`, `STATUS.md`, `streams/S2-profile/CONTRACTS.md`, `streams/S2-profile/PROGRESS.md`
- Validation: `cargo test -p ferros-profile`; update harness or truth surfaces only if the profile schema contract actually changes
- Constraints: Keep the slice inside S2 key material and signed profile evidence. Do not start `ferros profile export | import | grant | revoke` in this wave. Avoid changing downstream S3/S4 consumer boundaries unless the signed profile contract truly requires it.
- Last update: 2026-04-23

### WAVE-2026-04-23-05

- Title: Add Linux-backed `ferros profile init` to `show` proof
- Status: done
- Priority: P1
- Gate: G2
- Owning streams: S2 primary, S1 support if CI or workflow surfaces move, S8 truth-sync if gate docs change
- Goal: Land a repo-backed Linux proof for `ferros profile init` followed by `ferros profile show`, using the already-landed minimal CLI path without widening into `export | import | grant | revoke`.
- Anchor files: `.github/workflows/ci.yml`, `crates/ferros-node/src/bin/ferros.rs`, `crates/ferros-node/src/lib.rs`, `docs/gates/G2.md`, `STATUS.md`
- Validation: local `cargo test -p ferros-profile -p ferros-node`; repo-hosted Linux workflow or equivalent scripted proof for `ferros profile init` then `ferros profile show`
- Constraints: Keep the slice focused on Linux-backed evidence for the current `init | show` path. Do not start the remaining profile CLI subcommands in this wave.
- Last update: 2026-04-23

### WAVE-2026-04-23-04

- Title: Freeze profile.v0 golden fixture evidence
- Status: done
- Priority: P0
- Gate: G2
- Owning streams: S2 primary, S8 truth-sync if docs or harness surfaces move
- Goal: Add the dedicated frozen `schemas/fixtures/profile-valid.json` artifact, prove it matches `schemas/profile.v0.json`, and sync any harness or gate surfaces that still assume profile freeze evidence is missing.
- Anchor files: `schemas/profile.v0.json`, `schemas/fixtures/profile-valid.json`, `crates/ferros-profile/src/lib.rs`, `tools/generate-harness-constants.ps1`, `harnesses/_constants.js`, `harnesses/ferros-contract-validator.html`, `docs/gates/G2.md`, `STATUS.md`
- Validation: `cargo test -p ferros-profile`; regenerate harness constants if fixture coverage changes; confirm H1 contract validator still passes for the profile schema set
- Constraints: Keep the slice to profile fixture freeze evidence and truth-sync. Do not widen into new CLI subcommands or profile signing work.
- Last update: 2026-04-23

### WAVE-2026-04-23-03

- Title: Land the minimal S2 profile CLI slice
- Status: done
- Priority: P2
- Gate: G2
- Owning streams: S2 primary, S3 consumer awareness if contracts shift
- Goal: Ship the smallest useful `ferros profile init | show` path with filesystem-backed storage, using the already-landed `ProfileStore` as the persistence boundary.
- Anchor files: `crates/ferros-profile/src/lib.rs`, `crates/ferros-node/src/bin/ferros.rs`, `crates/ferros-node/src/lib.rs`, `docs/gates/G2.md`
- Validation: `cargo test -p ferros-profile -p ferros-node`
- Constraints: Keep the slice to `init` and `show` unless the implementation naturally supports one more subcommand with test coverage. Do not widen into import/export or signing in the same wave.
- Last update: 2026-04-23

### WAVE-2026-04-23-02

- Title: Add S4 policy property tests
- Status: done
- Priority: P1
- Gate: G3
- Owning streams: S4 primary
- Goal: Add property tests for `DenyByDefaultPolicy` that prove deny-by-default invariants across grant ordering and profile/capability mismatches without widening into unrelated runtime work.
- Anchor files: `crates/ferros-core/src/capability.rs`, `crates/ferros-core/tests/capability_policy.rs`, `crates/ferros-core/Cargo.toml`
- Validation: `cargo test -p ferros-core`
- Constraints: Keep the slice focused on policy properties and test dependencies. Do not claim full embedded readiness or broader `no_std` completion.
- Last update: 2026-04-23

### WAVE-2026-04-23-01

- Title: Freeze `CapabilityGrant` signing and verification evidence
- Status: done
- Priority: P0
- Gate: G2
- Owning streams: S2 primary, S8 truth-sync if docs move
- Goal: Finish the shared contract, harness, and gate-truth evidence for the now-landed signed and verifiable `CapabilityGrant` path without widening beyond the frozen stripped-payload signing rule.
- Anchor files: `schemas/capability-grant.v0.json`, `docs/contracts/CONTRACTS-OVERVIEW.md`, `tools/generate-harness-constants.ps1`, `harnesses/_constants.js`, `harnesses/ferros-contract-validator.html`, `docs/gates/G2.md`
- Validation: `cargo test -p ferros-profile`; confirm shared contract and harness surfaces match `schemas/capability-grant.v0.json`
- Constraints: Keep the slice inside G2 evidence. Do not start the full profile CLI here. Limit follow-up to contract, harness, and gate truth-sync for the signed `CapabilityGrant` boundary.
- Last update: 2026-04-23

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
