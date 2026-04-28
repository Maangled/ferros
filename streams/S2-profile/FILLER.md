# S2 Profile & Identity — Filler

Filler is research, cataloging, and spec writing. It is used to fill safe lane budget when owner waves are blocked or when the active critical-path lanes are full. Filler never reopens a closed gate, never mutates a frozen contract, and never retroactively changes evidence that has already been recorded. Filler output is forward-looking material that feeds future planning.

**S2 constraint:** S2 is in handoff. The frozen `profile.v0.json` and `capability-grant.v0.json` schemas must not be touched. S2 filler is limited to consumer-awareness research, profile surface planning, and import/recovery UX research that references the already-frozen contract without reopening it. Do not reopen G2.

---

## Near — items that accelerate the path to D1 or G4

1. **Profile surface entry-bar spec (feeds S5).**
   Catalog what a minimal honest first browser profile surface looks like above the frozen S2 contract: `init`, `show`, `export`, `import` only, localhost-only, no grant mutation. Reference `docs/legacy/personal-profile.html` as prior art. Output: `docs/research/S2-profile-surface-entry-bar.md`. Safe-with: S5 filler and profile-surface entry-bar wave.

2. **Profile recovery UX research note (feeds S5/S7).**
   Document the expected user-facing recovery paths: export/import on a new device, what happens when the profile file is missing, and what the CLI shows at each step. This is UX research against the frozen contract, not a reopening. Output: `docs/research/S2-profile-recovery-ux.md`. Safe-with: S5 and S7 planning waves.

3. **Import/export round-trip spec (feeds D1 evidence prep).**
   Spec the exact CLI commands and expected output for a profile export/import round-trip on the D1 target device. This feeds D1 evidence scripting so the operator session plan knows exactly what "profile creation/show" means. Output: `docs/research/S2-profile-import-export-round-trip.md`. Safe-with: HARDWARE-QUEUE UX session plan wave.

---

## Close — items that prepare content for the FERROS workflow pipeline

4. **Profile in-product help copy.**
   Draft the user-facing help copy for `ferros profile init`, `show`, `export`, `import`. Each command gets a one-paragraph plain-English explanation suitable for in-product help. Output: `docs/ux-copy/profile-commands-help.md`. Safe-with: S5 filler waves.

5. **Identity portability explainer.**
   One-page plain-English explainer of what FERROS profile portability means: how identity moves across devices, what the export bundle contains, and what is and is not portable at v0. Feeds S8 contributor and user-facing docs. Output: `docs/explainers/identity-portability.md`. Safe-with: S8 filler waves.

---

## Far — items that anticipate the post-launch gamified system layer

6. **Character profile template research.**
   Research note cataloging how the S2 profile v0 contract could extend post-launch to support character/persona profile templates (avatar, display name, role within a shared FERROS space). Explicitly framed as post-launch speculation that does not reopen profile.v0.json. Output: `docs/research/S2-character-profile-templates.md`. Safe-with: S6 and S8 filler waves.
