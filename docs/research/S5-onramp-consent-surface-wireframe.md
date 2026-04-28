# S5 Onramp Consent Surface Wireframe

Status: Batch F research handoff. This turns ADR-023 into a concrete S5 surface shape without wiring external calls or accepting proposed material.

## Governing Rule

ADR-023 says external systems are onramps, not identity truth. Inbound material is quarantined until a FERROS operator explicitly accepts it, and rejection must be auditable.

## Surface Shape

The first onramp consent surface should be an inbox-style work area inside the localhost shell:

| Region | Content |
|--------|---------|
| Source rail | Source system name, source type, and local arrival time |
| Proposal list | Proposed items grouped by source, with status `pending`, `accepted`, or `rejected` |
| Detail pane | Proposed item summary, fields supplied by the source, and fields FERROS would create if accepted |
| Consent actions | Accept and reject affordances, both explicitly operator-triggered |
| Audit strip | Last decision, operator-visible reason, and current quarantine state |

## First Sources To Model

| Source | Why it matters | First treatment |
|--------|----------------|-----------------|
| Home Assistant entity | Feeds S7 bridge planning and D1/G4 runway | Proposed material only; not a FERROS identity source |
| Imported profile bundle | Feeds S2 recovery UX | Local file import result only; not cloud sync |
| Future asset/library item | Feeds S6/S8 planning | Quarantined proposed material until accepted |

## Decision States

| State | Meaning | UI requirement |
|-------|---------|----------------|
| Pending | FERROS has seen proposed material but has not accepted it | Show source and proposed fields without mutating canonical state |
| Accepted | Operator chose to create or update FERROS-owned state | Show accepted timestamp and resulting FERROS target |
| Rejected | Operator declined the proposal | Keep enough local audit context for operator review |
| Stale | Proposal cannot be applied because the target contract moved or source vanished | Require refresh/re-review rather than silent apply |

## Implementation Notes For Later

- The first implementation should use local fixture/proposed-material inputs before any external bridge call.
- Accept/reject persistence must be explicitly scoped before code lands.
- Consent copy that depends on legal posture stays draft until counsel review clears the text.
- HA-specific proposals must not freeze HA fork internals or pairing order.

## Stop Lines

- Do not ingest external material directly into canonical state.
- Do not add HA bridge transport from this wireframe.
- Do not promote ADR-024 or introduce ledger finality claims.
- Do not claim D1 consent evidence from a wireframe.
