# Personal Profile — Progress Spec

**Current:** 1%
**Phase:** Active
**Status:** Prototype exists in `docs/personal-profile.html` and is the most developed surface in the repo.

## What This Is

This spec tracks the portable identity and progression root implemented in `docs/personal-profile.html`. It is the active Phase 0 development surface and already contains onboarding, consent handling, session modes, portability flows, and the seal-chain-backed model that other FERROS surfaces are expected to consume.

## Milestone Gates

| % | Gate | Deliverables | Done? |
|---|------|-------------|-------|
| 10% | Consent-safe shell | Genesis shell, staged onboarding, and `file://` compatibility exist without violating the consent model | ☐ |
| 20% | Onboarding cannot soft-lock | The consent path, stage flow, and critical entry interactions are robust enough that new users can always progress | ☐ |
| 30% | Portable identity modes | Full profile, session, alias, and recovery identity modes work with distinct storage and attribution rules | ☐ |
| 40% | Module and Deck identity layer | Template, module, Card, and Deck identity concepts are available for reuse by other FERROS surfaces | ☐ |
| 50% | Completed-profile onboarding | A user can arrive and end up with a basically complete, useful profile quickly without backend services | ☐ |
| 60% | Calendar consumption | Calendar or schedule systems can consume profile defaults, preferences, and composition data | ☐ |
| 70% | External prefill path | External integrations can prefill or hydrate profile data through documented contracts | ☐ |
| 80% | Verification and claims | Verification status, claims, and portable proof flows are documented and repo-visible | ☐ |
| 90% | Consistency audit | Achievement triggers, progression locks, and session protections are reviewed for internal consistency | ☐ |
| 100% | Production / Complete | Personal Profile is a production portable identity root for consent, progression, attribution, and verification | ☐ |

## Dependencies

- `README.md` — canonical status, implemented feature list, and Phase 0 framing.
- `docs/personal-profile.html` — current implementation surface.
- `docs/AGENT_GUIDE.md` — binding bug history and constraints for storage, hashing, and stage flow.
- `docs/adr/ADR-001-progression-lock-pattern.md` — seal chain and progression stages.
- `docs/adr/ADR-003-alias-system.md` — alias identity and claim flow.
- `docs/adr/ADR-004-template-profile-specification.md` — template schema baseline.
- `docs/adr/ADR-005-cross-device-identity-and-session-modes.md` — four-mode identity architecture.
- `docs/adr/ADR-006-level-zero-adaptive-onboarding.md` — adaptive onboarding and robot-helper constraints.
- `docs/adr/ADR-011-routine-module-system.md` — modules, starter decks, and schedule composition direction.

## Current Blockers

- `docs/AGENT_GUIDE.md` still calls for a bug sweep, achievements audit, and template expansion.
- The broader identity system remains an ecosystem bottleneck for cross-surface integration.
- Profile interoperability contracts for calendar, runtime, and external systems are still only partially defined.