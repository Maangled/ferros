# 2026-05-06 FRS-subcore-C3-W2 Truth-Sync Delta (L4)

Authority: docs/orchestration/AUTHORITY-MAP.md
Date: 2026-05-06
Run ID: FRS-subcore-20260506-C3-W2
Run profile: subcore-runtime
Track: code

## Lanes executed and no-op rationale

- L1 evidence-hardening: added one focused compensation/retry boundary rehearsal in `crates/ferros-runtime/tests/boundaries.rs` proving explicit caller retry after transient bus failure.
- L2 contract-width: no-op by rule. No additional scaffold contract increment was added because existing architecture-only scaffold vocabulary remained sufficient and widening this cycle would have been opportunistic.
- L3 validation: re-ran required runtime, scaffold, and foundation guard bundle.
- L4 truth-sync: serialized this delta artifact and appended one new newest-first run-log settlement entry.
- L5 seed-governance: invoked FERROS SubCore Lane Architect Agent and used its anti-narrowed seed set with breadth tags.

## Disputes and resolutions

- Dispute: whether L2 should force another scaffold contract change to satisfy lane count symmetry.
- Resolution: kept L2 as no-op-by-rule with explicit rationale because no concrete architecture vocabulary gap required a second increment.
- Dispute: whether compensation/retry evidence should imply adapter-level automatic replay semantics.
- Resolution: preserved hosted non-transactional boundary language and added evidence that retry remains caller-owned.

## Subcore contract and evidence changes this cycle

- Added `local_runway_adapter_requires_explicit_retry_after_transient_bus_failure` in `crates/ferros-runtime/tests/boundaries.rs`.
- Refined S4 owning contract language in `streams/S4-runtime/CONTRACTS.md` to state that compensation/retry policy is caller-owned on the hosted seam.
- No scaffold code changes landed in this cycle.

## Validation evidence

- `cargo test -p ferros-runtime --test boundaries`: pass (11 passed, 0 failed)
- `cargo test -p ferros-runtime --test x86_64_subcore_smoke`: pass (3 passed, 0 failed)
- `cargo test -p ferros-x86_64-scaffold`: pass (7 passed, 0 failed)
- `cargo check -p ferros-x86_64-scaffold --no-default-features`: pass
- `cargo test -p ferros-core foundation_surface_`: pass (4 passed, 0 failed)

## Claims added

- Hosted compensation/retry boundary evidence now explicitly proves that transient bus-route failure requires an explicit caller retry step.
- S4 owning docs now explicitly bind compensation/retry semantics to caller-owned policy for the current hosted adapter seam.

## Claims explicitly not added

- No bootloader success claim.
- No kernel boot success claim.
- No QEMU proof claim.
- No hardware bring-up claim.
- No gate-closure claim.
- No native-runtime proof claim.

## Residual pre-native gaps

- Evidence remains host-side rehearsal and does not prove native execution or bring-up.
- Adapter semantics remain non-transactional; no rollback or exactly-once delivery guarantee is published.
- No new boot, kernel, QEMU, hardware, or gate-proof surface moved.

## Next lane seeds

Attestation: the following seeds were sourced from FERROS SubCore Lane Architect Agent output and preserved as anti-narrowed breadth coverage.

- Seed A [continuity]: extend hosted bus-boundary rehearsal with multi-step transient failure followed by caller-driven retry success, asserting no implicit runtime retry.
- Seed B [contract-width]: add a narrow failure-classification contract test that locks transient versus terminal bus-error handling expectations without architecture changes.
- Seed C [evidence-hardening]: add a negative rehearsal test proving no side-effect replay occurs unless caller re-invokes after transient failure.
- Seed D [contract-width]: add a small architecture-only scaffold/runtime seam check validating current x86_64 scaffold error-shape compatibility expected by hosted boundary tests.
- Seed E [evidence-hardening]: add a concise truth-sync mapping note from hosted boundary tests to allowed claims and explicit non-claims to reduce claim drift.

## Preflight continuity note

All five authority markers matched the expected 2026-05-03 lock in this active stream session. No non-blocking preflight mismatch override was required for this cycle.
