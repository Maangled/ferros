# DOC-BATCH-2026-05-03-X86-FERROS-SUBCORE-01

Status: stop-clean closeout
Segment: `X86-FERROS-SUBCORE-01`
Track: code
Date: 2026-05-03

## Segment Summary
This user-directed code-track serial run executed the first x86_64 FERROS-root subcore seed batch under the accepted ADR-025 framework. The batch landed a coordination lock, four research notes, one host-side `ferros-runtime` subcore example, focused `ferros-runtime` subcore tests, focused `ferros-core` foundation-surface tests, and the closing truth-sync update. The batch stayed within a host-side and architecture-only claim ceiling and did not claim boot, kernel, QEMU, hardware, D1, G4, or FERROS-native OS success.

## Completed Lanes
- `83` coordination lock and lane architect packet
- `84` x86_64 FERROS UEFI boot-path note
- `85` x86_64 FERROS kernel and privilege handoff note
- `86` runtime port plan and host-side subcore example
- `87` runtime subcore boundary tests
- `88` portable foundation-surface tests
- `89` QEMU-OVMF smoke plan
- `90` serial truth-sync closeout

## Blocked Lanes
- None inside this batch. The next implementation edge is future explicit queue work, not an unresolved blocker inside the current seed batch.

## Evidence Produced
- `docs/orchestration/ADR-025-X86-FERROS-SUBCORE-01.md`
- `docs/research/S1-x86_64-ferros-uefi-boot-path.md`
- `docs/research/S1-x86_64-ferros-kernel-privilege-model.md`
- `docs/research/S4-x86_64-ferros-runtime-port-plan.md`
- `docs/research/S1-x86_64-qemu-ovmf-smoke-plan.md`
- `crates/ferros-runtime/examples/x86_64_subcore_smoke.rs`
- `crates/ferros-runtime/tests/x86_64_subcore_smoke.rs`
- `crates/ferros-core/tests/foundation_surface.rs`

## Claims Added
- The x86_64 FERROS-root subcore lanes now have a concrete first-batch coordination surface tied to real repo seams.
- The repo now contains a runnable host-side `ferros-runtime` subcore example for lifecycle, executor, and message-bus composition.
- The repo now contains focused `ferros-runtime` and `ferros-core` tests that harden the portable and host-side subcore seam for future x86_64 FERROS-root work.
- The repo now contains concrete research contracts for UEFI boot-path, kernel and privilege handoff, runtime port boundaries, and future QEMU-OVMF smoke inputs.

## Claims Explicitly Not Added
- No UEFI boot success.
- No kernel success.
- No QEMU success.
- No real hardware evidence.
- No D1 or G4 movement.
- No FERROS-native OS claim.

## Truth Surfaces Touched
- `docs/orchestration/WAVE-QUEUE.md`
- `docs/orchestration/WAVE-RUN-LOG.md`
- `docs/orchestration/doc-batches/DOC-BATCH-2026-05-03-X86-FERROS-SUBCORE-01.md`

## Next Queued Segment
No new Ready code-track items were left behind by this batch. The next honest follow-up is a separately queued x86_64 FERROS-root scaffold crate or workspace area for boot or kernel experiments, or a bounded host-side refactor that introduces trait-compatible non-`std` executor or bus backends.