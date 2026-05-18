/**
 * Packet Validator: Structural and semantic checks for route tokens and packets
 * Part of Check #1: Packet Validation in guardrail enforcement
 */
import { ExecutionReturn, Packet, ValidationResult } from './types';
export declare class PacketValidator {
    /**
     * Validate packet structure and route token fields
     * Performs Check #1: Packet Validation
     */
    static validatePacket(packet: Packet): ValidationResult;
    /**
     * Validate the lifecycle contract carried with the packet.
     */
    static validateLifecycleContract(packet: Packet): ValidationResult;
    /**
     * Validate that the normalized execution return satisfies the lifecycle contract.
     */
    static validateExecutionOutcome(packet: Packet, executionReturn: ExecutionReturn): ValidationResult;
    /**
     * Check if run_id is continuous with prior history
     * Placeholder for more sophisticated continuity validation
     */
    static checkRunIdContinuity(packet: Packet, _priorRunId?: string): ValidationResult;
    /**
     * Check target_stream matches expected agent identity
     */
    static checkTargetStreamMatch(packet: Packet, expectedStream: 'core' | 'subcore'): ValidationResult;
}
//# sourceMappingURL=packet-validator.d.ts.map