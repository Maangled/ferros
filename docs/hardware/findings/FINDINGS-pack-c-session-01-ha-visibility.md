# Findings — Pack C Session 01 HA Visibility

> Filled from agent-executed commands on `homelab001` against the separate Pack C Home Assistant host `MKY` under explicit operator authorization from `Maangled`. Backend captures live under `.local-artifacts/pack-c-session-01-ha-visibility/`, and the entity was also confirmed in the authenticated Home Assistant Entities UI during the same operator-attended session.

## Scope

This findings packet captures only the first separate-host Home Assistant visibility slice for the current FERROS bridge stand-in path. It is intentionally separate from the Pack B local rehearsal and from any later reboot or launch-grade bridge validation.

## Claim ceiling

- This packet proves only that `ferros-hub remote-report-state` wrote the documented stand-in entity `sensor.ferros_bridge_probe` to the remote Home Assistant host and that `ferros-hub remote-summary` immediately observed that entity afterward.
- This packet also proves operator-visible presence of the same stand-in entity in the authenticated Home Assistant Entities admin UI.
- This packet does not authorize a claim of D1 closure, G4 closure, independent install evidence, full power-cycle survival, reboot restoration, or a launch-grade non-stubbed Home Assistant bridge entity.

## Session header

| Field | Value |
|-------|-------|
| Date | `2026-05-04` |
| Operator | `Maangled` |
| Pack B DUT name | `homelab001` |
| Pack C HA host name | `MKY` |
| Topology note | `ferros-hub` ran from `/home/homelab001/apps/ferros` on `homelab001` against the separate Windows 10 Pro build 26200 Home Assistant host at `http://192.168.50.194:8123`; the temporary HA host was running through Docker Desktop 29.4.1 and the session remained operator attended. |

## Command transcript

```text
cd /home/homelab001/apps/ferros
mkdir -p .local-artifacts/pack-c-session-01-ha-visibility
export FERROS_HA_URL='http://192.168.50.194:8123'
export FERROS_HA_TOKEN='[provided interactively and then unset after capture]'
cargo run -p ferros-hub -- remote-report-state > .local-artifacts/pack-c-session-01-ha-visibility/remote-report-state.txt
cargo run -p ferros-hub -- remote-summary > .local-artifacts/pack-c-session-01-ha-visibility/remote-summary.txt
```

## Entity or dashboard evidence reference

| Field | Value |
|-------|-------|
| Entity or dashboard reference | `sensor.ferros_bridge_probe` and later same-day bridge-state sync entity `sensor.ferros_ha_local_bridge_status` (`FERROS Bridge Probe` and `FERROS ha-local-bridge Status` in the HA Entities UI at `/config/entities`) |
| Capture path | `.local-artifacts/pack-c-session-01-ha-visibility/remote-report-state.txt`; `.local-artifacts/pack-c-session-01-ha-visibility/remote-summary.txt`; `.local-artifacts/pack-c-session-02-bridge-state-sync/remote-report-state.txt`; `.local-artifacts/pack-c-session-02-bridge-state-sync/remote-summary.txt` |
| Observation note | `remote-report-state` first captured `entityId: sensor.ferros_bridge_probe` with `state: report-state` and summary `authenticated remote Home Assistant report-state upsert wrote sensor.ferros_bridge_probe`. The paired first `remote-summary` capture reported `locationName: Home`, `version: 2026.4.4`, `ferrosEntityCount: 1`, and `ferrosEntities: sensor.ferros_bridge_probe`. Later on the same day, after the bridge-state sync refactor and a fresh bearer token, `remote-report-state` captured `entityId: sensor.ferros_ha_local_bridge_status` with `state: allowed`, and the paired `remote-summary` capture reported `ferrosEntityCount: 2` with `ferrosEntities: sensor.ferros_bridge_probe,sensor.ferros_ha_local_bridge_status`. In the authenticated HA Entities admin UI, filtering for `FERROS` showed both `FERROS Bridge Probe` with state `report-state` and `FERROS ha-local-bridge Status` with state `allowed` under the `Ungrouped` section. |

## Recovery observation

| Field | Value |
|-------|-------|
| DUT restart or cold-boot event attempted | `no` |
| HA-side recovery observation | `No reboot, cold-boot, or Home Assistant restart recovery proof was attempted in this slice.` |
| Notes | `This session was limited to separate-host entity visibility and operator-visible UI confirmation. It did not test profile reload after reboot, agent re-registration after restart, or persistence of the stand-in entity across any restart boundary.` |

## Remaining gaps

- D1 is not closed by this packet because the other required D1 evidence rows remain open, especially consent-flow visibility and reboot-safe FERROS-side state.
- G4 remains blocked because this packet uses a documented stand-in entity rather than a non-stubbed launch-grade Home Assistant bridge entity on physical hardware.
- Full DUT-only power-cycle and Home Assistant restoration behavior remain unobserved.
- This packet still does not widen D1 or G4 claims, but it can now serve as the canonical `HARDWARE-2026-04-30-07` findings packet because the required Pack B DUT rehearsal has since landed.
- The fresh-token bridge-state sync adds stronger live validation for the runtime-backed HA sync path, but it still does not satisfy the launch-grade consent-deny, power-cycle restoration, or non-stand-in G4 requirements.

## Non-claims for this template

- No claim of D1 or G4 closure.
- No claim of independent install evidence.
- No claim of full power-cycle survival or reboot restoration.
- No claim that `sensor.ferros_bridge_probe` is a non-stubbed launch-grade Home Assistant entity.