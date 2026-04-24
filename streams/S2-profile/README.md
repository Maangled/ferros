# S2 — Profile & Identity

**Stream:** S2  
**Status:** 🟡 Active (G2 closed)  
**Gate:** G2

---

## Mission

Give every FERROS entity a locally-sovereign cryptographic identity. The profile is the root of consent. Every downstream stream that touches permissions, agents, or the hub takes a `ProfileId` or `CapabilityGrant` as input. Ship the types and file format early so downstream streams can code against stubs.

This stream is intentionally insulated from raw legacy-repo input: the G2 identity and consent contracts must be authored from FERROS invariants, not copied from pre-FERROS systems.

---

## Scope

- `ferros-profile` crate:
  - Ed25519 keypair, device-bound, optional passphrase wrap.
  - `ProfileId` type (derived from public key).
  - Signed capability grants + consent manifests.
  - Profile storage abstraction (filesystem-first; `sled`/`redb` later).
- Schema: `schemas/profile.v0.json` and `schemas/capability-grant.v0.json`.
- CLI subcommands: `ferros profile init | show | export | import | grant | revoke`.

---

## Out of scope

- Agent spawning or registration (S3).
- Runtime bus or executor (S4).
- UI for the profile (S5).
- Hardware pairing (S7).

---

## Dependencies

- **S1:** G1 is closed; the workspace and CI foundation already exist.
- **S6:** External prior art may inform downstream implementation only after S6 publishes ADRs. The frozen S2 v0 boundary remains FERROS-authored rather than copied from pre-FERROS systems.

---

## What this stream blocks

- **S3 Agent Center:** agents reference `ProfileId` and `CapabilityGrant`.
- **S4 Runtime:** consent bus enforces `CapabilityGrant`; needs the type.
- **S7 Hub:** pairing flow is profile + device → signed capability grants.
- **S8 Docs:** identity model docs depend on S2 types being stable.

---

## Definition of done (G2)

- [x] `ferros-profile` crate builds and passes `cargo test` locally.
- [x] `CapabilityGrant` sign → serialize → verify → revoke works end-to-end with Ed25519 and re-signing on revoke.
- [x] Profile round-trips: create → serialize → sign → verify → revoke.
- [x] `schemas/profile.v0.json` frozen as the unsigned published v0 consumer contract.
- [x] `schemas/capability-grant.v0.json` frozen as the stripped-payload signed envelope contract.
- [x] Rust/schema parity is enforced with a fixture-backed contract test against `schemas/profile.v0.json`.
- [x] CLI: `ferros profile init | show | export | import | grant | revoke` all functional through the real `ferros` binary against temp-file-backed local state; `show` stays on the frozen unsigned `profile.v0.json` boundary and persisted revoked grant state stays within the frozen grant boundary.
- [x] At least one golden fixture in `schemas/fixtures/` for a valid profile and a valid grant.
- [x] Negative fixture: invalid signature rejected.

---

## Likely crates / files

| Path | Role |
|------|------|
| `crates/ferros-profile/` | Identity crate |
| `schemas/profile.v0.json` | Frozen S2-owned unsigned published v0 consumer contract exercised by `ferros-profile` tests and H1 |
| `schemas/fixtures/minimal-stage0-profile.json` | Existing Stage 0 profile fixture used for serde and schema parity tests |
| `schemas/capability-grant.v0.json` | Frozen signed grant envelope schema exercised by `ferros-profile` tests |
| `schemas/fixtures/grant-valid.json` | Golden happy-path signed grant fixture |
| `schemas/fixtures/grant-invalid-sig.json` | Negative signed grant fixture with an invalid signature |

---

## Immediate next steps

1. Keep `SignedProfileDocument` Rust-local at v0 unless downstream portability needs a separate versioned signed-profile schema.
2. Hold the frozen `schemas/profile.v0.json` and `schemas/capability-grant.v0.json` v0 boundaries steady for downstream consumers.
3. Tighten parity or local CLI coverage only where it reinforces those frozen v0 contracts without widening them in place.
