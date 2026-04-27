# S5 — UX

**Stream:** S5  
**Status:** 🟨 Phase A active on the real landing page; Phase B localhost observation slice landed  
**Gate:** Contributes to launch-readiness; no blocking gate owned solely by S5

---

## Mission

WASM in the browser is the *forcing function* for clean API boundaries, not the public launch vehicle. Ship a working local web UI for the agent center; ship a WASM demo of `ferros-core` on the marketing site. Demoted from "launch artifact" — the real launch is hardware-first (see `LAUNCH.md`).

---

## Scope

### Phase A — Site cleanup
- Move `ferros-blueprint.html` to `/site/index.html` (coordinated with S1).
- Add an honest status banner to the site.
- Clean up dead HTML prototypes or archive to `docs/legacy/`.
- Make `ferros-blueprint.html` accessible as the primary site.

### Phase B — Agent center local web shell (first localhost read slice landed)
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

- **S1 (G1):** Site structure landed; remaining Phase A work continues on the real `/site/index.html`.
- **S3 + S4 (post-G3):** the first read-first JSON/RPC API from S3 now exists for Phase B; privileged write flows remain later follow-up work.

---

## What this stream blocks

- Launch-readiness perception (Phase B is required for a usable system).
- WASM boundary validation for `ferros-core` `no_std` feature (Phase C).

---

## Definition of done per phase

**Phase A:**
- [x] `/site/index.html` serves `ferros-blueprint.html` content.
- [x] Site has an honest status banner (links to `STATUS.md`).
- [ ] No dead links in the site.

**Phase B:**
- [x] Local web shell at `http://localhost:<port>` served by `ferros-node`.
- [x] Agent registry/detail, capability grant state, and deny-log observation consume the aggregated `agent.snapshot` read seam — no fake data.
- [x] Deny log visible in the UI.
- [x] Operator-assisted localhost acceptance can prove local `ferros agent run | stop` changes read back through the same `agent.snapshot` refresh seam without adding shell write controls; live deny generation remains outside the shell and can only be observed when pre-seeded through the existing local lifecycle/log seam.

The current Phase B slice is intentionally read-first. Privileged grant/revoke actions and broader browser acceptance coverage remain follow-up work.

**Phase C:**
- [ ] `ferros-core` compiles to `wasm32-unknown-unknown` with `no_std`.
- [ ] Demo page embedded in `/site/` shows a live capability grant/deny interaction.

---

## Likely crates / files

| Path | Role |
|------|------|
| `site/` | Static site root |
| `site/index.html` | `ferros-blueprint.html` moved here (S1 handles move) |
| `site/agent-center-shell.html` | Fixed-slot localhost shell asset (Phase B) |
| `crates/ferros-node/` | Equivalent local web shell server plus `/rpc` host (Phase B) |
| `site/wasm-demo/` | WASM demo (Phase C) |

---

## Immediate next steps

1. Verify the remaining site links and archive candidates against current inbound references.
2. Execute the archive plan from `DOCS-HTML-PROTOTYPE-AUDIT.md` once link hygiene is confirmed.
3. Keep the shell observation-only while upstream write surfaces remain unpublished, then stage privileged grant/revoke shell intents without bypassing S2/S3/S4 consent boundaries.
