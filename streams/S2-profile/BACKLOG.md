# S2 Profile & Identity — Backlog

---

## Now

- [x] Decide whether the additive `SignedProfileDocument` envelope stays Rust-local or becomes a published schema at G2 freeze
- [x] Refine `schemas/profile.v0.json` for G2 freeze
- [x] Freeze `schemas/capability-grant.v0.json` as the signed envelope contract
- [x] Broaden the fixture-backed `ProfileDocument` freeze evidence beyond the base and embedded signed-profile happy paths before freeze

## Next

- [x] Create `crates/ferros-profile/`
- [x] Add fixture-backed serde parsing for the minimal Stage 0 profile
- [x] Implement Ed25519 keypair generation and device binding
- [x] Implement grant + revoke logic with signature verification
- [x] Profile storage abstraction (filesystem-first)
- [x] CLI: `ferros profile init | show | export | import | grant | revoke`
- [x] Golden fixtures for valid profile and valid grant
- [x] Negative fixture for invalid signature
- [ ] Tighten parity or local CLI coverage only where it reinforces the frozen `profile.v0.json` and `capability-grant.v0.json` boundaries without widening them in place

## Later

- [ ] Optional passphrase wrap for keypair
- [ ] `sled` or `redb` storage backend
- [ ] Multi-device profile sync (post-launch)

## Blocked

No upstream gate blocker remains. G2 is closed; the remaining work is post-G2 parity or local CLI hardening plus optional future key-wrap and storage work.
