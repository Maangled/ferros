# HARDWARE-2026-04-30-06 Detached Reboot Handoff

> Operator-run handoff only. This note exists because rebooting `homelab001` may cause it to return on a different IP and strand the attached VS Code session. Use this note to capture the reboot boundary outside the current Copilot session and then resume from the saved artifacts afterward.

## Scope

This handoff covers only the reboot-sensitive portion of `HARDWARE-2026-04-30-06`. It does not claim D1 closure, G4 closure, full power-cycle survival, or real Home Assistant proof.

## Preconditions

- Run from a detached SSH session or local console, not from the attached VS Code session.
- Keep the separate Pack C Home Assistant host `MKY` running.
- Use the persistent Pack B profile path from the prior baseline if available. If continuing the current named Pack B path, that is `.local-state/pack-b-session-01-profile.json`.
- Save all outputs under `.local-artifacts/pack-b-session-02-handoff-mirror/`.

## Pre-reboot capture

Run on `homelab001` before reboot:

```text
cd /home/homelab001/apps/ferros
mkdir -p .local-artifacts/pack-b-session-02-handoff-mirror
cp -f .tmp/hub/*.json .local-artifacts/pack-b-session-02-handoff-mirror/ 2>/dev/null || true
cargo xtask hub-runway --keep-artifacts | tee .local-artifacts/pack-b-session-02-handoff-mirror/xtask-hub-runway.txt
cargo run -p ferros-hub -- summary | tee .local-artifacts/pack-b-session-02-handoff-mirror/pre-reboot-hub-summary.txt
cargo run -p ferros-hub -- deny-demo | tee .local-artifacts/pack-b-session-02-handoff-mirror/pre-reboot-deny-demo.txt
cargo run -p ferros-node --bin ferros -- agent list | tee .local-artifacts/pack-b-session-02-handoff-mirror/pre-reboot-agent-list.txt
cargo run -p ferros-node --bin ferros -- agent describe ha-local-bridge | tee .local-artifacts/pack-b-session-02-handoff-mirror/pre-reboot-agent-describe.txt
hostname | tee .local-artifacts/pack-b-session-02-handoff-mirror/pre-reboot-hostname.txt
uname -srmo | tee .local-artifacts/pack-b-session-02-handoff-mirror/pre-reboot-uname.txt
date -Is | tee .local-artifacts/pack-b-session-02-handoff-mirror/pre-reboot-time.txt
```

## Reboot boundary

- Record the reboot method used.
- Record the power-off time and the first time the machine is reachable again.
- Prefer reconnecting by hostname if that works on the LAN. If not, use the local console or router or DHCP surface to discover the new IP before resuming.

Suggested reboot command from the detached operator surface:

```text
sudo reboot
```

## Post-reboot capture

After reconnecting to `homelab001`, run:

```text
cd /home/homelab001/apps/ferros
date -Is | tee .local-artifacts/pack-b-session-02-handoff-mirror/post-reboot-time.txt
hostname | tee .local-artifacts/pack-b-session-02-handoff-mirror/post-reboot-hostname.txt
uname -srmo | tee .local-artifacts/pack-b-session-02-handoff-mirror/post-reboot-uname.txt
cargo run -p ferros-node --bin ferros -- profile show .local-state/pack-b-session-01-profile.json | tee .local-artifacts/pack-b-session-02-handoff-mirror/post-reboot-profile-show.txt
cargo run -p ferros-node --bin ferros -- agent list | tee .local-artifacts/pack-b-session-02-handoff-mirror/post-reboot-agent-list.txt
cargo run -p ferros-hub -- summary | tee .local-artifacts/pack-b-session-02-handoff-mirror/post-reboot-hub-summary.txt
```

If the profile path differs from `.local-state/pack-b-session-01-profile.json`, replace it with the actual persistent path used during the prior Pack B baseline.

## Fill targets after reconnect

Use the saved artifacts to fill:

- `docs/hardware/findings/FINDINGS-pack-b-session-02-handoff-mirror.md`
- `docs/orchestration/WAVE-RUN-LOG.md` with the detached reboot session result

## Claim ceiling

- A clean reboot observation is not the same thing as full DUT-only power-cycle survival.
- This handoff does not authorize a claim of D1 closure, G4 closure, or real Home Assistant recovery.
- If the IP changes and reconnect requires manual discovery, record that as operator context rather than smoothing it over.