# Templates & Profiles — Progress Spec

**Current:** 1%
**Phase:** Prototype
**Status:** Template profiles exist, but the system needs major expansion and a broader shared contract.

## What This Is

This spec tracks the template profile and profile-shape layer that currently appears in `docs/personal-profile.html` and the template ADRs. The repo already contains hardcoded template profiles and seeded schedules, but the system is still narrow in coverage and not yet generalized across FERROS surfaces.

## Milestone Gates

| % | Gate | Deliverables | Done? |
|---|------|-------------|-------|
| 10% | Schema baseline | Template profile schema is documented in the ADR set and reflected in the active prototype | ☐ |
| 20% | Initial template set | Repo contains a concrete starter set of hardcoded templates with visible selection flow | ☐ |
| 30% | Seeded schedule linkage | Template profiles and seeded schedules connect in the profile onboarding flow | ☐ |
| 40% | Profile portability rules | Template-derived profiles and alias usage rules are documented against identity constraints | ☐ |
| 50% | Local testable flow | A user can locally browse templates, preview them, and fork or use them in the existing prototype | ☐ |
| 60% | Shared profile contract | Repo-visible docs define how template/profile data should be consumed outside the profile page | ☐ |
| 70% | Expansion path | Additional archetypes, migration rules, or module-driven profile composition are documented | ☐ |
| 80% | Cross-surface profile use | Other FERROS surfaces have a documented path for consuming template or profile metadata | ☐ |
| 90% | Schema audit | Older template rules and newer addenda are reconciled into a stable documentation path | ☐ |
| 100% | Production / Complete | Templates and profiles have a stable shared schema, documented migration path, and platform-wide consumption contract | ☐ |

## Dependencies

- `README.md` — canonical percentage and status note.
- `docs/personal-profile.html` — current template browsing and use flow.
- `docs/adr/ADR-004-template-profile-specification.md` — original schema baseline and template set.
- `docs/adr/ADR-011-routine-module-system.md` — newer addendum context for extended profile composition.
- `docs/adr/ADR-003-alias-system.md` — alias use of template identities.

## Current Blockers

- README explicitly says the current template profile system needs major expansion.
- The repo contains an older template baseline plus a newer superseding direction, but not a single stabilized shared contract.
- Cross-surface use of templates outside the profile prototype is still TBD.