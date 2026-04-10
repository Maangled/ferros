# The Forge — Progress Spec

**Current:** 2%
**Phase:** Prototype
**Status:** Early architecture is visible in the workbench, but no general creation tools exist yet.

## What This Is

This spec tracks The Forge as the asset and template creation surface represented today by `docs/forge-workbench.html` and the loot-box asset manifests under `docs/assets/loot/`. The current repo shows a workbench, part indexing, project loading, and viewer state, but not a mature creation pipeline.

## Milestone Gates

| % | Gate | Deliverables | Done? |
|---|------|-------------|-------|
| 10% | Workbench shell | Docked workbench UI loads locally and exposes catalog, inspector, tools, and assistant regions | ☐ |
| 20% | Asset indexing | Parts and project manifests load from repo JSON sources with verifiable selection state | ☐ |
| 30% | Viewer contracts | Arena and project viewers accept selection changes and basic pose/state updates | ☐ |
| 40% | Project composition model | Part relationships, saved project manifests, and assembly rules are documented and repo-backed | ☐ |
| 50% | Local testable workflow | A contributor can open the workbench and inspect or switch between available parts and project assets without additional setup | ☐ |
| 60% | Creation workflow draft | Repo contains a documented authoring flow for creating or editing a project manifest instead of only browsing one | ☐ |
| 70% | Asset reuse model | Shared rendering/runtime contracts used by Forge assets are documented against the modular rendering system | ☐ |
| 80% | Template export path | Forge outputs or manifest conventions are defined well enough for other FERROS surfaces to consume them | ☐ |
| 90% | Multi-project coverage | More than the current narrow loot-box project set exists with the same manifest conventions | ☐ |
| 100% | Production / Complete | The Forge supports repo-verifiable asset creation, inspection, saved project composition, and reusable output contracts | ☐ |

## Dependencies

- `README.md` — defines The Forge at 2% and states that no creation tools exist yet.
- `docs/forge-workbench.html` — current workbench prototype and visible interaction surface.
- `docs/assets/loot/loot-box-3d.json` — canonical loot-box project manifest in current repo memory.
- `docs/assets/loot/projects/index.json` — current project index, intentionally narrow in scope.
- `docs/adr/ADR-008-modular-rendering-system.md` — rendering and runtime composition contract for shared assets.
- `docs/adr/ADR-009-four-corner-docking-layout.md` — layout semantics used by the workbench.

## Current Blockers

- README explicitly notes that creation tools do not exist yet.
- Current repo coverage is centered on the loot-box project and a single workbench flow.
- Forge output contracts for broader templates or reusable assets are still TBD.