# Findings — Pack B Session 01 Profile Baseline

> Filled from agent-executed local commands on the physical host `homelab001` under explicit operator authorization from `Maangled`. Raw captures live under `.local-artifacts/pack-b-session-01/`.

## Claim ceiling

- This findings packet captures only the first Pack B `x86_64` physical profile baseline slice on `homelab001`.
- It proves a real local profile init and show run on the named Pack B DUT and records the paired local hub rehearsal artifact set used by that session.
- This template does not authorize a claim of Home Assistant proof, full power-cycle survival, D1 closure, G4 closure, independent install evidence, remote transport, or canonical mutation.

## Session header

| Field | Value |
|-------|-------|
| Date | `2026-05-03` |
| Operator | `Maangled` |
| Pack B DUT name | `homelab001` |
| DUT OS version | `Linux 6.8.0-101-generic x86_64 GNU/Linux` |
| DUT repo path | `/home/homelab001/apps/ferros` |
| Session label | `pack-b-session-01` |

## Command transcript

```text
cd /home/homelab001/apps/ferros
mkdir -p .local-state .local-artifacts/pack-b-session-01
cargo xtask hub-runway --keep-artifacts
cargo run -p ferros-hub -- summary
cargo run -p ferros-node --bin ferros -- profile init .local-state/pack-b-session-01-profile.json
cargo run -p ferros-node --bin ferros -- profile show .local-state/pack-b-session-01-profile.json
hostname
uname -srmo
```

## Profile init result

| Field | Value |
|-------|-------|
| Command used | `cargo run -p ferros-node --bin ferros -- profile init .local-state/pack-b-session-01-profile.json` |
| Exit result | `0` |
| Reported profile path | `.local-state/pack-b-session-01-profile.json` |
| Notes | `Initialized profile id 1fbaa8c2b5da339f8cbb0e0e3afe1c55a6b46bbc078cc10d74c8d0186cd40920 with profile name Fresh Start; raw capture is in .local-artifacts/pack-b-session-01/profile-init.txt.` |

## Profile show result

| Field | Value |
|-------|-------|
| Command used | `cargo run -p ferros-node --bin ferros -- profile show .local-state/pack-b-session-01-profile.json` |
| Exit result | `0` |
| Observed fields summary | `JSON output showed meta version 1.0 at stage 0, identity Fresh Start / Newcomer, six level-1 attribute buckets, one genesis_pioneer achievement, an empty credentials list, and a one-entry seal chain.` |
| Notes | `The paired DUT-side rehearsal path was refreshed first with cargo xtask hub-runway --keep-artifacts and recorded with cargo run -p ferros-hub -- summary; raw captures are in .local-artifacts/pack-b-session-01/profile-show.txt, .local-artifacts/pack-b-session-01/xtask-hub-runway.txt, and .local-artifacts/pack-b-session-01/hub-summary.txt.` |

## Local artifact references

| Artifact or read path | Reference |
|-----------------------|-----------|
| `.tmp/hub/simulated-local-bridge-artifact.json` | `Produced by the runway refresh and copied to .local-artifacts/pack-b-session-01/simulated-local-bridge-artifact.json.` |
| `.tmp/hub/local-hub-state-snapshot.json` | `Produced by the runway refresh and copied to .local-artifacts/pack-b-session-01/local-hub-state-snapshot.json.` |
| `.tmp/hub/local-onramp-proposal.json` | `Produced by the runway refresh and copied to .local-artifacts/pack-b-session-01/local-onramp-proposal.json.` |
| `.tmp/hub/local-onramp-decision-receipt.json` | `Produced by the runway refresh and copied to .local-artifacts/pack-b-session-01/local-onramp-decision-receipt.json.` |

## Failure notes

`No command in this baseline slice failed. The earlier ha-local-bridge visibility mismatch remains unresolved because this baseline wave did not rerun ferros agent list or ferros agent describe.`

## Remaining gaps

- Separate-host Home Assistant proof remains deferred to Pack C.
- Clean reboot and full DUT-only power-cycle survival remain unobserved in this baseline slice.
- The bridge-agent visibility mismatch documented in the earlier homelab001 local findings packet is still unresolved.
- No D1 or G4 closure claim is supported by this baseline alone.

## Non-claims for this template

- No Home Assistant proof unless separately captured on a real Pack C host.
- No full power-cycle survival proof.
- No D1 or G4 closure.