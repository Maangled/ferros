/**
 * Packet Signer: HMAC-SHA256 signing and verification for packet payloads
 * Ensures packet integrity and enables audit trail
 */
/**
 * HMAC signer for packet authentication
 * Signs outgoing packets; verifies incoming packets
 */
export declare class PacketSigner {
    private secretKey;
    private algorithm;
    constructor(secretKey?: string);
    /**
     * Sign a packet payload and return base64-encoded signature
     */
    signPacket(payload: string): string;
    /**
     * Verify a packet signature
     * Returns true if signature is valid, false otherwise
     */
    verifySignature(payload: string, signature: string): boolean;
    /**
     * Inject signature into prompt metadata
     * Returns prompt with signature appended
     */
    injectSignature(prompt: string, signature: string): string;
    /**
     * Extract signature from prompt metadata
     * Returns { prompt (without metadata), signature } or { prompt, signature: null }
     */
    extractSignature(prompt: string): {
        prompt: string;
        signature: string | null;
    };
    /**
     * Rotate secret key (for key renewal without downtime)
     * Allows verification with both old and new keys for a grace period
     */
    rotateKey(newSecretKey: string): {
        old: string;
        new: string;
    };
}
export declare function getPacketSigner(secretKey?: string): PacketSigner;
//# sourceMappingURL=packet-signer.d.ts.map