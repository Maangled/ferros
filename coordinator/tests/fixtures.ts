import { Packet, PacketLifecycleContract } from '../src/types';

export function makeLifecycleContract(
  targetAgent: 'core' | 'subcore' = 'core',
  overrides: Partial<PacketLifecycleContract> = {}
): PacketLifecycleContract {
  return {
    cycle_id: `cycle-${targetAgent}-20260510-C1-W1`,
    work_order_id: `WO-${targetAgent.toUpperCase()}-20260510-C1-W1`,
    source_agent_id: 'FERROS Coding Agent',
    target_agent_id: targetAgent,
    owner_agent_id: 'FERROS Coding Agent',
    escalation_id: `ESC-${targetAgent.toUpperCase()}-20260510-C1-W1`,
    escalation_target_agent_id: 'FERROS Agent',
    escalation_reason_code: 'execution-lane-blocked',
    stop: {
      allowed_terminal_states: ['report', 'work_order', 'escalation', 'stopped'],
      stopped_reason_required: true,
    },
    ...overrides,
  };
}

export function makePacket(overrides: Partial<Packet> = {}): Packet {
  const route_token = {
    token_version: 'v2' as const,
    issued_by: 'FERROS Prompt Architect Agent' as const,
    target_stream: 'core' as const,
    target_family: null,
    run_id: 'FRS-core-20260510-C1-W1',
    parent_run_id: 'FRS-coding-20260510-C1-W0',
    recursion_depth: 1,
    issued_at: new Date().toISOString(),
    expiry_cycle: 'C1',
    posture: 'interactive',
    track: 'code' as const,
    ...(overrides.route_token || {}),
  };

  const hasMetadataOverride = Object.prototype.hasOwnProperty.call(overrides, 'metadata');
  let metadata = {
    lifecycle_contract: makeLifecycleContract(route_token.target_stream ?? 'core'),
  } as Packet['metadata'];

  if (hasMetadataOverride) {
    if (overrides.metadata === undefined) {
      metadata = undefined;
    } else {
      const lifecycleOverrides = overrides.metadata?.lifecycle_contract || {};
      metadata = {
        ...overrides.metadata,
        lifecycle_contract: makeLifecycleContract(route_token.target_stream ?? 'core', lifecycleOverrides),
      };
    }
  }

  return {
    route_token,
    payload: overrides.payload ?? 'packet payload',
    prompt: overrides.prompt ?? 'prompt payload',
    signature: overrides.signature,
    issued_at: overrides.issued_at ?? new Date().toISOString(),
    ttl_ms: overrides.ttl_ms ?? 300000,
    metadata,
  };
}
