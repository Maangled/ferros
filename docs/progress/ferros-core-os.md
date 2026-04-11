# FERROS Core OS — Progress Spec

**Current:** 1%
**Phase:** Architecture
**Status:** Architecture planning is defined for the Phase 0 rendering path, but no Rust source tree exists yet.

## What This Is

This spec tracks the operating-system side of FERROS as defined by `README.md` and `docs/architecture-overview.md`. It covers the boot-to-render path required for Phase 0, where the future system must render `ferros-blueprint.html` natively without relying on a browser engine.

## Hardware Target Matrix Summary

This is a planning summary only. [docs/core-hardware-targets.md](./../core-hardware-targets.md) is the authoritative device-class reference for FERROS hardware roles and target status.

| Class | Example Hardware | Near-Term Role | FERROS Status |
|------|-------------------|----------------|---------------|
| Micro node | ESP32-class boards | Sensors, switches, relays, tiny room controllers | Ecosystem device, not a Phase 0 OS target |
| Edge ARM node | Raspberry Pi 4/5, Jetson Orin Nano | Screens, cameras, room hubs, edge inference | Linux-first today, FERROS later |
| Personal x86 node | Laptops, handhelds, mini PCs | Daily-driver machine, profile root, local-first personal system | Major future FERROS target |
| Command x86 node | Desktops, servers, consumer GPU towers | Cluster coordinator, agents, heavy inference, household compute | Strongest early real-hardware FERROS target |

## Targeting Principles

- Phase 0 remains x86_64-first, QEMU-first, and UEFI-first.
- ARM edge hardware is a real FERROS migration path, but not the first kernel bring-up target.
- ESP32-class boards should be treated as FERROS-compatible peripherals, not as the smallest full FERROS computer.
- Household acceleration should be explicit local-network delegation, not a hard dependency for baseline operation.

## Scoped Components

| Component | Current | Notes |
|-----------|---------|-------|
| UEFI Bootloader | 1% | Rust-native boot path is required by `README.md`. |
| Kernel (x86_64) | 1% | Primary first implementation target for Phase 0. |
| Kernel (ARM/AArch64) | 0% | Target not started. |
| Kernel (RISC-V) | 0% | Target not started. |
| Memory Manager | 0% | Required for virtual memory initialization. |
| Filesystem Driver | 0% | Minimal read-only access is required for Phase 0. |
| HTML/CSS Renderer | 1% | Rendering requirements are specified in prose and the blueprint. |
| Framebuffer / Graphics | 0% | Needed for text, colors, gradients, and layout output. |
| Phase 0 Conformance | 1% | Success condition is rendering the founding blueprint natively. |

## Milestone Gates

| % | Gate | Deliverables | Done? |
|---|------|-------------|-------|
| 10% | Boot target defined | Boot flow, kernel entry assumptions, and Phase 0 component boundaries are documented in repo docs | ☐ |
| 20% | x86_64/QEMU contract fixed | UEFI handoff contract, framebuffer assumptions, and x86_64-first validation path are documented explicitly | ☐ |
| 30% | Hardware target matrix documented | Device classes, runtime roles, and the difference between OS targets and ecosystem devices are documented in `docs/core-hardware-targets.md` | ☐ |
| 40% | Device-class assumptions defined | Minimal storage, display, and runtime assumptions are documented for x86_64 targets, ARM edge nodes, and micro-node peripherals | ☐ |
| 50% | Emulated Phase 0 render contract | The full boot, parse, layout, and render path is testable in emulation without depending on service infrastructure | ☐ |
| 60% | First real x86_64 reference machine selected | One concrete desktop or server-class x86_64 machine is selected as the first real bring-up target | ☐ |
| 70% | Command-node bring-up plan | Real-machine command-node assumptions, debug path, and household role are documented | ☐ |
| 80% | ARM migration candidate documented | At least one ARM edge target is documented alongside the x86_64 bring-up path | ☐ |
| 90% | Device-class conformance checklist | A repo-visible checklist maps Phase 0 and migration expectations to each device class | ☐ |
| 100% | Production / Complete | FERROS boots, loads the blueprint from local storage, and renders it natively on QEMU and one real x86_64 reference machine, with a documented ARM migration path | ☐ |

## Dependencies

- `README.md` — defines the canonical percentages and the Phase 0 success criteria.
- `docs/architecture-overview.md` — defines the bootloader, kernel, storage, compositor, and rendering subsystem targets.
- `docs/core-hardware-targets.md` — defines device classes, runtime roles, and the difference between OS targets and FERROS-compatible devices.
- `ferros-blueprint.html` — Phase 0 conformance target that the future OS must render natively.
- `docs/deployment-roadmap.html` — separates the Linux deployment track from the future kernel track.
- `docs/adr/ADR-0001-start-new-do-not-fork.md` — confirms FERROS is a greenfield implementation, not a forked kernel tree.

## Current Blockers

- No mature Rust workspace or kernel source tree exists in the repo yet.
- The Phase 0 path is specified, but implementation artifacts for boot, memory, storage, and rendering are still TBD.
- Reference hardware selection and device-class validation still need to move from planning into concrete bring-up targets.