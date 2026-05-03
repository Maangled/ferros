# Findings — homelab001 Local Bring-up

> Filled from agent-executed local commands on `homelab001` under explicit operator authorization from `Maangled`. Raw captures live under `.local-artifacts/reentry-homehub-local-01/`.

## Claim Ceiling

- This findings packet captures only the local profile and hub bring-up slice on `homelab001`.
- Co-located Home Assistant, if mentioned, is local rehearsal only and not separate-host proof.
- This template does not authorize claims of Matter support, device control, D1 closure, G4 closure, launch readiness, or separate-host Home Assistant proof.

## Session Header

| Field | Value |
|-------|-------|
| Date | `2026-05-03` |
| Operator | `Maangled` |
| Execution mode | `agent-executed local commands on homelab001 under explicit operator authorization` |
| Machine | `homelab001` |
| Repo path | `/home/homelab001/apps/ferros` |
| Local state path | `/home/homelab001/apps/ferros/.local-state` |
| Local artifact path | `/home/homelab001/apps/ferros/.local-artifacts` |
| Session label | `reentry-homehub-local-01` |

## Machine Facts

| Field | Value |
|-------|-------|
| OS or kernel summary | `Linux 6.8.0-101-generic x86_64 GNU/Linux` |
| CPU summary | `Intel(R) Core(TM) i5-10400 CPU @ 2.90GHz` |
| Memory summary | `15 GiB total RAM; 5.9 GiB available at capture` |
| Network note | `enp4s0 was up at 192.168.50.234/24; passive neighbor-cache entries were visible via ip neigh show` |

## Exact Commands Run

```text
cd /home/homelab001/apps/ferros
mkdir -p .local-state .local-artifacts/reentry-homehub-local-01
cargo xtask hub-runway --keep-artifacts
cp -f .tmp/hub/*.json .local-artifacts/reentry-homehub-local-01/
cargo run -p ferros-node --bin ferros -- profile init .local-state/homelab001-profile.json
cargo run -p ferros-node --bin ferros -- profile show .local-state/homelab001-profile.json
cargo run -p ferros-hub -- summary
cargo run -p ferros-hub -- prove-bridge
cargo run -p ferros-node --bin ferros -- agent list
cargo run -p ferros-node --bin ferros -- agent describe ha-local-bridge
cargo run -p ferros-hub -- deny-demo
hostname
uname -srmo
free -h
ip -brief address show
ip neigh show
grep -m1 'model name' /proc/cpuinfo
```

## Hub Runway Refresh Result

| Field | Value |
|-------|-------|
| Command used | `cargo xtask hub-runway --keep-artifacts` |
| Exit result | `0` |
| Observed note | `The refresh emitted ha-local-bridge@0.1.0 in the summary, kept all four expected .tmp/hub JSON artifacts for inspection, and reported hubUnexpectedArtifacts: none.` |

## Profile Init Result

| Field | Value |
|-------|-------|
| Command used | `cargo run -p ferros-node --bin ferros -- profile init .local-state/homelab001-profile.json` |
| Exit result | `0` |
| Reported profile path | `.local-state/homelab001-profile.json` |
| Notes | `Initialized profile id a596f38bdb6242343b29f99340f9e466b65840cf4a943cf453a72f229347ae10 with profile name Fresh Start; raw capture is in .local-artifacts/reentry-homehub-local-01/profile-init.txt.` |

## Profile Show Result

| Field | Value |
|-------|-------|
| Command used | `cargo run -p ferros-node --bin ferros -- profile show .local-state/homelab001-profile.json` |
| Exit result | `0` |
| Observed fields summary | `JSON output showed meta version 1.0 at stage 0, identity Fresh Start / Newcomer, six level-1 attribute buckets, one genesis_pioneer achievement, an empty credentials list, and a one-entry seal chain.` |
| Notes | `Raw JSON capture is in .local-artifacts/reentry-homehub-local-01/profile-show.txt.` |

## Hub Summary Result

| Field | Value |
|-------|-------|
| Command used | `cargo run -p ferros-hub -- summary` |
| Exit result | `0` |
| Observed summary note | `registeredBridgeAgents: 1; bridgeAgent: ha-local-bridge@0.1.0; requesterProfileId: hub-local-bridge; policyDecision: allowed; bridgeStatus: allowed; summary: local-only bridge allowed report-state for simulated-bridge-entity.` |

## Hub Bridge Rehearsal Result

| Field | Value |
|-------|-------|
| Command used | `cargo run -p ferros-hub -- prove-bridge` |
| Exit result | `0` |
| Observed bridge note | `Allowed for simulated-bridge-entity with artifact .tmp/hub/simulated-local-bridge-artifact.json and receipt .tmp/hub/local-onramp-decision-receipt.json; output remained local-only and non-evidentiary.` |

## Agent Visibility Result

| Field | Value |
|-------|-------|
| List command used | `cargo run -p ferros-node --bin ferros -- agent list` |
| List exit result | `0` |
| Visible agent names | `echo, timer` |
| Describe command used | `cargo run -p ferros-node --bin ferros -- agent describe ha-local-bridge` |
| Describe exit result | `1` |
| Describe note | `The command returned unknown agent: ha-local-bridge even though ferros-hub summary reported bridgeAgent: ha-local-bridge@0.1.0.` |

## Optional Deny Visibility Check

| Field | Value |
|-------|-------|
| Command used | `cargo run -p ferros-hub -- deny-demo` |
| Exit result | `0` |
| Observed deny note | `denied:no-grants for simulated-bridge-entity without artifact [local-only; non-evidentiary]` |

