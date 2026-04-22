# Arena Runtime — Progress Spec

**Current:** 1%
**Phase:** Prototype
**Status:** Reusable portal/runtime behavior is emerging inside `docs/algo-trading-arena.html`, but it is not separated into its own runtime layer yet.

## What This Is

This spec tracks the reusable Arena Runtime layer that should host Cards and Decks as rendered experiences. It is distinct from the Battle Arena game itself: the runtime is the container or portal that could host a loot box, logo, animation, reward reveal, or battle surface.

## Milestone Gates

| % | Gate | Deliverables | Done? |
|---|------|-------------|-------|
| 10% | Runtime shell | A local runtime shell exists as a distinct concept or surface for rendering FERROS content | ☐ |
| 20% | Single-asset portal | One Card or asset can render inside the runtime as a portal-like view | ☐ |
| 30% | Deck rendering | A composed Deck or assembly can render inside the same runtime contract | ☐ |
| 40% | Animation loop support | The runtime supports local animation or simulation loops without depending on backend services | ☐ |
| 50% | Local reusable host | One local runtime can host a logo, animation, loot box, or simple interactive surface usefully on its own | ☐ |
| 60% | External embedding | Other FERROS surfaces can call, embed, or hand off content to the runtime through a documented contract | ☐ |
| 70% | Forge export compatibility | Forge can export Cards or Decks that target the Arena Runtime cleanly | ✅ (spec decided in ADR-016; implementation follows) |
| 80% | Standardized runtime modes | Runtime mode flags such as animation, battle, viewer, or reward reveal are documented and reusable | ☐ |
| 90% | Stable embed contract | The runtime has a stable, reviewable embed contract across products | ☐ |
| 100% | Production / Complete | FERROS has a production reusable portal/runtime layer for rendering and experiencing Cards and Decks across surfaces | ☐ |

## Dependencies

- `README.md` — canonical percentage and current tracker naming for the runtime.
- `docs/algo-trading-arena.html` — current overloaded prototype where runtime and game ideas are still mixed together.
- `docs/forge-workbench.html` — authoring-side source of Cards and Decks that should eventually target this runtime.
- `docs/adr/ADR-008-modular-rendering-system.md` — shared rendering/runtime conventions relevant to reusable embedded surfaces.
- `docs/adr/ADR-010-cards-and-decks-nomenclature.md` — universal Card/Deck/Bag vocabulary.
- `docs/adr/ADR-016-arena-export-target.md` — **Arena export contract** (ADR-016). Defines the `arena-deck` envelope shape, card resolution rules, version negotiation, and C8 hand-off. This is the contract this runtime must consume at the 70% gate.

## Current Blockers

- The runtime concept is still embedded inside a game-specific prototype rather than documented as its own surface.
- The repo does not yet define a stable contract for embedding or calling the runtime from other pages.
- The `arena-deck` export envelope (ADR-016) is now specified — runtime implementation of consumption is the next gate.

## ADR-016 Cross-Reference

The Forge-to-Arena export contract is decided. See [`docs/adr/ADR-016-arena-export-target.md`](../adr/ADR-016-arena-export-target.md) for the full specification. The runtime must implement the consumption side (parse envelope → resolve cards → send `ferros:init` to each card iframe) to reach the 70% gate.