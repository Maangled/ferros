# S5 UX — Contracts

---

## Contracts owned by S5

| Contract | Type | Location | Status |
|----------|------|----------|--------|
| Web shell server port / URL convention | Convention | TBD (`ferros-node` config) | ⬜ Post-G3 |
| Local shell intent vocabulary | Internal UI design boundary | `PHASE-B-SHELL-WIREFRAME.md` | 🟨 Pre-G3 design active |
| UI component contracts (Phase B) | TypeScript/HTML | `crates/ferros-web/` | ⬜ Post-G3 |

---

## Contracts consumed by S5

| Contract | Source | Purpose |
|----------|--------|---------|
| S3 JSON/RPC API | S3 | Phase B web shell reads agent registry and status |
| `no_std` WASM boundary | S4 | Phase C WASM demo requires `ferros-core` to compile to WASM |
| Site structure (`/site/`) | S1 | Phase A site cleanup depends on S1 layout decision |

---

## Notes

S5 is a consumer stream. It does not define new cross-stream data contracts. Its job is to expose existing contracts through a human-readable interface.

The shell intent vocabulary is an internal composition boundary for the future Phase B web shell. It is not a replacement for S3 or S4 contracts.
