# Surface Bootstrap Contract

**Status:** Phase 0 (prototype)  
**Last updated:** Remediation Plan v3

---

## 1. Terminology

| Term | Definition |
|------|-----------|
| **Surface** | A top-level HTML file served at `file://` or `https://`. Each surface owns its own DOM and script scope. |
| **Asset** | An HTML fragment or page loaded inside an `<iframe class="ferros-embed">` within a surface. Communicates with the host via `postMessage`. |
| **FerrosCore** | Shared runtime IIFE (`docs/assets/_core/ferros-core.js`). Exposes `window.FerrosCore`. |
| **_embed.js** | Host-side message router IIFE (`docs/assets/_embed.js`). Manages iframe registry, nonce handshake, and event routing. |
| **_runtime.js** | Asset-side IIFE (`docs/assets/_core/_runtime.js`). Inlined at build time into each asset HTML. |

---

## 2. Script Loading Rules

1. **Zero external dependencies.** No CDN, no npm, no module bundler. All scripts are classic `<script>` tags or inline IIFEs.
2. **FerrosCore loads synchronously** before any consumer script. It must appear as `<script src="assets/_core/ferros-core.js"></script>` before any inline `<script>` block that references `FerrosCore.*`.
3. **_embed.js loads synchronously** after FerrosCore (when the surface embeds assets). It expects `FerrosCore.generateRuntimeNonce()` to exist but falls back to `Math.random` if unavailable.
4. **_runtime.js is inlined** (not loaded via `<script src>`). Build tooling copies it into each asset HTML file, marked with `/* @inline _core/_runtime.js */`. It executes immediately on parse — no `DOMContentLoaded` wrapper.
5. **Assets never load FerrosCore.** Only surfaces load FerrosCore. Assets use `ferrosRuntime.*` provided by the inlined _runtime.js.

---

## 3. Surface Init Patterns

Surfaces currently use one of three init strategies:

### 3a. DOMContentLoaded Handler

```
window.addEventListener('DOMContentLoaded', init);
```

Used by: `personal-profile.html`, `agent-command-center.html`.

`init()` is typically `async` and runs loadProfile, migration, routing, and render in sequence.

### 3b. Immediate IIFE

```
(function() { /* … all logic … */ })();
```

Used by: `schedule-ledger.html`, `algo-trading-arena.html`.

All initialization occurs at parse time with no event listener. Inner IIFEs may initialize discrete subsystems.

### 3c. Bottom-of-Body Inline

```
<script>
  // code runs immediately as last element in <body>
  updateClock();
  setInterval(updateClock, 1000);
</script>
```

Used by: `docs/legacy/home-hud-dashboard.html`.

Relies on DOM being available because the script tag is after all markup.

### Recommended Default

New surfaces SHOULD use **3a (DOMContentLoaded)** unless they have no async initialization.

---

## 4. Canonical Script Order (Surface with Assets)

```html
<!-- 1. Styles (inline or external) -->
<style>/* ... */</style>

<!-- 2. Markup -->
<main>
  <iframe class="ferros-embed"
          src="./assets/cards/trading-card.html"
          data-ferros-asset="trading-card"
          data-ferros-control="demo"></iframe>
</main>

<!-- 3. FerrosCore (must be first script) -->
<script src="assets/_core/ferros-core.js"></script>

<!-- 4. Embed router (after FerrosCore) -->
<script src="assets/_embed.js"></script>

<!-- 5. Surface-specific logic (after both) -->
<script>
  // FerrosCore.* and _embed.js routing are available here
</script>
```

---

## 5. Asset Lifecycle

### 5.1 Host Side (_embed.js)

1. On `DOMContentLoaded` (or immediately if already loaded), `initAll()` runs.
2. Finds all `<iframe class="ferros-embed">` elements.
3. For each iframe: `registerIframe()` stores a reference keyed by `data-ferros-asset` name.
4. Attaches an `iframe.addEventListener('load', …)` handler.
5. On iframe load:
   - Builds config from `data-ferros-control` and `data-ferros-data` attributes.
   - Generates a nonce via `FerrosCore.generateRuntimeNonce()`.
   - Sends `{ type: 'ferros:init', config, nonce }` via `postMessage`.
   - Marks asset as ready, flushes queued messages.
