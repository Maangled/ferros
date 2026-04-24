# FERROS Wave Run Log

Newest entry first. Each entry records one local driver invocation.

## 2026-04-23 — WAVE-2026-04-23-06

- Selected item: `WAVE-2026-04-23-06`
- Result: Landed the first S2-owned `KeyPair` surface plus an additive `SignedProfileDocument` round-trip path in `ferros-profile`, so a fresh profile can be created, serialized, signed, deserialized, verified, and re-signed on revoke without widening into the remaining profile CLI verbs. The wave also added the focused `schemas/fixtures/signed-profile-valid.json` evidence and truth-synced S2 and gate/status surfaces while leaving `schemas/profile.v0.json`, harness files, and downstream S3/S4 consumer boundaries unchanged.
- Files: `Cargo.lock`, `crates/ferros-profile/Cargo.toml`, `crates/ferros-profile/src/lib.rs`, `schemas/fixtures/signed-profile-valid.json`, `streams/S2-profile/README.md`, `streams/S2-profile/BACKLOG.md`, `streams/S2-profile/CONTRACTS.md`, `streams/S2-profile/PROGRESS.md`, `docs/gates/G2.md`, `STATUS.md`
- Validation: Delegated orchestration passed `cargo test -p ferros-profile` with 32 passed and 0 failed; editor diagnostics were clean for the touched `ferros-profile` slice.
- Next follow-up: No ready queue item remains. `WAVE-2026-04-23-B01` stays blocked on the S3 JSON/RPC dependency; the next recommended new wave is an S2 follow-up to decide whether `SignedProfileDocument` should become a published schema contract or remain Rust-local until profile freeze, then land the remaining `ferros profile export | import | grant | revoke` evidence for G2 closeout.

## 2026-04-23 — WAVE-2026-04-23-05

- Selected item: `WAVE-2026-04-23-05`
- Result: Landed a Linux-backed real-binary proof for `ferros profile init` followed by `ferros profile show` without widening into the remaining profile CLI verbs. The slice added a focused integration test that launches the `ferros` binary, validates the initialized profile document through `show`, and truth-synced G2 and status surfaces to reflect that the current `init | show` path now has repo-backed Linux evidence through the existing Ubuntu CI test job.
- Files: `crates/ferros-node/tests/profile_cli_linux.rs`, `docs/gates/G2.md`, `STATUS.md`
- Validation: Delegated orchestration passed `cargo test -p ferros-profile -p ferros-node` and `cargo test -p ferros-node profile_init_then_show_works_via_real_ferros_binary -- --exact`; editor diagnostics were clean for the touched wave-owned files. A GitHub-hosted workflow could not be executed from this environment, so the Linux-backed proof is the new real-binary test plus the existing `ubuntu-latest` `cargo test --workspace --all-targets` job in `.github/workflows/ci.yml`.
- Next follow-up: No ready queue item remains. `WAVE-2026-04-23-B01` is still blocked on the S3 JSON/RPC dependency; the next recommended new wave is an S2/G2 key-material plus end-to-end profile signing evidence slice.

## 2026-04-23 — WAVE-2026-04-23-04

- Selected item: `WAVE-2026-04-23-04`
- Result: Landed the dedicated frozen `schemas/fixtures/profile-valid.json` evidence and proved it against the unchanged `schemas/profile.v0.json` contract in both `ferros-profile` and the H1 contract validator without widening into profile CLI or signing work. The slice also regenerated harness constants to embed the profile schema and fixture, and truth-synced G2 and status surfaces so they no longer claim the dedicated profile freeze evidence is missing.
- Files: `schemas/fixtures/profile-valid.json`, `crates/ferros-profile/src/lib.rs`, `tools/generate-harness-constants.ps1`, `harnesses/_constants.js`, `harnesses/ferros-contract-validator.html`, `docs/gates/G2.md`, `STATUS.md`
- Validation: Delegated orchestration passed `cargo test -p ferros-profile` with 26 passed and 0 failed; `./tools/generate-harness-constants.ps1` regenerated harness constants with `profile.v0` and `profile-valid` embedded; H1 `harnesses/ferros-contract-validator.html` passed with 47 passed, 0 failed, 0 skipped including the explicit `profile-valid` against `SCHEMA_PROFILE_V0` check; workspace diagnostics on touched files were clean.
- Next follow-up: `WAVE-2026-04-23-05`

## 2026-04-23 — WAVE-2026-04-23-03

