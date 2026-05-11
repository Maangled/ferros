import { PacketSigner } from '../src/packet-signer';

describe('PacketSigner', () => {
  const signer = new PacketSigner('test-secret-key');

  test('signs and verifies payload', () => {
    const payload = 'ferros-packet';
    const signature = signer.signPacket(payload);
    expect(signature).toBeTruthy();
    expect(signer.verifySignature(payload, signature)).toBe(true);
  });

  test('fails verification for modified payload', () => {
    const payload = 'ferros-packet';
    const signature = signer.signPacket(payload);
    expect(signer.verifySignature('ferros-packet-modified', signature)).toBe(false);
  });

  test('injects and extracts signature metadata', () => {
    const prompt = 'Prompt block';
    const signature = signer.signPacket('payload');
    const withSig = signer.injectSignature(prompt, signature);
    const extracted = signer.extractSignature(withSig);

    expect(extracted.signature).toBe(signature);
    expect(extracted.prompt).toContain('Prompt block');
    expect(extracted.prompt).not.toContain('PACKET_SIGNATURE');
  });
});
