# Phase 0 Definition

**Version:** 1.0  
**Status:** Active  
**Last updated:** 2026-04-11

Phase 0 is complete when FERROS has a working local-first personal computing platform that proves the core contract loop: identity → authoring → rendering → persistence → portability. This document defines the exit criteria.

---

## Primary User Journeys

Phase 0 success requires exactly three end-to-end journeys to work without bugs, without servers, and on `file://` protocol.

### Journey 1 — New User Creates and Owns a Profile

**Precondition:** No `ferros_profile` in localStorage. Fresh browser.

1. User opens `personal-profile.html`. Boot animation plays. Trade Window appears.
2. User accepts the Trade Window (grants localStorage consent).
3. User selects an assistance level.
4. User browses the template gallery and picks a template (e.g., Tesla, Curie).
5. User enters the character creation form: name, avatar, class, stream affinity, wake/sleep times, interests.
6. Stage 1 completes. Genesis seal is created (SHA-256 or djb2 fallback).
7. User completes the Stage 2 protocol checklist. Second seal is created. XP is awarded.
8. Stage 3 dashboard loads. Profile is fully functional with attributes, journal, achievements, and skill trees.
9. User exports their profile as a `.json` file via the Portability panel.
10. User opens a different browser (or clears storage). Imports the `.json`. Profile loads identically: same name, same seal chain, same deck references, same schedule.

**Exit gate (binary):** Exported profile imported in a fresh browser produces identical genesis hash, seal chain length, attribute totals, and — once implemented — deck IDs and schedule references.

### Journey 2 — Alias Session on a Foreign Device

**Precondition:** User has a profile at home. They are on a friend's computer.

1. User opens `personal-profile.html`. Trade Window appears.
2. User chooses "Explore as Alias" from the template gallery.
3. User selects a template identity (e.g., Fry). Alias session starts in `sessionStorage`.
4. User logs activities (journal entries, habit checks). All logs accumulate in the local alias session. **Nothing touches localStorage.**
5. User ends the alias session. A `.ferros-log` file downloads with per-entry seals.
6. User closes the tab. `sessionStorage` is cleared. No trace on the foreign device.
7. At home, user opens their real profile. Navigates to Portability → Claim.
8. User imports the `.ferros-log` file. Seal chain is verified. XP is merged into the real profile.

**Exit gate (binary):** Alias log imported on home device increments the correct attributes by the expected XP. Seal chain of the imported log validates against its own internal chain. `linkedTo` field updates from `null` to the real profile's genesis hash. No `localStorage` writes occur on the foreign device during the entire alias session.

### Journey 3 — Author, Render, Edit, Persist a Card

**Precondition:** User has a profile. Forge workbench and Arena Runtime are functional.

1. User opens the Forge workbench. The asset catalog lists available cards and projects.
2. User selects a card (e.g., a loot box part) from the catalog. It loads in the viewport.
3. User edits card parameters (transform, state preset, metadata) in the Inspector panel.
4. User saves the card locally (export or localStorage).
5. User opens the Arena Runtime. The runtime receives the card via `ferros:init` message.
6. The card renders in the runtime viewport with the same parameters the user set in the Forge.
7. The runtime sends `ferros:update` to change the card's state (e.g., open/close a loot box). The card responds.
8. User exports the card state. Reimporting it in Forge or Runtime produces the same visual result.

**Exit gate (binary):** A card edited in Forge and loaded in Runtime renders with identical transform, state, and visual output. A round-trip export/import cycle preserves all parameters. The `ferros:init` → `ferros:update` → `ferros:event` message loop completes without errors.

---

## Explicit Non-Goals for Phase 0

These are **not** Phase 0 deliverables. They are intentionally deferred.

| Non-Goal | Reason | Earliest Phase |
|----------|--------|----------------|
| Native OS boot / Rust kernel | Separate research track | Phase 1+ |
| Native HTML/CSS renderer | Requires kernel; separate track | Phase 1+ |
| Smart contract deployment or wallet integration | No local-first need; ADR-002 defines boundaries | Phase 4 |
| Backend server or API | `file://` protocol is the deployment target | Phase 1+ |
| Real device control (cameras, sensors, IoT) | Requires hardware integration layer | Phase 3 |
| Discord bot migration | Agent hosting is Phase 6 | Phase 6 |
| Crypto-first backend bridge | High risk; follows proven local loop | Phase 4+ |
| Multi-user sync or federation | Requires distributed layer | Phase 5 |
| Public showcase / marketing site | Not critical path; follows from real capabilities | After Wave 2 |
| Cross-browser hardening | After core loop is proven | Wave 4 |
| Mobile / responsive completeness | After core loop is proven | Wave 4 |
| CI / automated test infrastructure | Follows first manual e2e pass | Wave 1 exit |

---

## Phase 0 Capability Scoreboard

Replace percentage tracking with binary capability gates.

### Tier 1 — Contracts (must exist before building surfaces)

