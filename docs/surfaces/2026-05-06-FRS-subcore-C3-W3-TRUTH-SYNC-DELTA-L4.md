# 2026-05-06 FRS-subcore-C3-W3 Truth-Sync Delta (L4)

Authority: docs/orchestration/AUTHORITY-MAP.md
Date: 2026-05-06
Run ID: FRS-subcore-20260506-C3-W3
Run profile: subcore-runtime
Track: code

## Lane outcomes

- Seed A continuity: landed. Added multi-step transient failure rehearsal with caller-driven retry success in `crates/ferros-runtime/tests/boundaries.rs`.
- Seed B contract-width: landed. Added focused classification evidence that transient and terminal bus failures remain distinct caller-visible outcomes in `crates/ferros-runtime/tests/boundaries.rs`.
- Seed C evidence-hardening: landed. The same hosted boundary suite now proves no delivery occurs unless caller re-invokes after transient failure.
- Seed D contract-width: no-op by rule. No scaffold code increment was required because existing architecture-only scaffold vocabulary remained sufficient and no concrete compatibility defect emerged in this cycle.
- Seed E evidence-hardening: landed as truth-sync. This delta explicitly maps hosted boundary evidence to allowed claims and preserved non-claims.
- Seed F settlement: landed. This delta and one new newest-first run-log entry were written after validation settled.

## Disputes and resolutions

- Dispute: whether W3 required a new scaffold contract to satisfy contract-width coverage.
- Resolution: no-op by rule. Contract-width was extended at the hosted runtime boundary through caller-visible failure classification without widening scaffold scope.
- Dispute: whether multi-step transient failure coverage could drift into implicit retry or transactional language.
- Resolution: evidence and S4 owning docs stayed explicit that retry remains caller-owned and non-transactional.

## Hosted boundary evidence to allowed claims mapping

- Allowed claims from this cycle:
  - hosted runtime boundary now proves explicit caller-managed retry after transient route failure
  - hosted runtime boundary now preserves transient versus terminal route-failure classification for caller policy
  - hosted runtime boundary still prevents envelope delivery unless caller re-invokes after transient failure
- Explicitly not allowed from this cycle:
  - no transactional guarantee
  - no rollback guarantee
  - no exactly-once delivery guarantee
  - no bootloader, kernel, QEMU, hardware, gate, or native-runtime proof claim

## Validation evidence

- `cargo test -p ferros-runtime --test boundaries`: pass (13 passed, 0 failed)
- `cargo test -p ferros-runtime --test x86_64_subcore_smoke`: pass (3 passed, 0 failed)
- `cargo test -p ferros-x86_64-scaffold`: pass (7 passed, 0 failed)
- `cargo check -p ferros-x86_64-scaffold --no-default-features`: pass
- `cargo test -p ferros-core foundation_surface_`: pass (4 passed, 0 failed)

## Claims added

- Hosted compensation and retry evidence now covers repeated transient route failure with explicit caller-driven recovery.
- Hosted runtime evidence now proves transient and terminal bus-route failures stay distinct caller-visible outcomes.
- S4 owning docs now explicitly say repeated transient failures still require caller re-invocation before any delivery occurs.

## Claims explicitly not added

- No bootloader success claim.
- No kernel boot success claim.
- No QEMU proof claim.
- No hardware bring-up claim.
- No gate-closure claim.
- No native-runtime proof claim.

## Residual pre-native gaps

- Evidence remains hosted and pre-native only.
- Adapter semantics remain non-transactional and caller-owned for compensation or retry.
- No scaffold/runtime compatibility defect required a new architecture-only contract this cycle.
- No boot, kernel, QEMU, hardware, or gate evidence moved.

## Next lane seeds

Attestation: the following seeds were sourced from FERROS SubCore Lane Architect Agent output and preserved as anti-narrowed breadth coverage.

- Seed A [continuity]: extend the hosted adapter boundary suite to prove explicit retry never re-advances state and never duplicates executor submission after transient route failure.
- Seed B [contract-width]: define a narrow hosted runtime classification contract for recoverable versus terminal route outcomes at the bus or adapter edge so downstream callers do not invent competing policy vocabulary.
- Seed C [contract-width]: add architecture-only scaffold vocabulary that separates hosted rehearsal evidence artifacts from future native observation artifacts without implying execution proof.
- Seed D [evidence-hardening]: add an x86_64 hosted smoke rehearsal that exercises transient route failure followed by explicit caller retry so smoke evidence matches the boundary suite’s caller-owned recovery story.
- Seed E [evidence-hardening]: write a compact truth-sync closeout that records what is now proven in hosted runtime seams, what remains scaffold-only vocabulary, and what is still explicitly unproven.

## Preflight continuity note

All five authority markers matched the expected 2026-05-03 lock. No active-session mismatch override was required in this cycle.
