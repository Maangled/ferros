# FERROS Hub — Reference Hardware

> This document records tested hardware configurations for `ferros-hub`. It is updated when a new hardware target is confirmed working for G4 (launch gate).

---

## Status

⬜ No hardware confirmed yet. This document is a scaffold.

---

## Minimum requirements

| Component | Minimum | Notes |
|-----------|---------|-------|
| Architecture | `x86_64` or `aarch64` | Linux ABI required |
| RAM | 512 MB | 1 GB+ recommended |
| Storage | 4 GB | Persistent profile + grant storage |
| OS | Linux (any modern distribution) | Tested on Debian/Ubuntu; others may work |
| Network | Ethernet or Wi-Fi | Required for Home Assistant integration |

---

## Tested configurations

_(Fill in as hardware is confirmed)_

| Hardware | Architecture | OS | Status | Notes |
|----------|--------------|----|--------|-------|
| Raspberry Pi 4 (4 GB) | aarch64 | Raspberry Pi OS (64-bit) | ⬜ Planned | Primary launch target |
| x86_64 home server (Intel NUC or equivalent) | x86_64 | Ubuntu 22.04 LTS | ⬜ Planned | Secondary launch target |

---

## Confirmed working (G4 evidence)

_(Fill in when G4 evidence is recorded)_

| Date | Hardware | Architecture | OS version | `ferros-hub` version | Tester |
|------|----------|--------------|------------|---------------------|--------|

---

## Notes

- Cross-compilation for `aarch64-unknown-linux-gnu` is handled in CI. See `.github/workflows/ci.yml` (to be added in S1).
- The Pi 4 is the preferred first target because it is the most common home-server/edge platform and has a known-good `aarch64-linux-gnu` toolchain.
- Do not declare G4 closed based on a QEMU emulation of the Pi. Real hardware is required.
