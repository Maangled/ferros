# FERROS Orchestrator Coordinator Architecture

> **Status:** Implementation design (ready to code)
> **Component:** TypeScript module in `coordinator/`
> **Role:** Mediate inter-agent handoff (Coding Agent ↔ Core/SubCore agents)
> **Authority:** FERROS Orchestration Architect Agent
> **Related:** [HANDOFF-2026-05-09-COPILOT-SDK-VERIFIED.md](HANDOFF-2026-05-09-COPILOT-SDK-VERIFIED.md)

---

## Purpose

The Coordinator is an internal orchestration layer that:

1. **Validates packets** before inter-agent handoff (all 5 guardrails)
2. **Mediates SDK calls** to `@github/copilot-sdk` (create sessions, send prompts, handle events)
3. **Signs and verifies packet payloads** with HMAC to prevent tampering
4. **Traces execution lineage** via `parent_run_id` → `toolCallId` mapping
5. **Normalizes responses** back to `execution-return-{core|subcore}` classifications
6. **Enforces recursion ceiling** at depth 2 (platform hard-limit + coordinator check + agent spec check)
7. **Caps TTL** and prevents self-handoff loops

The Coordinator is **internal tooling**, not a user-facing agent. FERROS Coding Agent invokes it implicitly during inter-agent routing.

---

## Architecture: Three Layers

### Layer 1: Validation & Authorization (Coordinator)

```
Coding Agent provides packet
  ↓
Coordinator checks all 5 guardrails
  ├─ Packet valid? (route_token, target_stream, run_id)
  ├─ Recursion depth ≤ 2?
  ├─ parent_run_id present?
  ├─ TTL not expired?
  └─ target ≠ source?
  ↓ all pass
Session creation + handoff allowed
  ↓ any fail
Block + escalate + request corrected packet
```

**Coordinator responsibility:**
- `PacketValidator.validate(packet)` — structural checks
- `GuardrailChecker` — all 5 checks in sequence
- `PacketSigner` — HMAC sign outgoing, verify return
- Error logging with `toolCallId` for triage

### Layer 2: Session Lifecycle (SDK Wrapper)

```
Coordinator approved + signed packet
  ↓
SDK createSession(target_agent, customAgents, tools)
  ├─ Set infer: false on coordinator (no self-delegate)
  ├─ Set infer: true on core/subcore (allow specialists)
  └─ Attach tool scopes per agent
  ↓
SessionManager creates and returns ClientSession
  ├─ Listen for subagent.started (capture toolCallId)
  ├─ Listen for subagent.completed (capture response)
  └─ Listen for subagent.failed (log + escalate)
  ↓
sendAndWait(prompt) — blocking call
  ↓
Response captured + normalized
  ↓
deleteSession(sessionId) — cleanup
```

**SessionManager responsibility:**
- `createSession(agentId, tools, scope)` — pre-target agent, set infer policy
- `sendAndWait(prompt)` — blocking prompt/response
- Event listener attachment
- Session cleanup

### Layer 3: Response Normalization (Back to Coding Agent)

```
SDK response captured
  ↓
EventTracer maps:
  ├─ toolCallId (from subagent.started event)
  ├─ parent_run_id (from incoming packet)
  └─ execution context (what changed, what was proven)
  ↓
Normalizer returns execution-return-{core|subcore}
  ├─ classification: "execution-return-core" | "execution-return-subcore"
  ├─ parent_run_id: (preserved for routing)
  ├─ tool_call_id: (for triage & audit)
  ├─ response: (full payload)
  └─ timestamp: (when completed)
  ↓
Execution return sent back to Coding Agent
  ↓
Coding Agent re-classifies and routes next packet
```

**Response Normalizer responsibility:**
- `normalizeResponse(response, packet, toolCallId)` → `ExecutionReturn`
- Extract facts, claims, non-claims from response
- Preserve lineage (parent_run_id, tool_call_id)
- Return classification-ready object

---

## Module Structure

