# Schedule Ledger — Progress Spec

**Current:** 1%
**Phase:** Prototype
**Status:** Prototype exists in `docs/schedule-ledger.html`.

## What This Is

This spec tracks the scheduling pipeline currently represented by `docs/schedule-ledger.html`. In the updated planning model, calendar structure comes first, and ledger or feed-style presentation is the readable delivery layer built on top of that calendar data.

## Milestone Gates

| % | Gate | Deliverables | Done? |
|---|------|-------------|-------|
| 10% | Calendar shell | Standalone schedule surface loads locally with a clear calendar and planning shell | ☐ |
| 20% | Event and task structure | The prototype exposes usable event, task, or block structure instead of only presentation chrome | ☐ |
| 30% | Profile-to-calendar mapping | Profile defaults, templates, or routines can map into calendar structure through a documented model | ☐ |
| 40% | Module and Deck scheduling rules | Routine modules or Deck composition rules can be expressed as calendar structure | ☐ |
| 50% | Useful local calendar | The calendar workflow is locally useful enough to stand on its own without backend services | ☐ |
| 60% | Ledger and feed translation | Calendar data can be translated into a readable ledger or feed-style presentation layer | ☐ |
| 70% | Delivery prioritization | A prioritization or delivery model exists for what the user sees first in the ledger/feed layer | ☐ |
| 80% | External sharing | Schedule or ledger outputs can be shared with external surfaces or apps through documented contracts | ☐ |
| 90% | Cross-surface consistency | Calendar, ledger, and profile semantics are consistent across the broader FERROS docs | ☐ |
| 100% | Production / Complete | FERROS has a production scheduling system where calendar structure and ledger delivery work together as one local-first pipeline | ☐ |

## Dependencies

- `README.md` — canonical percentage and status source.
- `docs/schedule-ledger.html` — current prototype surface and feature scope.
- `docs/architecture-overview.md` — current browser-phase context and local-first constraints.
- `docs/AGENT_GUIDE.md` — repo convention that active development is still centered elsewhere, which affects integration readiness.
- `docs/adr/ADR-011-routine-module-system.md` — module and starter-deck composition direction for scheduling.

## Current Blockers

- README only identifies this surface as an existing prototype and does not define a broader calendar contract.
- Profile-to-calendar and calendar-to-ledger contracts are still emerging rather than stabilized.
- The current prototype still overemphasizes feed-style presentation relative to the calendar engine underneath it.