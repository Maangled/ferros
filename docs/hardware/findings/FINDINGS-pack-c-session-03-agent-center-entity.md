# Findings — Pack C Session 03 Agent-Center Entity

> Filled from agent-executed commands on `homelab001` against the separate Pack C Home Assistant host `MKY` under explicit operator authorization from `Maangled`. Backend captures live under `.local-artifacts/pack-c-session-03-agent-center-entity/`, and the updated entity was also confirmed in the authenticated Home Assistant Entities UI during the same operator-attended session.

## Scope

This findings packet captures only the first separate-host Home Assistant validation slice for the FERROS agent-center-backed bridge entity path. It is intentionally narrower than full G4 closure.

## Claim ceiling

- This packet proves that `ferros-hub remote-report-state` wrote `sensor.ferros_ha_local_bridge_status` with state `registered` to the separate Home Assistant host and that the saved HA state JSON now identifies that entity as sourced from `ferros-node-agent-center-state`, not the earlier local-only runway summary.
- This packet also proves operator-visible presence of the same entity in the authenticated Home Assistant Entities UI with state `registered`.
- This packet does not authorize G4 closure, consent-deny visibility in the HA UI, HA restoration after a full power cycle, or independent install proof.

## Session header

| Field | Value |
|-------|-------|
| Date | `2026-05-04` |
| Operator | `Maangled` |
| Pack B DUT name | `homelab001` |
| Pack C HA host name | `MKY` |
| Topology note | `ferros-hub` ran from `/home/homelab001/apps/ferros` on `homelab001` against the separate Windows 10 Pro build 26200 Home Assistant host at `http://192.168.50.194:8123`; the session remained operator attended. |

## Command transcript

```text
cd /home/homelab001/apps/ferros
mkdir -p .local-artifacts/pack-c-session-03-agent-center-entity
FERROS_HA_URL='http://192.168.50.194:8123' FERROS_HA_TOKEN='[provided interactively and then unset after capture]' cargo run -p ferros-hub -- remote-report-state > .local-artifacts/pack-c-session-03-agent-center-entity/remote-report-state.txt
FERROS_HA_URL='http://192.168.50.194:8123' FERROS_HA_TOKEN='[provided interactively and then unset after capture]' cargo run -p ferros-hub -- remote-summary > .local-artifacts/pack-c-session-03-agent-center-entity/remote-summary.txt
curl -s -H 'Authorization: Bearer [provided interactively and then unset after capture]' http://192.168.50.194:8123/api/states/sensor.ferros_ha_local_bridge_status > .local-artifacts/pack-c-session-03-agent-center-entity/remote-entity-state.json
```

## Entity evidence reference

| Field | Value |
|-------|-------|
| Entity reference | `sensor.ferros_ha_local_bridge_status` (`FERROS ha-local-bridge Status` in the HA Entities UI at `/config/entities`) |
| Capture path | `.local-artifacts/pack-c-session-03-agent-center-entity/remote-report-state.txt`; `.local-artifacts/pack-c-session-03-agent-center-entity/remote-summary.txt`; `.local-artifacts/pack-c-session-03-agent-center-entity/remote-entity-state.json` |
| Observation note | `remote-report-state` captured `entityId: sensor.ferros_ha_local_bridge_status` with `state: registered`. The saved HA state JSON captured `scope: local-agent-center`, `evidence: persisted-agent-center-runtime`, `state_source: ferros-node-agent-center-state`, `bridge_agent: ha-local-bridge`, `bridge_manifest_identity: ha-local-bridge@0.1.0`, `required_capabilities: [hub-local-bridge:bridge.observe]`, and no `stand_in_name` field. The paired `remote-summary` capture reported `ferrosEntityCount: 2` with `ferrosEntities: sensor.ferros_bridge_probe,sensor.ferros_ha_local_bridge_status`. In the authenticated HA Entities UI, filtering for `FERROS` showed `FERROS Bridge Probe` with state `report-state` and `FERROS ha-local-bridge Status` with state `registered` under the `Ungrouped` section. |

## Remaining gaps

- This packet does not prove consent-deny visibility in the Home Assistant UI; that remains a separate G4 requirement.
- This packet does not prove HA entity restoration after a full DUT-only power cycle.
- This packet does not prove independent install.
- G4 remains open.

## Non-claims for this template

- No G4 closure.
- No HA restoration-after-power-cycle claim.
- No independent install proof.