```
coordinator/src/
├── index.ts                      # Exports main coordinator API
├── coordinator.ts                # Main entry point: handoffToAgent(packet, targetAgent)
├── packet-validator.ts           # Validate route_token, target_stream, run_id
├── packet-signer.ts              # HMAC-SHA256 sign/verify packet payloads
├── guardrails.ts                 # All 5 guardrail checks in sequence
├── session-manager.ts            # SDK session creation, sendAndWait, cleanup
├── event-tracer.ts               # Event listener, toolCallId mapping, response capture
└── types.ts                      # TypeScript interfaces (Packet, ExecutionReturn, etc.)
```

### coordinator.ts (Main Entry Point)

```typescript
export async function handoffToAgent(
  packet: Packet,
  targetAgent: 'core' | 'subcore',
  options?: CoordinatorOptions
): Promise<ExecutionReturn | CoordinatorError> {
  
  // Step 1: Validate all 5 guardrails
  const guardrailResult = await guardrails.checkAll(packet, targetAgent);
  if (!guardrailResult.passed) {
    return {
      error: 'Guardrail failed',
      failedChecks: guardrailResult.failedChecks,
      details: guardrailResult.details,
      escalate: true
    };
  }
  
  // Step 2: Sign packet
  const signature = signer.signPacket(packet.payload, SECRET_KEY);
  const signedPrompt = injectSignature(packet.prompt, signature);
  
  // Step 3: Create session
  const session = await sessionManager.createSession(targetAgent);
  
  // Step 4: Send + wait for response
  const response = await session.sendAndWait({ prompt: signedPrompt });
  
  // Step 5: Normalize response
  const executionReturn = tracer.normalizeResponse(response, packet);
  
  // Step 6: Cleanup
  await sessionManager.cleanupSession(session.id);
  
  return executionReturn;
}

// For dual-execution (Core + SubCore simultaneous)
export async function handoffToBoth(
  corePacket: Packet,
  subcorePacket: Packet,
  options?: CoordinatorOptions
): Promise<{ core: ExecutionReturn, subcore: ExecutionReturn } | CoordinatorError> {
  
  const [coreResult, subcoreResult] = await Promise.all([
    handoffToAgent(corePacket, 'core', options),
    handoffToAgent(subcorePacket, 'subcore', options)
  ]);
  
  // Handle partial failures (one passed, one failed)
  if (isCoordinatorError(coreResult) || isCoordinatorError(subcoreResult)) {
    return {
      error: 'Dual handoff partially failed',
      core: coreResult,
      subcore: subcoreResult,
      escalate: true
    };
  }
  
  return { core: coreResult, subcore: subcoreResult };
}
```

### packet-validator.ts

```typescript
export function validatePacket(packet: Packet): ValidationResult {
  const errors: string[] = [];
  
  // Check 1: Route token present and structured
  if (!packet.route_token) errors.push("Missing route_token");
  if (packet.route_token?.token_version !== 'v2') errors.push("Invalid token_version");
  
  // Check 2: Target stream valid
  const { target_stream, target_family } = packet.route_token || {};
  if (target_stream && !['core', 'subcore'].includes(target_stream)) {
    errors.push(`Invalid target_stream: ${target_stream}`);
  }
  
  // Check 3: Run ID continuous
  if (!packet.route_token?.run_id) errors.push("Missing run_id");
  // (More sophisticated continuity check can compare against prior run history)
  
  return {
    valid: errors.length === 0,
    errors
  };
}
```

### guardrails.ts

