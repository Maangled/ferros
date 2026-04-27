# S1 Foundation — Contracts

S1 is the foundation layer. It does not consume contracts from other streams. It **publishes** the workspace structure, CI conventions, and tooling contracts that all other streams rely on.

---

## Contracts owned by S1

| Contract | Type | Location | Status |
|----------|------|----------|--------|
| Cargo workspace layout | Convention | `Cargo.toml` (root) | ✅ Implemented |
| CI pipeline contract | Workflow | `.github/workflows/ci.yml` | ✅ Implemented |
| Release candidate bundle workflow | Workflow | `.github/workflows/release.yml` | ✅ Implemented |
| Rust edition + toolchain pin | File | `rust-toolchain.toml` | ✅ Implemented |
| CODEOWNERS | File | `CODEOWNERS` | ✅ Implemented |
| Integration branch convention | Workflow | `.github/workflows/integration.yml` | ✅ Implemented |

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
