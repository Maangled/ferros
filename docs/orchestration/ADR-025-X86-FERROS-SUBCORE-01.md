# ADR-025 x86_64 FERROS Subcore Batch 01

Status: bounded coordination lock
Track: code
Date: 2026-05-03

## Purpose

This note starts the first bounded x86_64 FERROS-root subcore batch under the accepted ADR-025 framework. It does not create a new queue type or retire the current stream substrate. It maps the first `x86_64/FERROS` research lanes onto concrete repo seams so the orchestrator can execute real research, examples, and tests without pretending boot or firmware proof already exists.

## Claim ceiling

- This packet is subcore seed work only.
- It may add research notes, host-side examples, and focused portable tests.
- It may not claim UEFI boot success, kernel success, QEMU success, hardware evidence, D1 movement, G4 movement, or a FERROS-native OS run.

## Preserved authorities

- `docs/orchestration/WAVE-QUEUE.md` remains the live queue authority for this batch.
- `docs/orchestration/LOCAL-DRIVER.md` remains the orchestration authority, including the Lane Architect review step.
- `docs/adr/ADR-025-dual-root-hardware-runway.md` remains framework authority for Fastest versus FERROS root separation.
- `docs/streams/STREAM-E-CORE-OS.md` remains the long-range target posture for x86_64-first, UEFI-first, QEMU-first native work.

## Lane-to-seam map

| FERROS lane | Current repo seam | Near-term batch posture |
|-------------|-------------------|-------------------------|
| `R1` bootloader and boot path | `docs/streams/STREAM-E-CORE-OS.md`; `docs/progress/ferros-core-os.md` | research contract only |
| `R2` kernel and privilege model | `docs/streams/STREAM-E-CORE-OS.md`; future kernel source tree absent today | research contract only |
| `R3` process and lifecycle model | `crates/ferros-runtime/src/executor.rs`; `crates/ferros-runtime/src/local_runway.rs` | host-side example and tests now |
| `R4` memory and storage primitives | `crates/ferros-core`; `crates/ferros-runtime`; future no_std split still open | port-plan note now |
| `R5` driver and hardware abstraction | no driver tree exists yet | deferred |
| `R6` networking primitives | no FERROS-native network tree exists yet | deferred |
| `R7` display and UI subsystem | `docs/streams/STREAM-E-CORE-OS.md`; `ferros-blueprint.html` conformance target | research contract only |
| `R8` package, update, and onramp behavior | existing Fastest onramp surfaces plus future native packaging | deferred |

## First batch outputs

| Wave | Output | Why it is honest now |
|------|--------|----------------------|
| `83` | coordination lock | binds lanes to real seams before code expands |
| `84` | UEFI boot-path note | turns Stream E posture into a concrete artifact contract |
| `85` | kernel and privilege note | defines early handoff without claiming a kernel |
| `86` | runtime port plan plus runnable subcore example | exercises current lifecycle primitives on the host |
| `87` | runtime subcore tests | proves current host-side subcore composition |
| `88` | portable foundation tests | hardens the `ferros-core` seam used by future boot/runtime work |
| `89` | QEMU-OVMF smoke plan | turns R2 into explicit future checkpoints |
| `90` | serial truth-sync | records exactly what landed and what remains unproven |

## Non-goals for this batch

- No boot image.
- No UEFI application.
- No kernel crate.
- No QEMU execution.
- No real hardware session.
- No replacement of x86_64 Fastest lanes.

## Next proof edge after this batch

If the example and tests land cleanly, the next honest implementation edge is a dedicated x86_64 FERROS-root scaffold crate or workspace area for boot or kernel experiments. That is future work and should be queued explicitly instead of implied by these seed waves.