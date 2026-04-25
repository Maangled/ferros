# S5 UX — Docs HTML Prototype Audit

**Status:** Sprint 1 Lane C decision artifact  
**Date:** 2026-04-24  
**Scope:** Top-level `docs/*.html` prototypes only

---

## Why this exists

Phase A explicitly calls for cleaning up dead HTML prototypes in `docs/`, but that cleanup should not happen as broad churn. This note records the inbound-link-verified split after the first archive move.

The scope here is intentionally narrow:

- Includes the top-level HTML prototypes directly under `docs/`.
- Excludes `docs/assets/**/*.html`, `docs/surfaces/**`, `docs/legacy/**`, `site/`, and harnesses.
- Avoids destructive removal in Sprint 1 Lane C; S5 can recommend removals, but actual deletion should wait until inbound links and non-S5 owners are checked.

---

## Decision rules

- **Keep in docs root:** Still active S5 prior art, or historical material that current repo links still target.
- **Archived:** Historical reference moved into `docs/legacy/`.
- **Remove:** Safe to delete after link and owner validation. No removals are proposed in this sprint.

---

## Decisions

| File | Title / role | Decision | Reason |
|------|--------------|----------|--------|
| `docs/agent-command-center.html` | Agent command center prototype | **Keep in docs root** | Directly aligned with Phase B subject matter: agent list, status, and command-center flows. Keeps agent-center vocabulary close to S5. |
| `docs/forge-workbench.html` | Surface-shell workbench prototype | **Keep in docs root** | Strongest prior art for the surface-first shell: fixed slots, collapsible corners, focus-area center, persistent shell chrome. |
| `docs/home-hud-dashboard.html` | Home HUD dashboard | **Archived** | Contains useful HUD ideas, but the shell grammar is superseded by `forge-workbench.html` and the domain is not Phase B. Moved to `docs/legacy/`. |
| `docs/ferros-showcase.html` | Docs-only showcase / marketing prototype | **Keep in docs root** | Historical reference only, but current repo links still target it so it stays in `docs/` for now. |
| `docs/algo-trading-arena.html` | Trading arena concept | **Keep in docs root** | Historical reference only, but current repo links still target it so it stays in `docs/` for now. |
| `docs/architecture-design-lab-builder.html` | Architecture builder concept | **Archived** | Architecture planning prototype, not an active shell prior for S5 Phase B. Moved to `docs/legacy/`. |
| `docs/architecture-design-lab.html` | Architecture design lab | **Archived** | Same rationale as the builder variant; useful history, not active shell guidance. Moved to `docs/legacy/`. |
| `docs/deployment-roadmap.html` | Deployment roadmap visualization | **Keep in docs root** | Historical reference only, but current repo links still target it so it stays in `docs/` for now. |
| `docs/ferros-mind-map.html` | Mind map visualization | **Archived** | Conceptual map, not an implementation prior for site cleanup or Phase B shell composition. Moved to `docs/legacy/`. |
| `docs/ferros-project-map.html` | Project map visualization | **Archived** | Similar to the mind map: reference material, not active shell prior art. Moved to `docs/legacy/`. |
| `docs/personal-profile.html` | Personal progression profile | **Keep in docs root** | Historical reference only, but current repo links still target it so it stays in `docs/` for now. |
| `docs/schedule-ledger.html` | Personal schedule ledger | **Keep in docs root** | Historical reference only, but current repo links still target it so it stays in `docs/` for now. |

---

## Summary

- **Keep as active top-level `docs/` priors:** `agent-command-center.html`, `forge-workbench.html`
- **Keep in `docs/` for current inbound links:** `personal-profile.html`, `schedule-ledger.html`, `deployment-roadmap.html`, `algo-trading-arena.html`, `ferros-showcase.html`
- **Archived now in `docs/legacy/`:** `home-hud-dashboard.html`, `architecture-design-lab.html`, `architecture-design-lab-builder.html`, `ferros-mind-map.html`, `ferros-project-map.html`
- **Remove in Sprint 1 Lane C:** none

---

## Recommended Phase A follow-through

1. Repoint current repo links before moving the remaining historical docs-root HTML files.
2. Treat `forge-workbench.html` as the shell-layout prior and `agent-command-center.html` as the subject-matter prior for Phase B.

---

## References

- [README.md](./README.md)
- [SURFACE-FIRST-SHELL.md](./SURFACE-FIRST-SHELL.md)
- [PHASE-B-SHELL-WIREFRAME.md](./PHASE-B-SHELL-WIREFRAME.md)