```typescript
export async function checkAll(packet: Packet, targetAgent: string): Promise<GuardrailResult> {
  const results = {
    check1_PacketValid: await check1_PacketValid(packet),
    check2_RecursionDepth: await check2_RecursionDepth(packet),
    check3_ParentRunId: await check3_ParentRunId(packet),
    check4_TTLValid: await check4_TTLValid(packet),
    check5_SelfHandoffPrevention: await check5_SelfHandoffPrevention(packet, targetAgent)
  };
  
  const failedChecks = Object.entries(results)
    .filter(([_, passed]) => !passed)
    .map(([name, _]) => name);
  
  return {
    passed: failedChecks.length === 0,
    failedChecks,
    details: results
  };
}

export async function check1_PacketValid(packet: Packet): Promise<boolean> {
  const validation = validatePacket(packet);
  return validation.valid;
}

export async function check2_RecursionDepth(packet: Packet): Promise<boolean> {
  const depth = packet.route_token?.recursion_depth ?? 0;
  return depth <= 2;
}

export async function check3_ParentRunId(packet: Packet): Promise<boolean> {
  return !!packet.route_token?.parent_run_id;
}

export async function check4_TTLValid(packet: Packet): Promise<boolean> {
  const { issued_at, ttl_ms } = packet.route_token || {};
  if (!issued_at || !ttl_ms) return true; // TTL optional but if present, must be valid
  
  const issuedTime = new Date(issued_at).getTime();
  const expiryTime = issuedTime + ttl_ms;
  return Date.now() < expiryTime;
}

export async function check5_SelfHandoffPrevention(packet: Packet, targetAgent: string): Promise<boolean> {
  const targetStream = packet.route_token?.target_stream;
  return targetStream !== 'coordinator';
}
```

### session-manager.ts

```typescript
export async function createSession(targetAgent: 'core' | 'subcore'): Promise<ClientSession> {
  const agentDef = targetAgent === 'core' ? CORE_AGENT_DEF : SUBCORE_AGENT_DEF;
  
  return await SDK_CLIENT.createSession({
    agent: agentDef.id,
    customAgents: [agentDef],
    onPermissionRequest: async (agent, request) => {
      // Log permission request for audit
      console.log(`Permission request from ${agent}: ${request.description}`);
      return { approved: true };
    }
  });
}

export async function cleanupSession(sessionId: string): Promise<void> {
  await SDK_CLIENT.deleteSession(sessionId);
}
```

### event-tracer.ts

```typescript
export function setupEventListeners(session: ClientSession, packet: Packet) {
  let toolCallId: string;
  
  session.on('subagent.started', (event) => {
    toolCallId = event.toolCallId;
    console.log(`Handoff started: toolCallId=${toolCallId}`);
  });
  
  session.on('subagent.completed', (event) => {
    console.log(`Handoff completed: toolCallId=${event.toolCallId}`);
  });
  
  session.on('subagent.failed', (event) => {
    console.error(`Handoff failed: toolCallId=${event.toolCallId}, error=${event.error}`);
  });
  
  return { getToolCallId: () => toolCallId };
}

export function normalizeResponse(
  response: string,
  packet: Packet,
  toolCallId?: string
): ExecutionReturn {
  return {
    classification: packet.route_token.target_stream === 'core' 
      ? 'execution-return-core' 
      : 'execution-return-subcore',
    parent_run_id: packet.route_token.parent_run_id,
    tool_call_id: toolCallId,
    response,
    timestamp: new Date().toISOString()
  };
}
```

### types.ts

```typescript
export interface Packet {
  route_token: RouteToken;
  payload: string;
  prompt: string;
  issued_at: string;
  ttl_ms?: number;
}

export interface RouteToken {
  token_version: 'v2';
  target_stream: 'core' | 'subcore';
  target_family?: 'coding' | 'business' | 'architect';
  run_id: string;
  parent_run_id: string;
  recursion_depth: number;
  issued_at: string;
  expiry_cycle?: string;
}

export interface ExecutionReturn {
  classification: 'execution-return-core' | 'execution-return-subcore';
  parent_run_id: string;
  tool_call_id?: string;
  response: string;
  timestamp: string;
}

export interface CoordinatorError {
  error: string;
  failedChecks?: string[];
  details?: any;
  escalate: boolean;
}

export interface GuardrailResult {
  passed: boolean;
  failedChecks: string[];
  details: Record<string, boolean>;
}
```

---

## Integration with Coding Agent

The Coordinator is invoked **implicitly** by Coding Agent during inter-agent routing:

```
Coding Agent receives execution-return-core
  ↓ (internal)
Coding Agent calls Prompt Architect to construct Core continuation packet
  ↓ (internal)
Prompt Architect returns ready-to-paste packet with route_token, parent_run_id, recursion_depth
  ↓ (internal)
Coding Agent calls coordinator.handoffToAgent(packet, 'core')
  ↓
Coordinator validates, signs, creates session, sends, captures response
  ↓
Coordinator returns ExecutionReturn classification
  ↓ (internal)
Coding Agent re-classifies next step
```

