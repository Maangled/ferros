# S4 Runtime / OS Core — Progress

Reverse-chronological. Append a dated entry at the top per session.

---

## 2026-04-23 — S4 docs truth-synced to the current `ferros-core` boundary

- Updated the S4 stream docs to describe the current `ferros-core` boundary honestly: `std` remains the default feature, while `#![cfg_attr(not(feature = "std"), no_std)]` and `alloc` keep the core slice portable.
- Recorded this wave's narrow validation slice as host-side `cargo check -p ferros-core --no-default-features`, not as full embedded-target closure.
- Kept G3 explicitly open in S4-owned docs: property tests, host hardening beyond the convergence demo, and embedded-target / CI `no_std` validation remain outstanding.

## 2026-04-23 — `ferros-node demo` convergence path landed

- Added concrete in-memory runtime implementations for the `Executor` and `MessageBus` boundaries in `crates/ferros-runtime/`.
- Added `crates/ferros-node/` and a deterministic `demo` path that exercises the current S3 and S4 convergence surface.
- Verified the demo path against the current concrete S2 `CapabilityGrant` type, including a deny-by-default rejection.
- Kept the remaining S4 work focused on host hardening, property tests, and `no_std` readiness rather than widening runtime scope.

## 2026-04-23 — Runtime boundary slice published

- Added `crates/ferros-runtime/` to the workspace with FERROS-owned `Executor` and `MessageBus` traits only, keeping the boundary generic so S4 stays decoupled from S3's pre-G3 agent surface.
- Added `MessageEnvelope` to `crates/ferros-core/src/message.rs` with validated sender and recipient identifiers, capability, opaque payload bytes, and nonce fields.
- Added focused crate tests for the new core envelope surface and the runtime trait boundary, then updated S4-owned planning docs to reflect the published slice.

## 2026-04-23 — First ferros-core capability/policy slice landed

- Added `Capability`, `CapabilityRequest`, `CapabilityGrantView`, `PolicyDecision`, and `DenyByDefaultPolicy` to `crates/ferros-core/`.
- Kept the grant boundary FERROS-owned by aligning the adapter trait to S2's current minimal `CapabilityGrant { profile_id, capability }` shape without introducing a crate cycle.
- Added 10 focused grant/deny and validation tests to prove deny-by-default behavior before any runtime or message-bus work.

## 2026-04-21 — Stream scaffolded

- Stream directory and planning documents created.
- Blocked on G1 (S1 Foundation).
- Stub capability types can be sketched before G2; will be replaced by S2 types at G2.
- Policy engine design can begin independently.
