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

---

### Check 4: Stream E research findings vs active stream assumptions (E ↔ A–D)

Stream E is a parallel research track with explicit isolation rules (ORCHESTRATION.md §6).

**Specific reconciliation items:**

- [ ] **E-AD.1** — No imposed requirements: Confirm E has not filed or imposed requirements that Streams A–D must implement. Per §6, E files requests — it does not impose.
- [ ] **E-AD.2** — Fixture corpus as conformance target: Confirm E's R1 (renderer conformance suite) targets the Stream A fixture corpus as-is, without requiring modifications to the golden fixtures.
- [ ] **E-AD.3** — No blocking constraints: Confirm E's research findings do not block any Wave 1 execution PR in Streams A–D. Any finding that should affect A–D must be filed as a GitHub issue per §6.
- [ ] **E-AD.4** — `file://` compatibility: Confirm E's research does not introduce assumptions that conflict with the `file://` protocol compatibility requirement (AGENT_GUIDE.md constraint #1).

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
