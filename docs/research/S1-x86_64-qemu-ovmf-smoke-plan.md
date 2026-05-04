# x86_64 QEMU-OVMF Smoke Plan

Status: research note
Scope: x86_64 FERROS-root R1-R2 runway
Constraint: future command plan only; no QEMU proof

## Goal

Turn Stream E's QEMU-first posture into a concrete future smoke-plan contract so the first real emulation run has deterministic inputs, checkpoints, and artifact expectations.

## Sources

- `docs/streams/STREAM-E-CORE-OS.md`
- `docs/progress/ferros-core-os.md`
- `docs/research/S1-x86_64-ferros-uefi-boot-path.md`

## Future run ingredients

| Ingredient | Expected purpose | Current status |
|------------|------------------|----------------|
| OVMF firmware image | UEFI firmware for emulation | not selected in repo yet |
| `ferros-bootx64.efi` | future UEFI boot artifact | not implemented |
| `ferros-kernel-x86_64.elf` | future kernel artifact | not implemented |
| virtual disk image | future read-only blueprint or rootfs carrier | not implemented |
| serial capture path | first human-readable checkpoint trail | not implemented |

## Future command skeleton

```text
qemu-system-x86_64 \
  -machine q35 \
  -m 1024 \
  -bios <ovmf-firmware> \
  -drive format=raw,file=<ferros-disk-image> \
  -serial stdio
```

This command skeleton is planning-only. The angle-bracket inputs are intentionally unresolved until real boot artifacts exist.

## First smoke checkpoints

| Checkpoint | Expected proof if present |
|------------|---------------------------|
| firmware boots under OVMF | UEFI environment is reachable |
| boot artifact is found | handoff packaging is at least wired correctly |
| first serial checkpoint appears | kernel control reaches a human-readable surface |
| missing artifact or panic is explicit | failure is bounded enough for triage |

## Missing pieces before the first real smoke run

- A selected OVMF distribution and artifact path.
- A bootable UEFI application.
- A kernel artifact.
- A disk image builder or a documented manual image-assembly flow.
- A decision on whether serial output or framebuffer output is the first required checkpoint.

## Non-claims

- No QEMU command was run for this note.
- No boot artifact exists yet.
- No emulated or physical boot proof exists.
- No D1 or G4 movement is implied.