# S6 Ecosystem Harvest — Progress

Reverse-chronological. Append a dated entry at the top per session.

---

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
