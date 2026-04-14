# FERROS Progress Tracker

This index tracks Phase 0 progress using **binary capability gates** — not percentages. A capability is either done or not done. Progress is measured by how many gates pass, not by subjective estimates.

**Phase 0 Definition:** [PHASE-0-DEFINITION.md](./PHASE-0-DEFINITION.md)  
**Canonical Schemas:** [`schemas/`](../../schemas/)  
**Golden Fixtures:** [`schemas/fixtures/`](../../schemas/fixtures/)  
**Runtime Host Contract:** [`docs/contracts/runtime-host-v1.md`](../contracts/runtime-host-v1.md)

---

## How to Read This File

- Each capability has two status dimensions: **Artifact** (file/spec exists) and **Enforcement** (a harness runs tests against it and passes). A capability is not complete until both ✅.
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

| # | Capability | Artifact | Enforcement | Proof Harness |
|---|-----------|----------|-------------|---------------|
| C1 | Identity/session schema versioned with fixtures | ✅ | 🔧 built, pending run | `harnesses/ferros-contract-validator.html` |
| C2 | Profile schema versioned with fixtures | ✅ | 🔧 built, pending run | `harnesses/ferros-contract-validator.html` |
| C3 | Template schema versioned with fixtures | ✅ | 🔧 built, pending run | `harnesses/ferros-contract-validator.html` |
| C4 | Card schema versioned with fixtures | ✅ | 🔧 built, pending run | `harnesses/ferros-contract-validator.html` |
| C5 | Deck schema versioned with fixtures | ✅ | 🔧 built, pending run | `harnesses/ferros-contract-validator.html` |
| C6 | Schedule event schema versioned with fixtures | ✅ | 🔧 built, pending run | `harnesses/ferros-contract-validator.html` |
| C7 | Audit record schema versioned with fixtures | ✅ | 🔧 built, pending run | `harnesses/ferros-contract-validator.html` |
| C8 | Runtime host contract v1 (init/update/error/lifecycle) | ✅ | 🔧 built, pending run | `harnesses/runtime-harness.html` |
| C9 | Storage rules (what goes where, migration, corruption handling) | ✅ | 🔧 built, pending run | `harnesses/round-trip-harness.html` |
| C10 | Permission model skeleton (who can do what, consent capture) | ✅ | 🔧 built, pending run | `harnesses/negative-harness.html` |

**Legend:** ✅ confirmed pass &nbsp;|&nbsp; 🔧 harness built — open in Chrome to confirm &nbsp;|&nbsp; ⬜ not yet

**Wave 0 exit:** C1–C10 artifact ✅ AND enforcement ✅. Schemas versioned. Fixtures pass H1. Storage rules enforced by H2. Permission model enforced by H4.

### Tier 2 — Vertical Slice (Wave 1)

One real end-to-end loop works locally.

| # | Capability | Status | Proving |
|---|-----------|--------|---------|
| V1 | Template → Profile creation completes without soft-lock | 🔧 built, pending run | Journey 1 |
| V2 | Export → Import produces identical profile in fresh browser | ⬜ | Journey 1 |
| V3a | All 4 valid session modes complete expected flows (H2 round-trip) | 🔧 built, pending run | Journey 1 + 2 |
| V3b | Invalid session mode combinations rejected with correct error codes (H4 negative) | 🔧 built, pending run | Journey 1 + 2 |
| V4 | Alias session → export → claim → XP merge works | ⬜ | Journey 2 |
| V5 | Card loads in Forge → editable → renders in Runtime | ⬜ | Journey 3 |
| V6 | Runtime init/update/event loop completes | ⬜ | Journey 3 |
| V7 | Card round-trip export/import preserves all parameters | ⬜ | Journey 3 |
| V8 | Phase 0 acceptance harness (H5) proves Journey 1 end-to-end: create→export→clear→import→render→assert | 🔧 built, pending run | All journeys |

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

## Audit Remediation — Finding Classification

Twenty findings from the Phase 0 exit audit. Each is classified as **resolved** (implemented + tested), **hardened** (enforcement added, full verification deferred), or **documented-and-deferred** (contract/doc positioning only, no runtime change required this wave).

| # | Finding | Category | Step | Artifacts |
|---|---------|----------|------|-----------|
| 1 | Write-path guard missing on non-profile modes | ✅ Resolved | A2 | `canMutateDurableState()` in monolith + `FerrosCore` |
| 2 | Schema version mismatch (1.0 vs 2.0) | ✅ Resolved | A1 | `profile.schema.json`, `storage-rules.md` |
| 3 | Seal metadata not persisted (hashAlgorithm, nonce) | ✅ Resolved | A3 | `computeHash` → `{hash, algorithm}`, all fixtures |
| 4 | Export reads in-memory, not persisted state | ✅ Resolved | A4 | `exportProfile()` reads `localStorage` |
| 5 | Claim/session identity not in log envelope | 🔧 Hardened | C9 | `storage-rules.md` sessionId semantics, `permission-model.md` claim uniqueness |
| 6 | Origin validation missing from C8 | 🔧 Hardened | C8 | `runtime-host-v1.md` Section 9 |
| 7 | Templates inlined, not validated against schema | ✅ Resolved | B7 | `templates.json`, `generate-ferros-core.ps1` |
| 8 | Schedule-event schema not consumed at runtime | ⬜ Deferred → Wave 2 (S1) | C10 | Note in `storage-rules.md` |
| 9 | Card/deck not included in export | ⬜ Deferred → Wave 1 (V5-V7) | C11 | Note in `storage-rules.md` |
| 10 | Audit record retention unbounded | 🔧 Hardened | C12 | `permission-model.md` ring-buffer rule |
| 11 | `meta.version` / `schemaVersion` dual-field confusion | ✅ Resolved | A1 | `migrateProfileStructure()` rename, single canonical field |
| 12 | localStorage quota not handled | ✅ Resolved | A5 | `saveProfile()` try/catch + user modal |
| 13 | Seal chain compaction not addressed | ⬜ Deferred → Wave 4 (H5) | C13 | Note in `storage-rules.md` |
| 14 | Fixture corpus too narrow | ✅ Resolved | D14 | 5 new fixtures, `_constants.js` regenerated |
| 15 | No shared runtime core | ✅ Resolved | B6 | `ferros-core.js` IIFE, `window.FerrosCore` |
| 16 | Monolith duplicates contract logic | ✅ Resolved | B6 | Monolith delegates to `FerrosCore.*` |
| 17 | No black-box UI acceptance harness | ✅ Resolved | D15 | `harnesses/ui-acceptance-harness.html` (H6) |
| 18 | No contract manifest | ✅ Resolved | D16 | `docs/contracts/manifest.json` |
| 19 | Contract/fixture co-location fragmented | ⬜ Deferred → Wave 1 | D16 | Manifest mitigates; physical reorg deferred |
| 20 | Harnesses don't share core verification logic | ✅ Resolved | B6 | `ferros-core.js` wired to monolith |

**Totals:** 13 resolved, 3 hardened, 4 documented-and-deferred.

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