# S2 Profile & Identity — Backlog

---

## Now

- [ ] Design `ProfileId`, `KeyPair`, `CapabilityGrant`, `ConsentManifest` types (foundation slice started; key material and consent manifest still open)
- [ ] Draft `schemas/profile.v0.json`
- [ ] Draft `schemas/capability-grant.v0.json`

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

- Full S2 closeout remains blocked on G1 closure and CI-proven foundation evidence.