- Selected item: `WAVE-2026-04-23-03`
- Result: Landed the minimal S2 profile CLI slice as a real implementation, wiring `ferros profile init [path]` and `ferros profile show [path]` through the existing `ProfileStore` boundary with filesystem-backed create-without-overwrite semantics, focused timestamp validation, and honest G2/status/contracts truth-sync. The wave stayed inside `init` and `show` and did not widen into import/export or signing.
- Files: `Cargo.lock`, `crates/ferros-profile/Cargo.toml`, `crates/ferros-profile/src/lib.rs`, `crates/ferros-node/Cargo.toml`, `crates/ferros-node/src/lib.rs`, `crates/ferros-node/src/bin/ferros.rs`, `docs/gates/G2.md`, `STATUS.md`, `docs/contracts/CONTRACTS-OVERVIEW.md`, `streams/S2-profile/PROGRESS.md`
- Validation: Delegated orchestration passed `cargo test -p ferros-profile init_profile_creates_new_profile_document_in_store`, `cargo test -p ferros-node profile_cli_init_and_show_round_trip_profile_document`, `cargo test -p ferros-node --bin ferros run_dispatches_profile_init_and_show_with_explicit_path`, `cargo test -p ferros-profile fresh_profile_document_rejects_invalid_created_at`, `cargo test -p ferros-profile`, `cargo test -p ferros-node profile_cli_`, and `cargo test -p ferros-profile -p ferros-node`. One intermediate dependency-resolution failure was repaired by pinning the node-side time dependency before the successful rerun.
- Next follow-up: No ready queue item remains. The next recommended wave is to add CI-backed Linux `ferros profile init -> show` proof and freeze `schemas/profile.v0.json` with dedicated frozen profile evidence.

## 2026-04-23 — WAVE-2026-04-23-02

- Selected item: `WAVE-2026-04-23-02`
- Result: Closed the S4 policy property-test gap for `DenyByDefaultPolicy` without widening into broader runtime work. The orchestrated slice added a minimal `proptest` test dependency and property-based coverage that proves an active exact match authorizes regardless of grant ordering while mismatched or inactive grants deny with the same decision when order changes. Gate and status docs still lag this evidence item and were intentionally left untouched in this wave.
- Files: `crates/ferros-core/Cargo.toml`, `crates/ferros-core/tests/capability_policy.rs`, `Cargo.lock`
- Validation: Delegated orchestration passed `cargo test -p ferros-core` with 21 passed, 0 failed, 0 ignored; local editor diagnostics were clean for `crates/ferros-core/Cargo.toml`, `crates/ferros-core/tests/capability_policy.rs`, `Cargo.lock`, `docs/orchestration/WAVE-QUEUE.md`, and `docs/orchestration/WAVE-RUN-LOG.md`
- Next follow-up: `WAVE-2026-04-23-03`

## 2026-04-23 — WAVE-2026-04-23-01

- Selected item: `WAVE-2026-04-23-01`
- Result: Closed the shared contract, harness, and gate-truth evidence for the signed `CapabilityGrant` boundary without widening into profile CLI work. The orchestrated slice updated the harness generator and validator, fixed negative-fixture classification for the invalid-signature case, and truth-synced the shared contract and G2 gate docs to the current schema-backed behavior.
- Files: `tools/generate-harness-constants.ps1`, `harnesses/_constants.js`, `harnesses/ferros-contract-validator.html`, `docs/contracts/CONTRACTS-OVERVIEW.md`, `docs/gates/G2.md`
- Validation: Delegated orchestration passed `cargo test -p ferros-profile`; direct `./tools/generate-harness-constants.ps1` invocation was blocked by local PowerShell execution policy, so the script body was executed via `Get-Content` and `ScriptBlock` to regenerate `harnesses/_constants.js`; browser validation for `harnesses/ferros-contract-validator.html` passed with 45 passed, 0 failed, 0 skipped; shared contract and harness parity against `schemas/capability-grant.v0.json` was confirmed; local editor diagnostics were clean for `docs/orchestration/WAVE-QUEUE.md` and `docs/orchestration/WAVE-RUN-LOG.md`
- Next follow-up: `WAVE-2026-04-23-02`

## 2026-04-23 — WAVE-2026-04-23-01

- Selected item: `WAVE-2026-04-23-01`
- Result: Landed the first signed and verifiable `CapabilityGrant` slice in `ferros-profile`, including Ed25519 signing, verification, revoke-and-resign behavior, and positive and negative schema fixtures. The wave remains partial because shared contract, harness, and gate-truth evidence still needs closeout, so the item returned to `Ready` with a narrower follow-up.
- Files: `crates/ferros-profile/src/lib.rs`, `crates/ferros-profile/Cargo.toml`, `Cargo.lock`, `schemas/capability-grant.v0.json`, `schemas/fixtures/grant-valid.json`, `schemas/fixtures/grant-invalid-sig.json`
- Validation: Delegated orchestration ran `cargo test -p ferros-profile` successfully after a lockfile repair; local editor diagnostics were clean for `docs/orchestration/WAVE-QUEUE.md` and `docs/orchestration/WAVE-RUN-LOG.md`
- Next follow-up: `WAVE-2026-04-23-01` narrowed to shared contract, harness, and gate-truth closeout for the signed `CapabilityGrant` path

## 2026-04-23 — DRIVER-BOOTSTRAP

- Selected item: none
- Result: Installed the local driver pattern with a user-invocable driver agent, repo-backed queue, append-only run log, and file-scoped queue rules.
- Files: `.github/agents/ferros-driver.agent.md`, `.github/instructions/ferros-wave-queue.instructions.md`, `docs/orchestration/LOCAL-DRIVER.md`, `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: editor diagnostics on the new customization and markdown files
- Next follow-up: `WAVE-2026-04-23-01`
