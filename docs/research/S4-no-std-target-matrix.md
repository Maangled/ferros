# S4 Research Note — no_std Target Matrix

**Date:** 2026-04-27  
**Owning stream:** S4 primary; S7 consumer awareness  
**Output feeds:** D1 firmware spike planning (HARDWARE-2026-04-27-01, HARDWARE-2026-04-27-02)  
**Status:** Research note — catalogs existing CI evidence. Does not claim new CI runs.

---

## Purpose

This note catalogs the current state of `ferros-core` and `ferros-runtime` cross-compilation targets: which targets build clean today, which require feature flags, and what the D1 device target will require. It is planning input for the D1 firmware spike, not a S1 reopening.

---

## Current Target Matrix

### ferros-core (`crates/ferros-core/`)

`ferros-core` is designed for `no_std` compatibility. It uses `#![cfg_attr(not(feature = "std"), no_std)]` at the crate root and declares `extern crate alloc;` for heap-allocated types (e.g., `String`, `Vec`).

**Feature flags:**
- `default = ["std"]` — enables std features (required for std-linked host builds)
- `std` — disabled for bare-metal / embedded targets

| Target | Feature flags | Build status | CI enforced? |
|---|---|---|---|
| `x86_64-unknown-linux-gnu` (native, default) | `--features std` (default) | ✅ Clean | Yes — `cargo build --workspace` on ubuntu-latest |
| `x86_64-apple-darwin` / `aarch64-apple-darwin` (native) | `--features std` (default) | ✅ Clean | Yes — `cargo build --workspace` on macos-latest |
| `x86_64-pc-windows-msvc` (native) | `--features std` (default) | ✅ Clean | Yes — `cargo build --workspace` on windows-latest |
| `x86_64-unknown-linux-gnu` (no\_std check) | `--no-default-features` | ✅ Clean | Yes — `cargo check -p ferros-core --no-default-features` on ubuntu-latest |
| `thumbv7em-none-eabi` (Cortex-M4/M7, no\_std) | `--no-default-features` | ✅ Clean | Yes — `cargo check -p ferros-core --target thumbv7em-none-eabi --no-default-features` on ubuntu-latest |
| `aarch64-unknown-linux-gnu` (Pi / edge) | `--features std` | ✅ Reasonable expectation | Not CI-enforced today; requires cross-compilation toolchain |
| `wasm32-unknown-unknown` (WASM, Phase C) | `--no-default-features` | 🟡 Not yet validated | Not CI-enforced; Phase C S5 goal |

**Notes:**
- `thumbv7em-none-eabi` is Cortex-M4/M7 (ARMv7E-M with hardware float unit). This is the embedded target installed via `rustup target add thumbv7em-none-eabi` in CI (ubuntu-latest only).
- `ferros-core` has no third-party runtime dependencies (no `Cargo.toml` `[dependencies]` beyond dev-dependencies). Dev-dependency: `proptest = "=1.6.0"` (std, test-only).
- The `alloc` crate is used for `String` in `Capability`, `CapabilityRequest`, and `DenyByDefaultPolicy` implementations.

---

### ferros-runtime (`crates/ferros-runtime/`)

`ferros-runtime` uses `std` (it does not declare `no_std`). Its in-memory executor and message bus rely on `std::collections::VecDeque` and `std::collections::HashMap`.

| Target | Feature flags | Build status | Notes |
|---|---|---|---|
| `x86_64-unknown-linux-gnu` (native) | Default (std) | ✅ Clean | Host build only |
| `thumbv7em-none-eabi` | N/A — std dependency | ❌ Not buildable as-is | Would require a `no_std` rewrite of bus/executor |
| `aarch64-unknown-linux-gnu` | Default (std) | ✅ Reasonable expectation | Cross-compilation only; not CI-enforced |

**Notes:**
- For D1 on `aarch64`, `ferros-runtime` can be linked as-is since the Pi runs Linux (std available).
- For bare-metal (`thumbv7em-none-eabi`), only `ferros-core` is currently available. The runtime layer would need a `no_std` executor/bus implementation — this is a future S4 work item, not a D1 blocker.

---

### ferros-agents (`crates/ferros-agents/`)

`ferros-agents` uses `std` (serialization via `serde`, registry via `BTreeMap`). Compiles for std-linked targets only.

| Target | Build status | Notes |
|---|---|---|
| Host (Linux/macOS/Windows) | ✅ Clean | Full agent registry, RPC, and reference agents |
| `thumbv7em-none-eabi` | ❌ Not supported | serde + BTreeMap require alloc + std conventions |
| `aarch64-unknown-linux-gnu` | ✅ Reasonable expectation | Cross-compilation only |

---

### ferros-node (`crates/ferros-node/`)

The `ferros-node` binary (hosts the localhost shell and CLI) is std-linked and targets host platforms only.

| Target | Build status |
|---|---|
| `x86_64-unknown-linux-gnu` | ✅ CI-enforced |
| `aarch64-unknown-linux-gnu` | ✅ Expected (cross-compile required) |
| Bare-metal | ❌ Not applicable |

---

## D1 Device Target Requirements

### Primary target: x86_64 home server / laptop (Pack B)

The Pack B first bring-up plan uses `x86_64-unknown-linux-gnu`. This target:
- Requires no additional toolchain beyond `stable` Rust.
- Compiles `ferros-core`, `ferros-runtime`, `ferros-agents`, and `ferros-node` without cross-compilation.
- Is fully CI-enforced today.
- Is the recommended D1 target for firmware spike milestone 1 (Boot) and milestone 2 (Identify).

### Secondary target: aarch64 Pi / edge device

If the D1 device is a Raspberry Pi (`aarch64-unknown-linux-gnu`):
- Cross-compilation toolchain required: `rustup target add aarch64-unknown-linux-gnu` plus a linker (`aarch64-linux-gnu-gcc` or equivalent).
- All crates should cross-compile (all use std, which is available on Pi Linux).
- Not CI-enforced today; must be validated locally before D1.

---

## What Needs to Happen for D1

| Task | Target | CI-enforced? | Status |
|---|---|---|---|
| `cargo check -p ferros-core --no-default-features` | x86_64 | Yes | ✅ Done |
| `cargo check -p ferros-core --target thumbv7em-none-eabi --no-default-features` | Cortex-M | Yes (ubuntu) | ✅ Done |
| `cargo build --workspace` on host targets | x86_64, macOS, Windows | Yes | ✅ Done |
| `cargo build --target aarch64-unknown-linux-gnu` (ferros-node) | Pi | No | ⬜ Not yet validated |
| `cargo run --bin ferros -- demo` on target device | x86_64 | Yes (ubuntu CI) | ✅ Done in CI |

---

## Source Documents

- `.github/workflows/ci.yml` — CI job definitions (authoritative)
- `crates/ferros-core/Cargo.toml` — feature flags
- `rust-toolchain.toml` — toolchain pin (`stable`, `minimal`, `rustfmt` + `clippy`)
- `docs/hub/pack-b-bring-up-worksheet.md` — Pack B target definition
- `streams/S4-runtime/README.md` — S4 owned seams and invariants
