# Battle Arena — Progress Spec

**Current:** 1%
**Phase:** Prototype
**Status:** Game-specific prototype exists in `docs/algo-trading-arena.html`, but it still shares one overloaded surface with the reusable runtime concept.

## What This Is

This spec tracks the game-specific battle layer inside `docs/algo-trading-arena.html`. In the updated system model, Battle Arena is one product built on top of the reusable Arena Runtime rather than the runtime/container layer itself.

## Milestone Gates

| % | Gate | Deliverables | Done? |
|---|------|-------------|-------|
| 10% | Battle shell | Standalone battle shell loads locally with game-specific combat or competition surfaces visible | ☐ |
| 20% | Cards and Decks in battle context | Cards and Decks are displayed in a game context rather than only as a broad prototype showcase | ☐ |
| 30% | Arena Runtime dependency | Battle rendering or interaction is explicitly framed as a consumer of the Arena Runtime instead of a bespoke page-only system | ☐ |
| 40% | Game-state segmentation | Core battle state, rules, and loop boundaries are documented separately from runtime/container concerns | ☐ |
| 50% | Local playable prototype | A user can locally open the surface and exercise a coherent playable battle loop without backend services | ☐ |
| 60% | Crypto-first backend bridge | Optional backend, settlement, or verification hooks exist without replacing the local-first battle loop | ☐ |
| 70% | Identity/profile linkage | Battle state, rewards, or progress can consume FERROS profile and identity data through documented contracts | ☐ |
| 80% | Progression and rewards | Battle outputs connect to progression, rewards, or collectible state through a documented model | ☐ |
| 90% | Audited rules boundary | Rules, settlement boundaries, and contract assumptions are explicit and reviewable | ☐ |
| 100% | Production / Complete | Battle Arena is a production game built on the Arena Runtime with stable rules, identity linkage, and settlement boundaries | ☐ |

## Dependencies

- `README.md` — canonical percentage and current project naming.
- `docs/algo-trading-arena.html` — current prototype scope.
- `docs/progress/arena-runtime.md` — reusable runtime layer that should be separated from this game.
- `docs/adr/ADR-002-smart-contract-boundaries.md` — limits where contract-backed logic may exist.
- `docs/adr/ADR-010-cards-and-decks-nomenclature.md` — card/deck terminology baseline.
- `docs/architecture-overview.md` — local-first and standalone prototype constraints.

## Current Blockers

- The current prototype still mixes the reusable runtime layer with the specific battle game.
- Battle identity, persistence, and reward contracts are not yet documented as stable interfaces.
- Any real settlement or externally verified logic must stay inside the ADR-002 boundary, which is not yet implemented here.