import { runHandoffCli, parseHandoffCliArgs } from '../src/handoff-cli';
import { makePacket } from './fixtures';

function toCommandRuntimePacket() {
  const packet = makePacket();

  return {
    packet,
    commandRuntimePacket: {
      routeToken: {
        tokenVersion: packet.route_token.token_version,
        issuedBy: packet.route_token.issued_by,
        targetStream: packet.route_token.target_stream,
        targetFamily: packet.route_token.target_family,
        runId: packet.route_token.run_id,
        parentRunId: packet.route_token.parent_run_id,
        recursionDepth: packet.route_token.recursion_depth,
        runProfile: packet.route_token.run_profile,
        issuedAt: packet.route_token.issued_at,
        expiryCycle: packet.route_token.expiry_cycle,
        posture: packet.route_token.posture,
        track: packet.route_token.track,
      },
      payload: packet.payload,
      prompt: packet.prompt,
      signature: packet.signature,
      issuedAt: packet.issued_at,
      ttlMs: packet.ttl_ms,
      metadata: packet.metadata
        ? {
            ...packet.metadata,
            lifecycleContract: packet.metadata.lifecycle_contract
              ? {
                  ...packet.metadata.lifecycle_contract,
                  stop: {
                    allowedTerminalStates:
                      packet.metadata.lifecycle_contract.stop.allowed_terminal_states,
                    stoppedReasonRequired:
                      packet.metadata.lifecycle_contract.stop.stopped_reason_required,
                  },
                }
              : undefined,
          }
        : undefined,
    },
  };
}

describe('handoff-cli', () => {
  test('parses target and coordinator env defaults', () => {
    const args = parseHandoffCliArgs(['--target', 'subcore'], {
      FERROS_COORDINATOR_TIMEOUT_MS: '45000',
      FERROS_COORDINATOR_LOG_LEVEL: 'warn',
      FERROS_COORDINATOR_CAPTURE_EVENTS: 'false',
    });

    expect(args).toEqual({
      targetAgent: 'subcore',
      timeoutMs: 45000,
      logLevel: 'warn',
      captureEvents: false,
    });
  });

  test('runs handoff and prints normalized result JSON', async () => {
    let stdout = '';
    let stderr = '';
    const { packet, commandRuntimePacket } = toCommandRuntimePacket();
    let normalizedPacketParentRunId: string | undefined;

    const exitCode = await runHandoffCli(
      ['--target', 'core'],
      {
        async readInput() {
          return JSON.stringify(commandRuntimePacket);
        },
        writeOutput(text: string) {
          stdout += text;
        },
        writeError(text: string) {
          stderr += text;
        },
      },
      {},
      async (handoffPacket, targetAgent, options) => {
        normalizedPacketParentRunId = handoffPacket.route_token.parent_run_id;

        return {
          classification: `execution-return-${targetAgent}` as const,
          parent_run_id: handoffPacket.route_token.parent_run_id,
          response: `${targetAgent} complete`,
          timestamp: '2026-05-16T00:00:00Z',
          lifecycle_outcome: {
            kind: 'report',
            summary: options?.capture_events === false ? 'events disabled' : 'events enabled',
          },
        };
      }
    );

    expect(exitCode).toBe(0);
    expect(stderr).toBe('');
    expect(normalizedPacketParentRunId).toBe(packet.route_token.parent_run_id);
    expect(JSON.parse(stdout)).toMatchObject({
      classification: 'execution-return-core',
      parentRunId: packet.route_token.parent_run_id,
      response: 'core complete',
      lifecycleOutcome: {
        kind: 'report',
        summary: 'events enabled',
      },
      lifecycleErrors: [],
    });
  });

  test('prints normalized lifecycle contract details on coordinator error', async () => {
    let stdout = '';
    let stderr = '';
    const { commandRuntimePacket } = toCommandRuntimePacket();

    const exitCode = await runHandoffCli(
      ['--target', 'core'],
      {
        async readInput() {
          return JSON.stringify(commandRuntimePacket);
        },
        writeOutput(text: string) {
          stdout += text;
        },
        writeError(text: string) {
          stderr += text;
        },
      },
      {},
      async () => ({
        error: 'Lifecycle stop contract failed',
        failedChecks: ['execution_lifecycle_contract'],
        details: {
          errors: [
            "Lifecycle outcome 'denied' is not allowed by the packet stop contract",
          ],
          lifecycle_outcome: {
            kind: 'denied',
            summary: 'policy blocked',
            target_agent_id: 'FERROS Agent',
          },
          lifecycle_errors: ['Denied outcome is outside the declared stop contract'],
        },
        escalate: true,
        timestamp: '2026-05-16T00:00:00Z',
      })
    );

    expect(exitCode).toBe(0);
    expect(stderr).toBe('');
    expect(JSON.parse(stdout)).toMatchObject({
      error: 'Lifecycle stop contract failed',
      failedChecks: ['execution_lifecycle_contract'],
      details: {
        errors: [
          "Lifecycle outcome 'denied' is not allowed by the packet stop contract",
        ],
        lifecycleOutcome: {
          kind: 'denied',
          summary: 'policy blocked',
          targetAgentId: 'FERROS Agent',
        },
        lifecycleErrors: ['Denied outcome is outside the declared stop contract'],
      },
    });
  });
});