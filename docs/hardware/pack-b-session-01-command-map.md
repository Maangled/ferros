# Pack B Session 01 Command and Artifact Map

> Plan-only command map for the future first named Pack B session. Every command below is a future operator-run reference, not an executed result.

## Command map

| Local rehearsal command | Future DUT or operator reference | Planned purpose | Expected local-only artifact or read path | Non-claim boundary |
|-------------------------|----------------------------------|-----------------|------------------------------------------|--------------------|
| `cargo xtask hub-runway` | `cd [PLACEHOLDER - DUT repo path]; cargo xtask hub-runway` | Refresh the full local rehearsal chain on the named DUT or attached working copy before capture | `.tmp/hub/simulated-local-bridge-artifact.json`, `.tmp/hub/local-hub-state-snapshot.json`, `.tmp/hub/local-onramp-proposal.json`, `.tmp/hub/local-onramp-decision-receipt.json` | Local rehearsal only. Not hardware proof by itself. |
| `cargo run -p ferros-hub -- summary` | `cd [PLACEHOLDER - DUT repo path]; cargo run -p ferros-hub -- summary` | Read the bounded local hub summary on the planned DUT path | Summary stdout plus the current snapshot or proposal or decision context | Local readout only. Not durable runtime evidence. |
| `cargo run -p ferros-hub -- prove-bridge` | `cd [PLACEHOLDER - DUT repo path]; cargo run -p ferros-hub -- prove-bridge` | Re-run the bounded local bridge proof path before any future DUT-side mirror | `.tmp/hub/simulated-local-bridge-artifact.json` and compact proof output | Simulated local proof only. Not Home Assistant proof. |
| `ferros profile init` | `ferros profile init` on `[PLACEHOLDER - Pack B DUT name]` | Establish or refresh the local profile boundary on the target device | Reported local profile path | Not evidence until captured in a real session. |
| `ferros profile show` | `ferros profile show` on `[PLACEHOLDER - Pack B DUT name]` | Confirm the visible profile fields after init or restart | CLI output only | Not restart-safe proof by itself. |
| `ferros agent list` | `ferros agent list` on `[PLACEHOLDER - Pack B DUT name]` | Confirm agent-center read visibility before and after restart or power-cycle steps | CLI output only | Not re-registration proof until captured during a real session. |
| `ferros agent describe [PLACEHOLDER - agent name]` | `ferros agent describe [PLACEHOLDER - agent name]` | Inspect the named stand-in or future bridge agent manifest fields | CLI output only | Not Home Assistant proof. |
| `ferros agent logs [PLACEHOLDER - agent name]` | `ferros agent logs [PLACEHOLDER - agent name]` | Observe deny visibility or bridge-side notes through the FERROS inspection path | CLI output only | Not executed consent transport. |
| Post-power-cycle `ferros agent list` | `ferros agent list` after the DUT-only power cycle on `[PLACEHOLDER - Pack B DUT name]` | Record whether at least one agent re-registers after the required D1-style power-cycle step | CLI output only | Not a G4 claim by itself. |

## Planned artifact map

| Artifact name | Expected path | Future session use | What it does not prove on its own |
|---------------|---------------|--------------------|-----------------------------------|
| Simulated bridge artifact | `.tmp/hub/simulated-local-bridge-artifact.json` | Reference for stand-in or future bridge-field mirroring | Not a real HA entity registration |
| Local hub state snapshot | `.tmp/hub/local-hub-state-snapshot.json` | Reference for clean reboot or later power-cycle notes | Not full power-cycle survival by itself |
| Local onramp proposal artifact | `.tmp/hub/local-onramp-proposal.json` | Reference for proposal field mirroring | Not canonical acceptance or grant issuance |
| Local onramp decision receipt | `.tmp/hub/local-onramp-decision-receipt.json` | Reference for decision field mirroring | Not an executed consent event |
| Read-only runway summary | `/runway-summary.json` or the existing runway route | Reference for shell or log observation on the DUT | Not browser-issued write proof or remote transport |

## Operator notes

- Replace every placeholder before execution.
- Record the exact command actually used in the matching findings template.
- If a command or artifact path differs on the real DUT, record the difference rather than rewriting this map as evidence.

## Claim ceiling

- This file does not assert that any command has already been run on hardware.
- This file does not assert that any `.tmp/hub` artifact was produced on a real device.
- This file does not claim physical-device evidence, Home Assistant proof, D1 closure, G4 closure, or full power-cycle survival.