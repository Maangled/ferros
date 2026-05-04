# Findings — Pack C Session 04 Consent Deny

> Filled from agent-executed commands on `homelab001` against the separate Pack C Home Assistant host `MKY` under explicit operator authorization from `Maangled`. Backend captures live under `.local-artifacts/pack-c-session-04-consent-deny/`.

## Scope

This findings packet captures only the launch-grade consent-deny slice for the current FERROS bridge entity path.

## Claim ceiling

- This packet proves that a capability request for `ha-local-bridge` was denied on the FERROS agent-center path, that the denial was visible through `ferros agent logs`, and that the same denial metadata was synchronized into the remote Home Assistant entity attributes.
- This packet does not authorize G4 closure, HA restoration-after-power-cycle proof, or independent install proof.

## Session header

| Field | Value |
|-------|-------|
| Date | `2026-05-04` |
| Operator | `Maangled` |
| Pack B DUT name | `homelab001` |
| Pack C HA host name | `MKY` |
| Entity reference | `sensor.ferros_ha_local_bridge_status` |

## Command transcript

```text
cd /home/homelab001/apps/ferros
mkdir -p .local-artifacts/pack-c-session-04-consent-deny
cargo run -p ferros-node --bin ferros -- agent run ha-local-bridge > .local-artifacts/pack-c-session-04-consent-deny/agent-run-ha-local-bridge.txt
cargo run -p ferros-node --bin ferros -- agent logs ha-local-bridge > .local-artifacts/pack-c-session-04-consent-deny/agent-logs-ha-local-bridge.txt
FERROS_HA_URL='http://192.168.50.194:8123' FERROS_HA_TOKEN='[provided interactively and then unset after capture]' cargo run -p ferros-hub -- remote-report-state > .local-artifacts/pack-c-session-04-consent-deny/remote-report-state.txt
curl -s -H 'Authorization: Bearer [provided interactively and then unset after capture]' http://192.168.50.194:8123/api/states/sensor.ferros_ha_local_bridge_status > .local-artifacts/pack-c-session-04-consent-deny/remote-entity-state.json
```

## Denied request result

| Field | Value |
|-------|-------|
| Command used | `cargo run -p ferros-node --bin ferros -- agent run ha-local-bridge` |
| Exit result | `1` |
| Observed deny note | `authorization denied: ha-local-bridge missing bridge.observe` |
| Artifact reference | `.local-artifacts/pack-c-session-04-consent-deny/agent-run-ha-local-bridge.txt` |

## FERROS log result

| Field | Value |
|-------|-------|
| Command used | `cargo run -p ferros-node --bin ferros -- agent logs ha-local-bridge` |
| Exit result | `0` |
| Observed log note | `denied-start:ha-local-bridge missing bridge.observe` |
| Artifact reference | `.local-artifacts/pack-c-session-04-consent-deny/agent-logs-ha-local-bridge.txt` |

## Home Assistant entity result

| Field | Value |
|-------|-------|
| Command used | `cargo run -p ferros-hub -- remote-report-state` plus authenticated `GET /api/states/sensor.ferros_ha_local_bridge_status` |
| Entity state | `registered` |
| Synced deny metadata | `denied_start_count: 1`; `latest_deny_event: missing bridge.observe` |
| Artifact reference | `.local-artifacts/pack-c-session-04-consent-deny/remote-report-state.txt`; `.local-artifacts/pack-c-session-04-consent-deny/remote-entity-state.json` |

## Remaining gaps

- This packet does not prove HA entity restoration after a full DUT-only power cycle.
- This packet does not prove independent install.
- G4 remains open.

## Non-claims for this template

- No G4 closure.
- No HA restoration-after-power-cycle claim.
- No independent install proof.