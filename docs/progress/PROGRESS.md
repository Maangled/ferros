# FERROS Progress Tracker

This index tracks Phase 0 progress using **binary capability gates** — not percentages. A capability is either done or not done. Progress is measured by how many gates pass, not by subjective estimates.

**Phase 0 Definition:** [PHASE-0-DEFINITION.md](./PHASE-0-DEFINITION.md)  
**Canonical Schemas:** [`schemas/`](../../schemas/)  
**Golden Fixtures:** [`schemas/fixtures/`](../../schemas/fixtures/)  
**Runtime Host Contract:** [`docs/contracts/runtime-host-v1.md`](../contracts/runtime-host-v1.md)

---

## Wave 0 PR Tracker

| PR | # | Merged | Scope |
|----|---|--------|-------|
| PR 1 — Deterministic generators + named inventory | #41 | ✅ 2026-04-13 | `generate-harness-constants.ps1`, `generate-ferros-core.ps1`, `_constants.js` named arrays |
| PR 2 — Manifest hardening + fixture truth | #42 | ✅ 2026-04-14 | `manifest.json` C1–C10 entries, golden fixture corpus |
| PR 3 — H1 full contract matrix | #43 | ✅ 2026-04-15 | `ferros-contract-validator.html` C1–C7 groups |
| PR 4 — C8-C10 harness gaps | #44 | ✅ 2026-04-17 | H3 nonce handshake, H2 real round-trip, H4 deny probes |
| PR 5 — Supporting harness alignment + shared-core cleanup | #45 | ✅ 2026-04-17 | `trading-card.html` nonce echo, H5–H8 role separation, `personal-profile.html` seams |
| PR 6 — Docs/ADR reconciliation | #46 | ✅ 2026-04-17 | Contract docs, ferros-core-api.md, ADR addenda, PHASE-0-DEFINITION.md, PROGRESS.md |
| PR 7 — Final closure verification | #47 | ✅ 2026-04-17 | Regenerate → diff, `CLOSURE-EVIDENCE.md`, Wave 0 CLOSED |

**Wave 0 status: CLOSED** — Re-verified on 2026-04-18 after H3/runtime contract alignment and deterministic generator hardening. See [CLOSURE-EVIDENCE.md](../../CLOSURE-EVIDENCE.md) for the full closure record. **Next: Streams B and C enter Wave 1 in parallel per the [streams architecture](../streams/STREAMS-OVERVIEW.md).**

---

## Phase A — Foundation Finalization PR Tracker

