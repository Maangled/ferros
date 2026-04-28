# S4/S7 Reboot-Safe State Rehearsal Checklist

Status: Batch F research handoff. This is a rehearsal checklist for future D1/hardware work, not a completed reboot proof.

## Purpose

D1 requires FERROS-side state to remain understandable after reboot or restart. G4 later requires a real hub on physical hardware to survive a full power cycle. The current repo can rehearse the shape of that proof locally, but it cannot claim hardware recovery until a named session runs on a real device.

## Current Inputs

- S2 local profile and grant reload semantics.
- S3 local registry, lifecycle, log, and `agent.snapshot` observation surfaces.
- S4 node-local runtime reload helpers and current restart/reload boundary note.
- S5 lifecycle control bar and local shell harness.
- S7 power-cycle recovery protocol from Batch E.

## Rehearsal Sequence

| Phase | Action | Expected local observation |
|-------|--------|----------------------------|
| Baseline | Record selected profile path, visible `ProfileId`, selected agent, grant rows, and deny-log rows | Operator can identify the state being rehearsed |
| Local lifecycle | Run or stop one reference agent through the current local path | `agent.snapshot` reflects the changed local status |
| Restart rehearsal | Stop and restart the local shell/node process, then reload the same profile/state path | Profile and grant state reload; current limitations around runtime state are visible |
| Deny rehearsal | Attempt one denied action with missing grant coverage | Denial remains visible through current local inspection after refresh |
| Comparison | Compare before/after observations in a short operator note | Differences are named as current implementation gaps, not smoothed over |

## What Counts As A Gap

- Agent registration must be re-created manually or by current reference bootstrap rather than hub re-registration.
- Deny-log persistence depends on current local state path behavior, not a hub audit store.
- HA entity state is absent until S7 implementation exists.
- Power-cycle recovery is not proven without hardware.

## D1 Versus G4

| Question | D1 rehearsal answer | G4 answer |
|----------|---------------------|-----------|
| Can the operator see profile and consent state? | Rehearsable locally through S2/S5 surfaces | Must be shown on real hub flow |
| Can a denied request be visible? | Rehearsable through current deny surfaces | Must be shown in the HA/hub scenario |
| Can state survive process restart? | Partially rehearseable with current local state | Must survive device power cycle |
| Is launch ready? | No | Only after G4 evidence exists |

## Stop Lines

- Do not claim D1 or G4 evidence from this rehearsal checklist.
- Do not publish a broader S4 restart API.
- Do not hide missing `ferros-hub` or HA bridge implementation.
- Do not run hardware work until a human names the session window.
