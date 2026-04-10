# Personal Profile — Progress Spec

**Current:** 1%
**Phase:** Active
**Status:** Prototype exists in `docs/personal-profile.html` and is the most developed surface in the repo.

## What This Is

This spec tracks the RPG-style progression dashboard implemented in `docs/personal-profile.html`. It is the active Phase 0 development surface and already contains onboarding, consent handling, session modes, template profiles, portability flows, and the seal-chain-backed progression model documented in the ADR set.

## Milestone Gates

| % | Gate | Deliverables | Done? |
|---|------|-------------|-------|
| 10% | Genesis shell | Genesis page, staged onboarding shell, and local file-open compatibility exist in one standalone HTML file | ☐ |
| 20% | Consent boundary | Trade Window, session decline flow, and guarded persistence model are implemented and repo-visible | ☐ |
| 30% | Identity modes | Full profile, session mode, alias mode, and recovery mode are implemented with distinct storage rules | ☐ |
| 40% | Progression integrity | Seal-chain creation, verification, and fallback hashing are implemented in ways that work on `file://` | ☐ |
| 50% | Self-serve onboarding | A user can open the file locally and complete onboarding into the dashboard without extra instructions or services | ☐ |
| 60% | Portability flows | Export, import, alias-log claim, and recovery-session artifacts are implemented and repo-visible | ☐ |
| 70% | Template profile system | Template profiles, alias selection, and seeded schedules are implemented against documented schema rules | ☐ |
| 80% | Guided onboarding refinement | Adaptive onboarding, gated reveals, and robot-helper behaviors align with the active ADR set | ☐ |
| 90% | Consistency audit | Achievement triggers, progression locks, and session-mode protections are documented and internally consistent | ☐ |
| 100% | Production / Complete | Personal Profile delivers a stable, portable, consent-first progression surface with documented behavior across all supported session modes | ☐ |

## Dependencies

- `README.md` — canonical status, implemented feature list, and Phase 0 framing.
- `docs/personal-profile.html` — current implementation surface.
- `docs/AGENT_GUIDE.md` — binding bug history and constraints for storage, hashing, and stage flow.
- `docs/adr/ADR-001-progression-lock-pattern.md` — seal chain and progression stages.
- `docs/adr/ADR-003-alias-system.md` — alias identity and claim flow.
- `docs/adr/ADR-004-template-profile-specification.md` — template schema baseline.
- `docs/adr/ADR-005-cross-device-identity-and-session-modes.md` — four-mode identity architecture.
- `docs/adr/ADR-006-level-zero-adaptive-onboarding.md` — adaptive onboarding and robot-helper constraints.

## Current Blockers

- `docs/AGENT_GUIDE.md` still calls for a bug sweep, achievements audit, and template expansion.
- The broader identity system remains an ecosystem bottleneck for cross-surface integration.
- Some future-ledger and sync hooks remain explicitly TBD rather than implemented in this repo.