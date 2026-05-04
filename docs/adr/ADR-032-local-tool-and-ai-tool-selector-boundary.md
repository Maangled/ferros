# ADR-032 - Local tool and AI tool selector boundary

**Status:** Draft  
**Date:** 2026-05-04  
**Stream:** Cross-cutting / S5 / S7  
**Deciders:** Maangled  
**Domain tags:** architecture / policy / ecosystem / UX doctrine / cross-cutting  
**Primary evidence basis:** Research or precedent proof

_See [ADR-023-onramp-policy.md](./ADR-023-onramp-policy.md), [ADR-027-service-parity-broker-and-assurance-tiers.md](./ADR-027-service-parity-broker-and-assurance-tiers.md), [ADR-028-core-launch-and-optional-module-lanes.md](./ADR-028-core-launch-and-optional-module-lanes.md), and [../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md](../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md)._ 

---

## Context

The next shell wants visible tool lanes for Home Assistant, local LLM runtimes, external LLM APIs, and similar modules. FERROS needs one boundary for how these lanes are disclosed so operators can tell what is local, what is external, what can read or write, what artifacts are emitted, and what capabilities are required.

---

## Decision

**If accepted, FERROS will require every tool lane to declare transport scope, read/write ability, required capabilities, artifacts emitted, and whether results are evidentiary, staged, or non-evidentiary.**

This draft does not yet freeze the exact manifest or endpoint shape.

---

## Rationale

The UX should not allow an operator to confuse a visible tool lane with a hidden sync path or ambient write authority.

### Options considered

| Option | Summary | Reason not chosen (if rejected) |
|--------|---------|--------------------------------|
| Option A (draft target) | Explicit tool-lane disclosure for every module lane | - |
| Option B | Let each integration surface describe itself ad hoc | Rejected because disclosure quality will drift and confuse operators |
| Option C | Hide tool-lane mechanics entirely until all module lanes are fully built | Rejected because blocked and staged states are still useful and should remain honest |

---

## Consequences

**Positive:**
- Cleaner distinction between local and external tools.
- Better operator-session checks for unexpected write authority.
- Natural place for future module-lane manifests.

**Negative / trade-offs:**
- Requires manifest or adapter work that does not yet exist.
- UI may expose a large amount of disclosure detail that needs careful simplification.

---

## Compliance

- Do not present a tool lane without declaring local vs. external scope.
- Do not present write authority as implicit or ambient.
- Revisit this ADR if service-parity or detachable-provider rules evolve enough to require a richer selector model.

---

## Implementation Evidence

- Policy foundation: [ADR-027-service-parity-broker-and-assurance-tiers.md](./ADR-027-service-parity-broker-and-assurance-tiers.md) and [ADR-028-core-launch-and-optional-module-lanes.md](./ADR-028-core-launch-and-optional-module-lanes.md).
- UX seed: [../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md](../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md).

---

## Deferred Scope or Open Research

- Deferred: exact tool-lane manifest schema.
- Deferred: whether local and external tool lanes share one disclosure card or multiple card families.
- Deferred: how service-parity broker details map into operator-facing disclosure.

---

## References

- [ADR-023-onramp-policy.md](./ADR-023-onramp-policy.md)
- [ADR-027-service-parity-broker-and-assurance-tiers.md](./ADR-027-service-parity-broker-and-assurance-tiers.md)
- [ADR-028-core-launch-and-optional-module-lanes.md](./ADR-028-core-launch-and-optional-module-lanes.md)
