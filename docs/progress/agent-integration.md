# Agent Integration — Progress Spec

**Current:** 1%
**Phase:** Blocked
**Status:** Agent-driven platform updates are blocked by the user and identity system.

## What This Is

This spec tracks FERROS’s intended agent-driven content and operations model. The repo already contains agent-facing UI ideas and architectural language, but the shared identity, consent, and permission model is not yet mature enough to support safe platform-wide agent updates.

## Milestone Gates

| % | Gate | Deliverables | Done? |
|---|------|-------------|-------|
| 10% | Agent role framing | Repo docs define the intended role of agents in FERROS architecture and workflows | ☐ |
| 20% | UI prototype surface | Repo contains visible agent-oriented surfaces such as the command center and agent activity panels | ☐ |
| 30% | Permission boundary | Identity, consent, and capability requirements for agent actions are documented | ☐ |
| 40% | Audit and accountability model | Repo documents how agent actions should be attributable, inspectable, and constrained | ☐ |
| 50% | Local demonstration path | A contributor can inspect the current agent-oriented surfaces locally and understand the intended workflow without services | ☐ |
| 60% | Shared action contract | Repo-visible docs define what an agent update, command, or content mutation should look like | ☐ |
| 70% | Identity dependency cleared | Shared identity and consent rules are documented enough for agent actions to rely on them safely | ☐ |
| 80% | Cross-surface bridge | Agent update flows are documented for profile, dashboard, and command-center surfaces | ☐ |
| 90% | Migration plan | The repo documents how current external agent hosting workflows transition into FERROS-native operation | ☐ |
| 100% | Production / Complete | FERROS has a stable, identity-aware, auditable agent-integration model with documented update contracts across the platform | ☐ |

## Dependencies

- `README.md` — canonical percentage and explicit blocker note.
- `docs/AGENT_GUIDE.md` — current agent roles, constraints, and repo conventions.
- `docs/agent-command-center.html` — current command-center prototype surface.
- `docs/home-hud-dashboard.html` — current dashboard-side agent activity surface.
- `docs/architecture-overview.md` — agent-hosting layer and capability model context.
- `docs/progress/user-identity-system.md` — dependency bottleneck for safe agent action.

## Current Blockers

- README explicitly states that agent integration is blocked by the user system.
- Permission, consent, and accountability rules are not yet generalized across the platform.
- Current agent surfaces are prototypes without stable data or action contracts.