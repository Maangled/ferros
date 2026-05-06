# 2026-05-06 FRS-subcore-C3-W1 Truth-Sync Delta (L4)

Authority: docs/orchestration/AUTHORITY-MAP.md
Date: 2026-05-06
Run ID: FRS-subcore-20260506-C3-W1
Run profile: subcore-runtime
Track: code

## Lanes executed and no-op rationale

- L1: documented hosted adapter post-failure semantics in `streams/S4-runtime/CONTRACTS.md` so downstream readers do not infer rollback guarantees.
- L2: no-op by rule. No additional scaffold contract was added because one architecture-only lineage contract already landed in the prior micro-cycle and no bounded gap required a second increment this cycle.
- L3: re-ran the focused validation bundle for runtime, scaffold, and foundation guard surfaces.
- L4: serialized this truth-sync delta and appended one matching newest-first entry to `docs/orchestration/WAVE-RUN-LOG.md`.
- L5: kept claim language explicit about hosted rehearsal versus pre-native boundaries.

## Adapter post-failure semantics clarified this cycle

The S4 owning contract doc now states the bounded hosted behavior for `LocalRunwayAdapter` composition:

- Transition failure (`LocalRunwayAdapterError::Transition`): state does not advance; submit and route are not attempted.
- Executor failure (`LocalRunwayAdapterError::Executor`): state can already be advanced by transition; route is not attempted.
- Bus failure (`LocalRunwayAdapterError::Bus`): state can already be advanced and executor submit can already be persisted; envelope route did not complete.

This seam remains non-transactional. No atomic commit, rollback, or exactly-once guarantee is claimed.

## Architecture-only scaffold contract decision

No new scaffold contract was added in this cycle.

Decision rationale:

- Existing architecture-only scaffold vocabulary (`KernelHandoffContract` and `BootArtifactLineageContract`) is currently sufficient for bounded subcore claims.
- Adding another scaffold contract in this pass would have widened scope without closing a concrete defect.

## Validation evidence

- `cargo test -p ferros-runtime --test boundaries`: pass (10 passed, 0 failed)
- `cargo test -p ferros-runtime --test x86_64_subcore_smoke`: pass (3 passed, 0 failed)
- `cargo test -p ferros-x86_64-scaffold`: pass (7 passed, 0 failed)
- `cargo check -p ferros-x86_64-scaffold --no-default-features`: pass
- `cargo test -p ferros-core foundation_surface_`: pass (4 passed, 0 failed)

## Claims added

- S4 owning docs now explicitly define hosted adapter partial-progress semantics on downstream failures.
- Runtime and scaffold focused evidence remains green under the bounded subcore rehearsal seam.

## Claims explicitly not added

- No bootloader success claim.
- No kernel boot success claim.
- No QEMU boot proof claim.
- No hardware bring-up claim.
- No gate-closure claim.
- No native-runtime proof claim.

## Residual pre-native gaps

- Hosted seam evidence remains pre-native and does not prove runtime execution on hardware, boot flow completion, or QEMU execution.
- Adapter semantics are now explicit but still non-transactional; no rollback or exactly-once delivery contract is published.
- No new gate movement or execution-proof surface was introduced.

## Next lane seeds

- Seed A: if consumers require stronger guarantees, define a bounded contract for adapter compensation/retry semantics without implying execution proof.
- Seed B: only add another architecture-only scaffold contract when a concrete vocabulary gap appears in downstream seams.
- Seed C: keep future truth-sync notes explicit about hosted evidence versus pre-native boundaries.

## Preflight continuity note

All five authority version markers matched the expected 2026-05-03 lock during this run. No active-session mismatch override was required in this cycle.
