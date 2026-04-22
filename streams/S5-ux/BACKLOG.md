# S5 UX — Backlog

---

## Now

- [ ] Audit `docs/` HTML prototypes — list what to keep, archive, or remove
- [ ] Sketch Phase B UI wireframe against S3 JSON/RPC API

## Next (Phase A — after G1)

- [ ] Confirm `/site/` layout with S1
- [ ] Add honest status banner to site (links to STATUS.md)
- [ ] Archive dead HTML prototypes to `docs/legacy/` or remove
- [ ] Verify no dead links in site

## Next (Phase B — after G3)

- [ ] Scaffold `crates/ferros-web/` or equivalent web shell server
- [ ] Agent list view (reads from S3 registry via JSON/RPC)
- [ ] Agent describe view
- [ ] Capability grant view
- [ ] Deny log view
- [ ] Wire to real `ferros-node` backend — no fake data

## Later (Phase C — post-G3)

- [ ] Compile `ferros-core` to `wasm32-unknown-unknown`
- [ ] WASM demo page in `/site/wasm-demo/`
- [ ] CI job to verify WASM compilation

## Blocked

- Phase A: blocked on G1.
- Phase B: blocked on G3 (needs S3 JSON/RPC API).
