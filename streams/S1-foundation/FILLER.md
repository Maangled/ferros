# S1 Foundation — Filler

Filler is research, cataloging, and spec writing. It is used to fill safe lane budget when owner waves are blocked or when the active critical-path lanes are full. Filler never reopens a closed gate, never mutates a frozen contract, and never retroactively changes evidence that has already been recorded. Filler output is forward-looking material that feeds future planning, not corrections to settled work.

**S1 constraint:** S1's gate G1 is closed. S1 filler must NOT reopen G1 or revisit any G1 evidence claim. Acceptable S1 filler is forward-looking research on FERROS-as-OS substrate topics (boot, supervisor, hardware-native runtime assumptions) framed as research notes feeding S4 and S7 planning. S1 filler does not imply S1 is active again; it implies S1's accumulated knowledge is being mined for downstream benefit.

---

## Near — items that accelerate the path to D1 or G4

1. **Boot-sequence research note (feeds S7/S4).**
   Catalog the minimal FERROS boot sequence on the primary D1 target: supervisor init, profile mount, agent registry startup, first consent check. Output: `docs/research/S1-boot-sequence-d1-target.md`. Safe-with: all hardware and S4 planning waves.

2. **Supervisor boundary spec (feeds S4).**
   Write a one-page research note defining the supervisor / user-space boundary as FERROS currently sees it: what the runtime guarantees at boot, what a well-behaved agent may assume, and what is deliberately left undefined. Output: `docs/research/S1-supervisor-boundary-note.md`. Safe-with: S4 runtime planning and S7 hardware bring-up waves.

3. **Hardware-native runtime assumptions note (feeds S7 firmware spikes).**
   Document the assumptions FERROS-as-OS makes about the underlying hardware runtime: memory layout, stack alignment, peripheral availability, and watchdog behavior. Frame this as planning input for the D1 firmware spike, not a S1 reopening. Output: `docs/research/S1-hardware-runtime-assumptions.md`. Safe-with: all HARDWARE-QUEUE waves.

---

## Close — items that prepare content for the FERROS workflow pipeline

4. **In-product boot help spec.**
   Draft the user-facing explanation of what happens at first boot: profile init prompt, initial capability grants, first agent registration. Framed as in-product help copy that will eventually surface in the localhost shell or device UI. Output: `docs/ux-copy/boot-first-run-help.md`. Safe-with: S5 and S7 filler waves.

5. **Supervisor trust model explainer (for contributor onboarding).**
   One-page plain-English explainer of the trust model: why the supervisor is the only path for capability grants, why agents cannot escalate their own grants, and how this relates to consent-first. Feeds S8 contributor docs. Output: `docs/explainers/supervisor-trust-model.md`. Safe-with: S8 filler waves.

---

## Far — items that anticipate the post-launch gamified system layer

6. **FERROS-as-OS narrative spec.**
   Research note exploring what it means for FERROS to present itself as an OS-level identity and consent layer post-launch: what that narrative means for profile portability, device ownership transfer, and long-term OS substrate evolution. This is speculative framing for the post-launch roadmap, not a G1 reopening. Output: `docs/research/S1-ferros-as-os-narrative.md`. Safe-with: S6 and S8 filler waves.
