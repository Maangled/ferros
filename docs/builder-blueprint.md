# Builder Blueprint — Architecture Design Lab Direction

**Status:** Spec / Direction Document  
**Wave:** Wave 1 Exploration  
**Relates to:** `docs/progress/forge.md`, `docs/forge-workbench.html`, `docs/contracts/runtime-host-v1.md`, `docs/progress/arena-runtime.md`

---

## 1. What Forge Is Currently For

The Forge (`docs/forge-workbench.html`) is the FERROS **Wave 1 authoring surface for parametric 3D assets**. Its role in the platform model is to be the place where:

- **Cards** (atomic parametric assets — e.g., a loot box, an animation loop, a logo component) are authored and inspected.
- **Decks** (assembled compositions of Cards) are constructed and previewed locally.
- The **Bag** (local catalog) is browsed and selections are made.

Forge is currently at **40% / Prototype**: the workbench shell, asset catalog indexing, card visibility/selection, and parametric card fields all work. The next milestones are local assembly workflow → assistant edit bridge → command-center dispatch → Arena export → shared authoring contract → production.

The workbench uses a **four-corner docking layout** (per ADR-009): two floating panels on each side of a full-bleed center viewport. The viewport is an `<iframe>` seam that loads asset HTML targets and communicates via the `ferros:init` / `ferros:update` / `ferros:event` / `ferros:resize` postMessage protocol (Runtime Host Contract v1, `docs/contracts/runtime-host-v1.md`).

Forge is intentionally **file/local-first**: no build tools, no CDN imports, no Node, no `fetch` calls on the new offline surfaces. It must work when opened directly from disk via `file://`.

---

## 2. Is a Visual/3D Architecture Workbench Reasonable Under FERROS Constraints?

### Short answer: Yes — in phases, using HTML/CSS/JS "illusions" first.

FERROS constraints are strict:

| Constraint | Source | Impact on 3D authoring |
|---|---|---|
| Zero external dependencies | `docs/AGENT_GUIDE.md` §2 | No Three.js, Babylon.js, or WebGL frameworks — at least not as downloaded packages |
| No CDN / script imports / fetch for new offline surfaces | `docs/AGENT_GUIDE.md` §2 | Libraries must be inlined or forgone |
| `file://` protocol compatibility | `docs/AGENT_GUIDE.md` §1 | `crypto.subtle` unavailable; no module imports; no Service Workers |
| Iframe/postMessage seam | `docs/contracts/runtime-host-v1.md` | Viewer lives in a sandboxed iframe; host communicates via postMessage only |

**What "illusions" means here:** CSS 3D transforms, `perspective`, `rotateX/Y/Z`, `box-shadow`, and layered HTML/CSS panel stacking can produce convincing false-3D representations of walls, rooms, and building masses — as demonstrated by the existing loot-box card assets in `docs/assets/loot/`. These are not ray-traced renders; they are carefully crafted 2D/2.5D compositions that visually read as 3D. For architect review and schema validation at this stage, illusions are more than sufficient.

**The case for illusions as a first pass:**

1. **Zero-dependency compliance.** A CSS illusion requires no external library. It works on `file://` day one.
2. **Fast iteration.** Parametric walls, windows, and slabs modeled as CSS variables are instantly tweak-able through the inspector panel — the same way Forge card fields drive the loot-box card states today.
3. **Schema-proving.** The real deliverable at this stage is a validated data model for parametric architecture parts (wall, window, door, slab, roof) and assembly manifests (Deck equivalent). The visual illusion is the scaffolding that makes that schema tangible for architects.
4. **Architect review artifact.** A rich HTML/CSS workbench is a powerful review document for stakeholders who can open it in any browser without installing anything.

