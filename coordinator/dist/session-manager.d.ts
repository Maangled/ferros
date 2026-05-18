/**
 * Session Manager: SDK session lifecycle management
 * Wraps @github/copilot-sdk calls for session creation, prompt sending, and cleanup
 */
import { Packet, SessionManagerOptions } from './types';
type TargetAgent = 'core' | 'subcore';
/**
 * Interface for Copilot SDK client
 * (In real implementation, import from @github/copilot-sdk)
 */
export interface CopilotSDKClient {
    createSession(options: any): Promise<ClientSession>;
    deleteSession(sessionId: string): Promise<void>;
    resumeSession(sessionId: string): Promise<ClientSession>;
    listSessions(): Promise<SessionInfo[]>;
    shutdown?(): Promise<void>;
}
/**
 * Interface for Copilot SDK client session
 */
export interface ClientSession {
    id: string;
    agent?: string;
    on(event: string, handler: (event: any) => void): void;
    off(event: string, handler?: (event: any) => void): void;
    sendAndWait(options: {
        prompt: string;
    }): Promise<string>;
    send(prompt: string): void;
}
export interface SessionInfo {
    id: string;
    agent?: string;
    createdAt: string;
    lastActivity?: string;
}
/**
 * Session Manager: Manages SDK session lifecycle
 */
export declare class SessionManager {
    private sdkClient;
    private permissionHandler?;
    private activeSessions;
    private orchestratorBaseUrl?;
    private fetchImpl?;
    private sessionModel?;
    private sessionReasoningEffort?;
    constructor(options: SessionManagerOptions);
    /**
     * Get default SDK client.
     * Lazily loads the ESM-only Copilot SDK so CommonJS coordinator builds keep working.
     */
    private getDefaultSDKClient;
    /**
     * Create a new session targeting a specific agent
     */
    createSession(targetAgent: TargetAgent, packet?: Packet): Promise<ClientSession>;
    private getAgentName;
    /**
     * Get agent definition (Core or SubCore)
     */
    private getAgentDefinition;
    private resolveFetch;
    private buildOrchestratorPacketRequest;
    private buildOrchestratorIdempotencyKey;
    private enqueueOrchestratorPacket;
    /**
     * Send prompt and wait for response (blocking)
     */
    sendPrompt(session: ClientSession, prompt: string): Promise<string>;
    /**
     * Resume an existing session by ID (for explicit continuation)
     */
    resumeSession(sessionId: string): Promise<ClientSession>;
    /**
     * Cleanup session (delete from SDK)
     */
    cleanupSession(sessionId: string): Promise<void>;
    /**
     * List all active sessions
     */
    listActiveSessions(): Promise<SessionInfo[]>;
    /**
     * Cleanup all tracked sessions
     */
    cleanupAllSessions(): Promise<void>;
    shutdownIfIdle(): Promise<void>;
}
export declare function getSessionManager(options?: SessionManagerOptions): SessionManager;
export {};
//# sourceMappingURL=session-manager.d.ts.map