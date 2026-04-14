# FERROS Progress Tracker

This index tracks Phase 0 progress using **binary capability gates** — not percentages. A capability is either done or not done. Progress is measured by how many gates pass, not by subjective estimates.

**Phase 0 Definition:** [PHASE-0-DEFINITION.md](./PHASE-0-DEFINITION.md)  
**Canonical Schemas:** [`schemas/`](../../schemas/)  
**Golden Fixtures:** [`schemas/fixtures/`](../../schemas/fixtures/)  
**Runtime Host Contract:** [`docs/contracts/runtime-host-v1.md`](../contracts/runtime-host-v1.md)

---

## How to Read This File

- Each capability is a **binary gate**: ✅ passes or ⬜ not yet.
- Capabilities are organized into **tiers** (contracts → vertical slice → consumers → permissioned actions → hardening).
- Earlier tiers must pass before later tiers are meaningful.
- Core OS / native rendering is a **separate research track**, not gated by product waves.
- Individual module spec files (linked below) contain per-module milestone detail.
- `README.md` percentages remain for external visibility but are derived from capability gate counts, not estimated independently.

---

## Interconnected System Model

- **Card** — the atomic FERROS object.
- **Deck** — a composed collection of Cards.
- **Bag** — the local catalog or inventory of Cards and Decks.
- **Portal Runtime instance** — a rendered or experienced Deck/Card surface inside the Arena Runtime.
- **Profile-linked object** — any Card, Deck, or runtime state attributed to identity, permissions, rewards, or verification.

---

## Capability Scoreboard

### Tier 1 — Contracts (Wave 0)

Must exist before building surfaces.

| # | Capability | Status | Artifact |
|---|-----------|--------|----------|
| C1 | Identity/session schema versioned with fixtures | ✅ | `schemas/identity.schema.json` |
| C2 | Profile schema versioned with fixtures | ✅ | `schemas/profile.schema.json` |
| C3 | Template schema versioned with fixtures | ✅ | `schemas/template.schema.json` |
| C4 | Card schema versioned with fixtures | ✅ | `schemas/card.schema.json` |
| C5 | Deck schema versioned with fixtures | ✅ | `schemas/deck.schema.json` |
| C6 | Schedule event schema versioned with fixtures | ✅ | `schemas/schedule-event.schema.json` |
| C7 | Audit record schema versioned with fixtures | ✅ | `schemas/audit-record.schema.json` |
| C8 | Runtime host contract v1 (init/update/error/lifecycle) | ✅ | `docs/contracts/runtime-host-v1.md` |
| C9 | Storage rules (what goes where, migration, corruption handling) | ✅ | `docs/contracts/storage-rules.md` |
| C10 | Permission model skeleton (who can do what, consent capture) | ✅ | `docs/contracts/permission-model.md` |

**Wave 0 exit:** C1–C10 complete. Schemas versioned. Fixtures pass validation.

### Tier 2 — Vertical Slice (Wave 1)

One real end-to-end loop works locally.

| # | Capability | Status | Proving |
|---|-----------|--------|---------|
| V1 | Template → Profile creation completes without soft-lock | ⬜ | Journey 1 |
| V2 | Export → Import produces identical profile in fresh browser | ⬜ | Journey 1 |
| V3a | All 4 valid session modes complete expected flows (H2 round-trip) | ⬜ | Journey 1 + 2 |
| V3b | Invalid session mode combinations rejected with correct error codes (H4 negative) | ⬜ | Journey 1 + 2 |
| V4 | Alias session → export → claim → XP merge works | ⬜ | Journey 2 |
| V5 | Card loads in Forge → editable → renders in Runtime | ⬜ | Journey 3 |
| V6 | Runtime init/update/event loop completes | ⬜ | Journey 3 |
| V7 | Card round-trip export/import preserves all parameters | ⬜ | Journey 3 |
| V8 | Phase 0 acceptance harness (H5) proves Journey 1 end-to-end: create→export→clear→import→render→assert | ⬜ | All journeys |

**Wave 1 exit:** V1–V8 complete (V3 split into V3a + V3b). All 3 user journeys work end-to-end. H5 acceptance harness passes.

### Tier 3 — First Consumers (Wave 2)

Shared contracts serve more than one surface.

| # | Capability | Status | Surface |
|---|-----------|--------|---------|
| S1 | Schedule Ledger reads profile/template data via shared contract | ⬜ | Schedule Ledger |
| S2 | Battle Arena consumes Arena Runtime without custom data paths | ⬜ | Battle Arena |
| S3 | Showcase reads real capability status, not illustrative placeholders | ⬜ | Showcase |
| S4 | Shared contracts frozen at v1 for local-first use | ⬜ | All |

