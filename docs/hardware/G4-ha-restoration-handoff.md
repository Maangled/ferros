# G4 HA Restoration Handoff

> Operator-run handoff for the remaining G4 restoration proof. This note exists so the current agent-center-backed bridge entity path can be exercised across a full DUT-only power cycle without losing the capture plan.

## Scope

This handoff covers only the G4 requirement that after a full power cycle on `homelab001`, the FERROS profile loads, agents re-register, and the Home Assistant bridge entity is restored without manual intervention.

Already satisfied before this handoff:

- real agent-center-backed HA entity proof in `docs/hardware/findings/FINDINGS-pack-c-session-03-agent-center-entity.md`
- launch-grade consent-deny proof in `docs/hardware/findings/FINDINGS-pack-c-session-04-consent-deny.md`

Still required after this handoff:

- one full DUT-only power cycle on `homelab001`
- post-boot proof that `.local-state/pack-b-session-01-profile.json` still loads
- post-boot proof that `echo`, `ha-local-bridge`, and `timer` re-register
- post-boot proof that `sensor.ferros_ha_local_bridge_status` is restored on `MKY` without manual repair of the entity path

## Pre-power capture

Run on `homelab001` before pulling power:

```text
cd /home/homelab001/apps/ferros
mkdir -p .local-artifacts/pack-c-session-05-ha-restoration
date -Is | tee .local-artifacts/pack-c-session-05-ha-restoration/pre-power-time.txt
hostname | tee .local-artifacts/pack-c-session-05-ha-restoration/pre-power-hostname.txt
uname -srmo | tee .local-artifacts/pack-c-session-05-ha-restoration/pre-power-uname.txt
cargo run -p ferros-node --bin ferros -- profile show .local-state/pack-b-session-01-profile.json | tee .local-artifacts/pack-c-session-05-ha-restoration/pre-power-profile-show.txt
cargo run -p ferros-node --bin ferros -- agent list | tee .local-artifacts/pack-c-session-05-ha-restoration/pre-power-agent-list.txt
cargo run -p ferros-node --bin ferros -- agent describe ha-local-bridge | tee .local-artifacts/pack-c-session-05-ha-restoration/pre-power-agent-describe-ha-local-bridge.txt
FERROS_HA_URL='http://192.168.50.194:8123' FERROS_HA_TOKEN='[provide fresh token at execution time]' cargo run -p ferros-hub -- remote-report-state | tee .local-artifacts/pack-c-session-05-ha-restoration/pre-power-remote-report-state.txt
FERROS_HA_URL='http://192.168.50.194:8123' FERROS_HA_TOKEN='[provide fresh token at execution time]' cargo run -p ferros-hub -- remote-summary | tee .local-artifacts/pack-c-session-05-ha-restoration/pre-power-remote-summary.txt
curl -s -H 'Authorization: Bearer [provide fresh token at execution time]' http://192.168.50.194:8123/api/states/sensor.ferros_ha_local_bridge_status > .local-artifacts/pack-c-session-05-ha-restoration/pre-power-remote-entity-state.json
```

## Hard power boundary

- Keep the separate Pack C Home Assistant host `MKY` running.
- Remove power from `homelab001` only.
- Record the power-off time and the first time the host is reachable again.
- This handoff is for the full DUT-only power cut; a clean reboot does not satisfy the G4 restoration row.

## Post-power capture

After `homelab001` is reachable again, run:

```text
cd /home/homelab001/apps/ferros
date -Is | tee .local-artifacts/pack-c-session-05-ha-restoration/post-power-time.txt
hostname | tee .local-artifacts/pack-c-session-05-ha-restoration/post-power-hostname.txt
uname -srmo | tee .local-artifacts/pack-c-session-05-ha-restoration/post-power-uname.txt
uptime -s | tee .local-artifacts/pack-c-session-05-ha-restoration/post-power-boot-time.txt
ip -brief address show enp4s0 | tee .local-artifacts/pack-c-session-05-ha-restoration/post-power-ip.txt
cargo run -p ferros-node --bin ferros -- profile show .local-state/pack-b-session-01-profile.json | tee .local-artifacts/pack-c-session-05-ha-restoration/post-power-profile-show.txt
cargo run -p ferros-node --bin ferros -- agent list | tee .local-artifacts/pack-c-session-05-ha-restoration/post-power-agent-list.txt
cargo run -p ferros-node --bin ferros -- agent describe ha-local-bridge | tee .local-artifacts/pack-c-session-05-ha-restoration/post-power-agent-describe-ha-local-bridge.txt
FERROS_HA_URL='http://192.168.50.194:8123' FERROS_HA_TOKEN='[provide fresh token at execution time]' cargo run -p ferros-hub -- remote-report-state | tee .local-artifacts/pack-c-session-05-ha-restoration/post-power-remote-report-state.txt
FERROS_HA_URL='http://192.168.50.194:8123' FERROS_HA_TOKEN='[provide fresh token at execution time]' cargo run -p ferros-hub -- remote-summary | tee .local-artifacts/pack-c-session-05-ha-restoration/post-power-remote-summary.txt
curl -s -H 'Authorization: Bearer [provide fresh token at execution time]' http://192.168.50.194:8123/api/states/sensor.ferros_ha_local_bridge_status > .local-artifacts/pack-c-session-05-ha-restoration/post-power-remote-entity-state.json
```

## Fill targets after reconnect

Use the saved artifacts to update:

- `docs/gates/G4.md`
- `docs/orchestration/WAVE-RUN-LOG.md`
- a new findings packet for the restoration slice if you want the full packet separated from sessions 03 and 04

## Claim ceiling

- This handoff does not authorize G4 closure until the post-power artifacts actually show profile reload, agent re-registration, and HA entity restoration without manual repair.
- This handoff does not authorize independent install proof.