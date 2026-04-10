# Trading Arena — Progress Spec

**Current:** 1%
**Phase:** Prototype
**Status:** Prototype exists in `docs/algo-trading-arena.html`.

## What This Is

This spec tracks the card-based, gamified algorithmic trading prototype in `docs/algo-trading-arena.html`. The current repo shows a broad experiential surface that includes battle-lane trading, deck ideas, idle systems, creatures, loot, and profile affordances, but it remains a prototype rather than a validated trading platform.

## Milestone Gates

| % | Gate | Deliverables | Done? |
|---|------|-------------|-------|
| 10% | Arena shell | Standalone arena page loads locally with battle arena, deck, and profile sections visible | ☐ |
| 20% | Card interaction surface | Card display, lane layout, and visible interaction affordances are repo-verifiable in the prototype | ☐ |
| 30% | Mode segmentation | The prototype clearly separates trading, deck-building, idle, creature, and reward surfaces | ☐ |
| 40% | Card/deck vocabulary | Repo docs align arena terminology with FERROS card and deck naming conventions | ☐ |
| 50% | Local test pass | A user can open the file and meaningfully explore the major gameplay surfaces without any service dependency | ☐ |
| 60% | Data contract draft | Repo-visible documentation defines what trading state, deck state, and reward state would need to persist | ☐ |
| 70% | Contract boundary alignment | Any on-chain or bounty concepts are explicitly constrained to the ADR-approved boundary model | ☐ |
| 80% | Shared identity linkage | The arena has a documented dependency path to FERROS identity, assets, and profile systems | ☐ |
| 90% | Scope reduction or validation | The repo documents which current prototype surfaces are in-scope for a real first release versus illustrative extras | ☐ |
| 100% | Production / Complete | Trading Arena has a documented, testable core loop, stable card/deck contracts, and explicit identity and settlement boundaries | ☐ |

## Dependencies

- `README.md` — canonical percentage and project name.
- `docs/algo-trading-arena.html` — current prototype scope.
- `docs/adr/ADR-002-smart-contract-boundaries.md` — limits where contract-backed logic may exist.
- `docs/adr/ADR-010-cards-and-decks-nomenclature.md` — card/deck terminology baseline.
- `docs/architecture-overview.md` — local-first and standalone prototype constraints.

## Current Blockers

- The prototype scope is much broader than the repo’s documented production boundary.
- Arena identity, persistence, and asset ownership contracts are not yet documented as stable interfaces.
- Any real settlement or externally verified logic must stay inside the ADR-002 boundary, which is not yet implemented here.