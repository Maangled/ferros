# S3 Agent Center — Filler

Filler is research, cataloging, and spec writing. It is used to fill safe lane budget when owner waves are blocked or when the active critical-path lanes are full. Filler never reopens a closed gate, never mutates a frozen contract, and never retroactively changes evidence that has already been recorded. Filler output is forward-looking material that feeds future planning.

---

## Near — items that accelerate the path to D1 or G4

1. **Agent manifest catalog (feeds D1 prep).**
   Catalog the manifest fields for the two reference agents (`echo`, `timer`) and document what a minimal third agent (an HA-bridge shim) would declare. This is a docs-only catalog, not an implementation. Output: `docs/research/S3-agent-manifest-catalog.md`. Safe-with: HARDWARE-QUEUE firmware-spike waves.

2. **Remote transport boundary research note (feeds S7).**
   Document what the remote transport boundary for `ferros-node` would look like from the agent center's perspective: which read methods cross, which write methods stay local-only, and what the hub bridge agent would need the transport to expose. This is research for S7 planning, not a transport implementation. Output: `docs/research/S3-remote-transport-boundary.md`. Safe-with: S7 filler waves.

3. **Agent-center read-path explainer (feeds S5 and contributor onboarding).**
   One-page explainer of the read-first JSON/RPC contract: what each method (`agent.list`, `agent.describe`, `agent.snapshot`, `grant.list`, `denyLog.list`) returns and why the read-first posture exists. Output: `docs/explainers/agent-center-read-path.md`. Safe-with: S5 and S8 filler waves.

---

## Close — items that prepare content for the FERROS workflow pipeline

4. **Agent lifecycle in-product help copy.**
   Draft the user-facing help copy for `ferros agent list`, `describe`, `run`, `stop`, `logs`. Each command gets a one-paragraph plain-English explanation suitable for in-product help. Output: `docs/ux-copy/agent-commands-help.md`. Safe-with: S5 filler waves.

5. **Deny-log UX research note.**
   Research note on the deny-log as a user-facing surface: what information should be visible, what context is missing today, and how the deny-log could evolve into a consent-audit trail the user actually reads. Output: `docs/research/S3-deny-log-ux.md`. Safe-with: S5 filler waves.

---

## Far — items that anticipate the post-launch gamified system layer

6. **Agent marketplace research note.**
   Speculative research note on what a post-launch agent discovery and trust model could look like: how agents would be published, reviewed, and granted capabilities in a shared FERROS ecosystem. Not a roadmap commitment. Output: `docs/research/S3-agent-marketplace-research.md`. Safe-with: S6 and S8 filler waves.
