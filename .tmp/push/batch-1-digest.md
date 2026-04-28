# PUSH Batch 1 Digest

- Wave: `WAVE-2026-04-28-01`
- Status: landed
- Seed: top-level recursive G4 / D1 breadth push
- Shape: 7 landed owner lanes plus this bookkeeping closeout lane

## Lane Reports

| Lane | Surface | Files | LOC | Notes |
|---|---|---:|---:|---|
| L1 | S4 runtime scaffold | 2 | +109 / -0 | Added `LocalRunwayState` and transition checks in `ferros-runtime`. |
| L2 | S3 node runway summary | 1 | +466 / -10 | Added `LocalRunwaySummary`, `/runway-summary.json`, and focused tests in `ferros-node`. |
| L3 | S2 local consent snapshot | 1 | +129 / -3 | Added Rust-local `LocalConsentSnapshot` above the frozen v0 contracts. |
| L4 | S7 bridge seam catalog | 1 | +39 / -0 | Added a runway-only seam inventory for HA bridge follow-up. |
| L5 | S6 local push envelope | 2 | +248 / -2 | Added `local-push-audit-envelope.schema.json` plus ferros-data boundary constants. |
| L6 | S1 xtask burst helper | 1 | +40 / -7 | Added `cargo xtask burst` for focused push validation output. |
| L7 | S5 shell + harness runway route | 2 | +108 / -2 | Wired the shell and acceptance harness to the new local-only runway summary route. |
| L8 | S8 bookkeeping | 1 | pending | Batch digest + manifest closeout under `.tmp/push/`. |

## Stubs Left

- `crates/ferros-runtime/src/local_runway.rs` is not yet consumed by `ferros-node` or the shell.
- `crates/ferros-node/src/lib.rs` serves runway summary from the default local profile path only.
- `schemas/local-push-audit-envelope.schema.json` has no harness or codegen consumer yet.

## Validation

- `cargo test -p ferros-runtime`
- `cargo test -p ferros-profile local_consent_snapshot_`
- `cargo test -p ferros-data`
- `cargo check -p xtask`
- `cargo xtask burst`
- `cargo test -p ferros-node runway_summary`
- `cargo test -p ferros-node shell_listener_posts_json_rpc_`
- `get_errors` clean on touched Rust, HTML, schema, and markdown files

## Failures Surfaced

- No compile or test failures remained after validation.
- The initial shared `target` directory lock on Windows was avoided by pinning `CARGO_TARGET_DIR=target/copilot-push` for focused runs.

## Feed-Forward Hooks

- Wire `LocalRunwayState` into `ferros-node` summary generation rather than leaving it crate-local.
- Let the shell pass an explicit profile path or selector into the runway summary read.
- Emit `.tmp/push/` digests through a real S1/S6 local writer once the envelope consumer lands.