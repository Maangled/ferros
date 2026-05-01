# D1 Target Inventory

> Planning inventory for the selected first D1 target lane and its required companion roles. This file binds the current hardware runway to a concrete target class without inventing the exact hardware identifiers that must still be filled before execution. It is not evidence.

## Purpose

Use this inventory to record the selected first D1 target lane now and fill the exact device, host, and operator identifiers later when the real session hardware is pinned.

## Candidate inventory

| Candidate slot | Planned role | Current value | Form factor | OS or firmware baseline | Display capability | Known bring-up effort | Notes |
|----------------|--------------|---------------|-------------|-------------------------|--------------------|----------------------|-------|
| Primary target | Pack B `x86_64` D1 target | `Pack B - x86_64 lane` | Intel NUC, Lenovo Tiny, Beelink-class mini PC, or spare small-form-factor home server with SSD and wired LAN | Modern 64-bit Linux; prefer Debian 12 or Ubuntu 22.04 | Local console or temporary monitor or keyboard fallback for recovery | Lower-friction observability-first bring-up lane | Selected first D1 target class; exact DUT identifier still required before execution |
| Fallback target | Secondary D1 candidate | `Pack A - Pi lane` | Raspberry Pi 5 with USB 3 SSD preferred; Raspberry Pi 4 fallback | Raspberry Pi OS 64-bit or another modern 64-bit Linux | Local console or temporary monitor or keyboard fallback for recovery | Higher bring-up variance but still launch-valid | Use if Pack B availability changes or if a Pi-first pass becomes necessary |
| Pack C companion | Separate Home Assistant host | `Pack C - HA companion lane` | HA Green or Yellow, Raspberry Pi 4, or small `x86_64` box on the same LAN | Existing Home Assistant host on the same LAN | `not applicable` | Separate-host lab bring-up | Exact host identifier still required before execution |
| Operator station | Separate observation surface | `Separate laptop or desktop operator station` | Laptop or desktop | Modern host OS with SSH and note capture | Primary observation surface | Low | Exact operator-station identifier still required before execution |

## Primary target decision

| Field | Current value |
|-------|---------------|
| Selected primary D1 target | `Pack B - x86_64 lane` |
| Why this is the primary target | Most practical first integration target because it maximizes shell access, rollback, log capture, and DUT-only power-cycle observation while keeping Pack C separate. |
| Known blockers | Exact DUT identifier, Pack C host identifier, operator-station identifier, storage path, network note, and DUT-only power-cut method are still required before execution. |
| Required tools | SSH access, local console fallback, stable LAN, separate Pack C host, repeatable DUT-only power-cut path, and an artifact capture location. |

## Required before execution checklist

- [ ] Replace the selected Pack B lane with the exact DUT identifier that will be used for execution.
- [ ] Replace the Pack C HA companion lane with the exact host identifier that will be used for execution.
- [ ] Replace the operator-station description with the exact observation surface or host name that will be used for execution.
- [ ] Record the concrete storage path, network note, and DUT-only power-cut method in the session plan.
- [ ] Confirm the selected Pack B lane still remains the primary D1 target when the session is scheduled.
- [ ] Do not treat this inventory as evidence while any execution-time identifier remains placeholder-based.

## Non-claims

- This file does not claim that a physical device session has happened.
- This file does not claim D1 closure or G4 closure.
- This file does not claim Home Assistant proof, power-cycle proof, or independent install evidence.