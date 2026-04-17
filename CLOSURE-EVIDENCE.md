# Wave 0 Closure Evidence
Generated: 2026-04-17 (PR 7)

This file is the permanent record that Wave 0 closure was verified. It documents the final
deterministic regeneration results, gate harness expectations, contract coverage, artifact
inventory, known gaps, and human verification instructions.

---

## Section 1: Generator Freshness

Both generators were run against current source files as part of this PR. The results are
committed to the repository.

### `tools/generate-harness-constants.ps1` → `harnesses/_constants.js`

**Result: DIFF — changes committed and explained here.**

The regeneration produced a 21-line diff versus the previously committed version. All
differences are cosmetic serialization changes introduced by the PowerShell `ConvertTo-Json`
cmdlet on the current runtime (pwsh 7.x) versus the version used in prior PRs:

1. **Unicode escape normalization** (14 affected lines): apostrophes and ampersands that were
   previously encoded as `\u0027` and `\u0026` are now emitted as literal `'` and `&`
   characters. The JSON is semantically identical — both representations are valid per RFC 8259.

2. **ISO-8601 datetime format** (14 affected lines): fixture timestamps that were previously
   serialized with explicit milliseconds (e.g., `2026-04-10T14:00:00.000Z`) are now serialized
   without the `.000` fractional seconds suffix (e.g., `2026-04-10T14:00:00Z`). Both are valid
   ISO-8601 and pass JSON Schema `format: date-time` validation.

No schema, fixture, or inventory data changed. The named arrays (`FERROS_SCHEMAS`,
`FERROS_GOLDEN_FIXTURES`, `FERROS_NEGATIVE_FIXTURES`) and all embedded JSON are functionally
identical. This diff proves the generator ran cleanly and the output reflects the current
PowerShell serializer behavior.

### `tools/generate-ferros-core.ps1` → `docs/assets/_core/ferros-core.js`

**Result: DIFF — changes committed and explained here.**

The regeneration produced a 2-line diff:

1. **BOM removal** (line 1): the previously committed file had a UTF-8 BOM (`\xEF\xBB\xBF`) at
   the start, which was stripped by the current generator's `Set-Content -Encoding UTF8
   -NoNewline` call. UTF-8 without BOM is the standard encoding for JavaScript files.

2. **Unicode escape normalization in TEMPLATE_PROFILES** (line with `TEMPLATE_PROFILES`):
   same as `_constants.js` — `\u0026` and `\u0027` replaced with literal `&` and `'`
   characters in the embedded template JSON. Semantically identical.

No template data, logic, or API surface changed. The 12 template profiles are identical.
The BOM removal is a net improvement for cross-platform compatibility.

---

## Section 2: Gate Harness Expected Results

These are the expected results when each gate harness is opened in Chrome via `file://`.
All gate harnesses must show **ALL PASS, zero FAIL** for Wave 0 exit.

| Harness | File | Contract | Expected |
|---------|------|----------|----------|
| H1 | `harnesses/ferros-contract-validator.html` | C1–C7 | ALL PASS, zero FAIL |
| H2 | `harnesses/round-trip-harness.html` | C9 | ALL PASS, zero FAIL |
| H3 | `harnesses/runtime-harness.html` | C8 | ALL PASS, zero FAIL (D-5 nonce echo green after PR 5) |
| H4 | `harnesses/negative-harness.html` | C10 | ALL PASS, zero FAIL |

**H1 — `ferros-contract-validator.html` (C1–C7)**
Validates all 7 schemas against their fixtures using embedded JSON Schema validation.
Expected: all groups A–G pass. Zero failures. C7 (audit record) validates the schema
structure without a dedicated fixture — pass by structural check.

**H2 — `round-trip-harness.html` (C9)**
Tests the full export → clear → import → assert round-trip via `FerrosCore.serializeExport()`
and `FerrosCore.validateImport()`. Group D tests real persistence via iframe + `saveProfile()`.
Expected: groups A–D all pass.

