# Findings Template — homelab001 Local Bring-up

> Blank template only. This file is non-evidence until a real operator-attended session fills it with actual values.

## Claim Ceiling

- This template does not prove anything while blank.
- When filled later, it may capture only the local profile and hub bring-up slice on `homelab001`.
- Co-located Home Assistant, if mentioned, is local rehearsal only and not separate-host proof.
- This template does not authorize claims of Matter support, device control, D1 closure, G4 closure, launch readiness, or separate-host Home Assistant proof.

## Session Header

| Field | Value |
|-------|-------|
| Date | `___` |
| Operator | `Maangled` |
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
| Memory summary | `15 GiB RAM` |
| Network note | `192.168.50.234/24 on enp4s0` |

## Exact Commands Run

```text
___
```

## Profile Init Result

| Field | Value |
|-------|-------|
| Command used | `___` |
| Exit result | `___` |
| Reported profile path | `___` |
| Notes | `___` |

## Profile Show Result

| Field | Value |
|-------|-------|
| Command used | `___` |
| Exit result | `___` |
| Observed fields summary | `___` |
| Notes | `___` |

## Hub Summary Result

| Field | Value |
|-------|-------|
| Command used | `___` |
| Exit result | `___` |
| Observed summary note | `___` |

## Hub Bridge Rehearsal Result

| Field | Value |
|-------|-------|
| Command used | `___` |
| Exit result | `___` |
| Observed bridge note | `___` |

## Agent Visibility Result

| Field | Value |
|-------|-------|
| Command used | `___` |
| Visible agent names | `___` |
| Optional described agent | `ha-local-bridge` or observed session value |

## Local Artifact And State References

| Artifact or state path | Reference |
|------------------------|-----------|
| `.local-state/homelab001-profile.json` | `___` |
| `.local-artifacts/reentry-homehub-local-01/profile-init.txt` | `___` |
| `.local-artifacts/reentry-homehub-local-01/profile-show.txt` | `___` |
| `.local-artifacts/reentry-homehub-local-01/hub-summary.txt` | `___` |
| `.local-artifacts/reentry-homehub-local-01/hub-prove-bridge.txt` | `___` |
| `.local-artifacts/reentry-homehub-local-01/agent-list.txt` | `___` |
| `.local-artifacts/reentry-homehub-local-01/xtask-hub-runway.txt` | `___` |

## Optional Co-Located Home Assistant Note

| Field | Value |
|-------|-------|
| Home Assistant used in this session | `yes/no` |
| If yes, co-located on `homelab001` | `___` |
| Separate-host Home Assistant proof claimed | `no` |

## Optional LAN Device Observation

| Field | Value |
|-------|-------|
| Device label | `___` |
| IP or MAC if available | `___` |
| Discovery method | `___` |
| Observed service or behavior | `___` |
| Network usage observation path | `___` |
| Consent or ownership note | `___` |
| What Ferros can honestly claim | `___` |
| What Ferros cannot honestly claim | `___` |

## Failures

`___`

## Remaining Gaps

- `___`

## Non-Claims For This Template

- No separate-host Home Assistant proof.
- No Matter support claim.
- No device control claim.
- No packet-inspection or deep-telemetry claim.
- No D1 or G4 closure.

## HANDOFF CARD
- Lane ID: A3
- Status: complete
- Files read: docs/hardware/findings/FINDINGS-pack-b-session-01-profile-baseline.md; docs/hardware/findings/FINDINGS-pack-b-session-02-handoff-mirror.md; docs/hardware/findings/FINDINGS-pack-c-session-01-ha-visibility.md; docs/orchestration/REENTRY-PHASE0-HARDWARE-READINESS-AUDIT.md
- Files changed: docs/hardware/findings/FINDINGS-homelab001-local-bringup.md
- Evidence produced: local bring-up findings template for homelab001 profile and hub capture
- Claims added: template path for local bring-up evidence capture
- Claims explicitly not added: any real evidence, separate-host Home Assistant proof, D1 closure, G4 closure
- Validation: template completeness review against requested fields
- Residual risks: real command outputs still need to be captured and pasted into this template
- Next safe follow-up, if any: run the local bring-up commands and fill the template