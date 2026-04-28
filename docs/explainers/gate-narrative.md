# FERROS Gate Narrative — Plain-English Guide

**Date:** 2026-04-27  
**Owning stream:** S8 (docs and governance)  
**Status:** Explainer — no gate authority. Does not modify gate documents. Does not claim evidence.

---

## Who This Is For

This explainer is written for someone who is not a Rust developer and has not read the FERROS technical specs. It explains what the FERROS project's five milestone markers (G1, G2, G3, D1, G4) mean, in plain language, and why they exist.

---

## The Big Picture

FERROS is building a trust layer for AI agents running on home hardware. Before it is launched to the public, the project works through a sequence of milestone gates. Each gate proves something concrete — not by announcement, but by running code and collecting evidence that can be read by anyone.

The milestone sequence is: **G1 → G2 → G3 → D1 → G4 (launch)**.

Gates are not marketing events. They are checkpoints. A gate is "closed" only when the required evidence exists in the repository.

---

## G1 — CI Green on Three Platforms

**Plain English:** The FERROS code compiles and all tests pass on Linux, macOS, and Windows — automatically, on every code change.

**What was proved:**
- The Rust workspace builds without errors on three different operating systems.
- All tests pass.
- Code style and lint checks pass.
- The codebase is reproducibly buildable, not just "it works on my machine."

**What was NOT proved:**
- That FERROS does anything useful on a real device.
- That any agent has run.
- That the consent system works end-to-end.

**Current status:** G1 is closed. The CI pipeline runs on every push and maintains this guarantee.

---

## G2 — Profile Identity and CLI Lifecycle Proven

**Plain English:** A user (or operator) can create a FERROS profile, show it, export it, and import it — and those operations are tested and proven to work correctly.

**What was proved:**
- `ferros profile init`, `ferros profile show`, `ferros profile export`, and `ferros profile import` all work.
- The profile data shape is defined and frozen in a published schema (`profile.v0.json`).
- The profile is stable — once created, it can be round-tripped through export and import without data loss.

**What was NOT proved:**
- That agents can run using the profile.
- That capability grants are enforced.

**Current status:** G2 is closed. The profile schema is frozen and the CLI verbs are tested.

---

## G3 — Agent Center and Runtime Convergence

**Plain English:** The agent center (the list of agents and their capabilities) and the runtime (the policy engine that decides what agents are allowed to do) work together and are proven in CI tests.

**What was proved:**
- Agents can be registered in the agent center with a manifest describing their required capabilities.
- The runtime's deny-by-default policy correctly allows or denies capability requests based on the grant set.
- The executor (the scheduler) runs jobs in order.
- The message bus routes messages between agents correctly.
- All of the above is CI-tested on multiple platforms.

**What was NOT proved:**
- That any of this runs on real hardware.
- That a real Home Assistant device can be connected.
- That the system survives a power cycle.

**Current status:** G3 is closed. The convergence tests run in CI.

---

## D1 — Operator-Attended Single-Device Demo

**Plain English:** A real operator, on a real device, demonstrates that FERROS works end-to-end. This is a live demo, not automated CI. The operator captures a transcript as evidence.

### D1 is NOT launch. D1 is NOT G4.

D1 proves that FERROS works on at least one real device in at least one operator's hands. It does not prove that FERROS is ready for other people to install independently.

**What D1 requires:**

1. **Profile creation on the target device.** `ferros profile init` and `ferros profile show` run successfully on the actual device the operator is using — not in CI, on the real machine.

2. **An agent registered (or a named stand-in for the HA bridge).** The agent center must have at least one registered agent. For D1, a stand-in agent is acceptable in place of the real Home Assistant bridge, as long as the stand-in is named and uses a defined bridge seam.

3. **Consent flow visible.** The deny-by-default enforcement must be demonstrable. The operator must be able to show the deny-log — at least one denied request, visible in the localhost shell or CLI output — and explain that it is a policy decision, not a system error.

4. **Reboot-safe FERROS-side state.** After a full power cycle (power off, power on), the FERROS profile loads and at least one agent re-registers without manual intervention.

**What D1 does NOT require:**
- A Raspberry Pi specifically (an x86_64 home server or laptop qualifies).
- The real HA bridge (a named stand-in is acceptable).
- Unattended operation (a human is present for the demo).
- Multiple devices.
- Independent installation by a third party.

**Current status:** D1 is active runway. It is not closed. The evidence has not been collected.

---

## G4 — Launch

**Plain English:** FERROS is running on a real device (Raspberry Pi or home server), with a real Home Assistant setup, with consent enforced, surviving reboots, and ready for someone other than the core team to install it independently.

**What G4 requires (everything D1 requires, plus):**

- A real Raspberry Pi or home server (not just a laptop).
- A real Home Assistant entity registered through the FERROS bridge (not a stand-in).
- Consent denials observable in the HA dashboard or `ferros agent logs`.
- The system survives a full power cycle and the HA entity re-registers automatically.
- An independent private-beta installation — someone outside the core team successfully installs FERROS following only the published instructions, without being walked through it.

**Current status:** G4 is active. It is not closed. It depends on D1, plus:
- `ferros-hub` binary (not yet built).
- The real HA bridge implementation (not yet built).
- A target device (hardware not yet selected as of 2026-04-27).

---

## Why This Sequence Exists

| Gate | What it proves | Why it matters |
|---|---|---|
| G1 | The code compiles and tests pass everywhere | Trust that the codebase is real and reproducible |
| G2 | Identity and profile management work | Every downstream feature depends on knowing who is asking |
| G3 | Capability enforcement works in integration | The consent system is not just theory — it runs and is tested |
| D1 | A real human can use it on a real device | Proves the gap between "it works in CI" and "it works in practice" is closed |
| G4 | A third party can install and use it independently | The definition of a real launch |

---

## The D1 ≠ G4 Distinction (In Plain English)

D1 and G4 are often confused. Here is the clearest way to say the difference:

- **D1:** The FERROS team proves it to themselves, in a controlled session, with one device, and documents the proof.
- **G4:** A person who is not on the FERROS team proves it to themselves, independently, and it works.

D1 is a milestone on the road to launch. G4 is launch. You cannot skip D1 and call it G4. You also cannot declare G4 by doing what D1 asks.

---

## Glossary

| Term | Plain-English meaning |
|---|---|
| Gate (G1/G2/G3/G4) | A milestone that is proved by code and documented evidence |
| D1 | Operator-attended demo on one real device (not a gate in the G-series, but a required milestone before G4) |
| CI | Continuous Integration — automated tests that run on every code change |
| Deny-by-default | The policy that all capability requests are denied unless there is an explicit active grant |
| Profile | An identity record that identifies who (or what device) is making a request |
| Capability grant | A record that says "profile X is allowed to do capability Y" |
| Agent | A software module registered with the FERROS agent center that requests capabilities to do work |
| Power cycle | Turning a device fully off and back on — not just restarting the software |
| Stand-in | A placeholder agent that proves the plumbing works without implementing the final feature |

---

## Source Documents (for the technically curious)

- `docs/gates/G1.md`, `docs/gates/G2.md`, `docs/gates/G3.md`, `docs/gates/G4.md` — gate evidence (authoritative)
- `docs/gates/D1.md` — D1 milestone evidence requirements
- `LAUNCH.md` — authoritative definition of what launch means for FERROS
- `STATUS.md` — current project state
