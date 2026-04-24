# S5 UX — Contracts

---

## Contracts owned by S5

| Contract | Type | Location | Status |
|----------|------|----------|--------|
| Web shell server port / URL convention | Convention | `crates/ferros-node/src/lib.rs`, `site/agent-center-shell.html` | 🟡 `ferros-node shell [port]` serves the shell at `http://127.0.0.1:4317/` by default; custom ports remain allowed |
| Local shell intent vocabulary | Internal UI design boundary | `PHASE-B-SHELL-WIREFRAME.md`, `site/agent-center-shell.html` | 🟡 Wireframe landed and the first fixed-slot read slice now consumes it |
| UI component contracts (Phase B) | HTML/CSS/JS | `site/agent-center-shell.html` | 🟡 Fixed-slot read-first shell landed; privileged write actions remain open |

---

## Contracts consumed by S5

| Contract | Source | Purpose |
|----------|--------|---------|
| S3 JSON/RPC API | S3 | Phase B web shell reads agent registry, grant state, and deny-log data |
| `no_std` WASM boundary | S4 | Phase C WASM demo requires `ferros-core` to compile to WASM |
| Site structure (`/site/`) | S1 | Phase A site cleanup depends on S1 layout decision |

---

## Notes

S5 is a consumer stream. It does not define new cross-stream data contracts. Its job is to expose existing contracts through a human-readable interface.

The shell intent vocabulary is an internal composition boundary for the future Phase B web shell. It is not a replacement for S3 or S4 contracts.

The current localhost serving convention is intentionally narrow: `GET /` serves the shell and `POST /rpc` forwards the read-first JSON/RPC surface. Method ownership remains with S3, while S5 owns only the presentation and localhost-shell composition boundary.
