# Findings - Pack C Session 06 Windows Fresh-Host Bring-Up

> Filled from an operator-executed Windows FERROS packet against the homelab Home Assistant docker deployment on `homelab001`. Backend captures live under the timestamped Windows artifact roots recorded by `run-metadata.json` and the later `followup-metadata.json`.

## Scope

This findings packet captures a fresh explicit-profile Windows host bring-up, a homelab `homeassistant` container restart, and a later hard Windows restart follow-up.

## Claim ceiling

- This packet proves that FERROS revision `aafc16bc64012f7fee8fb2a2e2845015b5f6f615` built on a separate Windows host after a stable toolchain update, initialized a fresh explicit profile path, and manually created the Home Assistant bridge entity on `homelab001` through `remote-report-state`.
- This packet also proves that restarting only the `homeassistant` container on `homelab001` removed that entity until one manual `remote-report-state` repair.
- This packet further proves that a later hard Windows restart left the explicit profile file readable and did not remove the already-restored Home Assistant entity.
- This packet does not authorize G4 closure, automatic republish after Home Assistant restart, automatic republish on Windows boot, or independent install proof.

## Session header

| Field | Value |
|-------|-------|
| Date | `2026-05-04` |
| FERROS revision | `aafc16bc64012f7fee8fb2a2e2845015b5f6f615` |
| Windows host mode | `fresh explicit profile path` plus `fresh temp-rooted local FERROS state root` |
| Home Assistant host | `homelab001` |
| Entity reference | `sensor.ferros_ha_local_bridge_status` |

## Windows environment preparation

| Field | Value |
|-------|-------|
| First discarded setup issue | `PowerShell capture issue` |
| Second discarded setup issue | `stale Windows stable Rust toolchain` |
| Toolchain fix | `rustup update stable` moved `rustc` and `cargo` from `1.79.0` to `1.95.0` before the successful packet |
| Artifact references | `02a-rustup-update-stable.stdout.txt`; `02b-toolchain-version-after-update.stdout.txt` |

## Fresh Windows bring-up result

| Field | Value |
|-------|-------|
| Profile result | Fresh profile initialization and profile readback both succeeded on the new explicit profile path |
| Agent list result | `echo`, `ha-local-bridge`, and `timer` were all `registered` |
| Bridge describe result | `ha-local-bridge` version `0.1.0` with required capability `hub-local-bridge:bridge.observe` |
| Artifact references | `run-metadata.json`; `04-profile-show.stdout.txt`; `05-agent-list.stdout.txt`; `06-agent-describe-ha-local-bridge.stdout.txt` |

## Home Assistant result before restart

| Field | Value |
|-------|-------|
| Read-only summary before write | `remote-summary` found zero FERROS entities |
| Manual create step | One `remote-report-state` call created `sensor.ferros_ha_local_bridge_status` |
| Entity result after write | Home Assistant reported state `registered` with `state_source: ferros-node-agent-center-state` |
| Artifact references | `07-remote-summary-pre-restart.stdout.txt`; `08-remote-report-state-pre-restart.stdout.txt`; `09-ha-entity-state-pre-restart.stdout.txt` |

## Home Assistant container restart observation

| Field | Value |
|-------|-------|
| Restart boundary | Only the `homeassistant` container on `homelab001` was restarted |
| Automatic survival result | `no` |
| Read-only observation after restart | `remote-summary` again saw zero FERROS entities and the direct Home Assistant API fetch returned `Entity not found` |
| Assessment | `automatic_result=missing`; `repair_required=true` |
| Manual repair result | One manual `remote-report-state` call restored the entity immediately after the restart |
| Artifact references | `11a-automatic-survival-assessment.txt`; `13-ha-entity-state-post-restart-after-repair.stdout.txt` |

## Hard Windows restart follow-up

| Field | Value |
|-------|-------|
| Profile file result after host restart | The explicit profile file still existed and loaded |
| Temp-root local state file result | The temp-rooted local state file did not exist after the hard restart |
| Agent list result | The local CLI still showed `ha-local-bridge` as `registered` |
| Home Assistant entity result before repair | The entity was still present before any manual repair |
| Assessment | `manual_repair_required=false` |
| Limiting observation | The entity `last_updated` timestamp still matched the earlier manual-repair timestamp from the Home Assistant container restart recovery, so this follow-up does not prove automatic republish on Windows boot |
| Artifact references | `followup-metadata.json`; `07-host-restart-survival-assessment.txt` |

## Remaining gaps

- Automatic republish after Home Assistant restart remains open because the entity disappeared until a manual repair.
- Automatic republish on Windows boot remains open because the surviving entity timestamp matched the earlier manual repair rather than a fresh boot-time publish.
- This packet does not satisfy the G4 independent-install row because it was not a second non-primary home setup or an outside-operator install following only published instructions.
- G4 remains open.

## Non-claims for this template

- No G4 closure.
- No automatic republish claim after Home Assistant restart.
- No automatic republish claim on Windows boot.
- No independent install proof.