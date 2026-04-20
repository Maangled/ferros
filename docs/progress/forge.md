# The Forge — Progress Spec

**Current:** 40%
**Phase:** Prototype
**Status:** The workbench now loads a real card fixture into the Bag, exposes schema-aligned card fields in the inspector, and bridges that card into the runtime preview without widening shared contracts.

## What This Is

This spec tracks The Forge as the FERROS authoring system represented today by `docs/forge-workbench.html` and the loot-box asset manifests under `docs/assets/loot/`. In the platform model, Cards are atomic parametric assets, Decks are assembled compositions, and the Bag is the local catalog used to browse and select them.

## Milestone Gates

| % | Gate | Deliverables | Done? |
|---|------|-------------|-------|
| 10% | Workbench shell | Docked workbench UI loads locally and exposes Bag, inspector, tools, and assistant regions | ✅ |
| 20% | Asset catalog indexing | Cards and Deck manifests load from repo JSON sources with verifiable selection state | ✅ |
| 30% | Card visibility and selection | Atomic assets are visible, selectable, and clearly represented as Cards in the local workbench flow | ✅ |
| 40% | Parametric cards | Card-level parameters, state presets, or assembly options are documented and locally manipulable | ✅ |
| 50% | Local assembly workflow | A user can assemble Decks locally with drag, move, highlight, and basic preview behavior without backend services | ☐ |
| 60% | Assistant edit bridge | An assistant or local API bridge can create or edit Forge assets without replacing the standalone workflow | ☐ |
| 70% | Command-center dispatch | `docs/agent-command-center.html` or its successor can dispatch generation or edit jobs into Forge workflows | ☐ |
| 80% | Arena export target | Forge exports reusable Deck manifests or payloads that target the Arena Runtime cleanly | ☐ |
| 90% | Shared authoring contract | Authoring conventions are stable across products that consume Cards, Decks, and Bag browsing | ☐ |
| 100% | Production / Complete | Forge is a production authoring pipeline for parametric Cards and reusable Deck compositions | ☐ |

## Dependencies

- `README.md` — defines The Forge at 2% and states that no creation tools exist yet.
- `docs/forge-workbench.html` — current workbench prototype and visible interaction surface.
- `schemas/fixtures/deck-card-assembly-seam.json` — fixture-backed V5 card source used to prove the card→deck seam inside Forge.
- `docs/assets/cards/trading-card.html` — current runtime preview target for the V5 host-message bridge.
- `docs/assets/loot/loot-box-3d.json` — canonical loot-box project manifest in current repo memory.
- `docs/assets/loot/projects/index.json` — current project index, intentionally narrow in scope.
- `docs/adr/ADR-008-modular-rendering-system.md` — rendering and runtime composition contract for shared assets.
- `docs/adr/ADR-009-four-corner-docking-layout.md` — layout semantics used by the workbench.
- `docs/adr/ADR-010-cards-and-decks-nomenclature.md` — universal Card/Deck/Bag vocabulary.
- `docs/contracts/runtime-host-v1.md` — host-message seam the workbench uses for card preview sync.
- `docs/progress/arena-runtime.md` — target runtime surface that Forge should eventually export into.

## Current Blockers

- README explicitly notes that creation tools do not exist yet.
- Current repo coverage is centered on the loot-box project and a single workbench flow.
- V5 now covers one real card fixture, but multi-card local deck assembly remains a later step.
- V7 card export/import portability is still open; this pass intentionally stopped at editable local preview.
- Forge output contracts for broader templates or reusable runtime targets are still TBD.