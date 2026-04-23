# S5 UX — Backlog

---

## Now

- [x] Capture the Phase B surface-first shell note: surfaces instead of windows plus the six-degree reach rule (`SURFACE-FIRST-SHELL.md`)
- [x] Audit `docs/` HTML prototypes — list what to keep, archive, or remove (`DOCS-HTML-PROTOTYPE-AUDIT.md`)
- [x] Turn the surface-first shell note into a slot-based wireframe for inspect, grant, and deny-log flows (`PHASE-B-SHELL-WIREFRAME.md`)
- [x] Define the minimal shell intent vocabulary for focus, route selection, consent, and audit handoff (`PHASE-B-SHELL-WIREFRAME.md`)

## Next (Phase A — after G1)

- [ ] Confirm `/site/` layout with S1
- [ ] Add honest status banner to site (links to STATUS.md)
- [ ] Execute the archive plan from `DOCS-HTML-PROTOTYPE-AUDIT.md` once inbound links are checked
- [ ] Verify no dead links in site

## Next (Phase B — after G3)

- [ ] Scaffold `crates/ferros-web/` or equivalent web shell server
- [ ] Translate `PHASE-B-SHELL-WIREFRAME.md` into a fixed-slot HTML shell with visible collapse anchors
- [ ] Agent list surface (reads from S3 registry via JSON/RPC)
- [ ] Agent detail surface
- [ ] Capability grant surface
- [ ] Deny log surface
- [ ] Wire to real `ferros-node` backend — no fake data

## Later (Phase C — post-G3)

- [ ] Compile `ferros-core` to `wasm32-unknown-unknown`
- [ ] WASM demo page in `/site/wasm-demo/`
- [ ] CI job to verify WASM compilation

## Blocked

- Phase A: blocked on G1.
- Phase B: blocked on G3 (needs S3 JSON/RPC API).
