# 2026-05-09 FRS-subcore-C3-W4 Truth-Sync Delta (L4)

Authority: docs/orchestration/AUTHORITY-MAP.md
Date: 2026-05-09
Run ID: FRS-subcore-20260506-C3-W4
Run profile: subcore-runtime
Track: code

## Preflight version-lock result

- docs/orchestration/AUTHORITY-MAP.md marker matched expected `Last updated: 2026-05-09`.
- docs/orchestration/ORCHESTRATION-POLICY.md was confirmed as active policy source (not LOCAL-DRIVER.md or BATCH-MODE.md).
- docs/orchestration/ORCHESTRATION-EXECUTION.md was confirmed as active workflow source.
- docs/orchestration/ORCHESTRATION-AGENTS.md role map was confirmed current.
- docs/orchestration/QUEUE-SURFACES.md confirms code-track queue source as docs/orchestration/WAVE-QUEUE.md.
- authority_ack: none required.

## Lane outcomes

- Lane A continuity: landed in crates/ferros-runtime/tests/boundaries.rs by extending explicit retry evidence to cover non-duplication of executor submission under repeated transient route failures.
- Lane B contract-width: landed in crates/ferros-runtime/tests/boundaries.rs and streams/S4-runtime/CONTRACTS.md with narrow hosted recovery vocabulary (`recoverable` vs `terminal`) and explicit caller-owned retry policy.
- Lane C contract-width: landed in crates/ferros-x86_64-scaffold/src/lib.rs with architecture-only vocabulary separating hosted rehearsal artifacts from future native observation artifacts.
- Lane D evidence-hardening: landed in crates/ferros-runtime/tests/x86_64_subcore_smoke.rs by adding transient route failure followed by explicit caller retry rehearsal.
- Lane E evidence-hardening: this truth-sync closeout plus queue/run-log bookkeeping serialization.

## Hosted evidence now proven

- In hosted seams, explicit caller retry after transient route failure does not re-advance state and does not duplicate executor submission, as shown by passing boundaries tests.
- Hosted bus failure classification vocabulary is defined as recoverable versus terminal in boundaries.rs and mirrored in S4 CONTRACTS.
- x86_64 hosted smoke rehearsal now exercises transient route failure followed by explicit caller retry and passes.

## Scaffold-only vocabulary now present

- ferros-x86_64-scaffold exposes architecture-only vocabulary separating hosted rehearsal artifacts from future native observation artifacts.
- This separation remains vocabulary-only and does not claim native execution.

## Explicitly unproven surface list (ADR-025 ceiling; verbatim)

- Native-runtime execution proof of any kind.
- FERROS-native OS claim for any architecture.
- Bootloader, kernel, or QEMU execution proof.
- Hardware proof or physical-device evidence.
- Home Assistant proof. Gate closure. D1 or G4 movement.
- Transactional, rollback, or exactly-once delivery claim.
- Any claim that the adapter provides automatic retry.
- Any claim from a test not explicitly run in this wave.
- Background-autonomy or always-running S9 claims.

## Validation outputs

- Lane A and Lane B
  - cargo test -p ferros-runtime --test boundaries: pass (15 passed, 0 failed, 0 ignored, 0 measured, 0 filtered out)
- Lane D
  - cargo test -p ferros-runtime --test x86_64_subcore_smoke: pass (4 passed, 0 failed, 0 ignored, 0 measured, 0 filtered out)
- Lane C
  - cargo test -p ferros-x86_64-scaffold: pass (9 passed, 0 failed, 0 ignored, 0 measured, 0 filtered out)
  - cargo check -p ferros-x86_64-scaffold --no-default-features: pass
- Full-suite confirmation bundle
  - cargo test -p ferros-runtime --test boundaries: pass (15 passed, 0 failed)
  - cargo test -p ferros-runtime --test x86_64_subcore_smoke: pass (4 passed, 0 failed)
  - cargo test -p ferros-x86_64-scaffold: pass (9 passed, 0 failed)
  - cargo check -p ferros-x86_64-scaffold --no-default-features: pass
  - cargo test -p ferros-core foundation_surface_: pass (4 passed, 0 failed)

## Stop-condition and authority status

- Stop condition: none fired.
- Gatekeeper block: continue.
- Authority interruption: none.

## Claims added

- Hosted retry non-duplication evidence now covers repeated transient route failures with explicit caller retry.
- Hosted recovery vocabulary now explicitly distinguishes recoverable versus terminal route outcomes across boundary test evidence and S4 contract wording.
- x86_64 hosted smoke now includes transient-failure-then-retry rehearsal aligned with the boundary suite story.
- x86_64 scaffold vocabulary now separates hosted rehearsal artifacts from future native observation artifacts as architecture-only terms.

## Claims explicitly not added

- No native-runtime execution proof.
- No FERROS-native OS claim.
- No bootloader, kernel, or QEMU execution proof.
- No hardware proof or physical-device evidence.
- No Home Assistant proof.
- No gate closure, D1 movement, or G4 movement.
- No transactional, rollback, or exactly-once delivery claim.
- No automatic retry claim.
- No background-autonomy or always-running S9 claim.

## Next seed candidates (W5, anti-narrowed)

Attestation: sourced from FERROS SubCore Lane Architect Agent output, anti-narrowed, and ADR-025-safe.

- Seed A [continuity] anchors runtime boundaries and adapter seam (crates/ferros-runtime/tests/boundaries.rs, crates/ferros-runtime/src/local_runway.rs): extend explicit-retry evidence for repeated transient failures with no implicit replay and no duplicate delivery. Validation: cargo test -p ferros-runtime --test boundaries.
- Seed B [continuity] anchors runtime smoke seam (crates/ferros-runtime/tests/x86_64_subcore_smoke.rs, crates/ferros-runtime/src/local_runway.rs): broaden smoke rehearsal for transient-then-retry success while preserving caller-owned recovery framing. Validation: cargo test -p ferros-runtime --test x86_64_subcore_smoke.
- Seed C [contract-width] anchors runtime boundaries plus S4 contracts (crates/ferros-runtime/tests/boundaries.rs, streams/S4-runtime/CONTRACTS.md): lock recoverable versus terminal vocabulary alignment without introducing automatic retry language. Validation: cargo test -p ferros-runtime --test boundaries.
- Seed D [contract-width] anchors scaffold contract vocabulary (crates/ferros-x86_64-scaffold/src/lib.rs): add one bounded architecture-only vocabulary increment if a concrete gap appears, preserving non-claim scope. Validation: cargo test -p ferros-x86_64-scaffold; cargo check -p ferros-x86_64-scaffold --no-default-features.
- Seed E [evidence-hardening] anchors cross-family matrix (runtime boundaries and smoke, scaffold, foundation surface): rerun and publish full bounded proof matrix to prevent retry-only narrowing. Validation: cargo test -p ferros-runtime --test boundaries; cargo test -p ferros-runtime --test x86_64_subcore_smoke; cargo test -p ferros-x86_64-scaffold; cargo check -p ferros-x86_64-scaffold --no-default-features; cargo test -p ferros-core foundation_surface_.
