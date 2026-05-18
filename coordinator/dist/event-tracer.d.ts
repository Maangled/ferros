/**
 * Event Tracer: Captures and traces SDK events during agent execution
 * Maps toolCallId to parent_run_id for response routing
 * Normalizes SDK responses back to execution-return classifications
 */
import { ExecutionReturn, Packet } from './types';
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
export declare class EventTracer {
    private events;
    private toolCallIdMap;
    /**
     * Setup event listeners on SDK session
     * Returns mapping of events captured
     */
    setupEventListeners(session: any, packet: Packet): {
        getToolCallId: () => string | undefined;
        getEvents: () => SDKEvent[];
    };
    /**
     * Get parent_run_id from toolCallId mapping
     */
    getParentRunId(toolCallId?: string): string | undefined;
    /**
     * Normalize SDK response back to ExecutionReturn classification
     */
    normalizeResponse(response: string, packet: Packet, toolCallId?: string): ExecutionReturn;
    /**
     * Extract a single terminal lifecycle outcome from the response.
     */
    private extractLifecycleOutcome;
    private extractSectionSummary;
    /**
     * Extract facts from response
     * Looks for "## Facts" section
     */
    private extractFacts;
    /**
     * Extract claims from response
     * Looks for "## Claims" or "## Proven" section
     */
    private extractClaims;
    /**
     * Extract non-claims from response
     * Looks for "## Non-Claims" or "## Out of Scope" section
     */
    private extractNonClaims;
    /**
     * Extract residual risks from response
     * Looks for "## Residual Risks" or "## Risks" section
     */
    private extractRisks;
    /**
     * Get all captured events
     */
    getEvents(): SDKEvent[];
    /**
     * Clear event history
     */
    clearEvents(): void;
    /**
     * Format events for logging
     */
    formatEventsLog(): string;
}
export declare function getEventTracer(): EventTracer;
//# sourceMappingURL=event-tracer.d.ts.map