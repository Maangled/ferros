# S7 Smart-Home Hub — Filler

Filler is research, cataloging, and spec writing. It is used to fill safe lane budget when owner waves are blocked or when the active critical-path lanes are full. Filler never reopens a closed gate, never mutates a frozen contract, and never retroactively changes evidence that has already been recorded. Filler output is forward-looking material that feeds future planning.

---

## Near — items that accelerate the path to D1 or G4

1. **D1 bring-up checklist (feeds HARDWARE-QUEUE device inventory wave).**
   Research note listing the exact bring-up checks needed before the D1 demo can be declared ready: profile init on device, one HA entity visible, consent flow visible, power cycle passes. Cross-reference `docs/gates/D1.md`. Output: `docs/research/S7-d1-bring-up-checklist.md`. Safe-with: HARDWARE-QUEUE D1 device inventory wave.

2. **HA bridge seam catalog (feeds D1 firmware spike).**
   Catalog the bridge contract seams defined in the existing HA bridge runway contract: what the bridge shim agent must declare, what HA entity types are in scope for D1, and what the stand-in means for D1 evidence. This is a catalog of already-defined runway material, not new invention. Output: `docs/research/S7-ha-bridge-seam-catalog.md`. Safe-with: HARDWARE-QUEUE firmware-spike waves.

3. **Pairing checkpoint map explainer (feeds operator readiness).**
   Plain-English explainer of the six-checkpoint pairing map for an operator who will run the D1 demo: what each checkpoint means, what the operator must verify at each step, and what to do if a checkpoint fails. Output: `docs/research/S7-pairing-checkpoint-explainer.md`. Safe-with: HARDWARE-QUEUE UX session plan wave.

---

## Close — items that prepare content for the FERROS workflow pipeline

4. **Operator evidence surface spec (feeds S5 operator evidence surface wave).**
   Spec the operator-facing evidence surface implied by the Pack B bring-up worksheet: what fields are read-only, what the operator can filter, and how the surface maps to the existing S3 read-first JSON/RPC contract. Output: `docs/research/S7-operator-evidence-surface-spec.md`. Safe-with: S5 operator evidence surface filler.

5. **Hub power-cycle recovery protocol (feeds D1 evidence prep).**
   Document the expected recovery sequence after a power cycle: profile load order, agent re-registration sequence, HA re-connection status, and what "recovery complete" means for D1. Output: `docs/research/S7-power-cycle-recovery-protocol.md`. Safe-with: HARDWARE-QUEUE UX session plan wave.

---

## Far — items that anticipate the post-launch gamified system layer

6. **Multi-hub topology research note.**
   Speculative research note on what a post-launch multi-hub FERROS home installation looks like: how profile identity spans hubs, how agents re-register across hubs, and what the consent model looks like when the same profile is active on two devices. Not a G4 reopening; explicitly post-launch. Output: `docs/research/S7-multi-hub-topology.md`. Safe-with: S4 and S8 filler waves.
