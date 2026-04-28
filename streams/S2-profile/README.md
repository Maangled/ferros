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

## S7 consumer-boundary answers

These answers are S2-owned answers to the six open questions listed in `streams/S7-hub/README.md` and publish what S7 may rely on from S2 today. They keep the frozen `ProfileId` and `CapabilityGrant` v0 boundary steady, do not freeze handshake order, and do not ratify a final pairing protocol.

| Checkpoint | S2 answer |
|------------|-----------|
| bootstrap | S7 may assume durable identity bootstrap exists only when S2 can reload a locally persisted `KeyPair`; the stable `ProfileId` is derived from that Ed25519 verifying key, and that bootstrap identity state alone does not imply any existing `CapabilityGrant` yet. S2 does not publish who creates that state or in what order first-start pairing occurs. |
| grant check | Treat a capability as present and active only when a persisted signed grant for that capability verifies, binds to the local `ProfileId`, matches the local signer public key, and is not revoked. |
| deny visibility | At the S2 boundary, denied capability use is distinguishable only as missing grant state, revoked grant state, or invalid/mismatched signed grant material. S2 does not define how S4 or S7 must surface those outcomes in logs or UI. |
| persistence | After restart or power cycle, S7 may rely only on local profile and grant state that reloads and passes S2 local validation before reuse. `FileSystemProfileStore` is current filesystem-first implementation evidence, but its on-disk layout is not a published cross-stream contract. |
| revocation | A previously accepted grant becomes unusable when the signed envelope carries `revoked_at` and `revocation_reason`, has been re-signed, still verifies, and therefore reads as revoked. |
| re-registration | Treat a returning bridge agent as the same identity and grant context only when reload yields the same `ProfileId` from the persisted key and the relevant signed grants still verify, match that identity and signer, and remain active. Otherwise S7 should require a new approval path. |

---

## S5 consumer-awareness note

S5 has wired a checkpoint of the minimum honest first browser profile surface entry bar above the frozen S2 contract. The checkpoint consumes `ferros profile init`, `show`, `export`, and `import` only through the localhost `/profile` adapter, backed by the already-frozen `schemas/profile.v0.json` contract. It is not cleanly closed until focused Rust validation can run under WAVE-2026-04-28-18. S5 does not reopen G2 or mutate the S2 contract. Grant mutation, `revoke`, and remote profile access are explicitly out of scope for S5's stated profile surface. This is a read/init consumer relationship; S2's frozen boundary is unchanged.

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
