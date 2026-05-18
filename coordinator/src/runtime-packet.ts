import {
  PacketExecutionContext,
  LifecycleOutcomeKind,
  Packet,
  PacketLifecycleContract,
  PacketLifecycleStopContract,
  PacketMetadata,
} from './types';

const VALID_TERMINAL_STATES: LifecycleOutcomeKind[] = [
  'report',
  'work_order',
  'escalation',
  'denied',
  'archived',
  'stopped',
];

type JsonRecord = Record<string, unknown>;

function asRecord(value: unknown): JsonRecord | undefined {
  if (typeof value !== 'object' || value === null || Array.isArray(value)) {
    return undefined;
  }

  return value as JsonRecord;
}

function readString(record: JsonRecord, snakeKey: string, camelKey?: string): string | undefined {
  const value = record[snakeKey] ?? (camelKey ? record[camelKey] : undefined);
  return typeof value === 'string' ? value : undefined;
}

function readNumber(record: JsonRecord, snakeKey: string, camelKey?: string): number | undefined {
  const value = record[snakeKey] ?? (camelKey ? record[camelKey] : undefined);
  return typeof value === 'number' && Number.isFinite(value) ? value : undefined;
}

function readBoolean(record: JsonRecord, snakeKey: string, camelKey?: string): boolean | undefined {
  const value = record[snakeKey] ?? (camelKey ? record[camelKey] : undefined);
  return typeof value === 'boolean' ? value : undefined;
}

function readStringArray(
  record: JsonRecord,
  snakeKey: string,
  camelKey?: string
): string[] | undefined {
  const value = record[snakeKey] ?? (camelKey ? record[camelKey] : undefined);
  if (!Array.isArray(value)) {
    return undefined;
  }

  return value.filter((item): item is string => typeof item === 'string');
}

function normalizeStopContract(raw: JsonRecord | undefined): PacketLifecycleStopContract | undefined {
  if (!raw) {
    return undefined;
  }

    const allowedTerminalStates =
      readStringArray(raw, 'allowed_terminal_states', 'allowedTerminalStates') || [];

  return {
    allowed_terminal_states: allowedTerminalStates.filter(
      (state): state is LifecycleOutcomeKind =>
        VALID_TERMINAL_STATES.includes(state as LifecycleOutcomeKind)
    ),
    stopped_reason_required:
      readBoolean(raw, 'stopped_reason_required', 'stoppedReasonRequired') ?? false,
  };
}

function normalizeLifecycleContract(
  raw: JsonRecord | undefined
): PacketLifecycleContract | undefined {
  if (!raw) {
    return undefined;
  }

  return {
    cycle_id: readString(raw, 'cycle_id', 'cycleId') || '',
    work_order_id: readString(raw, 'work_order_id', 'workOrderId') || '',
    source_agent_id: readString(raw, 'source_agent_id', 'sourceAgentId') || '',
    target_agent_id: readString(raw, 'target_agent_id', 'targetAgentId') || '',
    owner_agent_id: readString(raw, 'owner_agent_id', 'ownerAgentId') || '',
    escalation_id: readString(raw, 'escalation_id', 'escalationId'),
    escalation_target_agent_id: readString(
      raw,
      'escalation_target_agent_id',
      'escalationTargetAgentId'
    ),
    escalation_reason_code: readString(raw, 'escalation_reason_code', 'escalationReasonCode'),
    stop: normalizeStopContract(asRecord(raw.stop)) || {
      allowed_terminal_states: [],
      stopped_reason_required: false,
    },
  };
}