**H3 — `runtime-harness.html` (C8)**
Tests the `ferros:init` / `ferros:event` / `ferros:error` lifecycle contract against
`docs/assets/cards/trading-card.html`. D-5 nonce echo was fixed in PR 5 (trading-card.html
now stores the nonce from `ferros:init` and echoes it in all outbound messages).
Expected: groups A–D all pass including D-5.

**H4 — `negative-harness.html` (C10)**
Tests that `canMutateDurableState()` correctly denies writes in non-full-profile modes and
`validateProfileShape()` correctly rejects malformed inputs. Group E added in PR 4 with 5 deny
probe tests.
Expected: groups A–E all pass.

---

## Section 3: Supporting Harness Expected Results

Supporting harnesses are not blocking gates but must not be broken. WARN results from
supporting harnesses are acceptable — see Section 6 for known WARN sources.

| Harness | File | Role | Expected |
|---------|------|------|----------|
| H5 | `harnesses/acceptance-harness.html` | Black-box acceptance | PASS or WARN (implementation gaps OK) |
| H6 | `harnesses/write-path-harness.html` | Durable write-path | PASS or WARN |
| H7 | `harnesses/semantic-fixture-linter.html` | Cross-fixture semantics | PASS or WARN |
| H8 | `harnesses/ui-acceptance-harness.html` | UI-layer acceptance | PASS or WARN |
| Preflight | `harnesses/preflight-check.html` | File:// readiness | ALL PASS |

**H5 — `acceptance-harness.html`**
Black-box acceptance harness for the Journey 1 end-to-end flow (create → export → clear →
import → render → assert). May WARN on steps that require a real localStorage save from a
prior session. PASS or WARN is acceptable for Wave 0.

**H6 — `write-path-harness.html`**
Tests the durable write-path boundary: `canMutateDurableState()`, `saveProfile()` guard,
localStorage quota handling. May WARN if the browser sandbox limits direct localStorage
manipulation. PASS or WARN is acceptable.

**H7 — `semantic-fixture-linter.html`**
Cross-fixture semantic consistency checks that JSON Schema cannot express — e.g., seal chain
continuity, aliasId/aliasName co-presence, timestamp ordering. May WARN on fixtures that
exercise edge cases. PASS or WARN is acceptable.

**H8 — `ui-acceptance-harness.html`**
UI-layer acceptance using DOM interactions and localStorage only (no `contentWindow` reads).
Tests UI flows for profile creation, mode switching, and export. May WARN on flows that
require prior state. PASS or WARN is acceptable.

**Preflight — `preflight-check.html`**
Verifies `file://` readiness: required assets exist, `ferros-core.js` loads, 12-file harness
inventory passes. Expected: ALL PASS. If any check fails, other harnesses cannot be trusted.

---

## Section 4: Contract Coverage Matrix

| Contract | Name | Schema(s) | Fixture(s) | Gate Harness(es) | Coverage |
|----------|------|-----------|------------|-----------------|---------|
| C1 | Identity/Session Schema | `schemas/identity.schema.json` | `alias-session-log.json`, `recovery-session-log.json`, `session-mode-invariants.json`, `invalid-dual-session-mode.json`, `invalid-duplicate-alias-claim.json` | H1 | Full |
| C2 | Profile Schema | `schemas/profile.schema.json` | `minimal-stage0-profile.json`, `mid-stage1-profile.json`, `full-profile-stage3.json`, `quota-boundary-profile.json`, `profile-export-envelope.json`, `invalid-broken-seal-chain.json`, `invalid-corrupted-export.json`, `invalid-split-save-state.json` | H1, H2 | Full |
| C3 | Template Profile | `schemas/template.schema.json` | `maximum-template-schedule.json`, `profile-template-archetype-seam.json` | H1 | Full |
| C4 | Card Schema | `schemas/card.schema.json` | `deck-card-assembly-seam.json` | H1 | Full |
| C5 | Deck Schema | `schemas/deck.schema.json` | `card-deck-roundtrip.json`, `deck-card-assembly-seam.json` | H1 | Full |
| C6 | Schedule Event Schema | `schemas/schedule-event.schema.json` | `schedule-event-source-seam.json` | H1 | Full |
| C7 | Audit Record Schema | `schemas/audit-record.schema.json` | *(none — structural only; see known gaps)* | H1 | Structural-only |
| C8 | Runtime Host Contract | `docs/contracts/runtime-host-v1.md` | *(none — behavioral; trading-card.html is conformance asset)* | H3 | Full |
| C9 | Storage Rules | `docs/contracts/storage-rules.md` | `profile-export-envelope.json`, `quota-boundary-profile.json`, `card-deck-roundtrip.json`, and 8 additional profile/identity fixtures | H2, H6 | Full |
| C10 | Permission Model | `docs/contracts/permission-model.md` | `invalid-forbidden-meta-field.json`, `invalid-dual-session-mode.json`, `invalid-split-save-state.json`, and deny probe tests | H4 | Full |

