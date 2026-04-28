# S8 Docs / Governance — Filler

Filler is research, cataloging, and spec writing. It is used to fill safe lane budget when owner waves are blocked or when the active critical-path lanes are full. Filler never reopens a closed gate, never mutates a frozen contract, and never retroactively changes evidence that has already been recorded. Filler output is forward-looking material that feeds future planning.

---

## Near — items that accelerate the path to D1 or G4

1. **ADR backlog triage (feeds system-track queue).**
   Catalog the topics in `docs/adr/_ROADMAP.md` and the system-track queue and confirm which ones have been addressed, which are still open, and which are blocked on other waves. Produce an updated `_ROADMAP.md` preamble note. Output: update `docs/adr/_ROADMAP.md` (additive, docs-only). Safe-with: SYSTEM-QUEUE waves.

2. **Contributor onboarding checklist (feeds GOVERNANCE.md follow-up).**
   Draft a plain-language contributor onboarding checklist: what a new contributor needs to read before opening a PR, what the stream model means for their first contribution, and how to use the wave queue. Output: `docs/onboarding/CONTRIBUTOR-CHECKLIST.md`. Safe-with: all other S8 filler waves.

3. **Gate narrative explainer (feeds external-facing comms).**
   One-page plain-English explainer of the G1–G4 gate progression and the new D1 demo gate for a non-technical audience: what each gate means in human terms and what "G4 is the launch gate" implies for partner timing. Output: `docs/explainers/gate-narrative.md`. Safe-with: all filler waves.

---

## Close — items that prepare content for the FERROS workflow pipeline

4. **GLOSSARY expansion.**
   Research pass over the current `GLOSSARY.md` to identify terms used in recent waves that are not yet defined: Batch Mode, track, size:S/L, gatekeeper, doc-batch, filler, D1. Add definitions without altering existing entries. Output: additive edits to `GLOSSARY.md`. Safe-with: all filler waves.

5. **Doc-batch template.**
   Draft the template file for `docs/orchestration/doc-batches/DOC-BATCH-TEMPLATE.md` so future batch summaries have a consistent shape. Output: `docs/orchestration/doc-batches/DOC-BATCH-TEMPLATE.md`. Safe-with: all orchestration waves.

---

## Far — items that anticipate the post-launch gamified system layer

6. **Post-launch governance research note.**
   Speculative research note on how FERROS governance scales post-launch: multi-contributor stream ownership, public ADR process, community consent for schema evolution, and what the current stream model needs to evolve into. Not a commitment; explicitly post-launch framing. Output: `docs/research/S8-post-launch-governance.md`. Safe-with: all filler waves.