6. A `MutationObserver` watches for dynamically added `ferros-embed` iframes.

### 5.2 Asset Side (_runtime.js)

1. IIFE executes immediately on parse.
2. Reads `document.documentElement.dataset.ferrosAsset` → `ASSET_NAME`.
3. Finds `#ferros-asset-root` element.
4. Detects embedding: `window.parent !== window`.
5. Sends `ferros:resize` to parent with initial dimensions.
6. Sets up `ResizeObserver` on root for ongoing height updates.
7. Registers `message` listener:
   - `ferros:init` → stores nonce, applies control mode, calls `window.onFerrosInit(config)`.
   - `ferros:update` → applies changes, calls `window.onFerrosUpdate(payload)`.
8. Exposes `window.ferrosRuntime = { root, isEmbedded, control(), emit(), reportSize(), isReducedMotion(), announceToScreenReader() }`.

### 5.3 Runtime Communication Flow

```
HOST                                  ASSET
────                                  ─────
  ──[iframe load]──────────────────────>
                                        _runtime.js: IIFE runs, sends ferros:resize
  <──[ferros:resize]────────────────────
  ──[ferros:init {config, nonce}]──────>
                                        stores nonce, calls onFerrosInit(config)
                                        ...user interaction...
  <──[ferros:event {asset, event, payload, nonce}]──
  validates nonce
  dispatches CustomEvent on document
  routes to target assets
  ──[ferros:update {payload}]──────────>
                                        calls onFerrosUpdate(payload)
```

---

## 6. Required Asset HTML Structure

```html
<!DOCTYPE html>
<html data-ferros-asset="my-asset-name">
<head>
  <style>/* asset styles */</style>
</head>
<body>
  <div id="ferros-asset-root">
    <!-- asset markup -->
  </div>
  <script>/* @inline _core/_runtime.js */</script>
  <script>
    window.onFerrosInit = function(config) { /* … */ };
    window.onFerrosUpdate = function(payload) { /* … */ };
  </script>
</body>
</html>
```

Requirements:
- `data-ferros-asset` on `<html>` element (must match host's `data-ferros-asset` attribute on the iframe).
- `#ferros-asset-root` element present in DOM.
- `_runtime.js` inlined before any asset-specific scripts.

---

## 7. Iframe Attributes

| Attribute | Required | Description |
|-----------|----------|-------------|
| `class="ferros-embed"` | Yes | Selector used by `_embed.js` to discover iframes |
| `data-ferros-asset` | Yes | Unique asset name (falls back to `src` basename) |
| `data-ferros-control` | No | Initial control mode: `"demo"`, `"live"`, etc. |
| `data-ferros-data` | No | JSON-serialized initial data payload |
| `data-ferros-routes` | No | Override default routing table for this asset |

---

## 8. Current Surface Inventory

| Surface | Loads FerrosCore | Uses _embed.js | Uses iframes | Init Pattern |
|---------|-----------------|----------------|--------------|--------------|
| personal-profile.html | Yes | No | No | DOMContentLoaded |
| docs/legacy/home-hud-dashboard.html | No | No | No | Bottom-of-body |
| schedule-ledger.html | No | No | No | Immediate IIFE |
| forge-workbench.html | No | No | Yes (manual) | Immediate IIFE |
| algo-trading-arena.html | No | No | No | Immediate IIFE |
| agent-command-center.html | No | No | No | DOMContentLoaded |

**Note:** The forge uses a plain `<iframe id="viewer-frame">` with manual `postMessage` — it does not follow the `_embed.js` protocol. Migration to the standard pattern is a future task.

---

## 9. Nonce Handshake (v1.1)

Per [runtime-host-v1.md](runtime-host-v1.md) §9.1:

1. Host generates nonce via `FerrosCore.generateRuntimeNonce()` (32-char hex).
2. Nonce is included in the `ferros:init` message.
3. Asset stores nonce and echoes it in every `ferros:event`.
4. Host validates nonce on receipt; mismatched events are silently dropped.
5. Under `file://`, this provides conformance guarantees only — not a security boundary.
