# User / Identity System — Progress Spec

**Current:** 1%
**Phase:** Architecture
**Status:** Primary bottleneck; consent-first, cross-device, and alias-mode behavior is defined but not fully generalized across the platform.

## What This Is

This spec tracks the shared identity, consent, and session-mode model that underpins FERROS. The current repo implements major parts of this model inside `docs/personal-profile.html` and the ADR set, but the system is still a bottleneck because broader platform features depend on the same consent-first, cross-device foundation.

## Milestone Gates

| % | Gate | Deliverables | Done? |
|---|------|-------------|-------|
| 10% | Identity model documented | Repo documents the local-first identity model, consent boundary, and supported session modes | ☐ |
| 20% | Session modes defined | Full profile, session, alias, and recovery modes are documented with explicit storage rules | ☐ |
| 30% | Consent boundary implemented | The current profile surface enforces a clear no-write path when consent is declined or recovery mode is active | ☐ |
| 40% | Alias and recovery artifacts | Portable `.ferros-log` and related claim/recovery artifacts are defined and repo-visible | ☐ |
| 50% | Local user test path | A user can locally exercise the existing identity flows without backend services or network assumptions | ☐ |
| 60% | Cross-surface contract | A repo-visible contract exists for how non-profile surfaces will consume identity and consent state | ☐ |
| 70% | Cross-device plan | Import, recovery, and attribution rules are documented beyond the single profile prototype | ☐ |
| 80% | Platform dependency unblock | Agent, dashboard, and other platform surfaces have a documented identity integration path | ☐ |
| 90% | Consistency audit | Storage rules, terminology, and accountability rules are consistent across README, ADRs, and active prototypes | ☐ |
| 100% | Production / Complete | FERROS has a stable, shared identity and consent system with platform-wide contracts for session handling, attribution, and persistence | ☐ |

## Dependencies

- `README.md` — canonical percentage and explicit bottleneck note.
- `docs/personal-profile.html` — current implementation surface for most identity behavior.
- `docs/AGENT_GUIDE.md` — storage, consent, and session-mode constraints.
- `docs/adr/ADR-003-alias-system.md` — alias identity and claim-flow behavior.
- `docs/adr/ADR-005-cross-device-identity-and-session-modes.md` — four session modes and portable log model.

## Current Blockers

- README identifies this as the primary bottleneck for the wider platform.
- The most complete implementation currently lives inside one prototype file rather than as a shared platform contract.
- Cross-surface identity consumption for dashboards, agents, and other products is still TBD.