# S6 Ecosystem Harvest — Filler

Filler is research, cataloging, and spec writing. It is used to fill safe lane budget when owner waves are blocked or when the active critical-path lanes are full. Filler never reopens a closed gate, never mutates a frozen contract, and never retroactively changes evidence that has already been recorded. Filler output is forward-looking material that feeds future planning.

---

## Near — items that accelerate the path to D1 or G4

1. **Asset library scaffold research (feeds system-track legal wave).**
   Research note cataloging what an asset library for FERROS would contain: data types harvested from legacy systems, consent requirements per asset class, and what the migration-first boundary means for asset import. Output: `docs/research/S6-asset-library-scaffold.md`. Safe-with: SYSTEM-QUEUE onramp ADR wave.

2. **Prior-art harvest index (feeds ADR backlog).**
   Catalog the prior-art HTML prototypes in `docs/legacy/` and `docs/` that contain domain knowledge relevant to active ADR work (especially card/deck nomenclature, parametric authoring, arena export). For each prototype, note which ADR it feeds and whether a harvest wave is warranted. Output: `docs/research/S6-prior-art-harvest-index.md`. Safe-with: S8 filler waves.

3. **`ferros-data` migration manifest completeness check (feeds G4 data-layer readiness).**
   Research note documenting what the ordered migration manifest currently covers and what migration scenarios are not yet covered (e.g., profile schema evolution, capability grant schema evolution). Output: `docs/research/S6-migration-manifest-completeness.md`. Safe-with: S4 filler waves.

---

## Close — items that prepare content for the FERROS workflow pipeline

4. **Migration UX research note (feeds S5 onboarding).**
   Document what the user sees when FERROS performs a data migration on their local profile: what prompt appears, what rollback looks like, and how the user confirms the migration succeeded. Feeds S5 onboarding flow spec. Output: `docs/research/S6-migration-ux.md`. Safe-with: S5 filler waves.

5. **Smart-contract draft scaffold (feeds system-track ledger ADR).**
   Draft placeholder structure for a smart-contract interface that could represent FERROS capability grants on a chosen ledger substrate. Explicitly labelled as a draft scaffold pending the ledger ADR decision. Output: `docs/research/S6-smart-contract-draft-scaffold.md`. Safe-with: SYSTEM-QUEUE ledger ADR wave.

---

## Far — items that anticipate the post-launch gamified system layer

6. **Trade-window / card-deck game mechanics research.**
   Speculative research note on the post-launch gamified layer: how FERROS profile assets could participate in a card-deck / trade-window game mechanic, what the consent model looks like for asset trades, and what data types the harvest layer would need to support. References ADR-010 and ADR-016. Output: `docs/research/S6-trade-window-card-deck-research.md`. Safe-with: S5 and S8 filler waves.
