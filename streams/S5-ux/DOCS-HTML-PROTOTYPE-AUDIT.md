# S5 UX — Docs HTML Prototype Audit

**Status:** Sprint 1 Lane C decision artifact  
**Date:** 2026-04-23  
**Scope:** Top-level `docs/*.html` prototypes only

---

## Why this exists

Phase A explicitly calls for cleaning up dead HTML prototypes in `docs/`, but that cleanup should not happen as broad churn. This note makes the keep/archive/remove decision explicit before any file moves happen.

The scope here is intentionally narrow:

- Includes the top-level HTML prototypes directly under `docs/`.
- Excludes `docs/assets/**/*.html`, `docs/surfaces/**`, `docs/legacy/**`, `site/`, and harnesses.
- Avoids destructive removal in Sprint 1 Lane C; S5 can recommend removals, but actual deletion should wait until inbound links and non-S5 owners are checked.

---

## Decision rules

- **Keep:** Still active S5 prior art for Phase A or Phase B work.
- **Archive:** Useful historical reference, but should not stay in the active top-level prototype set.
- **Remove:** Safe to delete after link and owner validation. No removals are proposed in this sprint.

---

## Decisions

| File | Title / role | Decision | Reason |
|------|--------------|----------|--------|
| `docs/agent-command-center.html` | Agent command center prototype | **Keep** | Directly aligned with Phase B subject matter: agent list, status, and command-center flows. Keeps agent-center vocabulary close to S5. |
| `docs/forge-workbench.html` | Surface-shell workbench prototype | **Keep** | Strongest prior art for the surface-first shell: fixed slots, collapsible corners, focus-area center, persistent shell chrome. |
| `docs/home-hud-dashboard.html` | Home HUD dashboard | **Archive** | Contains useful HUD ideas, but the shell grammar is superseded by `forge-workbench.html` and the domain is not Phase B. |
| `docs/ferros-showcase.html` | Docs-only showcase / marketing prototype | **Archive** | Public-facing site work should converge on `site/`, not a parallel docs prototype. Keep as history, not active shell guidance. |
| `docs/algo-trading-arena.html` | Trading arena concept | **Archive** | Unrelated to FERROS agent-center UX and not part of current stream scope. |
| `docs/architecture-design-lab-builder.html` | Architecture builder concept | **Archive** | Architecture planning prototype, not an active shell prior for S5 Phase B. |
| `docs/architecture-design-lab.html` | Architecture design lab | **Archive** | Same rationale as the builder variant; useful history, not active shell guidance. |
| `docs/deployment-roadmap.html` | Deployment roadmap visualization | **Archive** | Roadmap content is documentation, not a shell or site prototype S5 should keep active in docs root. |
| `docs/ferros-mind-map.html` | Mind map visualization | **Archive** | Conceptual map, not an implementation prior for site cleanup or Phase B shell composition. |
| `docs/ferros-project-map.html` | Project map visualization | **Archive** | Similar to the mind map: reference material, not active shell prior art. |
| `docs/personal-profile.html` | Personal progression profile | **Archive** | Closer to S2 profile ideas than S5 shell work; not part of current Phase A/B deliverables. |
| `docs/schedule-ledger.html` | Personal schedule ledger | **Archive** | Scheduling prototype outside the agent-center shell mission. |

---

## Summary

- **Keep in active top-level `docs/`:** `agent-command-center.html`, `forge-workbench.html`
- **Archive on Phase A cleanup pass:** the remaining ten top-level HTML prototypes
- **Remove in Sprint 1 Lane C:** none

---

## Recommended Phase A follow-through

1. Move the archive set into a dedicated `docs/legacy/` subfolder once G1 cleanup begins.
2. Check inbound links before moving anything outside the two keep files.
3. Treat `forge-workbench.html` as the shell-layout prior and `agent-command-center.html` as the subject-matter prior for Phase B.

---

## References

- [README.md](./README.md)
- [SURFACE-FIRST-SHELL.md](./SURFACE-FIRST-SHELL.md)
- [PHASE-B-SHELL-WIREFRAME.md](./PHASE-B-SHELL-WIREFRAME.md)