# Reconciliation Gate — Wave 1

> **Status:** CLOSED — reviewer closure recorded 2026-04-19
> **Tracking issue:** [Reconciliation Gate — Wave 1 #53](https://github.com/Maangled/ferros/issues/53)
> **Authority:** `docs/ORCHESTRATION.md` §4
> **Approver:** @Maangled

---

## Purpose

This document is the formal Reconciliation Gate for Wave 1, as specified in `docs/ORCHESTRATION.md` §4. Before any Wave 1 execution PR merges, every cross-stream check below had to be reviewed and resolved.

> A PR that merges before this reconciliation gate is closed violates ORCHESTRATION.md §4 and must be reverted.

**Reviewer closure note (2026-04-19):** @Maangled signed off B-A.1 through B-A.6, C-A.1 through C-A.5, D-BC.1, D-BC.2, D-BC.3, D-BC.5, and E-AD.1 through E-AD.4 at repo-inspection level. D-BC.4 is resolved green under the deferred-dependency interpretation: Stream D may reference profile-linked inventory/progression concepts only as optional/deferred upstream artifacts, and Wave 2 implementations must tolerate `bag`, `schedule`, and related downstream fields being present but not yet populated.

---

## Gate Preconditions

| Condition | Status | Evidence |
|-----------|--------|----------|
| Wave 0 closed | ✅ | PR #47, `CLOSURE-EVIDENCE.md` |
| Stream A schemas frozen at v1.0.0 | ✅ | 7 schemas in `schemas/`, all gate harnesses pass (H1 28/28, H2 21/21, H3 18/18, H4 20/20) |
| Stream B plan approved | ✅ | `docs/streams/STREAM-B-IDENTITY-COCKPIT.md` |
| Stream C plan approved | ✅ | `docs/streams/STREAM-C-CREATIVE-PIPELINE.md` |
| Stream D plan approved | ✅ | `docs/streams/STREAM-D-CONSUMER-SURFACES.md` |
| Stream E plan approved | ✅ | `docs/streams/STREAM-E-CORE-OS.md` |

---

## Cross-Stream Reconciliation Checks

### Check 1: Stream B optional-field assumptions vs Stream A schema freeze (A ↔ B)

Stream B surfaces (Personal Profile, Schedule Ledger, Agent Command Center) depend on the following Stream A contracts:

| Contract | Schema / Document | B's Dependency |
|----------|-------------------|----------------|
| C1 | `schemas/identity.schema.json` | Session mode enforcement (4 modes: full-profile, session, alias, recovery) |
| C2 | `schemas/profile.schema.json` | Profile object structure — `meta`, `identity`, `attributes`, `skills`, `achievements`, `journal`, `credentials`, `sealChain` |
| C3 | `schemas/template.schema.json` | Template → Profile creation pipeline (V1) |
| C6 | `schemas/schedule-event.schema.json` | Schedule Ledger event format (S1) |
| C7 | `schemas/audit-record.schema.json` | Agent Command Center audit trail (P1 precursor) |
| C9 | `docs/contracts/storage-rules.md` | localStorage write rules, quota handling, `saveProfile()` guard |
| C10 | `docs/contracts/permission-model.md` | `canMutateDurableState()` gate, session mode permission matrix |

**Specific reconciliation items:**

- [x] **B-A.1** — Profile schema v1.0.0 `additionalProperties: false` constraint: Confirm B's Wave 1 plan does not require adding new top-level fields to the profile object beyond those already in `profile.schema.json` (`auditTrail`, `schedule`, `completions`, `creditLog`, `bag` are present as V2 migration fields).
- [x] **B-A.2** — Schedule event schema: Confirm B's plan for S1 (Schedule Ledger reads profile/template data) is compatible with the `schedule-event.schema.json` structure (`id`, `kind`, `label`, `time`, optional `date` / `durationMin` / `stream` / `attribute`, and `source`).
- [x] **B-A.3** — Audit record schema: Confirm B's plan distinguishes between the portable C7 `.ferros-log` envelope (`logType`, `agent`, `entries`) and the in-profile audit trail ring buffer in `profile.schema.json`. B may not silently widen either contract.
- [x] **B-A.4** — `FerrosCore` API surface: Confirm B's Wave 1 execution relies only on the published `FerrosCore` API methods in `docs/contracts/ferros-core-api.md`, including the shared `loadProfile()` / `saveProfile()` / `pushAuditEntry()` helpers. No surface may call internal functions or bypass the API.
- [x] **B-A.5** — Session mode mutual exclusivity: Confirm B's session mode handling (V3a, V3b) respects the `oneOf` constraint in `identity.schema.json` — exactly one mode active at any time.
- [x] **B-A.6** — Storage rules: Confirm B's Ledger and Agent Center surfaces do not create custom localStorage keys. Durable state must flow through `FerrosCore.saveProfile()` into the shared profile object (`schedule`, `completions`, audit trail) only.

**Reviewer closure snapshot (2026-04-19):**

| Item | Provisional state | Repo-backed note |
|------|-------------------|------------------|
| B-A.1 | Green | `schemas/profile.schema.json` is `additionalProperties: false` and already includes `auditTrail`, `schedule`, `completions`, `creditLog`, and `bag`. |
| B-A.2 | Green | Stream B documentation is normalized to the current `schedule-event.schema.json` shape, and the Ledger now persists through the shared profile path. |
| B-A.3 | Green | Stream B documentation distinguishes the portable C7 audit-record envelope from the in-profile audit trail UX, with no silent schema widening. |
| B-A.4 | Green | `ferros-core.js` publishes `loadProfile()`, `saveProfile()`, and `pushAuditEntry()`, and the Stream B surfaces rely on the published API surface only. |
| B-A.5 | Green | `schemas/identity.schema.json` keeps the `oneOf` exclusivity rule, and H1/H4/H5 provide compatible evidence that only one mode should be active at any time. |
| B-A.6 | Green | `docs/schedule-ledger.html` migrates legacy ledger keys into `profile.schedule` / `profile.completions` via `FerrosCore.saveProfile()`, and `docs/agent-command-center.html` no longer persists `acc-tab`. |

---

### Check 2: Stream C card/deck export needs vs Stream A schema guarantees (A ↔ C)

Stream C surfaces (Forge, Arena Runtime) depend on the following Stream A contracts:

| Contract | Schema / Document | C's Dependency |
|----------|-------------------|----------------|
| C4 | `schemas/card.schema.json` | Card object structure — `id`, `kind`, `name`, `version`, `contentHash`, `renderFile`, `role`, `tags`, `attribution`, `state`, `transform`, `metadata` |
| C5 | `schemas/deck.schema.json` | Deck object structure — `id`, `kind`, `name`, `version`, `cards[]`, `contentHash`, `role`, `attribution`, `renderFile`, `defaultState`, `states` |
| C8 | `docs/contracts/runtime-host-v1.md` | Runtime host lifecycle: `ferros:init`, `ferros:event`, `ferros:update`, `ferros:error`, nonce handshake |
| C3 | `schemas/template.schema.json` | Card archetype templates (shared with B) |
| C9 | `docs/contracts/storage-rules.md` | Card/deck persistence in localStorage |

**Specific reconciliation items:**

- [x] **C-A.1** — Card schema `additionalProperties: false`: Confirm C's Forge does not require card fields beyond those in `card.schema.json`. The `metadata` field (type: `object`, `additionalProperties: true`) is the designated extension point for card-type-specific data.
- [x] **C-A.2** — Deck `cards[]` references: Confirm C's deck assembly uses `cardReference` objects (`cardId`, `slot`, `group`, `instanceOf`, `transform`) as defined. No additional reference fields needed.
- [x] **C-A.3** — Runtime host contract: Confirm C's Arena Runtime implements the full C8 lifecycle (`ferros:init` → `ferros:event` → `ferros:update`) with nonce echo as verified by H3 (18/18 pass).
- [x] **C-A.4** — Card round-trip: Confirm C's Wave 1 card/deck round-trip validates against the frozen C4/C5 schemas and keeps any portability/export envelope inside Forge or a later dedicated contract. Wave 1 must not assume widened card/deck helpers on `FerrosCore`.
- [x] **C-A.5** — Template lineage: Confirm C's card archetype templates reference the same `templates.json` corpus as B's profile templates (no shadow template sets).

**Reviewer closure snapshot (2026-04-19):**

| Item | Provisional state | Repo-backed note |
|------|-------------------|------------------|
| C-A.1 | Green | Stream C documentation is normalized around the actual card/deck schemas, with card-type-specific data living under `metadata` and identity linkage under `attribution`. |
| C-A.2 | Green | `schemas/deck.schema.json` already supports `cardReference` with `cardId`, `slot`, `group`, `instanceOf`, and `transform`, and the current seam fixtures do not assert any extra reference fields. |
| C-A.3 | Green | H3 is live-pass in the current browser session (`18/18`), including nonce echo and the C8 lifecycle. |
| C-A.4 | Green | Stream C is explicitly framed around Forge-owned schema validation for card/deck round-trip, keeping `FerrosCore.serializeExport()` scoped to profile portability until a dedicated contract exists. |
| C-A.5 | Green | Stream C points at `docs/assets/_core/templates.json`, `ferros-core.js` embeds `TEMPLATE_PROFILES`, and no shadow template corpus was found in the repo. |

---

### Check 3: Stream D assumptions about artifacts from B and C (B, C ↔ D)

Stream D surfaces (Showcase, Battle Arena, Trading) consume artifacts from B and C:

| Surface | From Stream B | From Stream C | From Stream A |
|---------|---------------|---------------|---------------|
| Showcase | Profile data (personalization) | Card examples (previews) | `manifest.json` (capability status) |
| Battle Arena | Profile identity (attribution) | Deck manifests, Arena Runtime | C8 (runtime host contract) |
| Trading | Profile portability token | Card schema objects | C9, C10 (storage, permission) |

**Specific reconciliation items:**

- [x] **D-BC.1** — Showcase reads `manifest.json`: Confirm D's plan for S3 only reads the `status` field from `manifest.json` contracts and does not assume additional status metadata beyond `"active"`.
- [x] **D-BC.2** — Battle Arena uses C8 only: Confirm D's Battle Arena plan sends decks to the Runtime exclusively via `ferros:init` message and receives state via `ferros:event` / `ferros:update`. No custom message shapes.
- [x] **D-BC.3** — Consumer constraint: Confirm D does not define custom localStorage keys, private schemas, or bespoke storage. All data access is through `FerrosCore` API (Rule 1, Rule 2, Rule 3 from `STREAM-D-CONSUMER-SURFACES.md`).
- [x] **D-BC.4** — Profile data format: Confirm D's surfaces expect the `profile.schema.json` v1.0.0 shape and treat downstream fields such as `schedule`, `bag`, and related progression/inventory concepts as optional/deferred upstream artifacts until Stream B populates them.
- [x] **D-BC.5** — Card/deck fixture availability: Confirm D's Wave 2 entry condition is correctly gated on Stream C producing card/deck fixtures (V5–V7) before D attempts to consume them.

**Reviewer closure snapshot (2026-04-19):**

| Item | Provisional state | Repo-backed note |
|------|-------------------|------------------|
| D-BC.1 | Green | `STREAM-D-CONSUMER-SURFACES.md` constrains Showcase to the current manifest surface (`status` + existing manifest metadata). `ferros-showcase.html` implementation remains Wave 2 work, but the contract assumption is not wider than `manifest.json`. |
| D-BC.2 | Green | The Stream D plan constrains Battle Arena to `ferros:init`, `ferros:event`, and `ferros:update` with no custom message shapes. Current code is still a prototype, so this is plan-aligned rather than implementation-proven. |
| D-BC.3 | Green | The Stream D document repeats the consumer-only constraint set clearly. Current consumer surfaces are still scaffolding rather than divergent implementations. |
| D-BC.4 | Green | Resolved as acceptable deferred dependency language: Stream D may reference profile-linked inventory/progression concepts only as optional/deferred upstream artifacts. Wave 2 implementations must tolerate `bag`, `schedule`, and related downstream fields being present but not yet populated. |
| D-BC.5 | Green | `STREAM-D-CONSUMER-SURFACES.md` explicitly gates Wave 2 entry on Stream C V5–V7 completion before D consumes those artifacts. |

---

### Check 4: Stream E research findings vs active stream assumptions (E ↔ A–D)

Stream E is a parallel research track with explicit isolation rules (ORCHESTRATION.md §6).

**Specific reconciliation items:**

- [x] **E-AD.1** — No imposed requirements: Confirm E has not filed or imposed requirements that Streams A–D must implement. Per §6, E files requests — it does not impose.
- [x] **E-AD.2** — Fixture corpus as conformance target: Confirm E's R1 (renderer conformance suite) targets the Stream A fixture corpus as-is, without requiring modifications to the golden fixtures.
- [x] **E-AD.3** — No blocking constraints: Confirm E's research findings do not block any Wave 1 execution PR in Streams A–D. Any finding that should affect A–D must be filed as a GitHub issue per §6.
- [x] **E-AD.4** — `file://` compatibility: Confirm E's research does not introduce assumptions that conflict with the `file://` protocol compatibility requirement (AGENT_GUIDE.md constraint #1).

**Reviewer closure snapshot (2026-04-19):**

| Item | Provisional state | Repo-backed note |
|------|-------------------|------------------|
| E-AD.1 | Green | The repo docs keep Stream E as research-only and non-imposing. |
| E-AD.2 | Green | `STREAM-E-CORE-OS.md` explicitly treats the Stream A fixture corpus as the conformance target for R1/R3. |
| E-AD.3 | Green | `STREAM-E-CORE-OS.md` and `ORCHESTRATION.md` both state that Stream E is non-blocking for A–D. |
| E-AD.4 | Green | Stream E documentation explicitly preserves the `file://` compatibility constraint rather than relaxing it. |

---

## Reviewer Closure Summary

Reviewer sign-off is recorded in this document.

**Closure result (2026-04-19):**

- Green / signed off: B-A.1 through B-A.6, C-A.1 through C-A.5, D-BC.1 through D-BC.5, E-AD.1 through E-AD.4
- Red / contract mismatch: none at repo-inspection level after the current cleanup pass
- Deferred-but-acceptable dependency language: D-BC.4 is green under the condition that Stream D treats downstream inventory/progression fields as optional until upstream artifacts exist

This means the reconciliation gate is ready for closure and no longer blocks Wave 1 execution work on contract-alignment grounds.

**Non-blocking hardening follow-up:** `FerrosCore.loadProfile()`, `FerrosCore.pushAuditEntry()`, and `FerrosCore.saveProfile()` are now published consumer helpers but are not yet covered by a dedicated harness. That remains a good hardening target, not a reconciliation blocker.

---

## Reconciliation Outcome

| Check | Parties | Status | Reviewer | Notes |
|-------|---------|--------|----------|-------|
| 1 — B vs A schemas | A ↔ B | ✅ Closed | @Maangled | 6 items signed off |
| 2 — C vs A schemas | A ↔ C | ✅ Closed | @Maangled | 5 items signed off |
| 3 — D vs B+C artifacts | B, C ↔ D | ✅ Closed | @Maangled | 5 items signed off; D-BC.4 resolved as deferred dependency language |
| 4 — E vs A–D assumptions | E ↔ A–D | ✅ Closed | @Maangled | 4 items signed off |

**Total reconciliation items: 20**

---

## Security Posture Verification

Per FERROS principles (README.md Core Philosophy), the Reconciliation Gate verifies:

| Principle | Verification |
|-----------|-------------|
| **Memory Safety as Infrastructure** | All schemas enforce `additionalProperties: false` — no uncontrolled field injection |
| **Secure by Default** | Permission model (C10) starts with no permissions; all grants are explicit via `canMutateDurableState()` |
| **Privacy by Design** | Session mode mutual exclusivity (C1) prevents data leakage across modes; `saveProfile()` guard is the security boundary |
| **User-Readable Consent** | Trade Window consent dialog is the entry gate; no storage without explicit consent |
| **Local-First Sovereignty** | All data persists locally; no network calls in any schema or contract; `file://` protocol compatibility maintained |
| **Observable, Accountable, Reviewable** | Audit trail (C7) and seal chain provide tamper-evident progression records |

---

## Consent Primitives

The FERROS consent model is verified through the following primitives, all of which must be intact across all streams entering Wave 1:

| Primitive | Contract | Enforcement |
|-----------|----------|-------------|
| Trade Window acceptance | C10, C1 | `ferros_trade_accepted` flag gates localStorage writes |
| `saveProfile()` guard | C9 | Guard rejects writes when `recoveryMode` or `sessionDeclined` is true — security boundary |
| `canMutateDurableState()` | C10 | Returns `true` only in `full-profile` mode |
| Session mode isolation | C1 | `oneOf` constraint in `identity.schema.json` — exactly one mode |
| Seal chain integrity | C2 | `sealChain[]` append-only via `addSeal()` — no direct mutation |
| Audit ring buffer | C7 | FIFO eviction at 1000 entries — bounded resource use |

---

## Process

1. This document was created alongside GitHub issue [Reconciliation Gate — Wave 1 #53](https://github.com/Maangled/ferros/issues/53).
2. @Maangled reviewed every checkbox in the four checks above and signed them off on 2026-04-19.
3. Any future cross-stream regression should reopen a targeted `cross-stream-conflict` issue per ORCHESTRATION.md §5 rather than silently reinterpreting the closed gate.
4. **This reconciliation gate is closed. Wave 1 execution PRs are no longer blocked on reconciliation review.**

---

*Gate document owner: @Maangled. Created: 2026-04-19. Closed: 2026-04-19. Authority: `docs/ORCHESTRATION.md` §4.*
