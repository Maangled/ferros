/**
 * FERROS Orchestrator Coordinator: TypeScript interfaces and types
 * Defines packet structures, execution returns, guardrail results, and SDK wrappers
 */

/**
 * Route token v2 schema for inter-agent handoff
 */
export interface RouteToken {
  token_version: 'v2';
  issued_by: 'FERROS Prompt Architect Agent' | 'FERROS Coding Agent';
  
  // Execution target (mutually exclusive)
  target_stream?: 'core' | 'subcore' | null;
  target_family?: 'coding' | 'business' | 'architect' | null;
  
  // Packet identity and lineage
  run_id: string;
  parent_run_id: string;
  recursion_depth: number;
  
  // Work and lifecycle
  run_profile?: string;
  issued_at: string;
  expiry_cycle?: string;
  posture?: string;
  track?: 'code' | 'system' | 'hardware';
}

export type LifecycleOutcomeKind =
  | 'report'
  | 'work_order'
  | 'escalation'
  | 'denied'
  | 'archived'
  | 'stopped';

export interface PacketLifecycleStopContract {
  allowed_terminal_states: LifecycleOutcomeKind[];
  stopped_reason_required: boolean;
}

export interface PacketLifecycleContract {
  cycle_id: string;
  work_order_id: string;
  source_agent_id: string;
  target_agent_id: string;
  owner_agent_id: string;
  escalation_id?: string;
  escalation_target_agent_id?: string;
  escalation_reason_code?: string;
  stop: PacketLifecycleStopContract;
}

export interface PacketExecutionContext {
  source_kind: string;
  packet_id: string;
  session_id: string;
  manager_agent_id: string;
  session_label?: string;
  lifecycle_thread_id?: string;
  lifecycle_thread_title?: string;
  origin_message_id?: string;
  origin_message_text?: string;
}

export interface PacketMetadata {
  lifecycle_contract?: PacketLifecycleContract;
  execution_context?: PacketExecutionContext;
  monitor_context?: PacketExecutionContext;
  [key: string]: any;
}

/**
 * Full packet structure for inter-agent handoff
 */
export interface Packet {
  route_token: RouteToken;
  payload: string;
  prompt: string;
  signature?: string;
  issued_at: string;
  ttl_ms?: number;
  metadata?: PacketMetadata;
}

export interface LifecycleOutcome {
  kind: LifecycleOutcomeKind;
  summary: string;
  work_order_id?: string;
  escalation_id?: string;
  target_agent_id?: string;
  stop_reason?: string;
}

/**
 * Execution return classification (response from Core/SubCore)
 */
export interface ExecutionReturn {
  classification: 'execution-return-core' | 'execution-return-subcore';
  parent_run_id: string;
  tool_call_id?: string;
  response: string;
  timestamp: string;
  facts?: string[];
  claims?: string[];
  non_claims?: string[];
  residual_risks?: string[];
  lifecycle_outcome?: LifecycleOutcome;
  lifecycle_errors?: string[];
}

/**
 * Coordinator error response
 */
export interface CoordinatorError {
  error: string;
  failedChecks?: string[];
  details?: Record<string, any>;
  escalate: boolean;
  timestamp: string;
}

/**
 * Guardrail check result
 */
export interface GuardrailCheckResult {
  name: string;
  passed: boolean;
  details?: string;
}

/**
 * Full guardrail result for all 5 checks
 */
export interface GuardrailResult {
  passed: boolean;
  failedChecks: string[];
  details: Record<string, GuardrailCheckResult>;
  timestamp: string;
}

/**
 * Validation result for packet structure
 */
export interface ValidationResult {
  valid: boolean;
  errors: string[];
  warnings?: string[];
}

export type SessionReasoningEffort = 'low' | 'medium' | 'high' | 'xhigh';

/**
 * Options for coordinator handoff
 */
export interface CoordinatorOptions {
  timeout_ms?: number;
  retry_on_failure?: boolean;
  log_level?: 'debug' | 'info' | 'warn' | 'error';
  capture_events?: boolean;
  sdk_client?: any;
  session_model?: string;
  session_reasoning_effort?: SessionReasoningEffort;
}

export interface FetchResponseLike {
  ok: boolean;
  status: number;
  statusText: string;
  text(): Promise<string>;
}

export type FetchLike = (
  url: string,
  init: {
    method: string;
    headers?: Record<string, string>;
    body?: string;
  }
) => Promise<FetchResponseLike>;

/**
 * Session manager options
 */
export interface SessionManagerOptions {
  sdk_client?: any;
  permission_handler?: (agent: string, request: any) => Promise<{ approved: boolean }>;
  orchestrator_base_url?: string;
  fetch_impl?: FetchLike;
  session_model?: string;
  session_reasoning_effort?: SessionReasoningEffort;
}

/**
 * Dual handoff result
 */
export interface DualHandoffResult {
  core: ExecutionReturn | CoordinatorError;
  subcore: ExecutionReturn | CoordinatorError;
  timestamp: string;
}

/**
 * Type guard to check if result is error
 */
export function isCoordinatorError(result: any): result is CoordinatorError {
  return result && result.error !== undefined && result.escalate !== undefined;
}

/**
 * Type guard to check if result is execution return
 */
export function isExecutionReturn(result: any): result is ExecutionReturn {
  return result && result.classification !== undefined && result.parent_run_id !== undefined;
}
