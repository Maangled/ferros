# S3 Agent Center — Progress

Reverse-chronological. Append a dated entry at the top per session.

---

## 2026-04-23 — Pre-G3 contract scaffold landed

- Added a standalone `crates/ferros-agents/` crate without touching the root workspace manifest.
- Defined a pre-G3 `Agent` trait, `AgentManifest`, `CapabilityRequirement`, and `AgentRegistry` boundary against the current S2 `ProfileId` and `CapabilityGrant` types.
- Added an `InMemoryAgentRegistry` with deterministic `BTreeMap` ordering plus manifest authorization helpers that deny by default when required capabilities are missing.
- Validated the scaffold with a manifest-scoped `cargo test` run.

## 2026-04-21 — Stream scaffolded

- Stream directory and planning documents created.
- Blocked on G2 (S2 Profile & Identity).
- `Agent` trait interface can be sketched in parallel with S2/S4 work.
- S6 harvest ADRs for `botgen-rust` and `workpace-rust` should be completed before S3 implementation begins.
