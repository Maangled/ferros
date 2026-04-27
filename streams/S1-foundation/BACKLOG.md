# S1 Foundation — Backlog

---

## Now

- [x] Add `Cargo.toml` workspace root
- [x] Add `rust-toolchain.toml`
- [x] Add `.github/workflows/ci.yml` (fmt / clippy / test on Linux, macOS, Windows)
- [x] Add initial workspace member crates (`ferros-core`, `ferros-profile`)
- [x] Move `ferros-blueprint.html` → `site/index.html`
- [x] Update `README.md` to honest current state
- [x] Add `CODEOWNERS`
- [x] Resolve workspace licensing metadata mismatch
- [x] Audit crate manifests and confirm current local wiring (`xtask` std-only; `ferros-profile` depends on `ferros-core`, `serde`, and `serde_json`)
- [x] Open the trivial G1 evidence PR
- [x] Observe a green `ci.yml` matrix run on `ubuntu-latest`, `macos-latest`, and `windows-latest` (run #24812246339, 2026-04-23)
- [x] Update `docs/gates/G1.md` after PR evidence lands
- [x] Close issue #62 as the G1 umbrella follow-up
- [ ] Tag `v0.0.1-foundation`

## Next

- [x] Add `xtask/` crate for build automation
- [x] Turn `xtask` into a real `cargo xtask ci` runner
- [x] Add `tools/sh/` and `tools/ps/` placeholder scripts
- [x] LICENSE split: `LICENSE` (code) + `LICENSE-DOCS` (docs)
- [x] `integration.yml` workflow for weekly integration branch merges
- [ ] Add a branch protection rule that requires `ci.yml` to pass before merge

## Later

- [x] Release workflow (`release.yml`)
- [ ] Dependabot / Renovate config for dependency updates
- [ ] MSRV policy documented in `README.md`
- [ ] Continue moving active HTML surfaces into `docs/surfaces/active/` per ADR-017

## Blocked

- [ ] GitHub-side actions only: tag creation and branch-protection verification cannot be completed from the local workspace alone
