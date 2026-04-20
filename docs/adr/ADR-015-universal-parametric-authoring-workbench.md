# ADR-015: Universal Parametric Authoring Workbench

## Status
Accepted

## Date
2026-04-20

## Context

The Forge (`docs/forge-workbench.html`) was introduced as a Wave 1 authoring surface for
parametric 3D assets — specifically loot-box Cards assembled from modular parts. It proved
that the four-corner docking layout (ADR-009), the Card/Deck/Bag nomenclature (ADR-010), the
runtime-host postMessage contract (C8, `docs/contracts/runtime-host-v1.md`), and an
iframe-isolated viewport could combine into a productive local workbench with zero external
dependencies.

A parallel conversation has surfaced a demand for an **Architecture Design Lab** — a
workbench for parametric building authoring (walls, windows, doors, slabs, roof modules,
assembled into homes and connected to the physical world through Twin Architecture). The
natural question: should this be a separate system, or a domain specialization of the same
Forge shell and contract model?

Simultaneously, the `docs/builder-blueprint.md` direction spec documented:

- The viability of HTML/CSS/JS "illusions" as the near-term visual layer for a dependency-
  free, `file://`-compatible workbench.
- The expected future need for a Rust-backed geometry/physics/simulation authority layer
  beneath the workbench ("where our Rust meets our HTML and JS").
- The desire to represent "Twin Architecture" — both digital twinning of the physical
  environment and the meshing of 3D modeling with web modeling into one system.

This ADR formalizes the cross-cutting architectural direction: **Forge is the base pattern
for a universal parametric authoring workbench, not a product tied to a single asset domain.**

### Forces

- **Scope creep risk.** Without a clear decision, each domain (game assets, architecture,
  materials, routines) could independently reinvent the workbench shell, fragmenting the
  platform.
- **Dependency constraint.** FERROS must remain zero-dependency for offline/`file://`
  surfaces. No downloaded libraries, no CDN, no Node build pipeline for HTML surfaces
  (`docs/AGENT_GUIDE.md` §2).
- **Visual truthfulness.** Stakeholders and architects need a reviewable visual artifact
  *now*. The Rust geometry/physics layer does not exist yet and must not block authoring UX.
- **Contract durability.** The visual layer will evolve (illusions → WebGL → Rust-native
  renderer). The data contracts must not change with it.
- **Consent-first principle.** Any AI/assistant edit path requires an explicit user approval
  gate before applying changes — a non-negotiable platform invariant.

---

## Decision

### 1. Forge is the universal parametric authoring workbench base pattern

The Forge shell — four-corner docking layout (ADR-009), iframe viewport seam, runtime-host
postMessage contract (C8), Bag/Inspector/Tools/Assistant panels, manifest-driven catalog
indexing — is the **canonical workbench pattern** for all FERROS authoring domains. It is
not limited to game/loot assets.

Domain-specific labs (Architecture Builder, materials editor, routine composer, or any
future domain) are **specializations** of the same shell and contract model, not independent
systems.

| Component | Universal (Forge base) | Domain specialization |
|-----------|----------------------|----------------------|
| Shell layout | Four-corner docking (ADR-009) | Corner panel labels and purposes vary by domain |
| Catalog / Bag | Card/Deck/Bag model (ADR-010) | Card `kind` values vary (e.g., `arch-part`, `arch-assembly`) |
| Viewport | iframe + runtime-host postMessage (C8) | Viewer HTML target varies per domain |
| Inspector | Schema-driven parameter fields | Field names/types are domain-specific |
| Manifest format | JSON with `id`, `kind`, `name`, `icon`, slots, instances, transforms | Domain extends the schema without breaking the base |
| Assistant gate | Consent-first prompt → Allow/Deny before applying | Prompt copy is domain-specific |

### 2. HTML/CSS/JS visual illusions are the accepted near-term rendering strategy

For prototyping, review, and authoring UX, CSS 3D transforms, perspective projection, and
layered HTML/CSS panel composition are the **accepted rendering technique** for the current
phase. This applies equally to the existing loot-box Card assets and to future architectural
part renderers (parametric walls, windows, slabs).

**Rationale:**
- Zero-dependency: works on `file://` day one.
- Parametric: CSS custom properties map directly to inspector fields (length, height,
  material, opening count, etc.) — the same pattern the Forge loot-box cards use today.
- Reviewable: HTML/CSS artifacts open in any browser with no install, making them effective
  stakeholder review documents.
- Schema-proving: the visual illusion is scaffolding that makes the data model tangible;
  it validates the manifest/schema/transform design before any renderer exists.

**Explicit boundary:** HTML/CSS/JS illusions are **not** the final geometry or physics truth
layer. They are the authoring and review shell. Accuracy claims (structural loads, clash
detection, photorealistic rendering, large-scale BIM import) are explicitly deferred to the
Rust-backed authority layer (see §3 below).

### 3. Future Rust-native geometry / physics / simulation authority layer is expected

