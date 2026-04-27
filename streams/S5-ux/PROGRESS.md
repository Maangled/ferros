# S5 UX — Progress

Reverse-chronological. Append a dated entry at the top per session.

---

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
