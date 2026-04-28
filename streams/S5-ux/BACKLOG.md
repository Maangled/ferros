# S5 UX — Backlog

---

## Now

- [x] Capture the Phase B surface-first shell note: surfaces instead of windows plus the six-degree reach rule (`SURFACE-FIRST-SHELL.md`)
- [x] Audit `docs/` HTML prototypes — list what to keep, archive, or remove (`DOCS-HTML-PROTOTYPE-AUDIT.md`)
- [x] Turn the surface-first shell note into a slot-based wireframe for inspect, grant, and deny-log flows (`PHASE-B-SHELL-WIREFRAME.md`)
- [x] Define the minimal shell intent vocabulary for focus, route selection, consent, and audit handoff (`PHASE-B-SHELL-WIREFRAME.md`)

## Phase A (active)

- [x] Confirm `/site/` layout in repo reality
- [x] Add honest status banner to the real landing page (links to `STATUS.md`)
- [ ] Repoint current repo links before moving the remaining historical docs-root HTML files
- [ ] Verify no dead links in site

## Phase B (active)

- [x] Scaffold `crates/ferros-web/` or equivalent web shell server
- [x] Translate `PHASE-B-SHELL-WIREFRAME.md` into a fixed-slot HTML shell with visible collapse anchors
- [x] Agent list surface (reads from S3 registry via JSON/RPC)
- [x] Agent detail surface
- [x] Capability grant surface
- [x] Deny log surface
- [x] Aggregate agent registry/detail, grant-state, and deny-log observation through the single `agent.snapshot` read seam
- [x] Wire to real `ferros-node` backend — no fake data
- [x] Same-origin UI acceptance for `ferros-node shell` plus `/rpc` proves snapshot-only refresh and inspector reuse from loaded shell state
- [x] Operator-assisted localhost acceptance proves local `ferros agent run | stop` state changes are observable through the same `agent.snapshot` refresh seam without adding shell write controls; deny generation remains outside the current shell/CLI surface
- [x] Define the minimum first shell-intent entry bar above the landed local-only `agent.run` / `agent.stop` JSON/RPC slice: selected-agent shell copy and slot ownership are the next honest surface, while browser-issued writes, grant/revoke, consent resolution, and broader browser control remain out of scope until a later code-backed follow-up exists
- [x] Land selected-agent shell intent copy and read-only slot affordances against the landed local-only `agent.run` / `agent.stop` JSON/RPC slice above `LocalAgentApi`; keep grant/revoke and other privileged controls out of the shell until S2/S3/S4 publish broader code-backed write surfaces
- [x] Define the minimum consent-gated browser-issued lifecycle control bar above the staged shell-intent copy before wiring shell calls to `agent.run` / `agent.stop`
- [x] Land browser-issued local lifecycle control bar (selected-agent `agent.run` / `agent.stop` only, active loaded-grant check plus explicit arm before write RPC transmission, backend denial refreshes through `agent.snapshot`, harness confirms an unarmed or missing-grant click does not transmit `agent.run` / `agent.stop`)
- [x] Define the minimum honest first browser profile surface entry bar above the frozen S2 contract: `init`, `show`, `export`, `import` only, localhost-only, no grant mutation, no S2 reopening, prior art from `docs/legacy/personal-profile.html` as shape reference only
- [ ] Land minimum profile surface on the localhost shell (`init`, `show`, `export`, `import` slots backed by frozen S2 CLI paths, harness proves surface stays within the frozen S2 boundary) — implementation checkpoint wired; Rust validation/closeout is blocked under WAVE-2026-04-28-18
- [x] Define the minimum onramp consent surface entry bar: one slot per proposed onramp item (source system name, proposed item description, consent prompt, accept/reject affordance), governed by ADR-023 invariants (quarantine until accepted; consent explicit and auditable; external system does not define identity; localhost-only; no wired onramp calls until code-backed follow-up)
- [ ] Land onramp consent surface on the localhost shell (proposed-item slot wired, audit-log seam captures explicit accept event, harness proves item cannot reach canonical state without explicit user action)
- [x] Define the consent-flow copy spec for the S5 consent gate, derived from `docs/legal/CONSENT-LANGUAGE.md` DRAFT sections (capability grant language, onramp accept language, deny-visibility disclosure), marked as draft pending counsel red-line
- [ ] Clear draft status on consent-flow copy spec once CONSENT-LANGUAGE.md has been reviewed by counsel and draft status cleared (coordinated update with legal scaffold)

## Later (Phase C — post-G3)

- [ ] Compile `ferros-core` to `wasm32-unknown-unknown`
- [ ] WASM demo page in `/site/wasm-demo/`
- [ ] CI job to verify WASM compilation

## Blocked

- No current upstream blocker. Privileged grant/revoke shell actions still depend on later S3/S4 write surfaces.
