# S2 Profile & Identity — Progress

Reverse-chronological. Append a dated entry at the top per session.

---

## 2026-04-23 — Signed CapabilityGrant verification path landed

- Added the first signed and verifiable `CapabilityGrant` envelope path to `ferros-profile`, including explicit Ed25519 verify and re-sign-on-revoke behavior.
- Froze the stripped JSON signing contract in `schemas/capability-grant.v0.json`: `profile_id`, `capability`, and optional `revoked_at` / `revocation_reason` are serialized in that order with no insignificant whitespace, while `signer_public_key` and `signature` stay envelope-only.
- Added `schemas/fixtures/grant-valid.json` and `schemas/fixtures/grant-invalid-sig.json`, plus tests for canonical payload examples, happy-path verification, invalid-signature rejection, and revoke round-trip coverage.
- G2 remains open: key material, `schemas/profile.v0.json` freeze, and the `ferros profile ...` CLI flows are still outstanding.

## 2026-04-23 — Filesystem profile store slice landed

- Added the first `ProfileStore` abstraction to `ferros-profile`.
- Added `FileSystemProfileStore` as the filesystem-first implementation for loading and saving `ProfileDocument` values.
- Covered the storage slice with a filesystem round-trip test against the existing minimal Stage 0 fixture.

## 2026-04-23 — Consent and revocation slice landed

- Added `ConsentManifest` to `ferros-profile` as the first FERROS-owned consent grouping surface over `CapabilityGrant` values.
- Extended `CapabilityGrant` with revocation metadata plus idempotent revoke semantics.
- Expanded grant tests to cover revocation, manifest profile matching, duplicate capability rejection, and active-grant filtering.
- Updated the draft grant schema so the new revocation fields stay within the current G2 freeze candidate boundary.

## 2026-04-23 — CapabilityGrant schema slice landed

- Added `schemas/capability-grant.v0.json` as the first S2-owned draft grant schema, mirroring the current `CapabilityGrant` serde boundary.
- Added `schemas/fixtures/grant-valid.json` as a golden happy-path grant fixture.
- Extended `ferros-profile` with a no-new-dependency structural contract test that round-trips the grant fixture and checks it against the draft grant schema.

## 2026-04-22 — Foundation slice landed in `ferros-profile`

- Added `crates/ferros-profile/` as a real workspace member.
- Implemented `ProfileId` and `CapabilityGrant` as compile-tested Rust types.
- Added a first fixture-backed `ProfileDocument` serde model that parses `schemas/fixtures/minimal-stage0-profile.json`.
- Added round-trip JSON tests against the minimal Stage 0 profile fixture.
- Stream remains blocked on G1 closure for meaningful downstream work, but the crate is no longer hypothetical.

## 2026-04-21 — Stream scaffolded

- Stream directory and planning documents created.
- Blocked on G1 (S1 Foundation).
- Types and schema design can begin in parallel with S1 if needed; crate can be added to workspace once S1 lands.
