# H2 Profile Round Trip

Session ID: OPS-2026-05-05-H2
Date: 2026-05-05
Operator: TBD at session start
Backlog item: HTB-003

Goal:

Confirm that the bounded `Profile` route performs local `init`, `show`, `export`, and `import` through `/profile` without reopening grant or revoke mutation.

Commands or routes to use:

1. From the repo root, run `cargo run -p ferros-node --bin ferros-node -- shell 4326`.
2. Open `http://127.0.0.1:4326/`.
3. In the Local profile path field, enter `.tmp/h2-profile-round-trip.json`.
4. In the Profile bundle path field, enter `.tmp/h2-profile-round-trip.bundle.json`.
5. Select the `Profile` route.
6. Run `Init`, then `Show`, then `Export`, then change the Local profile path to `.tmp/h2-profile-round-trip-imported.json` and run `Import`.
7. Inspect the center surface and the inspector after each action.

Expected observation:

1. Each action settles through `/profile` and renders a structured result on the `Profile` route.
2. The shell keeps the scope bounded to `init`, `show`, `export`, and `import` only.
3. No `grant` or `revoke` controls appear on the `Profile` route.
4. The inspector remains explicit about profile path, bundle path, action, and status summary.

Evidence capture path:

`docs/operator-sessions/findings/OPS-2026-05-05-H2-profile-round-trip.md`

Rollback path:

1. Stop the local shell server.
2. Remove the temporary `.tmp/h2-profile-round-trip*.json` artifacts if created.
3. If the route implies grant mutation or fails to render structured status, stop and route it as a hotfix.

Stop criteria:

1. Any profile action routes through JSON-RPC instead of `/profile`.
2. `grant` or `revoke` controls appear.
3. Export or import implies canonical mutation beyond the local bounded profile surface.
4. The route or inspector stops rendering structured status.

Immediate-task comments:

Record any copy that blurs the line between local bundle handling and wider account, grant, or authority claims.

Meta comments for coordinator:

Route broader questions about future profile mutation scope or remote profile workflows to the coordinator.