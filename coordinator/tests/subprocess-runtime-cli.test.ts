import {
  parseSubprocessRuntimeCliArgs,
  runSubprocessRuntimeCli,
} from '../src/subprocess-runtime-cli';

function makeCommandRuntimePacket() {
  return {
    routeToken: {
      tokenVersion: 'v2',
      issuedBy: 'FERROS Prompt Architect Agent',
      targetStream: 'core',
      targetFamily: null,
      runId: 'FRS-core-20260516-C1-W1',
      parentRunId: 'monitor:test-session:msg-41',
      recursionDepth: 1,
      issuedAt: '2026-05-16T00:00:00Z',
      expiryCycle: 'C1',
      posture: 'interactive',
      track: 'code',
    },
    payload: 'packet payload',
    prompt: 'prompt payload',
    issuedAt: '2026-05-16T00:00:00Z',
    ttlMs: 300000,
  };
}

describe('subprocess-runtime-cli', () => {
  test('parses required agent and optional overrides', () => {
    const args = parseSubprocessRuntimeCliArgs([
      '--agent',
      'ferros-coding-continuity',
      '--classification',
      'subprocess-continuity',
      '--summary',
      'continuity subprocess completed',
    ]);

    expect(args).toEqual({
      agent: 'ferros-coding-continuity',
      classification: 'subprocess-continuity',
      summary: 'continuity subprocess completed',
    });
  });

  test('reads a command runtime packet and emits normalized result JSON', async () => {
    let stdout = '';
    let stderr = '';
    const packet = makeCommandRuntimePacket();

    const exitCode = await runSubprocessRuntimeCli(
      ['--agent', 'ferros-coding-continuity'],
      {
        async readInput() {
          return JSON.stringify(packet);
        },
        writeOutput(text: string) {
          stdout += text;
        },
        writeError(text: string) {
          stderr += text;
        },
      }
    );

    expect(exitCode).toBe(0);
    expect(stderr).toBe('');
    expect(JSON.parse(stdout)).toMatchObject({
      classification: 'subprocess-ferros-coding-continuity',
      parentRunId: packet.routeToken.parentRunId,
      response: 'ferros-coding-continuity subprocess handoff complete',
      lifecycleOutcome: {
        kind: 'report',
        summary: 'ferros-coding-continuity subprocess runtime completed',
      },
    });
  });
});