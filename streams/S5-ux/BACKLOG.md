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
- [ ] Execute the archive plan from `DOCS-HTML-PROTOTYPE-AUDIT.md` once inbound links are checked
- [ ] Verify no dead links in site

## Phase B (active)

- [x] Scaffold `crates/ferros-web/` or equivalent web shell server
- [x] Translate `PHASE-B-SHELL-WIREFRAME.md` into a fixed-slot HTML shell with visible collapse anchors
- [x] Agent list surface (reads from S3 registry via JSON/RPC)
- [x] Agent detail surface
- [x] Capability grant surface
- [x] Deny log surface
- [x] Wire to real `ferros-node` backend — no fake data
- [x] Add focused UI acceptance coverage for `ferros-node shell` plus `/rpc`
- [ ] Stage privileged grant/revoke shell intents without bypassing S2/S3/S4 consent boundaries

## Later (Phase C — post-G3)

- [ ] Compile `ferros-core` to `wasm32-unknown-unknown`
- [ ] WASM demo page in `/site/wasm-demo/`
- [ ] CI job to verify WASM compilation

## Blocked

- No current upstream blocker. Privileged grant/revoke shell actions still depend on later S3/S4 write surfaces.