**Coding Agent does not call SDK directly.** Coordinator handles all SDK plumbing.

---

## Dual-Execution Flow

When Coding Agent receives both Core + SubCore reports in one message:

```
Coding Agent classifies as dual-execution-return
  ↓
Prompt Architect constructs both packets (invoke twice, parallel)
  ↓ receives corePacket + subcorePacket
Coding Agent calls coordinator.handoffToBoth(corePacket, subcorePacket)
  ↓
Coordinator validates both independently (all 5 checks per packet)
  ↓
Coordinator creates both sessions in parallel
  ↓
Coordinator sends both prompts in parallel (Promise.all)
  ↓
Coordinator normalizes both responses
  ↓
Returns { core: ExecutionReturn, subcore: ExecutionReturn }
  ↓
Coding Agent returns both execution returns in response
```

---

## Error Handling & Escalation

### Guardrail Failures

If any of the 5 guardrails fails:

```
coordinator.handoffToAgent(packet, targetAgent)
  ↓ guardrail fails (e.g., recursion_depth = 3)
Returns:
{
  error: 'Guardrail failed: recursion_depth > 2',
  failedChecks: ['check2_RecursionDepth'],
  details: { recursion_depth: 3 },
  escalate: true
}
  ↓
Coding Agent receives error
  ↓
Coding Agent escalates to FERROS Coding Agent Architect or FERROS Audit Recovery Officer Agent
```

### SDK Failures

If `createSession` or `sendAndWait` throws:

```
Coordinator catches exception
  ↓
Logs with packet signature, toolCallId, error details
  ↓
Returns CoordinatorError with escalate: true
  ↓
Coding Agent routes to FERROS Audit Recovery Officer Agent
```

---

## Testing Strategy

### Unit Tests (packet-validator, guardrails, signer)

```typescript
// Test valid packet
const validPacket = { route_token: { token_version: 'v2', ... } };
expect(validatePacket(validPacket).valid).toBe(true);

// Test missing route_token
const invalidPacket = { /* no route_token */ };
expect(validatePacket(invalidPacket).valid).toBe(false);

// Test recursion_depth > 2
const deepPacket = { route_token: { recursion_depth: 3 } };
expect(check2_RecursionDepth(deepPacket)).toBe(false);

// Test TTL expiry
const expiredPacket = { 
  route_token: { issued_at: '2026-01-01', ttl_ms: 60000 }
};
expect(check4_TTLValid(expiredPacket)).toBe(false);
```

### Integration Harness (end-to-end)

```typescript
// Mock SDK with mocked Core/SubCore responses
const mockSDK = {
  createSession: async () => mockSession,
  deleteSession: async () => {}
};

// Simulate Coding Agent → Coordinator → Core → response
const result = await coordinator.handoffToAgent(testPacket, 'core');
expect(result.classification).toBe('execution-return-core');
expect(result.parent_run_id).toBe(testPacket.route_token.parent_run_id);
```

---

## Deployment Notes

1. **Location:** Coordinator is built as a TypeScript module in `coordinator/`, not a Rust crate
2. **Dependency:** Requires `@github/copilot-sdk` (public preview)
3. **Initialization:** SDK client instantiated once at startup, reused for session creation
4. **Logging:** All guardrail failures, handoff attempts, and responses logged with `parent_run_id` and `tool_call_id` for audit trail
5. **Backward compatibility:** Manual copy-paste workflows remain supported in agent specs; Coordinator is an internal optimization layer

---

## Future Extensibility

1. **Backend abstraction:** If SDK changes, encapsulate SDK calls in `SessionManager` interface so alternative backends are pluggable
2. **Multi-family support:** Current design scoped to Coding → Core/SubCore; Business Agent handoff can reuse pattern with `target_family: business` extension
3. **Session pooling:** If performance requires, session reuse via `resumeSession` for same target + same parent_run_id
4. **Observability:** Structured logging can be routed to Log Triage Agent for deeper failure analysis

---

*Last updated: 2026-05-10*
