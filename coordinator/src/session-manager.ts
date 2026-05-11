/**
 * Session Manager: SDK session lifecycle management
 * Wraps @github/copilot-sdk calls for session creation, prompt sending, and cleanup
 */

import { SessionManagerOptions } from './types';

/**
 * Interface for Copilot SDK client
 * (In real implementation, import from @github/copilot-sdk)
 */
export interface CopilotSDKClient {
  createSession(options: any): Promise<ClientSession>;
  deleteSession(sessionId: string): Promise<void>;
  resumeSession(sessionId: string): Promise<ClientSession>;
  listSessions(): Promise<SessionInfo[]>;
}

/**
 * Interface for Copilot SDK client session
 */
export interface ClientSession {
  id: string;
  agent?: string;
  on(event: string, handler: (event: any) => void): void;
  off(event: string, handler?: (event: any) => void): void;
  sendAndWait(options: { prompt: string }): Promise<string>;
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
export class SessionManager {
  private sdkClient: CopilotSDKClient;
  private permissionHandler?: (agent: string, request: any) => Promise<{ approved: boolean }>;
  private activeSessions: Map<string, ClientSession> = new Map();

  constructor(options: SessionManagerOptions) {
    this.sdkClient = options.sdk_client || this.getDefaultSDKClient();
    this.permissionHandler = options.permission_handler;
  }

  /**
   * Get default SDK client (placeholder)
   * In real implementation, import and return Copilot SDK client
   */
  private getDefaultSDKClient(): CopilotSDKClient {
    // Placeholder client that fails at call time rather than constructor time.
    // This keeps coordinator construction testable without a live SDK binding.
    const unavailable = async () => {
      throw new Error('Copilot SDK client is not configured. Provide sdk_client in SessionManagerOptions.');
    };

    return {
      createSession: unavailable,
      deleteSession: unavailable,
      resumeSession: unavailable,
      listSessions: unavailable
    } as unknown as CopilotSDKClient;
  }

  /**
   * Create a new session targeting a specific agent
   */
  async createSession(targetAgent: 'core' | 'subcore'): Promise<ClientSession> {
    try {
      const agentMapping = {
        core: 'FERROS Core Agent',
        subcore: 'FERROS SubCore Agent'
      };

      const agentName = agentMapping[targetAgent];

      // Create session with target agent
      const session = await this.sdkClient.createSession({
        agent: agentName,
        customAgents: [this.getAgentDefinition(targetAgent)],
        onPermissionRequest: this.permissionHandler
          ? (agent: string, request: any) => this.permissionHandler!(agent, request)
          : undefined
      });

      // Track session
      this.activeSessions.set(session.id, session);

      console.log(`[SessionManager] Created session ${session.id} for agent ${agentName}`);

      return session;
    } catch (error) {
      console.error(`[SessionManager] Failed to create session: ${error}`);
      throw error;
    }
  }

  /**
   * Get agent definition (Core or SubCore)
   */
  private getAgentDefinition(targetAgent: 'core' | 'subcore'): any {
    const definitions = {
      core: {
        id: 'ferros-core',
        name: 'FERROS Core Agent',
        description: 'Executes lanes for main FERROS package across platform-neutral and cross-platform surfaces',
        infer: true,
        tools: [
          // Tools would be defined based on FERROS Core Agent capabilities
          'read',
          'search',
          'todo'
        ]
      },
      subcore: {
        id: 'ferros-subcore',
        name: 'FERROS SubCore Agent',
        description: 'Executes lanes for ADR-025 x86_64 FERROS-root incubation',
        infer: true,
        tools: ['read', 'search', 'todo']
      }
    };

    return definitions[targetAgent];
  }

  /**
   * Send prompt and wait for response (blocking)
   */
  async sendPrompt(session: ClientSession, prompt: string): Promise<string> {
    try {
      console.log(`[SessionManager] Sending prompt to session ${session.id}...`);
      const response = await session.sendAndWait({ prompt });
      console.log(`[SessionManager] Received response (${response.length} chars) from session ${session.id}`);
      return response;
    } catch (error) {
      console.error(`[SessionManager] Failed to send prompt: ${error}`);
      throw error;
    }
  }

  /**
   * Resume an existing session by ID (for explicit continuation)
   */
  async resumeSession(sessionId: string): Promise<ClientSession> {
    try {
      // Check if already in cache
      if (this.activeSessions.has(sessionId)) {
        console.log(`[SessionManager] Resumed session ${sessionId} from cache`);
        return this.activeSessions.get(sessionId)!;
      }

      const session = await this.sdkClient.resumeSession(sessionId);
      this.activeSessions.set(sessionId, session);
      console.log(`[SessionManager] Resumed session ${sessionId}`);
      return session;
    } catch (error) {
      console.error(`[SessionManager] Failed to resume session: ${error}`);
      throw error;
    }
  }

  /**
   * Cleanup session (delete from SDK)
   */
  async cleanupSession(sessionId: string): Promise<void> {
    try {
      await this.sdkClient.deleteSession(sessionId);
      this.activeSessions.delete(sessionId);
      console.log(`[SessionManager] Cleaned up session ${sessionId}`);
    } catch (error) {
      console.error(`[SessionManager] Failed to cleanup session: ${error}`);
      // Don't throw; cleanup failures should not block execution
    }
  }

  /**
   * List all active sessions
   */
  async listActiveSessions(): Promise<SessionInfo[]> {
    try {
      return await this.sdkClient.listSessions();
    } catch (error) {
      console.error(`[SessionManager] Failed to list sessions: ${error}`);
      return [];
    }
  }

  /**
   * Cleanup all tracked sessions
   */
  async cleanupAllSessions(): Promise<void> {
    const sessionIds = Array.from(this.activeSessions.keys());
    for (const sessionId of sessionIds) {
      await this.cleanupSession(sessionId);
    }
    console.log(`[SessionManager] Cleaned up ${sessionIds.length} sessions`);
  }
}

/**
 * Singleton instance for session manager
 */
let sessionManagerInstance: SessionManager | null = null;

export function getSessionManager(options?: SessionManagerOptions): SessionManager {
  if (!sessionManagerInstance) {
    sessionManagerInstance = new SessionManager(options || {});
  } else if (options?.sdk_client) {
    sessionManagerInstance = new SessionManager(options);
  }
  return sessionManagerInstance;
}
