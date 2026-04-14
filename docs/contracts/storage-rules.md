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

**Hard constraint:** No mode other than Full Profile may write to `localStorage`. This is enforced at `saveProfile()` which has a guard at the top: `if (recoveryMode || sessionDeclined) return;`. This guard **must never be removed**.

---

## Profile Schema Versioning

- Every profile in `localStorage` and every exported `.json` must have `meta.version` in `"MAJOR.MINOR"` semver format (e.g., `"1.0"`, `"2.0"`).
- The current schema version is **`2.0`** (profile.schema.json v2.0).
- `meta.version` is read on import to determine compatibility.

### Version Compatibility Matrix

| Stored version | Current version | Action |
|---|---|---|
| Same | Same | Accept |
| Older MAJOR | Current | Reject with `STORAGE_VERSION_MISMATCH` — no silent migration in Wave 0 |
| Newer MAJOR | Current | Reject with `STORAGE_VERSION_FUTURE` — cannot read future format |
| Missing `meta.version` | — | Reject with `STORAGE_VERSION_MISSING` |

**Wave 0 rule:** No automatic migration. Import must reject any profile where `meta.version` does not exactly match the current schema version. This rule may be relaxed in H1 of the Hardening wave.

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

**On rejection:** Surface a human-readable error message containing the error code. Do not write any partial data to storage. Do not modify any existing profile data.

---

## Export Rules

- Export must serialize the current profile from `localStorage` (`ferros_profile`).
- The export envelope must include: `ferrosVersion`, `exportedAt` (ISO timestamp), `profile` (the full profile object).
- The export file name must match the format: `ferros-profile-[sanitized-name]-[YYYY-MM-DD].json`.
- `sanitized-name` = name lowercased, spaces replaced with `-`, non-alphanumeric removed.
- `.ferros-log` files (alias/recovery) follow the naming rules in ADR-005.

---

## Corruption Detection

A profile in `localStorage` is considered **corrupt** if any of the following are true at load time:

1. `ferros_profile` is non-null but fails `JSON.parse()`.
2. `meta.genesisHash` is null and `meta.stage` > 0.
3. `sealChain` array length does not equal `meta.sealCount`.
4. `sealChain[n].seal` !== `meta.currentSeal` where n is the last seal.

On corrupt detection:
- Show a recovery UI prompt. Do not overwrite.
- Do not proceed to normal profile load.
- Offer: (a) clear storage and start fresh, or (b) download the raw corrupt data for manual recovery.

---

## Cleanup Rules

- `localStorage.removeItem('ferros_profile')` and `localStorage.removeItem('ferros_seal_chain')` must only be called together (atomic conceptual pair), never independently.
- `sessionStorage` may be cleared independently per key.
- Harnesses that clear storage between test runs must remove both keys, never just one.

---

## Wave 0 Enforcement Points

| Rule | Enforced By |
|---|---|
| Import rejection I-1 through I-10 | H2 round-trip-harness.html |
| Seal chain broken on import (I-6) | H2 round-trip-harness.html |
| Session mode does not write to localStorage | H4 negative-harness.html |
| Recovery mode writes nothing to any storage | H4 negative-harness.html |
| Export envelope has required fields | H2 round-trip-harness.html |
