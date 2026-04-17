# Contract: Storage Rules

**ID:** C9
**Version:** 1.0
**Status:** Active
**Last updated:** 2026-04-13
**Depends on:** ADR-001 (Progression-Lock), ADR-005 (Session Modes)
**Enforced by:** H2 (round-trip-harness.html), H4 (negative-harness.html)

---

## Purpose

This contract defines exactly where FERROS data lives, how it is versioned, how import/export is handled, and what constitutes a valid or invalid storage operation. It is **enforced behavior**, not documentation. Harnesses that breach these rules must fail loudly.

---

## Storage Locations

| Location | Key | Owner | Session Modes That Write |
|---|---|---|---|
| `localStorage` | `ferros_profile` | Profile module | Full Profile only |
| `localStorage` | `ferros_seal_chain` | Seal module | Full Profile only |
| `sessionStorage` | `session_declined` | Session gate | Session Mode only |
| `sessionStorage` | `ferros_alias_session` | Alias module | Alias Mode only |
| JS variables (no storage) | — | Recovery module | Recovery Mode only |
| Exported file (`.json`) | Profile export envelope | Portability panel | Full Profile only |
| Exported file (`.ferros-log`) | Audit record | Log export | Alias Mode, Recovery Mode |

**Hard constraint:** No mode other than Full Profile may write to `localStorage`.

Current enforcement is via a unified durable-write predicate (`canMutateDurableState`) that requires all of the following:

- Trade Window consent accepted (`_tradeWindowAccepted === true`)
- `sessionMode === false`
- `aliasMode === false`
- `recoveryMode === false`

`saveProfile()`, `addSeal()`, import confirmation, and claim confirmation all gate mutations through this predicate. This guard model **must never be removed**.

---

## Profile Schema Versioning

- Every profile in `localStorage` and every exported `.json` must have `meta.version` in `"MAJOR.MINOR"` semver format (e.g., `"1.0"`).
- The current schema version is **`1.0`** (Phase 0 freeze).
- `meta.version` is the single canonical version field. The internal `schemaVersion` marker used by the monolith's structural migration is an implementation detail, not a schema-level concept.
- `meta.version` is read on import to determine compatibility.

### Version Compatibility Matrix

| Stored version | Current version | Action |
|---|---|---|
| `"1.0"` | `"1.0"` | Accept |
| Older MAJOR (e.g. `"0.9"`) | `"1.0"` | Reject with `STORAGE_VERSION_MISMATCH` — no silent migration in Wave 0 |
| Newer MAJOR (e.g. `"2.0"`) | `"1.0"` | Reject with `STORAGE_VERSION_FUTURE` — cannot read future format |
| Missing `meta.version` | — | Reject with `STORAGE_VERSION_MISSING` |

**Wave 0 rule:** No automatic migration on import. Import must reject any profile where `meta.version` does not exactly match `"1.0"`. Post-Phase-0 upgrades to `"2.0"` require a dedicated offline upgrader/export-upgrader tool.

### Post-Phase-0 Version Roadmap

| Version | Target | Prerequisite |
|---|---|---|
| `"1.0"` | Phase 0 exit (current) | — |
| `"2.0"` | Post-exit, after shared runtime core and real upgrader tool ship | Real export/import upgrader, compatibility tests |

---

## Import Validation Rules

The following conditions cause import to be **hard rejected** (import must fail with an explicit error code, not silently degrade):

| # | Condition | Error Code |
|---|---|---|
| I-1 | `meta.version` field absent | `STORAGE_VERSION_MISSING` |
| I-2 | `meta.version` MAJOR does not match current | `STORAGE_VERSION_MISMATCH` |
| I-3 | `meta.genesisHash` absent or null | `STORAGE_GENESIS_MISSING` |
| I-4 | `sealChain` absent or not an array | `STORAGE_SEAL_CHAIN_MISSING` |
| I-5 | `sealChain[0].previousSeal !== "genesis"` | `STORAGE_SEAL_CHAIN_INVALID_ROOT` |
| I-6 | Any `sealChain[i].previousSeal !== sealChain[i-1].seal` for i > 0 | `STORAGE_SEAL_CHAIN_BROKEN` |
| I-7 | `identity.name` absent or empty | `STORAGE_IDENTITY_MISSING` |
| I-8 | Top-level required fields absent: `meta`, `identity`, `attributes`, `sealChain` | `STORAGE_SCHEMA_INCOMPLETE` |
| I-9 | JSON parse fails | `STORAGE_JSON_INVALID` |
| I-10 | Import file exceeds 2 MB | `STORAGE_SIZE_EXCEEDED` |

