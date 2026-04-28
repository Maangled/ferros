# S3 Deny-Log UX And Error-Envelope Seam

Status: Batch F research handoff. This document names what S5 can safely render today from the S3 local deny and write-error surfaces.

## Current Surfaces

| Surface | Current role | S5 may display |
|---------|--------------|----------------|
| `denyLog.list` | Read-first local deny-log observation | Denied local lifecycle attempts and agent-filtered deny rows |
| `agent.snapshot` | Aggregated local shell refresh | Agent records, grant rows, and deny-log rows in one read path |
| `agent.run` / `agent.stop` error envelope | Local-only lifecycle write result | Immediate backend denial summary for the operator |
| `ferros agent logs` | Local CLI inspection | Operator fallback outside the browser shell |

The current surfaces are localhost-only and local-state-backed. They are not a remote-control or HA-facing error contract.

## Renderable Deny Story

S5 should compose the deny story from existing fields and avoid inventing a shadow audit model:

1. Action attempted: selected agent plus intended method.
2. Gate status before write: unarmed, missing active grant, or ready to send.
3. Backend outcome after write: success or denial from the local-only RPC response.
4. Read-after-write observation: refreshed `agent.snapshot` rows and `denyLog.list` entries.
5. Operator next step: add or inspect grants through the current local S2/S3 surfaces, not through an unimplemented browser mutation path.

## Error Copy Rules

- Prefer specific missing-capability wording when the current local API exposes it.
- Preserve stable local deny summaries when detail is unavailable.
- Treat unknown agents and invalid params as operator-correctable local input problems.
- Treat unsupported methods as a product boundary, not a runtime failure.
- Do not promise that a browser-visible denial has been synchronized to a remote hub or HA dashboard.

## Candidate UI Mapping

| Backend signal | UI state |
|----------------|----------|
| Missing required capability before send | Blocked control plus missing-capability explanation |
| Operator has not armed action | Blocked control plus arm-required explanation |
| Backend authorization denial | Error state plus refreshed deny-log row |
| Unknown agent | Selection stale state plus refresh prompt |
| Unsupported method | Internal shell mismatch state; wave should halt if introduced by S5 |

## Stop Lines

- Do not add new S3 methods from this handoff.
- Do not widen JSON-RPC transport beyond localhost.
- Do not claim a shared remote error-envelope contract.
- Do not edit D1/G4 evidence surfaces.
- Do not convert deny logs into user consent records without an explicitly scoped consent/audit wave.