**Wave 2 exit:** S1–S4 complete. ≥3 surfaces consume shared contracts. Contracts frozen at v1.

### Tier 4 — Permissioned Actions (Wave 3)

| # | Capability | Status | Surface |
|---|-----------|--------|---------|
| P1 | One agent-triggered action flow with permission prompt + audit trail | ⬜ | Agent Integration |
| P2 | Identity/consent enforced across ≥2 surfaces | ⬜ | Cross-surface |
| P3 | Action contract versioned and tested | ⬜ | Agent Command Center |

**Wave 3 exit:** P1–P3 complete. One agent action flow works with audit trail.

### Tier 5 — Hardening (Wave 4)

| # | Capability | Status | Gate |
|---|-----------|--------|------|
| H1 | Schema migration rules implemented and tested | ⬜ | Storage |
| H2 | Cross-browser support matrix validated | ⬜ | Platform |
| H3 | Import/export corruption handling tested | ⬜ | Portability |
| H4 | Accessibility baseline checked | ⬜ | A11y |
| H5 | Performance budgets defined and met for core flows | ⬜ | Perf |
| H6 | Public docs match shipped behavior | ⬜ | Docs |

**Wave 4 exit:** H1–H6 complete. Platform is trustworthy for external users.

### Separate Track — Core OS & Native Rendering

| # | Capability | Status | Gate |
|---|-----------|--------|------|
| R1 | Renderer conformance suite exists with golden fixtures | ⬜ | Conformance |
| R2 | QEMU bring-up path proven with minimal reference target | ⬜ | Boot |
| R3 | Native rendering mapped against same fixture corpus as web surfaces | ⬜ | Parity |

---

## Wave Structure

| Wave | Goal | Entry | Exit |
|------|------|-------|------|
| **0** | Contract spine | Now | C1–C10 pass |
| **1** | Smallest local loop | C1–C10 pass | V1–V8 pass |
| **2** | First consumers | V1–V8 pass | S1–S4 pass |
| **3** | Permissioned actions | S1–S4 pass | P1–P3 pass |
| **4** | Hardening | P1–P3 pass | H1–H6 pass |
| **Research** | Core OS (parallel track) | Anytime | R1–R3 pass |

---

## Module Spec Files

Individual module specs retain their detailed milestones and dependency lists. They serve as implementation guides for each surface, not as progress percentages.

| Name | Phase | Wave Role | Spec Link |
|------|-------|-----------|-----------|
| FERROS Core OS | Research track | Separate — not gated by product waves | [ferros-core-os.md](./ferros-core-os.md) |
| Founding Blueprint | Research track | Separate — conformance target for native renderer | [blueprint.md](./blueprint.md) |
| Personal Profile | Wave 1 (vertical slice) | Core identity and onboarding surface | [personal-profile.md](./personal-profile.md) |
| User / Identity System | Wave 1 (vertical slice) | Extract shared identity contract from prototype | [user-identity-system.md](./user-identity-system.md) |
| Templates & Profiles | Wave 1 (vertical slice) | Schema baseline and template validation | [templates-and-profiles.md](./templates-and-profiles.md) |
| Assets, Cards & Decks | Wave 1 (vertical slice) | Card/Deck object model and Forge round-trip | [assets-cards-decks.md](./assets-cards-decks.md) |
| Arena Runtime | Wave 1 (vertical slice) | Separated runtime host layer | [arena-runtime.md](./arena-runtime.md) |
| The Forge | Wave 1 (vertical slice) | Authoring surface for the card round-trip | [forge.md](./forge.md) |
| Schedule Ledger | Wave 2 (first consumer) | Proves shared contract consumption | [schedule-ledger.md](./schedule-ledger.md) |
| Battle Arena | Wave 2 (first consumer) | Proves runtime can serve a second surface | [trading-arena.md](./trading-arena.md) |
| Showcase / Landing Page | Wave 2 (first consumer) | Reads real capability status | [showcase.md](./showcase.md) |
| Agent Integration | Wave 3 (permissioned actions) | Blocked until identity contract passes | [agent-integration.md](./agent-integration.md) |
| Agent Command Center | Wave 3 (permissioned actions) | Agent dispatch and audit surface | [agent-command-center.md](./agent-command-center.md) |
| Home HUD Dashboard | Wave 3+ (deferred) | Device control requires later infrastructure | [home-hud.md](./home-hud.md) |