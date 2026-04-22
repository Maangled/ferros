# S2 — Profile & Identity

**Stream:** S2  
**Status:** ⬜ Blocked on G1  
**Gate:** G2

---

## Mission

Give every FERROS entity a locally-sovereign cryptographic identity. The profile is the root of consent. Every downstream stream that touches permissions, agents, or the hub takes a `ProfileId` or `CapabilityGrant` as input. Ship the types and file format early so downstream streams can code against stubs.

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

- **S1 (G1 must be green):** Cargo workspace and CI must exist.

---

## What this stream blocks

- **S3 Agent Center:** agents reference `ProfileId` and `CapabilityGrant`.
- **S4 Runtime:** consent bus enforces `CapabilityGrant`; needs the type.
- **S7 Hub:** pairing flow is profile + device → signed capability grants.
- **S8 Docs:** identity model docs depend on S2 types being stable.

---

## Definition of done (G2)

- [ ] `ferros-profile` crate builds and passes `cargo test`.
- [ ] Profile round-trips: create → serialize → sign → verify → revoke.
- [ ] `schemas/profile.v0.json` frozen (feature-flag protected; no mutations after freeze).
- [ ] `schemas/capability-grant.v0.json` frozen.
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

1. Create `crates/ferros-profile/` with a `Cargo.toml` stub.
2. Define `ProfileId`, `KeyPair`, `CapabilityGrant`, `ConsentManifest` types.
3. Draft `schemas/profile.v0.json` — iterate until round-trip works.
4. Implement `grant` and `revoke` logic with signature verification.
5. Wire CLI subcommands.
6. Freeze schema under feature flag `profile-schema-v0`.
