# S4 Runtime / OS Core — Filler

Filler is research, cataloging, and spec writing. It is used to fill safe lane budget when owner waves are blocked or when the active critical-path lanes are full. Filler never reopens a closed gate, never mutates a frozen contract, and never retroactively changes evidence that has already been recorded. Filler output is forward-looking material that feeds future planning.

---

## Near — items that accelerate the path to D1 or G4

1. **`no_std` target matrix note (feeds S7 firmware spikes).**
   Catalog the current state of `ferros-core` and `ferros-runtime` cross-compilation targets: which targets build clean today, which require feature flags, and what the D1 device target requires. Output: `docs/research/S4-no-std-target-matrix.md`. Safe-with: HARDWARE-QUEUE firmware-spike waves.

2. **Restart/reload boundary spec (feeds S5 and S7).**
   Document the S4 restart/reload boundary as defined in the locked tests: what `restart` and `reload` mean at the runtime level, what the shell or hub must not do to trigger an unintended restart, and what the boundary looks like from S7's perspective during power-cycle recovery. Output: `docs/research/S4-restart-reload-boundary-spec.md`. Safe-with: S5 and S7 planning waves.

3. **Policy engine invariant catalog (feeds D1 evidence prep).**
   Catalog the 10+ unit tests and property tests covering grant/deny scenarios. Express each invariant in one plain-English sentence so the D1 demo operator understands what "deny by default" means in practice. Output: `docs/research/S4-policy-engine-invariants.md`. Safe-with: all D1 planning waves.

---

## Close — items that prepare content for the FERROS workflow pipeline

4. **Runtime in-product help copy.**
   Draft brief user-facing copy explaining the FERROS runtime posture: what the runtime does, what an agent is not allowed to do without a grant, and why FERROS rejects capability escalation. Output: `docs/ux-copy/runtime-help.md`. Safe-with: S5 and S8 filler waves.

5. **Executor and message-bus explainer.**
   One-page plain-English explainer of the in-memory executor and message bus for contributor onboarding: what each component does, where messages flow, and what the bus guarantees. Output: `docs/explainers/executor-message-bus.md`. Safe-with: S8 filler waves.

---

## Far — items that anticipate the post-launch gamified system layer

6. **Multi-device runtime boundary research note.**
   Speculative research note on what the runtime boundary would need to support in a multi-device post-launch FERROS installation: shared bus across devices, supervisor trust across hops, and what today's in-memory single-host model would need to evolve into. Not a roadmap commitment; explicitly post-launch. Output: `docs/research/S4-multi-device-runtime-boundary.md`. Safe-with: S7 and S8 filler waves.
