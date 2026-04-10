# Assets, Cards & Decks — Progress Spec

**Current:** 1%
**Phase:** Planned
**Status:** Card systems are referenced across the repo, but the shared asset and deck model is barely started.

## What This Is

This spec tracks the shared asset, card, and deck model that shows up across Forge, Trading Arena, and newer ADRs. The repo contains naming decisions, rendering contracts, and some prototype surfaces, but not yet a stable, shared implementation contract spanning the ecosystem.

## Milestone Gates

| % | Gate | Deliverables | Done? |
|---|------|-------------|-------|
| 10% | Naming baseline | Repo contains explicit card and deck terminology rules for future shared use | ☐ |
| 20% | Prototype references | Multiple prototype surfaces visibly use cards, decks, or asset-like components | ☐ |
| 30% | Rendering contract | Shared runtime or rendering conventions for reusable assets are documented in ADRs or asset files | ☐ |
| 40% | Asset identity rules | Repo docs define how assets, cards, or decks are identified, versioned, or composed | ☐ |
| 50% | Local exploration path | A contributor can inspect current asset and card surfaces locally across the relevant prototypes without extra tooling | ☐ |
| 60% | Shared data contract | Repo-visible docs define minimum schema expectations for cards, decks, or reusable assets | ☐ |
| 70% | Cross-surface reuse | At least two surfaces share the same documented card or asset conventions rather than independent prototype-only language | ☐ |
| 80% | Composition workflow | Forge, arena, or profile/module flows document how assets or decks are assembled and exchanged | ☐ |
| 90% | Stable ecosystem role | The shared asset/card/deck layer has documented responsibilities across product surfaces | ☐ |
| 100% | Production / Complete | FERROS has a stable, shared asset/card/deck model with documented identity, rendering, and composition contracts | ☐ |

## Dependencies

- `README.md` — canonical percentage and status note.
- `docs/algo-trading-arena.html` — visible card and deck-driven prototype surface.
- `docs/forge-workbench.html` — asset-oriented workbench surface.
- `docs/adr/ADR-008-modular-rendering-system.md` — shared asset runtime and rendering contract.
- `docs/adr/ADR-010-cards-and-decks-nomenclature.md` — canonical vocabulary for cards and decks.
- `docs/adr/ADR-011-routine-module-system.md` — newer module and deck composition direction.

## Current Blockers

- README states that the card system is only barely started.
- Current implementations are spread across prototypes and ADRs rather than stabilized shared code or docs.
- Asset identity, persistence, and exchange rules remain only partially specified.