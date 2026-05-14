/**
 * Packet Validator: Structural and semantic checks for route tokens and packets
 * Part of Check #1: Packet Validation in guardrail enforcement
 */

import {
  ExecutionReturn,
  LifecycleOutcomeKind,
  Packet,
  PacketLifecycleContract,
  ValidationResult,
} from './types';

const VALID_TERMINAL_STATES: LifecycleOutcomeKind[] = [
  'report',
  'work_order',
  'escalation',
  'denied',
  'archived',
  'stopped',
];

export class PacketValidator {
  /**
   * Validate packet structure and route token fields
   * Performs Check #1: Packet Validation
   */
  static validatePacket(packet: Packet): ValidationResult {
    const errors: string[] = [];
    const warnings: string[] = [];

    // Check packet exists
    if (!packet) {
      errors.push('Packet is null or undefined');
      return { valid: false, errors };
    }

    // Check route_token exists
    if (!packet.route_token) {
      errors.push('Missing route_token');
      return { valid: false, errors, warnings };
    }

    const { route_token } = packet;

    // Check token version
    if (route_token.token_version !== 'v2') {
      errors.push(`Invalid token_version: expected 'v2', got '${route_token.token_version}'`);
    }

    // Check run_id format
    if (!route_token.run_id) {
      errors.push('Missing run_id');
    } else if (!route_token.run_id.match(/^FRS-\w+-\d{8}-C\d+-W\d+$/)) {
      warnings.push(`run_id format unusual: ${route_token.run_id}`);
    }

    // Check target_stream XOR target_family (must have one, not both)
    const hasStream = route_token.target_stream !== null && route_token.target_stream !== undefined;
    const hasFamily = route_token.target_family !== null && route_token.target_family !== undefined;

    if (hasStream && hasFamily) {
      errors.push('Both target_stream and target_family set; should be mutually exclusive (XOR)');
    }
    if (!hasStream && !hasFamily) {
      errors.push('Neither target_stream nor target_family set; exactly one is required');
    }

    // Validate target_stream if present
    if (hasStream) {
      const validStreams = ['core', 'subcore'];
      if (!validStreams.includes(route_token.target_stream!)) {
        errors.push(`Invalid target_stream: '${route_token.target_stream}'; must be 'core' or 'subcore'`);
      }
    }

    // Validate target_family if present
    if (hasFamily) {
      const validFamilies = ['coding', 'business', 'architect'];
      if (!validFamilies.includes(route_token.target_family!)) {
        errors.push(`Invalid target_family: '${route_token.target_family}'; must be one of ${validFamilies.join(', ')}`);
      }
    }

    // Check issued_at format (ISO date string)
    if (!route_token.issued_at) {
      errors.push('Missing issued_at');
    } else if (!route_token.issued_at.match(/^\d{4}-\d{2}-\d{2}T?\d{2}:\d{2}:\d{2}/)) {
      errors.push(`Invalid issued_at format: '${route_token.issued_at}'; expected ISO 8601 date`);
    }

    // Check issued_by
    if (!route_token.issued_by) {
      errors.push('Missing issued_by');
    } else if (!['FERROS Prompt Architect Agent', 'FERROS Coding Agent'].includes(route_token.issued_by)) {
      errors.push(`Invalid issued_by: '${route_token.issued_by}'`);
    }

    // Check packet payload
    if (!packet.prompt) {
      errors.push('Missing prompt payload');
    }

    const lifecycleValidation = this.validateLifecycleContract(packet);
    if (!lifecycleValidation.valid) {
      errors.push(...lifecycleValidation.errors);
    }
    if (lifecycleValidation.warnings?.length) {
      warnings.push(...lifecycleValidation.warnings);
    }

    return {
      valid: errors.length === 0,
      errors,
      warnings: warnings.length > 0 ? warnings : undefined
    };
  }

