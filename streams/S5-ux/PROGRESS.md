# S5 UX — Progress

Reverse-chronological. Append a dated entry at the top per session.

---

## 2026-04-30 - Proposal plus decision observation proved on the existing runway route

- Extended `site/agent-center-shell.html` so the existing runway route now renders both pending-consent proposed material and the recorded local decision rehearsal receipt from the read-only `/runway-summary.json` surface in both the runway panel and the inspector without adding a new route, accept/reject controls, or browser-issued grant/revoke controls.
- Extended `harnesses/localhost-shell-acceptance-harness.html` so the same-origin H9 path now proves the runway route button copy, restart-aware route copy, proposal-plus-decision read-only claim ceiling, decision receipt field set on the runway surface, and matching inspector detail on the same route.
- Focused validation passed with `cargo test -p ferros-node shell_route_serves_localhost_acceptance_harness`, clean harness diagnostics, and a rebuilt live same-origin H9 runway/proposal/decision run on port 4319 that finished `70 passed`, `0 failed`, `2 skipped`.
- Kept the slice local-only and narrow: no accept/reject flow, no canonical mutation, no remote transport, no broader browser privilege, and no D1/G4 or hardware claim.

## 2026-04-29 - Display-only onramp observation landed on the existing runway route

- Extended `site/agent-center-shell.html` so the existing runway route now renders pending-consent proposed material from the read-only `/runway-summary.json` surface in both the runway panel and the inspector without adding a new route, accept/reject controls, or browser-issued grant/revoke controls.
- Extended `harnesses/localhost-shell-acceptance-harness.html` so the same-origin H9 path now proves the runway route button copy, proposal read-only claim ceiling, pending-consent proposal field set on the runway surface, no in-surface controls, and matching inspector detail on the same route.
- Focused validation passed with `cargo test -p ferros-node shell_route_serves_localhost_acceptance_harness`, clean harness diagnostics, and a rebuilt live same-origin H9 runway/onramp run on port 4319.
- Kept the slice local-only and narrow: no accept/reject flow, no canonical mutation, no remote transport, no broader browser privilege, and no D1/G4 or hardware claim.

## 2026-04-28 - Same-origin local lifecycle and profile proof loop closed on the live shell

- Extended `site/agent-center-shell.html` so the live localhost shell now renders structured `/profile` adapter outcomes for local `init`, `show`, `export`, and `import` only, with `show` staying off JSON-RPC and profile `grant` / `revoke` controls still absent.
- Extended `harnesses/localhost-shell-acceptance-harness.html` so one embedded same-origin path now proves the narrow local lifecycle allow/deny loop and the `/profile` adapter outcomes together, including refreshed deny visibility after a revoked-grant backend rejection.
- Focused validation passed with `cargo test -p ferros-node shell_route_serves_localhost_acceptance_harness` and a rebuilt live harness run that finished `55 passed`, `0 failed`, `0 skipped`.
- Kept the slice local-only and narrow: no profile grant/revoke, no remote transport, no broader browser privilege, and no D1/G4 or hardware claim.

## 2026-04-28 - Localhost profile surface checkpoint wired, Rust validation blocked

- Added a Profile route and profile action controls to `site/agent-center-shell.html` for local `init`, `show`, `export`, and `import` only.
- Wired those controls to a same-origin `/profile` adapter in `crates/ferros-node/src/lib.rs`, backed by the existing S2 CLI/store paths rather than the read-first JSON/RPC contract.
- Extended `harnesses/localhost-shell-acceptance-harness.html` so it monitors `/profile` separately from `/rpc` and checks that profile `show` does not transmit JSON-RPC while profile `grant` and `revoke` controls stay absent.
- Node inline-script syntax checks passed for the shell and harness. Rust validation is blocked in this environment because cargo cannot execute the local Rust toolchain (`Access is denied`) and escalation was rejected by the environment usage gate, so the profile surface remains an implementation checkpoint rather than a cleanly closed S5 item.
- Kept frozen S2 schemas untouched and did not add browser profile grant/revoke, remote profile access, D1/G4 evidence, or ADR-024 changes.

## 2026-04-28 - Consent-gated browser lifecycle control bar landed

- Extended `site/agent-center-shell.html` so the live localhost shell can send selected-agent `agent.run` / `agent.stop` over the existing local-only RPC path only after the loaded grant rows satisfy the agent's required capabilities and the operator arms the action.
- Extended `harnesses/localhost-shell-acceptance-harness.html` so the same-origin harness proves an unarmed or missing-grant lifecycle click does not transmit `agent.run` or `agent.stop`, while retaining read-after-write observation through `agent.snapshot`.
- Added served-asset assertions in `crates/ferros-node/src/lib.rs` so `cargo test -p ferros-node shell_route_` locks the embedded lifecycle control and harness gate proof.
- Focused validation passed with `cargo test -p ferros-node shell_route_`, `cargo test -p ferros-node agent_write_rpc_`, `cargo test -p ferros-node shell_listener_posts_json_rpc_`, and a Node syntax check of the two inline shell/harness scripts. Live browser harness validation remains the next session-level proof when the localhost shell is running.
- Kept the slice narrow: no grant/revoke actions, profile mutation, remote transport, new JSON/RPC methods, D1/G4 evidence claims, or broader S4 restart/reload semantics.

