# Home HUD Dashboard — Progress Spec

**Current:** 1%
**Phase:** Prototype
**Status:** Prototype exists in `docs/home-hud-dashboard.html`.

## What This Is

This spec tracks the FERROS home dashboard prototype in `docs/home-hud-dashboard.html`. The current page shows a kiosk-style home interface with schedule, device controls, cameras, project cards, and agent activity, but it remains a standalone visual and interaction prototype.

## Milestone Gates

| % | Gate | Deliverables | Done? |
|---|------|-------------|-------|
| 10% | HUD shell | Standalone dashboard page loads locally with the full kiosk layout intact | ☐ |
| 20% | Status and schedule surfaces | Status bar, clock, schedule areas, and checklist flows are visible and navigable | ☐ |
| 30% | Device-control surface | Camera, climate, lock, lighting, and device-control panels exist in the prototype | ☐ |
| 40% | Agent activity surface | HUD shows an agent-activity or coordination area that matches the broader FERROS automation vision | ☐ |
| 50% | Local usability pass | A user can open the page locally and understand the dashboard model without any backend service | ☐ |
| 60% | Device contract draft | Repo-visible docs define how cameras, controls, and consent-sensitive actions would be represented | ☐ |
| 70% | Identity and consent plan | A documented dependency path exists for binding dashboard actions to FERROS identity and consent rules | ☐ |
| 80% | Deployment readiness plan | Repo docs define how the HUD fits Phase 3 and practical kiosk deployment targets | ☐ |
| 90% | Data ownership audit | Dashboard state, agent actions, and home telemetry ownership are documented as local-first | ☐ |
| 100% | Production / Complete | Home HUD is a documented, identity-aware, deployable kiosk surface with stable contracts for device and agent interactions | ☐ |

## Dependencies

- `README.md` — canonical percentage and platform positioning.
- `docs/home-hud-dashboard.html` — current prototype surface.
- `docs/deployment-roadmap.html` — practical hardware and kiosk deployment context.
- `docs/architecture-overview.md` — Phase 3 device-control direction and local-first architecture.
- `docs/agent-command-center.html` — adjacent coordination surface for future agent actions.

## Current Blockers

- The current page contains illustrative project-card progress values that are not authoritative project status.
- Device, camera, and telemetry integrations are not connected to real services in this repo.
- Identity, consent, and agent-access boundaries for home actions are still dependent on broader shared infrastructure work.