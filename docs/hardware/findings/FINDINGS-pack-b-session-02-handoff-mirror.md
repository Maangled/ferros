# Findings — Pack B Session 02 Handoff Mirror

> Filled from agent-executed post-reboot commands on `homelab001` under explicit operator authorization from `Maangled` after a clean reboot at `2026-05-04 01:56:40`. The DUT returned on the same IPv4 address `192.168.50.234/24`, which allowed capture to resume immediately. Raw captures live under `.local-artifacts/pack-b-session-02-handoff-mirror/`.

## Claim ceiling

- This findings packet captures only the DUT-side mirror of the local handoff chain on the named Pack B device.
- A clean reboot observation is not the same thing as full power-cycle survival.
- This findings packet does not authorize a claim of real Home Assistant proof, D1 closure, G4 closure, or executed consent transport.

## Session header

| Field | Value |
|-------|-------|
| Date | `2026-05-04` |
| Operator | `Maangled` |
| Pack B DUT name | `homelab001` |
| Named stand-in or bridge path | `ha-local-bridge` local-only stand-in mirrored through `cargo xtask hub-runway --keep-artifacts` and `cargo run -p ferros-hub -- summary` |

## Bridge artifact or stand-in output

| Field | Value |
|-------|-------|
| Command or route used | `cargo xtask hub-runway --keep-artifacts`; `cargo run -p ferros-hub -- summary` |
| Artifact or output reference | `.local-artifacts/pack-b-session-02-handoff-mirror/simulated-local-bridge-artifact.json`; `.local-artifacts/pack-b-session-02-handoff-mirror/post-reboot-hub-summary.txt`; `.local-artifacts/pack-b-session-02-handoff-mirror/post-reboot-xtask-hub-runway.txt` |
| Bridge agent or stand-in name | `ha-local-bridge`; stand-in `simulated-bridge-entity` |
| Requested capability or action | `bridge.observe`; `report-state` |

## Proposal fields

| Field | Value |
|-------|-------|
| `proposalId` | `proposal-ha-local-bridge-simulated-bridge-entity-report-state` |
| Quarantine status | `quarantined-pending-consent` |
| Local artifact path | `.tmp/hub/local-onramp-proposal.json` (copied to `.local-artifacts/pack-b-session-02-handoff-mirror/local-onramp-proposal.json`) |
| Notes | `local-onramp-proposal.json` points back to source `.tmp/hub/simulated-local-bridge-artifact.json`, keeps `scope=local-only`, and remains `evidence=non-evidentiary`.` |

## Decision fields

| Field | Value |
|-------|-------|
| `proposalId` | `proposal-ha-local-bridge-simulated-bridge-entity-report-state` |
| `decisionLabel` | `allowed` |
| `decisionDetail` | `local-only operator rehearsal allowed report-state for proposal proposal-ha-local-bridge-simulated-bridge-entity-report-state` |
| Local artifact path | `.tmp/hub/local-onramp-decision-receipt.json` (copied to `.local-artifacts/pack-b-session-02-handoff-mirror/local-onramp-decision-receipt.json`) |

## Shell or log observation

| Field | Value |
|-------|-------|
| Observation path | `.local-artifacts/pack-b-session-02-handoff-mirror/post-reboot-xtask-hub-runway.txt`; `.local-artifacts/pack-b-session-02-handoff-mirror/post-reboot-hub-summary.txt`; `.local-artifacts/pack-b-session-02-handoff-mirror/local-hub-state-snapshot.json` |
| Proposal visible | `yes` — `hubOnrampProposalStatus: quarantined-pending-consent` and the copied proposal artifact both match the local handoff map |
| Decision visible | `yes` — `hubOnrampDecisionLabel: allowed`, `hubOnrampDecisionProposalId: proposal-ha-local-bridge-simulated-bridge-entity-report-state`, and the copied decision receipt match the local handoff map |
| Restart context visible | `yes` — `restartReload: reloaded`, `restartPriorBridgeManifest: ha-local-bridge@0.1.0`, and `restartPriorPolicyDecision: allowed` were present after reboot |

## Deny visibility

| Field | Value |
|-------|-------|
| Observation path | `.local-artifacts/pack-b-session-02-handoff-mirror/post-reboot-deny-demo.txt` |
| Denied request visible | `yes` |
| Notes | `ferros-hub deny-demo: denied:no-grants for simulated-bridge-entity without artifact [local-only; non-evidentiary]` |

## Restart observation boundary

| Observation type | Result | Notes |
|------------------|--------|-------|
| Clean reboot observation | `observed` | `uptime -s` reported boot time `2026-05-04 01:56:40`; post-reboot capture resumed at `2026-05-04T01:59:16+00:00`; `enp4s0` remained at `192.168.50.234/24`, and the post-reboot hub summary showed `restartReload: reloaded`.` |
| Full DUT-only power-cycle survival | `not claimed unless separately attempted` | `This packet covers a clean reboot only. No separate DUT-only hard power cut was performed.` |

## Post-restart agent re-registration observation

| Field | Value |
|-------|-------|
| Command or route used | `cargo run -p ferros-node --bin ferros -- agent list`; `cargo run -p ferros-node --bin ferros -- agent describe ha-local-bridge` |
| At least one agent visible after restart or power cycle | `yes` |
| Observation note | `post-reboot-agent-list.txt` showed `echo`, `ha-local-bridge`, and `timer` all registered. `post-reboot-agent-describe.txt` resolved `ha-local-bridge` with required capability `hub-local-bridge:bridge.observe`.` |

## Remaining gaps

- This packet does not prove full DUT-only power-cycle survival; it records a clean reboot only.
- This packet does not prove real Home Assistant visibility; Pack C remains the separate-host HA evidence surface.
- This packet captures local proposal and decision rehearsal artifacts only; it does not claim consent acceptance or canonical mutation.
- D1 and G4 remain open.

## Non-claims for this template

- No real Home Assistant proof unless separately captured on Pack C.
- No D1 or G4 closure.
- No full power-cycle survival unless explicitly and separately observed.