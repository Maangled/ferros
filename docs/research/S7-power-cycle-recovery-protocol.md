# S7 Research Note - Power-Cycle Recovery Protocol

**Date:** 2026-04-28
**Owning stream:** S7 primary; S1 and S4 consumer awareness
**Output feeds:** D1 UX session script; hardware-session planning
**Status:** Protocol draft only. No hardware findings, D1 evidence, or G4 evidence are claimed.

---

## Purpose

This note composes the S1 boot-sequence input and the S4 restart/reload boundary into a D1-ready power-cycle recovery protocol. It is a scriptable protocol for a future human/device session, not a record of a completed session.

---

## Recursion Controller Decision

This lane was the only Batch E recursion candidate because it composes S1 boot readiness, S4 reload behavior, and S7 D1/G4 power-cycle expectations.

**Decision:** recursion denied.

**Reason:** the safe write scope stays in one S7-owned anchor file after the S1 and S4 input notes land. A child plan would add coordination cost without creating a new owner boundary or anchor set. Shared truth surfaces remain out of scope.

---

## Inputs

| Input | Role |
|---|---|
| `docs/research/S1-boot-sequence-d1-target.md` | Defines pre-reboot baseline and S1/session-owned boot checks |
| `docs/research/S4-restart-reload-boundary.md` | Defines reload/process/reboot/power-cycle distinctions |
| `docs/gates/D1.md` | Defines D1 evidence requirements |
| `docs/hub/pack-b-bring-up-worksheet.md` | Provides the operator worksheet shape |
| `docs/hub/reference-hardware.md` | Hardware runway authority |

---

## Preconditions

Before running this protocol, the operator must have:

- named the target device or stand-in;
- chosen a persistent profile/state path;
- confirmed the boot command or binary path;
- confirmed how to remove and restore only the device-under-test power;
- opened an artifact capture location for terminal output and timestamps;
- confirmed that D1 is being rehearsed, not closed.

If any precondition is missing, file a finding or update the hardware queue instead of running an evidence session.

---

## Protocol Steps

### 1. Baseline before power cut

Run and capture:

```bash
ferros profile show
ferros agent list
ferros agent logs echo
```

Record:

- profile id;
- agent rows visible;
- any deny-log entry used for consent-flow visibility;
- state path and binary/build reference.

### 2. Clean process stop, if applicable

If FERROS is running as a manual shell process, stop it cleanly and record the command. If no long-running process is active, record that the current path is CLI-only.

Do not call this a supervised service.

### 3. Full power cycle

Remove power from the device under test only. Do not restart the separate HA host if one is present.

Record:

- method used;
- power-off time;
- power-on time;
- first sign the device is reachable again.

### 4. Post-boot checks

After the target is reachable, run:

```bash
ferros profile show
ferros agent list
ferros agent logs echo
```

If the localhost shell is part of the session, start it manually and refresh `agent.snapshot`.

### 5. Compare

| Check | Passing D1 rehearsal result |
|---|---|
| Profile id | Same as pre-cycle |
| Profile path | Persistent path, not temp state |
| Agent list | At least one agent visible after boot/start path |
| Deny visibility | Existing or newly seeded denial is visible through CLI or shell |
| Operator attendance | Operator remained present and recorded steps |

---

## What Counts for D1

D1 can accept:

- operator-attended manual startup;
- one target device or x86_64 stand-in;
- a named HA bridge stand-in if the real bridge is absent;
- FERROS-side profile and agent state recovery after a power cycle.

D1 cannot accept:

- a claim that HA re-registered after reboot unless that actually happened;
- production supervisor claims;
- G4 launch language;
- evidence reconstructed from memory rather than session artifacts.

---

## What Remains G4-Only

| Requirement | Why it is G4-only |
|---|---|
| `ferros-hub` launch binary | Not built yet |
| Real HA entity restored after power cycle | D1 only requires FERROS-side state |
| Independent private-beta install | Launch criterion |
| Unattended recovery | D1 is operator-attended |
| Reference hardware finalization | Encouraged for D1, required for G4 |

---

## Known Unknowns

- Which physical device or stand-in will be named for the first session.
- Whether the state path on that device survives full power loss.
- Whether an HA stand-in or real bridge path will be available.
- Whether any later S3/S4 API change alters the agent re-registration check.
- Whether the operator will start from a prebuilt binary or `cargo run`.

---

## Findings Handoff

Actual session results belong under `docs/hardware/findings/` or the D1 evidence table only after a human/device session. This protocol file should not be edited to become evidence; it is the script that produces evidence elsewhere.

