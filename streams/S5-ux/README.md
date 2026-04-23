# S5 — UX

**Stream:** S5  
**Status:** 🟨 Phase A active; Phase B blocked on G3  
**Gate:** Contributes to launch-readiness; no blocking gate owned solely by S5

---

## Mission

WASM in the browser is the *forcing function* for clean API boundaries, not the public launch vehicle. Ship a working local web UI for the agent center; ship a WASM demo of `ferros-core` on the marketing site. Demoted from "launch artifact" — the real launch is hardware-first (see `LAUNCH.md`).

---

## Scope

### Phase A — Site cleanup (unblocks at G1)
- Move `ferros-blueprint.html` to `/site/index.html` (coordinated with S1).
- Add an honest status banner to the site.
- Clean up dead HTML prototypes or archive to `docs/legacy/`.
- Make `ferros-blueprint.html` accessible as the primary site.

### Phase B — Agent center local web shell (unblocks at G3)
- Local web UI served by `ferros-node` on `localhost`.
- Talks to `ferros-agents` (S3) over JSON/RPC.
- Shows: agent list, agent status, capability grants, deny log.
- ADR-019 is the prior-art boundary for shell composition, typed IPC, and capability-scoped UI messaging.

### Phase C — WASM demo (post-G3, background)
- Compile `ferros-core` to WASM.
- Embed a capability-gate demo in the marketing site (`site/`).
- Used to validate `no_std`/WASM boundary hygiene.

---

## Out of scope

- Agent implementation logic (S3).
- Runtime policy engine (S4).
- Home Assistant integration (S7).
- Public marketing site copy or branding decisions.

---

## Dependencies

- **S1 (G1):** Site structure and workspace must exist for Phase A.
- **S3 + S4 (G3):** JSON/RPC API from S3 must exist for Phase B.

---

## What this stream blocks

- Launch-readiness perception (Phase B is required for a usable system).
- WASM boundary validation for `ferros-core` `no_std` feature (Phase C).

---

## Definition of done per phase

**Phase A:**
- [ ] `/site/index.html` serves `ferros-blueprint.html` content.
- [ ] Site has an honest status banner (links to `STATUS.md`).
- [ ] No dead links in the site.

**Phase B:**
- [ ] Local web shell at `http://localhost:<port>` served by `ferros-node`.
- [ ] Agent list, agent describe, capability grant view — no fake data.
- [ ] Deny log visible in the UI.

**Phase C:**
- [ ] `ferros-core` compiles to `wasm32-unknown-unknown` with `no_std`.
- [ ] Demo page embedded in `/site/` shows a live capability grant/deny interaction.

---

## Likely crates / files

| Path | Role |
|------|------|
| `site/` | Static site root |
| `site/index.html` | `ferros-blueprint.html` moved here (S1 handles move) |
| `crates/ferros-web/` | Web shell server (Phase B) |
| `site/wasm-demo/` | WASM demo (Phase C) |

---

## Immediate next steps

1. Coordinate with S1 on site structure — confirm `/site/` layout.
2. Audit existing HTML prototypes in `docs/`; identify what to archive.
3. Design Phase B UI wireframe against S3's JSON/RPC API (can begin pre-G3).