The workbench shell is designed to be **renderer-agnostic at the viewport seam**. The
iframe/postMessage contract (C8) means the viewport viewer can be replaced — from an
HTML/CSS illusion to a WebGL canvas to a Rust-compiled WASM renderer to a full native OS
surface — without changing the workbench shell, the inspector, or the manifest schema.

The **singularity of FERROS Rust meeting the HTML/JS surface** is explicitly an open
research question assigned to the Core OS track (separate from product waves). When the
Rust geometry/physics layer matures:

- It will implement the same C8 runtime-host postMessage protocol, or a successor contract
  that extends it without breaking existing viewers.
- Manifests, schemas, slots, instances, and transforms authored in the HTML-first workbench
  phase will remain valid inputs — the data contract is the durable layer.
- The workbench shell (HTML/JS) will continue to host the authoring UX; the Rust layer
  provides physics truth *beneath* the seam, not around it.

This is consistent with ADR-014's three-layer decomposition: `ferros-core/` (domain logic
and contracts) sits below `ferros-surfaces/` (HTML workbench), and the Rust geometry layer
will be a further implementation of `ferros-core/` contracts, not a replacement of the
surface layer.

### 4. Manifests, schemas, transforms, slots, instances, and runtime-host contracts are the durable boundary

The following artifacts are the **stable contract surface** across workbench domains and
across rendering generations:

| Artifact | Role |
|----------|------|
| JSON manifest | Describes a Card or Deck: `id`, `kind`, `name`, `icon`, `components[]`, `slots[]`, `instances[]` |
| JSON schema | Validates the manifest shape (e.g., `card.schema.json`, `deck.schema.json`, future `arch-part.schema.json`) |
| Transform objects | `{ tx, ty, tz, rx, ry, rz, scale }` — domain-independent positioning |
| Slot/instance model | Named slots on a parent Card; instances reference a `partId` + transform, enabling reuse |
| Runtime-host contract (C8) | `ferros:init` / `ferros:update` / `ferros:event` / `ferros:resize` postMessage protocol |

Visual layers, rendering engines, and domain labels are implementation details. The manifest
schema and runtime-host protocol are not.

### 5. Domain-specific labs specialize the shell — Architecture Builder / Builder Blueprint / Twin Architecture Lab as the first example

The Architecture Design Lab (`docs/builder-blueprint.md`, planned surface:
`docs/builder-blueprint-architecture-lab.html`) is the **first live example** of a
domain-specialized workbench. It:

- Uses the same four-corner shell as Forge (per ADR-009).
- Extends the Card/Deck/Bag model (per ADR-010) with `kind: "arch-part"` and
  `kind: "arch-assembly"` Card types.
- Introduces domain-specific inspector fields (wall length/height/thickness/material,
  window width/sill-height/mullion, door swing/jamb-depth, etc.) in the same inspector
  section pattern the Forge uses today.
- Hosts a **Twin Architecture** concept in its Assistant panel:
  - *Traditional digital twin*: digital representation of a physical environment, kept in
    sync with the physical state.
  - *FERROS twin*: the meshing of 3D modeling and web modeling into one unified authoring
    workflow — the same parametric manifest that drives the HTML illusion viewer also drives
    the eventual Rust geometry engine.
- Uses HTML/CSS illusion viewers for individual parts (parametric wall, window, door, slab,
  roof module) and assembly scenes (micro home, rowhouse, cabin).

### 6. Consent-first principle is preserved for all assistant / AI edit paths

All workbench variants — including the Architecture Design Lab — must implement the
consent-first gate before any AI/assistant-proposed edit is applied to a manifest or schema.
The gate pattern is:

1. User describes intent or assistant proposes a change.
2. A **proposal** is rendered in the assistant panel (not yet applied).
3. User must click **Allow** before the change is written to any manifest, slot, or
   inspector field.
4. Denial leaves the workbench state unchanged and records the refusal in the audit trail
   (C7, audit record schema).

This applies equally to Forge (loot-box domain) and all domain specializations.

---

## Consequences

### Positive

- A single workbench shell architecture services any number of authoring domains without
  structural divergence. New labs can be bootstrapped by copying the shell and replacing
  catalog fixtures and inspector field definitions.
- The iframe/postMessage seam allows the rendering layer to evolve independently — from CSS
  illusions to WASM renderer — without touching the workbench shell or contract schema.
- Manifests authored today (loot-box Cards, architecture parts, assemblies) remain valid
  inputs to the future Rust geometry layer because the data contract is designed first.
- Stakeholders (architects, game designers) can review parametric artifacts in any browser
  from `file://` without install, from day one.
- The consent-first gate is applied consistently across all agent-assisted edit flows,
  regardless of domain.

### Negative

- Illusion-based viewers do not satisfy accuracy requirements for structural engineering,
  BIM export, or physics simulation. This is a deliberate non-goal for the current phase
  but must be communicated clearly to stakeholders to prevent expectation mismatch.
- Extending the workbench shell to a new domain requires understanding the existing shell
  pattern, the C8 runtime-host contract, and the ADR-010 Card schema — a learning cost for
  new contributors.
