# S5 UX — Progress

Reverse-chronological. Append a dated entry at the top per session.

---

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
