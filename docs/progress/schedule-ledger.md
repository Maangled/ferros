# Schedule Ledger — Progress Spec

**Current:** 1%
**Phase:** Prototype
**Status:** Prototype exists in `docs/schedule-ledger.html`.

## What This Is

This spec tracks the newspaper-themed scheduling and habit surface implemented in `docs/schedule-ledger.html`. The current prototype demonstrates a personal feed, a multi-step editor, map-style time views, and local interactions, but it is still a standalone planning surface rather than an integrated FERROS subsystem.

## Milestone Gates

| % | Gate | Deliverables | Done? |
|---|------|-------------|-------|
| 10% | Ledger shell | Standalone schedule ledger page loads locally with its newspaper-style shell intact | ☐ |
| 20% | Feed view | My Feed style schedule/task view exists with visible task cards and reading layout | ☐ |
| 30% | Editor workflow | Multi-step editor flow exists for creating and reviewing schedule entries | ☐ |
| 40% | Map views | Year, week, and day map views are available as repo-visible navigation states | ☐ |
| 50% | Local test flow | A user can open the file locally and test the core feed, editor, and view-switching behaviors without extra setup | ☐ |
| 60% | Persistence model | Repo-visible persistence behavior and stored schedule shape are documented instead of only implied by the prototype | ☐ |
| 70% | Profile linkage plan | A documented contract exists for connecting the ledger to identity, progression, or template data | ☐ |
| 80% | Shared system integration | Ledger conventions are aligned with FERROS profile, module, or dashboard surfaces through documented interfaces | ☐ |
| 90% | Cross-surface consistency | Terminology, schedule semantics, and state ownership are consistent with the broader FERROS docs | ☐ |
| 100% | Production / Complete | Schedule Ledger is a documented, testable, and integrated scheduling surface with stable data contracts and user-visible workflows | ☐ |

## Dependencies

- `README.md` — canonical percentage and status source.
- `docs/schedule-ledger.html` — current prototype surface and feature scope.
- `docs/architecture-overview.md` — current browser-phase context and local-first constraints.
- `docs/AGENT_GUIDE.md` — repo convention that active development is still centered elsewhere, which affects integration readiness.

## Current Blockers

- README only identifies this surface as an existing prototype and does not define a broader integration contract.
- Identity, profile, and agent-system dependencies for schedule syncing are still TBD.
- The ledger remains a standalone prototype rather than a shared subsystem.