---

## Section 5: Artifact Inventory

### Schemas (7)

| File | Contract |
|------|----------|
| `schemas/identity.schema.json` | C1 |
| `schemas/profile.schema.json` | C2 |
| `schemas/template.schema.json` | C3 |
| `schemas/card.schema.json` | C4 |
| `schemas/deck.schema.json` | C5 |
| `schemas/schedule-event.schema.json` | C6 |
| `schemas/audit-record.schema.json` | C7 |

### Fixtures (19 total: 13 golden + 6 negative)

**Golden fixtures (13):**

| File | Contracts |
|------|-----------|
| `schemas/fixtures/minimal-stage0-profile.json` | C2 |
| `schemas/fixtures/mid-stage1-profile.json` | C2 |
| `schemas/fixtures/full-profile-stage3.json` | C2 |
| `schemas/fixtures/quota-boundary-profile.json` | C2, C9 |
| `schemas/fixtures/profile-export-envelope.json` | C2, C9 |
| `schemas/fixtures/alias-session-log.json` | C1 |
| `schemas/fixtures/recovery-session-log.json` | C1 |
| `schemas/fixtures/session-mode-invariants.json` | C1 |
| `schemas/fixtures/maximum-template-schedule.json` | C3 |
| `schemas/fixtures/profile-template-archetype-seam.json` | C3, C2 |
| `schemas/fixtures/card-deck-roundtrip.json` | C5, C9 |
| `schemas/fixtures/deck-card-assembly-seam.json` | C4, C5 |
| `schemas/fixtures/schedule-event-source-seam.json` | C6 |

**Negative fixtures (6):**

| File | Contract |
|------|----------|
| `schemas/fixtures/invalid-broken-seal-chain.json` | C2 |
| `schemas/fixtures/invalid-corrupted-export.json` | C2 |
| `schemas/fixtures/invalid-dual-session-mode.json` | C1, C10 |
| `schemas/fixtures/invalid-duplicate-alias-claim.json` | C1 |
| `schemas/fixtures/invalid-forbidden-meta-field.json` | C10 |
| `schemas/fixtures/invalid-split-save-state.json` | C2, C10 |

### Harnesses (8 + preflight = 9)

| ID | File | Role | Gate? |
|----|------|------|-------|
| H1 | `harnesses/ferros-contract-validator.html` | C1–C7 full schema matrix | Gate |
| H2 | `harnesses/round-trip-harness.html` | C9 storage round-trip | Gate |
| H3 | `harnesses/runtime-harness.html` | C8 runtime host lifecycle | Gate |
| H4 | `harnesses/negative-harness.html` | C10 permission model deny probes | Gate |
| H5 | `harnesses/acceptance-harness.html` | Black-box Journey 1 acceptance | Supporting |
| H6 | `harnesses/write-path-harness.html` | C9 durable write-path boundary | Supporting |
| H7 | `harnesses/semantic-fixture-linter.html` | Cross-fixture semantic consistency | Supporting |
| H8 | `harnesses/ui-acceptance-harness.html` | UI-layer acceptance | Supporting |
| — | `harnesses/preflight-check.html` | File:// readiness / asset inventory | Supporting |

