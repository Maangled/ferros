# 2026-05-04 Reboot And Resume Handoff

> Repo handoff note for the next session. This is a coordination surface only; it is not evidence.

## Current truth

- The detached reboot handoff has now been executed on `homelab001`.
- `homelab001` came back from the clean reboot on the same IPv4 address `192.168.50.234/24`.
- Post-reboot artifacts now live under `.local-artifacts/pack-b-session-02-handoff-mirror/`.
- The temporary Windows Home Assistant host `MKY` is intentionally still up for follow-on proof.
- `ferros-hub remote-report-state` now syncs Home Assistant bridge state from the local hub runtime summary when that summary is available, with fallback to the earlier probe payload.
- Local validation for that bridge change is green.
- Live post-refactor HA validation has now succeeded with a fresh bearer token. The saved artifacts are under `.local-artifacts/pack-c-session-02-bridge-state-sync/`, and the authenticated HA Entities UI showed both `FERROS Bridge Probe` and `FERROS ha-local-bridge Status`.

## Next truthful actions

1. Use the saved artifacts under `.local-artifacts/pack-b-session-02-handoff-mirror/` and `.local-artifacts/pack-c-session-02-bridge-state-sync/` to drive any remaining review of the two findings packets.
2. Keep D1 and G4 claim ceilings unchanged unless a later run adds launch-grade consent-deny or power-cycle restoration proof.

## Non-claims

- No launch-grade HA proof, consent-deny UI proof, or HA restoration-after-power-cycle proof has been earned by this note.
- No D1 or G4 closure has been earned by this note.