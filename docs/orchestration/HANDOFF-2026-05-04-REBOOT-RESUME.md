# 2026-05-04 Reboot And Resume Handoff

> Repo handoff note for the next session. This is a coordination surface only; it is not evidence.

## Current truth

- `homelab001` should not be rebooted from the attached VS Code session because it may come back on a different IP and strand the session.
- The reboot-sensitive lane is prepared in `docs/hardware/HARDWARE-2026-04-30-06-detached-reboot-handoff.md`.
- The temporary Windows Home Assistant host `MKY` is intentionally still up for follow-on proof.
- `ferros-hub remote-report-state` now syncs Home Assistant bridge state from the local hub runtime summary when that summary is available, with fallback to the earlier probe payload.
- Local validation for that bridge change is green.
- Live post-refactor HA validation is currently blocked by bearer-token expiry. The last attempted rerun against `MKY` returned `401 Unauthorized`.

## Next truthful actions

1. Run the detached reboot handoff from a detached SSH session or local console on `homelab001`.
2. After reconnect, use the saved artifacts under `.local-artifacts/pack-b-session-02-handoff-mirror/` to fill `docs/hardware/findings/FINDINGS-pack-b-session-02-handoff-mirror.md`.
3. If live HA validation is still needed after reboot, obtain a fresh operator-provided HA bearer token or another authenticated operator path and rerun `cargo run -p ferros-hub -- remote-report-state` plus `cargo run -p ferros-hub -- remote-summary`.
4. Keep D1 and G4 claim ceilings unchanged unless the reboot artifacts and any fresh HA observations actually support a wider claim.

## Non-claims

- No reboot result has been captured yet.
- No new live HA proof has been added after the bridge-state refactor.
- No D1 or G4 closure has been earned by this note.