# ADR-028 - Core launch boundary and optional module lanes

**Status:** Accepted  
**Date:** 2026-05-04  
**Stream:** Cross-cutting / S7 / S8  
**Deciders:** Maangled  
**Domain tags:** architecture / governance / policy / launch / ecosystem / cross-cutting  
**Primary evidence basis:** Operational proof

_See [ADR-022-decision-program-governance.md](./ADR-022-decision-program-governance.md), [_INDEX.md](./_INDEX.md), and [../../DOCTRINE.md](../../DOCTRINE.md). Cross-reference [ADR-023-onramp-policy.md](./ADR-023-onramp-policy.md) for the inbound-external-system consent invariant and [ADR-025-dual-root-hardware-runway.md](./ADR-025-dual-root-hardware-runway.md) for the current real-hardware runway structure._

---

## Context

The written `v0.2.0` launch definition had drifted into treating Home Assistant as a mandatory launch blocker. That choice drove multiple days of operational work on the HA bridge even though the repo structure already kept `ferros-hub` as a distinct crate and the broader product direction still centers on raw FERROS core behavior on the primary server and other lab devices. The current rollout plan is lab-first: prove the core hub on the main server, repeat the bring-up on additional coordinated lab-controlled devices, then move into controlled test homes with known IoT devices. Strict unmanaged independent installs are valuable later, but they are not the next honest blocker. At the same time, Home Assistant remains useful, and local LLM runtimes or external LLM APIs may become even more important near-term module lanes. FERROS therefore needs an explicit boundary between core launch readiness and optional integration-module readiness.

---

## Decision

**FERROS `v0.2.0` launch is defined by core hub readiness on real hardware, while Home Assistant, local LLM runtimes, external LLM APIs, and similar external surfaces are optional post-install module lanes that do not block the current launch gate.**

The immediate install bar is one coordinated clean install or reprovision on additional lab-controlled hardware. Strict unmanaged independent installs are deferred until after the coordinated lab rollout and later controlled test-home phase.

---

## Rationale

This option keeps the repo aligned with the actual product path instead of letting one useful integration become the entire launch definition.

### Options considered

| Option | Summary | Reason not chosen (if rejected) |
|--------|---------|--------------------------------|
| Option A (chosen) | Core FERROS launch is the blocker; external integrations ship as optional module lanes | - |
| Option B | Keep Home Assistant as a mandatory G4 blocker and continue using strict independent-install wording immediately | Rejected because it makes a side integration dominate the launch path and sets an install bar that does not match the current coordinated lab rollout plan |
| Option C | Remove Home Assistant from planning entirely and discard the existing bridge findings | Rejected because the HA work is still valuable module-lane evidence and the same optional-module posture should also support future LLM-facing integrations |

---

## Consequences

**Positive:**
- Core launch can now advance on the server and additional lab devices without waiting on HA-specific restoration behavior.
- Existing HA findings stay useful as module-lane evidence instead of being thrown away.
- The install bar now matches the real rollout sequence: primary server, additional coordinated devices, then controlled test homes, then unmanaged installs.
- Local LLM runtimes and external LLM APIs can be planned as first-class module lanes instead of afterthoughts.

**Negative / trade-offs:**
- `v0.2.0` becomes a narrower claim than the earlier "hub with active HA integration" wording.
- A separate module-packaging or module-discovery decision is still needed later if optional integrations are to ship cleanly.
- Historical findings that were written under the old HA-blocker launch definition now need explicit historical wording so they are not mistaken for the current policy.

---

## Compliance

- If a future launch revision requires any external integration for safety-critical or minimum useful product behavior, revisit this ADR.
- If optional integrations cannot be packaged, installed, upgraded, or removed cleanly enough to remain optional in practice, revisit this ADR.
- When the controlled test-home phase begins, revisit the install criteria and decide when unmanaged independent installs become a real blocker.

---

## Implementation Evidence

- Operational proof: [docs/gates/D1.md](../gates/D1.md), [docs/hardware/findings/FINDINGS-pack-b-session-01-profile-baseline.md](../hardware/findings/FINDINGS-pack-b-session-01-profile-baseline.md), and [docs/hardware/findings/FINDINGS-pack-c-session-06-windows-fresh-host.md](../hardware/findings/FINDINGS-pack-c-session-06-windows-fresh-host.md) together show the current core hardware lane plus a coordinated second-device reprovision path.
- Policy surfaces: [../../LAUNCH.md](../../LAUNCH.md), [../gates/G4.md](../gates/G4.md), and [../hub/reference-hardware.md](../hub/reference-hardware.md) now encode the core-launch versus module-lane split.

---

## Deferred Scope or Open Research

- Deferred: module packaging, discovery, install, upgrade, and removal UX.
- Deferred: whether Home Assistant, local LLM runtimes, and external LLM APIs share one module manifest model or need different module classes.
- Deferred: the exact trigger that upgrades unmanaged independent installs from a deferred milestone into a required gate.

---

## References

- [ADR-023-onramp-policy.md](./ADR-023-onramp-policy.md)
- [ADR-025-dual-root-hardware-runway.md](./ADR-025-dual-root-hardware-runway.md)
- [../../LAUNCH.md](../../LAUNCH.md)
- [../gates/G4.md](../gates/G4.md)
- [../hardware/findings/FINDINGS-pack-c-session-03-agent-center-entity.md](../hardware/findings/FINDINGS-pack-c-session-03-agent-center-entity.md)
- [../hardware/findings/FINDINGS-pack-c-session-05-ha-restoration.md](../hardware/findings/FINDINGS-pack-c-session-05-ha-restoration.md)
- [../hardware/findings/FINDINGS-pack-c-session-06-windows-fresh-host.md](../hardware/findings/FINDINGS-pack-c-session-06-windows-fresh-host.md)