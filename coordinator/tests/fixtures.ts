import { Packet } from '../src/types';

export function makePacket(overrides: Partial<Packet> = {}): Packet {
  return {
    route_token: {
      token_version: 'v2',
      issued_by: 'FERROS Prompt Architect Agent',
      target_stream: 'core',
      target_family: null,
      run_id: 'FRS-core-20260510-C1-W1',
      parent_run_id: 'FRS-coding-20260510-C1-W0',
      recursion_depth: 1,
      issued_at: new Date().toISOString(),
      expiry_cycle: 'C1',
      posture: 'interactive',
      track: 'code'
    },
    payload: 'packet payload',
    prompt: 'prompt payload',
    issued_at: new Date().toISOString(),
    ttl_ms: 300000,
    ...overrides
  };
}
