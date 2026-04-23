# S4 Runtime / OS Core — Contracts

---

## Contracts owned by S4

| Contract | Type | Location | Status |
|----------|------|----------|--------|
| `Capability` type | Rust type | `crates/ferros-core/src/capability.rs` | ✅ Created |
| `CapabilityRequest` type | Rust type | `crates/ferros-core/src/capability.rs` | ✅ Created |
| `CapabilityGrantView` trait | Rust trait | `crates/ferros-core/src/capability.rs` | ✅ Created |
| `PolicyEngine` trait | Rust trait | `crates/ferros-core/src/capability.rs` | ✅ Created |
| `MessageEnvelope` type | Rust type | `crates/ferros-core/src/message.rs` | ⬜ Not yet created |
| `Executor` trait | Rust trait | `crates/ferros-runtime/src/executor.rs` | ⬜ Not yet created |
| In-process bus protocol | Rust trait | `crates/ferros-runtime/src/bus.rs` | ⬜ Not yet created |

---

## Contracts consumed by S4

| Contract | Source | Purpose |
|----------|--------|---------|
| `CapabilityGrant` | S2 | Policy engine evaluates grants; current alignment is `profile_id` + `capability` via `CapabilityGrantView` until direct convergence |
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

`ferros-core` must expose a `no_std` feature flag so it can eventually compile for embedded targets (`thumbv7em-none-eabi`). This constraint should be enforced by a CI job that compiles `ferros-core` with `--no-default-features` and `--target thumbv7em-none-eabi` once the baseline is stable.
