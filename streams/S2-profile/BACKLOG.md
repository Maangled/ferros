# S2 Profile & Identity — Backlog

---

## Now

- [ ] Design `ProfileId`, `KeyPair`, `CapabilityGrant`, `ConsentManifest` types (can start before G1)
- [ ] Draft `schemas/profile.v0.json`
- [ ] Draft `schemas/capability-grant.v0.json`

## Next

- [ ] Create `crates/ferros-profile/` (after G1)
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

- Crate creation blocked on G1 (Cargo workspace must exist).
