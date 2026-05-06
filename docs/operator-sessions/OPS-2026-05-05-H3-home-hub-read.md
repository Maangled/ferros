# H3 Home-Hub Read

Session ID: OPS-2026-05-05-H3
Date: 2026-05-05
Operator: TBD at session start
Backlog item: HTB-010

Goal:

Inspect the new `Home-Hub` route as a read-only topology and source-lineage surface without treating Home Assistant or host state as canonical identity truth.

Commands or routes to use:

1. From the repo root, run `cargo run -p ferros-node --bin ferros-node -- shell 4326`.
2. Open `http://127.0.0.1:4326/`.
3. In the Local profile path field, enter `/definitely/missing/profile.json`.
4. Select the `Home-Hub` route.
5. Inspect the topology observation card, consent boundary, proposed material, rehearsal receipt, restart lineage, and the inspector pane.

Expected observation:

1. The route names a bridge, stand-in entity, proposal, route-local artifact, and snapshot lineage when available.
2. The route remains display-only, local-only, and non-canonical.
3. The route does not imply Home Assistant identity truth, executed consent, or sealed hardware proof.
4. The inspector keeps bridge and lineage fields explicit enough for later audit work.

Evidence capture path:

`docs/operator-sessions/findings/OPS-2026-05-05-H3-home-hub-read.md`

Rollback path:

1. Stop the local shell server.
2. Do not infer any device-side authority from the route.
3. If topology or lineage wording overclaims certainty, stop and route it to the coordinator.

Stop criteria:

1. The route fails to load.
2. The route implies canonical Home Assistant state, executed consent, or remote transport.
3. The route loses source-lineage or artifact naming.
4. The inspector no longer exposes bridge or lineage fields clearly.

Immediate-task comments:

Record whether the topology language is clear enough to support later Home-Hub audit or operator work.

Meta comments for coordinator:

Route broader questions about Home-Hub write flows, device truth, or audit assignment to the coordinator.