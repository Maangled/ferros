# Copilot SDK Inter-Agent Handoff: Verified Platform Surface

> **Status:** Verified ground truth (2026-05-09/10)
> **Source:** `@github/copilot-sdk` (public preview)
> **Maintainer:** FERROS Orchestration Architect Agent
> **Related:** [ORCHESTRATOR-COORDINATOR-ARCHITECTURE.md](ORCHESTRATOR-COORDINATOR-ARCHITECTURE.md)

---

## Verified API Surface

### Session Creation

```typescript
const session = await client.createSession({
  model?: string;                           // optional model ID
  agent?: string;                           // target agent name (e.g., "FERROS Core Agent")
  customAgents?: CustomAgent[];             // per-agent tool/scope definitions
  onPermissionRequest?: (agent, request) => Promise<PermissionResponse>;
});
```

- Pre-targets a named custom agent (e.g., `"FERROS Core Agent"`)
- Returns `ClientSession` object with `sendAndWait` and event listener methods
- No history-passing from prior sessions; continuity via `resumeSession(sessionId)` only

### Blocking Prompt/Response

```typescript
const response = await session.sendAndWait({
  prompt: string;  // full prompt payload with route_token + packet
});
```

- Blocking call; waits for model response
- Returns response text
- No streaming; returns complete response when ready

### Event Listeners

```typescript
session.on('subagent.started', (event) => {
  const { toolCallId, agent } = event;
  // toolCallId is used to derive parent_run_id for response routing
});

session.on('subagent.completed', (event) => {
  const { toolCallId, result } = event;
  // Capture result here
});

session.on('subagent.failed', (event) => {
  const { toolCallId, error } = event;
  // Log error; toolCallId enables tracing
});
```

- Events emitted during session execution
- `toolCallId` links to the invocation and enables parent_run_id derivation
- No timeout event; check session state via polling if needed

### Session Continuity

```typescript
const session2 = await client.resumeSession(sessionId);
```

- Resumes a prior session by ID
- Use only for explicit continuation loops (recursion_depth > 0 in same session)
- Not for history-passing to fresh sessions

### Session Management

```typescript
const sessions = await client.listSessions();        // list all active sessions
await client.deleteSession(sessionId);               // cleanup session
```

- Session cleanup is explicit; no automatic cleanup
- TTL and idle limits are platform-managed (not hardcoded at 30 minutes; enforce coordinator-side)

---

## Per-Agent Scope & Tools

### Custom Agent Definition

```typescript
interface CustomAgent {
  id: string;              // agent identifier
  name: string;            // display name
  description: string;     // agent purpose
  tools: ToolDefinition[]; // available tools
  infer?: boolean;         // auto-delegate to other agents (default: true)
  mcpServers?: MCP[];      // attachable MCP servers
}
```

- `tools` array enforces least-privilege: only listed tools are available
- `infer: false` prevents auto-delegation (use on Coordinator agent)
- `mcpServers` enable MCP integration per agent

### Tool Restriction Pattern

```typescript
const coordinatorAgent = {
  id: 'ferros-orchestrator-coordinator',
  name: 'FERROS Orchestrator Coordinator',
  infer: false,  // coordinator does not delegate; it routes
  tools: [
    // only permit internal routing/validation tools
    // no general code execution or file write
  ]
};

const coreAgent = {
  id: 'ferros-core',
  name: 'FERROS Core Agent',
  infer: true,  // core can delegate to specialists
  tools: [
    // full tool set for core execution
  ]
};
```

---

## FERROS Guardrails Mapping

All five guardrails must be enforced before calling `session.sendAndWait()`:

### 1. Packet Validation

- **Check:** Route token present, `target_stream` matches agent identity, `run_id` is continuous with prior work
- **Enforcement point:** Coordinator calls `validatePacket(packet)` before `createSession`
- **Failure action:** Log error, refuse handoff, escalate to Coding Agent

### 2. Recursion Depth Check

- **Check:** If packet contains `recursion_depth`, confirm it does not exceed 2
- **Enforcement point:** Coordinator checks `packet.route_token.recursion_depth <= 2` before handoff
- **Failure action:** Refuse handoff, escalate upward instead (no self-issue at depth ≥ 2)
- **Platform hard-limit:** Sub-agents are restricted to 1 level of internal recursion; coordinator adds second layer of enforcement

### 3. Parent Packet ID (Traceability)

- **Check:** Packet must include `parent_run_id` for response routing
- **Enforcement point:** Coordinator checks `packet.route_token.parent_run_id !== undefined` before handoff
- **Failure action:** Halt, request corrective packet from Prompt Architect
- **Response tracing:** When `subagent.started` event fires, capture `toolCallId` and map it to `parent_run_id` for return routing

### 4. TTL Check

- **Check:** If packet has `issued_at` and TTL window, confirm not expired
- **Enforcement point:** Coordinator checks `now < issued_at + ttl_ms` before handoff
- **Failure action:** Refuse stale packet, request fresh packet from Prompt Architect
- **Note:** Platform idle limit is platform-managed; coordinator also enforces at handoff time

### 5. Self-Handoff Prevention

- **Check:** Target agent identity must not be source agent identity
- **Enforcement point:** Coordinator checks `packet.route_token.target_stream !== coordinatorIdentity` before handoff
- **Failure action:** Refuse self-handoff, escalate
- **Configuration:** Set `infer: false` on Coordinator agent to prevent implicit self-delegation

---

## Guardrails Enforcement Sequence

