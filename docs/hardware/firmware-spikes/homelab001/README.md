# homelab001 Firmware-Spike Target Plan

> Planning note only for the chosen D1 device. This file does not by itself constitute D1 evidence, G4 evidence, Home Assistant proof, or ADR-025 operational closure.

## Target summary

| Field | Value | Note |
|-------|-------|------|
| Target device | `homelab001` | Current chosen D1 device and Pack B `x86_64` lane |
| Architecture | `x86_64` | Matches the first hardware runway target |
| Repo path | `/home/homelab001/apps/ferros` | Local operator shell path |
| State path | `/home/homelab001/apps/ferros/.local-state` | Candidate persistent state path |
| Artifact root | `/home/homelab001/apps/ferros/.local-artifacts` | Local capture root for transcripts and copied artifacts |
| Current HA mode | Co-located temporary mode on `homelab001` | Separate-host Pack C proof remains deferred |

## Claim ceiling

- This plan is docs-only.
- Boot and identify milestones may be rehearsed locally on `homelab001`, but that does not by itself close D1 or G4.
- Accept-grant and report-state remain blocked on the current bridge-agent visibility mismatch and later separate-host HA proof.
- No firmware code, runtime daemon claim, reboot-safe persistence claim, or full power-cycle claim is added here.

## Milestone plan

| Milestone | Expected deliverable | Toolchain or wiring needed | Known unknowns |
|-----------|----------------------|----------------------------|----------------|
| `boot` | The DUT boots to a stable Linux shell, the FERROS repo is reachable, `.local-state` and `.local-artifacts` are writable, and the local hub rehearsal chain can be refreshed without mutating external systems. | Local shell on `homelab001`; Rust and Cargo toolchain already used by FERROS; wired LAN; stable local power; `cargo xtask hub-runway --keep-artifacts`. | The isolated DUT-only power-cut path is still deferred in co-located mode; no separate operator station is being exercised in this first local slice. |
| `identify` | `ferros profile init` and `ferros profile show` succeed on the named DUT and identify one concrete local profile path plus visible profile fields. | `cargo run -p ferros-node --bin ferros -- profile init ...`; `cargo run -p ferros-node --bin ferros -- profile show ...`; writable `.local-state` path; operator-attended shell capture. | Restart-safe reload is not proven by a same-session init and show; D1 evidence rows remain blank until a later real gate capture writes them down. |
| `accept-grant` | One honest capability-proposal and decision chain is visible from the DUT side, with a named stand-in or bridge path and a clear record of whether anything was accepted versus still local-only rehearsal. | `cargo xtask hub-runway --keep-artifacts`; `cargo run -p ferros-hub -- summary`; `cargo run -p ferros-hub -- prove-bridge`; future S5/onramp consent surface or explicit stand-in note; copied `.tmp/hub` proposal and decision artifacts. | No canonical capability grant has been issued from a real consent flow; `ha-local-bridge` is reported by `ferros-hub` but not by `ferros agent list`, so the current local bridge visibility path is not yet trustworthy enough to overclaim acceptance. |
| `report-state` | One HA-facing state report or explicitly named stand-in report is captured with enough context to say whether the DUT produced a local-only rehearsal artifact versus a real Pack C-visible state update. | Local hub summary and proof commands; later Pack C HA host or a clearly labeled local stand-in; dashboard or log capture once a separate host exists. | Current evidence stops at a local-only stand-in artifact; no separate-host HA entity visibility exists yet, so report-state cannot be promoted beyond rehearsal output. |

## Current execution posture

| Milestone | Current posture | Why |
|-----------|-----------------|-----|
| `boot` | Ready now on `homelab001` | Current local shell and repo path are stable and already exercised |
| `identify` | Ready now on `homelab001` | The profile init and show slice is executable on the chosen DUT |
| `accept-grant` | Partial rehearsal only | Local proposal and decision artifacts exist, but no accepted real grant and no trustworthy bridge-agent listing path exist yet |
| `report-state` | Deferred | Separate-host Pack C proof has not been run |

## Sources held constant for this plan

- `docs/hub/reference-hardware.md`
- `docs/hub/pack-b-bring-up-worksheet.md`
- `docs/hardware/pack-b-session-01-plan.md`
- `docs/hardware/pack-b-session-01-command-map.md`
- `docs/hardware/findings/FINDINGS-homelab001-local-bringup.md`