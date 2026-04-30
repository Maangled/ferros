# S5 — UX

**Stream:** S5  
**Status:** 🟨 Phase A active on the real landing page; Phase B localhost shell slice landed with narrow lifecycle/profile controls and display-only onramp observation
**Gate:** Contributes to launch-readiness; no blocking gate owned solely by S5

> Current checkpoint: the first localhost browser profile surface is landed for `init`, `show`, `export`, and `import` through the local `/profile` adapter with focused route-test coverage, and the existing runway route now renders pending-consent proposed material plus the recorded local decision rehearsal receipt from `/runway-summary.json` as display-only, local-only, non-evidentiary observation with same-origin H9 proof. Remaining S5 follow-up is onramp accept/reject wiring without reopening the frozen S2 contract or widening browser privileges.

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
- **S2:** the first localhost browser profile surface consumes the frozen `ferros profile init | show | export | import` boundary through the local `/profile` adapter with focused route-test coverage, and the remaining S5 work is limited to holding that narrow boundary steady while onramp accept/reject wiring proceeds separately without reopening G2, schema shape, or browser privilege scope.
- **S3 + S4 (post-G3):** the current localhost shell host exposes a local-only `agent.run` / `agent.stop` JSON/RPC slice above `LocalAgentApi`, and the Phase B shell now wires a selected-agent lifecycle control bar against that backend slice. The browser gate checks loaded active grant rows before transmitting either write. Grant/revoke actions, remote transport, and broader privileged flows remain later follow-up work.

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
- [x] Operator-assisted localhost acceptance can prove local `ferros agent run | stop` changes read back through the same `agent.snapshot` refresh seam.
- [x] Browser-issued local lifecycle control bar is wired for selected-agent `agent.run` / `agent.stop` only. The shell checks loaded active grant rows before write RPC transmission, requires an explicit arm checkbox, refreshes through `agent.snapshot` after success or backend denial, and the same-origin harness now proves an unarmed or missing-grant click does not transmit `agent.run` / `agent.stop`.
- [x] Existing runway route renders pending-consent proposed material plus the recorded decision rehearsal receipt from the read-only `/runway-summary.json` surface, and the same-origin H9 harness proves that observation stays local-only, non-evidentiary, and display-only with no in-surface controls.

The current Phase B slice is still read-first for observation, with one narrow localhost-only lifecycle write bar, one narrow localhost-only profile surface, and one display-only onramp observation path on the existing runway route. That profile slice is landed for `init`, `show`, `export`, and `import` through the local `/profile` adapter only, and it is already backed by focused `ferros-node` route tests and Rust validation. The remaining S5 follow-up keeps `show` off JSON-RPC and leaves profile `grant` / `revoke`, onramp accept/reject wiring, remote transport, and broader browser privileges out of scope.

The shell now stages selected-agent lifecycle intent copy and can submit selected-agent `agent.run` / `agent.stop` only through the grant-aware local lifecycle bar.

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

The minimum consent-gated browser-issued lifecycle control bar above the staged shell-intent copy is now wired in `site/agent-center-shell.html` and covered by `harnesses/localhost-shell-acceptance-harness.html`.

| Constraint | Definition |
|------------|------------|
| Scope | `agent.run` and `agent.stop` only, for the currently selected agent on the current localhost shell. No other write actions, no grant mutation, no `revoke`, no broader browser control. |
| Consent/audit gating | Gating begins **before** the write RPC is sent from the browser. The shell checks the loaded `agent.snapshot` grant rows for each selected agent's required capabilities and requires an explicit arm checkbox before transmitting the call. |
| Deny visibility | If the backend denies a transmitted lifecycle request, the shell refreshes through `agent.snapshot` so any persisted deny evidence remains visible through the audit slot and deny-log route without requiring a separate observation path. |
| Deny-by-default demonstration | The S3/S4 backend path remains covered by focused denied-lifecycle RPC tests that persist a deny-log entry. The shell harness covers the browser pre-write gate; live operator proof still depends on having persisted deny state or a backend denial to read back. |
| Publication gate | The wired bar is limited to selected-agent local lifecycle actions. Focused Rust route/RPC tests and the same-origin harness asset now cover the served control surface and the pre-write browser gate; live harness validation remains the session-level proof when the localhost shell is running. |

Grant/revoke actions, consent resolution for non-lifecycle operations, broader browser control, and S4 restart/reload semantics remain out of scope for this bar and require a separate follow-up.

---

## Phase B: minimum profile surface entry bar

The minimum honest first browser profile surface entry bar above the frozen S2 contract is now landed as the first localhost browser profile slice in `site/agent-center-shell.html` and `crates/ferros-node/src/lib.rs`, with `init`, `show`, `export`, and `import` running through the thin local `/profile` adapter. That existing `ferros-node` adapter path is route-test-backed now; the remaining S5 follow-up is to keep the surface narrow without reopening the frozen S2 contract or browser privileges.

