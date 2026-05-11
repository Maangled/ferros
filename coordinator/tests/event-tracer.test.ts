import { EventTracer } from '../src/event-tracer';
import { makePacket } from './fixtures';

describe('EventTracer', () => {
  test('captures started/completed events and maps toolCallId', () => {
    const tracer = new EventTracer();
    const handlers: Record<string, (event: any) => void> = {};

    const fakeSession = {
      on: (name: string, handler: (event: any) => void) => {
        handlers[name] = handler;
      }
    };

    const packet = makePacket();
    const ctx = tracer.setupEventListeners(fakeSession, packet);

    handlers['subagent.started']({ toolCallId: 'tc-1', agent: 'FERROS Core Agent' });
    handlers['subagent.completed']({ toolCallId: 'tc-1', agent: 'FERROS Core Agent' });

    expect(ctx.getToolCallId()).toBe('tc-1');
    expect(tracer.getParentRunId('tc-1')).toBe(packet.route_token.parent_run_id);
    expect(tracer.getEvents().length).toBe(2);
  });

  test('normalizes response classification by target stream', () => {
    const tracer = new EventTracer();
    const corePacket = makePacket({
      route_token: { ...makePacket().route_token, target_stream: 'core' }
    });
    const result = tracer.normalizeResponse('## Facts\n- ok', corePacket, 'tc-2');

    expect(result.classification).toBe('execution-return-core');
    expect(result.parent_run_id).toBe(corePacket.route_token.parent_run_id);
    expect(result.tool_call_id).toBe('tc-2');
  });
});
