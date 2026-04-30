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
| `ferros-core` `std`/`no_std` boundary | Cargo feature boundary | `crates/ferros-core/Cargo.toml`, `crates/ferros-core/src/lib.rs` | 🟡 `std` is still the default feature; local `thumbv7em-none-eabi` + `--no-default-features` proof is recorded, and CI is configured to enforce the same check |
| `Executor` trait | Rust trait | `crates/ferros-runtime/src/executor.rs` | ✅ Created |
| In-process bus protocol | Rust trait | `crates/ferros-runtime/src/bus.rs` | ✅ Created |
| `InMemoryExecutor` type | Rust type | `crates/ferros-runtime/src/executor.rs` | 🟡 Convergence implementation created |
| `InMemoryMessageBus` type | Rust type | `crates/ferros-runtime/src/bus.rs` | 🟡 Convergence implementation created |
| `ferros-node demo` host path | Rust binary + library | `crates/ferros-node/src/lib.rs` | 🟡 Convergence implementation created |
| `/runway-summary(.json)` local runway surface | Localhost read-only route + serializable summary payload | `crates/ferros-node/src/lib.rs` | 🟡 Existing local-only runway surface; now additively carries optional `hubRestart`, `hubOnrampProposal`, and `hubOnrampDecisionReceipt` context from the hub summary seam |

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

## S7 runway classification

This is an S4-owned classification of the landed S7 seam brief. It does not turn the current runtime helpers into a finished hub contract, and it does not authorize `crates/ferros-hub/` scaffolding.

| Surface | S4-owned classification |
|---------|-------------------------|
| `CapabilityRequest`, `CapabilityGrantView`, `PolicyEngine::evaluate`, `DenyByDefaultPolicy`, `PolicyDecision`, `PolicyDenialReason` | Sufficient now for S7 runway planning as the authoritative policy and deny-reason boundary. S7 should consume these S4 types and reasons instead of inventing any hub-local grant or deny model. |
| current `ferros-runtime` executor plus in-process bus boundary | Sufficient now only to name the current runtime container S7 expects a future hub to wrap. It is not, by itself, a published restart or re-registration contract. |
| nearest reload helpers `runtime_with_state(state_path)`, `CliState::load(path)`, and `LocalProfileStore::load_local_profile(path)` | A narrow docs-only reload boundary is now published for S7 runway planning: validated local profile/grant reload plus fixed reference-runtime state replay are described here, while a broader hub-facing restart API and durable re-registration contract remain unpublished. |
| existing `/runway-summary(.json)` seam with optional `hubRestart`, `hubOnrampProposal`, and `hubOnrampDecisionReceipt` children | Published now as local-only, read-only runway context plus proposal-and-decision rehearsal observation for downstream shell/operator observation. It is sufficient for bounded restart-state display and pending-consent proposed-material plus recorded decision-rehearsal display on the current localhost path, but it is not a durable hub restart API, an accept/reject flow, canonical mutation, power-cycle evidence, remote transport, or a gate-closing surface. |

- The S4 policy seam is already strong enough for S7 runway planning.
- The published reload boundary and the additive `hubRestart` runway-summary child are intentionally narrow: validated local profile/grant reload, fixed reference-runtime state replay, and read-only bounded restart observation are in bounds today, while broader durable hub-facing restart and re-registration semantics remain unpublished.

### Narrow published reload boundary

This subsection is docs-only and S4-owned. It records the exact current helper behavior S7 may rely on now without turning the helpers into a finished hub contract.

| Helper | What S7 may rely on now | What remains unpublished |
|--------|-------------------------|--------------------------|
| `runtime_with_state(state_path)` in `crates/ferros-node/src/lib.rs` | It calls `CliState::load(state_path)`, then `build_reference_runtime()`, and replays persisted state only onto that fixed reference runtime. `Registered` leaves the default registration set as-is; `Running` and `Stopped` are replayed; unsupported persisted statuses error. It does not discover arbitrary agents. | This is still a node-local helper over the fixed reference runtime, not a durable hub-facing restart or re-registration API. |
| `CliState::load(path)` in `crates/ferros-node/src/lib.rs` | It reads the exact path passed in, returns the default empty state when the file is missing, and accepts only persisted `status` and `log` lines. Malformed lines, unknown entry kinds, and unsupported statuses error. | This does not publish a hub-owned file format, storage contract, or restart choreography beyond the current strict node-local parser. |
| `LocalProfileStore::load_local_profile(path)` in `crates/ferros-profile/src/lib.rs` | It returns `LocalProfileState::new(load_profile, load_key_pair, load_signed_grants)`. Missing signed-grants state becomes an empty grant list, and `LocalProfileState::new` validates that each signed grant verifies, matches the local profile id, matches the local signer public key, and does not duplicate a capability. | This publishes validated local profile/grant reload only. It does not publish durable hub restart, pairing, or re-registration semantics. |

---

## Note on `no_std`

`ferros-core` currently exposes `std` as its default feature in `Cargo.toml` and uses `#![cfg_attr(not(feature = "std"), no_std)]` plus `extern crate alloc` in `src/lib.rs` to keep the current core surface portable. The honest proof now recorded for this slice is local `cargo check -p ferros-core --no-default-features`, local `cargo check -p ferros-core --target thumbv7em-none-eabi --no-default-features`, and CI configured to run that same thumb-target check. This does not claim a remote workflow pass.
