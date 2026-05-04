# x86_64 FERROS Runtime Port Plan

Status: research note
Scope: x86_64 FERROS-root R3-R4
Constraint: host-side seam hardening only; no no_std success claim

## Goal

Use the current `ferros-core` and `ferros-runtime` crates as the host-side subcore seam for x86_64 FERROS-root work, and define the minimum refactors future native work will need without claiming that the runtime is already portable to UEFI or a kernel.

## Current seam inventory

| Seam | Current anchor | Today |
|------|----------------|-------|
| portable policy and message types | `crates/ferros-core` | `no_std`-ready with `alloc` and portable token validation |
| job scheduling | `crates/ferros-runtime/src/executor.rs` | std-hosted in-memory queue |
| message routing | `crates/ferros-runtime/src/bus.rs` | std-hosted in-memory bus |
| lifecycle checkpoints | `crates/ferros-runtime/src/local_runway.rs` | pure state machine with host-side tests |

## Immediate port posture

The current host-side runtime can already act as a subcore rehearsal seam if the repo preserves two boundaries:

- `ferros-core` remains the portable foundation surface for capability, request, and message types.
- `ferros-runtime` remains explicit that its current in-memory implementations are host-side stand-ins, not native supervisor proof.

## Future split points

| Future need | Likely seam |
|-------------|-------------|
| no_std job queue | trait-compatible executor backend separate from `InMemoryExecutor` |
| no_std message dispatch | trait-compatible bus backend separate from `InMemoryMessageBus` |
| native checkpoint persistence | lifecycle state surface that can be backed by a native store later |
| native supervisor or task model | thin orchestration layer above the portable `LocalRunwayState` transitions |

## What the host-side example should prove now

- the lifecycle state machine can progress deterministically to `Active`,
- jobs can be scheduled in submission order,
- a message can be routed through the current bus seam,
- the example remains explicitly std-hosted and local-only.

## What remains unproven

- no_std execution for `ferros-runtime`,
- boot-time allocator story,
- native interrupt or task wake-up model,
- kernel or UEFI integration.

## Non-claims

- No no_std `ferros-runtime` port exists.
- No kernel supervisor exists.
- No boot success is implied.
- No QEMU or hardware proof is added by this note.