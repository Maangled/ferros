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
- **S3 + S4 (post-G3):** the current localhost shell host now exposes a local-only `agent.run` / `agent.stop` JSON/RPC slice above `LocalAgentApi`, and the Phase B shell now stages selected-agent lifecycle intent copy against that backend slice while still keeping browser-issued writes out; broader browser control and privileged write flows remain later follow-up work.

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

The current Phase B slice is intentionally read-first and still read-only. Privileged grant/revoke actions and broader browser acceptance coverage remain follow-up work.

The shell now stages selected-agent lifecycle intent copy and read-only slot affordances above the upstream local-only lifecycle/write JSON/RPC slice, but it still does not submit browser-issued writes.

## First shell-intent slice

The first shell-side follow-up above the landed localhost `agent.run` / `agent.stop` backend slice is now landed as selected-agent intent copy and read-only slot affordances:

- Scope it to the currently selected agent on the current localhost shell only; do not invent a second browser-side control path or imply remote transport.
- Stage intent as shell copy and read-only affordances in the existing focus, tools, and consent/audit slots so the user can see where local lifecycle intent belongs before the shell is allowed to send write RPC.
- Keep read-after-intent observation on the current manual refresh plus `agent.snapshot`, `agent.describe`, and `denyLog.list` surfaces instead of introducing subscriptions, background polling claims, or a second observation path.
- Keep grant/revoke actions, consent resolution, browser-issued privileged writes, broader browser control, and S4 restart/reload semantics out of scope until a later code-backed follow-up exists.

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

## Phase B: browser-issued lifecycle control bar

The minimum consent-gated browser-issued lifecycle control bar above the staged shell-intent copy is defined as follows. This is the stated next honest surface; it does not yet exist as wired browser code.

| Constraint | Definition |
|------------|------------|
| Scope | `agent.run` and `agent.stop` only, for the currently selected agent on the current localhost shell. No other write actions, no grant mutation, no `revoke`, no broader browser control. |
| Consent/audit gating | Gating begins **before** the write RPC is sent from the browser. The shell must confirm a capability grant exists for the requested action before transmitting the call. |
| Deny visibility | A denied request must be observable in the shell's deny-log slot after the attempt without requiring navigation away from the current view. |
| Deny-by-default demonstration | At least one denied request must be loggable and visible through the current S3 deny-log surface before the control bar is declared wired. |
| Publication gate | This control bar lands only after the consent/audit gating path is observable in the shell, S4 deny-by-default enforcement is confirmed through the S3 read path after a lifecycle attempt, and a harness proves the consent gate fires before the write RPC is transmitted. |

Grant/revoke actions, consent resolution for non-lifecycle operations, broader browser control, and S4 restart/reload semantics remain out of scope for this bar and require a separate follow-up.

---

## Phase B: minimum profile surface entry bar

The minimum honest first browser profile surface entry bar above the frozen S2 contract is defined as follows. This is the stated next profile surface; it does not yet exist as wired browser code.

| Constraint | Definition |
|------------|------------|
| Scope | `init`, `show`, `export`, `import` only. Localhost-only. No grant mutation. No `revoke`. No re-negotiation of the S2 contract. |
| Backend | Each slot calls the already-frozen CLI path (`ferros profile init`, `show`, `export`, `import`) through the JSON/RPC surface or a thin local adapter. Does not reopen `schemas/profile.v0.json` or `schemas/capability-grant.v0.json`. |
| Prior art | `docs/legacy/personal-profile.html` is the shape reference only. Does not constitute G2 re-evidence. |
| Publication gate | This profile surface lands only after the four CLI paths are confirmed reachable through the localhost shell host and a harness proves the surface stays within the frozen S2 boundary. |

Grant mutation, `revoke`, remote profile access, and any S2 contract reopening remain explicitly out of scope for this surface.

---

## Immediate next steps

1. Verify the remaining site links and archive candidates against current inbound references.
2. Execute the archive plan from `DOCS-HTML-PROTOTYPE-AUDIT.md` once link hygiene is confirmed.
3. Land the browser-issued lifecycle control bar (consent/audit gating wired, deny-by-default demonstrable, harness confirms gate fires before write RPC is sent) as a code-backed follow-up.
