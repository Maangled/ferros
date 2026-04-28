# S5 Live Lifecycle Harness Proof Checklist

Status: Batch F research handoff. This is a proof plan for the already-landed localhost shell lifecycle bar, not a D1 or G4 evidence record.

## Fixed Inputs

- `site/agent-center-shell.html` now exposes selected-agent `agent.run` and `agent.stop` controls behind an active-grant check plus an explicit arm checkbox.
- `harnesses/localhost-shell-acceptance-harness.html` already proves unarmed or missing-grant clicks do not transmit lifecycle write RPC.
- `crates/ferros-node/src/lib.rs` serves the shell and same-origin harness from the local shell host.
- The only lifecycle write methods in scope are the current local-only `agent.run` and `agent.stop` JSON-RPC calls.

## Proof Scope

The next live proof should show four things, in this order:

1. The shell can load a current `agent.snapshot` from the local host.
2. The lifecycle bar refuses to send write RPC while the operator has not armed the action.
3. The lifecycle bar refuses to send write RPC when the selected agent's required capabilities are not covered by loaded active grants.
4. When the operator arms an action and the selected agent has active matching grants, the shell may send exactly one lifecycle write RPC and then refresh through `agent.snapshot`.

This proof is local-shell acceptance evidence only. It does not close D1, does not count as G4 hardware evidence, and does not publish remote transport.

## Operator Checklist

| Step | Expected observation | Stop if |
|------|----------------------|---------|
| Start the local shell host on an unused localhost port | `GET /` serves the shell; the harness path serves the same-origin acceptance page | The shell or harness cannot be loaded from the same host |
| Load the shell without selecting an agent | Lifecycle controls remain inert and no write RPC is sent | A write RPC can be emitted before agent selection |
| Select an agent while no matching active grant is loaded | The consent/audit area explains the missing grant state and the action stays blocked | The browser sends `agent.run` or `agent.stop` before grant coverage exists |
| Arm the action while grant coverage is still missing | The arm state alone is not enough to transmit the write | The arm checkbox bypasses grant coverage |
| Load a profile path with matching active grant state and refresh | The selected agent shows grant coverage sufficient for its requirements | The UI cannot distinguish active grants from absent or revoked grants |
| Arm and click the selected lifecycle action | Exactly one lifecycle write RPC is sent, followed by `agent.snapshot` refresh | Multiple writes are sent for one click or the shell fails to refresh after the write |
| Force a backend denial from the current local path | The denial is visible through the write response and/or refreshed deny-log view | The operator cannot find the denied action after refresh |

## Harness Assertions To Keep

- Count outgoing `/rpc` calls by method name.
- Assert zero write calls before arm.
- Assert zero write calls when active grant coverage is missing.
- Assert exactly one write call for a single armed allowed action.
- Assert one post-write `agent.snapshot` refresh.
- Assert deny visibility is read from existing response and deny-log surfaces, not from a new S5-only state cache.

## Stop Lines

- Do not add grant or revoke actions to this proof.
- Do not add remote transport, auth, subscriptions, or health endpoints.
- Do not edit D1 or G4 evidence tables from this checklist.
- Do not treat browser-local success as hardware or launch evidence.
