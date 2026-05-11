import { SessionManager } from '../src/session-manager';

function makeSdkStub() {
  const events: Record<string, (event: any) => void> = {};
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
      deleteSession: async () => {},
      resumeSession: async () => session,
      listSessions: async () => [{ id: 'session-1', agent: 'FERROS Core Agent', createdAt: new Date().toISOString() }]
    },
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
});
