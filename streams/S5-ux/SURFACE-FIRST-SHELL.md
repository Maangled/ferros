# S5 UX — Surface-First Shell Note

**Status:** Active design note  
**Date:** 2026-04-23  
**Applies to:** Phase B local web shell

---

## Why this exists

S5 already points at Forge and ADR-019 as shell prior art, but it does not yet turn that prior art into a concrete rule for the local agent-center UI. This note captures the current direction before Phase B wireframes or HTML work widen the shell shape.

---

## Inputs

- The Forge workbench already behaves like a surface shell rather than a window manager: fixed home zones, collapse-to-corner behavior, a central work area, and persistent top/bottom chrome.
- ADR-019 treats named-slot shell composition, focus-mode HUD transitions, and capability-scoped UI messaging as valid S5 prior art.

---

## Rule 1 — Surfaces, not windows

**FERROS Phase B composes the shell from named surfaces with fixed home slots. It does not use draggable desktop-window behavior as the primary interaction model.**

A surface may:
- live in a home slot,
- collapse to a visible anchor,
- expand into focus mode,
- swap content inside its assigned slot.

A surface may not:
- spawn as a free-floating draggable window,
- rely on overlapping z-stack juggling as the normal way to work,
- bury primary actions behind chains of unnamed popups or transient modal layers.

The shell owns layout, focus, and route changes. Child surfaces request those changes through typed shell intents rather than repositioning themselves.

---

## Rule 2 — Six-degree reach

**Nothing in the Phase B shell should be more than six interaction degrees from the default shell state.**

For this note, one interaction degree is one intentional shell transition:

1. Reveal or expand a collapsed surface.
2. Switch the active surface within a slot.
3. Select a top-level object such as an agent, grant, or deny event.
4. Open that object's detail or focus state.
5. Arm a privileged action.
6. Confirm or dismiss inside the consent or audit surface.

Scrolling, reading, hover states, and typing inside an already open surface do not count as additional degrees.

If a primary workflow needs more than six degrees, the shell topology is wrong. S5 should promote the action, preserve more context in place, or move the workflow into a better home slot.

---

## Initial Phase B topology

This keeps the Forge shell grammar but swaps in agent-center content.

| Shell zone | Forge prior art | Phase B role |
|-----------|-----------------|--------------|
| Top edge | Nav / identity bar | Shell identity, connection health, consent state |
| Center | Viewport | Primary surface focus: selected agent, grant review, or deny-log deep view |
| Top-left | Bag | Agent registry or route list |
| Top-right | Inspector | Selected agent details, capability state, object metadata |
| Bottom-left | Tools | Quick actions, filters, dispatch controls |
| Bottom-right | Assistant | Consent, audit handoff, or assistant guidance |
| Bottom edge | Status bar | Active route, backend state, recent errors, deny counters |

Focus mode should enlarge the current primary surface while collapsed anchors remain visible in their home slots.

---

## Example task budgets

These are the task budgets the next wireframe should satisfy.

| Workflow | Degrees |
|----------|---------|
| Inspect agent status | 1. Expand registry if needed. 2. Select agent. 3. Focus details. |
| Grant capability | 1. Select agent. 2. Open capability detail. 3. Choose grant. 4. Arm grant. 5. Open consent or audit surface. 6. Confirm. |
| Review deny log | 1. Switch a slot to deny-log view. 2. Filter or select an entry. |

Any Phase B mockup or HTML shell that breaks these budgets needs an explicit justification.

---

## Build implication

- The slot-based follow-on now lives in `PHASE-B-SHELL-WIREFRAME.md`.
- The first HTML shell should preserve fixed home surfaces and focus-mode expansion rather than introduce movable windows.
- Typed shell intents should at minimum cover focus, collapse, route selection, consent, and audit handoff.

---

## References

- [README.md](./README.md)
- [BACKLOG.md](./BACKLOG.md)
- [DOCS-HTML-PROTOTYPE-AUDIT.md](./DOCS-HTML-PROTOTYPE-AUDIT.md)
- [PHASE-B-SHELL-WIREFRAME.md](./PHASE-B-SHELL-WIREFRAME.md)
- [docs/adr/ADR-019-harvest-workpace.md](../../docs/adr/ADR-019-harvest-workpace.md)
- [docs/progress/forge.md](../../docs/progress/forge.md)