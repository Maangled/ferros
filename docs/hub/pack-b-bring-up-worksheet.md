# FERROS Hub - Pack B Bring-Up Worksheet

> Derived from `docs/hub/reference-hardware.md`. That file remains authoritative. This worksheet is prep and evidence-capture only for the Pack B x86_64 device under test plus a separate Pack C Home Assistant host. It does not create a second authority map and does not imply that any G4 item is complete.

## Scope

Use this worksheet to capture the first Pack B bring-up session against the existing topology, pre-run, and G4 evidence-prep sections in the authoritative hardware runway. Keep entries at the level of operator notes and evidence placeholders only.

## Session header

| Field | Capture |
|-------|---------|
| Session date | |
| Operator | |
| Location class | |
| Pack B DUT model / identifier | |
| Pack B DUT storage | |
| Pack C HA host model / identifier | |
| Operator station | |
| DUT OS version | |
| HA host OS / HA version | |
| Network / addressing note | |
| DUT-only power-cut method | |
| Notes / artifact path | |

## Topology confirmation

| Role | Expected from reference hardware | Session capture |
|------|----------------------------------|-----------------|
| Device under test | Pack B x86_64 mini PC or home server with SSD and wired LAN | |
| Home Assistant host | Separate Pack C device on the same LAN | |
| Operator station | Separate laptop or desktop for SSH, dashboard observation, and evidence capture | |
| Power arrangement | DUT power can be cut without taking HA down | |

## Pre-run checks

| Check | What to confirm now | Session capture |
|-------|---------------------|-----------------|
| OS image | Exact 64-bit Linux image, version, and update state recorded for the DUT | |
| Storage choice | Exact boot and data medium recorded, writable, and sized with headroom for logs | |
| Persistent state path | Candidate persistent directory or mount point identified for future profile, grant, and log material | |
| Network stability | DUT hostname or address is stable, SSH works, and the HA host is reachable on the same LAN | |
| Clock and time sync | Time sync is functioning on both DUT and HA host | |
| Clean reboot path | DUT can reboot and come back without manual filesystem or network repair | |
| Recovery path | Local console, HDMI, or serial fallback is known if SSH disappears | |
| Power control | Operator knows how DUT-only power can be removed later without taking HA down | |
| Session notes | Tester, location class, and exact hardware identifiers are ready to capture | |
| Pairing boundary | Notes stay at the level of constraints and open questions only | |

## G4 evidence-capture placeholders

Leave these rows blank until a real session produces artifacts. These placeholders are keyed to the existing G4 evidence map in `docs/hub/reference-hardware.md`. They are capture fields only, not completion claims.

| G4 evidence item | Existing proof point to mirror | Capture placeholder |
|------------------|--------------------------------|---------------------|
| Cross-compile `ferros-hub` | Successful `x86_64-unknown-linux-gnu` build on the Pack B class | Build command or ref: ___ ; log or artifact path: ___ ; notes: ___ |
| Physical device run | One real Pack B DUT session, not laptop or VM | Session timestamp: ___ ; DUT identity: ___ ; operator note: ___ |
| Device profile persists | `ferros profile init` on the DUT, then restart and reload the same profile | Profile path or ref: ___ ; restart note: ___ ; artifact or log: ___ |
| HA bridge agent is listed | `ferros agent list` on the DUT shows the bridge agent once it exists | Command or log ref: ___ ; timestamp: ___ ; notes: ___ |
| Real HA entity is visible | One real entity on the separate HA host dashboard | Dashboard or screenshot ref: ___ ; entity id: ___ ; notes: ___ |
| Consent deny is visible | One denied request captured in logs and surfaced to the operator | Deny log ref: ___ ; HA or UI ref if any: ___ ; notes: ___ |
| Full power-cycle survival | DUT-only cold boot restores profile, bridge agent, and HA-visible state | Power-cut method: ___ ; recovery note: ___ ; artifact or log: ___ |
| Independent install | Same bring-up contract repeated on a second non-primary home setup | Second-site note: ___ ; operator and date: ___ ; artifact path: ___ |