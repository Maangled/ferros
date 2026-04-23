# S6 Ecosystem Harvest — Progress

Reverse-chronological. Append a dated entry at the top per session.

---

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
