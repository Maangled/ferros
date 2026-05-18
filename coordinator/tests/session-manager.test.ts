import { SessionManager } from '../src/session-manager';
import { makePacket } from './fixtures';

function makeSdkStub() {
  const events: Record<string, (event: any) => void> = {};
  const deleted: string[] = [];
  const session = {
    id: 'session-1',
    on: (event: string, cb: (e: any) => void) => {
      events[event] = cb;
    },
    off: () => {},
    sendAndWait: async ({ prompt }: { prompt: string }) => `response:${prompt}`,
    send: () => {}
  };

  return {
    client: {
      createSession: async () => session,
      deleteSession: async (sessionId: string) => {
        deleted.push(sessionId);
      },
      resumeSession: async () => session,
      listSessions: async () => [{ id: 'session-1', agent: 'FERROS Core Agent', createdAt: new Date().toISOString() }]
    },
    deleted,
    session
  };
}

describe('SessionManager', () => {
  test('creates session and sends prompt', async () => {
    const sdk = makeSdkStub();
    const mgr = new SessionManager({ sdk_client: sdk.client as any });

    const session = await mgr.createSession('core');
    const response = await mgr.sendPrompt(session, 'hello');

    expect(session.id).toBe('session-1');
    expect(response).toBe('response:hello');
  });

  test('resumes and lists sessions', async () => {
    const sdk = makeSdkStub();
    const mgr = new SessionManager({ sdk_client: sdk.client as any });

    const resumed = await mgr.resumeSession('session-1');
    const sessions = await mgr.listActiveSessions();

    expect(resumed.id).toBe('session-1');
    expect(sessions.length).toBeGreaterThan(0);
  });

  test('registers orchestrator packet on session creation when configured', async () => {
    const sdk = makeSdkStub();
    const fetchCalls: Array<{ url: string; init: { method: string; headers?: Record<string, string>; body?: string } }> = [];
    const mgr = new SessionManager({
      sdk_client: sdk.client as any,
      orchestrator_base_url: 'http://127.0.0.1:4417/',
      fetch_impl: async (url, init) => {
        fetchCalls.push({ url, init });
        return {
          ok: true,
          status: 200,
          statusText: 'OK',
          text: async () => ''
        };
      }
    });

    await mgr.createSession('core', makePacket());

    expect(fetchCalls).toHaveLength(1);
    expect(fetchCalls[0].url).toBe('http://127.0.0.1:4417/orchestrator/packets');
    expect(fetchCalls[0].init.method).toBe('POST');
    expect(fetchCalls[0].init.headers?.['Idempotency-Key']).toBe(
      'ferros-coordinator:core:FRS-core-20260510-C1-W1:WO-CORE-20260510-C1-W1'
    );
    expect(JSON.parse(fetchCalls[0].init.body || '{}')).toEqual({
      sessionId: 'session-1',
      manager: 'FERROS Core Agent',
      summary: 'Coordinator handoff to FERROS Core Agent for WO-CORE-20260510-C1-W1',
      originMessageId: 'FRS-core-20260510-C1-W1',
      workOrderId: 'WO-CORE-20260510-C1-W1',
      lifecycleThreadId: 'cycle-core-20260510-C1-W1',
      idempotencyKey: 'ferros-coordinator:core:FRS-core-20260510-C1-W1:WO-CORE-20260510-C1-W1'
    });
  });

  test('cleans up SDK session when orchestrator registration fails', async () => {
    const sdk = makeSdkStub();
    const mgr = new SessionManager({
      sdk_client: sdk.client as any,
      orchestrator_base_url: 'http://127.0.0.1:4417',
      fetch_impl: async () => ({
        ok: false,
        status: 503,
        statusText: 'Service Unavailable',
        text: async () => 'orchestrator offline'
      })
    });

    await expect(mgr.createSession('core', makePacket())).rejects.toThrow(
      'Failed to enqueue orchestrator packet (503 Service Unavailable): orchestrator offline'
    );
    expect(sdk.deleted).toEqual(['session-1']);
  });
});
