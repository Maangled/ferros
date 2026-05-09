# 2026-05-09 Copilot SDK Verified Orchestration Handoff

Authority: docs/orchestration/AUTHORITY-MAP.md

> Repo handoff note for the next session. This is a coordination surface only; it is not evidence.

## Prompt to use for next session

You are picking up FERROS orchestration work. Read this packet before planning or editing.

### Platform reality (verified 2026-05-09)

Use these as ground truth:

- `@github/copilot-sdk` (public preview) is the programmatic orchestration surface.
- `client.createSession({ model, agent, customAgents, onPermissionRequest })` supports pre-targeting a named custom agent.
- `session.sendAndWait({ prompt })` is the blocking prompt/response path.
- `session.on(event => ...)` emits sub-agent lifecycle events including `subagent.started`, `subagent.completed`, and `subagent.failed`.
- Per-agent `tools` can be explicitly scoped for least-privilege.
- Per-agent `infer: boolean` controls auto-selection behavior.
- Per-agent `mcpServers` are attachable per custom agent.
- `client.resumeSession(sessionId, options?)` is the supported cross-session continuity mechanism.
- `client.listSessions(filter?)` and `client.deleteSession(sessionId)` support explicit session cleanup.
- Idle timeout default is not an enforced short platform TTL; coordinator-side TTL enforcement is required when freshness limits matter.

Treat these as non-ground-truth and do not design around them:

- `code chat -m <agent>` CLI assumption.
- VS Code Chat Session Provider API as the FERROS orchestration surface.
- Injecting prior history into a brand-new `createSession` call as continuity.
- A hard baked-in 30-minute idle timeout claim.

### Guardrails for implementation and routing

- Keep FERROS policy ceilings unchanged; do not claim policy movement from this packet alone.
- Keep recursion bounded by FERROS policy (max depth 2), while also respecting platform delegation limits.
- Set `infer: false` on the primary coordinator agent to avoid self-handoff loops.
- Derive parent-run lineage from sub-agent lifecycle event metadata (for example, tool-call IDs) instead of ad hoc IDs.
- Enforce packet freshness with coordinator-side timestamp checks before dispatch.
- Use fresh session creation for isolated tasks and `resumeSession` only for intentional continuity.

### Minimal coordinator skeleton (shape only)

1. Start Copilot client.
2. Create a session with:
   - explicit `model`
   - `agent` set to the intended coordinator name
   - `customAgents` with scoped `tools`
   - `onPermissionRequest` policy
3. Subscribe to `session.on(...)` before first send.
4. Enforce application TTL/freshness checks on the outgoing packet.
5. Call `session.sendAndWait({ prompt })`.
6. Disconnect or resume per explicit session lifecycle policy.

### VS Code/UI scope (non-programmatic)

- `vscode://GitHub.Copilot-Chat/chat?mode=agent` opens agent mode in VS Code.
- GitHub Copilot Agents “Open in VS Code” is a UI handoff flow.
- These do not replace SDK-based programmatic orchestration.

### Non-claims

- No gate closure, launch-grade proof, or STATUS movement is earned by this handoff packet.
- No queue/run-log movement is earned by this handoff packet.
