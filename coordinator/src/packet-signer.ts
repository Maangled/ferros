/**
 * Packet Signer: HMAC-SHA256 signing and verification for packet payloads
 * Ensures packet integrity and enables audit trail
 */

import * as crypto from 'crypto';

/**
 * HMAC signer for packet authentication
 * Signs outgoing packets; verifies incoming packets
 */
export class PacketSigner {
  private secretKey: string;
  private algorithm: string = 'sha256';

  constructor(secretKey?: string) {
    // In production, load from environment variable or secure vault
    this.secretKey = secretKey || process.env.FERROS_PACKET_SECRET || 'ferros-default-dev-key';
    if (this.secretKey === 'ferros-default-dev-key') {
      console.warn('[PacketSigner] Using default dev key; set FERROS_PACKET_SECRET for production');
    }
  }

  /**
   * Sign a packet payload and return base64-encoded signature
   */
  signPacket(payload: string): string {
    const hmac = crypto
      .createHmac(this.algorithm, this.secretKey)
      .update(payload, 'utf8')
      .digest('base64');
    return hmac;
  }

  /**
   * Verify a packet signature
   * Returns true if signature is valid, false otherwise
   */
  verifySignature(payload: string, signature: string): boolean {
    const expectedSignature = this.signPacket(payload);
    // Use constant-time comparison to prevent timing attacks
    return crypto.timingSafeEqual(Buffer.from(signature), Buffer.from(expectedSignature));
  }

  /**
   * Inject signature into prompt metadata
   * Returns prompt with signature appended
   */
  injectSignature(prompt: string, signature: string): string {
    const metadata = `[PACKET_SIGNATURE: ${signature}]`;
    // Append metadata at the very end so it doesn't affect prompt parsing
    return `${prompt}\n\n${metadata}`;
  }

  /**
   * Extract signature from prompt metadata
   * Returns { prompt (without metadata), signature } or { prompt, signature: null }
   */
  extractSignature(prompt: string): { prompt: string; signature: string | null } {
    const signatureMatch = prompt.match(/\[PACKET_SIGNATURE: ([^\]]+)\]$/m);
    if (signatureMatch) {
      const signature = signatureMatch[1];
      const cleanPrompt = prompt.replace(/\n*\[PACKET_SIGNATURE: [^\]]+\]$/m, '');
      return { prompt: cleanPrompt, signature };
    }
    return { prompt, signature: null };
  }

  /**
   * Rotate secret key (for key renewal without downtime)
   * Allows verification with both old and new keys for a grace period
   */
  rotateKey(newSecretKey: string): { old: string; new: string } {
    const oldKey = this.secretKey;
    this.secretKey = newSecretKey;
    return { old: oldKey, new: newSecretKey };
  }
}

/**
 * Singleton instance
 */
let signerInstance: PacketSigner | null = null;

export function getPacketSigner(secretKey?: string): PacketSigner {
  if (!signerInstance) {
    signerInstance = new PacketSigner(secretKey);
  }
  return signerInstance;
}
