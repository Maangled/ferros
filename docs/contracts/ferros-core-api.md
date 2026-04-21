# FerrosCore API Contract

**Module:** `docs/assets/_core/ferros-core.js`  
**Pattern:** Classic-script IIFE → `window.FerrosCore`  
**Version:** 1.0  
**Dependencies:** None (zero external deps, file:// compatible)

---

## Public API Surface

All functions and properties listed below are accessible on `window.FerrosCore` after loading
`ferros-core.js`. This is the complete public surface — no private helpers are exposed.

| Symbol | Kind | Harness(es) |
|--------|------|-------------|
| `VERSION` | property | H1, H2, H3, H4 |
| `TEMPLATE_PROFILES` | property | H1, H5 |
| `ALIAS_SESSION_STORAGE_KEY` | property | H4, Stream A surfaces |
| `PORTABLE_LOG_XP_PER_ENTRY` | property | H2, H5 |
| `computeHash` | async function | H1, H2 |
| `hashWithAlgorithm` | async function | H2 |
| `createSealEntry` | async function | H2, H5 |
| `verifyChain` | function | H2 |
| `verifyChainFull` | async function | H2 |
| `createAliasSession` | function | H5, Stream A surfaces |
| `appendAliasSessionEntry` | function | H5, Stream A surfaces |
| `serializeAliasSessionLog` | function | H2, H5 |
| `serializeRecoverySessionLog` | function | H2, Stream A surfaces |
| `validatePortableLog` | async function | H2, H4 |
| `applyPortableLogClaim` | async function | H2, H4, H6 |
| `canMutateDurableState` | function | H4 |
| `validateImport` | function | H2 |
| `validateProfileShape` | function | H4 |
| `loadProfile` | function | Stream B, Stream D consumers |
| `pushAuditEntry` | function | Stream B, Stream D consumers |
| `saveProfile` | function | Stream B, Stream D consumers |
| `generateRuntimeNonce` | function | H3 |
| `validateRuntimeMessage` | function | H3 |
| `templateBlockToEvent` | function | H1 |
| `templateToEvents` | function | H1 |
| `serializeExport` | function | H2 |

---

## Properties

| Property | Type | Description |
|----------|------|-------------|
| `VERSION` | `string` | Current schema version (`"1.0"`) |
| `TEMPLATE_PROFILES` | `Array<object>` | Embedded template profiles (populated by `generate-ferros-core.ps1` at build time; inline fallback preserved for standalone loading) |
| `ALIAS_SESSION_STORAGE_KEY` | `string` | Canonical `sessionStorage` key for alias-session backing state (`"ferros_alias_session"`) |
| `PORTABLE_LOG_XP_PER_ENTRY` | `number` | Fixed XP grant per claimed portable-log entry (`15`) |

---

## Methods

### Hashing & Sealing

#### `computeHash(data) → Promise<{hash, algorithm}>`

Computes a hash of the string `data`. Uses SHA-256 via `crypto.subtle` when available (`https://`), falls back to djb2 on `file://`.

- **Returns:** `{ hash: string, algorithm: 'sha256' | 'djb2' }`
- **Harnesses:** H1, H2

#### `hashWithAlgorithm(data, algorithm) → Promise<{ok, hash?, code?}>`

Deterministic hash helper for cross-context re-verification. Given a specific algorithm, produces the same hash or returns an error if the algorithm is unavailable.

- **Params:** `data` (string), `algorithm` (`'sha256'` | `'djb2'`)
- **Returns:** `{ ok: true, hash }` or `{ ok: false, code: 'SHA256_UNAVAILABLE' | 'UNKNOWN_HASH_ALGORITHM' }`
- **Harnesses:** H2 (used internally by `verifyChainFull`)

#### `createSealEntry(taskId, taskData, previousSeal) → Promise<object>`

Creates a new seal chain entry with `{ taskId, seal, previousSeal, timestamp, data, hashAlgorithm, nonce }`. Generates a random nonce, hashes the payload, and returns the complete entry.

- **Params:** `taskId` (string), `taskData` (object), `previousSeal` (string | null, `"genesis"` for first)
- **Returns:** Seal entry object ready for `sealChain.push()`
- **Harnesses:** H2, H5

### Chain Verification

#### `verifyChain(chain) → {valid, brokenAt?}`

Linkage-only chain verification. Checks that `chain[0].previousSeal === "genesis"` and that each subsequent entry's `previousSeal` matches the prior entry's `seal`. Does **not** rehash.

- **Params:** `chain` (Array of seal entries)
- **Returns:** `{ valid: true }` or `{ valid: false, brokenAt: number }`
- **Harnesses:** H2

#### `verifyChainFull(chain) → Promise<{valid, brokenAt?, reason?}>`

Full chain verification: runs `verifyChain` linkage check first, then re-hashes every entry using the stored `hashAlgorithm` and `nonce` to confirm each `seal` field is correct.

- **Params:** `chain` (Array of seal entries, each with `taskId`, `seal`, `previousSeal`, `timestamp`, `data`, `hashAlgorithm`, `nonce`)
- **Returns:** `{ valid: true }` or `{ valid: false, brokenAt: number, reason: 'LINKAGE_BROKEN' | 'SEAL_FIELDS_MISSING' | 'HASH_MISMATCH' | 'SHA256_UNAVAILABLE' }`
- **Harnesses:** H2

### Portable Logs

#### `createAliasSession(alias, options?) → {ok, session?, code?, detail?}`

Creates the canonical alias-session object stored at `sessionStorage[ALIAS_SESSION_STORAGE_KEY]`. The returned session contains a generated `sessionId`, normalized alias identity, `sessionStart`, `sessionEnd`, and an empty `entries` array.

- **Params:** `alias` (`{ id, name, icon?, aliasClass?, class?, tagline? }`), `options?` (`{ now?, sessionId? }`)
- **Returns:** `{ ok: true, session }` or `{ ok: false, code: 'ALIAS_REQUIRED' | 'ALIAS_ID_REQUIRED' | 'ALIAS_NAME_REQUIRED' }`
- **Consumers:** Personal Profile alias-mode entry point, Stream A surfaces

#### `appendAliasSessionEntry(session, entry) → {ok, session?, code?, detail?}`

Appends a normalized alias activity entry to a canonical alias session and updates `sessionEnd`. This is the only supported way to add portable-log entries before export.

- **Params:** `session` (alias session object), `entry` (`{ ts?, text, type? }`)
- **Returns:** `{ ok: true, session }` or `{ ok: false, code: 'PORTABLE_LOG_SESSION_REQUIRED' | 'PORTABLE_LOG_ENTRY_TEXT_REQUIRED' }`
- **Consumers:** Personal Profile alias-mode journal flow

#### `serializeAliasSessionLog(session, options?) → {ok, log?, code?, detail?}`

Builds the canonical `.ferros-log` alias envelope from an alias session, including `sessionId`, normalized alias identity, entry seals, `entryCount`, and claim instructions.

- **Params:** `session` (alias session object), `options?` (`{ ferrosVersion?, claimInstructions? }`)
- **Returns:** `{ ok: true, log }` or `{ ok: false, code, detail? }`
- **Harnesses:** H2 portable-log group, H5 alias export journey

#### `serializeRecoverySessionLog(session, options?) → {ok, log?, code?, detail?}`

Builds the canonical `.ferros-log` recovery envelope. The canonical recovery identity field is `recovery`; legacy `profile` recovery payloads remain accepted only at validation time for backwards compatibility.

- **Params:** `session` (recovery session-like object), `options?` (`{ ferrosVersion?, claimInstructions? }`)
- **Returns:** `{ ok: true, log }` or `{ ok: false, code, detail? }`
- **Harnesses:** H2 portable-log group

#### `validatePortableLog(raw) → Promise<{ok, log?, sessionId?, entryCount?, xpGain?, integrityWarning?, warnings?, code?, detail?}>`

Canonical parser/validator for alias and recovery `.ferros-log` envelopes. Enforces `sessionId`, entry-count consistency, alias/recovery identity requirements, and per-entry seal verification. Missing or mismatched entry seals downgrade to `integrityWarning: true`; malformed envelopes hard-fail.

- **Params:** `raw` (parsed JSON object)
- **Returns:** `{ ok: true, log, sessionId, entryCount, xpGain, integrityWarning, warnings }` or `{ ok: false, code, detail? }`
- **Error codes:** `PORTABLE_LOG_REQUIRED`, `PORTABLE_LOG_TYPE_INVALID`, `PORTABLE_LOG_SESSION_ID_REQUIRED`, `PORTABLE_LOG_ENTRY_COUNT_INVALID`, `PORTABLE_LOG_ENTRY_COUNT_MISMATCH`, `PORTABLE_LOG_ALIAS_REQUIRED`, `PORTABLE_LOG_RECOVERY_REQUIRED`, `PORTABLE_LOG_ENTRIES_REQUIRED`
- **Harnesses:** H2 (portable-log validation), H4 (negative claim-path validation)

#### `applyPortableLogClaim(profile, rawLog, options) → Promise<{ok, profile?, sealChain?, claimId?, sessionId?, xpGain?, integrityWarning?, warnings?, code?, detail?}>`

Canonical claim/merge path for alias and recovery portable logs. Validates the envelope, rejects duplicate `sessionId` values before mutation, merges journal rows, updates `meta.xp` and `meta.claimedAliasSessions`, appends an `alias-claim` / `recovery-claim` seal, emits `seal-added` plus `alias-claimed` / `recovery-claimed`, and optionally persists through `saveProfile()`.

- **Params:** `profile` (object), `rawLog` (parsed log object), `options` (`{ persist?: boolean, flags?, now? }`)
- **Returns:** `{ ok: true, profile, sealChain, claimId, sessionId, xpGain, integrityWarning, warnings }` or `{ ok: false, code, detail? }`
- **Error codes:** `CLAIM_DUPLICATE_SESSION`, `DURABLE_WRITE_FORBIDDEN`, plus any `validatePortableLog()` or `saveProfile()` errors
- **Harnesses:** H2 (duplicate rejection), H4 (deny matrix), H6 (persist + reload verification)

### Profile Validation

#### `validateProfileShape(profile) → {ok, code, detail}`

Write-time profile shape validator (A1b). Whitelist-based field checks — rejects any profile with undeclared top-level or `meta` fields. Validates:
- Top-level allowed + required properties
- `meta` allowed + required properties
- `journalEntry.type` enum (6 values: `activity`, `journal`, `system`, `claim-event`, `claimed-alias`, `claimed-recovery`)
- Achievement required fields (`id`, `name`)

- **Params:** `profile` (object)
- **Returns:** `{ ok: true, code: null }` or `{ ok: false, code: 'PROFILE_SHAPE_INVALID', detail: string }`
- **Harnesses:** H4 (Group E — E-5 validates rejection of undeclared fields)

#### `validateImport(raw) → {ok, code?, detail?}`

Import validation per storage-rules.md I-1 through I-9. Validates the parsed import envelope object (not the raw file bytes — I-10 size enforcement occurs before parsing at the surface layer). Checks version compatibility, structural completeness, genesis hash, seal chain presence, and seal chain linkage.

- **Params:** `raw` (parsed JSON object — the import envelope `{ ferrosVersion, exportedAt, profile, sealChain }`)
- **Returns:** `{ ok: true, code: null }` or `{ ok: false, code: string, detail?: string }`
- **Error codes:** `STORAGE_JSON_INVALID`, `STORAGE_SCHEMA_INCOMPLETE`, `STORAGE_VERSION_MISSING`, `STORAGE_VERSION_MISMATCH`, `STORAGE_GENESIS_MISSING`, `STORAGE_SEAL_CHAIN_MISSING`, `STORAGE_SEAL_CHAIN_INVALID_ROOT`, `STORAGE_SEAL_CHAIN_BROKEN`, `STORAGE_IDENTITY_MISSING`
- **Harnesses:** H2 (Group A — I-1..I-9)

### Permission

#### `canMutateDurableState(flags) → boolean`

Unified durable-write predicate (C9/C10). Returns `true` only when all of the following hold: `tradeWindowAccepted === true`, `sessionMode === false`, `aliasMode === false`, `recoveryMode === false`.

- **Params:** `flags` (`{ tradeWindowAccepted: boolean, sessionMode: boolean, aliasMode: boolean, recoveryMode: boolean }`)
- **Returns:** `boolean`
- **Harnesses:** H4 (Group E — E-1 deny session, E-2 deny alias, E-3 deny recovery, E-4 allow full-profile)

### Profile Persistence

#### `loadProfile() → {ok, code?, detail?, profile?, sealChain?}`

Shared localStorage loader for cross-surface consumers. Reads `ferros_profile`, falls back to `ferros_seal_chain` if `profile.sealChain` is absent, runs the same corruption checks described in C9, then applies the minimal optional-field normalizer used by the profile surface.

- **Returns:** `{ ok: true, profile, sealChain }` or `{ ok: false, code, detail? }`
- **Error codes:** `STORAGE_UNAVAILABLE`, `PROFILE_NOT_FOUND`, `STORAGE_JSON_INVALID`, `STORAGE_GENESIS_STAGE_MISMATCH`, `STORAGE_SEAL_COUNT_MISMATCH`, `STORAGE_LAST_SEAL_MISMATCH`
- **Consumers:** Schedule Ledger, Stream D consumer surfaces

#### `pushAuditEntry(profile, action, detail?) → {ok, code?, entry?, profile}`

Appends a bounded FIFO audit entry to `profile.auditTrail` using the profile-schema action enum (`seal-added`, `profile-saved`, `profile-imported`, `alias-claimed`, `recovery-claimed`). This is the in-profile audit ring buffer, not the portable C7 `.ferros-log` envelope.

- **Params:** `profile` (object), `action` (string), `detail?` (object | null)
- **Returns:** `{ ok: true, entry, profile }` or `{ ok: false, code: 'PROFILE_REQUIRED' | 'AUDIT_ACTION_INVALID', detail? }`
- **Consumers:** Personal Profile, consumer surfaces that append profile-scoped audit entries

#### `saveProfile(profile, options) → {ok, code?, detail?, profile?, sealChain?}`

Shared localStorage writer for consumer surfaces. Writes only when `canMutateDurableState(options.flags)` passes, validates the profile shape, updates `meta.lastModified` and `meta.revision`, writes both `ferros_profile` and `ferros_seal_chain`, and optionally appends a `profile-saved` audit entry.

- **Params:** `profile` (object), `options` (`{ flags, sealChain?, skipAudit?, auditDetail? }`)
- **Returns:** `{ ok: true, profile, sealChain }` or `{ ok: false, code, detail? }`
- **Error codes:** `PROFILE_REQUIRED`, `STORAGE_UNAVAILABLE`, `DURABLE_WRITE_FLAGS_REQUIRED`, `DURABLE_WRITE_FORBIDDEN`, `PROFILE_SHAPE_INVALID`, `STORAGE_QUOTA_EXCEEDED`
- **Consumers:** Schedule Ledger, Stream D consumer surfaces

### Runtime Messaging (A4 / C8)

#### `generateRuntimeNonce() → string`

Generates a 32-character hex nonce (128-bit) for per-asset message authentication. Uses `crypto.getRandomValues` when available, `Math.random` fallback on `file://`.

- **Returns:** 32-character hex string
- **Harnesses:** H3 (Group D — D-5 nonce handshake)

#### `validateRuntimeMessage(msg, expectedNonce) → boolean`

Returns `true` if `msg.nonce === expectedNonce`. Used by the host to reject messages from unauthorized frames.

- **Params:** `msg` (object), `expectedNonce` (string)
- **Returns:** `boolean`
- **Harnesses:** H3 (Group D — D-4 hostile frame rejection, D-5 valid nonce acceptance)

### Template Transform (C1 → C6)

#### `templateBlockToEvent(block, templateId, blockIndex, stream?) → object`

Transforms a single template schedule block (`{time, label}`) into a C6-conforming schedule event object: `{ id, kind:'block', label, time, source:{type:'template', templateId} }`. Optionally includes `stream` if provided.

- **Params:** `block` (`{time, label}`), `templateId` (string), `blockIndex` (number), `stream?` (string)
- **Returns:** C6 schedule event object
- **Harnesses:** H1 (Group 5 — template→event transformation)

#### `templateToEvents(template) → Array<object>`

Transforms an entire template's `templateSchedule.blocks` array into C6 schedule events by calling `templateBlockToEvent` for each block. Returns `[]` if the template has no blocks.

- **Params:** `template` (object with `{id, stream?, templateSchedule:{blocks:[{time,label}]}}`)
- **Returns:** `Array<object>` — C6 schedule event objects
- **Harnesses:** H1 (Group 5 — template→event transformation)

### Serialization

#### `serializeExport(profile, sealChain) → object`

Builds the canonical export envelope per C9 export rules.

- **Returns:** `{ ferrosVersion: string, exportedAt: string (ISO 8601), profile: object, sealChain: Array }`
- **Harnesses:** H2 (Group D — D-2 export envelope shape)

---

## Loading

```html
<script src="assets/_core/ferros-core.js"></script>
<script>
  // All methods available on window.FerrosCore
  var result = FerrosCore.validateImport(data);
</script>
```

No module system. No async loading. Script tag order matters — FerrosCore must load before consumers.

All harnesses use `<script src="../docs/assets/_core/ferros-core.js"></script>` (relative path from `harnesses/` directory).

---

## Harness Cross-Reference

| Harness | File | FerrosCore functions used |
|---------|------|--------------------------|
| H1 | `harnesses/ferros-contract-validator.html` | `computeHash`, `templateToEvents`, `templateBlockToEvent`, `VERSION`, `TEMPLATE_PROFILES` |
| H2 | `harnesses/round-trip-harness.html` | `validateImport`, `verifyChain`, `verifyChainFull`, `serializeExport`, `serializeAliasSessionLog`, `serializeRecoverySessionLog`, `validatePortableLog`, `applyPortableLogClaim`, `createSealEntry`, `hashWithAlgorithm`, `computeHash`, `VERSION`, `PORTABLE_LOG_XP_PER_ENTRY` |
| H3 | `harnesses/runtime-harness.html` | `generateRuntimeNonce`, `validateRuntimeMessage`, `VERSION` |
| H4 | `harnesses/negative-harness.html` | `canMutateDurableState`, `validateProfileShape`, `validatePortableLog`, `applyPortableLogClaim`, `VERSION`, `ALIAS_SESSION_STORAGE_KEY` |
| H5 | `harnesses/acceptance-harness.html` | `validateImport`, `createSealEntry`, `PORTABLE_LOG_XP_PER_ENTRY`, `TEMPLATE_PROFILES` |
| H6 | `harnesses/write-path-harness.html` | `validateProfileShape`, `applyPortableLogClaim`, `loadProfile`, `verifyChainFull`, `canMutateDurableState`, `VERSION` |
| H7 | `harnesses/semantic-fixture-linter.html` | `VERSION` |
| H8 | `harnesses/ui-acceptance-harness.html` | `VERSION` |
| H9 | `harnesses/consumer-helper-harness.html` | `loadProfile`, `pushAuditEntry`, `saveProfile`, `canMutateDurableState` |
