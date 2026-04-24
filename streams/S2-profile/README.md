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
- [x] `CapabilityGrant` sign → serialize → verify → revoke works end-to-end with Ed25519 and re-signing on revoke.
- [x] Profile round-trips: create → serialize → sign → verify → revoke.
- [ ] `schemas/profile.v0.json` frozen (feature-flag protected; no mutations after freeze).
- [x] `schemas/capability-grant.v0.json` frozen as the stripped-payload signed envelope contract.
- [x] Rust/schema parity is enforced with a fixture-backed contract test against `schemas/profile.v0.json`.
- [ ] CLI: `ferros profile init | show | export | import | grant | revoke` all functional.
- [x] At least one golden fixture in `schemas/fixtures/` for a valid profile and a valid grant.
- [x] Negative fixture: invalid signature rejected.

---

## Likely crates / files

| Path | Role |
|------|------|
| `crates/ferros-profile/` | Identity crate |
| `schemas/profile.v0.json` | Draft S2-owned profile schema exercised by `ferros-profile` tests |
| `schemas/fixtures/minimal-stage0-profile.json` | Existing Stage 0 profile fixture used for serde and schema parity tests |
| `schemas/capability-grant.v0.json` | Frozen signed grant envelope schema exercised by `ferros-profile` tests |
| `schemas/fixtures/grant-valid.json` | Golden happy-path signed grant fixture |
| `schemas/fixtures/grant-invalid-sig.json` | Negative signed grant fixture with an invalid signature |

---

## Immediate next steps

1. Decide whether the additive `SignedProfileDocument` envelope remains a Rust-local proof surface or earns its own published schema when `profile.v0.json` freezes.
2. Freeze `schemas/profile.v0.json` and expand parity enforcement beyond the minimal Stage 0 happy path.
3. Wire CLI subcommands, starting with `ferros profile init | show` and then building `grant` / `revoke` on the landed signed envelope contract.
4. Decide whether the promised `profile-schema-v0` freeze mechanism is still the right path before calling profile v0 frozen.