function normalizeExecutionContext(
  raw: JsonRecord | undefined
): PacketExecutionContext | undefined {
  if (!raw) {
    return undefined;
  }

  return {
    source_kind: readString(raw, 'source_kind', 'sourceKind') || 'monitor',
    packet_id: readString(raw, 'packet_id', 'packetId') || '',
    session_id: readString(raw, 'session_id', 'sessionId') || '',
    manager_agent_id:
      readString(raw, 'manager_agent_id', 'managerAgentId') ||
      readString(raw, 'manager') ||
      '',
    session_label: readString(raw, 'session_label', 'sessionLabel'),
    lifecycle_thread_id: readString(raw, 'lifecycle_thread_id', 'lifecycleThreadId'),
    lifecycle_thread_title: readString(raw, 'lifecycle_thread_title', 'lifecycleThreadTitle'),
    origin_message_id: readString(raw, 'origin_message_id', 'originMessageId'),
    origin_message_text: readString(raw, 'origin_message_text', 'originMessageText'),
  };
}

function normalizeMetadata(raw: JsonRecord | undefined): PacketMetadata | undefined {
  if (!raw) {
    return undefined;
  }

  const metadata: PacketMetadata = { ...raw };
  const lifecycleContract = normalizeLifecycleContract(
    asRecord(raw.lifecycle_contract) || asRecord(raw.lifecycleContract)
  );
  const executionContext = normalizeExecutionContext(
    asRecord(raw.execution_context) ||
      asRecord(raw.executionContext) ||
      asRecord(raw.monitor_context) ||
      asRecord(raw.monitorContext)
  );

  if (lifecycleContract) {
    metadata.lifecycle_contract = lifecycleContract;
  }

  if (executionContext) {
    metadata.execution_context = executionContext;
  }

  delete metadata.lifecycleContract;
  delete metadata.executionContext;
  delete metadata.monitorContext;
  delete metadata.monitor_context;

  return metadata;
}

function normalizeRouteToken(raw: JsonRecord): Packet['route_token'] {
  return {
    token_version:
      (readString(raw, 'token_version', 'tokenVersion') || 'v2') as Packet['route_token']['token_version'],
    issued_by:
      (readString(raw, 'issued_by', 'issuedBy') ||
        'FERROS Prompt Architect Agent') as Packet['route_token']['issued_by'],
    target_stream: ((readString(raw, 'target_stream', 'targetStream') || null) as
      | Packet['route_token']['target_stream']
      | null),
    target_family: ((readString(raw, 'target_family', 'targetFamily') || null) as
      | Packet['route_token']['target_family']
      | null),
    run_id: readString(raw, 'run_id', 'runId') || '',
    parent_run_id: readString(raw, 'parent_run_id', 'parentRunId') || '',
    recursion_depth: readNumber(raw, 'recursion_depth', 'recursionDepth') ?? 0,
    run_profile: readString(raw, 'run_profile', 'runProfile'),
    issued_at: readString(raw, 'issued_at', 'issuedAt') || '',
    expiry_cycle: readString(raw, 'expiry_cycle', 'expiryCycle'),
    posture: readString(raw, 'posture'),
    track: readString(raw, 'track') as Packet['route_token']['track'],
  };
}

export function normalizeRuntimePacket(input: unknown): Packet {
  const raw = asRecord(input);
  if (!raw) {
    throw new Error('Packet JSON must be an object');
  }

  const routeToken = asRecord(raw.route_token) || asRecord(raw.routeToken);
  if (!routeToken) {
    throw new Error('Missing route_token or routeToken');
  }

  return {
    route_token: normalizeRouteToken(routeToken),
    payload: readString(raw, 'payload') || '',
    prompt: readString(raw, 'prompt') || '',
    signature: readString(raw, 'signature'),
    issued_at: readString(raw, 'issued_at', 'issuedAt') || '',
    ttl_ms: readNumber(raw, 'ttl_ms', 'ttlMs'),
    metadata: normalizeMetadata(asRecord(raw.metadata)),
  };
}

export function parseRuntimePacketJson(packetJson: string): Packet {
  let parsed: unknown;

  try {
    parsed = JSON.parse(packetJson);
  } catch (error) {
    throw new Error(`Failed to parse packet JSON: ${error}`);
  }

  return normalizeRuntimePacket(parsed);
}