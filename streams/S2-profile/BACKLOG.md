# S2 Profile & Identity — Backlog

---

## Now

- [ ] Finish the identity type surface: `KeyPair` and full profile-level signing are still open after `ProfileId`, signed `CapabilityGrant`, and `ConsentManifest` landed
- [ ] Refine `schemas/profile.v0.json` for G2 freeze
- [x] Freeze `schemas/capability-grant.v0.json` as the signed envelope contract
- [ ] Expand the existing fixture-backed `ProfileDocument` contract test beyond the minimal Stage 0 fixture before freeze

## Next

- [x] Create `crates/ferros-profile/`
- [x] Add fixture-backed serde parsing for the minimal Stage 0 profile
- [ ] Implement Ed25519 keypair generation and device binding
- [x] Implement grant + revoke logic with signature verification
- [x] Profile storage abstraction (filesystem-first)
- [ ] CLI: `ferros profile init | show | export | import | grant | revoke`
- [x] Golden fixtures for valid profile and valid grant
- [x] Negative fixture for invalid signature

## Later

- [ ] Optional passphrase wrap for keypair
- [ ] `sled` or `redb` storage backend
- [ ] Multi-device profile sync (post-launch)

## Blocked

No upstream gate blocker remains. S2 is now the active gate; the remaining work is implementation, schema freeze, and CI evidence.
