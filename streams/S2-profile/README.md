# S2 — Profile & Identity

**Stream:** S2  
**Status:** 🟡 Active  
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
- **S6:** External prior art may inform downstream implementation only after S6 publishes ADRs. S2 does not mine old repos directly while G2 is active.

---

## What this stream blocks

- **S3 Agent Center:** agents reference `ProfileId` and `CapabilityGrant`.
- **S4 Runtime:** consent bus enforces `CapabilityGrant`; needs the type.
- **S7 Hub:** pairing flow is profile + device → signed capability grants.
- **S8 Docs:** identity model docs depend on S2 types being stable.

---

## Definition of done (G2)

- [x] `ferros-profile` crate builds and passes `cargo test` locally.
- [ ] Profile round-trips: create → serialize → sign → verify → revoke.
- [ ] `schemas/profile.v0.json` frozen (feature-flag protected; no mutations after freeze).
- [ ] `schemas/capability-grant.v0.json` frozen.
- [ ] Rust schema parity is enforced with a jsonschema-backed test against `schemas/profile.v0.json`.
- [ ] CLI: `ferros profile init | show | export | import | grant | revoke` all functional.
- [ ] At least one golden fixture in `schemas/fixtures/` for a valid profile and a valid grant.
- [ ] Negative fixture: invalid signature rejected.

---

## Likely crates / files

| Path | Role |
|------|------|
| `crates/ferros-profile/` | Identity crate |
| `schemas/profile.v0.json` | Profile schema |
| `schemas/capability-grant.v0.json` | Grant schema |
| `schemas/fixtures/profile-valid.json` | Golden fixture |
| `schemas/fixtures/grant-valid.json` | Golden fixture |
| `schemas/fixtures/grant-invalid-sig.json` | Negative fixture |

---

## Immediate next steps

1. Extend the crate beyond the foundation slice: fixture-backed serde model, key material, and consent-manifest types.
2. Draft `schemas/profile.v0.json` and `schemas/capability-grant.v0.json` from the crate boundary.
3. Add schema parity enforcement so the Rust model cannot drift from `profile.v0.json` before freeze.
4. Implement `grant` and `revoke` logic with signature verification.
5. Wire CLI subcommands.
6. Freeze schema under feature flag `profile-schema-v0`.
