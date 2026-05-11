/**
 * FERROS Orchestrator Coordinator: Public API exports
 */

// Types
export * from './types';

// Coordinator
export { OrchestrationCoordinator, getCoordinator, handoffToAgent, handoffToBoth } from './coordinator';

// Validation
export { PacketValidator } from './packet-validator';

// Signing
export { PacketSigner, getPacketSigner } from './packet-signer';

// Guardrails
export { GuardrailChecker } from './guardrails';

// Session Management
export { SessionManager, getSessionManager } from './session-manager';
export type { ClientSession, SessionInfo } from './session-manager';

// Event Tracing
export { EventTracer, getEventTracer } from './event-tracer';
export type { SDKEvent } from './event-tracer';
