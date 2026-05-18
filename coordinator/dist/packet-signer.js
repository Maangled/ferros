"use strict";
/**
 * Packet Signer: HMAC-SHA256 signing and verification for packet payloads
 * Ensures packet integrity and enables audit trail
 */
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
exports.PacketSigner = void 0;
exports.getPacketSigner = getPacketSigner;
const crypto = __importStar(require("crypto"));
/**
 * HMAC signer for packet authentication
 * Signs outgoing packets; verifies incoming packets
 */
class PacketSigner {
    constructor(secretKey) {
        this.algorithm = 'sha256';
        // In production, load from environment variable or secure vault
        this.secretKey = secretKey || process.env.FERROS_PACKET_SECRET || 'ferros-default-dev-key';
        if (this.secretKey === 'ferros-default-dev-key') {
            console.warn('[PacketSigner] Using default dev key; set FERROS_PACKET_SECRET for production');
        }
    }
    /**
     * Sign a packet payload and return base64-encoded signature
     */
    signPacket(payload) {
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
    verifySignature(payload, signature) {
        const expectedSignature = this.signPacket(payload);
        // Use constant-time comparison to prevent timing attacks
        return crypto.timingSafeEqual(Buffer.from(signature), Buffer.from(expectedSignature));
    }
    /**
     * Inject signature into prompt metadata
     * Returns prompt with signature appended
     */
    injectSignature(prompt, signature) {
        const metadata = `[PACKET_SIGNATURE: ${signature}]`;
        // Append metadata at the very end so it doesn't affect prompt parsing
        return `${prompt}\n\n${metadata}`;
    }
    /**
     * Extract signature from prompt metadata
     * Returns { prompt (without metadata), signature } or { prompt, signature: null }
     */
    extractSignature(prompt) {
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
    rotateKey(newSecretKey) {
        const oldKey = this.secretKey;
        this.secretKey = newSecretKey;
        return { old: oldKey, new: newSecretKey };
    }
}
exports.PacketSigner = PacketSigner;
/**
 * Singleton instance
 */
let signerInstance = null;
function getPacketSigner(secretKey) {
    if (!signerInstance) {
        signerInstance = new PacketSigner(secretKey);
    }
    return signerInstance;
}
//# sourceMappingURL=packet-signer.js.map