**I-10 scope note:** Size enforcement occurs at the file/blob import boundary before JSON parse. `FerrosCore.validateImport(raw)` validates parsed objects and therefore cannot enforce byte-size limits by itself.

**On rejection:** Surface a human-readable error message containing the error code. Do not write any partial data to storage. Do not modify any existing profile data.

---

## Export Rules

- Export **must read from `localStorage`** (the last persisted state), not from in-memory variables. This ensures transactional consistency — the export always represents the most recently saved profile, not transient in-memory mutations.
- The export envelope must include exactly: `{ ferrosVersion, exportedAt, profile, sealChain }`.
  - `ferrosVersion`: the current schema version string (e.g., `"1.0"`).
  - `exportedAt`: ISO 8601 timestamp of the export moment.
  - `profile`: the full profile object.
  - `sealChain`: the full seal chain array (mirrors `profile.sealChain` for backwards compatibility but is the authoritative copy).
- The export file name must match the format: `ferros-profile-[sanitized-name]-[YYYY-MM-DD].json`.
- `sanitized-name` = name lowercased, spaces replaced with `-`, non-alphanumeric removed.
- `.ferros-log` files (alias/recovery) follow the naming rules in ADR-005.
- **Phase 0:** Profile exports do not include cards or decks. Card/deck portability is a separate concern addressed in Wave 1+ (V5-V7).

## Seal Entry Metadata

Each seal chain entry must include the following fields:

| Field | Type | Description |
|---|---|---|
| `taskId` | string | Identifier for the sealed action |
| `seal` | string | Computed hash value |
| `previousSeal` | string | `"genesis"` for first seal, otherwise prior seal's hash |
| `timestamp` | string (ISO 8601) | When the seal was created |
| `data` | object | Arbitrary task-specific payload |
| `hashAlgorithm` | string (`"sha256"` or `"djb2"`) | Algorithm used to compute the seal hash |
| `nonce` | integer | Random nonce consumed during computation, persisted for deterministic re-verification |

**Rationale:** Storing `hashAlgorithm` and `nonce` alongside every seal enables cross-context re-verification. Without them, a seal computed with SHA-256 on `https://` cannot be deterministically re-verified on `file://` (where only djb2 is available), because the random nonce used in computation would be lost.

### Seal Chain Compaction (Future)

Phase 0 stores the full seal chain with no compaction. Checkpoint/compaction (e.g., snapshot every N seals) is deferred to Wave 4 hardening (H5 performance budget).

---

## Corruption Detection

A profile in `localStorage` is considered **corrupt** if any of the following are true at load time:

1. `ferros_profile` is non-null but fails `JSON.parse()`. → `STORAGE_JSON_INVALID`
2. `meta.genesisHash` is null and `meta.stage` > 0. → `STORAGE_GENESIS_STAGE_MISMATCH`
3. `sealChain` array length does not equal `meta.sealCount` (skip if `sealCount` is undefined for v1.0 backwards compatibility). → `STORAGE_SEAL_COUNT_MISMATCH`
4. `sealChain[n].seal` !== `meta.currentSeal` where n is the last seal. → `STORAGE_LAST_SEAL_MISMATCH`

**Enforcement scope:** These checks are enforced in `loadProfile()` at application startup — not only on import. Every time a profile is loaded from `localStorage`, all four corruption checks run before the profile is accepted into memory.

On corrupt detection:
- Show a recovery UI prompt. Do not overwrite.
- Do not proceed to normal profile load. `loadProfile()` returns `false`.
- Offer: (a) clear storage and start fresh, or (b) download the raw corrupt data for manual recovery.

### Load-Time Shape Normalization

After corruption checks pass, `loadProfile()` applies a minimal shape normalizer to default missing optional meta fields for backwards compatibility with v1.0 profiles:

| Field | Default | Reason |
|---|---|---|
| `meta.revision` | `0` | Added in A1a for cross-tab coordination (B1) |
| `meta.xp` | `0` | Added in A1a for alias claim XP tracking |
| `meta.claimedAliasSessions` | `[]` | Added in A1a for alias dedupe |
| `meta.sealBroken` | `false` | Added in A1a for seal integrity flag |
| `meta.schemaVersion` | `1` | Added in A1a for structural migration guard |

This normalizer is **not** a migration engine — it only provides safe defaults for recently-added optional fields so that downstream code does not need null checks.

### Seal Chain Load Priority

