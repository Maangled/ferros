# S2 Profile & Identity — Progress

Reverse-chronological. Append a dated entry at the top per session.

---

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