### Generators (2)

| File | Output |
|------|--------|
| `tools/generate-harness-constants.ps1` | `harnesses/_constants.js` |
| `tools/generate-ferros-core.ps1` | `docs/assets/_core/ferros-core.js` |

### Generated Files (2)

| File | Source | Role |
|------|--------|------|
| `harnesses/_constants.js` | All schemas + all fixtures | Embedded constants for harness use |
| `docs/assets/_core/ferros-core.js` | `docs/assets/_core/templates.json` | Shared runtime IIFE bundle |

### Contract Docs (6)

| File | Contracts |
|------|-----------|
| `docs/contracts/manifest.json` | All C1–C10 |
| `docs/contracts/storage-rules.md` | C9 |
| `docs/contracts/permission-model.md` | C10 |
| `docs/contracts/ferros-core-api.md` | C8, C9 (FerrosCore API) |
| `docs/contracts/runtime-host-v1.md` | C8 |
| `docs/contracts/surface-bootstrap.md` | C8 |

### ADRs with Wave 0 Addenda (5)

| File | Addendum topic |
|------|----------------|
| `docs/adr/ADR-001-progression-lock-pattern.md` | Hardened closure baseline, H1–H8 numbering |
| `docs/adr/ADR-004-template-profile-specification.md` | Build-time + runtime template validation, templateToEvents bridge |
| `docs/adr/ADR-007-single-file-system.md` | Single-file constraint maintained, ferros-core.js extraction |
| `docs/adr/ADR-013-legacy-integration-strategy.md` | No legacy ports in Wave 0, boundary confirmed |
| `docs/adr/ADR-014-three-layer-decomposition.md` | Three layers mapped to actual file structure |

---

## Section 6: Known Gaps and Deferred Items

### Supporting harness WARN expectations

- **H5 (acceptance-harness.html)**: May WARN on Journey 1 steps that require a real
  `saveProfile()` write from a prior browser session. Black-box acceptance of the full
  create-export-import loop is partially dependent on browser localStorage state. Deferred
  to Wave 1 vertical slice (V8) where the full journey will be provable end-to-end.

- **H6 (write-path-harness.html)**: May WARN on quota handling tests if the browser sandbox
  limits artificial quota exhaustion. The `canMutateDurableState()` guard is fully tested;
  quota recovery behavior is documented in `storage-rules.md` and confirmed by code inspection.

- **H7 (semantic-fixture-linter.html)**: May WARN on cross-fixture consistency checks for
  fixtures that intentionally exercise boundary conditions (e.g., null genesisHash in
  `minimal-stage0-profile.json`). These WARNs indicate known edge cases, not violations.

- **H8 (ui-acceptance-harness.html)**: May WARN on UI flows that require prior session state
  (e.g., testing the export button requires a profile to already be saved). Full UI acceptance
  is deferred to Wave 1 (V8).

### Contracts with partial coverage

- **C7 (Audit Record Schema)**: No dedicated golden fixture. The schema is validated
  structurally in H1 but there is no `audit-record.json` fixture exercising the full entry
  lifecycle. Full fixture deferred to Wave 1 when a surface consumes audit records.

- **C8 (Runtime Host Contract)**: Origin validation (`§9.1`) is enforcement-on-conformance
  only for `file://` — the browser does not enforce cross-origin restrictions in the same way
  as `https://`. This is documented in `runtime-host-v1.md §9` and is acceptable for
  Wave 0 local-first use.

### Deferred items (explicitly punted to Wave 1+)

| # | Item | Target |
|---|------|--------|
| 1 | C7 audit record golden fixture | Wave 1 (first audit consumer) |
| 2 | C6 schedule-event runtime consumption | Wave 2 (S1 — Schedule Ledger) |
| 3 | Card/deck inclusion in export envelope | Wave 1 (V5–V7) |
| 4 | Seal chain compaction strategy | Wave 4 (H5 hardening) |
| 5 | Contract/fixture physical co-location reorg | Wave 1 |
| 6 | Full end-to-end Journey 1 acceptance (H5) | Wave 1 (V8) |
| 7 | Cross-browser support validation | Wave 4 (H2) |

