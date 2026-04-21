# FERROS — Launch Definition

> This document defines what **launch** means for FERROS. It also explicitly states what does **not** count as launch.

---

## What launch is

**Launch = `ferros-hub` v0.2.0 running on real home hardware (Raspberry Pi or home server), with consent enforced end-to-end and Home Assistant integration active.**

Specifically, the following conditions must all be true simultaneously:

1. A single `ferros-hub` binary runs on an `aarch64-linux` or `x86_64-linux` home device (not a CI VM, not QEMU, not a developer laptop).
2. The device holds a real device profile (Ed25519 keypair, device-bound, created via `ferros profile init`).
3. At least one real Home Assistant entity is registered through the FERROS agent center — not mocked, not stubbed.
4. Capability consent is enforced: an agent that requests a capability it was not granted is denied at the bus level, logged, and the denial is visible in the HA UI or `ferros agent logs`.
5. The hub survives a full power cycle. After reboot: profile loads, agents re-register, HA entity is restored.
6. A private beta install exists on at least one home setup that is not the primary developer's machine.

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
| Home Assistant integration is drafted | Draft ≠ running on hardware |
| Fewer than two independent home installs | One install could be a fluke; ≤5 private beta installs required before wider mention |

---

## Why this definition matters

Redefining "launch" as a website event or a crates publish creates pressure to ship a facade. FERROS is a **home-sovereign system**. The only honest launch is a running system on a device in someone's home, doing real work with real consent enforcement.

This definition also prevents premature scaling. Do not announce, write blog posts, or invite a broader community until G4 is green and at least two independent private-beta installs are confirmed working.

---

## Launch checklist (G4)

See `docs/gates/G4.md` for the full entry criteria, required evidence, and exit criteria.

Short form:

- [ ] `ferros-hub` binary compiles for `aarch64-unknown-linux-gnu`
- [ ] Boots and loads profile on Pi / home server
- [ ] `ferros agent list` shows at least one registered agent
- [ ] At least one real HA entity registered and visible in HA dashboard
- [ ] Consent deny verified: ungranted capability request is blocked and logged
- [ ] Power cycle survival confirmed
- [ ] At least one independent private-beta install confirmed
- [ ] `LAUNCH.md` updated with install date and hardware spec

---

## Post-launch scope

After G4 is green:

- Announce privately to ≤5 trusted users. Gather feedback for 2–4 weeks before any public mention.
- Address critical issues found in private beta before opening a public channel.
- Tag `v0.2.0` only after private beta is stable.
- Wider community, documentation, and contributor onboarding follow `v0.2.0`.
