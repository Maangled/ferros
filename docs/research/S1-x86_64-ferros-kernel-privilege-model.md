# x86_64 FERROS Kernel and Privilege Model

Status: research note
Scope: x86_64 FERROS-root R2
Constraint: architecture-only; no kernel proof

## Goal

Define the first honest privilege and handoff posture for x86_64 FERROS-root work so future boot and kernel experiments have a bounded target instead of jumping straight from Stream E prose to implementation.

## Sources

- `docs/streams/STREAM-E-CORE-OS.md`
- `docs/progress/ferros-core-os.md`
- `docs/adr/ADR-025-dual-root-hardware-runway.md`

## Recommended early posture

The first kernel experiment should stay small and explicit:

1. A UEFI-loaded entry artifact transfers control to a single early kernel context.
2. The early kernel runs with maximum privilege while memory, interrupts, and output are still minimal.
3. User-space, multi-process isolation, and package-loading semantics stay deferred until the early kernel can emit deterministic checkpoints.

This keeps the first proof target narrow: controlled handoff plus bounded observation, not a full operating environment.

## First privilege phases

| Phase | Owner | Near-term responsibility |
|-------|-------|--------------------------|
| Firmware phase | platform or OVMF | initialize UEFI services and hand control to the FERROS boot artifact |
| Bootloader phase | future FERROS UEFI application | locate the kernel artifact and pass machine context forward |
| Early kernel phase | future FERROS x86_64 kernel | establish minimal memory and output primitives |
| Subcore services phase | future runtime or supervisor primitives | begin scheduling, message routing, storage, and display services |

## Why the early kernel should stay narrow

- `docs/progress/ferros-core-os.md` still says no Rust kernel source tree exists.
- Stream E treats QEMU bring-up as the first Phase 0 success condition, not full user-space parity.
- `ferros-runtime` already exposes lifecycle and message seams on the host side, which means the native kernel can defer richer service architecture until the first handoff is visible.

## Deferred questions

- When to introduce ring separation beyond the first kernel checkpoint.
- Whether the first service model is one address space with explicit internal boundaries or a true multi-process model.
- How much of `ferros-runtime` should become kernel-resident versus hosted above the kernel.
- Which storage and display primitives belong below versus above the first privilege split.

## Non-claims

- No kernel crate exists yet.
- No privilege boundary is implemented.
- No userspace model is fixed.
- No QEMU or hardware kernel run is claimed.