## 2026-04-27 - Selected-agent lifecycle intent copy landed in the live shell

- Extended `site/agent-center-shell.html` so the live localhost shell now stages selected-agent lifecycle intent copy and read-only slot affordances against the landed local-only `agent.run` / `agent.stop` backend slice without issuing browser writes.
- Extended `harnesses/localhost-shell-acceptance-harness.html` so the same-origin shell acceptance path now checks that selected-agent intent copy updates in the audit and tools slots and flips between `agent.run` and `agent.stop` as local state changes are observed.
- Kept the shell read-only: grant/revoke, consent resolution, browser-issued lifecycle writes, and broader browser control remain out of scope.

## 2026-04-27 - Minimum first shell-intent entry bar defined above the landed local-only lifecycle/write slice

- Defined the next honest S5 publication boundary above the landed localhost `agent.run` / `agent.stop` backend slice: the first shell follow-up is selected-agent intent copy and slot ownership only, not a browser-issued write flow.
- Locked the observation rule for that future shell-intent slice to the existing manual refresh plus `agent.snapshot`, `agent.describe`, and `denyLog.list` path so S5 does not invent a second observation seam while staging the UI.
- Kept grant/revoke actions, consent resolution, browser-issued privileged writes, broader browser control, and broader S4 restart/reload claims explicitly out of scope.

## 2026-04-26 - Upstream local-only lifecycle/write JSON-RPC slice landed, but the shell stays observation-only

- Truth-synced the S5 owner docs to the newly landed S3/S4 boundary: the current localhost shell host now supports local-only `agent.run` and `agent.stop` JSON/RPC methods above `LocalAgentApi`.
- Kept the Phase B shell itself observation-only in the docs because S5 has not yet staged UI intents or controls on top of that local-only backend slice, and grant/revoke plus broader browser-control work remain out of scope.

## 2026-04-26 - Phase B write-side dependency narrowed to the future local-only JSON/RPC lifecycle slice

- Truth-synced the S5 owner docs to the newly defined S3 boundary: the next honest write-side dependency for the shell is only a local-only `agent.run` / `agent.stop` JSON/RPC slice above `LocalAgentApi` on the current localhost shell host.
- Kept the shell observation-only in the docs until that code-backed slice exists, and kept grant/revoke plus broader browser-control work explicitly out of scope for the current Phase B state.

## 2026-04-26 - Operator-assisted local lifecycle observation added to the live shell harness

- Extended `harnesses/localhost-shell-acceptance-harness.html` so the same-origin acceptance path can pause for out-of-band local `ferros agent run echo` and `ferros agent stop echo` commands, refresh the shell, and prove those local lifecycle changes still read back through exactly one `agent.snapshot` call.
- Kept the shell observation-only: the harness adds no shell write controls, and live deny generation remains outside the current shell and CLI surface, so deny observation stays optional and only applies when external local state is pre-seeded.
- Truth-synced the S5 backlog and README to distinguish the landed operator-assisted lifecycle observation proof from the still-open privileged write and broader browser-control follow-up.

## 2026-04-25 - Snapshot seam consumed in the live shell and harness

- Switched the shell from fan-out `agent.list` / `grant.list` / `denyLog.list` plus `agent.describe` to one `agent.snapshot` read and local snapshot reuse for inspector selection.
- Updated the same-origin localhost acceptance harness so it monitors `/rpc` fetches, asserts exactly one `agent.snapshot` refresh call, asserts zero extra RPCs while selecting a loaded agent, and drives the iframe profile-path input through the correct frame-window event constructor.
- Live browser validation passed at `http://127.0.0.1:4317/` and `http://127.0.0.1:4317/harnesses/localhost-shell-acceptance.html`, with the harness finishing 16/16 checks against the real local shell.
- Kept the slice observation-only: no privileged writes, no grant mutation claims, and no G4 evidence.
- Editor diagnostics are clean on the touched S5 files in this session.

## 2026-04-24 - Five docs prototypes archived into docs/legacy

- Moved `home-hud-dashboard.html`, `architecture-design-lab.html`, `architecture-design-lab-builder.html`, `ferros-mind-map.html`, and `ferros-project-map.html` from `docs/` into `docs/legacy/`.
- Updated the archive truth surfaces to record the verified split: `agent-command-center.html` and `forge-workbench.html` stay as active priors in `docs/`, while `personal-profile.html`, `schedule-ledger.html`, `deployment-roadmap.html`, `algo-trading-arena.html`, and `ferros-showcase.html` remain in `docs/` for now because current repo links still target them.
- Fixed the moved `ferros-mind-map.html` relative links so archive peers stay same-folder and kept docs-root targets now resolve through `../`.

