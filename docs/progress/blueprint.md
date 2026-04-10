# Founding Blueprint — Progress Spec

**Current:** 1%
**Phase:** Active
**Status:** The founding blueprint already exists and serves as the read-only Phase 0 conformance target.

## What This Is

This spec tracks `ferros-blueprint.html` as the rendering target for Phase 0. It is not a product feature or an implementation surface; it is the visual and structural specification the future FERROS renderer must match.

## Milestone Gates

| % | Gate | Deliverables | Done? |
|---|------|-------------|-------|
| 10% | Text fidelity target | All blueprint text blocks, headings, and labels are identified as conformance checkpoints | ☐ |
| 20% | Box model target | Panels, cards, spacing, borders, and radii are captured as fidelity requirements | ☐ |
| 30% | Color target | Background colors, accent colors, and text contrast are enumerated as visual checks | ☐ |
| 40% | Gradient target | Header and card gradients are captured as explicit render requirements | ☐ |
| 50% | Layout target | Flex, grid, cards, tables, and sticky nav layout requirements are documented against the file structure | ☐ |
| 60% | Typography target | Embedded font, readable size, line height, and code styling expectations are documented | ☐ |
| 70% | Table fidelity target | Data tables, labels, and section chrome are renderable without browser-specific shortcuts | ☐ |
| 80% | Full-page pass | A full-page visual checklist exists for comparing native FERROS output to the blueprint source | ☐ |
| 90% | Conformance harness | A repeatable test method exists for checking native render output against the blueprint artifact | ☐ |
| 100% | Production / Complete | FERROS renders `ferros-blueprint.html` natively with a visually acceptable full-page match and no network dependency | ☐ |

## Dependencies

- `ferros-blueprint.html` — the authoritative Phase 0 artifact and rendering target.
- `README.md` — states that the blueprint is both the specification and the Phase 0 conformance test.
- `docs/architecture-overview.md` — defines the HTML and CSS feature support needed to render the blueprint natively.

## Current Blockers

- The native boot, parse, layout, and render pipeline does not exist yet in this repo.
- No repo-visible conformance harness currently compares native render output to the blueprint artifact.
- The blueprint is read-only, so fidelity improvements must happen in the future renderer rather than in the source document.