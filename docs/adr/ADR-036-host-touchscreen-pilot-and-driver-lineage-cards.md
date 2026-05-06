# ADR-036 - Host touchscreen pilot and driver-lineage cards

**Status:** Draft
**Date:** 2026-05-05
**Stream:** S5 / S8 / Cross-cutting
**Deciders:** Maangled
**Domain tags:** architecture / UX doctrine / policy / research / real-world application / cross-cutting
**Primary evidence basis:** Research or precedent proof

_See [ADR-010-cards-and-decks-nomenclature.md](./ADR-010-cards-and-decks-nomenclature.md), [ADR-026-ai-assistant-card-deck-ide.md](./ADR-026-ai-assistant-card-deck-ide.md), [ADR-029-human-operator-session-orchestration-and-evidence-flow.md](./ADR-029-human-operator-session-orchestration-and-evidence-flow.md), [ADR-033-home-topology-and-onramp-source-lineage.md](./ADR-033-home-topology-and-onramp-source-lineage.md), and [../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md](../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md)._ 

---

## Context

FERROS now has a localhost shell that is close enough to real operator work that it should be exercised on a dedicated touchscreen surface, but current human testing still runs on a Linux host rather than the later FERROS subcore. That means display, input, compositor, and USB or GPU driver state sit outside the current FERROS evidence chain. The immediate need is to make touchscreen sessions useful without lying about that boundary. The longer-range need is to let ACC and the later card/deck IDE represent those host-side dependencies as auditable cards so humans and agents can inspect, trace, and eventually challenge them rather than treating the rendering/input stack as an invisible assumption.

---

## Decision

**FERROS will treat pre-subcore touchscreen sessions on the current Linux host as explicit host-module pilots: operator packets must record connector path and host driver metadata, and future audit surfaces may project those host dependencies as driver-lineage cards without claiming FERROS-managed integrity yet.**

The working idea that interaction capture could later auto-project into reusable cards and decks is preserved here only as deferred research under the provisional label `Autosave`.

---

## Rationale

The hub concept should be proven on a real touch-adjacent operator surface now, even if the driver chain is not yet FERROS-owned. Recording the host stack explicitly gives FERROS a safer path: honest current testing, explicit evidence ceilings, and a future card/deck audit model that can later be tightened when subcore-owned drivers exist.

### Options considered

| Option | Summary | Reason not chosen (if rejected) |
|--------|---------|--------------------------------|
| Option A (chosen) | Run Linux-host touchscreen sessions now, but record connector path, host drivers, and evidence ceiling explicitly and treat them as future card candidates | - |
| Option B | Defer all touchscreen work until FERROS subcore display and input drivers exist | Rejected because it delays operator-surface proof and blocks useful hub-adjacent testing on the current machine |
| Option C | Treat current Linux-host touchscreen drivers as de facto trusted FERROS evidence | Rejected because it overclaims integrity and blurs the line between host dependencies and FERROS-owned runtime proof |

---

## Consequences

**Positive:**
- FERROS can begin touch-screen operator testing on the current Linux machine without pretending the driver chain is already sovereign.
- Operator findings can capture concrete connector and driver facts now, making later subcore comparisons and audits easier.
- ACC and the future card/deck IDE gain a real candidate domain for cardized system values: display path, input stack, driver modules, and audit assignments.

**Negative / trade-offs:**
- Current touch-screen sessions carry a weaker evidence ceiling than later subcore-owned driver sessions.
- Human-test packets become slightly heavier because they must capture host stack context, not just UI observations.
- The idea of automatic interaction-to-card capture is intentionally left unratified, so some desired workflow remains manual for now.

---

## Compliance

- Do not describe Linux-host touchscreen sessions as FERROS-managed driver proof.
- Do not imply that current host display/input drivers are audited, modularized, or sealed by FERROS.
- Record connector path and observed driver metadata whenever a touchscreen session is part of operator evidence.
- Revisit this ADR when subcore-owned display/input driver work exists or when ACC can actually project and assign driver-lineage cards.

---

## Implementation Evidence

- Current touch-shell posture: [../../site/agent-center-shell.html](../../site/agent-center-shell.html) and [../../harnesses/localhost-shell-acceptance-harness.html](../../harnesses/localhost-shell-acceptance-harness.html).
- Current operator-session loop: [ADR-029-human-operator-session-orchestration-and-evidence-flow.md](./ADR-029-human-operator-session-orchestration-and-evidence-flow.md), [../backlogs/HUMAN-TEST-BACKLOG.md](../backlogs/HUMAN-TEST-BACKLOG.md), and [../operator-sessions/OPS-2026-05-05-HTB-004-onramp-review.md](../operator-sessions/OPS-2026-05-05-HTB-004-onramp-review.md).
- Card/deck projection direction: [ADR-026-ai-assistant-card-deck-ide.md](./ADR-026-ai-assistant-card-deck-ide.md).

---

## Deferred Scope or Open Research

- Deferred: auditing or modularizing host display/input drivers on the current Linux machine.
- Deferred: defining the exact card family for host driver lineage, connector path, and rendering-stack provenance.
- Deferred: automatic interaction capture into card/deck artifacts under the provisional label `Autosave`.
- Deferred: broader philosophical or debate-deck semantics for cards such as `Reason`; those remain research and nomenclature work rather than a frozen operator-session policy.

---

## References

- [ADR-010-cards-and-decks-nomenclature.md](./ADR-010-cards-and-decks-nomenclature.md)
- [ADR-026-ai-assistant-card-deck-ide.md](./ADR-026-ai-assistant-card-deck-ide.md)
- [ADR-029-human-operator-session-orchestration-and-evidence-flow.md](./ADR-029-human-operator-session-orchestration-and-evidence-flow.md)
- [ADR-033-home-topology-and-onramp-source-lineage.md](./ADR-033-home-topology-and-onramp-source-lineage.md)
- [../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md](../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md)
- [../architecture-overview.md](../architecture-overview.md)