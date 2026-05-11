import { GuardrailChecker } from '../src/guardrails';
import { makePacket } from './fixtures';

describe('GuardrailChecker', () => {
  test('passes all checks for valid core packet', async () => {
    const packet = makePacket();
    const result = await GuardrailChecker.checkAll(packet, 'core');
    expect(result.passed).toBe(true);
    expect(result.failedChecks).toHaveLength(0);
  });

  test('fails recursion depth > 2', async () => {
    const packet = makePacket({
      route_token: { ...makePacket().route_token, recursion_depth: 3 }
    });
    const result = await GuardrailChecker.checkAll(packet, 'core');
    expect(result.passed).toBe(false);
    expect(result.failedChecks).toContain('check2_RecursionDepth');
  });

  test('fails missing parent_run_id', async () => {
    const packet = makePacket({
      route_token: { ...makePacket().route_token, parent_run_id: '' }
    });
    const result = await GuardrailChecker.checkAll(packet, 'core');
    expect(result.passed).toBe(false);
    expect(result.failedChecks).toContain('check3_ParentRunId');
  });

  test('fails expired TTL', async () => {
    const packet = makePacket({
      issued_at: new Date(Date.now() - 600000).toISOString(),
      ttl_ms: 1000
    });
    const result = await GuardrailChecker.checkAll(packet, 'core');
    expect(result.passed).toBe(false);
    expect(result.failedChecks).toContain('check4_TTLValid');
  });

  test('fails target stream mismatch under packet validation', async () => {
    const packet = makePacket({
      route_token: { ...makePacket().route_token, target_stream: 'subcore' }
    });
    const result = await GuardrailChecker.checkAll(packet, 'core');
    expect(result.passed).toBe(false);
    expect(result.failedChecks).toContain('check1_PacketValid');
  });
});
