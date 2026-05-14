/**
 * Event Tracer: Captures and traces SDK events during agent execution
 * Maps toolCallId to parent_run_id for response routing
 * Normalizes SDK responses back to execution-return classifications
 */

import {
  ExecutionReturn,
  LifecycleOutcome,
  LifecycleOutcomeKind,
  Packet,
} from './types';

const LIFECYCLE_SECTION_PATTERNS: Array<{
  kind: LifecycleOutcomeKind;
  label: string;
  pattern: RegExp;
}> = [
  {
    kind: 'work_order',
    label: 'Work Order',
    pattern: /##\s*(?:Work Order|Work-Order|Next Work Order)\s*\n([\s\S]*?)(?=\n##|$)/i,
  },
  {
    kind: 'escalation',
    label: 'Escalation',
    pattern: /##\s*Escalation\s*\n([\s\S]*?)(?=\n##|$)/i,
  },
  {
    kind: 'report',
    label: 'Report',
    pattern: /##\s*(?:Report|Status Report|Report Back)\s*\n([\s\S]*?)(?=\n##|$)/i,
  },
  {
    kind: 'denied',
    label: 'Denied',
    pattern: /##\s*Denied\s*\n([\s\S]*?)(?=\n##|$)/i,
  },
  {
    kind: 'archived',
    label: 'Archived',
    pattern: /##\s*Archived\s*\n([\s\S]*?)(?=\n##|$)/i,
  },
  {
    kind: 'stopped',
    label: 'Stop',
    pattern: /##\s*(?:Stop|Stopped|Stop Reason)\s*\n([\s\S]*?)(?=\n##|$)/i,
  },
];

type LifecycleSectionMatch =
  | {
      kind: LifecycleOutcomeKind;
      label: string;
      summary: string;
      empty?: false;
    }
  | {
      kind: LifecycleOutcomeKind;
      label: string;
      empty: true;
      summary?: never;
    };

export interface SDKEvent {
  type: 'started' | 'completed' | 'failed';
  toolCallId?: string;
  agent?: string;
  timestamp: string;
  data?: any;
}

/**
 * Event Tracer
 */
export class EventTracer {
  private events: SDKEvent[] = [];
  private toolCallIdMap: Map<string, string> = new Map(); // toolCallId -> parent_run_id

  /**
   * Setup event listeners on SDK session
   * Returns mapping of events captured
   */
  setupEventListeners(session: any, packet: Packet): {
    getToolCallId: () => string | undefined;
    getEvents: () => SDKEvent[];
  } {
    let lastToolCallId: string | undefined;

    // Listen for subagent.started event
    if (session.on) {
      session.on('subagent.started', (event: any) => {
        const sdkEvent: SDKEvent = {
          type: 'started',
          toolCallId: event.toolCallId,
          agent: event.agent,
          timestamp: new Date().toISOString(),
          data: event
        };

        lastToolCallId = event.toolCallId;

        // Map toolCallId to parent_run_id for response routing
        if (event.toolCallId && packet.route_token.parent_run_id) {
          this.toolCallIdMap.set(event.toolCallId, packet.route_token.parent_run_id);
        }

        this.events.push(sdkEvent);
        console.log(`[EventTracer] Agent started: toolCallId=${event.toolCallId}, agent=${event.agent}`);
      });

      session.on('subagent.completed', (event: any) => {
        const sdkEvent: SDKEvent = {
          type: 'completed',
          toolCallId: event.toolCallId,
          agent: event.agent,
          timestamp: new Date().toISOString(),
          data: event
        };

        this.events.push(sdkEvent);
        console.log(`[EventTracer] Agent completed: toolCallId=${event.toolCallId}`);
      });

      session.on('subagent.failed', (event: any) => {
        const sdkEvent: SDKEvent = {
          type: 'failed',
          toolCallId: event.toolCallId,
          agent: event.agent,
          timestamp: new Date().toISOString(),
          data: { error: event.error, reason: event.reason }
        };

        this.events.push(sdkEvent);
        console.error(`[EventTracer] Agent failed: toolCallId=${event.toolCallId}, error=${event.error}`);
      });
    }

    return {
      getToolCallId: () => lastToolCallId,
      getEvents: () => this.events
    };
  }

  /**
   * Get parent_run_id from toolCallId mapping
   */
  getParentRunId(toolCallId?: string): string | undefined {
    if (!toolCallId) return undefined;
    return this.toolCallIdMap.get(toolCallId);
  }

  /**
   * Normalize SDK response back to ExecutionReturn classification
   */
  normalizeResponse(
    response: string,
    packet: Packet,
    toolCallId?: string
  ): ExecutionReturn {
    const targetStream = packet.route_token.target_stream;
    const classification: 'execution-return-core' | 'execution-return-subcore' =
      targetStream === 'core' ? 'execution-return-core' : 'execution-return-subcore';
    const lifecycle = this.extractLifecycleOutcome(response, packet);

    return {
      classification,
      parent_run_id: packet.route_token.parent_run_id,
      tool_call_id: toolCallId,
      response,
      timestamp: new Date().toISOString(),
      facts: this.extractFacts(response),
      claims: this.extractClaims(response),
      non_claims: this.extractNonClaims(response),
      residual_risks: this.extractRisks(response),
      lifecycle_outcome: lifecycle.outcome,
      lifecycle_errors: lifecycle.errors,
    };
  }

