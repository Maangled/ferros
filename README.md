# FERROS

FERROS is a local-first platform project with two active layers:

1. A contract-first web prototype layer under `docs/` and `harnesses/`.
2. A newly bootstrapped Rust workspace used to establish cross-platform build discipline.

This repository is in early foundation stage. It is not yet a bootable Rust OS.

## Current State (Honest)

- Stream A contract and harness baseline is green in the web layer (see `docs/progress/PROGRESS.md`).
- S1 foundation scaffolding is now present:
  - Root Cargo workspace (`Cargo.toml`)
  - Rust toolchain pin (`rust-toolchain.toml`)
  - CI workflows (`.github/workflows/ci.yml`, `.github/workflows/integration.yml`)
  - Ownership map (`CODEOWNERS`)
  - Dual shell helpers (`tools/sh/`, `tools/ps/`)
- The founding blueprint now lives at `site/index.html`.

## Repository Layout

- `site/` - founding blueprint and site entry
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
```

## Primary References

- Progress tracker: `docs/progress/PROGRESS.md`
- Stream map: `docs/streams/STREAMS-OVERVIEW.md`
- S1 gate docs: `streams/S1-foundation/README.md`
- Agent guide: `docs/AGENT_GUIDE.md`

## License

- Code: `LICENSE`
- Documentation: `LICENSE-DOCS`
