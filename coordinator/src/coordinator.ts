/**
 * FERROS Orchestrator Coordinator: Main entry point
 * Orchestrates inter-agent handoff with guardrail enforcement via Copilot SDK
 * 
 * Public API:
 * - handoffToAgent(packet, targetAgent) -> ExecutionReturn | CoordinatorError
 * - handoffToBoth(corePacket, subcorePacket) -> DualHandoffResult | CoordinatorError
 */

import {
  Packet,
  ExecutionReturn,
  CoordinatorError,
  CoordinatorOptions,
  DualHandoffResult
} from './types';
import { PacketValidator } from './packet-validator';
import { GuardrailChecker } from './guardrails';
import { getPacketSigner } from './packet-signer';
import { SessionManager, getSessionManager } from './session-manager';
import { EventTracer, getEventTracer } from './event-tracer';

/**
 * Main Coordinator class
 */
export class OrchestrationCoordinator {
  private guardrailChecker = GuardrailChecker;
  private packetValidator = PacketValidator;
  private packetSigner = getPacketSigner();
  private sessionManager: SessionManager;
  private eventTracer: EventTracer;
  private options: CoordinatorOptions;

  constructor(options?: CoordinatorOptions) {
    this.options = {
      timeout_ms: options?.timeout_ms || 30000,
      retry_on_failure: options?.retry_on_failure ?? false,
      log_level: options?.log_level || 'info',
      capture_events: options?.capture_events ?? true,
      ...options
    };

    this.sessionManager = getSessionManager({ sdk_client: this.options.sdk_client });
    this.eventTracer = getEventTracer();

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
  async handoffToAgent(
    packet: Packet,
    targetAgent: 'core' | 'subcore',
    options?: CoordinatorOptions
  ): Promise<ExecutionReturn | CoordinatorError> {
    const startTime = Date.now();
    const handoffId = `${packet.route_token.parent_run_id}-${targetAgent}`;
    let sessionId: string | undefined;

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
        } as CoordinatorError;
      }

      this.log('info', `[${handoffId}] All guardrails passed ✓`);

      // Step 2: Sign packet payload
      const signature = this.packetSigner.signPacket(packet.payload);
      const signedPrompt = this.packetSigner.injectSignature(packet.prompt, signature);

      this.log('info', `[${handoffId}] Packet signed with HMAC-SHA256`);

      // Step 3: Create SDK session
      this.log('info', `[${handoffId}] Creating session...`);
      const session = await this.sessionManager.createSession(targetAgent);
      sessionId = session.id;

      // Setup event listeners
      let eventCtx: { getToolCallId: () => string | undefined; getEvents: () => any[] } | undefined;
      if (this.options.capture_events) {
        eventCtx = this.eventTracer.setupEventListeners(session, packet);
      }

      // Step 4: Send prompt (with timeout)
      this.log('info', `[${handoffId}] Sending prompt...`);
      const responsePromise = this.sessionManager.sendPrompt(session, signedPrompt);
      const response = await this.withTimeout(
        responsePromise,
        this.options.timeout_ms || 30000,
        `Handoff to ${targetAgent} agent`
      );

      // Step 5: Normalize response
      const toolCallId = eventCtx?.getToolCallId();
      const executionReturn = this.eventTracer.normalizeResponse(response, packet, toolCallId);

      const lifecycleValidation = this.packetValidator.validateExecutionOutcome(packet, executionReturn);
      if (!lifecycleValidation.valid) {
        this.log(
          'error',
          `[${handoffId}] Lifecycle contract failure: ${lifecycleValidation.errors.join('; ')}`
        );

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
        } as CoordinatorError;
      }

      this.log('info', `[${handoffId}] Handoff completed successfully (${Date.now() - startTime}ms)`);

      return executionReturn;
    } catch (error) {
      this.log('error', `[${handoffId}] Handoff failed: ${error}`);

      return {
        error: `Handoff to ${targetAgent} agent failed: ${error}`,
        details: { error },
        escalate: true,
        timestamp: new Date().toISOString()
      } as CoordinatorError;
    } finally {
      if (sessionId) {
        await this.sessionManager.cleanupSession(sessionId);
      }
    }
  }

  /**
   * Dual-execution handoff: Simultaneous Core + SubCore
   *
   * Validates both packets independently, creates both sessions in parallel,
   * sends both prompts in parallel, captures both responses
   */
  async handoffToBoth(
    corePacket: Packet,
    subcorePacket: Packet,
    options?: CoordinatorOptions
  ): Promise<DualHandoffResult | CoordinatorError> {
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
        } as CoordinatorError;
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
            } as CoordinatorError),
        subcoreFailures.length === 0
          ? this.handoffToAgent(subcorePacket, 'subcore', options)
          : Promise.resolve({
              error: `SubCore packet guardrail failures: ${subcoreFailures.join(', ')}`,
              failedChecks: subcoreFailures,
              escalate: true,
              timestamp: new Date().toISOString()
            } as CoordinatorError)
      ]);

      this.log('info', `[${handoffId}] Dual handoff completed`);

      return {
        core: coreResult,
        subcore: subcoreResult,
        timestamp: new Date().toISOString()
      };
    } catch (error) {
      this.log('error', `[${handoffId}] Dual handoff failed: ${error}`);

      return {
        error: `Dual handoff failed: ${error}`,
        details: { error },
        escalate: true,
        timestamp: new Date().toISOString()
      } as CoordinatorError;
    }
  }

  /**
   * Timeout wrapper for async operations
   */
  private withTimeout<T>(promise: Promise<T>, timeoutMs: number, operation: string): Promise<T> {
    return Promise.race([
      promise,
      new Promise<T>((_, reject) =>
        setTimeout(() => reject(new Error(`Operation '${operation}' timed out after ${timeoutMs}ms`)), timeoutMs)
      )
    ]);
  }

  /**
   * Logging helper
   */
  private log(level: 'info' | 'error' | 'warn' | 'debug', message: string): void {
    if (
      (level === 'debug' && this.options.log_level === 'debug') ||
      (level === 'info' && ['debug', 'info'].includes(this.options.log_level || 'info')) ||
      (level === 'warn' && ['debug', 'info', 'warn'].includes(this.options.log_level || 'info')) ||
      level === 'error'
    ) {
      const timestamp = new Date().toISOString();
      console.log(`[${timestamp}] ${level.toUpperCase()} ${message}`);
    }
  }
}

/**
 * Singleton instance
 */
let coordinatorInstance: OrchestrationCoordinator | null = null;

export function getCoordinator(options?: CoordinatorOptions): OrchestrationCoordinator {
  if (!coordinatorInstance) {
    coordinatorInstance = new OrchestrationCoordinator(options);
  }
  return coordinatorInstance;
}

/**
 * Public API convenience functions
 */
export async function handoffToAgent(
  packet: Packet,
  targetAgent: 'core' | 'subcore',
  options?: CoordinatorOptions
): Promise<ExecutionReturn | CoordinatorError> {
  return getCoordinator(options).handoffToAgent(packet, targetAgent, options);
}

export async function handoffToBoth(
  corePacket: Packet,
  subcorePacket: Packet,
  options?: CoordinatorOptions
): Promise<DualHandoffResult | CoordinatorError> {
  return getCoordinator(options).handoffToBoth(corePacket, subcorePacket, options);
}
