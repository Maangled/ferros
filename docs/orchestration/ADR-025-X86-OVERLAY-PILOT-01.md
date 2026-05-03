# ADR-025 x86_64 Overlay Pilot 01

Status: Active
Date: 2026-05-03
Authority: docs/orchestration/LOCAL-DRIVER.md

## Mission
Start bounded implementation of the ADR-025 x86_64 overlay pilot without replacing the current S1-S8 stream, gate, and queue substrate.

## Authority Lock
- `STATUS.md`, `docs/orchestration/LOCAL-DRIVER.md`, `docs/orchestration/BATCH-MODE.md`, `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/SYSTEM-QUEUE.md`, and `docs/orchestration/HARDWARE-QUEUE.md` remain the live execution authority.
- ADR-025 remains Proposed and non-binding until the guardrail checks are resolved and disposition is recorded through ADR-022 governance.
- The current S1-S8 stream stack is not retired or folded into ADR-025 by this pilot segment.
- Copilot chat remains the temporary median orchestrator until Agent Center APIs and telemetry exist.

## Pilot Objectives
- Establish one explicit x86_64 overlay pilot surface that maps the current repo truth into provisional `Fastest` and `FERROS` roots.
- Queue one x86_64 lane-map wave that can test the proposed Fastest grouping and the FERROS-side architecture grouping without mutating current stream authority.
- Queue one provisional S9 service-packet wave that must prove non-redundant reload or handoff behavior instead of duplicating S8 truth-sync.
- Queue one metadata-translation wave so ADR-025 lane packets can preserve current `size`, `parallel-safe-with`, `serial-after`, `solo`, and `track` behavior before any migration is attempted.

## Claim Ceiling
- No ADR-025 promotion.
- No S1-S8 retirement.
- No D1 closure.
- No G4 closure.
- No physical-device evidence claim.
- No real Home Assistant proof claim.
- No FERROS-native OS claim.
- No background-autonomy or always-running-lane claim.

## Candidate Overlay Shape
- `x86_64/Fastest` is the first overlay root because it already matches the current practical integration path.
- `x86_64/FERROS` remains architecture-first and virtualized for now: bootloader, kernel, process, memory, driver, storage, networking, display/UI, and package/update research.
- The proposed Fastest grouping of `S1-S4` backend, `S5` mediator, and `S6-S9` operator-facing surfaces is treated as a candidate taxonomy until a lane-map wave tests it against current stream truth.
- `S9` remains provisional and may propose reloads or reroutes, but may not replace S8 truth-sync or queue authority.

## Initial Lane Plan
1. `O0` overlay coordination lock
2. `O1` x86_64 Fastest/FERROS lane map
3. `O2` provisional S9 non-redundancy packet
4. `O3` queue-metadata translation for ADR-025 lane packets

## Expected Outcome
- The overlay pilot is now a queued, schedulable system-track packet instead of an informal chat-only plan.
- The next waves can test lane shape, S9 value, and metadata translation without replacing current execution authority.
- The repo keeps one honest path for future Agent Center or hub traffic visualization while the current driver model remains intact.

## HANDOFF CARD
- Lane ID: O0
- Status: complete
- Files read: STATUS.md; docs/orchestration/LOCAL-DRIVER.md; docs/orchestration/BATCH-MODE.md; docs/orchestration/WAVE-QUEUE.md; docs/orchestration/SYSTEM-QUEUE.md; docs/orchestration/HARDWARE-QUEUE.md; docs/adr/ADR-025-dual-root-hardware-runway.md; docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-guardrail-scoreboard.md; docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-guardrails-batch1.md; docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-guardrails-batch2.md
- Files changed: docs/orchestration/ADR-025-X86-OVERLAY-PILOT-01.md
- Evidence produced: coordination authority note for the ADR-025 overlay pilot kickoff
- Claims added: the pilot now has explicit authority lock, claim ceiling, and queued next lanes
- Claims explicitly not added: ADR promotion, stream retirement, gate movement, hardware evidence, background autonomy
- Validation: markdown structure and scope review
- Residual risks: S9 non-redundancy, metadata translation, and lane-boundary enforcement are still unproven
- Next safe follow-up, if any: queue and execute the x86_64 lane-map and provisional S9 waves