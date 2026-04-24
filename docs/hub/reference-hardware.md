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

## Candidate launch hardware

These rows are runway candidates, not confirmed launch evidence.

| Hardware | Architecture | OS candidate | Status | Why it is on the runway | Pre-run checks |
|----------|--------------|--------------|--------|--------------------------|----------------|
| Raspberry Pi 4 (4 GB) | `aarch64` | Raspberry Pi OS 64-bit or Debian 12 | ⬜ Planned | Common home hub form factor and the clearest launch-story device. | 64-bit image, reliable PSU, persistent storage choice, LAN path to HA. |
| x86_64 home server (Intel NUC or equivalent) | `x86_64` | Ubuntu 22.04 LTS or Debian 12 | ⬜ Planned | Fastest likely path for early bring-up and easier local observability. | Clean Linux install, persistent data directory, systemd/service plan, LAN path to HA. |

---

## Bring-up checklist for the first hardware session

- Pick one primary device and record the exact model, RAM, storage medium, and OS image version.
- Decide whether Home Assistant will run on the same machine or on a separate box on the same LAN.
- Reserve persistent storage locations for the device profile, capability grants, and logs.
- Confirm the device can reboot cleanly without manual recovery steps.
- Capture the operator/tester, install date, and install location class (lab, private beta home, etc.).
- Prepare a consent-deny test case, but do not freeze the final pairing handshake in this doc before S2/S3/S4 implementation surfaces are real.

---

## Evidence fields to capture once implementation exists

Fill this in only when `ferros-hub` is actually running on the device.

| Date | Hardware | OS version | HA topology | Profile persists after restart | Agent re-registers | Consent deny visible | Full power-cycle result | Tester |
|------|----------|------------|-------------|-------------------------------|--------------------|----------------------|-------------------------|--------|

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
