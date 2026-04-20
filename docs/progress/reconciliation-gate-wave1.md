# Reconciliation Gate — Wave 1

> **Status:** OPEN — initiated 2026-04-20
> **Tracking issue:** [Reconciliation Gate — Wave 1 #53](https://github.com/Maangled/ferros/issues/53)
> **Authority:** `docs/ORCHESTRATION.md` §4
> **Approver:** @Maangled

---

## Purpose

This document is the formal Reconciliation Gate for Wave 1, as specified in `docs/ORCHESTRATION.md` §4. Before any Wave 1 execution PR merges, every cross-stream check below must be reviewed and resolved.

> A PR that merges before this reconciliation gate is closed violates ORCHESTRATION.md §4 and must be reverted.

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

- [ ] **B-A.1** — Profile schema v1.0.0 `additionalProperties: false` constraint: Confirm B's Wave 1 plan does not require adding new top-level fields to the profile object beyond those already in `profile.schema.json` (`auditTrail`, `schedule`, `completions`, `creditLog`, `bag` are present as V2 migration fields).
- [ ] **B-A.2** — Schedule event schema: Confirm B's plan for S1 (Schedule Ledger reads profile/template data) is compatible with the `schedule-event.schema.json` structure (`eventId`, `profileId`, `type`, `title`, `startTime`, `duration`, `dueDate`, `schemaVersion`).
- [ ] **B-A.3** — Audit record schema: Confirm B's plan for audit trail logging (P1 precursor) uses only the `auditEntry` actions defined in the schema (`seal-added`, `profile-saved`, `profile-imported`, `alias-claimed`, `recovery-claimed`). If B needs additional audit actions (e.g., `agent_directive`), this must be versioned as a schema addition.
- [ ] **B-A.4** — `FerrosCore` API surface: Confirm B's Wave 1 execution relies only on the published `FerrosCore` API methods in `docs/contracts/ferros-core-api.md`. No surface may call internal functions or bypass the API.
- [ ] **B-A.5** — Session mode mutual exclusivity: Confirm B's session mode handling (V3a, V3b) respects the `oneOf` constraint in `identity.schema.json` — exactly one mode active at any time.
- [ ] **B-A.6** — Storage rules: Confirm B's Ledger and Agent Center surfaces write through `FerrosCore.saveProfile()` only (C9) and do not create custom localStorage keys.

**Mechanical triage snapshot (2026-04-19, repo inspection only):**

| Item | Provisional state | Repo-backed note |
|------|-------------------|------------------|
| B-A.1 | Likely green | `schemas/profile.schema.json` is `additionalProperties: false` and already includes `auditTrail`, `schedule`, `completions`, `creditLog`, and `bag`. |
| B-A.2 | Likely green | Stream B documentation has been normalized to the current `schedule-event.schema.json` shape. Remaining work is implementation in the Ledger surface, not contract-language mismatch. |
| B-A.3 | Likely green | Stream B documentation now distinguishes the portable C7 audit-record envelope from the in-profile audit trail UX. Remaining work is implementation, not schema vocabulary mismatch. |
| B-A.4 | Red | `personal-profile.html` delegates to `FerrosCore`, but the Schedule Ledger and Agent Center prototypes still rely on bespoke state/localStorage behavior instead of the published API surface. |
| B-A.5 | Likely green | `schemas/identity.schema.json` keeps the `oneOf` exclusivity rule, and H1/H4/H5 all provide compatible evidence that only one mode should be active at a time. |
| B-A.6 | Red | `docs/schedule-ledger.html` persists `ferros_schedule` and `ferros_completions` directly, and `docs/agent-command-center.html` persists `acc-tab` directly. |

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

- [ ] **C-A.1** — Card schema `additionalProperties: false`: Confirm C's Forge does not require card fields beyond those in `card.schema.json`. The `metadata` field (type: `object`, `additionalProperties: true`) is the designated extension point for card-type-specific data.
- [ ] **C-A.2** — Deck `cards[]` references: Confirm C's deck assembly uses `cardReference` objects (`cardId`, `slot`, `group`, `instanceOf`, `transform`) as defined. No additional reference fields needed.
- [ ] **C-A.3** — Runtime host contract: Confirm C's Arena Runtime implements the full C8 lifecycle (`ferros:init` → `ferros:event` → `ferros:update`) with nonce echo as verified by H3 (18/18 pass).
- [ ] **C-A.4** — Card round-trip: Confirm C's export/import path uses `FerrosCore.validateImport()` and `FerrosCore.serializeExport()` per the API contract. No custom serialization.
- [ ] **C-A.5** — Template lineage: Confirm C's card archetype templates reference the same `templates.json` corpus as B's profile templates (no shadow template sets).

**Mechanical triage snapshot (2026-04-19, repo inspection only):**

| Item | Provisional state | Repo-backed note |
|------|-------------------|------------------|
| C-A.1 | Likely green | Stream C documentation has been normalized around the actual card/deck schemas, with card-type-specific data living under `metadata` and identity linkage under `attribution`. |
| C-A.2 | Likely green | `schemas/deck.schema.json` already supports `cardReference` with `cardId`, `slot`, `group`, `instanceOf`, and `transform`, and the current seam fixtures do not assert any extra reference fields. |
| C-A.3 | Green | H3 is live-pass in the current browser session (`18/18`), including nonce echo and the C8 lifecycle. |
| C-A.4 | Red | `FerrosCore.validateImport()` and `FerrosCore.serializeExport()` are currently C9 profile-envelope helpers, not general card/deck serializers, and `forge-workbench.html` is not yet wired to them. |
| C-A.5 | Likely green | Stream C points at `docs/assets/_core/templates.json`, `ferros-core.js` embeds `TEMPLATE_PROFILES`, and no shadow template corpus was found in the repo. |

---

### Check 3: Stream D assumptions about artifacts from B and C (B, C ↔ D)

Stream D surfaces (Showcase, Battle Arena, Trading) consume artifacts from B and C:

| Surface | From Stream B | From Stream C | From Stream A |
|---------|---------------|---------------|---------------|
| Showcase | Profile data (personalization) | Card examples (previews) | `manifest.json` (capability status) |
| Battle Arena | Profile identity (attribution) | Deck manifests, Arena Runtime | C8 (runtime host contract) |
| Trading | Profile portability token | Card schema objects | C9, C10 (storage, permission) |

**Specific reconciliation items:**

- [ ] **D-BC.1** — Showcase reads `manifest.json`: Confirm D's plan for S3 only reads the `status` field from `manifest.json` contracts and does not assume additional status metadata beyond `"active"`.
- [ ] **D-BC.2** — Battle Arena uses C8 only: Confirm D's Battle Arena plan sends decks to the Runtime exclusively via `ferros:init` message and receives state via `ferros:event` / `ferros:update`. No custom message shapes.
- [ ] **D-BC.3** — Consumer constraint: Confirm D does not define custom localStorage keys, private schemas, or bespoke storage. All data access is through `FerrosCore` API (Rule 1, Rule 2, Rule 3 from `STREAM-D-CONSUMER-SURFACES.md`).
- [ ] **D-BC.4** — Profile data format: Confirm D's surfaces expect the `profile.schema.json` v1.0.0 shape and do not assume fields that B has not yet produced (e.g., D should not assume `schedule` or `bag` fields are populated until B's respective waves complete).
- [ ] **D-BC.5** — Card/deck fixture availability: Confirm D's Wave 2 entry condition is correctly gated on Stream C producing card/deck fixtures (V5–V7) before D attempts to consume them.

