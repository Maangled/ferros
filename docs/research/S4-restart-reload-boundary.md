# S4 Research Note - Restart and Reload Boundary

**Date:** 2026-04-28
**Owning stream:** S4 primary; S7 consumer awareness
**Output feeds:** S7 power-cycle recovery protocol; later runtime hardening
**Status:** Boundary spec only. No new runtime API or D1/G4 evidence is claimed.

---

## Purpose

This note records the current restart/reload boundary in plain terms. S4 already has reload helpers and tests that prove local profile/grant reload plus fixed reference-runtime state replay. That is useful for D1 planning, but it is not a durable hub restart or re-registration contract.

---

## Published Today

| Helper or surface | What it proves today | Source |
|---|---|---|
| `LocalProfileStore::load_local_profile(path)` | Local profile, keypair, and signed grants can reload and validate from a path | `streams/S4-runtime/CONTRACTS.md`; S2 progress |
| `CliState::load(path)` | Current local CLI state can load strict `status` and `log` lines, defaulting empty when missing | `streams/S4-runtime/CONTRACTS.md` |
| `runtime_with_state(state_path)` | A fixed reference runtime can replay current local agent state for echo/timer style rows | `streams/S4-runtime/CONTRACTS.md` |
| `agent.snapshot` | The localhost shell can observe agent rows, grant state, and deny log in one read | S3/S5 docs |
| Policy tests | Deny-by-default reasons remain deterministic | `docs/research/S4-policy-engine-invariants.md` |

These surfaces are implementation-backed local seams, not a launch service.

---

## Restart Levels

| Level | Definition | Current status |
|---|---|---|
| Reload | Re-read local profile/grant and CLI state from disk without rebooting the device | Partially implementation-backed |
| Process restart | Stop the local FERROS process and start it again using the same state path | Planned by operator script, not a published service contract |
| Clean reboot | OS reboot; filesystem stays healthy; operator can start FERROS again | D1-relevant, session-owned |
| Full power cycle | Device power removed and restored; persistent state must survive | D1/G4-relevant, hardware-owned |
| Hub re-registration | Bridge agent returns after restart/power cycle as part of hub process | Not published today |
| HA recovery | Home Assistant entity is restored without manual intervention | G4 only; not D1 |

---

## Boundary Rules

1. **Reload is not reboot.** A helper that loads state from a path does not prove that a physical device survives a power cut.
2. **Reference-runtime replay is fixed-scope.** The current replay path covers the current local reference runtime, not arbitrary bridge agents.
3. **Profile/grant validation is S2-owned.** S4 consumes validated local profile and grant state; it does not publish a new profile file format.
4. **Deny behavior is runtime-backed.** Denied lifecycle attempts should remain observable through the same S3/S5 deny-log path after a reload.
5. **Hub restart remains unpublished.** No durable hub-facing restart API exists yet.

---

## D1-Ready Interpretation

D1 may rely on the current reload boundary as a planning input:

- before reboot, record `ferros profile show`, `ferros agent list`, and deny-log state;
- after reboot, record the same commands;
- compare profile id and visible agent rows;
- treat discrepancies as findings, not as evidence closure.

This is enough to script a D1 rehearsal. It is not enough to close D1 until run on the named target device.

---

## What Remains Unpublished

| Missing surface | Owning stream |
|---|---|
| Hub process startup/restart choreography | S7/S4 |
| Agent re-registration contract for non-reference bridge agents | S3/S7 |
| Persistent hub storage layout | S7/S2/S4 |
| Service manager or installer | S1/S7 |
| HA entity restoration after power cycle | S7 |

---

## Validation Trail to Reuse

When this boundary is later implemented or expanded, use focused validation that matches the claim:

| Claim | Expected validation |
|---|---|
| Profile/grant reload | Focused `ferros-profile` reload tests |
| Local agent state replay | Focused `ferros-node` reload/runtime tests |
| Shell observation after state change | `agent.snapshot`, `agent.describe`, and `denyLog.list` checks |
| Process restart | Start/stop process transcript with same state path |
| Full power cycle | Hardware session artifact, not a unit test |

---

## Handoff to S7

S7 should consume this note as the restart/reload input for the power-cycle recovery protocol. The protocol should preserve the distinction between:

- local state reload already proven in code;
- device restart that must be captured in a session;
- hub/HA recovery still unpublished and out of D1 scope.

