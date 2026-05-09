# FERROS Authority Interruption Contract

> **Canonical authority.** This document defines what happens when authority drift or version-lock mismatch is discovered after a run has already started.
> See [AUTHORITY-MAP.md](AUTHORITY-MAP.md) for the full document index.

---

## Purpose

FERROS runs fail closed on malformed routing and policy violations.

This contract covers the narrower case where a run is already in flight and authority mismatch is discovered mid-run. The objective is to avoid two unsafe extremes:

- blindly continuing under drifted authority,
- hard-stopping without a resumable decision artifact.

---

## Owner and approval model

- **Coordinator owner:** FERROS Agent.
- **Operator role:** approve one bounded decision from the typed enum below.
- **Execution agents:** must honor the coordinator decision and associated constraints.

---

## Authority interruption decision enum

The only valid authority-mismatch decisions are:

1. `continue-current-state`
2. `continue-but-freeze-new-lanes`
3. `refresh-authority-and-resume`
4. `abort-and-reissue`

This enum is separate from gatekeeper decisions (`continue | stop-clean | stop-escalate`) and does not replace them.

---

## Trigger conditions

Open an authority interruption when any of these is true:

- version-lock marker mismatch on canonical orchestration docs,
- route-token schema mismatch across packets or domains,
- policy reference drift where packet assumptions no longer match canonical wording,
- mid-run handoff requires expansion but current authority snapshot is stale.

---

## Required authority_ack record

Every interruption decision must produce an `authority_ack` artifact recorded in truth-sync output.

Use template:
- `docs/orchestration/AUTHORITY-ACK.template.md`

Required fields:

```yaml
authority_ack:
  ack_id: "ACK-<YYYYMMDD>-<N>"
  run_id: "FRS-..."
  detected_at: "YYYY-MM-DD"
  mismatch_summary: "..."
  decision: "continue-current-state|continue-but-freeze-new-lanes|refresh-authority-and-resume|abort-and-reissue"
  lane_expansion_frozen: true|false
  scope_limit: "..."
  expiry: "C<N> or timestamp"
  approved_by: "operator identity"
  coordinator: "FERROS Agent"
  follow_up_required: true|false
```

---

## Checkpoint schedule

Authority checks must run at these boundaries:

1. kickoff preflight,
2. before opening new lane expansion,
3. before shared truth-sync writes,
4. before promotion/retirement decisions for agent families.

When a mismatch appears at a checkpoint, pause and apply this contract before state expansion proceeds.

---

## Route-token normalization

Use these semantics across coding and business packets:

- `route_token.target_stream` means execution stream only (`core` or `subcore` when applicable).
- `route_token.target_family` means execution family (`coding`, `business`, or `architect`).
- `track` remains queue-track scope (`code`, `system`, `hardware`) and is not a stream identifier.

Do not overload `target_stream` with agent names or domain-family identities.

---

## Response requirements when interruption opens

FERROS Agent must return:

1. mismatch summary,
2. decision options constrained to the enum,
3. selected decision,
4. authority_ack fields,
5. resume or abort steps,
6. residual risks and explicit non-claims.

When emitting interruption output, include a filled `authority_ack` block using the template contract.

---

*Last updated: 2026-05-09*
