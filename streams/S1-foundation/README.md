# S1 — Foundation

**Stream:** S1  
**Status:** 🔴 Not started  
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

- [ ] `cargo build` passes on `x86_64-linux`, `x86_64-macos`, `x86_64-windows`.
- [ ] `cargo test` passes (empty test suite is acceptable at G1).
- [ ] `cargo fmt --check` passes.
- [ ] `cargo clippy -- -D warnings` passes.
- [ ] CI workflow runs on every PR.
- [ ] `ferros-blueprint.html` lives at `/site/index.html`.
- [ ] `README.md` reflects actual project state.
- [ ] `CODEOWNERS` exists with per-stream ownership entries.

---

## Likely crates / files

| Path | Role |
|------|------|
| `Cargo.toml` | Workspace root |
| `rust-toolchain.toml` | Rust edition pin |
| `.github/workflows/ci.yml` | CI pipeline |
| `xtask/` | Build task crate |
| `tools/sh/` | Bash helper scripts |
| `tools/ps/` | PowerShell helper scripts |
| `site/index.html` | `ferros-blueprint.html` moved here |
| `CODEOWNERS` | Per-stream ownership |

---

## Immediate next steps

1. Add `Cargo.toml` workspace root with an empty `[workspace]` members list.
2. Add `rust-toolchain.toml`.
3. Add `.github/workflows/ci.yml` with fmt/clippy/test jobs.
4. Move `ferros-blueprint.html` → `site/index.html`.
5. Update `README.md` to reflect current state and link to new docs.
6. Add `CODEOWNERS`.
7. Open PR tagged `[S1]`, target G1 gate.
