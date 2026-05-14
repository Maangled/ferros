import { handoffToBoth } from '../src/coordinator';
import { Packet } from '../src/types';

class MockSession {
  id: string;
  private handlers: Record<string, ((event: any) => void)[]> = {};

  constructor(id: string) {
    this.id = id;
  }

  on(event: string, handler: (event: any) => void): void {
    if (!this.handlers[event]) this.handlers[event] = [];
    this.handlers[event].push(handler);
  }

  off(): void {}
  send(): void {}

  async sendAndWait({ prompt }: { prompt: string }): Promise<string> {
    const isSubcore = this.id.includes('subcore');
    this.emit('subagent.started', {
      toolCallId: `${this.id}-tc`,
      agent: isSubcore ? 'FERROS SubCore Agent' : 'FERROS Core Agent'
    });

    const response = this.id.includes('subcore')
      ? '## Report\n- Returned to FERROS Coding with x86_64 rehearsal notes.\n## Facts\n- subcore completed\n## Claims\n- rehearsal proven\n## Residual Risks\n- hardware gap'
      : '## Report\n- Returned to FERROS Coding with runtime boundary notes.\n## Facts\n- core completed\n## Claims\n- runtime boundary proven\n## Residual Risks\n- integration pending';

    this.emit('subagent.completed', { toolCallId: `${this.id}-tc`, result: response });
    return response;
  }

  private emit(event: string, payload: any): void {
    (this.handlers[event] || []).forEach(h => h(payload));
  }
}

class MockSdkClient {
  async createSession(options: any): Promise<MockSession> {
    const agent = options?.agent || 'unknown';
    const id = agent.toLowerCase().includes('subcore') ? 'subcore-session' : 'core-session';
    return new MockSession(id);
  }

  async deleteSession(_sessionId: string): Promise<void> {}
  async resumeSession(_sessionId: string): Promise<MockSession> { return new MockSession('resumed-session'); }
  async listSessions(): Promise<any[]> { return []; }
}

function packet(target: 'core' | 'subcore', parent = 'FRS-coding-20260510-C1-W0', depth = 1, ttlMs = 300000): Packet {
  return {
    route_token: {
      token_version: 'v2',
      issued_by: 'FERROS Prompt Architect Agent',
      target_stream: target,
      target_family: null,
      run_id: `FRS-${target}-20260510-C1-W1`,
      parent_run_id: parent,
      recursion_depth: depth,
      issued_at: new Date().toISOString(),
      expiry_cycle: 'C1',
      posture: 'interactive',
      track: 'code'
    },
    payload: `${target}-payload`,
    prompt: `${target}-prompt`,
    issued_at: new Date().toISOString(),
    ttl_ms: ttlMs,
    metadata: {
      lifecycle_contract: {
        cycle_id: `cycle-${target}-20260510-C1-W1`,
        work_order_id: `WO-${target.toUpperCase()}-20260510-C1-W1`,
        source_agent_id: 'FERROS Coding Agent',
        target_agent_id: target,
        owner_agent_id: 'FERROS Coding Agent',
        escalation_id: `ESC-${target.toUpperCase()}-20260510-C1-W1`,
        escalation_target_agent_id: 'FERROS Agent',
        escalation_reason_code: 'execution-lane-blocked',
        stop: {
          allowed_terminal_states: ['report', 'work_order', 'escalation', 'stopped'],
          stopped_reason_required: true,
        }
      }
    }
  };
}

async function runDualExecutionHarness(): Promise<void> {
  const sdkClient = new MockSdkClient();

  const corePacket = packet('core');
  const subcorePacket = packet('subcore');

  // This call validates both packets independently, runs both handoffs in parallel,
  // and returns normalized execution-return structures.
  const result = await handoffToBoth(corePacket, subcorePacket, {
    timeout_ms: 5000,
    log_level: 'info',
    sdk_client: sdkClient
  });

  console.log('Dual execution result:');
  console.log(JSON.stringify(result, null, 2));

  // TTL failure scenario
  const staleCore = packet('core', 'FRS-coding-20260510-C1-W0', 1, 1);
  staleCore.issued_at = new Date(Date.now() - 60000).toISOString();
  const ttlResult = await handoffToBoth(staleCore, subcorePacket, { timeout_ms: 5000, sdk_client: sdkClient });
  console.log('TTL scenario result:');
  console.log(JSON.stringify(ttlResult, null, 2));

  // Recursion ceiling scenario
  const deepSubcore = packet('subcore', 'FRS-coding-20260510-C1-W0', 3);
  const recursionResult = await handoffToBoth(corePacket, deepSubcore, { timeout_ms: 5000, sdk_client: sdkClient });
  console.log('Recursion scenario result:');
  console.log(JSON.stringify(recursionResult, null, 2));
}

runDualExecutionHarness().catch((err) => {
  console.error('Harness failed:', err);
  process.exit(1);
});
