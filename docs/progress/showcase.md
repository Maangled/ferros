# Showcase / Landing Page — Progress Spec

**Current:** 10%
**Phase:** Prototype
**Status:** Prototype exists in `docs/ferros-showcase.html`, and the contracts section now reads `docs/contracts/manifest.json` locally to show live status and evidence links with a visible failure state.

## What This Is

This spec tracks the public-facing FERROS landing page implemented in `docs/ferros-showcase.html`. It is a marketing and documentation surface for explaining FERROS, its architecture, and the roadmap, not an implementation surface for the platform itself.

## Milestone Gates

| % | Gate | Deliverables | Done? |
|---|------|-------------|-------|
| 10% | Landing shell | A standalone public-facing page exists and loads locally without dependencies | ✅ |
| 20% | Architecture narrative | The page explains FERROS architecture and philosophy using repo-consistent language | ☐ |
| 30% | Roadmap narrative | Roadmap sections align with the canonical README phases without becoming the source of truth for percentages | ☐ |
| 40% | Contributor guidance | The page contains contributor-facing entry points and references to repo docs | ☐ |
| 50% | Public-readiness baseline | A new viewer can understand what FERROS is, what Phase 0 means, and where to start by opening the page locally | ☐ |
| 60% | Content accuracy pass | Claims on the page are reconciled against README, architecture docs, and ADR constraints | ☐ |
| 70% | Responsive completeness | Layout, narrative sections, and key callouts are documented as usable on desktop and mobile viewports | ☐ |
| 80% | Documentation handoff | The page clearly routes technical readers to the blueprint, architecture overview, AGENT guide, and ADR set | ☐ |
| 90% | Public consistency audit | Marketing copy, architecture claims, and visual roadmap cues are consistent with current repo state | ☐ |
| 100% | Production / Complete | Showcase is an accurate, public-ready explanation surface for FERROS with verified content and stable entry points | ☐ |

## Dependencies

- `README.md` — canonical roadmap, philosophy, and current-state source.
- `docs/ferros-showcase.html` — current landing page prototype.
- `docs/contracts/manifest.json` — live capability manifest consumed by the contracts section under the local file-origin workflow.
- `docs/architecture-overview.md` — architecture language baseline.
- `ferros-blueprint.html` — core Phase 0 reference artifact the showcase points toward.

## Current Blockers

- The page contains illustrative roadmap progress bars that are not authoritative for actual project percentages.
- The manifest-backed contracts panel is live, but it currently exposes only the existing manifest surface: contract identity, status, and evidence links.
- Public-facing claims still depend on a rapidly evolving prototype repo surface.
- Final public-readiness criteria are not yet documented outside the prototype itself.