# FERROS Hub — Reference Hardware Runway

> This document is the S7 hardware runway for `ferros-hub`. It records candidate launch hardware, bring-up assumptions, and the evidence that must exist before G4 can close.

---

## Current mode

- S7 is still in runway mode. G4 cannot close until G3 is green and a real `ferros-hub` binary runs on physical home hardware.
- This file is planning and evidence-prep only until a device moves into the confirmed evidence table below.
- `LAUNCH.md` and `docs/gates/G4.md` remain the authoritative launch criteria. This file should not be used to imply that launch evidence already exists.

---

## Launch-aligned constraints

These are the hardware-side constraints that must be satisfied before a device can count toward launch:

| Constraint | Why it matters |
|-----------|----------------|
| Physical home hardware only | Launch excludes CI, QEMU, and developer-laptop demos. |
| Linux on `aarch64` or `x86_64` | Matches the device classes allowed by `LAUNCH.md`. |
| Persistent storage for profile and grants | G4 requires the device profile and grants to survive restart and full power cycle. |
| Reachable Home Assistant deployment | A real HA entity must register through the agent center and appear in the dashboard. |
| Ability to observe consent denial | The first hardware topology must support checking that deny events are logged and visible in HA UI or `ferros agent logs`. |
| Repeatable reboot test path | G4 requires profile reload, agent re-registration, and HA entity restoration after power cycle. |

---

## Minimum runway requirements

| Component | Minimum | Preferred runway target | Notes |
|-----------|---------|-------------------------|-------|
| Architecture | `x86_64` or `aarch64` | One candidate in each class | Keeps both launch-valid home-device paths open. |
| RAM | 512 MB | 1 GB+ | Headroom for runtime, logs, and HA bridge work. |
| Storage | 4 GB persistent storage | SSD or high-endurance SD plus backup path | Profile, grants, and logs must survive reboot and power loss. |
| OS | Modern 64-bit Linux | Debian 12 / Ubuntu 22.04 / Raspberry Pi OS 64-bit | Pick a boring distro first; novelty does not help G4. |
| Network | Ethernet or reliable Wi-Fi | Stable LAN path to Home Assistant | Avoid first-run hardware that depends on flaky wireless recovery. |
| Power | Stable supply | UPS or known-good PSU | Needed for honest power-cycle testing. |

---

## Current hardware-design pack runway

This section is planning shorthand only. A "pack" is a candidate bundle of device, storage, network, and power choices for the first lab sessions. None of these packs count as implementation or launch evidence.

| Pack | Primary candidate | Acceptable fallback | Planned runway use | Why it is practical now | Future validation target |
|------|-------------------|---------------------|--------------------|-------------------------|--------------------------|
| Pack A - Pi lane | Raspberry Pi 5 (8 GB) with 32-128 GB USB 3 SSD, wired Ethernet, and official PSU | Raspberry Pi 4 (4 GB) with high-endurance microSD only if the SSD path is not ready yet | Primary `aarch64` device for first in-lab prep | Closest match to the expected home-hub form factor and easy to reproduce later | Later prove reboot-safe state and cold-boot recovery on the chosen storage medium |
| Pack B - x86_64 lane | Intel NUC, Lenovo Tiny, or Beelink-class mini PC with 8 GB RAM, 128 GB SSD, and wired Ethernet | Spare small-form-factor PC or home server with SSD and stable LAN access | Primary observability-first device while G3 still blocks honest implementation runs | Easier shell access, rollback, and log capture for early hardware prep | Later prove the same restart and power-cycle behavior once the runtime path exists |
| Pack C - HA companion lane | Separate Home Assistant host on HA Green/Yellow, Raspberry Pi 4, or small x86_64 box on the same LAN | Existing always-on home server on the same LAN | First lab-side HA environment, kept separate from the device under test when possible | Keeps device restarts and HA uptime distinguishable in later notes | Later confirm that device-only and HA-only restarts can be observed separately once integration exists |

---

## First Home Assistant lab topology

This is the candidate topology for the first honest lab sessions. It is still runway guidance, not a claim that the HA path is implemented.

| Role | First-choice arrangement | Acceptable fallback | Runway note |
|------|--------------------------|---------------------|-------------|
| Device under test | One Pack A or Pack B host at a time | Same hardware with temporary monitor/keyboard access for recovery | Keep the first session to one DUT so reboot notes and storage behavior stay attributable |
| Home Assistant host | Separate Pack C device on the same switch or subnet | Same LAN over stable Wi-Fi only if Ethernet is not available | Separate-host topology is preferred because later restart evidence should isolate DUT behavior |
| Operator station | Separate laptop or desktop for SSH, dashboard viewing, and note capture | Tablet plus local console fallback | Do not rely on the DUT itself as the only observation surface |
| Network path | Same private LAN with DHCP reservations or other stable addressing | Static IP notes if reservations are not available | Avoid cloud relays, mesh-only hops, or VPN dependence in the first lab layout |
| Power arrangement | Known-good PSUs for both boxes, with a way to cut DUT power without also dropping HA | Simple outlet switch or smart plug reserved for later rehearsal only | Record the power arrangement now so later cold-boot tests can be repeatable |

