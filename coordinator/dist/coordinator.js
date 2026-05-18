"use strict";
/**
 * FERROS Orchestrator Coordinator: Main entry point
 * Orchestrates inter-agent handoff with guardrail enforcement via Copilot SDK
 *
 * Public API:
 * - handoffToAgent(packet, targetAgent) -> ExecutionReturn | CoordinatorError
 * - handoffToBoth(corePacket, subcorePacket) -> DualHandoffResult | CoordinatorError
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.OrchestrationCoordinator = void 0;
exports.getCoordinator = getCoordinator;
exports.handoffToAgent = handoffToAgent;
exports.handoffToBoth = handoffToBoth;
const packet_validator_1 = require("./packet-validator");
const guardrails_1 = require("./guardrails");
const packet_signer_1 = require("./packet-signer");
const session_manager_1 = require("./session-manager");
const event_tracer_1 = require("./event-tracer");
/**
 * Main Coordinator class
 */
class OrchestrationCoordinator {
    constructor(options) {
        this.guardrailChecker = guardrails_1.GuardrailChecker;
        this.packetValidator = packet_validator_1.PacketValidator;
        this.packetSigner = (0, packet_signer_1.getPacketSigner)();
        this.options = {
            timeout_ms: options?.timeout_ms || 30000,
            retry_on_failure: options?.retry_on_failure ?? false,
            log_level: options?.log_level || 'info',
            capture_events: options?.capture_events ?? true,
            ...options
        };
        this.sessionManager = (0, session_manager_1.getSessionManager)({
            sdk_client: this.options.sdk_client,
            session_model: this.options.session_model,
            session_reasoning_effort: this.options.session_reasoning_effort,
        });
        this.eventTracer = (0, event_tracer_1.getEventTracer)();
        this.log('info', `[Coordinator] Initialized with timeout=${this.options.timeout_ms}ms`);
    }
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
    async handoffToAgent(packet, targetAgent, options) {
        const startTime = Date.now();
        const handoffId = `${packet.route_token.parent_run_id}-${targetAgent}`;
        let sessionId;
        try {
            this.log('info', `[${handoffId}] Starting handoff to ${targetAgent} agent...`);
            // Step 1: Validate all 5 guardrails
            this.log('info', `[${handoffId}] Checking guardrails...`);
            const guardrailResult = await this.guardrailChecker.checkAll(packet, targetAgent);
            if (!guardrailResult.passed) {
                const formattedResult = this.guardrailChecker.formatResults(guardrailResult);
                this.log('error', `[${handoffId}] Guardrail failure:\n${formattedResult}`);
                return {
                    error: 'Guardrail check failed',
                    failedChecks: guardrailResult.failedChecks,
                    details: guardrailResult.details,
                    escalate: true,
                    timestamp: guardrailResult.timestamp
                };
            }
            this.log('info', `[${handoffId}] All guardrails passed ✓`);
            // Step 2: Sign packet payload
            const signature = this.packetSigner.signPacket(packet.payload);
            const signedPrompt = this.packetSigner.injectSignature(packet.prompt, signature);
            this.log('info', `[${handoffId}] Packet signed with HMAC-SHA256`);
            // Step 3: Create SDK session
            this.log('info', `[${handoffId}] Creating session...`);
            const session = await this.sessionManager.createSession(targetAgent, packet);
            sessionId = session.id;
            // Setup event listeners
            let eventCtx;
            if (this.options.capture_events) {
                eventCtx = this.eventTracer.setupEventListeners(session, packet);
            }
            // Step 4: Send prompt (with timeout)
            this.log('info', `[${handoffId}] Sending prompt...`);
            const responsePromise = this.sessionManager.sendPrompt(session, signedPrompt);
            const response = await this.withTimeout(responsePromise, this.options.timeout_ms || 30000, `Handoff to ${targetAgent} agent`);
            // Step 5: Normalize response
            const toolCallId = eventCtx?.getToolCallId();
            const executionReturn = this.eventTracer.normalizeResponse(response, packet, toolCallId);
            const lifecycleValidation = this.packetValidator.validateExecutionOutcome(packet, executionReturn);
            if (!lifecycleValidation.valid) {
                this.log('error', `[${handoffId}] Lifecycle contract failure: ${lifecycleValidation.errors.join('; ')}`);
                return {
                    error: 'Lifecycle stop contract failed',
                    failedChecks: ['execution_lifecycle_contract'],
                    details: {
                        errors: lifecycleValidation.errors,
                        warnings: lifecycleValidation.warnings,
                        lifecycle_outcome: executionReturn.lifecycle_outcome,
                        lifecycle_errors: executionReturn.lifecycle_errors,
                    },
                    escalate: true,
                    timestamp: new Date().toISOString()
                };
            }
            this.log('info', `[${handoffId}] Handoff completed successfully (${Date.now() - startTime}ms)`);
            return executionReturn;
        }
        catch (error) {
            this.log('error', `[${handoffId}] Handoff failed: ${error}`);
            return {
                error: `Handoff to ${targetAgent} agent failed: ${error}`,
                details: { error },
                escalate: true,
                timestamp: new Date().toISOString()
            };
        }
        finally {
            if (sessionId) {
                await this.sessionManager.cleanupSession(sessionId);
            }
            await this.sessionManager.shutdownIfIdle();
        }
    }
    /**
     * Dual-execution handoff: Simultaneous Core + SubCore
     *
     * Validates both packets independently, creates both sessions in parallel,
     * sends both prompts in parallel, captures both responses
     */
    async handoffToBoth(corePacket, subcorePacket, options) {
        const handoffId = `${corePacket.route_token.parent_run_id}-dual`;
        try {
            this.log('info', `[${handoffId}] Starting dual handoff to Core + SubCore...`);
            // Validate both packets in parallel
            this.log('info', `[${handoffId}] Validating both packets...`);
            const [coreGuardrails, subcoreGuardrails] = await Promise.all([
                this.guardrailChecker.checkAll(corePacket, 'core'),
                this.guardrailChecker.checkAll(subcorePacket, 'subcore')
            ]);
            // Check for failures
            const coreFailures = coreGuardrails.failedChecks;
            const subcoreFailures = subcoreGuardrails.failedChecks;
            if (coreFailures.length > 0) {
                this.log('error', `[${handoffId}] Core packet guardrail failures: ${coreFailures.join(', ')}`);
            }
            if (subcoreFailures.length > 0) {
                this.log('error', `[${handoffId}] SubCore packet guardrail failures: ${subcoreFailures.join(', ')}`);
            }
            // If both failed, escalate
            if (coreFailures.length > 0 && subcoreFailures.length > 0) {
                return {
                    error: 'Both Core and SubCore packets failed guardrail checks',
                    details: { core: coreGuardrails, subcore: subcoreGuardrails },
                    escalate: true,
                    timestamp: new Date().toISOString()
                };
            }
            // Execute handoffs in parallel
            const [coreResult, subcoreResult] = await Promise.all([
                coreFailures.length === 0
                    ? this.handoffToAgent(corePacket, 'core', options)
                    : Promise.resolve({
                        error: `Core packet guardrail failures: ${coreFailures.join(', ')}`,
                        failedChecks: coreFailures,
                        escalate: true,
                        timestamp: new Date().toISOString()
                    }),
                subcoreFailures.length === 0
                    ? this.handoffToAgent(subcorePacket, 'subcore', options)
                    : Promise.resolve({
                        error: `SubCore packet guardrail failures: ${subcoreFailures.join(', ')}`,
                        failedChecks: subcoreFailures,
                        escalate: true,
                        timestamp: new Date().toISOString()
                    })
            ]);
            this.log('info', `[${handoffId}] Dual handoff completed`);
            return {
                core: coreResult,
                subcore: subcoreResult,
                timestamp: new Date().toISOString()
            };
        }
        catch (error) {
            this.log('error', `[${handoffId}] Dual handoff failed: ${error}`);
            return {
                error: `Dual handoff failed: ${error}`,
                details: { error },
                escalate: true,
                timestamp: new Date().toISOString()
            };
        }
    }
    /**
     * Timeout wrapper for async operations
     */
    withTimeout(promise, timeoutMs, operation) {
        return Promise.race([
            promise,
            new Promise((_, reject) => setTimeout(() => reject(new Error(`Operation '${operation}' timed out after ${timeoutMs}ms`)), timeoutMs))
        ]);
    }
    /**
     * Logging helper
     */
    log(level, message) {
        if ((level === 'debug' && this.options.log_level === 'debug') ||
            (level === 'info' && ['debug', 'info'].includes(this.options.log_level || 'info')) ||
            (level === 'warn' && ['debug', 'info', 'warn'].includes(this.options.log_level || 'info')) ||
            level === 'error') {
            const timestamp = new Date().toISOString();
            console.log(`[${timestamp}] ${level.toUpperCase()} ${message}`);
        }
    }
}
exports.OrchestrationCoordinator = OrchestrationCoordinator;
/**
 * Singleton instance
 */
let coordinatorInstance = null;
function getCoordinator(options) {
    if (!coordinatorInstance) {
        coordinatorInstance = new OrchestrationCoordinator(options);
    }
    return coordinatorInstance;
}
/**
 * Public API convenience functions
 */
async function handoffToAgent(packet, targetAgent, options) {
    return getCoordinator(options).handoffToAgent(packet, targetAgent, options);
}
async function handoffToBoth(corePacket, subcorePacket, options) {
    return getCoordinator(options).handoffToBoth(corePacket, subcorePacket, options);
}
//# sourceMappingURL=coordinator.js.map