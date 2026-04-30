# S6 Ecosystem Harvest — Progress

Reverse-chronological. Append a dated entry at the top per session.

---

## 2026-04-30 - Shared local-runway guardrail layer and H1 proposal parity landed

- Extended `crates/ferros-data/src/lib.rs` so `ferros-data` now owns reusable local-runway wording, scope, evidence, and `.tmp/hub` path guardrails for downstream local hub and node surfaces.
- The current hub and runtime consumers now reuse that layer for local summary artifacts and for dropping invalid runway children from the existing read-only summary seam instead of projecting malformed local data.
- WAVE-2026-04-30-74 brought the published `schemas/onramp-proposal.schema.json` plus regenerated H1 constants and validator coverage back into parity with that same local-only ceiling instead of leaving the stricter banned-word enforcement only in runtime helpers.
- Kept ADR-020 posture intact: no new partner-facing schema, no canonical mutation, no real accept or reject transport, no remote transport, and no gate advancement is claimed from this guardrail layer.

## 2026-04-30 — Local onramp decision receipt boundary published for the hub rehearsal packet

- Extended `crates/ferros-data/src/lib.rs` so `ferros-data` now owns the typed `LocalOnrampDecisionReceipt` model, bounded validation, and local JSON artifact writer for one recorded local operator decision linked to proposed material.
- The current local hub rehearsal packet now consumes that boundary end-to-end: hub emission, schema/H1 admission, additive runway-summary observation, display-only shell/harness proof, and `cargo xtask hub-runway` validation all reuse the same local-only primitive rather than inventing a second decision model.
- Kept ADR-020 posture intact: local-only primitive ownership is stronger, but no partner-facing schema, no canonical mutation, no real accept/reject transport, no remote transport, and no gate advancement is claimed from this boundary.

## 2026-04-29 — Local onramp proposal boundary published for the hub rehearsal packet

- Extended `crates/ferros-data/src/lib.rs` so `ferros-data` now owns the typed `LocalOnrampProposal` model, bounded validation, and local JSON artifact writer for quarantined pending-consent proposed material.
- The current local hub rehearsal packet now consumes that boundary end-to-end: hub emission, schema/H1 admission, additive runway-summary observation, display-only shell/harness proof, and `cargo xtask hub-runway` validation all reuse the same local-only primitive rather than inventing a second proposed-material model.
- Kept ADR-020 posture intact: local-only primitive ownership is stronger, but no partner-facing schema, no canonical mutation, no remote transport, and no gate advancement is claimed from this boundary.

## 2026-04-28 — local-push envelope admitted to harness consumers and emitted by burst helper

- Extended `crates/ferros-data/src/lib.rs` so the typed local-push audit envelope now serializes and writes local JSON artifacts while keeping authority, consent, and scope inside the existing schema boundary.
- Regenerated `harnesses/_constants.js` and extended `harnesses/ferros-contract-validator.html` so the local-push schema is admitted into harness constants and exercised by an inline validation case.
- Extended `xtask/src/main.rs` so `cargo xtask burst` now emits `.tmp/push/burst-local-push-envelope.json` as local-only non-partner-facing output under `.tmp/push/`.
- Validation passed with `cargo test -p ferros-data`, `cargo check -p xtask`, `cargo xtask burst`, and a direct harness contract-validator run after regeneration.
- Kept the schema unchanged and the output local-only: no gate movement, no hardware or HA claim, and no downstream stream advancement beyond harness and burst-helper consumers.

## 2026-04-26 — ferros-data migration-first boundary hardened with ordered manifest coverage

- Extended `crates/ferros-data/src/lib.rs` to publish the ordered migration path manifest for the baseline and ordered-child tightening SQL files instead of leaving that sequence implicit in tests only.
- Tightened the crate-local ADR-020 proof so the combined migration coverage now checks both sides of the final ordered-child contract: at least one parent must be present, and both parent columns cannot be present at the same time.
- Validation passed with `cargo test -p ferros-data`.

## 2026-04-23 — ferros-data admitted to the root workspace

- Added `crates/ferros-data/` to the root Cargo workspace and removed the crate-local standalone `[workspace]` marker used during the earlier isolated scaffold phase.
- Validated the narrow admission slice with `cargo metadata --no-deps` and `cargo test -p ferros-data` from the repository root.
- Updated S6 stream tracking docs to reflect that `ferros-data` is now a root workspace member, while downstream consumers remain intentionally untouched.
- This moves ADR-020 from standalone scaffold status to verified workspace residency; it does not claim any S3, S5, or gate advancement.

## 2026-04-23 — ferros-data scaffold started under ADR-020

- Created `crates/ferros-data/` as a standalone crate so Lane D can start the data slice without editing the root workspace manifest.
- Added an owned baseline SQL migration that establishes `revision_base`, a JSONB snapshot table, and a database-side parent invariant as the first ADR-020 proof points.
- Added a tiny Rust boundary API and crate-local tests that pin migration authority, JSONB snapshots, and application prevalidation expectations.
- Validated this slice with `cargo test --manifest-path crates/ferros-data/Cargo.toml`; workspace membership remains deferred to a later lane.

## 2026-04-23 — ADR-020 landed from sheetgen audit

- Converted `.tmp/sg-r.md` into ADR-020 as the third concrete S6 harvest decision.
- Chose migration-first authority for future `ferros-data` relational invariants.
- Recorded adoption of JSONB snapshot, polymorphic-parent dual validation, and revision-base patterns.
- Rejected drift-prone three-source schema maintenance and runtime rewriting of committed contract files.

## 2026-04-23 — ADR-019 landed from workpace audit

- Converted `.tmp/w-r.md` into ADR-019 as the second concrete S6 harvest decision.
- Recorded that FERROS adopts typed IPC, signed-delivery, and capability-scoped UI-bus patterns from workpace-rust.
- Recorded that iframe isolation, IndexedDB, blob URLs, actix-web sessions, and other browser-bound mechanics are reference-only or discarded.
- Marked workpace-rust as a post-G3 S5 shell prior-art source rather than a direct implementation dependency.

## 2026-04-23 — ADR-018 landed from botgen audit

- Converted `.tmp/bg-r.md` into ADR-018 as the first concrete S6 harvest decision.
- Recorded that FERROS adopts or adapts botgen-rust lifecycle, queue, task-history, and manifest-shape ideas, but does not directly port botgen implementation layers.
- Marked botgen-rust as the first completed audit and ADR pass in S6.
- Established ADR-018 as the handoff point for S3 and S4, replacing direct legacy-repo reading in those implementation streams.

## 2026-04-23 — Harvest lane activated after G1 closure

- G1 closure means S6 is no longer waiting on workspace foundation just to begin audits and ADR writing.
- Canonical harvest ADR numbering is now ADR-018 (`botgen-rust`), ADR-019 (`workpace-rust`), ADR-020 (`sheetgen-rust`) to avoid collisions with existing ADR-016 and ADR-017.
- Captured the external prior-art policy: old repos flow into FERROS through S6 ADRs, not directly into S2, S3, S4, S5, or S7 implementation.
- Set `botgen-rust` as the first execution slice because it is the highest-value input for S3 and S4.

## 2026-04-21 — Stream scaffolded

- Stream directory and planning documents created.
- Blocked on G1 for crate extraction, but audits can begin immediately.
- Initial ADR placeholders pre-allocated before later renumbering.
- Harvest ADR for `botgen-rust` is P0 since it unblocks S3 design.
