# H1 Current Shell Read Surfaces

Session ID: OPS-2026-05-05-H1
Date: 2026-05-05
Operator: TBD at session start
Backlog item: HTB-001, HTB-002, HTB-008, HTB-009, HTB-011, HTB-012

Goal:

Inspect the current shell read surfaces as one grouped session: selected-agent clarity, deny visibility and recovery posture, proposed-material inspection, receipt readback, tool-lane disclosure, and evidence-badge honesty.

Commands or routes to use:

1. From the repo root, run `cargo run -p ferros-node --bin ferros-node -- shell 4326`.
2. Open `http://127.0.0.1:4326/`.
3. In the Local profile path field, enter `/definitely/missing/profile.json`.
4. Select `Agents`, choose one visible agent, and inspect the center surface, status rail, tools copy, and inspector detail.
5. Select `Runway` and inspect `Pending consent proposed material`, `Local decision rehearsal receipt`, `Consent boundary`, `Operator recovery posture`, and their evidence badges.
6. Select `Deny log` and confirm the shell renders a visible live or empty state without losing route context.
7. Record whether the tool and audit lane copy stays explicit about local-only scope, blocked consent, and the absence of hidden writes.

Expected observation:

1. `Agents` makes the selected agent, lifecycle intent, required capabilities, and status-rail context easy to identify.
2. `Runway` keeps proposed material, local decision rehearsal receipt, blocked consent boundary, and recovery posture on one read-only surface.
3. Evidence badges remain backed by named seams and restrained wording such as `display-only`, `non-evidentiary`, `local-only`, or `blocked` rather than unsupported verification claims.
4. Tools and audit copy remains explicit about scope and limits: no hidden grant mutation, no remote transport, and no canonical mutation.
5. `Deny log` stays readable as persisted evidence or an honest empty state.

Evidence capture path:

`docs/operator-sessions/findings/OPS-2026-05-05-H1-current-shell-read-surfaces.md`

Rollback path:

1. Stop the local shell server.
2. Do not retain extra profile or bundle artifacts for this session.
3. If any route becomes misleading, stop the session and route the issue as a hotfix or Agent Backlog item.

Stop criteria:

1. The shell stops rendering one of the target routes.
2. Any route implies authoritative evidence, grant mutation, canonical acceptance, or remote transport without a named seam.
3. Evidence-badge wording overclaims certainty or security.
4. The deny surface, tool-lane disclosure, or route-local artifact copy becomes hidden or ambiguous.

Immediate-task comments:

Record exact strings or badges that feel too strong, too vague, or inconsistent across Agents, Runway, and Deny log.

Meta comments for coordinator:

Route broader architecture or queue-priority comments to the coordinator rather than folding them into a single route pass/fail note.