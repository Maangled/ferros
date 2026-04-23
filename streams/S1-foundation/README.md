# S1 — Foundation

**Stream:** S1  
**Status:** 🟡 In progress (local gate checks green; awaiting CI matrix confirmation)  
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

All other streams. G1 must close before S2–S8 work meaningfully.

---

## Definition of done

- [ ] `cargo build` passes on `x86_64-linux`, `x86_64-macos`, `x86_64-windows` (Windows local ✅; Linux/macOS pending CI run).
- [ ] `cargo test` passes (empty test suite is acceptable at G1) (Windows local ✅; Linux/macOS pending CI run).
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

1. Open PR tagged `[S1]` targeting G1.
2. Confirm Linux and macOS matrix jobs pass in CI.
3. Mark S1 G1 closed and unblock S2 start.