As of A2, `profile.sealChain` is the **canonical** source for the seal chain at load time. The separate `ferros_seal_chain` localStorage key is used as a **fallback only** — if `profile.sealChain` is missing or not an array, `loadProfile()` attempts to read `ferros_seal_chain` before resorting to an empty array.

At write time, `saveProfile()` continues to write both `profile.sealChain` (as part of the profile envelope) and `ferros_seal_chain` (as a backup key) for one version cycle. The backup key will be removed in a future version.

---

## Cleanup Rules

- `localStorage.removeItem('ferros_profile')` and `localStorage.removeItem('ferros_seal_chain')` must only be called together (atomic conceptual pair), never independently.
- `sessionStorage` may be cleared independently per key.
- Harnesses that clear storage between test runs must remove both keys, never just one.

---

## Cross-Tab Coordination (B1)

FERROS uses the browser's `storage` event to coordinate between tabs. The `storage` event fires in all **other** tabs when `localStorage` is modified — it does not fire in the tab that wrote the change.

### Revision Counter

`meta.revision` is an integer counter incremented by `saveProfile()` on every write. It serves as the canonical ordering mechanism for cross-tab conflict detection.

### Reconciliation Rules

When a `storage` event fires with `key === "ferros_profile"`:

1. Parse the incoming profile from `event.newValue`.
2. Compare `incoming.meta.revision` against `local.meta.revision`.
3. If `incoming > local`: reload the profile via `loadProfile()`, refresh the UI, show a notification.
4. If `incoming <= local`: ignore — the local tab has a newer or equal version.

**Conflict resolution:** Last-write wins. There is no merge logic. Two tabs writing simultaneously will produce the highest revision, and the `storage` event will propagate that revision to the other tab.

**Phase 0 limitation:** No real-time locking or distributed consensus. The revision counter approach is sufficient for a single-user, multi-tab scenario.

---

## Wave 0 Enforcement Points

| Rule | Enforced By |
|---|---|
| Import rejection I-1 through I-9 (FerrosCore-validated; I-10 size check enforced at surface before parse) | H2 round-trip-harness.html (Group A) |
| Seal chain broken on import (I-6) | H2 round-trip-harness.html (Group A) |
| True export → clear → import → assert round-trip | H2 round-trip-harness.html (Group D) |
| Session mode does not write to localStorage | H4 negative-harness.html (Group A + E) |
| Recovery mode writes nothing to any storage | H4 negative-harness.html (Group A + E) |
| Export envelope has required fields (`ferrosVersion`, `exportedAt`, `profile`, `sealChain`) | H2 round-trip-harness.html (Group D) |
| Write-time profile shape validation (A1b) | H6 write-path-harness.html |
| Load-time corruption detection (A2) | H6 write-path-harness.html |
| Load-time shape normalization (A2) | H6 write-path-harness.html |

---

## `.ferros-log` Envelope Format

Alias and recovery session logs exported as `.ferros-log` files follow this canonical envelope:

```json
{
  "ferrosVersion": "1.0",
  "logType": "alias-session | recovery-session",
  "sessionId": "<deterministic session identifier>",
  "alias": { "id": "...", "name": "...", "icon": "...", "class": "...", "attribution": "unlinked" },
  "sessionStart": "<ISO 8601>",
  "sessionEnd": "<ISO 8601>",
  "entries": [ ... ],
  "claimInstructions": "..."
}
```

### `sessionId` Semantics

- `sessionId` is the **canonical claim-identity carrier**. It uniquely identifies this session log for de-duplication during claim.
- **Uniqueness:** `sessionId` must be unique per session. Generated deterministically from `alias.id + sessionStart` or equivalent.
- **Claim check:** On claim, the receiving profile checks `profile.meta.claimedAliasSessions` (array of previously claimed `sessionId` values). Duplicate `sessionId` → reject with `CLAIM_DUPLICATE_SESSION`.
- **Audit records** may mirror `sessionId` secondarily, but the `.ferros-log` envelope is the primary carrier (not `audit-record.schema.json`).
- See also C10 for consent/deny semantics on claim operations.

---

## Schedule-Event Schema Positioning

The `schedule-event.schema.json` (C6) is a **future-consumer-only** schema in Phase 0. No runtime code currently produces or consumes C6-formatted events. Transformation from internal schedule block format to C6 event format is required before Wave 2 gate S1.

---

## Card/Deck Export Exclusion

Phase 0 exports do not include cards or decks. There are no placeholder fields for card/deck data in the export envelope. Card/deck portability is a separate concern addressed in Wave 1+ (V5-V7). See ADR-010 for card/deck nomenclature.
