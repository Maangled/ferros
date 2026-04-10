# Agent Command Center — Progress Spec

**Current:** 1%
**Phase:** Prototype
**Status:** Prototype exists in `docs/agent-command-center.html`.

## What This Is

This spec tracks the web-based coordination surface in `docs/agent-command-center.html` that is intended to replace the current Discord-centric workflow around `botgen-rust`. The repo currently shows a mock operations console with agent roster, chat, project board, deployment, and home-integration panels, but not a connected backend.

## Milestone Gates

| % | Gate | Deliverables | Done? |
|---|------|-------------|-------|
| 10% | Console shell | Standalone command center page loads locally with multi-pane layout intact | ☐ |
| 20% | Agent roster prototype | Visible agent cards, status badges, and role surfaces are implemented in the prototype | ☐ |
| 30% | Operations views | Chat, Kanban/project board, deployment history, and device panels are all present as navigable prototype surfaces | ☐ |
| 40% | Migration framing | Repo docs clearly state that this surface is the intended replacement path for Discord or botgen-rust coordination workflows | ☐ |
| 50% | Local test pass | A contributor can open the file and understand the major coordination surfaces without backend services | ☐ |
| 60% | Data contract draft | Repo-visible contracts exist for agent state, messages, task state, and deployment records | ☐ |
| 70% | Identity integration plan | Agent actions, governance, and device access are tied to a documented identity and consent model | ☐ |
| 80% | Backend bridge plan | A documented path exists for connecting this UI to real agent infrastructure and live operations feeds | ☐ |
| 90% | Auditability model | FERROS-specific expectations for logging, accountability, and permission scope are documented for command-center operations | ☐ |
| 100% | Production / Complete | Agent Command Center has stable data contracts, identity-aware controls, and a documented bridge to real agent infrastructure | ☐ |

## Dependencies

- `README.md` — canonical percentage and note that this surface replaces Discord integration from botgen-rust.
- `docs/agent-command-center.html` — current prototype surface.
- `docs/AGENT_GUIDE.md` — active repo constraints and agent-oriented workflow context.
- `docs/architecture-overview.md` — agent hosting and permission-layer context.
- `docs/adr/ADR-005-cross-device-identity-and-session-modes.md` — identity-mode constraints relevant to human and agent actions.

## Current Blockers

- The current prototype is not wired to a real backend, event stream, or governance model.
- The user and identity system is still the broader platform bottleneck for safe agent actions.
- Permission and audit contracts for live operations are not yet documented as stable interfaces.