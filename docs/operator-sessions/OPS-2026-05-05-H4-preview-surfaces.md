# H4 Preview Surfaces

Session ID: OPS-2026-05-05-H4
Date: 2026-05-05
Operator: TBD at session start
Backlog item: HTB-005, HTB-006

Goal:

Inspect the new `Forge` and `Arena` routes as preview-only surfaces, confirming that authoring, export posture, runtime posture, and result staging remain non-authoritative and non-evidentiary.

Commands or routes to use:

1. From the repo root, run `cargo run -p ferros-node --bin ferros-node -- shell 4326`.
2. Open `http://127.0.0.1:4326/`.
3. In the Local profile path field, enter `.tmp/h4-preview-profile.json`.
4. In the Profile bundle path field, enter `.tmp/h4-preview.bundle.json`.
5. Select `Agents` and choose one visible agent so the shell has an explicit selected-agent runtime subject.
6. Select `Forge` and inspect the preview-only bundle target, profile action summary, and route-local artifact copy.
7. Select `Arena` and inspect the runtime posture, selected-agent detail, deny/recovery posture, and non-evidentiary handoff copy.

Expected observation:

1. `Forge` remains preview-only: bundle target, profile action result, and route-local artifact context are visible without implying authoritative publish or canonical provenance closure.
2. `Arena` remains preview-only: selected-agent runtime posture and handoff language stay non-evidentiary and out of Profile until an explicit future seam exists.
3. The inspector makes the preview boundaries explicit for both routes.
4. Neither route implies minting, public authority, automatic promotion into Profile, or hidden writes.

Evidence capture path:

`docs/operator-sessions/findings/OPS-2026-05-05-H4-preview-surfaces.md`

Rollback path:

1. Stop the local shell server.
2. Remove any temporary `.tmp/h4-preview*` artifacts if created.
3. If either route overclaims authority or provenance, stop and route it as a hotfix or coordinator note.

Stop criteria:

1. `Forge` or `Arena` fails to load.
2. Either route implies canonical publish, profile progression, or hidden mutation.
3. Preview-only or non-evidentiary wording disappears.
4. The inspector no longer shows the relevant preview-boundary fields.

Immediate-task comments:

Record exact strings that feel too weak, too strong, or too ambiguous in the preview-boundary copy.

Meta comments for coordinator:

Route broader ideas about later authoring, provenance, simulation, or progression systems to the coordinator.