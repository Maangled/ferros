# FERROS Wave Run Log

Newest entry first. Each entry records one local driver invocation.

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
