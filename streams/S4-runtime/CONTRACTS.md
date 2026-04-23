# S4 Runtime / OS Core — Contracts

---

## Contracts owned by S4

| Contract | Type | Location | Status |
|----------|------|----------|--------|
| `Capability` type | Rust type | `crates/ferros-core/src/capability.rs` | ✅ Created |
| `CapabilityRequest` type | Rust type | `crates/ferros-core/src/capability.rs` | ✅ Created |
| `CapabilityGrantView` trait | Rust trait | `crates/ferros-core/src/capability.rs` | ✅ Created |
| `PolicyEngine` trait | Rust trait | `crates/ferros-core/src/capability.rs` | ✅ Created |
| `MessageEnvelope` type | Rust type | `crates/ferros-core/src/message.rs` | ✅ Created |
| `ferros-core` `std`/`no_std` boundary | Cargo feature boundary | `crates/ferros-core/Cargo.toml`, `crates/ferros-core/src/lib.rs` | 🟡 `std` is still the default feature; host `--no-default-features` compile slice is validated, but embedded-target / CI validation remains open |
| `Executor` trait | Rust trait | `crates/ferros-runtime/src/executor.rs` | ✅ Created |
| In-process bus protocol | Rust trait | `crates/ferros-runtime/src/bus.rs` | ✅ Created |
| `InMemoryExecutor` type | Rust type | `crates/ferros-runtime/src/executor.rs` | 🟡 Convergence implementation created |
| `InMemoryMessageBus` type | Rust type | `crates/ferros-runtime/src/bus.rs` | 🟡 Convergence implementation created |
| `ferros-node demo` host path | Rust binary + library | `crates/ferros-node/src/lib.rs` | 🟡 Convergence implementation created |

---

## Contracts consumed by S4

| Contract | Source | Purpose |
|----------|--------|---------|
| `CapabilityGrant` | S2 | Policy engine now evaluates the current concrete type directly through `CapabilityGrantView`; G2 freeze still owns the final contract shape |
| `Agent` trait | S3 | Runtime hosts agents via this interface |
| Cargo workspace | S1 | Build and CI |

---

## Downstream consumers

| Stream | What it consumes |
|--------|-----------------|
| S3 Agent Center | `Executor` trait — agents run inside this executor |
| S7 Hub | `ferros-runtime` — hub wraps the runtime for edge deployment |

---

## Note on `no_std`

`ferros-core` currently exposes `std` as its default feature in `Cargo.toml` and uses `#![cfg_attr(not(feature = "std"), no_std)]` plus `extern crate alloc` in `src/lib.rs` to keep the current core surface portable. The honest validation slice for this wave is host-side `cargo check -p ferros-core --no-default-features`. Embedded-target validation (`--target thumbv7em-none-eabi`) and CI enforcement remain open work.
