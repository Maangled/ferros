import { OrchestrationCoordinator } from '../src/coordinator';
import { isCoordinatorError, isExecutionReturn } from '../src/types';
import { makePacket } from './fixtures';

class MockSession {
  id: string;
  private response: string;
  private handlers: Record<string, ((event: any) => void)[]> = {};

  constructor(id: string, response: string) {
    this.id = id;
    this.response = response;
  }

  on(event: string, handler: (event: any) => void): void {
    if (!this.handlers[event]) {
      this.handlers[event] = [];
    }
    this.handlers[event].push(handler);
  }

  off(): void {}

  send(): void {}

  async sendAndWait(): Promise<string> {
    this.emit('subagent.started', {
      toolCallId: `${this.id}-tc`,
      agent: 'FERROS Core Agent'
    });
    this.emit('subagent.completed', {
      toolCallId: `${this.id}-tc`,
      agent: 'FERROS Core Agent'
    });
    return this.response;
  }

  private emit(event: string, payload: any): void {
    (this.handlers[event] || []).forEach((handler) => handler(payload));
  }
}

class MockSdkClient {
  private response: string;

  constructor(response: string) {
    this.response = response;
  }

  async createSession(): Promise<MockSession> {
    return new MockSession('core-session', this.response);
  }

  async deleteSession(): Promise<void> {}

  async resumeSession(): Promise<MockSession> {
    return new MockSession('core-session', this.response);
  }

  async listSessions(): Promise<any[]> {
    return [];
  }
}

describe('OrchestrationCoordinator', () => {
  test('fails closed when response omits lifecycle outcome', async () => {
    const coordinator = new OrchestrationCoordinator({
      sdk_client: new MockSdkClient('## Facts\n- runtime slice complete'),
      log_level: 'error',
      timeout_ms: 5000,
    });

    const result = await coordinator.handoffToAgent(makePacket(), 'core');

    expect(isCoordinatorError(result)).toBe(true);
    if (isCoordinatorError(result)) {
      expect(result.error).toContain('Lifecycle stop contract failed');
      expect(result.failedChecks).toContain('execution_lifecycle_contract');
    }
  });

  test('accepts response with explicit report outcome', async () => {
    const coordinator = new OrchestrationCoordinator({
      sdk_client: new MockSdkClient(
        '## Report\n- Returned to FERROS Coding with the next runtime action.\n## Facts\n- runtime slice complete'
      ),
      log_level: 'error',
      timeout_ms: 5000,
    });

    const result = await coordinator.handoffToAgent(makePacket(), 'core');

    expect(isExecutionReturn(result)).toBe(true);
    if (isExecutionReturn(result)) {
      expect(result.lifecycle_outcome?.kind).toBe('report');
    }
  });
});