## 2026-04-24 — Same-origin localhost shell acceptance harness landed

- Added `harnesses/localhost-shell-acceptance-harness.html` as a dedicated live-shell acceptance surface served by `ferros-node shell` rather than stretching the older file-based prototype harness past its real boundary.
- Extended `crates/ferros-node/src/lib.rs` so the local shell host now serves the harness at `/harnesses/localhost-shell-acceptance.html`, keeping the harness same-origin with `GET /` and `POST /rpc` so it can black-box the real DOM and live read-first transport.
- Focused validation passed in two layers: `cargo test -p ferros-node shell_route_` stayed green, and live browser validation at `http://127.0.0.1:4317/harnesses/localhost-shell-acceptance.html` passed 13/13 checks across route switching, agent detail, grant empty-state degradation, deny-log empty state, and the read-only audit slot.
- Kept the slice read-first: no new JSON/RPC methods, no privileged write actions, no S5 prototype archive moves, and no claim that broader browser acceptance is fully closed.

## 2026-04-24 — First localhost shell slice landed

- Added `site/agent-center-shell.html` as the first real fixed-slot localhost shell and wired it to the read-first S3 JSON/RPC routes via `ferros-node shell [port]`.
- Extended `crates/ferros-node/src/lib.rs` so the current local host now serves `GET /` for the shell and `POST /rpc` for the existing read-first JSON/RPC contract instead of relying on fake data or a parallel transport path.
- Added focused shell route tests to `cargo test -p ferros-node`, then browser-validated the live shell at `http://127.0.0.1:4317/` against real agent, grant-state, and deny-log data.
- Fixed the inspector capability rendering bug so required capabilities now show the real profile identifier instead of an `undefined:*` placeholder after the shell asset is rebuilt into the binary.
- Kept the wave read-first: no privileged write actions, no grant mutation through the shell, and no claim that Phase A cleanup is complete.

---

## 2026-04-24 — Prototype authority and archive note landed

- Added a surface-authority map to `site/index.html` so the live landing page now points readers to the two kept docs prototypes as reference-only priors and to the archive note.
- Added prototype-status banners to `docs/agent-command-center.html` and `docs/forge-workbench.html` so each page states it is incubation prior art rather than a shipped localhost shell.
- Added `docs/legacy/html-prototype-status.html` to separate active references from archive material without moving files before inbound-link checks.
- Kept the lane inside Phase A doc and link hygiene: no Phase B shell implementation, no `STATUS.md` or gate doc edits, and no claim that the local shell is further along than blocked pre-G3 work.

## 2026-04-23 — Landing-page status banner and docs sync landed

- Added a repository-status banner to `site/index.html` so the real FERROS landing page now states that Phase A site cleanup is active, the local agent-center shell remains Phase B work pending G3, and launch is still hardware-first.
- Updated `README.md` so S5 no longer reads as if Phase A is waiting on initial `/site/` bring-up; marked the landing-page move and status banner as landed work.
- Updated `BACKLOG.md` to treat Phase A as active repo work instead of a G1-blocked placeholder.
- Kept the lane fully inside Phase A: no local web shell work, no JSON/RPC work, and no changes to S3 or S4 code.

## 2026-04-23 — Prototype audit and shell wireframe landed

- Added `DOCS-HTML-PROTOTYPE-AUDIT.md` to classify the top-level `docs/*.html` prototypes into explicit keep/archive/remove decisions.
- Kept `agent-command-center.html` as the Phase B subject-matter prior and `forge-workbench.html` as the shell-layout prior.
- Marked the remaining top-level docs prototypes for archive during Phase A cleanup; proposed no removals in this sprint to avoid destructive churn before link checks.
- Added `PHASE-B-SHELL-WIREFRAME.md` to turn the surface-first shell note into a concrete slot map, workflow budget, and minimal typed shell intent vocabulary.
- Left all work inside `streams/S5-ux/` and avoided changes to S3, S4, S6, S8, or `STATUS.md`.

## 2026-04-23 — Surface-first shell note landed

- Added `SURFACE-FIRST-SHELL.md` as the current Phase B UX artifact.
- Captured the rule that S5 composes named surfaces in fixed home slots rather than treating the local shell as a draggable window manager.
- Added the six-degree reach rule so inspect, capability grant, and deny-log workflows have a measurable shell-depth limit before HTML work starts.
- Anchored the note to existing Forge shell behavior and ADR-019 guidance on slot composition, focus-mode chrome, and typed shell intents.
- Updated the backlog so the next build slice is a slot-based wireframe plus minimal shell intent vocabulary.

## 2026-04-21 — Stream scaffolded

- Stream directory and planning documents created.
- Phase A blocked on G1 (S1 Foundation).
- Phase B blocked on G3 (S3 + S4 minimal demo).
- Phase C is background work post-G3.
- Existing HTML prototypes in `docs/` need an audit pass to determine what to archive.