  /**
   * Validate the lifecycle contract carried with the packet.
   */
  static validateLifecycleContract(packet: Packet): ValidationResult {
    const errors: string[] = [];
    const contract = packet.metadata?.lifecycle_contract;

    if (!contract) {
      return {
        valid: false,
        errors: ['Missing metadata.lifecycle_contract'],
      };
    }

    const requiredStringFields: Array<[keyof PacketLifecycleContract, string]> = [
      ['cycle_id', 'cycle_id'],
      ['work_order_id', 'work_order_id'],
      ['source_agent_id', 'source_agent_id'],
      ['target_agent_id', 'target_agent_id'],
      ['owner_agent_id', 'owner_agent_id'],
    ];

    for (const [field, label] of requiredStringFields) {
      const value = contract[field];
      if (typeof value !== 'string' || value.trim() === '') {
        errors.push(`Missing lifecycle_contract.${label}`);
      }
    }

    if (!contract.stop) {
      errors.push('Missing lifecycle_contract.stop');
      return { valid: false, errors };
    }

    if (!Array.isArray(contract.stop.allowed_terminal_states) || contract.stop.allowed_terminal_states.length === 0) {
      errors.push('Missing lifecycle_contract.stop.allowed_terminal_states');
      return { valid: false, errors };
    }

    const invalidStates = contract.stop.allowed_terminal_states.filter(
      (state) => !VALID_TERMINAL_STATES.includes(state)
    );
    if (invalidStates.length > 0) {
      errors.push(
        `Invalid lifecycle_contract.stop.allowed_terminal_states: ${invalidStates.join(', ')}`
      );
    }

    if (
      contract.stop.allowed_terminal_states.includes('stopped') &&
      contract.stop.stopped_reason_required !== true
    ) {
      errors.push(
        "lifecycle_contract.stop.stopped_reason_required must be true when 'stopped' is allowed"
      );
    }

    if (contract.stop.allowed_terminal_states.includes('escalation')) {
      if (!contract.escalation_id || contract.escalation_id.trim() === '') {
        errors.push('Missing lifecycle_contract.escalation_id');
      }
      if (!contract.escalation_target_agent_id || contract.escalation_target_agent_id.trim() === '') {
        errors.push('Missing lifecycle_contract.escalation_target_agent_id');
      }
    }

    if (
      packet.route_token.target_stream &&
      contract.target_agent_id &&
      packet.route_token.target_stream !== contract.target_agent_id
    ) {
      errors.push(
        `lifecycle_contract.target_agent_id '${contract.target_agent_id}' does not match route_token.target_stream '${packet.route_token.target_stream}'`
      );
    }

    return { valid: errors.length === 0, errors };
  }

  /**
   * Validate that the normalized execution return satisfies the lifecycle contract.
   */
  static validateExecutionOutcome(packet: Packet, executionReturn: ExecutionReturn): ValidationResult {
    const errors: string[] = [];
    const lifecycleValidation = this.validateLifecycleContract(packet);
    if (!lifecycleValidation.valid) {
      errors.push(...lifecycleValidation.errors);
    }

    const contract = packet.metadata?.lifecycle_contract;
    if (executionReturn.lifecycle_errors?.length) {
      errors.push(...executionReturn.lifecycle_errors);
    }

    if (!executionReturn.lifecycle_outcome) {
      errors.push('Execution return missing lifecycle outcome');
      return { valid: false, errors };
    }

    const outcome = executionReturn.lifecycle_outcome;
    if (!outcome.summary || outcome.summary.trim() === '') {
      errors.push(`Execution return lifecycle outcome '${outcome.kind}' is missing a summary`);
    }

    if (
      contract &&
      !contract.stop.allowed_terminal_states.includes(outcome.kind)
    ) {
      errors.push(
        `Lifecycle outcome '${outcome.kind}' is not allowed by the packet stop contract`
      );
    }

    if (
      outcome.kind === 'work_order' &&
      contract?.work_order_id &&
      outcome.work_order_id &&
      outcome.work_order_id !== contract.work_order_id
    ) {
      errors.push(
        `Lifecycle work_order_id '${outcome.work_order_id}' does not match contract '${contract.work_order_id}'`
      );
    }

    if (outcome.kind === 'escalation') {
      if (!contract?.escalation_id || !contract.escalation_target_agent_id) {
        errors.push('Execution return escalated without escalation fields in the packet lifecycle contract');
      }
      if (
        contract?.escalation_id &&
        outcome.escalation_id &&
        outcome.escalation_id !== contract.escalation_id
      ) {
        errors.push(
          `Lifecycle escalation_id '${outcome.escalation_id}' does not match contract '${contract.escalation_id}'`
        );
      }
    }

    if (
      outcome.kind === 'stopped' &&
      contract?.stop.stopped_reason_required &&
      (!outcome.stop_reason || outcome.stop_reason.trim() === '')
    ) {
      errors.push('Execution return stopped without a stop reason');
    }

    return { valid: errors.length === 0, errors };
  }

  /**
   * Check if run_id is continuous with prior history
   * Placeholder for more sophisticated continuity validation
   */
  static checkRunIdContinuity(packet: Packet, _priorRunId?: string): ValidationResult {
    const errors: string[] = [];

    if (!packet.route_token.run_id) {
      errors.push('Missing run_id for continuity check');
    }

    // TODO: Implement actual continuity check against wave history
    // For now, accept any valid run_id format
    if (!packet.route_token.run_id.match(/^FRS-/)) {
      errors.push(`run_id does not have FERROS prefix: '${packet.route_token.run_id}'`);
    }

    return { valid: errors.length === 0, errors };
  }

  /**
   * Check target_stream matches expected agent identity
   */
  static checkTargetStreamMatch(packet: Packet, expectedStream: 'core' | 'subcore'): ValidationResult {
    const errors: string[] = [];

    if (packet.route_token.target_stream !== expectedStream) {
      errors.push(
        `target_stream mismatch: packet targets '${packet.route_token.target_stream}' but handoff is to '${expectedStream}'`
      );
    }

    return { valid: errors.length === 0, errors };
  }
}