## Local Artifact And State References

| Artifact or state path | Reference |
|------------------------|-----------|
| `.local-state/homelab001-profile.json` | `Present; created by profile init and read back by profile show.` |
| `.local-artifacts/reentry-homehub-local-01/command-exit-codes.tsv` | `Present; records the exit code for every executed command in this session.` |
| `.local-artifacts/reentry-homehub-local-01/xtask-hub-runway.txt` | `Present; exit 0.` |
| `.local-artifacts/reentry-homehub-local-01/copied-hub-artifacts.txt` | `Present; lists the copied .tmp/hub JSON artifacts.` |
| `.local-artifacts/reentry-homehub-local-01/simulated-local-bridge-artifact.json` | `Present; copied from .tmp/hub after the runway refresh.` |
| `.local-artifacts/reentry-homehub-local-01/local-hub-state-snapshot.json` | `Present; copied from .tmp/hub after the runway refresh.` |
| `.local-artifacts/reentry-homehub-local-01/local-onramp-proposal.json` | `Present; copied from .tmp/hub after the runway refresh.` |
| `.local-artifacts/reentry-homehub-local-01/local-onramp-decision-receipt.json` | `Present; copied from .tmp/hub after the runway refresh.` |
| `.local-artifacts/reentry-homehub-local-01/profile-init.txt` | `Present; exit 0.` |
| `.local-artifacts/reentry-homehub-local-01/profile-show.txt` | `Present; exit 0.` |
| `.local-artifacts/reentry-homehub-local-01/hub-summary.txt` | `Present; exit 0.` |
| `.local-artifacts/reentry-homehub-local-01/hub-prove-bridge.txt` | `Present; exit 0.` |
| `.local-artifacts/reentry-homehub-local-01/agent-list.txt` | `Present; exit 0.` |
| `.local-artifacts/reentry-homehub-local-01/agent-describe.txt` | `Present; exit 1 with unknown agent: ha-local-bridge.` |
| `.local-artifacts/reentry-homehub-local-01/hub-deny-demo.txt` | `Present; exit 0.` |
| `.local-artifacts/reentry-homehub-local-01/host-facts.txt` | `Present; hostname, kernel, memory, address, neighbor-cache, and CPU summary capture.` |

## Optional Co-Located Home Assistant Note

| Field | Value |
|-------|-------|
| Home Assistant used in this session | `no` |
| If yes, co-located on `homelab001` | `n/a` |
| Separate-host Home Assistant proof claimed | `no` |
| Observation note | `No Home Assistant process interaction was started, stopped, or inspected in this segment.` |

## Optional LAN Device Observation

| Field | Value |
|-------|-------|
| Device label | `Passive neighbor-cache observation only` |
| IP or MAC if available | `Multiple cached peers were visible on enp4s0; examples include 192.168.50.1 (reachable) and 192.168.50.146 (reachable).` |
| Discovery method | `Read-only ip neigh show from the host-facts capture.` |
| Observed service or behavior | `Neighbor-cache presence only; no service discovery, scan, or control attempt.` |
| Network usage observation path | `.local-artifacts/reentry-homehub-local-01/host-facts.txt` |
| Consent or ownership note | `Passive local cache inspection only; no active network scan or packet capture.` |
| What Ferros can honestly claim | `homelab001 observed existing LAN neighbor-cache entries during the session.` |
| What Ferros cannot honestly claim | `device identity certainty, device control, Matter support, or telemetry beyond passive cache presence.` |

## Failures

- `cargo run -p ferros-node --bin ferros -- agent describe ha-local-bridge` exited `1` with `unknown agent: ha-local-bridge`.
- `cargo run -p ferros-node --bin ferros -- agent list` showed only `echo` and `timer`, which does not match the `ferros-hub summary` line `bridgeAgent: ha-local-bridge@0.1.0`.

## Remaining Gaps

- Reconcile why `ferros-hub summary` reports `ha-local-bridge@0.1.0` while `ferros agent list` and `ferros agent describe ha-local-bridge` do not surface that agent.
- Separate-host Home Assistant validation remains deferred.
- No device-control, Matter, D1, G4, or launch-readiness evidence was produced by this local-only segment.

## Non-Claims For This Template

- No separate-host Home Assistant proof.
- No Matter support claim.
- No device control claim.
- No packet-inspection or deep-telemetry claim.
- No D1 or G4 closure.

## HANDOFF CARD
- Lane ID: F2
- Status: complete
- Files read: docs/hardware/homelab001-local-bringup-runbook.md; .local-artifacts/reentry-homehub-local-01/*; .local-state/homelab001-profile.json; docs/orchestration/REENTRY-HOMEHUB-LOCAL-ORCH-01.md
- Files changed: docs/hardware/findings/FINDINGS-homelab001-local-bringup.md
- Evidence produced: filled findings packet from agent-executed local profile and hub bring-up commands on homelab001
- Claims added: actual local profile and hub outputs are captured; artifact paths are recorded; the bridge-agent visibility mismatch is documented as a failure
- Claims explicitly not added: separate-host Home Assistant proof, D1 closure, G4 closure, launch readiness, Matter support, device control
- Validation: raw capture review against command-exit-codes.tsv and the generated artifact set
- Residual risks: the ha-local-bridge visibility mismatch remains unresolved
- Next safe follow-up, if any: queue the bridge-agent visibility investigation segment