---

## Section 7: Verification Instructions

Steps for a human to independently verify Wave 0 closure at this commit:

### Prerequisites
- Chrome (or Chromium) installed
- PowerShell 7+ (`pwsh`) installed
- Git and a local clone of the repository at this commit

### Step 1: Clone the repository

```sh
git clone https://github.com/Maangled/ferros.git
cd ferros
git checkout <this-commit-sha>
```

### Step 2: Regenerate `_constants.js` and verify zero diff

```powershell
pwsh -ExecutionPolicy Bypass -File tools/generate-harness-constants.ps1
git diff harnesses/_constants.js
```

Expected: **zero diff** (the committed file is byte-identical to a fresh regeneration at this
commit).

### Step 3: Regenerate `ferros-core.js` and verify zero diff

```powershell
pwsh -ExecutionPolicy Bypass -File tools/generate-ferros-core.ps1
git diff docs/assets/_core/ferros-core.js
```

Expected: **zero diff** (the committed file is byte-identical to a fresh regeneration at this
commit).

### Step 4: Open gate harnesses in Chrome

Open each file directly via `file://` — do not use a local server. Chrome allows `file://`
access for these harnesses.

```
file:///path/to/ferros/harnesses/ferros-contract-validator.html   (H1)
file:///path/to/ferros/harnesses/round-trip-harness.html           (H2)
file:///path/to/ferros/harnesses/runtime-harness.html              (H3)
file:///path/to/ferros/harnesses/negative-harness.html             (H4)
```

For each harness, click **Run Tests** (or equivalent trigger) and verify:
- All test groups show **PASS**
- Zero **FAIL** results
- The result summary shows the expected pass count

### Step 5: Open preflight check

```
file:///path/to/ferros/harnesses/preflight-check.html
```

Expected: all checks pass (ferros-core.js loads, required assets found, 12-file inventory
confirmed). If preflight fails, other harnesses cannot be trusted.

### Step 6: Open supporting harnesses (optional, for completeness)

```
file:///path/to/ferros/harnesses/acceptance-harness.html           (H5)
file:///path/to/ferros/harnesses/write-path-harness.html           (H6)
file:///path/to/ferros/harnesses/semantic-fixture-linter.html      (H7)
file:///path/to/ferros/harnesses/ui-acceptance-harness.html        (H8)
```

Expected: PASS or WARN (see Section 6 for known WARN sources). Any FAIL from a supporting
harness should be investigated but does not block Wave 0 closure.

### Step 7: Confirm artifact inventory

Verify the following files exist at their expected paths:

**Schemas:** `schemas/{identity,profile,template,card,deck,schedule-event,audit-record}.schema.json`

**Golden fixtures (13):**
`schemas/fixtures/{minimal-stage0-profile,mid-stage1-profile,full-profile-stage3,quota-boundary-profile,profile-export-envelope,alias-session-log,recovery-session-log,session-mode-invariants,maximum-template-schedule,profile-template-archetype-seam,card-deck-roundtrip,deck-card-assembly-seam,schedule-event-source-seam}.json`

**Negative fixtures (6):**
`schemas/fixtures/{invalid-broken-seal-chain,invalid-corrupted-export,invalid-dual-session-mode,invalid-duplicate-alias-claim,invalid-forbidden-meta-field,invalid-split-save-state}.json`

**Harnesses:**
`harnesses/{ferros-contract-validator,round-trip-harness,runtime-harness,negative-harness,acceptance-harness,write-path-harness,semantic-fixture-linter,ui-acceptance-harness,preflight-check}.html`

**Generated:**
`harnesses/_constants.js`, `docs/assets/_core/ferros-core.js`

**Contract docs:**
`docs/contracts/{manifest.json,storage-rules.md,permission-model.md,ferros-core-api.md,runtime-host-v1.md,surface-bootstrap.md}`
