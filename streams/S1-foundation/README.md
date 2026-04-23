# S1 — Foundation

**Stream:** S1  
**Status:** ✅ G1 closed; follow-on hygiene remains  
**Gate:** G1

---

## Mission

Make the repository a coherent Rust workspace that CI can build and test on Linux, macOS, and Windows. Everything else depends on this being green.

---

## Scope

- Convert the repo root to a Cargo workspace (`Cargo.toml` with `[workspace]`).
- Move `ferros-blueprint.html` and related site assets to `/site/index.html`.
- Set up GitHub Actions CI: `fmt`, `clippy`, `test` on Ubuntu, macOS, Windows.
- Dual-platform tooling: `tools/sh/` (bash scripts) + `tools/ps/` (PowerShell scripts) + `xtask/` crate for complex build tasks.
- Establish `rust-toolchain.toml` pinning the stable Rust edition.
- License split: `LICENSE` (code) + `LICENSE-DOCS` (documentation).
- Honest `README.md` that reflects actual state, not aspirational state.

---

## Out of scope

- Any application crate logic (that is S2–S7).
- Documentation beyond README and status (that is S8).
- Home Assistant integration (that is S7).

---

## Dependencies

None — S1 is the root stream.

---

## What this stream blocks

G1 is closed. Ongoing S1 hygiene no longer blocks S2–S8 execution, but the foundation tag and branch-protection follow-through still matter for repo discipline.

---

## Definition of done

- [x] `cargo build` passes on `x86_64-linux`, `x86_64-macos`, `x86_64-windows` (CI run #24812246339, 2026-04-23).
- [x] `cargo test` passes (empty test suite is acceptable at G1) (CI run #24812246339, 2026-04-23).
- [x] `cargo fmt --check` passes.
- [x] `cargo clippy -- -D warnings` passes.
- [x] CI workflow exists and runs on every PR.
- [x] `ferros-blueprint.html` lives at `/site/index.html`.
- [x] `README.md` reflects actual project state.
- [x] `CODEOWNERS` exists with per-stream ownership entries.

---

## Likely crates / files

| Path | Role |
|------|------|
| `Cargo.toml` | Workspace root |
| `rust-toolchain.toml` | Rust edition pin |
| `.github/workflows/ci.yml` | CI pipeline |
| `crates/ferros-core/` | Initial runtime crate skeleton |
| `crates/ferros-profile/` | Initial profile crate skeleton |
| `xtask/` | Build task crate |
| `tools/sh/` | Bash helper scripts |
| `tools/ps/` | PowerShell helper scripts |
| `site/index.html` | `ferros-blueprint.html` moved here |
| `CODEOWNERS` | Per-stream ownership |

---

## Immediate next steps

1. Tag `v0.0.1-foundation`.
2. Verify branch protection on `main` requires the CI status checks before merge.
3. Keep release automation, dependency automation, and MSRV policy as rolling hygiene while S2 takes the critical path.
