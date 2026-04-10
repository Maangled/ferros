# FERROS Core OS — Progress Spec

**Current:** 1%
**Phase:** Architecture
**Status:** Architecture planning is defined for the Phase 0 rendering path, but no Rust source tree exists yet.

## What This Is

This spec tracks the operating-system side of FERROS as defined by `README.md` and `docs/architecture-overview.md`. It covers the boot-to-render path required for Phase 0, where the future system must render `ferros-blueprint.html` natively without relying on a browser engine.

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
| 20% | Early boot contract | UEFI handoff contract, framebuffer assumptions, and x86_64-first target are documented in a dedicated implementation plan or ADR | ☐ |
| 30% | Memory bring-up design | Virtual memory, allocator strategy, and isolation model are documented with concrete subsystem boundaries | ☐ |
| 40% | Storage read path design | Minimal filesystem read path for loading `ferros-blueprint.html` is specified without implying a finished storage stack | ☐ |
| 50% | Parsing contract | HTML parsing and CSS layout support targets are enumerated against the Phase 0 success criteria | ☐ |
| 60% | Render pipeline contract | Text rendering, colors, gradients, borders, and layout fidelity checks are documented as verifiable acceptance tests | ☐ |
| 70% | QEMU execution target | QEMU or KVM run path, debug assumptions, and reference test procedure are captured in repo docs | ☐ |
| 80% | Reference hardware target | At least one real hardware target, boot assumptions, and display path expectations are documented | ☐ |
| 90% | Conformance checklist | A repo-visible checklist maps each Phase 0 success criterion to a concrete subsystem test | ☐ |
| 100% | Production / Complete | FERROS boots, loads the blueprint from local storage, and renders it natively on QEMU and one reference hardware target | ☐ |

## Dependencies

- `README.md` — defines the canonical percentages and the Phase 0 success criteria.
- `docs/architecture-overview.md` — defines the bootloader, kernel, storage, compositor, and rendering subsystem targets.
- `ferros-blueprint.html` — Phase 0 conformance target that the future OS must render natively.
- `docs/deployment-roadmap.html` — separates the Linux deployment track from the future kernel track.
- `docs/adr/ADR-0001-start-new-do-not-fork.md` — confirms FERROS is a greenfield implementation, not a forked kernel tree.

## Current Blockers

- No mature Rust workspace or kernel source tree exists in the repo yet.
- The Phase 0 path is specified, but implementation artifacts for boot, memory, storage, and rendering are still TBD.
- Reference hardware and QEMU execution procedures are not yet captured as dedicated implementation docs.