# AUTHORITY_ACK Template

Use this template when authority mismatch is detected after a run begins and a typed interruption decision is required.

Authority: docs/orchestration/AUTHORITY-MAP.md
Contract: docs/orchestration/AUTHORITY-INTERRUPTION.md

```yaml
authority_ack:
  ack_id: "ACK-<YYYYMMDD>-<N>"
  run_id: "FRS-<stream-or-family>-<YYYYMMDD>-C<N>-W<N>"
  detected_at: "YYYY-MM-DD"
  mismatch_summary: "<what drifted and where>"
  decision: "continue-current-state|continue-but-freeze-new-lanes|refresh-authority-and-resume|abort-and-reissue"
  lane_expansion_frozen: true|false
  scope_limit: "<bounded allowed scope while mismatch remains>"
  expiry: "C<N> or timestamp"
  approved_by: "<operator identity>"
  coordinator: "FERROS Agent"
  follow_up_required: true|false
  follow_up_action: "<refresh packet|reissue kickoff|abort run|none>"
```

## Usage notes

- This template does not replace gatekeeper decisions.
- Gatekeeper decision enum remains: `continue | stop-clean | stop-escalate`.
- Use this template only for authority interruption decisions.
- Store the completed record in truth-sync output for the active run.
