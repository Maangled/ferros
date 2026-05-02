# ADR-009: Four-Corner Docking Layout System

**Status:** Accepted
**Date:** 2026-04-06
**Context:** FERROS UI layout — panel docking, ergonomic placement, cross-page consistency

---

## Context

The Forge workbench initially used a rigid three-column grid (sidebar, viewport, inspector).
This layout locked panels to full-height columns, prevented the viewer from filling the screen,
and left no room for new tools or overlays without adding more columns or tabs.

FERROS pages need a layout framework that:
- Gives each page up to **8 dock zones** (4 corners + 4 edges) without colliding
- Lets panels collapse to near-invisible corner anchors so the main content goes full-screen
- Supports overlay-style UIs (transparent floating panels) and solid-border UIs (game arenas
  with 3D button frames) from the same docking contract
- Accounts for left-hand / right-hand ergonomics on touch and VR surfaces

---

## Decision

**Adopt a four-corner, four-edge docking system as the standard layout primitive.**

### Dock Zones

A square viewport defines 8 zones:

```
  ┌──── TOP EDGE ────┐
  │ TL             TR │
  │                   │
LEFT              RIGHT
EDGE               EDGE
  │                   │
  │ BL             BR │
  └── BOTTOM EDGE ────┘
```

- **Corners (TL, TR, BL, BR):** Floating panels that collapse into their respective corner.
  Each corner can host one primary panel. Panels from the same corner never migrate to another
  corner — they always dock back to their origin.
- **Edges (Top, Right, Bottom, Left):** Reserved for persistent strips: navigation bars,
  status bars, hand/card trays, tool hotkey rails, viewport controls. Edges are page-specific
  and may be empty.

### Corner Semantics (Forge Default)

| Corner | Panel       | Rationale |
|--------|-------------|-----------|
| TL     | Bag         | Asset catalog — browse Cards and Decks |
| TR     | Inspector   | Selection context and animation presets |
| BL     | Tools       | Left-hand quick-access tool palette |
| BR     | Assistant   | AI chat, context-aware suggestions |

Other pages may assign different panels to the same corners. The Arena game layout, for
example, would use:
- **Bottom edge:** Player hand (card fan)
- **Right edge:** Viewport / camera controls
- **Left edge:** Tool or ability hotkeys
- **Top edge:** Information, navigation, score

### Collapse Behavior

- Panels collapse **both horizontally and vertically** into a compact corner widget showing
  only the panel title (emoji + label) and an expand toggle.
- **Collapsed opacity:** ~0.22 (nearly invisible), rising to ~0.5 on hover.
- **Open opacity:** ~0.88, rising to ~0.94 on hover.
- Transitions are CSS-driven (`opacity`, `width`, `height`) with ~0.25s ease timing.
- The title text remains visible in both states so users can always identify docked panels.

### Ergonomic Rationale

- **Left side = tool switching.** Most users are right-handed; the left hand (or left VR
  controller) handles coarse actions: swap tools, toggle abilities, select from hotkeys.
- **Right side = view control.** The right hand handles fine manipulation (mouse, stylus,
  right controller). The right edge exposes camera controls, projection toggles, and object
  rotation so the dominant hand stays on the viewport.
- **Bottom edge = hand / tray.** Card games place the player's hand along the bottom for
  natural thumb reach on mobile and a familiar table-game layout.
- **Top edge = info / nav.** Status, breadcrumbs, and global navigation live at the top where
  users expect persistent chrome.

### Overlay vs. Solid Modes

Each page chooses its visual mode:
- **Overlay mode** (Forge, workbenches): Translucent floating panels with backdrop blur and
  rounded corners. Content is visible behind panels.
- **Solid mode** (game arenas, battles): Opaque edge borders, potentially with 3D-styled
  button frames. The viewport is inset within the border frame.

Both modes use the same 8-zone model; only the visual treatment differs.

### Context Switching and Responsive Variants

Some FERROS pages need to switch logical contexts (room, floor, project, board state) while
preserving dock ownership and user muscle memory. The accepted pattern is to keep the
docking contract stable while swapping the viewport payload and any context-specific inspector
content.

- Use the **top edge** for persistent context chrome when needed: chips, tabs, breadcrumbs,
  or a selector that stays visible while the main surface changes.
- One edge rail may remain independently scrollable for dense controls or status while the
  viewport stays fixed.
- Context switches must not reassign panel home corners. Bag, Tools, Inspector, and Assistant
  surfaces keep their origin even when their content changes.
- On narrow touch surfaces, pages may compress top-edge chrome and stack edge content, but
  corner identities and collapse targets remain stable.
- Pages that introduce context switching should document what persists across a context change
  (selection, camera, draft state, filters) versus what intentionally resets.

---

## Consequences

- Every new FERROS page gets a layout contract (which zones it uses) defined in its manifest
  or documentation, preventing ad-hoc panel placement.
- Panels that outgrow their corner can stack vertically (two panels in the same corner open
  above/below each other) or switch to a tabbed mode, but they always return to their home
  corner when collapsed.
- The edge zones remain reserved for future implementation (hand tray, hotkey rail, etc.).
  Only corners are implemented in this iteration.
- Pages with many controls must distribute them across zones rather than nesting everything
  in a single sidebar, reducing buried-UI problems.
