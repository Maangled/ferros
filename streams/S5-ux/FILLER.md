# S5 UX — Filler

Filler is research, cataloging, and spec writing. It is used to fill safe lane budget when owner waves are blocked or when the active critical-path lanes are full. Filler never reopens a closed gate, never mutates a frozen contract, and never retroactively changes evidence that has already been recorded. Filler output is forward-looking material that feeds future planning.

---

## Near — items that accelerate the path to D1 or G4

1. **Profile surface wireframe (feeds profile-surface entry-bar wave).**
   Sketch a slot-map wireframe for the minimal profile surface: `init`, `show`, `export`, `import` slots on the localhost shell. Document what each slot renders and what backend calls it makes against the frozen S2 contract. Output: `docs/research/S5-profile-surface-wireframe.md`. Safe-with: S2 filler waves and profile-surface entry-bar wave.

2. **Consent flow UX research note (feeds D1 prep).**
   Document what the operator-visible consent flow looks like today in the localhost shell: where the deny-log appears, what a granted vs. denied capability looks like in the UI, and what a user would need to see to understand the flow. This is research for the D1 demo script. Output: `docs/research/S5-consent-flow-ux.md`. Safe-with: HARDWARE-QUEUE UX session plan wave.

3. **Shell navigation depth audit (feeds Phase C planning).**
   Catalog the current six-degree reach from the agent-center shell home slot to the grant, deny-log, and profile views. Identify any workflow that currently exceeds six degrees and flag it for remediation. Output: `docs/research/S5-shell-navigation-depth-audit.md`. Safe-with: S3 filler waves.

---

## Close — items that prepare content for the FERROS workflow pipeline

4. **Onboarding flow spec (feeds in-product help).**
   Spec the new-user onboarding flow for the localhost shell: what the user sees on first launch, what prompts appear before a profile exists, and how the shell recovers from an empty state. Output: `docs/research/S5-onboarding-flow-spec.md`. Safe-with: S2 and S8 filler waves.

5. **Operator evidence surface wireframe (feeds operator evidence surface wave).**
   Sketch the operator-facing evidence surface implied by the Pack B bring-up worksheet: which fields are visible, what the read-only constraint means for the layout, and how the operator distinguishes bring-up state from steady-state. Output: `docs/research/S5-operator-evidence-surface-wireframe.md`. Safe-with: S7 filler waves.

---

## Far — items that anticipate the post-launch gamified system layer

6. **Card/deck UX research note.**
   Speculative research note on how a card/deck game interaction layer could surface in the FERROS shell post-launch: trade windows, deck views, card-collection affordances. References ADR-010 nomenclature and ADR-015 parametric authoring. Output: `docs/research/S5-card-deck-ux-research.md`. Safe-with: S6 and S8 filler waves.
