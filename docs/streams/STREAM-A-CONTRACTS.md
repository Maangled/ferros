# Stream A — Contract & Backend Infrastructure

> **Stream status:** Wave 0 CLOSED (PR #47). Currently in Wave 1 hardening.
> **Philosophy:** Be the bedrock. Contracts first, always. Binary gates, not percentages.

---

## What This Stream Is

Stream A is the foundation. Every other stream builds on the contracts defined here. If Stream B needs to know what a Profile looks like, it reads `schemas/profile.schema.json`. If Stream C needs to know how a Card is structured, it reads `schemas/card.schema.json`. No stream invents its own rules — they all defer to Stream A.

This means Stream A must finish its foundational work first. But "finish" does not mean "done forever." It means: **publish a stable version that other streams can depend on, then keep hardening without breaking what's already shipping.**

The key insight is that schemas can be versioned. Stream A Wave 0 published `v1.0.0`. That baseline is now frozen. Future changes go through the evolution cascade defined in ADR-012.

---

## What Contracts Exist

All contracts are managed in `docs/contracts/manifest.json`. Here is the full current inventory:

### Schemas (7 files in `schemas/`)

| ID | Name | File | Fixtures | Gate Harness |
|----|------|------|----------|-------------|
| C1 | Identity / Session Schema | `schemas/identity.schema.json` | 4 fixtures | H1 |
| C2 | Profile Schema | `schemas/profile.schema.json` | 9 fixtures | H1, H2 |
| C3 | Template Profile | `schemas/template.schema.json` | 1 fixture | H1 |
| C4 | Card Schema | `schemas/card.schema.json` | 2 golden | H1 |
| C5 | Deck Schema | `schemas/deck.schema.json` | 2 golden | H1 |
| C6 | Schedule Event Schema | `schemas/schedule-event.schema.json` | 1 fixture | H1 |
| C7 | Audit Record Schema | `schemas/audit-record.schema.json` | 4 fixtures | H1 |

### Contract Documents (in `docs/contracts/`)

| ID | Name | File | Gate Harness |
|----|------|------|-------------|
| C8 | Runtime Host Contract | `runtime-host-v1.md` | H3 |
| C9 | Storage Rules | `storage-rules.md` | H2, H6 |
| C10 | Permission Model | `permission-model.md` | H4 |

### Generated Files

| File | Generator | Role |
|------|-----------|------|
| `harnesses/_constants.js` | `tools/generate-harness-constants.ps1` | Embeds all schema + fixture JSON for harness use |
| `docs/assets/_core/ferros-core.js` | `tools/generate-ferros-core.ps1` | Shared runtime IIFE — `window.FerrosCore` |

---

## Current Contract State

**Wave 0: CLOSED** — All 10 contracts (C1–C10) have artifact ✅ and enforcement ✅.

Full closure record: `CLOSURE-EVIDENCE.md`

### Deferred Items (Not Wave 0 Blockers)

These items were identified during Wave 0 and explicitly deferred. They are **not Wave 0 reopeners** — they are Stream A Wave 1 work items.

| # | Item | Target | Current State |
|---|------|--------|---------------|
| 1 | C7 audit record fixture coverage | Wave 1 | Closed on `main` via alias/recovery portable-log fixtures plus the claimed-alias merge fixture |
| 2 | C6 schedule-event runtime consumption | Wave 2 (S1 — Schedule Ledger) | `templateToEvents()` exists; no surface consumes it yet |
| 3 | Card/deck inclusion in export envelope | Wave 1 (V5–V7) | Storage rules note deferred item |
| 4 | Seal chain compaction strategy | Wave 4 (H5 hardening) | Note in storage-rules.md |
| 5 | Contract/fixture physical co-location | Wave 1 | Manifest mitigates; physical reorg deferred |
| 6 | Full end-to-end Journey 1 acceptance (H5) | Wave 1 (V8) | Complete — H5 is green at 30/30 with alias export → claim coverage |
| 7 | Cross-browser support validation | Wave 4 (H2) | Not started |

---

## Freeze Policy

### The Rule

> **Wave 0 contracts are frozen at v1.0.0 as of PR #47.**

This means:
- No field additions to existing schemas without a version bump
- No field removals without a major version bump and migration rule
- No harness changes that break existing green tests
- **Any new audit finding goes to the backlog — never to a Wave 0 reopener**

### Why Freeze?

The "audit → reopen → audit" cycle was the project's primary bottleneck. Every time a new finding was discovered, Wave 0 was declared "not closed," which blocked all other streams from starting. The freeze rule breaks this cycle by separating two concerns:

1. **Is the contract good enough to build on?** → Yes. Wave 0 is closed.
2. **Are there things we'd like to improve?** → Yes. That's what Wave 1 is for.

The second question never invalidates the first.

### How New Findings Are Triaged

When an audit produces a new finding after Wave 0 closure:

```
New finding discovered
        │
        ▼
Is it a critical security or data-loss issue?
        │
   ┌────┴────┐
   YES       NO
   │         │
   ▼         ▼
Hotfix    Classify by severity:
release   - LOW: document in deferred backlog
           - MED: Stream A Wave 1 work item
           - HIGH: immediate patch within current wave
```

Findings are **never** classified as "Wave 0 reopeners" unless a gate harness is actively failing on main. If H1–H4 all pass, Wave 0 is closed.

---

## Schema Versioning Rules

All schemas follow semantic versioning via `"$schema"` and `"version"` fields.

### Version Bump Rules

| Change Type | Version Impact | Migration Rule Required? |
|-------------|---------------|--------------------------|
| Add optional field | `1.x.0` minor bump | No |
| Add required field | `2.0.0` major bump | Yes — backward migration required |
| Remove field | `2.0.0` major bump | Yes — forward migration required |
| Rename field | `2.0.0` major bump | Yes |
| Change field type | `2.0.0` major bump | Yes |
| Change validation rule (stricter) | `1.x.0` minor bump | Recommended |

### Migration Cascade

When a schema bumps a major version, ADR-012 (Schema Evolution Cascade) defines the required artifacts:
1. New schema file at new version
2. Migration function in `ferros-core.js`
3. Migration test fixture proving the transformation
4. Updated harness coverage

This is enforced by the Wave 4 gate: H1 (Schema migration rules implemented and tested).

---

## Gate Harnesses

Stream A uses four primary gate harnesses. All run in Chrome via `file://`.

### H1 — `harnesses/ferros-contract-validator.html`

**Contracts:** C1–C7 (all 7 schemas)  
**What it tests:** Every schema against its fixtures. JSON Schema validation using embedded fixture corpus from `harnesses/_constants.js`. Organized into groups A–G, one group per schema.  
**Expected result:** ALL PASS, zero FAIL.  
**When to run:** After any schema change, after any fixture change, after regenerating `_constants.js`.

### H2 — `harnesses/round-trip-harness.html`

**Contract:** C9 (Storage Rules)  
**What it tests:** Full export → clear → import → assert round-trip via `FerrosCore.serializeExport()` and `FerrosCore.validateImport()`. Group D tests real persistence via iframe + `saveProfile()`.  
**Expected result:** ALL PASS, zero FAIL.  
**When to run:** After any change to storage rules, export/import logic, or profile schema.

### H3 — `harnesses/runtime-harness.html`

**Contract:** C8 (Runtime Host Contract)  
**What it tests:** `ferros:init` / `ferros:event` / `ferros:error` lifecycle against `docs/assets/cards/trading-card.html`. Nonce echo confirmed in PR 5.  
**Expected result:** ALL PASS, zero FAIL.  
**When to run:** After any change to the runtime host contract or to `trading-card.html`.

### H4 — `harnesses/negative-harness.html`

**Contract:** C10 (Permission Model)  
**What it tests:** `canMutateDurableState()` correctly denies writes in non-full-profile modes. `validateProfileShape()` rejects malformed inputs. Group E: 5 deny probe tests.  
**Expected result:** ALL PASS, zero FAIL.  
**When to run:** After any permission model change, after any profile validation change.

### Supporting Harnesses

| ID | File | Role | WARN acceptable? |
|----|------|------|-----------------|
| H5 | `harnesses/acceptance-harness.html` | Journey 1 acceptance | Yes (Wave 0) |
| H6 | `harnesses/write-path-harness.html` | Durable write-path | Yes |
| H7 | `harnesses/semantic-fixture-linter.html` | Cross-fixture semantics | Yes |
| H8 | `harnesses/ui-acceptance-harness.html` | UI-layer acceptance | Yes |
| — | `harnesses/preflight-check.html` | File:// readiness | No — must ALL PASS |

---

## The `ferros-core.js` API

`docs/assets/_core/ferros-core.js` is generated by `tools/generate-ferros-core.ps1`. It exposes `window.FerrosCore`, the shared runtime used by every harness and every surface.

Full API documentation: `docs/contracts/ferros-core-api.md`

### Key Methods

```javascript
// Profile read/write
FerrosCore.saveProfile(profileObject)        // writes to localStorage with quota guard
FerrosCore.loadProfile()                      // reads from localStorage
FerrosCore.serializeExport(profileObject)    // returns portable export envelope
FerrosCore.validateImport(envelope)          // validates and returns hydrated profile

// Permission gates
FerrosCore.canMutateDurableState(sessionMode) // returns boolean
FerrosCore.validateProfileShape(profileObject) // throws on malformed input

// Audit and integrity
FerrosCore.pushAuditEntry(entry)             // appends to audit ring (cap 1000)
FerrosCore.computeHash(data)                 // returns {hash, algorithm}

// Template bridge
FerrosCore.templateToEvents(template)        // converts template → schedule events (C6)
```

---

## Philosophy: Why Contracts-First?

### Why schemas before surfaces?

If you build surfaces first, you end up with four different opinions about what a Profile looks like. Surface 1 uses `user.name`. Surface 2 uses `profile.displayName`. Surface 3 uses `meta.handle`. When you try to make them share data, you get a translation layer on top of a translation layer.

Contracts-first means: **we decide the shape of truth before anyone builds a form.** The schema is the arbiter. Surfaces conform to it.

### Why binary gates, not percentages?

Percentages invite negotiation. "We're at 80% — close enough." Binary gates don't. Either the harness passes or it doesn't. Either the schema validates or it doesn't.

This makes progress unambiguous. When all four gate harnesses show ALL PASS, Wave 0 is done. No interpretation required.

### Why `ferros-core.js` as a shared IIFE?

Because FERROS runs on `file://` in Chrome, not on a server. There's no module loader, no npm, no build step at runtime. The shared core is embedded as a plain IIFE that attaches to `window.FerrosCore`. Every harness and every surface loads it the same way:

```html
<script src="../docs/assets/_core/ferros-core.js"></script>
```

This is not a limitation — it's a deliberate architectural choice. The platform must work offline, locally, without any infrastructure. The IIFE pattern enforces that contract.

---

## Stream A Wave 1 — Hardening Backlog

These items are the current Stream A work queue. The first two still gate Phase A closure / Phase B entry.

| # | Item | Priority | Notes |
|---|------|----------|-------|
| 1 | Wave 1 closure evidence + audit reconciliation | HIGH | PR 11 closes the Wave 1 ceremony and findings #9 / #19 |
| 2 | C6 runtime consumption contract | HIGH | Promote `templateToEvents()` to a consumed contract before S1 begins |
| 3 | Contract/fixture physical co-location | LOW | Manifest mitigates the nav issue; reorg is cosmetic |
| 4 | Seal chain compaction strategy | LOW | Wave 4 (H5) target |
| 5 | Schema evolution cascade tooling | MED | L10 (WASM validators) is research track |
