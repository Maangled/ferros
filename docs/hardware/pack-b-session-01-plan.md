# Pack B Session 01 Plan

> Plan-only handoff for the first named Pack B session. This file uses placeholders where real hardware or operator identities are not yet recorded. It is not evidence.

## Session identity

| Field | Current plan value | Note |
|-------|--------------------|------|
| Session label | `pack-b-session-01` | Stable planning label only |
| Pack B DUT name | `[PLACEHOLDER - x86_64 DUT name required before execution]` | Must be a real Pack B class device before execution |
| Pack C HA host name | `[PLACEHOLDER - HA host name required before execution]` | Must stay separate from the DUT |
| Operator station | `[PLACEHOLDER - operator station required before execution]` | Separate observation surface |
| Operator | `[PLACEHOLDER - operator name required before execution]` | Human-attended session only |
| DUT repo path | `[PLACEHOLDER - repo path on DUT required before execution]` | Used by the command map |
| Persistent state path | `[PLACEHOLDER - path for profile or grant or log material required before execution]` | Future durability observation only |
| Network note | `[PLACEHOLDER - LAN or addressing note required before execution]` | Must support DUT and HA host reachability |
| DUT-only power-cut method | `[PLACEHOLDER - exact power-cut method required before execution]` | Must not power down the HA host |
| Artifact capture root | `[PLACEHOLDER - output directory or note bundle required before execution]` | For future screenshots and transcripts only |

## Required before execution checklist

- [ ] Replace every placeholder in the session identity table with a real value.
- [ ] Confirm the DUT is a Pack B class `x86_64` device.
- [ ] Confirm the HA host is a separate Pack C machine on the same LAN.
- [ ] Confirm the operator station is separate from both DUT and HA host.
- [ ] Confirm the persistent state path is writable and intended for restart-safe material.
- [ ] Confirm the DUT-only power-cut method is isolated from the HA host.
- [ ] Confirm the artifact capture root exists or is planned.
- [ ] Do not execute the session while any placeholder remains unresolved.

## Authority inputs

- `docs/hub/local-code-runway-inventory.md`
- `docs/hub/reference-hardware.md`
- `docs/hub/pack-b-bring-up-worksheet.md`
- `docs/research/S7-d1-bring-up-checklist.md`
- `docs/hardware/pack-b-session-01-command-map.md`

## Topology contract for this planned session

| Role | Planned value | Constraint |
|------|---------------|------------|
| Device under test | `[PLACEHOLDER - Pack B DUT name required before execution]` | Must be the only DUT for this session |
| Home Assistant host | `[PLACEHOLDER - Pack C HA host name required before execution]` | Must remain separate from the DUT |
| Operator station | `[PLACEHOLDER - operator station required before execution]` | Must remain the observation surface |
| Storage path | `[PLACEHOLDER - persistent state path required before execution]` | Must be suitable for restart observations |
| Network | `[PLACEHOLDER - network note required before execution]` | Must support local shell or CLI observation |
| Power arrangement | `[PLACEHOLDER - DUT-only power-cut method required before execution]` | Must isolate the DUT from the HA host |

## Planned observation scope

This planned session may later record only the following if they are actually observed:

- profile init or show on the DUT,
- agent-center read via the localhost shell or CLI,
- named stand-in or bridge staging output,
- deny visibility,
- one full DUT-only power-cycle observation, with any clean reboot used only as a pre-check and not as the final session boundary.

## Explicit non-claims

- This file does not claim physical-device evidence has already been earned.
- This file does not claim a real Home Assistant entity or dashboard result exists.
- This file does not claim D1 closure or G4 closure.
- This file does not claim full power-cycle survival, independent install evidence, remote transport, canonical profile or grant mutation, or executed consent transport.