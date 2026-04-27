# S4 Runtime / OS Core — Progress

Reverse-chronological. Append a dated entry at the top per session.

---

## 2026-04-26 — Narrow S4 support landed for the first broader local-only wrapper/API slice

- Refactored `crates/ferros-node/src/lib.rs` so the extracted local host-controller seam now supports a published local-only `LocalAgentApi` wrapper above CLI formatting instead of leaving the first broader slice only as docs.
- Kept the S4 support narrow: the landed slice still reuses the current reference-host bootstrap and local state path, and it does not publish remote transport, remote-control semantics, or broader S4 restart/reload guarantees.
- Focused validation passed with `cargo test -p ferros-node local_agent_api_`, `cargo test -p ferros-node agent_cli_`, `cargo test -p ferros-node agent_read_rpc_`, and `cargo test -p ferros-node shell_listener_posts_json_rpc_`.

## 2026-04-26 — First internal local host-controller surface extracted above argv parsing

- Refactored the current local `ferros agent` lifecycle/log path in `crates/ferros-node/src/lib.rs` so a dedicated internal `LocalAgentController` now owns the state-path and runtime-loader behavior above argv parsing instead of leaving that controller logic flattened inside the CLI execution function.
- Kept the slice internal and local-only: the extracted surface is not a published broader lifecycle/write wrapper/API, not a remote-control contract, and not a broader S4 restart/reload publication.
- Focused validation passed with `cargo test -p ferros-node agent_cli_`, `cargo test -p ferros-node agent_read_rpc_`, and `cargo test -p ferros-node shell_listener_posts_json_rpc_`.

## 2026-04-26 — `ferros-core` thumb-target `no_std` proof wired into CI

- Recorded the honest embedded-target proof as local `cargo check -p ferros-core --target thumbv7em-none-eabi --no-default-features`.
- Updated GitHub Actions to install `thumbv7em-none-eabi` on the Ubuntu test lane and run the same `ferros-core` check there.
- Truth-synced the S4-owned docs to say CI is configured to enforce that thumb-target check, without claiming a remote workflow pass.

## 2026-04-24 — Localhost shell host seam hardened with listener-level smoke coverage

- Extracted a bounded listener loop in `crates/ferros-node/src/lib.rs` so the localhost shell host can be exercised through a real `TcpListener` in focused tests instead of only through route-level units.
- Added real socket-smoke coverage for `GET /` and `POST /rpc`, proving the current shell host serves the embedded shell HTML and returns a live `agent.list` JSON-RPC response through the same HTTP parser and writer used by `ferros-node shell`.
- Kept the slice shallow and read-first: no new JSON/RPC methods, no transport expansion, no persistence changes, and no `ferros-hub` semantics.
- Focused validation passed with `cargo test -p ferros-node shell_`.

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