```
Coordinator receives packet from Coding Agent
  ↓
Check 1: validatePacket(packet) — route_token, target_stream, run_id
  ↓ pass/fail
Check 2: recursion_depth <= 2 — internal recursion only
  ↓ pass/fail
Check 3: parent_run_id present — traceability
  ↓ pass/fail
Check 4: TTL not expired — issued_at + ttl_ms > now
  ↓ pass/fail
Check 5: target_stream !== source — no self-handoff
  ↓ pass/fail
ALL 5 PASS → createSession + sendAndWait allowed
ANY 1 FAILS → block handoff, escalate, request corrected packet
```

---

## Example Single Handoff (Core Agent)

```typescript
// Coordinator has received packet from Coding Agent

// Step 1: Validate all 5 guardrails
const validationErrors = [];
if (!packet.route_token) validationErrors.push("Missing route_token");
if (packet.route_token.recursion_depth > 2) validationErrors.push("Depth > 2");
if (!packet.route_token.parent_run_id) validationErrors.push("Missing parent_run_id");
if (now > packet.route_token.issued_at + packet.ttl_ms) validationErrors.push("Expired");
if (packet.route_token.target_stream === 'coordinator') validationErrors.push("Self-handoff");

if (validationErrors.length > 0) {
  return { error: "Guardrail check failed", details: validationErrors };
}

// Step 2: Create session targeting Core agent
const session = await client.createSession({
  agent: 'FERROS Core Agent',
  customAgents: [coreAgentDefinition],
  onPermissionRequest: (agent, request) => {
    // Handle permission prompts if needed
    return { approved: true };
  }
});

// Step 3: Listen for traceability events
let toolCallId;
session.on('subagent.started', (event) => {
  toolCallId = event.toolCallId;
  console.log(`Core handoff started, toolCallId=${toolCallId}`);
});

// Step 4: Send packet
const response = await session.sendAndWait({
  prompt: packet.serialized_prompt_with_signature
});

// Step 5: Capture response and normalize back to execution-return
const executionReturn = {
  classification: 'execution-return-core',
  parent_run_id: packet.route_token.parent_run_id,
  tool_call_id: toolCallId,
  response: response,
  timestamp: new Date().toISOString()
};

// Step 6: Cleanup
await client.deleteSession(session.id);

return executionReturn;
```

---

## Dual-Execution Example (Core + SubCore Simultaneous)

```typescript
// Coordinator receives packet FOR Core and packet FOR SubCore

// All 5 guardrails checked independently for each packet
const coreErrors = validate(corePacket);
const subcoreErrors = validate(subcorePacket);

if (coreErrors.length > 0 && subcoreErrors.length > 0) {
  return { error: "Both packets failed validation", core: coreErrors, subcore: subcoreErrors };
}

// Create both sessions in parallel
const [coreSession, subcoreSession] = await Promise.all([
  client.createSession({ agent: 'FERROS Core Agent', customAgents: [...] }),
  client.createSession({ agent: 'FERROS SubCore Agent', customAgents: [...] })
]);

// Send both prompts in parallel
const [coreResponse, subcoreResponse] = await Promise.all([
  coreSession.sendAndWait({ prompt: corePacket.prompt }),
  subcoreSession.sendAndWait({ prompt: subcorePacket.prompt })
]);

// Cleanup both sessions
await Promise.all([
  client.deleteSession(coreSession.id),
  client.deleteSession(subcoreSession.id)
]);

// Return both execution returns
return {
  core: { classification: 'execution-return-core', response: coreResponse, ... },
  subcore: { classification: 'execution-return-subcore', response: subcoreResponse, ... }
};
```

---

## Anti-Patterns (Do Not Use)

| Anti-pattern | Why not | Correct approach |
|--------------|---------|------------------|
| `code chat -m <agent>` CLI | CLI not documented; does not exist in tooling | Use `@github/copilot-sdk` `createSession` |
| Pass prior session history into `createSession` | Not supported; no history continuation parameter | Use `resumeSession(sessionId)` for explicit continuity |
| Hard-coded 30-minute platform idle TTL | TTL is platform-managed; not hardcoded at 30 min | Enforce TTL at coordinator validation layer (check 4) |
| Assume platform blocks sub-agent self-delegation | Platform does not prevent; config prevents | Set `infer: false` on Coordinator agent |
| Skip guardrail validation on "trusted" packets | Attackable if packet is modified mid-transport | All 5 checks are mandatory before every `sendAndWait` |

---

## Non-Verified Surfaces (Out of Scope)

- **VS Code Chat Session Provider API** — Not the FERROS orchestration surface; SDK is correct layer
- **VS Code command `vscode://GitHub.Copilot-Chat/chat?mode=agent`** — Opens UI only, not programmatic
- **Direct window/document access in SDK** — Not available; prompt/response only
- **Streaming responses** — SDK returns complete responses, not streaming

---

## References

- [ORCHESTRATOR-COORDINATOR-ARCHITECTURE.md](ORCHESTRATOR-COORDINATOR-ARCHITECTURE.md) — Coordinator design and implementation guidance
- [ORCHESTRATION-AGENTS.md](ORCHESTRATION-AGENTS.md) — Agent roles and invocation gates
- [FERROS Coding Agent spec](../../.github/agents/ferros-coding-agent.agent.md) — Packet construction and routing
- [FERROS Prompt Architect spec](../../.github/agents/ferros-prompt-architect.agent.md) — Prompt standards and guardrail details

*Last updated: 2026-05-10*
