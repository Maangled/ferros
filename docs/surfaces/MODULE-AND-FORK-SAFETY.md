# Module and Fork Safety

This note defines the current safety posture for shared UX modules and surface forks.

It is an architecture note, not a ratified ADR. Its job is to make the current operator-UX scaffolding safe to build against while the next wave of implementation lands.

See [../adr/ADR-008-modular-rendering-system.md](../adr/ADR-008-modular-rendering-system.md), [../adr/ADR-017-html-surface-incubation-strategy.md](../adr/ADR-017-html-surface-incubation-strategy.md), and [../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md](../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md).

---

## Layer model

1. **Base Shell Module**
   - Owns shell chrome, route anchors, status rail, accessibility posture, focus rules, and shared layout grammar.
   - Protected: changes here affect every surface using the shared shell grammar.

2. **Shared Utility Modules**
   - Own reusable pieces consumed by two or more surfaces.
   - Examples: `ProposedMaterialCard`, `ReceiptStrip`, `EvidenceBadge`, `SourceLineageCard`, `ToolLaneCard`, `OperatorStepCard`.

3. **Surface Modules**
   - Own standalone surfaces such as Profile, ACC, Forge, Arena, Home-Hub, and future variants.
   - Surface modules may fork more freely as long as they respect the protected boundaries below.

4. **Data Adapter Modules**
   - Own the translation layer between UI modules and backend seams such as `/rpc`, `/profile`, `/runway-summary(.json)`, local artifacts, and future module manifests.

5. **Capability and Consent Gate Modules**
   - Own explicit arm/confirm logic, blocked-state language, local-only disclosure, and ADR-023 staged-state boundaries.

6. **Harness Modules**
   - Own validation of base stability, adapter boundaries, touch posture, and consent invariants.

---

## Protected boundaries

The following boundaries are not available for casual surface forking:

1. Frozen schemas under `schemas/`
2. Local-only transport and current localhost shell posture
3. ADR-023 quarantine-before-canonical invariant
4. No hidden grant or revoke through the browser surface
5. No automatic sync from external systems into profile, grants, or progression
6. No unsupported evidence or security claims

If a fork needs to cross one of these boundaries, it should become a named ADR, contract change, or system-lane queue item instead of an incidental surface tweak.

---

## Safe fork points

Safe fork points today:

1. Surface-specific layout, panel order, and visual treatment
2. Optional visual motifs translated through FERROS-safe language
3. Additional local-only display components that do not introduce new write paths
4. Alternative Home-Hub, Forge, or Arena presentation layers that remain honest about blocked or staged state

Unsafe fork points today:

1. Browser grant/revoke mutation
2. Remote transport claims
3. Canonical mutation before explicit accept
4. Tool lanes with implicit read/write authority
5. Evidence badges without proof mapping

---

## Review requirements

| Change type | Minimum review expectation |
|-------------|----------------------------|
| Base Shell Module change | One implementation review plus one safety review; rerun shell harnesses |
| Shared Utility Module used by 2+ surfaces | One implementation review plus cross-surface sanity check |
| Surface-only fork | One owner review for the surface and one quick check against protected boundaries |
| Data Adapter or gate-module change | One implementation review plus consent/boundary validation |
| Badge, receipt, or lineage language change | One docs/safety review to prevent unsupported claims |

---

## Current fork rule

Until a fuller module ADR lands, surface forks are allowed only when they:

1. preserve the protected boundaries above,
2. remain explicit about blocked, staged, runway, or non-evidentiary state,
3. do not silently widen browser privilege,
4. keep the discovery-note quarantine intact.

---

## Relationship to the discovery note

The discovery note may influence shell grammar and visual motifs, but it does not grant permission to fork protected boundaries.

Useful motifs may be translated into FERROS-safe modules.

Quarantined ideas remain outside the module system until separate ADR review occurs.
