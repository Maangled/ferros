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
- [ ] Define the minimum consent-gated browser-issued lifecycle control bar above the staged shell-intent copy before wiring shell calls to `agent.run` / `agent.stop`

## Later (Phase C — post-G3)

- [ ] Compile `ferros-core` to `wasm32-unknown-unknown`
- [ ] WASM demo page in `/site/wasm-demo/`
- [ ] CI job to verify WASM compilation

## Blocked

- No current upstream blocker. Privileged grant/revoke shell actions still depend on later S3/S4 write surfaces.
