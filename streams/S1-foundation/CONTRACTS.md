# S1 Foundation — Contracts

S1 is the foundation layer. It does not consume contracts from other streams. It **publishes** the workspace structure, CI conventions, and tooling contracts that all other streams rely on.

---

## Contracts owned by S1

| Contract | Type | Location | Status |
|----------|------|----------|--------|
| Cargo workspace layout | Convention | `Cargo.toml` (root) | ⬜ Not yet created |
| CI pipeline contract | Workflow | `.github/workflows/ci.yml` | ⬜ Not yet created |
| Rust edition + toolchain pin | File | `rust-toolchain.toml` | ⬜ Not yet created |
| CODEOWNERS | File | `CODEOWNERS` | ⬜ Not yet created |
| Integration branch convention | Convention | `CONTRIBUTING.md` | ⬜ See S8 |

---

## Contracts consumed by S1

None. S1 is the root.

---

## Interface published to other streams

All streams must:

- Add their crate(s) to the `Cargo.toml` `[workspace]` members list.
- Follow the CI pipeline contract: all crates must pass `fmt`, `clippy`, `test`.
- Use the pinned Rust toolchain from `rust-toolchain.toml`.
- Respect `CODEOWNERS` path ownership.