- The Rust geometry/physics layer is an unresolved research question. Until it exists, the
  "twin" in Twin Architecture is an HTML/CSS approximation, not a physics-accurate model.

### Activation condition

This ADR is Accepted immediately. Domain-specialized workbenches that follow the pattern
defined here (shell, contracts, consent gate) are conformant with FERROS architecture.
Domain-specialized workbenches that deviate (bypass the C8 seam, skip the consent gate,
introduce external dependencies in the offline surface) require a new ADR with explicit
justification.

---

## Non-Goals

- This ADR does not define the Rust geometry/physics/simulation engine. That work belongs
  to the Core OS research track (R1–R3 in `docs/progress/PROGRESS.md`).
- This ADR does not commit to a specific timeline or wave gate for the Architecture Design
  Lab. It is a Wave 1 Exploration item per `docs/progress/PROGRESS.md`.
- This ADR does not prescribe specific inspector field schemas for the Architecture Builder
  domain. Those are defined in domain-specific schema files (future `arch-part.schema.json`,
  `arch-assembly.schema.json`).
- This ADR does not change the loot-box / game-asset Forge workflow. The existing Forge at
  40% continues its milestone path as defined in `docs/progress/forge.md`.
- This ADR does not endorse downloading or bundling any external 3D library (Three.js,
  Babylon.js, Cannon.js, etc.) as a dependency of any FERROS offline surface. The
  zero-dependency constraint from `docs/AGENT_GUIDE.md` §2 remains in force.

---

## Alternatives Considered

### Alt A: Build the Architecture Lab as a completely separate system
Rejected. Without a shared shell contract, the Architecture Lab would duplicate layout
logic (ADR-009), catalog patterns (ADR-010), and runtime-host contracts (C8). Platform
fragmentation with no architectural gain.

### Alt B: Wait for Rust geometry layer before building any architecture workbench
Rejected. The Rust geometry layer is a research-track item with no committed timeline.
Blocking stakeholder review and schema design on an unscheduled research item is the
pattern that stalled predecessor repos. HTML/CSS illusions are sufficient for the current
authoring and review goals.

### Alt C: Adopt an existing open-source BIM/3D library (Three.js, IFC.js)
Rejected for the offline/`file://` surface. The zero-dependency constraint means no
downloaded packages. Inline/self-hosted inclusion of a minified 3D library is technically
possible but creates a maintenance and license burden inconsistent with the FERROS
dependency posture. The Rust geometry layer is the correct home for this capability when
it arrives.

### Alt D: Use WebGL directly without a framework
Technically compatible with `file://` and zero external dependencies. Deferred, not
rejected. Raw WebGL is significantly more implementation effort than CSS illusions for the
prototype-and-review phase goal. When CSS illusions hit their accuracy ceiling and the
Rust layer is not yet ready, raw WebGL (inline, no library) is the next step on the near-
term rendering path. This decision should be revisited at the 70% Forge milestone gate.

---

## Follow-Up Work

| Item | Owner track | Wave / Priority |
|------|-------------|-----------------|
| Create `docs/builder-blueprint-architecture-lab.html` — first domain-specialized workbench using this ADR's shell pattern | Stream C (Creative Pipeline) | Wave 1 Exploration |
| Define `arch-part.schema.json` and `arch-assembly.schema.json` in `schemas/` | Stream C | Wave 1 |
| Add Architecture Builder Lab to harness coverage once schemas exist | Stream A (Governance) | Wave 1 / Wave 2 |
| Research: enumerate candidate Rust geometry crates (no download; capability survey only) | Core OS research track | Anytime |
| Define C8 successor contract that supports Rust-backed renderer | Core OS research track | Wave 3+ |
| Revisit raw WebGL path at Forge 70% milestone gate | Stream C | Wave 2 |

---

## Related

- [ADR-009](./ADR-009-four-corner-docking-layout.md) — four-corner docking layout, the
  universal workbench shell spatial contract
- [ADR-010](./ADR-010-cards-and-decks-nomenclature.md) — Card/Deck/Bag nomenclature,
  the universal asset model
- [ADR-013](./ADR-013-legacy-integration-strategy.md) — legacy integration strategy;
  sheetgen-rust architectural drawing patterns as a source domain
- [ADR-014](./ADR-014-three-layer-decomposition.md) — three-layer Rust decomposition;
  Rust geometry layer will sit in `ferros-core/` when it lands
- [`docs/progress/forge.md`](../progress/forge.md) — The Forge progress spec; base
  workbench implementation this ADR generalizes
- [`docs/builder-blueprint.md`](../builder-blueprint.md) — Architecture Design Lab
  direction spec; the document that prompted this ADR
- [`docs/contracts/runtime-host-v1.md`](../contracts/runtime-host-v1.md) — C8 runtime
  host contract; the durable seam between workbench shell and viewer
- [`docs/progress/PROGRESS.md`](../progress/PROGRESS.md) — wave/capability tracker;
  Architecture Builder Lab listed as Wave 1 Exploration
