# S2 Profile & Identity — Contracts

---

## Contracts owned by S2

These are the cross-stream interfaces that S2 publishes. Other streams **must not** define their own identity or grant types — they consume these.

| Contract | Type | Location | Status |
|----------|------|----------|--------|
| `ProfileId` type | Rust type | `crates/ferros-profile/src/lib.rs` | ✅ Created |
| `CapabilityGrant` type | Rust type | `crates/ferros-profile/src/lib.rs` | ✅ Created |
| `ConsentManifest` type | Rust type | `crates/ferros-profile/src/lib.rs` | ⬜ Not yet created |
| `schemas/profile.v0.json` | JSON Schema | `schemas/profile.v0.json` | 🟡 Drafted; exercised by `ferros-profile` tests |
| `schemas/capability-grant.v0.json` | JSON Schema | `schemas/capability-grant.v0.json` | 🟡 Drafted; exercised by `ferros-profile` tests |

---

## Current repo state

`schemas/profile.v0.json` and `schemas/capability-grant.v0.json` are the S2-owned draft freeze candidates currently referenced by the `ferros-profile` test suite. `schemas/fixtures/grant-valid.json` now anchors the grant happy path while signed and revocable grant fields remain future work.

---

## Contracts consumed by S2

| Contract | Source | Purpose |
|----------|--------|---------|
| Cargo workspace | S1 | Build and CI |
| `rust-toolchain.toml` | S1 | Toolchain pin |

---

## Schema freeze policy

`profile.v0.json` now exists as the draft freeze candidate. Once `profile.v0.json` and `capability-grant.v0.json` are frozen (G2), they **must not** be mutated in place. New fields go into `v1` schemas with explicit migration rules. See `ROADMAP.md` — coordination rules.

---

## Downstream consumers

| Stream | What it consumes |
|--------|-----------------|
| S3 Agent Center | `ProfileId`, `CapabilityGrant` — agents require a granted profile to spawn |
| S4 Runtime | `CapabilityGrant` — consent bus enforces grants at the policy engine |
| S7 Hub | `ProfileId` — pairing flow creates device-bound profile + issues grants |
