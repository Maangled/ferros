# Findings — Pack C Session 05 HA Restoration

> Filled from agent-executed commands on `homelab001` against the separate Pack C Home Assistant host `MKY` under explicit operator authorization from `Maangled`. Backend captures live under `.local-artifacts/pack-c-session-05-ha-restoration/`.

## Scope

This findings packet captures the first full-power-cycle restoration attempt for the current agent-center-backed bridge entity path.

## Claim ceiling

- This packet proves that after the DUT-only reboot, the FERROS profile still loaded and the local bridge agent still re-registered on `homelab001`.
- This packet also proves that the Home Assistant bridge entity was still present on `MKY` before any manual resync, but its `last_updated` timestamp remained pre-boot, so the current path did not restore the entity automatically after reboot.
- This packet does not authorize G4 closure.

## Session header

| Field | Value |
|-------|-------|
| Date | `2026-05-04` |
| Operator | `Maangled` |
| Pack B DUT name | `homelab001` |
| Pack C HA host name | `MKY` |
| DUT boot time | `2026-05-04 03:17:12` |

## Local FERROS recovery result

| Field | Value |
|-------|-------|
| Profile command | `cargo run -p ferros-node --bin ferros -- profile show .local-state/pack-b-session-01-profile.json` |
| Profile exit result | `0` |
| Agent list command | `cargo run -p ferros-node --bin ferros -- agent list` |
| Agent list exit result | `0` |
| Agent rows visible | `echo`, `ha-local-bridge`, `timer` all `registered` |
| Bridge describe command | `cargo run -p ferros-node --bin ferros -- agent describe ha-local-bridge` |
| Bridge describe exit result | `0` |
| Artifact references | `.local-artifacts/pack-c-session-05-ha-restoration/post-power-profile-show.txt`; `.local-artifacts/pack-c-session-05-ha-restoration/post-power-agent-list.txt`; `.local-artifacts/pack-c-session-05-ha-restoration/post-power-agent-describe-ha-local-bridge.txt` |

## Home Assistant restoration observation

| Field | Value |
|-------|-------|
| Read-only check before manual resync | `cargo run -p ferros-hub -- remote-summary`; authenticated `GET /api/states/sensor.ferros_ha_local_bridge_status` |
| Entity present before manual resync | `yes` |
| Pre-resync HA timestamps | `last_updated` and `last_reported` were both `2026-05-04T03:03:36.156712+00:00`, which is earlier than the DUT boot time `2026-05-04 03:17:12` |
| Automatic restoration satisfied | `no` |
| Manual repair check | After a post-boot `cargo run -p ferros-hub -- remote-report-state`, the HA entity `last_updated` advanced to `2026-05-04T03:20:10.221892+00:00`, proving the bridge path still worked when manually resynced but was not restored automatically |
| Artifact references | `.local-artifacts/pack-c-session-05-ha-restoration/post-power-remote-summary-before-sync.txt`; `.local-artifacts/pack-c-session-05-ha-restoration/post-power-remote-entity-state-before-sync.json`; `.local-artifacts/pack-c-session-05-ha-restoration/post-power-remote-report-state.txt`; `.local-artifacts/pack-c-session-05-ha-restoration/post-power-remote-entity-state-after-sync.json` |

## Root-cause hint from saved artifacts

| Field | Value |
|-------|-------|
| Pre-resync entity attributes | `agent_center_state_present: true`, `denied_start_count: 1`, `latest_deny_event: missing bridge.observe` |
| Post-resync entity attributes | `agent_center_state_present: false`, `denied_start_count: 0`, `latest_deny_event: null` |
| Observation note | `The current post-boot manual sync path still writes the bridge entity, but it did so without a persisted local agent-center state file after reboot, which is consistent with the current temporary-state-based host path and does not satisfy the G4 no-manual-intervention restoration requirement.` |

## Remaining gaps

- G4 power-cycle restoration remains open because the entity was not restored automatically after reboot.
- Independent install remains open.

## Non-claims for this template

- No G4 closure.
- No independent install proof.