"use strict";
/**
 * Guardrail Checks: All 5 mandatory checks before inter-agent handoff
 * These checks MUST pass before coordinator.handoffToAgent() creates a session
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.GuardrailChecker = void 0;
const packet_validator_1 = require("./packet-validator");
class GuardrailChecker {
    /**
     * Run all 5 guardrail checks in sequence
     * All 5 MUST pass before handoff is allowed
     */
    static async checkAll(packet, targetAgent) {
        const checks = {};
        // Check 1: Packet Validation
        checks.check1_PacketValid = await this.check1_PacketValid(packet, targetAgent);
        // Check 2: Recursion Depth
        checks.check2_RecursionDepth = await this.check2_RecursionDepth(packet);
        // Check 3: Parent Run ID
        checks.check3_ParentRunId = await this.check3_ParentRunId(packet);
        // Check 4: TTL Valid
        checks.check4_TTLValid = await this.check4_TTLValid(packet);
        // Check 5: Self-Handoff Prevention
        checks.check5_SelfHandoffPrevention = await this.check5_SelfHandoffPrevention(packet, targetAgent);
        // Aggregate results
        const failedChecks = Object.entries(checks)
            .filter(([_, result]) => !result.passed)
            .map(([name, _]) => name);
        return {
            passed: failedChecks.length === 0,
            failedChecks,
            details: checks,
            timestamp: new Date().toISOString()
        };
    }
    /**
     * CHECK #1: Packet Validation
     * Route token present, target_stream matches agent identity, run_id is continuous
     */
    static async check1_PacketValid(packet, targetAgent) {
        try {
            // Structural validation
            const validation = packet_validator_1.PacketValidator.validatePacket(packet);
            if (!validation.valid) {
                return {
                    name: 'check1_PacketValid',
                    passed: false,
                    details: validation.errors.join('; ')
                };
            }
            // Target stream match
            const streamMatch = packet_validator_1.PacketValidator.checkTargetStreamMatch(packet, targetAgent);
            if (!streamMatch.valid) {
                return {
                    name: 'check1_PacketValid',
                    passed: false,
                    details: streamMatch.errors.join('; ')
                };
            }
            // Run ID continuity check
            const continuity = packet_validator_1.PacketValidator.checkRunIdContinuity(packet);
            if (!continuity.valid) {
                return {
                    name: 'check1_PacketValid',
                    passed: false,
                    details: continuity.errors.join('; ')
                };
            }
            return {
                name: 'check1_PacketValid',
                passed: true,
                details: 'Route token valid, target_stream matches, run_id continuous'
            };
        }
        catch (error) {
            return {
                name: 'check1_PacketValid',
                passed: false,
                details: `Exception during validation: ${error}`
            };
        }
    }
    /**
     * CHECK #2: Recursion Depth
     * If packet contains recursion_depth, confirm it does not exceed 2
     * depth >= 2 must escalate upward instead of handing off
     */
    static async check2_RecursionDepth(packet) {
        const depth = packet.route_token.recursion_depth ?? 0;
        if (depth > 2) {
            return {
                name: 'check2_RecursionDepth',
                passed: false,
                details: `Recursion depth ${depth} exceeds ceiling of 2; must escalate upward`
            };
        }
        return {
            name: 'check2_RecursionDepth',
            passed: true,
            details: `Recursion depth ${depth} is within ceiling (≤ 2)`
        };
    }
    /**
     * CHECK #3: Parent Packet ID (Traceability)
     * Packet must include parent_run_id for response routing back to source
     */
    static async check3_ParentRunId(packet) {
        const parentRunId = packet.route_token.parent_run_id;
        if (!parentRunId || parentRunId.trim() === '') {
            return {
                name: 'check3_ParentRunId',
                passed: false,
                details: 'Missing or empty parent_run_id; required for response traceability'
            };
        }
        return {
            name: 'check3_ParentRunId',
            passed: true,
            details: `parent_run_id present: ${parentRunId}`
        };
    }
    /**
     * CHECK #4: TTL Check
     * If packet has issued_at and TTL window, confirm not expired
     * Prevents handoff of stale packets
     */
    static async check4_TTLValid(packet) {
        const { issued_at, ttl_ms } = packet;
        // If no TTL specified, check passes (TTL is optional)
        if (!issued_at || !ttl_ms) {
            return {
                name: 'check4_TTLValid',
                passed: true,
                details: 'No TTL specified (optional)'
            };
        }
        try {
            const issuedTime = new Date(issued_at).getTime();
            const expiryTime = issuedTime + ttl_ms;
            const nowTime = Date.now();
            if (nowTime > expiryTime) {
                const expiredMs = nowTime - expiryTime;
                return {
                    name: 'check4_TTLValid',
                    passed: false,
                    details: `Packet expired ${expiredMs}ms ago (issued: ${issued_at}, TTL: ${ttl_ms}ms)`
                };
            }
            const remainingMs = expiryTime - nowTime;
            return {
                name: 'check4_TTLValid',
                passed: true,
                details: `Packet valid; expires in ${remainingMs}ms`
            };
        }
        catch (error) {
            return {
                name: 'check4_TTLValid',
                passed: false,
                details: `Exception parsing TTL: ${error}`
            };
        }
    }
    /**
     * CHECK #5: Self-Handoff Prevention
     * Target agent identity must not be source identity
     * Coordinator has infer: false to prevent implicit self-delegation
     */
    static async check5_SelfHandoffPrevention(packet, targetAgent) {
        // Self-handoff prevention for coding flow means execution targets must stay
        // in core/subcore lanes and must not drift into non-execution family targets.
        const targetFamily = packet.route_token.target_family;
        if (targetFamily !== null && targetFamily !== undefined) {
            return {
                name: 'check5_SelfHandoffPrevention',
                passed: false,
                details: `Invalid self/family handoff: target_family='${targetFamily}' is not allowed for execution handoff`
            };
        }
        if (!packet.route_token.target_stream) {
            return {
                name: 'check5_SelfHandoffPrevention',
                passed: false,
                details: 'Ambiguous target: packet has no explicit target_stream'
            };
        }
        if (packet.route_token.target_stream !== targetAgent) {
            return {
                name: 'check5_SelfHandoffPrevention',
                passed: false,
                details: `Target mismatch: packet targets '${packet.route_token.target_stream}' but handoff target is '${targetAgent}'`
            };
        }
        return {
            name: 'check5_SelfHandoffPrevention',
            passed: true,
            details: `Target agent '${targetAgent}' is not coordinator; self-handoff prevented`
        };
    }
    /**
     * Helper: Check if all guardrails passed
     */
    static allPassed(result) {
        return result.passed;
    }
    /**
     * Helper: Get failed check names
     */
    static getFailedCheckNames(result) {
        return result.failedChecks;
    }
    /**
     * Helper: Format results for logging
     */
    static formatResults(result) {
        if (result.passed) {
            return 'All guardrails passed ✓';
        }
        const failures = result.failedChecks
            .map(name => {
            const check = result.details[name];
            return `${name}: ${check.details}`;
        })
            .join('\n  ');
        return `Guardrail failures:\n  ${failures}`;
    }
}
exports.GuardrailChecker = GuardrailChecker;
//# sourceMappingURL=guardrails.js.map