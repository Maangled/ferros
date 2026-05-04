# ADR-033 - Home topology and onramp source lineage

**Status:** Draft  
**Date:** 2026-05-04  
**Stream:** S7 / S5 / Cross-cutting  
**Deciders:** Maangled  
**Domain tags:** architecture / UX doctrine / policy / real-world application / cross-cutting  
**Primary evidence basis:** Research or precedent proof

_See [ADR-023-onramp-policy.md](./ADR-023-onramp-policy.md), [ADR-028-core-launch-and-optional-module-lanes.md](./ADR-028-core-launch-and-optional-module-lanes.md), [../hub/reference-hardware.md](../hub/reference-hardware.md), and [../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md](../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md)._ 

---

## Context

The Home-Hub lane now needs a stable way to show topology, source lineage, and proposed material without letting Home Assistant or any other external source become identity truth. The paper-era home-topology motifs are useful for operator readability, but the FERROS version must stay within the current onramp and module-lane boundaries.

---

## Decision

**If accepted, FERROS will present Home-Hub topology and source lineage as observation and proposal surfaces only until explicit acceptance or named evidence upgrades their status.**

This draft scopes how topology and source lineage should be described, not the final rendering details.

---

## Rationale

Operators need a clear map of where proposed material came from, what seam adapted it, and whether it is still runway, staged, or evidence-backed.

### Options considered

| Option | Summary | Reason not chosen (if rejected) |
|--------|---------|--------------------------------|
| Option A (draft target) | Show topology and source lineage as observation-first, proposal-safe surfaces | - |
| Option B | Treat Home-Hub topology as implied trust once a bridge is configured | Rejected because it conflicts with ADR-023 and current module-lane boundaries |
| Option C | Hide topology until full accept/reject and runtime control are finished | Rejected because honest observation and blocked-state UX still help operators now |

---

## Consequences

**Positive:**
- Home-Hub surfaces can become useful before the full control path exists.
- Operators can distinguish local runway state from canonical or launch-grade evidence.
- Source-lineage displays get a stable home.

**Negative / trade-offs:**
- Requires careful blocked-state language to avoid overclaiming.
- Some useful topology graphics may stay observational longer than desired.

---

## Compliance

- Do not treat external Home-Hub sources as identity truth.
- Do not present proposed topology or entity data as canonical before explicit acceptance.
- Revisit this ADR if a later launch revision makes any home-topology surface safety-critical.

---

## Implementation Evidence

- Current policy: [ADR-023-onramp-policy.md](./ADR-023-onramp-policy.md) and [ADR-028-core-launch-and-optional-module-lanes.md](./ADR-028-core-launch-and-optional-module-lanes.md).
- Current hub groundwork: [../hub/reference-hardware.md](../hub/reference-hardware.md) and the Pack B / Pack C findings chain.

---

## Deferred Scope or Open Research

- Deferred: exact topology-card structure and iconography.
- Deferred: whether Home-Hub source lineage should be one card family or a shared module also used by Forge and Arena.
- Deferred: how restart-safe or evidence-backed topology states map into badges.

---

## References

- [ADR-023-onramp-policy.md](./ADR-023-onramp-policy.md)
- [ADR-028-core-launch-and-optional-module-lanes.md](./ADR-028-core-launch-and-optional-module-lanes.md)
- [../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md](../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md)