**Mechanical triage snapshot (2026-04-19, repo inspection only):**

| Item | Provisional state | Repo-backed note |
|------|-------------------|------------------|
| D-BC.1 | Red | `docs/contracts/manifest.json` currently exposes contract metadata plus a single `status` field, while `STREAM-D-CONSUMER-SURFACES.md` expects Showcase to display artifact and enforcement status separately. `ferros-showcase.html` is still hardcoded placeholder UI. |
| D-BC.2 | Likely green | The Stream D plan explicitly constrains Battle Arena to `ferros:init`, `ferros:event`, and `ferros:update` with no custom message shapes. Current code is still a prototype, so this is plan-aligned rather than implementation-proven. |
| D-BC.3 | Likely green | The Stream D document repeats the consumer-only constraint set clearly. Current consumer surfaces are still scaffolding rather than divergent implementations. |
| D-BC.4 | Pending decision | Stream D assumes profile-linked inventory/progression concepts, but B has not yet produced all downstream artifacts. Reviewer should decide whether this is acceptable deferred dependency language or an implicit field assumption. |
| D-BC.5 | Green | `STREAM-D-CONSUMER-SURFACES.md` explicitly gates Wave 2 entry on Stream C V5–V7 completion before D consumes those artifacts. |

---

### Check 4: Stream E research findings vs active stream assumptions (E ↔ A–D)

