# ADR-008: Modular Rendering System

**Status:** Accepted
**Date:** 2026-04-02
**Context:** FERROS visual asset system — module boundaries, shared rendering primitives, build pipeline

---

## Context

The FERROS asset ecosystem has grown to 16 standalone HTML files totaling ~13,600 lines.
Cross-asset auditing reveals ~900 lines of byte-identical duplication: design tokens copied 16
times, messaging protocol scaffolding copied 16 times, accessibility primitives scattered
inconsistently, and animation keyframes defined identically in up to 7 files.

The shared infrastructure layer (`_tokens.css`, `_embed.js`, `_ledger.html`) already exists as
canonical sources but remains disconnected from all 16 assets — every asset copy-pastes
definitions internally. The `_embed.js` router wraps routed payloads inside `e.data.payload`,
but not all assets read from that path (e.g., loot-box-3d reads `e.data.lootTable` directly),
creating silent protocol divergence.

ADR-007 (Single File System) explicitly allows multi-file source during Phase 1 (HTML
prototypes) with concatenation into single artifacts in Phase 2. This ADR defines the module
boundaries for that multi-file source and the contract for Phase 2 assembly.

**Guiding principle:** FERROS render assets remain authorable as standalone HTML compositions;
modularization exists to centralize repeated runtime, tokens, and reusable behaviors — not to
hide asset structure behind a framework.

---

## Decision

**Adopt a three-layer modular rendering system.**

### Layer 1: Rendering Core (`docs/assets/_core/`)

Canonical, consumed by all 16 assets. Contains:
- `_runtime.js` — asset-side messaging IIFE (emit, reportSize, message listener, control
  mode, reduced-motion, screen-reader announce, ResizeObserver)
- `_a11y.css` — `.sr-only`, `:focus-visible`, `prefers-reduced-motion` base rules
- `_stage.css` — perspective grid floor, horizon fog

Design tokens live in the existing `_tokens.css` (promoted to single canonical source, ~100
lines absorbing rarity and stage tokens). `_core/` does NOT contain tokens.

`_core/` imports nothing. Everything else can import `_core/`.

### Layer 2: Shared Library (`docs/assets/_lib/`)

Domain-specific primitives consumed by multiple assets. Opt-in per-asset. Contains:
- `_rarity.css` / `_rarity.js` — rarity badge/glow classes, probability engine
- `_shimmer.css` — `ticketShimmer` keyframe (7 consumers)
- `_particles.js` — generic radial burst particles
- `_abilities.css` — card ability animation keyframes + button styles
- `_sector-avatars.js` — sector emoji map
- `_escape-html.js` — `esc()` XSS-safe helper

Extraction rule: only create a `_lib/` module when 2+ assets share the same code.
`_lib/` modules import `_core/` only. `_lib/` modules do NOT import each other.

### Layer 3: Asset Compositions (existing asset files)

Each asset remains a single self-contained HTML file. Contains inline markers for the `_core/`
and `_lib/` modules it needs, plus its own geometry, state machine, UI, and event handlers.
Assets NEVER import other assets.

### Assembly Contract

Each asset file is a working HTML document at all times. Shared code is included inline
(copy-pasted during development, auto-inlined by build script for distribution).

Inline marker syntax (one rule per context):
- CSS context: `/* @inline _core/_a11y.css */`
- JS context: `/* @inline _core/_runtime.js */`
- HTML context: `<!-- @inline _tokens.css -->`

Pattern: `@inline <relative-path-from-docs/assets/>`. Build script
(`docs/assets/_build/inline.js`, Phase 2 of ADR-007) replaces markers + pasted content
with fresh copies from the source file.

### Runtime API Contract

`_core/_runtime.js` exposes `window.ferrosRuntime`:
```
ferrosRuntime.root                  — #ferros-asset-root element
ferrosRuntime.isEmbedded            — boolean
ferrosRuntime.control()             — current control mode string
ferrosRuntime.emit(event, payload)  — sends postMessage + CustomEvent
ferrosRuntime.reportSize()          — sends ferros:resize to parent
ferrosRuntime.isReducedMotion()     — boolean from matchMedia
ferrosRuntime.announceToScreenReader(text)
```

Assets provide optional callbacks:
```
window.onFerrosInit(config)         — called on ferros:init
window.onFerrosUpdate(payload)      — called on ferros:update (normalized to e.data.payload)
```

Contract rules:
1. Runtime OWNS `root.dataset.control`. Assets read it via CSS `[data-control]` selectors.
2. Runtime reads `ASSET_NAME` from `document.documentElement.dataset.ferrosAsset`.
3. Runtime normalizes inbound `ferros:update` to always read `e.data.payload`.
4. `emit()` sends BOTH `postMessage` (iframe parent) AND `CustomEvent` (same-page listeners).
5. Assets provide `#ferros-sr-announce` element if using screen-reader announcements.

### Module Versioning

Every module file starts with a version header: `/* ferros-runtime v1.0.0 */`.
`_ledger.html` tracks which module version each asset was last inlined from.

Semver-light:
- Patch: bugfix (safe to auto-update all consumers)
- Minor: additive (safe, no breaking changes)
- Major: breaking (requires per-asset review)

Build script validates version headers match between source modules and inlined copies.

---

## What Is Explicitly NOT Modularized

| Item | Why It Stays Local |
|------|-------------------|
| Camera presets | Every asset uses unique perspective/rotation values |
| State machines | Deeply coupled to each asset's DOM structure |
| DOM structure / HTML templates | "Open the file, see the structure" is a core authoring value |
| Loot tables and content data | Content, not infrastructure |
| Odds overlays and result UIs | Asset-specific composition, not shared rendering |

---

## Consequences

### Positive
- Token drift eliminated (one source of truth for all tokens)
- Protocol bugs fixed once (loot-box `ferros:update` divergence fixed by canonical runtime)
- Accessibility consistency enforced (`.sr-only`, `isReducedMotion()`, `focus-visible` all
  from one source)
- New assets scaffold in ~50 lines instead of ~200

### Negative
- Developers must know which code is canonical vs. local
- The `@inline` markers are a convention until the build script enforces them
- 3 new directories (`_core/`, `_lib/`, `_build/`) for agents to be aware of

### Risks
- Premature extraction creates modules with single consumers (mitigate: Rule of Three)
- Build script becomes a development requirement (mitigate: assets always work standalone)
- CSS specificity collisions between core and asset styles (mitigate: core = low specificity,
  assets = scoped to own classes)

---

## Anti-Patterns to Avoid

- No `registerComponent()` factories — this is not React
- No ES modules (`import`/`export`) — breaks `file://`
- No JS theme injection — CSS custom properties handle theming
- No shared state store — iframe assets have no shared JS context
- No DOM templates — structure stays in HTML where it's visible
- No base class inheritance — assets are compositions, not subclasses

---

## Related ADRs

- [ADR-007: Single File System](./ADR-007-single-file-system.md) — establishes Phase 1→2→3→4
  migration path; this ADR defines the Phase 1→2 module boundaries
- [ADR-001: Progression-Lock Pattern](./ADR-001-progression-lock-pattern.md) — hash-chain
  integrity applies to module hashes in the ledger
- [ADR-0001: Start New, Do Not Fork](./ADR-0001-start-new-do-not-fork.md) — Rust from scratch