Same-box DUT + Home Assistant setup can still help with package or OS preparation, but it is not the preferred first lab topology for later restart evidence because coupled restarts hide device recovery behavior.

---

## Storage, network, and power assumptions

These are current runway assumptions for the first hardware sessions. They reserve durability and observability needs without freezing downstream implementation details.

| Area | Current runway assumption | Why it stays provisional |
|------|---------------------------|--------------------------|
| Storage | Candidate device state should live on persistent local block storage: USB 3 SSD on Pi when possible, internal SSD on `x86_64`, and a journaling filesystem such as `ext4`; note the exact mount point that would later hold profile, grant, and log material | Exact layout, ownership, and file paths depend on downstream runtime surfaces, so this doc only fixes the durability expectation |
| Network | First lab should keep DUT and HA on the same private LAN with SSH reachability, HA dashboard reachability, and working time sync; internet access is helpful for OS prep but should not become a permanent control-plane assumption | Final ports, discovery behavior, and bridge transport are not S7-runway decisions yet |
| Power | DUT needs a stable PSU sized for storage peripherals; HA should ideally stay powered during DUT-only restart rehearsals; note whether a UPS or switchable outlet is available before the session | This is evidence-prep only until a real restart or power-cycle rehearsal happens and is recorded |
| Observability | Persisted logs plus a simple recovery path such as SSH, local console, or serial fallback should exist before the first honest reboot rehearsal | The exact commands and log surfaces remain implementation-dependent |

---

## Pre-run checks before the first on-device session

These checks are meant to reduce false starts in the first lab session. Completing them does not count as G4 evidence.

| Check | What to confirm now | Why it matters later |
|-------|---------------------|----------------------|
| OS image | Exact 64-bit Linux image, version, and update state recorded for the DUT | Later evidence needs reproducible device setup, not vague "latest image" notes |
| Storage choice | Exact boot and data medium recorded, writable, and sized with headroom for logs | Reboot-safe storage cannot be assessed later if the medium was never pinned down |
| Persistent state path | Candidate persistent directory or mount point identified for future profile, grant, and log material | S7 can reserve durability expectations now without defining final runtime layout |
| Network stability | DUT hostname or address is stable, SSH works, and HA host is reachable on the same LAN | Later restart and HA visibility checks need a boring network story |
| Clock and time sync | NTP or equivalent time sync is functioning on both DUT and HA host | Time drift will make future reboot and consent-deny observations hard to trust |
| Clean reboot path | DUT can reboot and come back without manual filesystem or network repair | Cold-boot tests are meaningless if the clean reboot path is already fragile |
| Recovery path | Local console, HDMI, or serial fallback is known if SSH disappears | Prevents a lab session from turning into guesswork after a failed restart |
| Power control | Operator knows exactly how DUT-only power can be removed later without taking HA down too | Power-cycle evidence requires a repeatable and isolated cut path |
| Session notes | Tester, location class, chosen pack, and exact hardware identifiers are ready to capture | Launch evidence later needs traceable notes, not reconstructed memory |
| Pairing boundary | Notes stay at the level of constraints and open questions; no doc-local handshake is frozen here | S7 should not redefine authoritative pairing semantics before downstream seams stabilize |

---

## Future validation targets to prepare for now

These targets are intentionally forward-looking. They describe what later hardware validation should prove, not what is complete today.

| Future target | Why prepare for it now | What would eventually count as evidence |
|---------------|------------------------|-----------------------------------------|
| Reboot-safe profile and grant storage | Storage and mount decisions made now affect whether restart evidence can be trusted later | A recorded session showing the real device state still loads after a clean reboot |
| Full power-cycle survival | Power arrangement and recovery path should be ready before the first cold-boot rehearsal | A recorded session showing the same state survives an abrupt power cut and cold start without manual repair |
| HA recovery visibility | Separate-host topology makes later device-only recovery observable | A recorded session showing the HA-side view recovers after DUT restart once the integration path exists |
| Consent-deny observability | Operator station and log capture should be planned before implementation lands | A recorded session showing a real deny event is visible in HA UI or FERROS logs once that surface exists |
| Repeated restart stability | Hardware prep should support more than a one-off happy-path run | Multiple recorded restart or cold-boot passes on the same chosen hardware pack |

---

## Evidence fields to capture once implementation exists

Fill this in only when the real on-device path exists. Until then, this table is a runway template.

| Date | Hardware pack | Exact device and storage | OS version | HA topology | Profile persists after clean reboot | Grant state survives full power cycle | HA-side recovery visible once available | Consent deny visible once available | Tester |
|------|---------------|--------------------------|------------|-------------|------------------------------------|--------------------------------------|--------------------------------------|------------------------------------|--------|

---

## Confirmed working hardware

Only add a row here when the hardware also satisfies the G4 evidence checklist.

| Hardware | Architecture | Confirmed on | `ferros-hub` version | Evidence location | Notes |
|----------|--------------|--------------|----------------------|-------------------|-------|

---

## Not sufficient for launch evidence

- Cross-compiling without running on the target device.
- QEMU or any other emulated hardware run.
- A developer laptop demo.
- A mocked or stubbed Home Assistant entity.
- Pairing notes that describe a future protocol but have not been exercised on hardware.
