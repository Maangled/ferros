# S7 HA Bridge Seam Catalog

Status: runway only. This note does not claim a landed HA bridge, D1 evidence, or G4 evidence.

## Purpose

Catalog the local-only seams a future `ferros-hub` or HA bridge must consume before FERROS can honestly publish a reboot-safe Home Assistant path. This is a planning surface for the current G4 and D1 runway, not an implementation contract.

## Reusable local seams

- Agent registry observation already exists through the local `ferros agent list`, `ferros agent describe`, and localhost shell read surfaces.
- Agent lifecycle state already reads back through `agent.snapshot` plus the local-only `agent.run` and `agent.stop` path.
- Grant visibility already reads through the current local profile path and deny-by-default policy evaluation.
- The new `/runway-summary.json` shell route exposes a local-only observation bundle for profile, deny, and stand-in agent readiness without widening JSON-RPC.

## Consent boundaries

- Registration is not consent. A bridge may discover or stage material, but runtime execution still depends on an explicit grant.
- Any Home Assistant entity or external material remains proposed FERROS material until a FERROS-owned acceptance surface moves it across the boundary.
- Grant creation, grant revoke, remote bridge control, and external authority remain out of scope for this note.

## Reboot and replay hooks

- `LocalProfileStore::load_local_profile(path)` is the current local profile reload seam.
- `CliState::load(path)` is the current local runtime-state replay seam.
- `runtime_with_state(state_path)` is the current local runtime rebuild seam.
- Power-cycle proof remains pending. Current restart-safe seams are local reload helpers only, not hardware evidence.

## Missing honest implementation seams

- No real `ferros-hub` binary exists yet.
- No published HA bridge transport or pairing choreography exists yet.
- No reboot-safe local storage proof exists for the future bridge path.
- No hardware-backed D1 or G4 evidence is recorded from this note.

## Feed-forward hooks

- S5 can consume the local runway summary surface to stage an operator-readable launch checklist without widening privileges.
- S6 can keep local push and audit envelopes local-only until a real bridge artifact flow exists.
- S7 should treat power-cycle and re-registration as separate proofs from local process restart.# S7 HA Bridge Seam Catalog

Status: Batch F research handoff. This catalog names the current seams a future HA bridge must consume before `crates/ferros-hub/` exists.

## Current Seam Inventory

| Seam | Owner | Current truth | Bridge relevance |
|------|-------|---------------|------------------|
| Device/profile identity | S2 | Local profile and grant state can reload and verify against frozen v0 contracts | Bridge must bind to local `ProfileId` and active grants |
| Agent registration | S3 | `AgentRegistry` plus local `agent.list` / `agent.describe` observation exist | Bridge must be a visible registered agent |
| Lifecycle control | S3/S4 | Local-only `agent.run` / `agent.stop` exists through the current shell host | Bridge control can plan around local lifecycle verbs only |
| Deny visibility | S3/S4/S5 | Local deny summaries and `denyLog.list` are visible through the shell path | Bridge deny must be operator-visible before evidence is honest |
| Restart/reload | S4 | Node-local reload helpers exist; durable hub restart API is unpublished | Bridge cannot yet claim re-registration after power loss |
| Onramp consent | S5/ADR-023 | External material must be quarantined until accepted | HA entities arrive as proposed material, not identity truth |
| Prior-art boundary | S6 | HA fork is reference-scoped; ADRs govern direct harvest | Bridge design must not bulk-port HA fork internals |

## D1 Stand-In Boundary

D1 may use a named stand-in only if the evidence wave says exactly what is standing in for the missing HA bridge surface. A stand-in can help rehearse operator flow, but it cannot become G4 evidence and cannot hide missing `ferros-hub`, HA bridge, or physical-device work.

## Missing Before Real Bridge Implementation

1. A `ferros-hub` crate or binary boundary.
2. A bridge agent manifest and registration wrapper.
3. A real HA connection shape owned by the S7 implementation wave.
4. Restart and re-registration behavior that survives device power cycle.
5. Operator-visible deny attribution in the hub scenario.
6. A hardware session window with named device and HA host.

## Escalation Rules

- If bridge planning needs a new S3 lifecycle wrapper, queue that as S3-owned work.
- If bridge planning needs durable restart semantics, queue that as S4-owned work.
- If bridge planning needs profile/grant fields beyond the frozen v0 contracts, stop and escalate rather than editing schemas.
- If bridge planning starts producing actual hardware observations, switch to hardware track and use the parked hardware queue.

## Stop Lines

- Do not scaffold `crates/ferros-hub/` from this catalog.
- Do not freeze pairing order.
- Do not claim HA bridge execution or G4 evidence.
- Do not edit `docs/gates/G4.md`.
