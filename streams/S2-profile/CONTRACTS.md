# S2 Profile & Identity — Contracts

---

## Contracts owned by S2

These are the cross-stream interfaces that S2 publishes. Other streams **must not** define their own identity or grant types — they consume these.

| Contract | Type | Location | Status |
|----------|------|----------|--------|
| `ProfileId` type | Rust type | `crates/ferros-profile/src/lib.rs` | ✅ Created |
| `KeyPair` type | Rust type | `crates/ferros-profile/src/lib.rs` | ✅ Created |
| `CapabilityGrant` type | Rust type | `crates/ferros-profile/src/lib.rs` | ✅ Created |
| `ConsentManifest` type | Rust type | `crates/ferros-profile/src/lib.rs` | ✅ Created |
| `SignedProfileDocument` type | Rust type | `crates/ferros-profile/src/lib.rs` | ✅ Created as a Rust-local signed envelope at v0; embedded-profile parity stays tied to `profile.v0.json` |
| `schemas/profile.v0.json` | JSON Schema | `schemas/profile.v0.json` | ✅ Frozen unsigned published v0 consumer contract; exercised by `ferros-profile` tests and the H1 validator |
| `schemas/capability-grant.v0.json` | JSON Schema | `schemas/capability-grant.v0.json` | ✅ Frozen signed envelope; exercised by `ferros-profile` tests |

---

## Current repo state

`schemas/profile.v0.json` is now the frozen S2-owned unsigned published v0 consumer contract referenced by the `ferros-profile` parity tests and the H1 contract validator. `KeyPair` now owns Ed25519 key generation plus local device labeling, derives `ProfileId` from the verifying key, and signs the additive `SignedProfileDocument` envelope without mutating the base `ProfileDocument` or `profile.v0.json` consumer contract. The signed profile payload is reconstructed from `profile_id`, the canonicalized `profile` object, and the optional `revoked_at` / `revocation_reason` fields; `signer_public_key` and `signature` remain envelope-only fields and are never part of the signed payload. `SignedProfileDocument` remains a Rust-local v0 contract and focused test surface rather than a published signed-profile schema, and the signed-profile fixture now revalidates its embedded `profile` payload against `profile.v0.json` to keep that boundary honest.

`schemas/capability-grant.v0.json` continues to freeze the signed grant envelope contract, `schemas/fixtures/grant-valid.json` anchors the happy path, `schemas/fixtures/grant-invalid-sig.json` anchors invalid-signature rejection, and `SignedCapabilityGrant` preserves the current flattened envelope shape without changing the runtime `CapabilityGrant` or `CapabilityGrantView` consumer boundary.

The grant verification contract is now published in `schemas/capability-grant.v0.json`: independent verifiers must rebuild the signed payload from only `profile_id`, `capability`, and optional `revoked_at` and `revocation_reason`, emit UTF-8 JSON with no insignificant whitespace in exactly that member order, verify the Ed25519 signature with `signer_public_key`, and only then trust the embedded grant fields. `signer_public_key` and `signature` are envelope-only fields and are never part of the signed payload.

---

## Contracts consumed by S2

| Contract | Source | Purpose |
|----------|--------|---------|
| Cargo workspace | S1 | Build and CI |
| `rust-toolchain.toml` | S1 | Toolchain pin |

---

## Schema freeze policy

`profile.v0.json` is now frozen as the unsigned published v0 consumer contract and must not be mutated in place. The additive `SignedProfileDocument` path deliberately stays Rust-local at v0 and is enforced through parity coverage that revalidates the embedded `profile` payload against `profile.v0.json`; if signed-profile portability is needed later, it must publish a separate versioned schema rather than widen `profile.v0.json` in place. `capability-grant.v0.json` is now frozen at the stripped-payload signed-envelope contract described above and must not be mutated in place; new grant fields go into a `v1` schema with explicit migration rules. See `ROADMAP.md` — coordination rules.

---

## Downstream consumers

| Stream | What it consumes |
|--------|-----------------|
| S3 Agent Center | `ProfileId`, `CapabilityGrant` — agents require a granted profile to spawn |
| S4 Runtime | `CapabilityGrant` — consent bus enforces grants at the policy engine |
| S7 Hub | `ProfileId` — pairing flow creates device-bound profile + issues grants |