**What illusions cannot do (and don't need to do yet):**

- Real physics simulation or collision detection
- Accurate structural load analysis
- Photorealistic rendering
- Large-scale terrain / BIM data import

These capabilities will require the Rust-backed geometry/physics layer described in §4.

---

## 3. Recommended Phased Path

### Phase 1 — Near-Term: HTML/CSS/JS Workbench + Visual Illusions

**Goal:** Fast prototyping, schema proving, and architect review.

- Build `docs/builder-blueprint-architecture-lab.html` as a **Forge-style workbench** (four-corner docking, iframe viewport, status bar).
- Central viewport uses CSS 3D illusions for part and assembly previews — the same technique as the loot-box assets.
- Library/Bag panel contains parametric building **Parts** (wall, window, door, slab, roof module) and **Assemblies** (micro home, cabin, rowhouse) as catalog entries — mirroring Forge's `PART_INDEX` and `ASSEMBLY_INDEX` fixture pattern.
- Inspector panel exposes part-level parametric fields (length, height, material, etc.) using the same `editor-grid` / `editor-field` pattern as Forge's card inspector.
- Tool palette (select, move, draw wall, place window, etc.) updates a status label in the bottom bar.
- Assistant panel (bottom-right) is consent-first: a "Generate Plan" action populates a proposal area but does not apply changes until the architect clicks Allow. This mirrors the FERROS consent philosophy.
- No backend calls. All data is in-page JSON fixtures.
- Status: **starts the lab** — the workbench skeleton that architects can open, review, and critique.

**Output artifacts:**
- `docs/builder-blueprint-architecture-lab.html` (workbench)
- `docs/assets/architecture/parts/` — placeholder part viewer HTML targets (parametric wall, window, door, slab, roof)
- `docs/assets/architecture/assemblies/` — placeholder assembly viewer HTML targets (micro-home, cabin, rowhouse)

### Phase 2 — Mid-Term: Richer Local Assembly + Parametric Authoring

**Goal:** Use the existing Card/Deck/Bag / manifest / iframe seam from Forge to manage real parametric architecture components.

- Architecture Parts become **Cards** conforming to `schemas/card.schema.json`. Part parameters (wall thickness, window dimensions, etc.) are Card fields.
- Architecture Assemblies become **Decks** conforming to `schemas/deck.schema.json`. Each assembly is a Deck manifest listing Part Cards with transforms (`{ tx, ty, tz, rx, ry, rz, scale }`), named slots, and `instanceOf` references.
- The Builder workbench can load, assemble, and locally preview multi-part assemblies without backend services — matching Forge's Milestone Gate at 50% (local assembly workflow).
- The assistant edit bridge (Forge Gate 60%) enables an assistant or local agent bridge to create or edit architecture Parts without replacing the standalone workflow.
- The **Twin Architecture** positioning becomes concrete: the workbench tracks a physical-world coordinate anchor per Assembly (lat/lon or local-CRS) and can generate a minimal JSON export that references both the digital model and its real-world anchor — the "twin manifest."
- Phase 2 reuses the Runtime Host Contract (`ferros:init`, `ferros:update`) to drive part viewer iframes from the workbench.

### Phase 3 — Long-Term: Rust-Backed Geometry/Physics Bridge

**Goal:** Optional migration or bridge to the Rust-native OS renderer when it exists.

- FERROS is a Rust-native OS at its long-term horizon. When the core OS geometry/physics layer materializes, the HTML workbench becomes the **authoring front-end** that hands off validated Assembly manifests to the Rust layer for accurate structural analysis, physics simulation, and high-fidelity rendering.
- The bridge will likely follow the WASM path already scoped in Legacy Integration item L10 (`workpace-rust build-wasm.sh`). The Assembly manifest schema (Phase 2) becomes the contract between the HTML authoring surface and the Rust renderer.
- The iframe/postMessage seam from the Runtime Host Contract v1 is already designed for this: the viewer iframe can be swapped from a CSS-illusion HTML target to a WASM-backed renderer without changing the host workbench code.
- The consent-first assistant panel evolves into a full agent integration point, connected to the Agent Command Center (`docs/agent-command-center.html`) and, eventually, to the Sheetgen/Botgen architectural drawing automation pipeline.

---

## 4. Forge → Arena Runtime → Architecture Builder / Twin Architecture Lab

```
                        ┌──────────────────────────────────────────────────────────┐
                        │                     FERROS Platform                       │
                        └──────────────────────────────────────────────────────────┘
                                              │
              ┌───────────────────────────────┼───────────────────────────────┐
              │                               │                               │
   ┌──────────▼──────────┐       ┌────────────▼───────────┐     ┌────────────▼──────────────┐
   │   THE FORGE          │       │   ARENA RUNTIME         │     │  ARCHITECTURE BUILDER LAB  │
   │  (authoring surface) │       │  (rendering/experience) │     │  (Twin Architecture lab)   │
   │                      │       │                         │     │                            │
   │  Cards · Decks · Bag │──────▶│  Hosts Cards + Decks    │     │  Parts · Assemblies · Bag  │
   │  3D asset authoring  │       │  as live portals        │     │  parametric building parts │
   │  parametric fields   │       │  animation, battle,     │     │  assembly into homes       │
   │  loot-box manifests  │       │  reward reveal, viewer  │     │  twin architecture export  │
   └──────────────────────┘       └─────────────────────────┘     └────────────────────────────┘
              │                                                              │
              │   both use iframe / postMessage (Runtime Host Contract v1)  │
              └──────────────────────────────┬───────────────────────────────┘
                                             │
                                  ┌──────────▼──────────┐
                                  │  RUST OS (future)   │
                                  │  geometry · physics  │
                                  │  WASM bridge         │
                                  └─────────────────────┘
```

**Forge** is the source: it authors atomic assets (Cards) and compositions (Decks). Its manifests (`docs/assets/loot/`) and iframe/postMessage seam are the canonical authoring patterns.

**Arena Runtime** is the destination for Forge outputs: it renders Cards and Decks as live experiences (game portals, reward reveals, animation loops). Arena is currently at 1% — the runtime concept is still embedded inside the trading arena prototype.

**Architecture Builder Lab** is a **parallel authoring surface** using the same Forge patterns, but specialized for parametric building components (Parts = Cards, Assemblies = Decks). It introduces the **Twin Architecture** dimension:

- **Traditional digital twin:** the Assembly manifest can carry a real-world coordinate anchor, linking the digital model to a physical location.
- **FERROS twin of mediums:** the HTML/CSS/JS workbench and the (future) Rust OS renderer are "twinned" — the same Assembly manifest is valid input for both. The authoring surface and the runtime surface are two sides of the same model.

The Architecture Builder Lab does **not** replace Forge. It extends the same Card/Deck/Bag vocabulary and Runtime Host Contract into a new domain, proving that the FERROS authoring model is general enough to cover 3D game assets *and* parametric building design under the same contracts.

---

## 5. Generator Prompt / Spec

The following is a ready-to-use prompt for generating `docs/builder-blueprint-architecture-lab.html`. This prompt is the strongest visual-direction artifact possible for architects to review, grounded in the current Forge shell.

---

### PROMPT — FERROS BUILDER BLUEPRINT: Architecture Design Lab Workbench

**Repository:** `Maangled/ferros`  
**Output file:** `docs/builder-blueprint-architecture-lab.html`  
**Primary reference:** `docs/forge-workbench.html` — reuse its layout patterns: `forge-shell`, `forge-nav`, four corner panels (`corner-tl`, `corner-tr`, `corner-bl`, `corner-br`), `forge-viewport` iframe, `forge-status`, catalog/inspector structure, `panel-header`, `panel-scroll`, `catalog-group`, `asset-item`, `inspector-section`, `editor-grid`, `editor-field`, panel collapse behavior, mono typography, translucent glass panels, CSS variable system from `docs/assets/_tokens.css`.  
**Secondary reference:** `docs/ferros-blueprint.html` — use the FERROS design system variables, card/panel/section patterns, gradient header, badge system.  
**FERROS constraints:** Zero external dependencies. No CDN links. No `<script src="...">`. No ES module imports. No `fetch` calls. Must work on `file://` protocol. Inline all CSS in a `<style>` block. Inline all JS in a `<script>` block.

---

#### Product Intent (write this into the UI copy)

Build a **parametric architecture workbench** — the Forge for Architects — that can:
- define **Parts** (parametric walls, windows, doors, stairs, roof modules) as the architectural equivalent of Cards
- assemble **Buildings** (homes, cabins, rowhouses) from those Parts as Assemblies — the architectural equivalent of Decks
- run a **Twin Architecture** pipeline:
  - *Traditional twin:* digital model of the physical environment, anchored to real-world coordinates
  - *FERROS twin of mediums:* the HTML/CSS/JS workbench and the future Rust OS renderer are two sides of the same model — one manifest, two surfaces

---

#### Data Model (in-page JS fixtures, no fetch)

Embed three JS constants at the top of the script block:

```js
const PART_INDEX = [
  { id: 'wall-straight',   label: 'Parametric Wall (straight)', group: 'Parts',     subtitle: 'part · wall · parametric',   note: 'Straight wall segment with studs, openings list, and material override' },
  { id: 'wall-curved',     label: 'Parametric Wall (curved)',   group: 'Parts',     subtitle: 'part · wall · parametric',   note: 'Curved wall segment; radius + arc angle control' },
  { id: 'window-rect',     label: 'Window (rect)',              group: 'Parts',     subtitle: 'part · window · parametric', note: 'Rectangular window with sill height, mullion pattern, frame thickness' },
  { id: 'door-swing',      label: 'Door (swing)',               group: 'Parts',     subtitle: 'part · door · parametric',   note: 'Single swing door; left/right hand, jamb depth' },
  { id: 'floor-slab',      label: 'Floor Slab',                 group: 'Parts',     subtitle: 'part · slab · parametric',   note: 'Horizontal slab with thickness, elevation, and material' },
  { id: 'roof-module',     label: 'Roof Module',                group: 'Parts',     subtitle: 'part · roof · parametric',   note: 'Gable or shed roof; pitch, overhang, and thickness' },
  { id: 'micro-home-v0',   label: 'Micro Home v0',              group: 'Assemblies', subtitle: 'assembly · home · v0',       note: '~35 m² single-room starter assembly; 4 walls + slab + roof' },
  { id: 'rowhouse-v0',     label: 'Rowhouse v0',                group: 'Assemblies', subtitle: 'assembly · rowhouse · v0',   note: 'Narrow 2-storey rowhouse; shared party wall ready' },
  { id: 'cabin-v0',        label: 'Cabin v0',                   group: 'Assemblies', subtitle: 'assembly · cabin · v0',      note: 'Timber cabin; steep pitch roof + deck module' },
  { id: 'lot-boundary',    label: 'Lot Boundary',               group: 'Sites',     subtitle: 'site · context · boundary',  note: 'Rectangular lot perimeter with setback guide' },
  { id: 'terrain-patch',   label: 'Terrain Patch',              group: 'Sites',     subtitle: 'site · context · terrain',   note: 'Flat terrain tile; slope angle and texture placeholder' },
  { id: 'street-grid',     label: 'Street Grid',                group: 'Sites',     subtitle: 'site · context · grid',      note: 'Urban street grid tile; lane width + sidewalk' },
  { id: 'scan-import',     label: 'Scan Import',                group: 'Twin',      subtitle: 'twin · connector · scan',    note: 'Placeholder: import from photogrammetry or LiDAR scan' },
  { id: 'map-tiles',       label: 'Map Tiles',                  group: 'Twin',      subtitle: 'twin · connector · map',     note: 'Placeholder: anchor assembly to real-world map tile' },
];

const ASSEMBLY_INDEX = [
  { id: 'micro-home-v0', label: 'Micro Home v0',  version: '0.1.0', description: 'Minimal single-room home assembly.' },
  { id: 'rowhouse-v0',   label: 'Rowhouse v0',    version: '0.1.0', description: 'Narrow two-storey rowhouse.' },
  { id: 'cabin-v0',      label: 'Cabin v0',        version: '0.1.0', description: 'Timber cabin with steep roof.' },
];

const ASSEMBLY_MANIFESTS = {
  'micro-home-v0': {
    id: 'micro-home-v0', schemaVersion: '1.0.0',
    components: [
      { slot: 'north-wall',  instanceOf: 'wall-straight', transform: { tx: 0,   ty: 0, tz: 0,   rx: 0, ry: 0,   rz: 0, scale: 1 }, params: { length: 5000, height: 2700, thickness: 140 } },
      { slot: 'south-wall',  instanceOf: 'wall-straight', transform: { tx: 0,   ty: 0, tz: 5000, rx: 0, ry: 0,   rz: 0, scale: 1 }, params: { length: 5000, height: 2700, thickness: 140 } },
      { slot: 'east-wall',   instanceOf: 'wall-straight', transform: { tx: 5000, ty: 0, tz: 0,  rx: 0, ry: 90,  rz: 0, scale: 1 }, params: { length: 5000, height: 2700, thickness: 140 } },
      { slot: 'west-wall',   instanceOf: 'wall-straight', transform: { tx: 0,   ty: 0, tz: 0,   rx: 0, ry: 90,  rz: 0, scale: 1 }, params: { length: 5000, height: 2700, thickness: 140 } },
      { slot: 'slab',        instanceOf: 'floor-slab',    transform: { tx: 0,   ty: 0, tz: 0,   rx: 0, ry: 0,   rz: 0, scale: 1 }, params: { thickness: 200, elevation: 0 } },
      { slot: 'roof',        instanceOf: 'roof-module',   transform: { tx: 0,   ty: 2700, tz: 0, rx: 0, ry: 0,  rz: 0, scale: 1 }, params: { pitch: 25, overhang: 300, thickness: 100 } },
    ]
  }
};
```

---

#### Inspector Parametric Fields by Part Type

When a Part is selected, the Inspector renders appropriate fields using the `editor-grid` / `editor-field` pattern from Forge:

| Part | Fields |
|---|---|
| wall-straight | `length` (mm), `height` (mm), `thickness` (mm), `material` (text), `studsSpacing` (mm), `openings` (list) |
| wall-curved | `radius` (mm), `arcAngle` (deg), `height` (mm), `thickness` (mm), `material` (text) |
| window-rect | `width` (mm), `height` (mm), `sillHeight` (mm), `mullionPattern` (text), `frameThickness` (mm) |
| door-swing | `width` (mm), `height` (mm), `swing` (L/R), `jambDepth` (mm) |
| floor-slab | `thickness` (mm), `elevation` (mm), `material` (text) |
| roof-module | `pitch` (deg), `overhang` (mm), `thickness` (mm) |

---

#### Required Workbench Layout (match Forge exactly)

Use the same 5-region shell as `docs/forge-workbench.html`:

**1. Top Nav Bar (`forge-nav`)**
```
[FERROS BUILDER BLUEPRINT]  [Architecture Design Lab · parametric assembly + twin architecture]
                                [7 parts indexed ●]  [twin: disconnected ○]  [consent: required ⚠]
```

**2. Center Viewport (`forge-viewport`)**
- Keep the `<iframe>` viewer pattern from Forge.
- Default empty-state message (shown before any selection):

  > *No part selected.*  
  > *Choose a part or assembly from the Library to begin.*  
  > *Builder v0.1 · Twin Architecture*

- HUD corners (absolutely positioned `<div>` overlays on the viewport):
  - Top-left: `BUILDER v0.1`
  - Top-right: `scene: house-prototype-01 | mode: parametric`
  - Bottom-left: unit badge: `mm`
  - Bottom-right: `twin: disconnected`

**3. Top-Left Panel — Library (`corner-tl`)**

Panel title: `📦 Library`

Catalog groups (collapsible, same `.catalog-group` pattern as Forge):
- **Parts** — wall-straight, wall-curved, window-rect, door-swing, floor-slab, roof-module
- **Assemblies** — micro-home-v0, rowhouse-v0, cabin-v0
- **Sites / Context** — lot-boundary, terrain-patch, street-grid
- **Twin Connectors** — scan-import (placeholder), map-tiles (placeholder)

Each item: icon emoji + label + mono subtitle + note line.

**4. Top-Right Panel — Inspector (`corner-tr`)**

Panel title: `🧭 Inspector`

Inspector sections (`.inspector-section` pattern from Forge):

- **Selection** — name, type, role, uid
- **Parametrics** — part-specific `editor-grid` fields per the table above; show placeholder state when nothing is selected: *"Select a part to edit its parameters."*
- **Constraints & Rules** — snap settings toggle, min/max constraint note, collision/overlap rules (placeholder label)
- **Twin Mapping** — toggle: "Anchor to real-world coordinate system" (disabled placeholder); label: "Link to sensor/scan source" (disabled placeholder)
- **Actions** — buttons: `Add to Assembly`, `Duplicate`, `Export`, `Request AI Assist…` (opens consent area)

**5. Bottom-Left Panel — Tools (`corner-bl`)**

Panel title: `🛠 Tools`

Tool button grid (toggle-style, 3-column grid). Each tool updates `data-active-tool` and the status bar label when clicked:

```
[Select]      [Move]        [Rotate]
[Scale]       [Draw Wall]   [Place Window]
[Place Door]  [Define Room] [Measure]
[Section Cut] [Orbit Camera]
```

**6. Bottom-Right Panel — Lab Assistant (`corner-br`)**

Panel title: `🤖 Lab Assistant`

Prominent two-line **Twin Architecture** definition block at the top of this panel:
```
Twin Architecture
  · Digital twin of physical space
  · Twin of mediums: 3D modeling ↔ web modeling system
```

Content:
- Textarea: `Describe what you want to build…`
- Button: `Generate Plan` — populates a read-only "proposal" area below
- Button: `Propose Parametric Parts` — lists suggested parts in the proposal area
- **Consent gate** (always visible after any proposal is generated):
  - Label: *"AI proposal ready — review before applying"*
  - `[Allow ✓]` button — applies the proposal (stub: logs to console)
  - `[Deny ✗]` button — clears the proposal area
  - Copy below: *"FERROS is consent-first. No changes are applied without your explicit approval."*

**7. Bottom Status Bar (`forge-status`)**

```
● ready  |  selection: —  |  tool: select  |  grid: on  |  snap: on  |  twin: disconnected  |  units: mm  |  source: library
```

---

#### Viewer Strategy (keep iframe seam)

Like Forge, the viewport loads an HTML render target in an iframe using the Runtime Host Contract v1 postMessage protocol. For each part/assembly, point `iframe.src` to a planned path:

- `docs/assets/architecture/parts/parametric-wall.html`
- `docs/assets/architecture/parts/parametric-window.html`
- `docs/assets/architecture/parts/parametric-door.html`
- `docs/assets/architecture/parts/floor-slab.html`
- `docs/assets/architecture/parts/roof-module.html`
- `docs/assets/architecture/assemblies/micro-home.html`
- `docs/assets/architecture/assemblies/rowhouse.html`
- `docs/assets/architecture/assemblies/cabin.html`

For the prototype, if the viewer file does not yet exist, use a `data:text/html,…` URL with a minimal CSS 3D illusion placeholder that represents the selected part type. This keeps the workbench functional without requiring separate files on day one.

**CSS 3D illusion target for a wall:** a simple `perspective` + `rotateX`/`rotateY` CSS box in blueprint colors, with CSS custom properties driving width, height, and depth from postMessage `ferros:update` payloads.

---

#### JS Behavior (minimal, in-page only)

Implement in a single `<script>` block. No modules, no fetch, no CDN.

```
selectPart(id)
  → update Inspector fields for the selected part
  → update status bar "selection:" field
  → load viewer iframe (data: URL or path)
  → post ferros:init to iframe

selectTool(name)
  → toggle .tool-active class on the tool button
  → update status bar "tool:" field

generateProposal()
  → populate proposal textarea with stub text based on Library selection
  → show consent gate UI

applyProposal()
  → stub: console.log('proposal applied')
  → clear proposal area
  → hide consent gate

denyProposal()
  → clear proposal area
  → hide consent gate

togglePanel(id)
  → existing Forge panel collapse behavior (toggle panel-collapsed class + arrow icon)
```

---

#### Visual Direction / Aesthetic

- Inherit the FERROS dark-panel design system: `--bg-deep`, `--bg-surface`, `--text-primary`, `--accent-blue`, `--accent-green`, `--border-dim` from `_tokens.css`.
- Blueprint accent color: use `--accent-blue` for architecture grid lines, panel borders, and HUD labels.
- Add a subtle blueprint-grid background to the viewport: CSS `background-image: linear-gradient(…)` repeating grid at `20px` intervals, very low opacity (~5%), no external resource.
- Panel glass: same `backdrop-filter: blur(12px)` + `rgba(14, 18, 28, 0.72)` as Forge.
- Typography: same mono stack as Forge (`'SF Mono', 'Cascadia Code', 'Fira Code', 'Consolas', monospace`).

---

#### Naming / Branding Requirements

- Page `<title>`: `FERROS BUILDER BLUEPRINT — Architecture Design Lab`
- Nav brand text: `FERROS BUILDER BLUEPRINT`
- Nav subtitle: `Architecture Design Lab · parametric assembly + twin architecture`
- The phrase **Twin Architecture** must appear prominently in the Assistant panel with its two-line definition.
- Do not use the phrase "Revit replacement" or reference competitors by name in the UI copy.

---

#### Technical Requirements Checklist

- [ ] All CSS in a single `<style>` block
- [ ] All JS in a single `<script>` block, no `type="module"`
- [ ] No external `<script src>` tags
- [ ] No `fetch()` calls
- [ ] Works when opened via `file://` with no server
- [ ] Four-corner docking layout matching Forge
- [ ] Center viewport is a full-bleed `<iframe>`
- [ ] Catalog groups are collapsible
- [ ] Clicking a Library item populates Inspector and loads viewer
- [ ] Tool palette toggles active tool + updates status bar
- [ ] Assistant panel has consent gate before applying any proposal
- [ ] Bottom status bar reflects current selection, tool, and twin state
- [ ] Responsive at 1400px, 1100px, 760px breakpoints (matching Forge media queries)
- [ ] File size target: under 150 KB

---

## 6. Where to Link This Document

- **`docs/progress/PROGRESS.md`** — Add a reference in the Wave 1 upcoming work section as the Architecture Builder Lab direction spec.
- **`docs/progress/forge.md`** — Add a "Related Specs" note at the bottom pointing to this document, since the Architecture Builder Lab is the first Forge-pattern extension into a new domain.
- **`README.md`** — When `docs/builder-blueprint-architecture-lab.html` is created, list it in the surfaces table alongside Forge.

---

## 7. Terminology Used in This Document

All terms below are consistent with the FERROS platform vocabulary:

| Term | Meaning in this context |
|---|---|
| **Part** | Parametric building component (architectural equivalent of a Forge Card) |
| **Assembly** | Composed building from multiple Parts (architectural equivalent of a Forge Deck) |
| **Bag** | Local catalog of available Parts and Assemblies (same term as Forge) |
| **Twin Architecture** | (1) Digital twin of a physical building/site; (2) FERROS-specific pairing of the HTML authoring surface and the future Rust OS renderer as two sides of the same model |
| **Builder Blueprint** | This direction document; also the name of the workbench page it specifies |
| **Illusions** | CSS 3D transforms used to produce convincing false-3D representations without WebGL or external libraries |
| **Manifest** | JSON document describing an Assembly's component list, slots, transforms, and parameters — same format as Forge Deck manifests |
| **iframe seam** | The Runtime Host Contract v1 postMessage channel between the workbench and the viewer iframe |
