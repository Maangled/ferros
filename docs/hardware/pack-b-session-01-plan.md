# Pack B Session 01 Plan

> Plan-only handoff for the first named Pack B session. This file records current planning facts and does not by itself constitute evidence.

## Temporary mode note (2026-05-03)

For immediate bring-up on `homelab001`, this session allows co-located DUT and HA host usage.
Separate-host HA validation and isolated DUT-only hard power-cut proof are deferred to a later segment.

## Session identity

| Field | Current plan value | Note |
|-------|--------------------|------|
| Session label | `pack-b-session-01` | Stable planning label only |
| Pack B DUT name | `homelab001` | Pack B class `x86_64` machine for local bring-up |
| Pack C HA host name | `homelab001` (temporary co-located mode) | Separate-host validation deferred |
| Operator station | `homelab001` local shell | Current observation surface |
| Operator | `Maangled` | Human-attended session owner |
| DUT repo path | `/home/homelab001/apps/ferros` | Used by the command map |
| Persistent state path | `/home/homelab001/apps/ferros/.local-state` | Local durability observation path |
| Network note | `homelab001` on `192.168.50.234/24` (`enp4s0`) | Supports local shell or CLI observation |
| DUT-only power-cut method | `[DEFERRED - define once DUT and HA are separated]` | Deferred under temporary co-located mode |
| Artifact capture root | `/home/homelab001/apps/ferros/.local-artifacts` | For screenshots and transcripts |

## Required before execution checklist

- [ ] Replace every placeholder in the session identity table with a real value.
- [ ] Confirm the DUT is a Pack B class `x86_64` device.
- [ ] Confirm the HA host role is available (co-located temporary mode allowed).
- [ ] Confirm the operator station is explicitly declared.
- [ ] Confirm the persistent state path is writable and intended for restart-safe material.
- [ ] Confirm the DUT-only power-cut method is defined, or explicitly deferred under co-located mode.
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
| Device under test | `homelab001` | Must be the only DUT for this session |
| Home Assistant host | `homelab001` (temporary co-located mode) | Separate-host requirement deferred for current local bring-up |
| Operator station | `homelab001` local shell | Observation surface for this session |
| Storage path | `/home/homelab001/apps/ferros/.local-state` | Suitable for restart observations |
| Network | `192.168.50.234/24` on `enp4s0` | Supports local shell or CLI observation |
| Power arrangement | Deferred in co-located mode | Hard DUT-only power-cut validation moved to later separated-host segment |

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