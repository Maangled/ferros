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

## Next

- [x] Add `xtask/` crate for build automation
- [x] Turn `xtask` into a real `cargo xtask ci` runner
- [x] Add `tools/sh/` and `tools/ps/` placeholder scripts
- [x] LICENSE split: `LICENSE` (code) + `LICENSE-DOCS` (docs)
- [x] `integration.yml` workflow for weekly integration branch merges

## Later

- [ ] Release workflow (`release.yml`)
- [ ] Dependabot / Renovate config for dependency updates
- [ ] MSRV policy documented in `README.md`

## Blocked

- [ ] First PR-based GitHub Actions matrix proof for G1
