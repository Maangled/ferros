/**
 * FERROS Orchestrator Coordinator: Main entry point
 * Orchestrates inter-agent handoff with guardrail enforcement via Copilot SDK
 *
 * Public API:
 * - handoffToAgent(packet, targetAgent) -> ExecutionReturn | CoordinatorError
 * - handoffToBoth(corePacket, subcorePacket) -> DualHandoffResult | CoordinatorError
 */
import { Packet, ExecutionReturn, CoordinatorError, CoordinatorOptions, DualHandoffResult } from './types';
/**
 * Main Coordinator class
 */
export declare class OrchestrationCoordinator {
    private guardrailChecker;
    private packetValidator;
    private packetSigner;
    private sessionManager;
    private eventTracer;
    private options;
    constructor(options?: CoordinatorOptions);
    /**
     * Main handoff entry point: Single agent (Core or SubCore)
     *
     * Flow:
     * 1. Validate all 5 guardrails
     * 2. Sign packet payload
     * 3. Create SDK session
     * 4. Send prompt
     * 5. Capture response and normalize
     * 6. Cleanup session
     */
    handoffToAgent(packet: Packet, targetAgent: 'core' | 'subcore', options?: CoordinatorOptions): Promise<ExecutionReturn | CoordinatorError>;
    /**
     * Dual-execution handoff: Simultaneous Core + SubCore
     *
     * Validates both packets independently, creates both sessions in parallel,
     * sends both prompts in parallel, captures both responses
     */
    handoffToBoth(corePacket: Packet, subcorePacket: Packet, options?: CoordinatorOptions): Promise<DualHandoffResult | CoordinatorError>;
    /**
     * Timeout wrapper for async operations
     */
    private withTimeout;
    /**
     * Logging helper
     */
    private log;
}
export declare function getCoordinator(options?: CoordinatorOptions): OrchestrationCoordinator;
/**
 * Public API convenience functions
 */
export declare function handoffToAgent(packet: Packet, targetAgent: 'core' | 'subcore', options?: CoordinatorOptions): Promise<ExecutionReturn | CoordinatorError>;
export declare function handoffToBoth(corePacket: Packet, subcorePacket: Packet, options?: CoordinatorOptions): Promise<DualHandoffResult | CoordinatorError>;
//# sourceMappingURL=coordinator.d.ts.map