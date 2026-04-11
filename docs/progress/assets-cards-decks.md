# Assets, Cards & Decks — Progress Spec

**Current:** 1%
**Phase:** Planned
**Status:** Card systems are referenced across the repo, but the shared asset and deck model is barely started.

## What This Is

This spec tracks the shared object model that shows up across Forge, Arena surfaces, profile modules, and newer ADRs. In the FERROS platform model, everything user-facing should be representable as a Card when atomic, a Deck when composed, a Bag entry when cataloged locally, a runtime instance when rendered, and a profile-linked object when attributed or permissioned.

## Milestone Gates

| % | Gate | Deliverables | Done? |
|---|------|-------------|-------|
| 10% | Taxonomy baseline | Card, Deck, and Bag terminology is explicitly locked in repo ADRs | ☐ |
| 20% | Profile adoption | Profile modules and starter Decks consume the Card/Deck model in a documented way | ☐ |
| 30% | Forge and arena adoption | Forge and arena-facing surfaces visibly use the same object vocabulary | ☐ |
| 40% | Runtime instance model | Repo docs define how a rendered Card or Deck appears as a runtime or portal instance | ☐ |
| 50% | Local-first object model | Cards, Decks, Bag entries, and runtime instances are locally understandable and useful without services | ☐ |
| 60% | Shared authoring and consumption contract | Repo-visible docs define how products author, consume, and exchange Cards and Decks | ☐ |
| 70% | Profile-linked attribution | Objects can be associated with identity, permissions, rewards, or progression through documented contracts | ☐ |
| 80% | Verification hooks | Verification state or interoperability hooks exist for profile-linked objects without breaking local-first use | ☐ |
| 90% | Stable cross-surface model | The same object model is reused consistently across Forge, Arena, Profile, and schedule surfaces | ☐ |
| 100% | Production / Complete | FERROS has a production universal object model for Cards, Decks, Bags, runtime instances, and profile-linked objects | ☐ |

## Dependencies

- `README.md` — canonical percentage and status note.
- `docs/algo-trading-arena.html` — visible card and deck-driven prototype surface.
- `docs/forge-workbench.html` — asset-oriented workbench surface.
- `docs/adr/ADR-008-modular-rendering-system.md` — shared asset runtime and rendering contract.
- `docs/adr/ADR-010-cards-and-decks-nomenclature.md` — canonical vocabulary for cards and decks.
- `docs/adr/ADR-011-routine-module-system.md` — newer module and deck composition direction.
- `docs/progress/arena-runtime.md` — reusable runtime layer for rendered Cards and Decks.

## Current Blockers

- README states that the card system is only barely started.
- Current implementations are spread across prototypes and ADRs rather than stabilized shared contracts.
- Runtime-instance and profile-linked object semantics are still only partially specified.