Stream E is a parallel research track with explicit isolation rules (ORCHESTRATION.md §6).

**Specific reconciliation items:**

- [ ] **E-AD.1** — No imposed requirements: Confirm E has not filed or imposed requirements that Streams A–D must implement. Per §6, E files requests — it does not impose.
- [ ] **E-AD.2** — Fixture corpus as conformance target: Confirm E's R1 (renderer conformance suite) targets the Stream A fixture corpus as-is, without requiring modifications to the golden fixtures.
- [ ] **E-AD.3** — No blocking constraints: Confirm E's research findings do not block any Wave 1 execution PR in Streams A–D. Any finding that should affect A–D must be filed as a GitHub issue per §6.
- [ ] **E-AD.4** — `file://` compatibility: Confirm E's research does not introduce assumptions that conflict with the `file://` protocol compatibility requirement (AGENT_GUIDE.md constraint #1).

**Mechanical triage snapshot (2026-04-19, repo inspection only):**

| Item | Provisional state | Repo-backed note |
|------|-------------------|------------------|
| E-AD.1 | Likely green | The repo docs keep Stream E as research-only and non-imposing. Reviewer may still want a GitHub issue scan before closing the checkbox. |
| E-AD.2 | Green | `STREAM-E-CORE-OS.md` explicitly treats the Stream A fixture corpus as the conformance target for R1/R3. |
| E-AD.3 | Green | `STREAM-E-CORE-OS.md` and `ORCHESTRATION.md` both state that Stream E is non-blocking for A–D. |
| E-AD.4 | Green | Stream E documentation explicitly preserves the `file://` compatibility constraint rather than relaxing it. |

---

## Provisional Triage Summary

This triage is preparation work only. The checkboxes above remain unchecked until @Maangled or a designated reviewer explicitly signs them off.

**Current provisional read (repo inspection only):**

- Likely green / green: B-A.1, B-A.2, B-A.3, B-A.5, C-A.1, C-A.2, C-A.3, C-A.5, D-BC.2, D-BC.3, D-BC.5, E-AD.1, E-AD.2, E-AD.3, E-AD.4
- Red / contract mismatch: B-A.4, B-A.6, C-A.4, D-BC.1
- Pending reviewer decision: D-BC.4

This means the reconciliation gate is no longer an undifferentiated 20-item list. The highest-value closure path is now clear:

1. Resolve the remaining Stream B API/storage mismatches first (`B-A.4`, `B-A.6`).
2. Decide whether Stream C should gain a public card/deck validation/export API in `FerrosCore`, or whether `C-A.4` should be rewritten as a Forge-owned schema-validation rule.
3. Decide whether Stream D's Showcase requirement should be reduced to the current manifest surface or whether the manifest must grow richer status metadata (`D-BC.1`).
4. Close the likely-green items only after reviewer sign-off.

---

## Reconciliation Outcome

| Check | Parties | Status | Reviewer | Notes |
|-------|---------|--------|----------|-------|
| 1 — B vs A schemas | A ↔ B | ⬜ Pending | @Maangled | 6 items |
| 2 — C vs A schemas | A ↔ C | ⬜ Pending | @Maangled | 5 items |
| 3 — D vs B+C artifacts | B, C ↔ D | ⬜ Pending | @Maangled | 5 items |
| 4 — E vs A–D assumptions | E ↔ A–D | ⬜ Pending | @Maangled | 4 items |

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
2. @Maangled or a designated reviewer works through every checkbox in the four checks above.
3. Each item is marked ✅ when verified or ❌ with a note if a conflict is found.
4. If a conflict is found, a `cross-stream-conflict` issue is filed per ORCHESTRATION.md §5.
5. **This reconciliation gate must be closed (all items verified) before any Wave 1 execution PR merges.**

---

*Gate document owner: @Maangled. Created: 2026-04-20. Authority: `docs/ORCHESTRATION.md` §4.*
