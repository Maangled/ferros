import { PacketValidator } from '../src/packet-validator';
import { makePacket } from './fixtures';

describe('PacketValidator', () => {
  test('accepts valid packet', () => {
    const packet = makePacket();
    const result = PacketValidator.validatePacket(packet);
    expect(result.valid).toBe(true);
    expect(result.errors).toHaveLength(0);
  });

  test('rejects missing route_token', () => {
    const packet = { ...makePacket(), route_token: undefined as any };
    const result = PacketValidator.validatePacket(packet as any);
    expect(result.valid).toBe(false);
    expect(result.errors.join(' ')).toContain('Missing route_token');
  });

  test('rejects invalid target_stream', () => {
    const packet = makePacket({
      route_token: {
        ...makePacket().route_token,
        target_stream: 'invalid' as any
      }
    });
    const result = PacketValidator.validatePacket(packet);
    expect(result.valid).toBe(false);
    expect(result.errors.join(' ')).toContain('Invalid target_stream');
  });

  test('rejects when both target_stream and target_family are set', () => {
    const packet = makePacket({
      route_token: {
        ...makePacket().route_token,
        target_stream: 'core',
        target_family: 'coding'
      }
    });
    const result = PacketValidator.validatePacket(packet);
    expect(result.valid).toBe(false);
    expect(result.errors.join(' ')).toContain('mutually exclusive');
  });

  test('checks target stream match', () => {
    const packet = makePacket({
      route_token: { ...makePacket().route_token, target_stream: 'subcore' }
    });
    const result = PacketValidator.checkTargetStreamMatch(packet, 'core');
    expect(result.valid).toBe(false);
  });
});
