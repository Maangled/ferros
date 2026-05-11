/**
 * Event Tracer: Captures and traces SDK events during agent execution
 * Maps toolCallId to parent_run_id for response routing
 * Normalizes SDK responses back to execution-return classifications
 */

import { Packet, ExecutionReturn } from './types';

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

    return {
      classification,
      parent_run_id: packet.route_token.parent_run_id,
      tool_call_id: toolCallId,
      response,
      timestamp: new Date().toISOString(),
      facts: this.extractFacts(response),
      claims: this.extractClaims(response),
      non_claims: this.extractNonClaims(response),
      residual_risks: this.extractRisks(response)
    };
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
