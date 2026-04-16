# FerrosCore API Contract

**Module:** `docs/assets/_core/ferros-core.js`  
**Pattern:** Classic-script IIFE → `window.FerrosCore`  
**Version:** 1.0  
**Dependencies:** None (zero external deps, file:// compatible)

---

## Properties

| Property | Type | Description |
|----------|------|-------------|
| `VERSION` | `string` | Current schema version (`"1.0"`) |
| `TEMPLATE_PROFILES` | `Array<object>` | Embedded template profiles (populated by `generate-ferros-core.ps1`) |

---

## Methods

### Hashing & Sealing

#### `computeHash(data) → Promise<{hash, algorithm}>`

Computes a hash of the string `data`. Uses SHA-256 via `crypto.subtle` when available (`https://`), falls back to djb2 on `file://`.

- **Returns:** `{ hash: string, algorithm: 'sha256' | 'djb2' }`

#### `hashWithAlgorithm(data, algorithm) → Promise<{ok, hash?, code?}>`

Deterministic hash helper for cross-context re-verification. Given a specific algorithm, produces the same hash or returns an error if the algorithm is unavailable.

- **Params:** `data` (string), `algorithm` (`'sha256'` | `'djb2'`)
- **Returns:** `{ ok: true, hash }` or `{ ok: false, code: 'SHA256_UNAVAILABLE' }`

#### `createSealEntry(taskId, taskData, previousSeal) → Promise<object>`

Creates a new seal chain entry with `{ taskId, seal, previousSeal, timestamp, data, hashAlgorithm, nonce }`. Generates a random nonce, hashes the payload, and returns the complete entry.

- **Params:** `taskId` (string), `taskData` (object), `previousSeal` (string | null, `"genesis"` for first)
- **Returns:** Seal entry object ready for `sealChain.push()`

### Chain Verification

#### `verifyChain(chain) → {valid, brokenAt?, error?}`

Verifies the integrity of a seal chain. Checks `previousSeal` linkage and performs deterministic re-hashing (using stored `hashAlgorithm` and `nonce`) to confirm each seal matches.

- **Params:** `chain` (Array of seal entries)
- **Returns:** `{ valid: true }` or `{ valid: false, brokenAt: number, error: string }`

### Profile Validation

#### `validateProfileShape(profile) → {ok, code, detail}`

Write-time profile shape validator (A1b). Hand-written whitelist checks. Validates:
- Top-level allowed + required properties
- `meta` allowed + required properties
- `journalEntry.type` enum (6 values)
- Achievement required fields (`id`, `name`)

- **Params:** `profile` (object)
- **Returns:** `{ ok: true, code: null }` or `{ ok: false, code: 'PROFILE_SHAPE_INVALID', detail: string }`

#### `validateImport(raw) → {ok, code?, detail?}`

Import validation per storage-rules.md I-1 through I-8. Checks version compatibility, structural completeness, genesis hash, seal chain presence.

- **Params:** `raw` (parsed JSON object — the import envelope)
- **Returns:** `{ ok: true }` or `{ ok: false, code: string, detail?: string }`

### Permission

#### `canMutateDurableState(flags) → boolean`

Checks whether the current session is allowed to write to localStorage. Requires all flags to pass: tradeWindowAccepted, sessionMode=false, aliasMode=false, recoveryMode=false.

- **Params:** `flags` (object with boolean properties: `tradeWindowAccepted`, `sessionMode`, `aliasMode`, `recoveryMode`)
- **Returns:** `boolean`

### Runtime Messaging (A4)

#### `generateRuntimeNonce() → string`

Generates a 32-character hex nonce (128-bit) for per-asset message authentication. Uses `crypto.getRandomValues` when available, `Math.random` fallback on `file://`.

#### `validateRuntimeMessage(msg, expectedNonce) → boolean`

Returns `true` if `msg.nonce === expectedNonce`. Used by the host to reject messages from unauthorized frames.

### Template Transform (C1)

#### `templateBlockToEvent(block, templateId, blockIndex, stream?) → object`

Transforms a single template schedule block (`{time, label}`) into a C6-conforming schedule event object with `{id, kind:'block', label, time, source:{type:'template', templateId}}`.

#### `templateToEvents(template) → Array<object>`

Transforms an entire template's `templateSchedule.blocks` array into C6 schedule events. Returns `[]` if the template has no blocks.

### Serialization

#### `serializeExport(profile, sealChain) → object`

Builds the canonical export envelope: `{ ferrosVersion, exportedAt, profile, sealChain }`.

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
