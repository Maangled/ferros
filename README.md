# FERROS

FERROS is a local-first platform project with two active layers:

1. A contract-first web prototype layer under `docs/` and `harnesses/`.
2. A Rust workspace that now carries the first real profile, runtime, agent-center, node-host, and data slices.

G1 is closed. G2 is the active gate, and G3 remains blocked on G2. This repository is still not a bootable Rust OS.

## Current State (Honest)

- Stream A contract and harness baseline is green in the web layer (see `docs/progress/PROGRESS.md`).
- S1 foundation scaffolding is present and G1 is closed:
  - Root Cargo workspace (`Cargo.toml`)
  - Rust toolchain pin (`rust-toolchain.toml`)
  - CI workflows (`.github/workflows/ci.yml`, `.github/workflows/integration.yml`)
  - Workspace command runner (`cargo xtask ci`)
  - Ownership map (`CODEOWNERS`)
  - Dual shell helpers (`tools/sh/`, `tools/ps/`)
  - Workspace crates: `ferros-core`, `ferros-profile`, `ferros-runtime`, `ferros-agents`, `ferros-node`, and `ferros-data`
- S3 has a shipped local agent CLI in the `ferros` binary (`agent list | describe | run | stop | logs`) and a test-backed `ferros-node demo`, but G3 is still blocked on G2 and remaining runtime evidence.
- S4 has landed the first `ferros-core` policy slice, `ferros-runtime`, and a validated `cargo check -p ferros-core --no-default-features` compile slice; this is not yet a claim of full embedded or `no_std` readiness.
- S5 Phase A is live on `site/index.html`: the landing page moved there and now carries an honest repository-status banner. The local agent-center web shell is still Phase B work pending G3.
- S6 has admitted `crates/ferros-data/` to the root workspace; downstream consumer adoption remains stream-owned follow-up work.
- S8 now has the core governance baseline in repo (`STATUS.md`, gate docs, contracts overview, `CONTRIBUTING.md`, `DOCTRINE.md`, ADR template and index surfaces, `SECURITY.md`, `THREAT-MODEL.md`, `GOVERNANCE.md`, `CODE_OF_CONDUCT.md`, and contributor intake templates); good-first-issue seeding remains open follow-up work.

## Repository Layout

- `site/` - founding blueprint and site entry
- `crates/` - Rust workspace crates (`ferros-core`, `ferros-profile`, `ferros-runtime`, `ferros-agents`, `ferros-node`, `ferros-data`)
- `docs/` - architecture docs, ADRs, stream plans, progress records
- `schemas/` - schema contracts and fixtures
- `harnesses/` - browser-run validation harnesses
- `streams/` - stream-scoped planning and gates
- `xtask/` - Rust automation crate placeholder

## Validate Foundation Gate (S1 / G1)

Run from repository root:

```powershell
cargo build --workspace --all-targets
cargo test --workspace --all-targets
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo xtask ci
```

## Primary References

- Progress tracker: `docs/progress/PROGRESS.md`
- Stream map: `docs/streams/STREAMS-OVERVIEW.md`
- S1 gate docs: `streams/S1-foundation/README.md`
- Agent guide: `docs/AGENT_GUIDE.md`

## License

- Code: `LICENSE`
- Documentation: `LICENSE-DOCS`
- Licensing notes: `LICENSING.md`
