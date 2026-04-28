# S2 Research Note - Profile Recovery UX Runway

**Date:** 2026-04-28
**Owning stream:** S2 primary; S5 consumer awareness
**Output feeds:** S5 profile surface planning; D1 operator scripts
**Status:** Runway note only. Frozen schemas are not changed.

---

## Purpose

This note defines what "profile recovery" can honestly mean today. S2 has a closed G2 boundary and a real CLI lifecycle: `init`, `show`, `export`, `import`, `grant`, and `revoke`. The recovery UX must stay inside that boundary until a later storage or key-wrap decision exists.

---

## Current Recovery Building Blocks

| Building block | Current meaning |
|---|---|
| `ferros profile show` | Read the current unsigned profile document |
| `ferros profile export <path>` | Write a local export bundle/document for operator-controlled backup |
| `ferros profile import <path>` | Import a local bundle/document and validate it before use |
| Rollback-safe import | Invalid grant state should not leave partial local profile artifacts behind |
| Frozen profile schema | `schemas/profile.v0.json` stays the unsigned published consumer contract |
| Frozen grant schema | `schemas/capability-grant.v0.json` stays the signed grant envelope contract |

Recovery is local file recovery. It is not cloud sync, social login, remote restore, or marketplace account recovery.

---

## UX Principles

1. **Show before import.** The operator should be able to inspect the current profile and the import candidate before replacing local state.
2. **Export is explicit.** No automatic backup claim exists today.
3. **Import is local-only.** A path chosen by the operator is the source; there is no remote account lookup.
4. **Schema stays frozen.** Recovery UI must not add profile fields or mutate v0 schemas.
5. **Failure must be calm.** If import fails validation, the UX should say the previous local state was preserved.

---

## Minimum S5 Surface Shape

When S5 implements the profile surface, the recovery portion can be four slots:

| Slot | Backing command | User-facing intent |
|---|---|---|
| Current profile | `ferros profile show` | "What profile is active here?" |
| Export | `ferros profile export <path>` | "Save a local copy to a path I choose." |
| Import preview | read candidate path before commit | "Show me what I am about to restore." |
| Import commit | `ferros profile import <path>` | "Replace local state only after validation." |

Grant mutation and revoke remain out of scope for the first browser profile surface.

---

## Recovery States

| State | UX copy should communicate |
|---|---|
| No local profile | Run `init` or import a local bundle |
| Current profile loaded | Profile id and created-at are visible |
| Export succeeded | Local file path is shown |
| Import candidate invalid | Import rejected; existing state preserved |
| Import succeeded | New profile id is active and can be shown |
| Grant state invalid | Import rejected or rolled back; do not partially trust imported grants |

---

## D1 Relationship

D1 does not require a full browser recovery UI. D1 needs `profile init` and `profile show` to work on the target device, and profile state must survive a shell reopen and power cycle.

This note still helps D1 because it gives the operator language for export/import rehearsals:

- export can be used to capture the profile artifact before reboot;
- import can be used in a scripted rehearsal only if the transcript makes clear it is not the D1 power-cycle proof;
- after power cycle, `profile show` must work without re-importing.

---

## Out of Scope

- New profile schema version.
- Passphrase wrapping or key recovery.
- Remote backup account.
- Browser-issued grant/revoke.
- Multi-device sync.
- D1 or G4 evidence closure.

---

## Batch F Input

Batch F can use this note to draft the S5 profile surface wireframe. The wireframe should keep the recovery UX as a local, explicit, reversible file operation until S2 publishes a broader storage or sync contract.

