# FERROS — Launch Definition

> This document defines what **launch** means for FERROS. It also explicitly states what does **not** count as launch.

---

## What launch is

**Launch = `ferros-hub` v0.2.0 running on real home hardware (Raspberry Pi or home server), with consent enforced end-to-end, reboot-safe core behavior, and a coordinated clean install path.**

Optional integrations such as Home Assistant, local LLM runtimes, and external LLM APIs are post-install module lanes. They can advance in parallel, but they do not block the core `v0.2.0` launch definition.

Specifically, the following conditions must all be true simultaneously:

1. A single `ferros-hub` binary runs on an `aarch64-linux` or `x86_64-linux` home device (not a CI VM, not QEMU, not a developer laptop).
2. The device holds a real device profile (Ed25519 keypair, device-bound, created via `ferros profile init`).
3. `ferros agent list` shows at least one real registered agent on the running hub.
4. Capability consent is enforced: an agent that requests a capability it was not granted is denied at the bus level, logged, and the denial is visible through `ferros agent logs` or another operator-visible FERROS surface.
5. The hub survives a full power cycle. After reboot: profile loads, agents re-register, and core runtime state is restored without manual intervention.
6. A coordinated clean install or reprovision exists on at least one additional lab device or fresh target image beyond the primary server. This does not require an unmanaged third-party install.

---

## What does not count as launch

The following events are progress milestones, not launch:

| Event | Why it is not launch |
|-------|----------------------|
| A PR merges | Code landing is table stakes, not the product |
| The site goes live | A website is marketing; launch is a running system |
| `crates.io` publish | Publishing a library is not a shipped product |
| QEMU or CI demo | Virtualized execution does not prove real-hardware readiness |
| A laptop demo | Not a real home install |
| `v0.1.0-rc` tagged | MVP gate — proves the system is real, not that it is deployed |
| Agent center shell works in browser | S5 Phase B milestone — important, but launch is hardware-first |
| An optional module is drafted | A drafted Home Assistant, local LLM, or external API module does not define core launch |
| A future unmanaged independent install | Valuable later, but strict independence is deferred until the controlled test-home rollout |

---

## Why this definition matters

Redefining "launch" as a website event or a crates publish creates pressure to ship a facade. FERROS is a **home-sovereign system**. The only honest launch is a running system on a device in someone's home, doing real work with real consent enforcement.

This definition also prevents premature scaling. FERROS is on a lab-first rollout: first make the core hub boring on the primary server and additional coordinated devices, then work through controlled test homes, and only after that ask for unmanaged independent installs.

---

## Launch checklist (G4)

See `docs/gates/G4.md` for the full entry criteria, required evidence, and exit criteria.

Short form:

- [x] `ferros-hub` binary builds for the chosen Pack B `x86_64-unknown-linux-gnu` launch target
- [x] Boots and loads profile on the chosen home-server lane
- [x] `ferros agent list` shows registered agents on the running hub
- [x] Consent deny verified: ungranted capability request is blocked and logged on an operator-visible FERROS surface
- [x] Power cycle survival confirmed
- [x] At least one coordinated clean install or reprovision confirmed on additional lab-controlled hardware
- [x] `LAUNCH.md` now records the current install date and hardware spec

---

## Current Core Launch Record

The current core launch lane is the Pack B `x86_64` home-server path centered on `homelab001`.

| Field | Current record |
|-------|----------------|
| Primary hardware lane | Physical Pack B `x86_64` home-server class DUT `homelab001` |
| Hardware specification | `Linux 6.8.0-101-generic x86_64 GNU/Linux` on the named physical host, as captured in the Pack B profile baseline packet |
| First confirmed target-side provisioning date | `2026-05-03` |
| Coordinated secondary-device reprovision evidence | `2026-05-04` Windows fresh-host packet with a fresh explicit profile path and temp-rooted local FERROS state root |
| Rollout note | Strict unmanaged independent installs are deferred until after coordinated lab rollout and later controlled test homes |

This section records the current launch anchor for the closed G4 launch packet.

`v0.2.0` was tagged on `2026-05-04` from this Pack B launch lane.

---

## Post-launch scope

After G4 is green:

- Tag `v0.2.0` once the core launch packet is complete and stable on the chosen hardware lane.
- Continue coordinated rollout across the server plus additional lab devices.
- Validate optional module lanes such as Home Assistant, local LLM runtimes, and external LLM APIs without treating them as core launch blockers.
- Use controlled test homes to work out the final kinks before asking for unmanaged independent installs or making wider public claims.
