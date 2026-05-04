# D1 Hard Power Closeout Handoff

> Operator-run handoff for the remaining D1 reboot-safe evidence row. This note exists so the final hard power-cut capture can resume cleanly even if the current VS Code session is interrupted.

## Scope

This handoff covers only the remaining D1 reboot-safe FERROS-side state check on `homelab001`.

Already satisfied before the hard power cut:

- profile init or show evidence in `docs/hardware/findings/FINDINGS-pack-b-session-01-profile-baseline.md`
- named HA stand-in evidence in `docs/hardware/findings/FINDINGS-pack-c-session-01-ha-visibility.md`
- consent-flow visibility evidence in `docs/hardware/findings/FINDINGS-pack-b-session-03-d1-consent-shell.md`

Still required after the hard power cut:

- one full DUT-only power cycle on `homelab001`
- post-boot proof that the Pack B profile still loads
- post-boot proof that at least one agent re-registers without manual state repair

## Pre-power artifacts already captured

These files are already present under `.local-artifacts/pack-b-session-03-d1-closeout/` and do not need to be rerun before pulling power:

- `pre-power-time.txt`
- `pre-power-hostname.txt`
- `pre-power-uname.txt`
- `pre-power-profile-show.txt`
- `pre-power-agent-list.txt`
- `pre-power-agent-describe-ha-local-bridge.txt`
- `pre-power-denied-run.txt`
- `pre-power-agent-logs-ha-local-bridge.txt`
- `pre-power-agent-snapshot.json`

## Hard power boundary

- Keep the separate Pack C Home Assistant host `MKY` running.
- Remove power from `homelab001` only.
- Record the power-off time and the first time the host is reachable again.
- A clean reboot does not satisfy this handoff; this step is specifically for the full DUT-only power cut.

## Post-power capture

After `homelab001` is reachable again, run:

```text
cd /home/homelab001/apps/ferros
date -Is | tee .local-artifacts/pack-b-session-03-d1-closeout/post-power-time.txt
hostname | tee .local-artifacts/pack-b-session-03-d1-closeout/post-power-hostname.txt
uname -srmo | tee .local-artifacts/pack-b-session-03-d1-closeout/post-power-uname.txt
cargo run -p ferros-node --bin ferros -- profile show .local-state/pack-b-session-01-profile.json | tee .local-artifacts/pack-b-session-03-d1-closeout/post-power-profile-show.txt
cargo run -p ferros-node --bin ferros -- agent list | tee .local-artifacts/pack-b-session-03-d1-closeout/post-power-agent-list.txt
cargo run -p ferros-node --bin ferros -- agent describe ha-local-bridge | tee .local-artifacts/pack-b-session-03-d1-closeout/post-power-agent-describe-ha-local-bridge.txt
```

If the profile path differs from `.local-state/pack-b-session-01-profile.json`, replace it with the actual persistent path used in the Pack B baseline.

## Fill targets after reconnect

Use the saved artifacts to update:

- `docs/gates/D1.md`
- `docs/orchestration/WAVE-RUN-LOG.md`
- `docs/hardware/findings/FINDINGS-pack-b-session-03-d1-consent-shell.md` only if a post-power addendum is needed

## Claim ceiling

- This handoff does not authorize D1 closure until the post-power artifacts are actually captured and checked.
- This handoff does not authorize G4 closure, HA recovery claims, or independent install claims.
- If the host returns on a different IP or needs manual console work, record that operator context explicitly.