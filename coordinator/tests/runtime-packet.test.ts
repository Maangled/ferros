import { normalizeRuntimePacket } from '../src/runtime-packet';

function makeRawPacket() {
  return {
    routeToken: {
      tokenVersion: 'v2',
      issuedBy: 'FERROS Coding Agent',
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
    metadata: {
      lifecycleContract: {
        cycleId: 'thread-41',
        workOrderId: 'wo-41',
        sourceAgentId: 'FERROS Agent',
        targetAgentId: 'core',
        ownerAgentId: 'Software Architect',
        escalationId: 'esc-41',
        escalationTargetAgentId: 'FERROS Agent',
        escalationReasonCode: 'execution-lane-blocked',
        stop: {
          allowedTerminalStates: ['report', 'work_order', 'escalation', 'stopped'],
          stoppedReasonRequired: true,
        },
      },
    },
  };
}

describe('runtime-packet', () => {
  test('normalizes canonical executionContext into execution_context metadata', () => {
    const packet = normalizeRuntimePacket({
      ...makeRawPacket(),
      metadata: {
        ...makeRawPacket().metadata,
        executionContext: {
          sourceKind: 'monitor',
          packetId: 'pkt-41',
          sessionId: 'test-session',
          managerAgentId: 'Software Architect',
          sessionLabel: 'Runtime session',
          lifecycleThreadId: 'thread-41',
          lifecycleThreadTitle: 'Software packet wo-41',
          originMessageId: 'msg-41',
          originMessageText: 'continue the seam',
        },
      },
    });

    expect(packet.metadata?.execution_context).toEqual({
      source_kind: 'monitor',
      packet_id: 'pkt-41',
      session_id: 'test-session',
      manager_agent_id: 'Software Architect',
      session_label: 'Runtime session',
      lifecycle_thread_id: 'thread-41',
      lifecycle_thread_title: 'Software packet wo-41',
      origin_message_id: 'msg-41',
      origin_message_text: 'continue the seam',
    });
  });

  test('maps legacy monitorContext into canonical execution_context metadata', () => {
    const packet = normalizeRuntimePacket({
      ...makeRawPacket(),
      metadata: {
        ...makeRawPacket().metadata,
        monitorContext: {
          packetId: 'pkt-42',
          sessionId: 'test-session',
          manager: 'Software Architect',
          sessionLabel: 'Legacy runtime session',
          lifecycleThreadId: 'thread-42',
          lifecycleThreadTitle: 'Software packet wo-42',
          originMessageId: 'msg-42',
          originMessageText: 'use the continuity agent',
        },
      },
    });

    expect(packet.metadata?.execution_context).toEqual({
      source_kind: 'monitor',
      packet_id: 'pkt-42',
      session_id: 'test-session',
      manager_agent_id: 'Software Architect',
      session_label: 'Legacy runtime session',
      lifecycle_thread_id: 'thread-42',
      lifecycle_thread_title: 'Software packet wo-42',
      origin_message_id: 'msg-42',
      origin_message_text: 'use the continuity agent',
    });
  });
});