| Constraint | Definition |
|------------|------------|
| Scope | `init`, `show`, `export`, `import` only. Localhost-only. No grant mutation. No `revoke`. No re-negotiation of the S2 contract. No browser privilege widening. |
| Backend | Each slot calls the already-frozen CLI path (`ferros profile init`, `show`, `export`, `import`) through the thin local `/profile` adapter. `show` stays on `/profile` and does not route through JSON-RPC. Does not reopen `schemas/profile.v0.json` or `schemas/capability-grant.v0.json`, and does not widen the read-first JSON/RPC contract. |
| Prior art | `docs/legacy/personal-profile.html` is the shape reference only. Does not constitute G2 re-evidence. |
| Publication gate | The first slice is landed and route-test-backed. The harness checks `/profile` separately from `/rpc`, proves `show` avoids JSON-RPC, and proves profile `grant` / `revoke` controls remain absent. |

Grant mutation, `revoke`, remote profile access, any S2 contract reopening, and any browser privilege widening remain explicitly out of scope for this surface.

---

## Phase B: consent-flow copy spec (draft)

> **Draft — pending counsel red-line.** This spec is derived from `docs/legal/CONSENT-LANGUAGE.md` DRAFT. It carries the same DRAFT status and must not be used as final consent language until `CONSENT-LANGUAGE.md` has been reviewed by counsel and the draft status has been cleared.

The following spec defines the user-visible copy for the S5 consent gate. It is derived directly from the draft sections in `docs/legal/CONSENT-LANGUAGE.md` and mirrors their structure. S5 is the first consumer of these draft language sections.

| Copy slot | Source section in CONSENT-LANGUAGE.md | Draft copy | Notes |
|-----------|---------------------------------------|------------|-------|
| Capability grant consent | "Capability Grant Language" | "You are granting [agent name] the ability to [capability description]. This grant is stored locally and can be revoked at any time from the consent log. Granting this capability does not share your data with any third party unless you explicitly configure an external integration." | Use counsel-approved variant when available. |
| Onramp accept consent | "Onramp Accept Language" | "You are accepting [data description] from [external system name] into your FERROS profile. This data will become part of your local FERROS state. You can review or remove it from your profile at any time. Accepting this data does not affect [external system name] — it remains in your local FERROS installation only." | Use counsel-approved variant when available. |
| Deny-visibility disclosure | "Denial and Deny-by-Default" | "FERROS denies capability requests by default. If an agent you expect to work is not functioning, check the deny log. A denial is not permanent — you can issue a capability grant at any time." | Use counsel-approved variant when available. |

Once `CONSENT-LANGUAGE.md` is cleared by counsel, the draft tags on this spec and those sections must be removed together in a coordinated update.

---

## Phase B: onramp consent surface entry bar

The read-only observation half of this onramp path is now landed on the existing runway route: the shell displays pending-consent proposed material plus the recorded local decision rehearsal receipt from the read-only `/runway-summary.json` surface, and the same-origin H9 harness proves that observation stays local-only, non-evidentiary, and display-only with no in-surface controls. The accept/reject consent surface defined below is the stated next onramp-facing surface; it does not yet exist as wired browser code. ADR-023 is the governing policy.

| Constraint | Definition |
|------------|------------|
| Scope | One slot per proposed onramp item. Shows: source system name, proposed item description (e.g., HA entity name and type, calendar event title, contact display name), consent prompt, accept/reject affordance. Localhost-only. |
| Governing invariant | Inbound data is quarantined until accepted (ADR-023). Passively having an integration enabled is not consent. The accept action must be auditable — loggable as a consent event through the existing S3/S4 audit-log surface. |
| What the slot does NOT do | The slot does not automatically populate profile, capability grants, or any sealed progression record. It does not call the external system. It does not invent HA bridge protocol details, pairing handshake order, or consent UI internals beyond what is stated here. |
| Publication gate | The display-only observation half is already landed on the runway route. The accept/reject slot lands only after the S3/S4 audit-log seam can capture an explicit accept event for a staged onramp item, and a harness proves the item does not reach canonical state without an explicit user action. |

Calendar, social-graph, and marketplace onramp variants follow the same pattern. The entry bar definition is channel-agnostic; channel-specific consent copy is derived from `docs/legal/CONSENT-LANGUAGE.md` (draft).

---

## Immediate next steps

1. Verify the remaining site links and archive candidates against current inbound references.
2. Execute the archive plan from `DOCS-HTML-PROTOTYPE-AUDIT.md` once link hygiene is confirmed.
3. Implement the localhost onramp consent surface only after the audit-log seam can record explicit accept events, without widening browser privileges or reopening the frozen S2 boundary.
