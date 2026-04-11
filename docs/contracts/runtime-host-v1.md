# Runtime Host Contract v1

**Version:** 1.0.0  
**Status:** Draft  
**Related ADRs:** ADR-008 (Modular Rendering System)  
**Implementation:** `docs/assets/_core/_runtime.js`

---

## Purpose

The Runtime Host Contract defines how FERROS assets (Cards, Decks, compositions) are loaded, controlled, and communicate with their host environment. Any surface that renders FERROS assets — Forge workbench, Arena Runtime, Home HUD, or any future embedder — must implement this contract.

---

## 1. Message Protocol

All communication uses `postMessage` (cross-frame) and `CustomEvent` (same-document).

### 1.1 Host → Asset Messages

#### `ferros:init`

Sent once when the asset is first loaded or embedded.

```json
{
  "type": "ferros:init",
  "config": {
    "control": "demo | interactive | static",
    "state": "Open",
    "pose": {},
    "version": "1.0.0"
  }
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `config.control` | string | Yes | Control mode for the asset |
| `config.state` | string | No | Named state preset to apply on init |
| `config.pose` | object | No | Arbitrary pose data for init |
| `config.version` | string | No | Contract version the host expects |

**Asset response:** Call `window.onFerrosInit(config)` if defined. Apply control mode. Report initial size.

**Error behavior:** If the asset does not recognize the control mode, it MUST fall back to `"demo"` and emit a `ferros:event` with `event: "unsupported-control"`.

#### `ferros:update`

Sent any time the host wants to change asset state.

```json
{
  "type": "ferros:update",
  "payload": {
    "control": "interactive",
    "state": "Closed",
    "pose": { "lidAngle": 0 },
    "action": "set-state"
  }
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `payload.control` | string | No | New control mode |
| `payload.state` | string | No | Named state preset to apply |
| `payload.pose` | object | No | Arbitrary pose data |
| `payload.action` | string | No | Named action for the asset to perform |

**Idempotency:** Sending the same `ferros:update` twice MUST produce the same visual result. Assets must not accumulate duplicate state changes.

**Error behavior:** If `payload.action` is unrecognized, the asset MUST ignore it and emit `ferros:event` with `event: "unsupported-action"`.

### 1.2 Asset → Host Messages

#### `ferros:event`

Emitted by the asset when something happens worth reporting.

```json
{
  "type": "ferros:event",
  "asset": "loot-box-assembly",
  "event": "state-changed",
  "payload": { "state": "Open", "previousState": "Closed" }
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `asset` | string | Yes | Asset name from `data-ferros-asset` attribute |
| `event` | string | Yes | Event name |
| `payload` | object | No | Event-specific data |

**Standard events:**
- `state-changed` — asset transitioned between named states
- `ready` — asset has finished initialization
- `error` — asset encountered a problem
- `unsupported-control` — requested control mode not supported
- `unsupported-action` — requested action not recognized

#### `ferros:resize`

Emitted when the asset's rendered dimensions change.

```json
{
  "type": "ferros:resize",
  "asset": "loot-box-assembly",
  "width": 400,
  "height": 300
}
```

---

## 2. Asset Requirements

An asset that conforms to this contract MUST:

1. Set `data-ferros-asset="asset-name"` on the `<html>` element
2. Provide a `#ferros-asset-root` element as the rendering root
3. Include `_runtime.js` (or inline its contract-conforming equivalent)
4. Never redefine `window.ferrosRuntime`

An asset MAY:

1. Define `window.onFerrosInit(config)` to handle initialization
2. Define `window.onFerrosUpdate(payload)` to handle state changes
3. Provide `#ferros-sr-announce` for screen reader announcements

---

## 3. Control Modes

| Mode | Behavior |
|------|----------|
| `demo` | Default. Asset runs its own animation/interaction loop. No external input expected. |
| `interactive` | Asset responds to user input (click, drag, keyboard). Host may also send `ferros:update`. |
| `static` | Asset renders a single frame. No animation. No input. Used for thumbnails and previews. |

Control mode is set via:
- `?control=mode` query parameter on direct load
- `config.control` in `ferros:init` message
- `payload.control` in `ferros:update` message

Last-write wins. The current mode is stored in `#ferros-asset-root`'s `data-control` attribute.

---

## 4. Lifecycle

```
Host                             Asset
  |                                |
  |-- ferros:init { config } ---->|
  |                                |-- onFerrosInit(config)
  |                                |-- applyControl()
  |                                |-- reportSize()
  |<---- ferros:event { ready } --|
  |                                |
  |-- ferros:update { payload } ->|
  |                                |-- onFerrosUpdate(payload)
  |<---- ferros:event { ... } ----|
  |                                |
  |-- ferros:update { payload } ->|  (idempotent)
  |                                |
```

### 4.1 Init Timing

- `ferros:init` is sent ONCE per asset load
- If the asset is already loaded and receives a second `ferros:init`, it MUST re-initialize cleanly (no zombie state)
- The asset MUST emit `ferros:event { event: "ready" }` after init completes

### 4.2 Error Semantics

Assets MUST NOT throw uncaught exceptions from `onFerrosInit` or `onFerrosUpdate`. If an error occurs:

1. Log to console
2. Emit `ferros:event { event: "error", payload: { message: "..." } }`
3. Continue operating in the last known good state

### 4.3 Persistence Boundaries

- Assets do NOT write to `localStorage` or `sessionStorage` directly
- All persistence is the host's responsibility
- The host decides what to save based on `ferros:event` messages
- Assets MAY use in-memory state but MUST NOT assume it persists across loads

---

## 5. Versioning

- The contract version is `1.0.0` (semver)
- The host MAY send `config.version` in `ferros:init`
- If the asset does not support the requested version, it MUST emit `ferros:event { event: "unsupported-version", payload: { supported: "1.0.0", requested: "X.Y.Z" } }` and fall back to its own version
- Backward-incompatible changes require a major version bump and a new ADR

---

## 6. Capability Negotiation

Future extension point. Not implemented in v1.

When needed, the host will send a `ferros:capabilities` message listing what it supports (e.g., `["animation", "3d-transforms", "audio"]`). The asset will respond with its own capability list. Intersection determines available features.

For v1: all assets assume `demo`, `interactive`, and `static` control modes are available. No other capabilities are negotiated.

---

## 7. Reduced Motion

- `window.ferrosRuntime.isReducedMotion()` returns `true` if `prefers-reduced-motion: reduce` matches
- Assets MUST respect this by disabling animations or providing static alternatives
- The `_a11y.css` base rules handle basic motion reduction

---

## 8. Test Criteria

The contract is validated when:

1. An asset loaded in an iframe receives `ferros:init` and emits `ferros:event { ready }`
2. A `ferros:update` changing state produces a matching `ferros:event { state-changed }`
3. Sending the same `ferros:update` twice produces no additional state change events
4. An unrecognized action produces `ferros:event { unsupported-action }` and no crash
5. `ferros:resize` is emitted on init with correct dimensions
6. The asset works on `file://` protocol with no exceptions
