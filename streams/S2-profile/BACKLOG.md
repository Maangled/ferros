# S2 Profile & Identity — Backlog

---

## Now

- [ ] Finish the identity type surface: `KeyPair` and `ConsentManifest` are still open after `ProfileId` and `CapabilityGrant` landed
- [ ] Refine `schemas/profile.v0.json` for G2 freeze
- [x] Draft `schemas/capability-grant.v0.json`
- [ ] Expand the existing fixture-backed `ProfileDocument` contract test beyond the minimal Stage 0 fixture before freeze

## Next

- [x] Create `crates/ferros-profile/`
- [x] Add fixture-backed serde parsing for the minimal Stage 0 profile
- [ ] Implement Ed25519 keypair generation and device binding
- [ ] Implement grant + revoke logic with signature verification
- [ ] Profile storage abstraction (filesystem-first)
- [ ] CLI: `ferros profile init | show | export | import | grant | revoke`
- [ ] Golden fixtures for valid profile and valid grant
- [ ] Negative fixture for invalid signature

## Later

- [ ] Optional passphrase wrap for keypair
- [ ] `sled` or `redb` storage backend
- [ ] Multi-device profile sync (post-launch)

## Blocked

No upstream gate blocker remains. S2 is now the active gate; the remaining work is implementation, schema freeze, and CI evidence.