| # | Capability | Status | Artifact |
|---|-----------|--------|----------|
| C1 | Identity/session schema versioned with fixtures | Not started | `schemas/identity.schema.json` |
| C2 | Profile schema versioned with fixtures | Not started | `schemas/profile.schema.json` |
| C3 | Template schema versioned with fixtures | Not started | `schemas/template.schema.json` |
| C4 | Card schema versioned with fixtures | Not started | `schemas/card.schema.json` |
| C5 | Deck schema versioned with fixtures | Not started | `schemas/deck.schema.json` |
| C6 | Schedule event schema versioned with fixtures | Not started | `schemas/schedule-event.schema.json` |
| C7 | Audit record schema versioned with fixtures | Not started | `schemas/audit-record.schema.json` |
| C8 | Runtime host contract v1 (init/update/error/lifecycle) | Not started | `docs/contracts/runtime-host-v1.md` |
| C9 | Storage rules (what goes where, migration, corruption handling) | Not started | `docs/contracts/storage-rules.md` |
| C10 | Permission model skeleton (who can do what, consent capture) | Not started | `docs/contracts/permission-model.md` |

### Tier 2 — Vertical Slice (one real loop end-to-end)

| # | Capability | Status | Proving Journey |
|---|-----------|--------|-----------------|
| V1 | Template → Profile creation completes without soft-lock | Not started | J1 steps 1–8 |
| V2 | Export → Import produces identical profile in fresh browser | Not started | J1 steps 9–10 |
| V3 | All 4 session modes work with deterministic fixtures | Not started | J1, J2 |
| V4 | Alias session → export → claim → XP merge works | Not started | J2 |
| V5 | Card loads in Forge → editable → renders in Runtime | Not started | J3 steps 1–6 |
| V6 | Runtime init/update/event loop completes | Not started | J3 steps 5–7 |
| V7 | Card round-trip export/import preserves all parameters | Not started | J3 step 8 |
| V8 | One browser e2e test suite passes for the vertical slice | Not started | All 3 journeys |

### Tier 3 — First Consumers (shared contracts serve >1 surface)

| # | Capability | Status | Surface |
|---|-----------|--------|---------|
| S1 | Schedule Ledger reads profile/template data via shared contract | Not started | Schedule Ledger |
| S2 | Battle Arena consumes Arena Runtime without custom data paths | Not started | Battle Arena |
| S3 | Showcase reads real capability status, not illustrative placeholders | Not started | Showcase |
| S4 | Shared contracts frozen at v1 for local-first use | Not started | All |

### Tier 4 — Permissioned Actions

| # | Capability | Status | Surface |
|---|-----------|--------|---------|
| P1 | One agent-triggered action flow with permission prompt + audit trail | Not started | Agent Integration |
| P2 | Identity/consent enforced across ≥2 surfaces | Not started | Cross-surface |
| P3 | Action contract versioned and tested | Not started | Agent Command Center |

### Tier 5 — Hardening

| # | Capability | Status | Gate |
|---|-----------|--------|------|
| H1 | Schema migration rules implemented and tested | Not started | Storage |
| H2 | Cross-browser support matrix validated | Not started | Platform |
| H3 | Import/export corruption handling tested | Not started | Portability |
| H4 | Accessibility baseline checked | Not started | A11y |
| H5 | Performance budgets defined and met for core flows | Not started | Perf |
| H6 | Public docs match shipped behavior | Not started | Docs |

### Separate Track — Core OS & Native Rendering

| # | Capability | Status | Gate |
|---|-----------|--------|------|
| R1 | Renderer conformance suite exists with golden fixtures | Not started | Conformance |
| R2 | QEMU bring-up path proven with minimal reference target | Not started | Boot |
| R3 | Native rendering mapped against same fixture corpus as web surfaces | Not started | Parity |

---

## Wave Structure

| Wave | Goal | Exit Criteria |
|------|------|---------------|
| **0** | Contract spine | C1–C10 complete. Schemas versioned. Fixtures pass validation. |
| **1** | Smallest local-first loop | V1–V8 complete. Journey 1, 2, and 3 work end-to-end. One e2e test passes. |
| **2** | First true consumers | S1–S4 complete. ≥3 surfaces consume shared contracts. Contracts frozen at v1. |
| **3** | Permissioned actions | P1–P3 complete. Agent flow works with audit trail. |
| **4** | Hardening | H1–H6 complete. Platform is trustworthy for external users. |
| **Research** | Core OS (parallel) | R1–R3 complete. Not gated by product waves. |

---

## Related ADRs

All Phase 0 work is bound by:
- ADR-001 (Progression-Lock Pattern) — seal chain integrity, `file://` fallback
- ADR-002 (Smart Contract Boundaries) — 4 approved on-chain use cases only; none required for Phase 0
- ADR-003 (Alias System) — bearer identity, sessionStorage constraint
- ADR-004 (Template Profile Specification) — 8+ hardcoded templates, schema requirements
- ADR-005 (Cross-Device Identity) — 4 mutually exclusive session modes
- ADR-006 (Level Zero Onboarding) — boot sequence, adaptive difficulty
- ADR-007 (Single File System) — cards not files
- ADR-008 (Modular Rendering System) — 3-layer architecture
- ADR-009 (Four-Corner Docking) — layout contract
- ADR-010 (Cards & Decks Nomenclature) — vocabulary
- ADR-011 (Routine Module System) — module/deck composition