  /**
   * Extract a single terminal lifecycle outcome from the response.
   */
  private extractLifecycleOutcome(
    response: string,
    packet: Packet
  ): { outcome?: LifecycleOutcome; errors?: string[] } {
    const matches: LifecycleSectionMatch[] = [];

    for (const section of LIFECYCLE_SECTION_PATTERNS) {
      const match = response.match(section.pattern);
      if (!match) {
        continue;
      }

      const summary = this.extractSectionSummary(match[1]);
      if (!summary) {
        matches.push({ kind: section.kind, label: section.label, empty: true });
        continue;
      }

      matches.push({ kind: section.kind, label: section.label, summary });
    }

    if (matches.length === 0) {
      return {
        errors: [
          'Missing lifecycle outcome section (expected one of Report, Work Order, Escalation, Denied, Archived, or Stop)',
        ],
      };
    }

    if (matches.some((match) => match.empty)) {
      const emptyLabels = matches
        .filter((match) => match.empty)
        .map((match) => match.label);
      return {
        errors: [`Lifecycle outcome section is empty: ${emptyLabels.join(', ')}`],
      };
    }

    if (matches.length > 1) {
      return {
        errors: [
          `Multiple lifecycle outcome sections found: ${matches.map((match) => match.label).join(', ')}`,
        ],
      };
    }

    const match = matches[0] as Extract<LifecycleSectionMatch, { summary: string }>;
    const contract = packet.metadata?.lifecycle_contract;
    const outcome: LifecycleOutcome = {
      kind: match.kind,
      summary: match.summary,
      work_order_id: match.kind === 'work_order' ? contract?.work_order_id : undefined,
      escalation_id: match.kind === 'escalation' ? contract?.escalation_id : undefined,
      target_agent_id:
        match.kind === 'escalation' ? contract?.escalation_target_agent_id : undefined,
      stop_reason: match.kind === 'stopped' ? match.summary : undefined,
    };

    return { outcome };
  }

  private extractSectionSummary(section: string): string {
    const lines = section
      .split('\n')
      .map((line) => line.trim())
      .filter(Boolean);

    if (lines.length === 0) {
      return '';
    }

    const bulletLine = lines.find(
      (line) => line.startsWith('-') || line.startsWith('•')
    );
    const summaryLine = bulletLine || lines[0];

    return summaryLine.replace(/^[-•]\s*/, '').trim();
  }

  /**
   * Extract facts from response
   * Looks for "## Facts" section
   */
  private extractFacts(response: string): string[] | undefined {
    const factsMatch = response.match(/##\s*Facts?\s*\n([\s\S]*?)(?=\n##|$)/i);
    if (factsMatch) {
      return factsMatch[1]
        .split('\n')
        .filter(line => line.trim().startsWith('-') || line.trim().startsWith('•'))
        .map(line => line.replace(/^[-•]\s*/, '').trim())
        .filter(f => f.length > 0);
    }
    return undefined;
  }

  /**
   * Extract claims from response
   * Looks for "## Claims" or "## Proven" section
   */
  private extractClaims(response: string): string[] | undefined {
    const claimsMatch = response.match(/##\s*(Claims?|Proven)\s*\n([\s\S]*?)(?=\n##|$)/i);
    if (claimsMatch) {
      return claimsMatch[2]
        .split('\n')
        .filter(line => line.trim().startsWith('-') || line.trim().startsWith('•'))
        .map(line => line.replace(/^[-•]\s*/, '').trim())
        .filter(c => c.length > 0);
    }
    return undefined;
  }

  /**
   * Extract non-claims from response
   * Looks for "## Non-Claims" or "## Out of Scope" section
   */
  private extractNonClaims(response: string): string[] | undefined {
    const nonClaimsMatch = response.match(/##\s*(Non-Claims?|Out of Scope|Not Claimed)\s*\n([\s\S]*?)(?=\n##|$)/i);
    if (nonClaimsMatch) {
      return nonClaimsMatch[2]
        .split('\n')
        .filter(line => line.trim().startsWith('-') || line.trim().startsWith('•'))
        .map(line => line.replace(/^[-•]\s*/, '').trim())
        .filter(nc => nc.length > 0);
    }
    return undefined;
  }

  /**
   * Extract residual risks from response
   * Looks for "## Residual Risks" or "## Risks" section
   */
  private extractRisks(response: string): string[] | undefined {
    const risksMatch = response.match(/##\s*(Residual Risks?|Risks?)\s*\n([\s\S]*?)(?=\n##|$)/i);
    if (risksMatch) {
      return risksMatch[2]
        .split('\n')
        .filter(line => line.trim().startsWith('-') || line.trim().startsWith('•'))
        .map(line => line.replace(/^[-•]\s*/, '').trim())
        .filter(r => r.length > 0);
    }
    return undefined;
  }

  /**
   * Get all captured events
   */
  getEvents(): SDKEvent[] {
    return this.events;
  }

  /**
   * Clear event history
   */
  clearEvents(): void {
    this.events = [];
    this.toolCallIdMap.clear();
  }

  /**
   * Format events for logging
   */
  formatEventsLog(): string {
    return this.events
      .map(e => `[${e.timestamp}] ${e.type.toUpperCase()}: toolCallId=${e.toolCallId}, agent=${e.agent}`)
      .join('\n');
  }
}

/**
 * Singleton instance
 */
let tracerInstance: EventTracer | null = null;

export function getEventTracer(): EventTracer {
  if (!tracerInstance) {
    tracerInstance = new EventTracer();
  }
  return tracerInstance;
}
