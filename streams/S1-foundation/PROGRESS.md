# S1 Foundation — Progress

Reverse-chronological. Append a dated entry at the top per session.

---

## 2026-04-22 — Workspace members and xtask validation landed

- Expanded the workspace to include `crates/ferros-core` and `crates/ferros-profile` as initial Rust members.
- Replaced the `xtask` placeholder with a real `cargo xtask ci` command path.
- Added `.cargo/config.toml` so the `cargo xtask` alias works without extra local setup.
- Aligned workspace licensing metadata to the repository GPL code license and documented the split-license model in `LICENSING.md`.
- Local foundation validation now passes through `cargo xtask ci` (fmt, clippy, build, test).

## 2026-04-22 — G1 foundation backlog implemented

- Added root Rust workspace (`Cargo.toml`) with `xtask` member.
- Added `rust-toolchain.toml` pinned to stable with `rustfmt` and `clippy` components.
- Added CI workflow (`.github/workflows/ci.yml`) for fmt/clippy/build/test on Linux, macOS, and Windows.
- Added weekly integration branch sync workflow (`.github/workflows/integration.yml`).
- Added `CODEOWNERS` with per-stream ownership entries.
- Added cross-platform tooling placeholders: `tools/sh/bootstrap.sh` and `tools/ps/bootstrap.ps1`.
- Added `LICENSE-DOCS` and rewrote root `README.md` to an honest current-state summary.
- Moved blueprint payload to `site/index.html` and left a compatibility redirect at `ferros-blueprint.html`.
- Local validation passed: `cargo build`, `cargo test`, `cargo fmt --check`, `cargo clippy -- -D warnings`.

## 2026-04-21 — Stream scaffolded

- Stream directory and planning documents created.
- S1 is the first active stream; no code changes yet.
- Immediate next steps captured in README.md.
- Waiting on contributor to begin Cargo workspace setup.