| PR | Item | Commit / PR # | Status |
|----|------|---------------|--------|
| PR 8  | A2 — H8 clean rerun + environmental notes | Merged [#60](https://github.com/Maangled/ferros/pull/60) | ✅ Merged |
| PR 9  | A1 — H9 consumer-helper harness | Merged [#64](https://github.com/Maangled/ferros/pull/64) | ✅ Merged |
| PR 10 | A4 — V4 alias → claim → XP merge | Landed `main` @ `8d7c123` | ✅ Landed |
| PR 11 | A3 + A5 — Wave 1 closure evidence | This PR | ✅ Active |
| PR 12 | B1 — Arena Export Target ADR | Planned | ⬜ |
| PR 13 | B2 — C6 runtime consumption spec | Planned | ⬜ |

Umbrella: [Phase A — Foundation Finalization #62](https://github.com/Maangled/ferros/issues/62) · Plan: [PR-PLAN-PR8-PR13.md](./PR-PLAN-PR8-PR13.md)

**Wave 1 status: CLOSED** — All V1–V8 capabilities verified 2026-04-22. See [WAVE-1-CLOSURE-EVIDENCE.md](./WAVE-1-CLOSURE-EVIDENCE.md) for the full closure record. Phase B entry begins with PR 12.

---

## Reconciliation Gate — Wave 1

> **Status: CLOSED** — Reviewer sign-off recorded 2026-04-19. Tracking issue: [Reconciliation Gate — Wave 1 #53](https://github.com/Maangled/ferros/issues/53).

Per `docs/ORCHESTRATION.md` §4, a cross-stream reconciliation review must complete before any Wave 1 execution PR merges. The gate document with all 20 reconciliation items is at:

📋 **[`reconciliation-gate-wave1.md`](./reconciliation-gate-wave1.md)**

| Check | Parties | Items | Status |
|-------|---------|-------|--------|
| B vs A schemas | A ↔ B | 6 | ✅ Closed |
| C vs A schemas | A ↔ C | 5 | ✅ Closed |
| D vs B+C artifacts | B, C ↔ D | 5 | ✅ Closed |
| E vs A–D assumptions | E ↔ A–D | 4 | ✅ Closed |

**Result:** Reconciliation review is complete and no longer blocks Wave 1 execution PR merges.

---

## Wave 0 Harness Status

> Run each harness by opening it in Chrome via `file://`. All harnesses load `ferros-core.js` via `<script src="../docs/assets/_core/ferros-core.js">`.

Fresh gate verification on 2026-04-22 in Chrome via `file://`: H1 `30/30`, H2 `25/25`, H4 `26/26`. H3 remains last verified on 2026-04-18 at `18/18`.
Live supporting-harness verification in the current browser session on 2026-04-22: H5 `30/30` PASS, H9 `28/28` PASS. H6 `25/25` PASS, H7 `107/107` PASS, and Preflight `6/6` PASS remain from the 2026-04-19 session. H8 clean rerun completed 2026-04-21 — see `docs/progress/H8-RERUN-ENV-NOTES.md`.

| Harness | File | Contracts | Gate? | Verified status |
|---------|------|-----------|-------|-----------------|
| H1 | `harnesses/ferros-contract-validator.html` | C1–C7 | ✅ Gate | PASS — 30/30 |
| H2 | `harnesses/round-trip-harness.html` | C9 | ✅ Gate | PASS — 25/25 |
| H3 | `harnesses/runtime-harness.html` | C8 | ✅ Gate | PASS — 18/18 (nonce echo green; resize semantics aligned to C8) |
| H4 | `harnesses/negative-harness.html` | C10 | ✅ Gate | PASS — 26/26 |
| H5 | `harnesses/acceptance-harness.html` | V1/V8 | Supporting | PASS — 30/30 |
| H6 | `harnesses/write-path-harness.html` | C9 write | Supporting | PASS — 25/25 |
| H7 | `harnesses/semantic-fixture-linter.html` | C2/C4/C5/C6 | Supporting | PASS — 107/107 |
| H8 | `harnesses/ui-acceptance-harness.html` | C10/UI | Supporting | PASS — 17/17 (clean rerun 2026-04-21; see H8-RERUN-ENV-NOTES.md) |
| H9 | `harnesses/consumer-helper-harness.html` | C7/C9/C10 helpers | Supporting | PASS — 28/28 |
| Preflight | `harnesses/preflight-check.html` | C7/C8 inventory | Supporting | PASS — 6/6 |

---

## Wave 0 Contract Coverage

| Contract | Name | Artifact | Schema | Fixtures | Gate harness | Status |
|---|---|---|---|---|---|---|
| C1 | Identity/Session Schema | ✅ | `schemas/identity.schema.json` | 4 fixtures | H1 | ✅ |
| C2 | Profile Schema | ✅ | `schemas/profile.schema.json` | 9 fixtures | H1, H2 | ✅ |
| C3 | Template Profile | ✅ | `schemas/template.schema.json` | 1 fixture | H1 | ✅ |
| C4 | Card Schema | ✅ | `schemas/card.schema.json` | 2 fixtures | H1 | ✅ |
| C5 | Deck Schema | ✅ | `schemas/deck.schema.json` | 2 fixtures | H1 | ✅ |
| C6 | Schedule Event Schema | ✅ | `schemas/schedule-event.schema.json` | 1 fixture | H1 | ✅ |
| C7 | Audit Record Schema | ✅ | `schemas/audit-record.schema.json` | 4 fixtures | H1 | ✅ |
| C8 | Runtime Host Contract | ✅ | `docs/contracts/runtime-host-v1.md` | none | H3 | ✅ |
| C9 | Storage Rules | ✅ | `docs/contracts/storage-rules.md` | 15 fixtures | H2 | ✅ |
| C10 | Permission Model | ✅ | `docs/contracts/permission-model.md` | none | H4 | ✅ |

**Known gaps / deferred items:**
- C6 schedule-event runtime consumption is deferred to Wave 2 (S1). `FerrosCore.templateToEvents()` provides the transformation bridge but no surface consumes it yet.
- C8 origin validation on `file://` is enforcement-on-conformance only — not a hard security boundary (documented in runtime-host-v1.md §9).

---

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
| C1 | Identity/session schema versioned with fixtures | ✅ | ✅ | `harnesses/ferros-contract-validator.html` |
| C2 | Profile schema versioned with fixtures | ✅ | ✅ | `harnesses/ferros-contract-validator.html` |
| C3 | Template schema versioned with fixtures | ✅ | ✅ | `harnesses/ferros-contract-validator.html` |
| C4 | Card schema versioned with fixtures | ✅ | ✅ | `harnesses/ferros-contract-validator.html` |
| C5 | Deck schema versioned with fixtures | ✅ | ✅ | `harnesses/ferros-contract-validator.html` |
| C6 | Schedule event schema versioned with fixtures | ✅ | ✅ | `harnesses/ferros-contract-validator.html` |
| C7 | Audit record schema versioned with fixtures | ✅ | ✅ | `harnesses/ferros-contract-validator.html` |
| C8 | Runtime host contract v1 (init/update/error/lifecycle) | ✅ | ✅ | `harnesses/runtime-harness.html` |
| C9 | Storage rules (what goes where, migration, corruption handling) | ✅ | ✅ | `harnesses/round-trip-harness.html` |
| C10 | Permission model skeleton (who can do what, consent capture) | ✅ | ✅ | `harnesses/negative-harness.html` |

**Legend:** ✅ confirmed pass &nbsp;|&nbsp; 🔧 harness built — open in Chrome to confirm &nbsp;|&nbsp; ⬜ not yet

**Wave 0 exit:** C1–C10 artifact ✅ AND enforcement ✅. Schemas versioned. Fixtures pass H1. Storage rules enforced by H2. Permission model enforced by H4.

### Tier 2 — Vertical Slice (Wave 1)

One real end-to-end loop works locally.

| # | Capability | Status | Proving |
|---|-----------|--------|---------|
| V1 | Template → Profile creation completes without soft-lock | ✅ | Journey 1 |
| V2 | Export → Import produces identical profile in fresh browser | ✅ | Journey 1 |
| V3a | All 4 valid session modes complete expected flows (H2 round-trip) | ✅ | Journey 1 + 2 |
| V3b | Invalid session mode combinations rejected with correct error codes (H4 negative) | ✅ | Journey 1 + 2 |
| V4 | Alias session → export → claim → XP merge works | ✅ | Journey 2 |
| V5 | Card loads in Forge → editable → renders in Runtime | ✅ | Journey 3 |
| V6 | Runtime init/update/event loop completes | ✅ | Journey 3 |
| V7 | Card round-trip export/import preserves all parameters | ✅ | Journey 3 |
| V8 | Phase 0 acceptance harness (H5) proves Journey 1 end-to-end: create→export→clear→import→render→assert | ✅ | All journeys |

**Wave 1 exit:** V1–V8 complete (V3 split into V3a + V3b). All 3 user journeys work end-to-end. H5 acceptance harness passes.

### Tier 3 — First Consumers (Wave 2)

Shared contracts serve more than one surface.

| # | Capability | Status | Surface |
|---|-----------|--------|---------|
| S1 | Schedule Ledger reads profile/template data via shared contract | ⬜ | Schedule Ledger |
| S2 | Battle Arena consumes Arena Runtime without custom data paths | ⬜ | Battle Arena |
| S3 | Showcase reads real capability status, not illustrative placeholders | ✅ | Showcase |
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

> **Naming note:** The capability IDs below use `H1`–`H6` to reference Wave 4 hardening capabilities. These are **different from** the Wave 0 harness file IDs (H1–H8) used in the Wave 0 Harness Status table above. The harness file IDs are an artifact of their order of introduction and will be renamed in a future cleanup PR.

| # | Capability | Status | Gate |
|---|-----------|--------|------|
| H1 | Schema migration rules implemented and tested | ⬜ | Storage |
| H2 | Cross-browser support matrix validated | ⬜ | Platform |
| H3 | Import/export corruption handling tested | ⬜ | Portability |
| H4 | Accessibility baseline checked | ⬜ | A11y |
| H5 | Performance budgets defined and met for core flows | ⬜ | Perf |
| H6 | Public docs match shipped behavior | ✅ | Docs (Wave 0 PR 6 closes this for Wave 0 scope) |

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

**Parallelization:** Starting after Wave 0 closure, work advances via the [five-stream architecture](../streams/STREAMS-OVERVIEW.md). Streams B (Identity Cockpit) and C (Creative Pipeline) enter Wave 1 in parallel. Stream D (Consumer Surfaces) enters Wave 2 once B and C produce their artifacts. Stream E (Core OS) proceeds independently. The wave table above describes sequential dependencies; the streams system describes how those waves execute in parallel.

---

## Audit Remediation — Finding Classification

Twenty findings from the Phase 0 exit audit. Each is classified as **resolved** (implemented + tested), **hardened** (enforcement added, full verification deferred), or **documented-and-deferred** (contract/doc positioning only, no runtime change required this wave).

| # | Finding | Category | Step | Artifacts |
|---|---------|----------|------|-----------|
| 1 | Write-path guard missing on non-profile modes | ✅ Resolved | A2 | `canMutateDurableState()` in monolith + `FerrosCore` |
| 2 | Schema version mismatch (1.0 vs 2.0) | ✅ Resolved | A1 | `profile.schema.json`, `storage-rules.md` |
| 3 | Seal metadata not persisted (hashAlgorithm, nonce) | ✅ Resolved | A3 | `computeHash` → `{hash, algorithm}`, all fixtures |
| 4 | Export reads in-memory, not persisted state | ✅ Resolved | A4 | `exportProfile()` reads `localStorage` |
| 5 | Claim/session identity not in log envelope | ✅ Resolved | C9 | `sessionId` emitted by `downloadAliasLog()` and `downloadRecoveryLog()`; documented in `storage-rules.md` and `permission-model.md` |
| 6 | Origin validation missing from C8 | 🔧 Hardened | C8 | `runtime-host-v1.md` Section 9 |
| 7 | Templates inlined, not validated against schema | ✅ Resolved | B7 | `templates.json`, `generate-ferros-core.ps1` |
| 8 | Schedule-event schema not consumed at runtime | ⬜ Deferred → Wave 2 (S1) | C10 | Note in `storage-rules.md` |
| 9 | Card/deck not included in export | ✅ Resolved — V5/V6/V7 complete; card round-trip verified by `card-deck-roundtrip.json` + H5 PASS 30/30 | C11 | `schemas/fixtures/card-deck-roundtrip.json`; see [WAVE-1-CLOSURE-EVIDENCE.md](./WAVE-1-CLOSURE-EVIDENCE.md) |
| 10 | Audit record retention unbounded | ✅ Resolved | C12 | `pushAuditEntry()` FIFO ring buffer (cap 1000) in monolith; documented in `permission-model.md` |
| 11 | `meta.version` / `schemaVersion` dual-field confusion | ✅ Resolved | A1 | `migrateProfileStructure()` rename, single canonical field |
| 12 | localStorage quota not handled | ✅ Resolved | A5 | `saveProfile()` try/catch + user modal |
| 13 | Seal chain compaction not addressed | ⬜ Deferred → Wave 4 (H5) | C13 | Note in `storage-rules.md` |
| 14 | Fixture corpus too narrow | ✅ Resolved | D14 | 5 new fixtures, `_constants.js` regenerated |
| 15 | No shared runtime core | ✅ Resolved | B6 | `ferros-core.js` IIFE, `window.FerrosCore` |
| 16 | Monolith duplicates contract logic | ✅ Resolved | B6 | Monolith delegates to `FerrosCore.*` |
| 17 | No black-box UI acceptance harness | ✅ Resolved | D15 | H8 uses DOM + localStorage only; no `contentWindow` state reads |
| 18 | No contract manifest | ✅ Resolved | D16 | `docs/contracts/manifest.json` |
| 19 | Contract/fixture co-location fragmented | 🔧 Deferred → Wave 3+ housekeeping (manifest mitigates; physical reorg low value; final disposition recorded in [WAVE-1-CLOSURE-EVIDENCE.md](./WAVE-1-CLOSURE-EVIDENCE.md)) | D16 | Manifest mitigates; physical reorg deferred |
| 20 | Harnesses don't share core verification logic | ✅ Resolved | B6 | H2 + H5 load `ferros-core.js`, delegate to `FerrosCore.validateImport` + `FerrosCore.serializeExport` |

**Totals:** 16 resolved, 1 hardened, 3 documented-and-deferred. Wave 1 audit dispositions recorded in [WAVE-1-CLOSURE-EVIDENCE.md](./WAVE-1-CLOSURE-EVIDENCE.md).

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
| The Forge | Wave 1 (vertical slice) | Authoring surface for card round-trip and local deck assembly | [forge.md](./forge.md) |
| Architecture Builder Lab | Wave 1 Exploration | Forge-pattern extension for parametric building authoring and Twin Architecture | [../builder-blueprint.md](../builder-blueprint.md) |
| Schedule Ledger | Wave 2 (first consumer) | Proves shared contract consumption | [schedule-ledger.md](./schedule-ledger.md) |
| Battle Arena | Wave 2 (first consumer) | Proves runtime can serve a second surface | [trading-arena.md](./trading-arena.md) |
| Showcase / Landing Page | Wave 2 (first consumer) | Reads real capability status | [showcase.md](./showcase.md) |
| Agent Integration | Wave 3 (permissioned actions) | Blocked until identity contract passes | [agent-integration.md](./agent-integration.md) |
| Agent Command Center | Wave 3 (permissioned actions) | Agent dispatch and audit surface | [agent-command-center.md](./agent-command-center.md) |
| Home HUD Dashboard | Wave 3+ (deferred) | Device control requires later infrastructure | [home-hud.md](./home-hud.md) |

---

## Legacy Integration Tracking

Patterns from predecessor repos are ported wave-by-wave per [ADR-013](../adr/ADR-013-legacy-integration-strategy.md).

| # | Pattern | Source | Target Wave | Status |
|---|---------|--------|-------------|--------|
| L1 | Harness drift detection (extend generate-harness-constants.ps1) | sheetgen-rust YAML→test-gen | Wave 0 | ⬜ |
| L2 | Agent trait spec for C8 runtime host | botgen-rust core/shared/agent.rs | Wave 3 | ⬜ |
| L3 | Command bus architecture for C7 audit enforcement | botgen-rust architecture | Wave 3 | ⬜ |
| L4 | Three-layer decomposition standard | sheetgen-rust src/{domain,storage,api} | Wave 1 | ⬜ Spec: ADR-014 (Proposed → Accepted on Wave 1 entry with first Rust code) |
| L5 | Template engine for Forge card authoring | botgen-rust template_engine.rs | Wave 1 Track B | ⬜ |
| L6 | Dependency graph for Card→Template chains | sheetgen-rust dependencies.rs | Wave 1 Track B | ⬜ |
| L7 | Work queue patterns for Schedule Ledger | botgen-rust work queue docs | Wave 2 | ⬜ |
| L8 | Voting/ranking for Arena | workpace-rust modules/voting | Wave 2 | ⬜ |
| L9 | Full agent system + routing | botgen-rust services/agents + core/routing | Wave 3 | ⬜ |
| L10 | WASM contract validators | workpace-rust build-wasm.sh | Research | ⬜ |

**Rule:** Legacy items are only activated when their target wave is entered. Do not port ahead of the wave structure.