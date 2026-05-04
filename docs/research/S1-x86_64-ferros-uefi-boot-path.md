# x86_64 FERROS UEFI Boot Path

Status: research note
Scope: x86_64 FERROS-root R1
Constraint: architecture-only; no boot proof

## Goal

Translate the Stream E x86_64-first, UEFI-first posture into a concrete boot-path contract that future FERROS-root work can implement without improvising artifact names or checkpoint semantics.

## Sources

- `docs/streams/STREAM-E-CORE-OS.md`
- `docs/progress/ferros-core-os.md`
- `docs/adr/ADR-025-dual-root-hardware-runway.md`

## Future boot-path contract

```text
QEMU or real x86_64 power on
  -> OVMF or platform UEFI firmware initializes
  -> FERROS UEFI application loads from the EFI system partition
  -> UEFI application verifies or discovers the kernel artifact
  -> Bootloader hands a bounded machine context to the kernel entry point
  -> Kernel emits the first serial or framebuffer checkpoint
```

## Artifact families to preserve

| Artifact | Purpose | Current status |
|----------|---------|----------------|
| `ferros-bootx64.efi` | future UEFI application entry artifact | not implemented |
| `ferros-kernel-x86_64.elf` | future kernel handoff artifact | not implemented |
| `ferros-rootfs.img` | future read-only Phase 0 disk image | not implemented |
| `qemu-serial.log` | future human-readable boot checkpoint trail | not implemented |

These names are planning anchors only. They are not present in the repo and are not implied by this note to exist already.

## Minimum checkpoints the boot path will need

| Checkpoint | Why it matters |
|------------|----------------|
| firmware entered | proves the run started under UEFI rather than a legacy or Linux-hosted shim |
| boot artifact located | proves the UEFI application found the next-stage payload |
| kernel handoff attempted | proves the boot boundary is explicit and inspectable |
| first kernel checkpoint emitted | proves control reached the native runtime path |

## What stays unproven

- Whether the future boot artifact should be PE-only, ELF-backed, or use a two-stage packaging rule.
- The exact memory map contract for early kernel entry.
- The framebuffer versus serial-first ordering for the very first observable output.
- Any claim that QEMU or real hardware can boot today.

## Non-claims

- No UEFI application exists yet.
- No kernel exists yet.
- No QEMU success is claimed.
- No hardware evidence exists.
- No D1 or G4 movement is implied.