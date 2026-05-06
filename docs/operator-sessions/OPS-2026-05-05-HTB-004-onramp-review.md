# HTB-004 Onramp Review

Session ID: OPS-2026-05-05-HTB-004
Date: 2026-05-05
Operator: TBD at session start
Backlog item: HTB-004

Goal:

Confirm that the current runway route shows proposed onramp material, the local decision rehearsal receipt, the blocked consent boundary, and the operator recovery posture honestly while keeping accept/reject closed and non-canonical.

Commands or routes to use:

1. From the repo root, run `cargo run -p ferros-node --bin ferros-node -- shell 4326`.
2. Open `http://127.0.0.1:4326/`.
3. In the Local profile path field, enter `/definitely/missing/profile.json`.
4. Select the `Runway` route.
5. Inspect the runway surface and the inspector pane without refreshing away from the route.

Optional host-touchscreen preflight on this machine:

1. Prefer the single-cable motherboard USB-C path if it works; otherwise record the fallback cable path explicitly.
2. Record the Linux host display/input stack used for the session: connector path, display server/compositor if relevant, and the observed driver/module names exposed by the host.
3. Treat those host drivers as observed external modules, not sealed FERROS evidence.

Expected observation:

1. The runway route renders `Pending consent proposed material`, `Local decision rehearsal receipt`, `Consent boundary`, and `Operator recovery posture` on the same read-only surface.
2. The runway route exposes no accept or reject buttons and no in-surface controls for canonical mutation.
3. The consent-boundary copy keeps canonical state blocked, shows a route-local artifact path, and states that an audit seam is required before canonical state can change.
4. The recovery-state copy names the current posture, keeps the checkpoint and reload state explicit, and states the next honest move without implying accepted or canonical onramp state.
5. The route continues to describe the material as local-only, display-only, and non-evidentiary, with no remote transport, Home Assistant proof, or G4 closure claims.

Evidence capture path:

`docs/operator-sessions/findings/OPS-2026-05-05-HTB-004-onramp-review.md`

If the session uses the touchscreen, include a short host-touchscreen preflight block in the findings: connector path, driver/module names, and whether the shell stayed usable without hover-only interactions.

Rollback path:

1. Stop the local shell server.
2. Do not retain any extra local profile or grant artifacts from this session.
3. If the route becomes misleading, stop the session and route the issue to the coordinator as a hotfix or Agent Backlog item.

Stop criteria:

1. The runway route fails to load.
2. Any accept or reject control appears on the runway surface.
3. Any copy implies canonical acceptance, grant creation, remote transport, Home Assistant proof, or closed-gate evidence.
4. The consent-boundary card or recovery-state card is missing.

Immediate-task comments:

Record exact strings that feel misleading, too strong, or too weak for the blocked-state and recovery posture.

Meta comments for coordinator:

Route broader policy, architecture, or lane-priority comments to the coordinator instead of folding them into the pass or fail result.