# ADR-012: Schema Evolution and Sub-Schema Cascade

## Status
Accepted

## Context

During the Phase 0 audit (remediation plan v3, commit f5cab8c), every schema violation found in `profile.schema.json` followed the same pattern:

1. A new feature adds a field to a **nested sub-schema** (e.g., `meta`, `journalEntry`, `achievement`).
2. The sub-schema has `additionalProperties: false`.
3. The code writes the field successfully (JavaScript doesn't enforce JSON Schema at runtime).
4. Existing tests continue to pass because no harness validates the **write output** against the schema.
5. The violation is invisible until a round-trip import or an explicit shape-check rejects the data.

The audit found **14 undeclared fields** across 4 sub-schemas and 4 top-level properties, all caused by this cascade:

| Sub-schema | Undeclared fields | Root cause |
|---|---|---|
| top-level properties | schedule, completions, creditLog, bag | `migrateProfileStructure()` added V2 fields |
| meta | schemaVersion, claimedAliasSessions, xp, sealBroken, revision | Multiple features added meta fields |
| journalEntry.type enum | claim-event, claimed-alias, claimed-recovery | Alias claim flow added new entry types |
| journalEntry optional fields | aliasId, aliasName, linkedTo, claimId, sealBroken | Claim entries carry linking data |
| achievement required fields | Code wrote {id, name, earnedAt}; schema required {id, name, desc, icon, unlocked, unlockedAt} | Misread of schema during implementation |

## Decision

### Rule 1: Schema-first development

Any feature that writes a new field to a JSON Schema-governed object **must** update the schema **before** or **simultaneously with** the code change. The schema is the source of truth, not the code.

### Rule 2: Sub-schema cascade checklist

When adding a field to a sub-schema that has `additionalProperties: false`:

1. **Add the property** to the sub-schema's `properties` object.
2. **Decide on `required`** — add to the `required` array only if every existing document must have this field. Prefer optional with a `default` for backward compatibility.
3. **Update type unions** — if the field is nullable, use `["string", "null"]` not just `"string"`.
4. **Update enum arrays** — if the field introduces a new enum value (e.g., journalEntry.type), add to the enum declaration.
5. **Add or update a fixture** — at minimum, one golden fixture must exercise the new field.
6. **Run write-path harness** — `FerrosCore.validateProfileShape()` must accept a document with the new field.
7. **Run semantic linter** — H7 must pass with the new fixture.

### Rule 3: Normalization over migration

For **optional** fields with defaults (e.g., `revision: 0`, `xp: 0`, `claimedAliasSessions: []`), prefer load-time normalization over formal migration:

```javascript
// Shape normalizer — runs after JSON.parse, before corruption checks
if (typeof profile.meta.revision !== 'number') profile.meta.revision = 0;
if (!Array.isArray(profile.meta.claimedAliasSessions)) profile.meta.claimedAliasSessions = [];
```

This approach:
- Avoids a migration engine in Phase 0
- Handles both legacy profiles and new profiles identically
- Is explicitly documented in `storage-rules.md`

For **structural** changes (adding a new required top-level key, changing a required field's type), use `migrateProfileStructure()` with a `schemaVersion` guard.

### Rule 4: Write-time validation as safety net

`FerrosCore.validateProfileShape()` is a hand-written whitelist validator that runs on every `saveProfile()` call. It catches:
- Unknown top-level keys
- Unknown meta keys
- Invalid journalEntry.type values
- Missing achievement required fields

This is a **runtime safety net**, not a substitute for Rule 1. It catches cascade violations that slip through code review.

### Rule 5: Fixture coverage for every sub-schema

Each sub-schema with `additionalProperties: false` must have at least one golden fixture that exercises **all** of its optional fields. The semantic fixture linter (H7) validates this.

Current coverage:

| Sub-schema | Fixture exercising all fields |
|---|---|
| profile (top-level) | full-profile-stage3 |
| meta | profile-template-archetype-seam |
| identity | full-profile-stage3 |
| achievement | full-profile-stage3 |
| journalEntry | profile-template-archetype-seam (claim entry with aliasId, aliasName, claimId) |
| seal | full-profile-stage3 |
| card | card-deck-roundtrip, deck-card-assembly-seam |
| deck | deck-card-assembly-seam |
| schedule-event | schedule-event-source-seam |
| template | maximum-template-schedule |

## Consequences

### Positive
- Schema violations are caught at development time (Rule 1) and at runtime (Rule 4).
- The cascade checklist (Rule 2) prevents the most common class of schema drift.
- Normalization (Rule 3) keeps backward compatibility simple without a migration framework.
- Fixture coverage (Rule 5) ensures the linter can detect regressions.

### Negative
- `validateProfileShape()` must be updated in sync with schema changes — it's a hand-written mirror of the schema's structure. This is intentional for Phase 0 (no JSON Schema validator library), but should be revisited in Phase 1.
- The 7-step checklist adds friction to "quick" field additions. This friction is the point — it prevents the exact class of bugs found in the audit.

### Risks
- If a future change adds a deeply nested sub-schema (e.g., `schedule.archetype.config`) without `additionalProperties: false`, the cascade checklist won't catch violations at that level. Mitigation: apply `additionalProperties: false` to all new sub-schemas by default.
