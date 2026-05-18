/**
 * Guardrail Checks: All 5 mandatory checks before inter-agent handoff
 * These checks MUST pass before coordinator.handoffToAgent() creates a session
 */
import { Packet, GuardrailCheckResult, GuardrailResult } from './types';
export declare class GuardrailChecker {
    /**
     * Run all 5 guardrail checks in sequence
     * All 5 MUST pass before handoff is allowed
     */
    static checkAll(packet: Packet, targetAgent: 'core' | 'subcore'): Promise<GuardrailResult>;
    /**
     * CHECK #1: Packet Validation
     * Route token present, target_stream matches agent identity, run_id is continuous
     */
    static check1_PacketValid(packet: Packet, targetAgent: 'core' | 'subcore'): Promise<GuardrailCheckResult>;
    /**
     * CHECK #2: Recursion Depth
     * If packet contains recursion_depth, confirm it does not exceed 2
     * depth >= 2 must escalate upward instead of handing off
     */
    static check2_RecursionDepth(packet: Packet): Promise<GuardrailCheckResult>;
    /**
     * CHECK #3: Parent Packet ID (Traceability)
     * Packet must include parent_run_id for response routing back to source
     */
    static check3_ParentRunId(packet: Packet): Promise<GuardrailCheckResult>;
    /**
     * CHECK #4: TTL Check
     * If packet has issued_at and TTL window, confirm not expired
     * Prevents handoff of stale packets
     */
    static check4_TTLValid(packet: Packet): Promise<GuardrailCheckResult>;
    /**
     * CHECK #5: Self-Handoff Prevention
     * Target agent identity must not be source identity
     * Coordinator has infer: false to prevent implicit self-delegation
     */
    static check5_SelfHandoffPrevention(packet: Packet, targetAgent: 'core' | 'subcore'): Promise<GuardrailCheckResult>;
    /**
     * Helper: Check if all guardrails passed
     */
    static allPassed(result: GuardrailResult): boolean;
    /**
     * Helper: Get failed check names
     */
    static getFailedCheckNames(result: GuardrailResult): string[];
    /**
     * Helper: Format results for logging
     */
    static formatResults(result: GuardrailResult): string;
}
//# sourceMappingURL=guardrails.d.ts.map