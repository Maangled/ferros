/**
 * Session Manager: SDK session lifecycle management
 * Wraps @github/copilot-sdk calls for session creation, prompt sending, and cleanup
 */

import fs from 'fs';
import path from 'path';
import { FetchLike, Packet, SessionManagerOptions, SessionReasoningEffort } from './types';

const REPO_ROOT = path.resolve(__dirname, '..', '..');
const AGENT_MANIFEST_PATH = path.join(REPO_ROOT, 'agents', 'manifest.json');
const COPILOT_CLI_SHIM_PATH = path.join(
  REPO_ROOT,
  'coordinator',
  'node_modules',
  '.bin',
  process.platform === 'win32' ? 'copilot.cmd' : 'copilot'
);
const COPILOT_CLI_PACKAGE_PATH = path.join(
  REPO_ROOT,
  'coordinator',
  'node_modules',
  '@github',
  'copilot-sdk',
  'node_modules',
  `@github/copilot-${process.platform}-${process.arch}`,
  process.platform === 'win32' ? 'copilot.exe' : 'copilot'
);
const COPILOT_NPM_LOADER_PATH = path.join(
  REPO_ROOT,
  'coordinator',
  'node_modules',
  '@github',
  'copilot-sdk',
  'node_modules',
  '@github',
  'copilot',
  'npm-loader.js'
);

type TargetAgent = 'core' | 'subcore';

interface AgentSourceManifest {
  entries: AgentSourceManifestEntry[];
}

interface AgentSourceManifestEntry {
  id: string;
  display_name: string;
  description?: string;
  source_path: string;
  mirror_path: string;
  tools?: string[];
}

interface LoadedAgentDefinition {
  id: string;
  name: string;
  displayName: string;
  description: string;
  tools: string[] | null;
  prompt: string;
}

interface SessionAgentDefinition {
  name: string;
  displayName: string;
  description: string;
  tools: string[] | null;
  prompt: string;
  infer: boolean;
}

const TARGET_AGENT_IDS: Record<TargetAgent, string> = {
  core: 'ferros-core',
  subcore: 'ferros-subcore',
};

type RuntimeCopilotSession = {
  sessionId: string;
  on(event: string, handler: (event: any) => void): () => void;
  send(options: { prompt: string }): Promise<string>;
  sendAndWait(
    options: { prompt: string },
    timeout?: number
  ): Promise<{ data?: { content?: string } } | undefined>;
};

type RuntimeCopilotClient = {
  start(): Promise<void>;
  stop(): Promise<Error[]>;
  forceStop?(): Promise<void>;
  createSession(config?: Record<string, unknown>): Promise<RuntimeCopilotSession>;
  resumeSession(
    sessionId: string,
    config?: Record<string, unknown>
  ): Promise<RuntimeCopilotSession>;
  listSessions(): Promise<Array<{ sessionId: string; startTime: Date; modifiedTime: Date }>>;
  deleteSession(sessionId: string): Promise<void>;
};

const importCopilotSdk = new Function(
  'specifier',
  'return import(specifier);'
) as (specifier: string) => Promise<{
  CopilotClient: new (options?: Record<string, unknown>) => RuntimeCopilotClient;
}>;

function parseFrontmatter(content: string): Record<string, string> {
  const lines = content.split(/\r?\n/);
  if (lines[0] !== '---') {
    return {};
  }

  const fields: Record<string, string> = {};
  let currentKey: string | null = null;
  let listValues: string[] = [];

  const flushList = () => {
    if (!currentKey) {
      return;
    }
    fields[currentKey] = listValues.join('\n');
    currentKey = null;
    listValues = [];
  };

  for (let index = 1; index < lines.length; index += 1) {
    const line = lines[index];
    if (line.trim() === '---') {
      flushList();
      break;
    }

    const trimmed = line.trimEnd();
    const keyValueMatch = trimmed.match(/^([A-Za-z0-9_-]+):(.*)$/);
    if (keyValueMatch) {
      flushList();
      const [, rawKey, rawValue] = keyValueMatch;
      const value = rawValue.trim();
      if (value.length === 0) {
        currentKey = rawKey.trim();
      } else {
        fields[rawKey.trim()] = value;
      }
      continue;
    }

    if (currentKey && trimmed.trimStart().startsWith('-')) {
      listValues.push(trimmed.trim().replace(/^\-\s*/, ''));
    }
  }

  return fields;
}

function parseFrontmatterList(value: string | undefined): string[] {
  if (!value) {
    return [];
  }

  const trimmed = String(value).trim();
  if (!trimmed) {
    return [];
  }

  if (trimmed.startsWith('[') && trimmed.endsWith(']')) {
    return trimmed
      .slice(1, -1)
      .split(',')
      .map((item) => item.trim().replace(/^['"]|['"]$/g, ''))
      .filter(Boolean);
  }

  return trimmed
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter(Boolean);
}

function stripFrontmatter(content: string): string {
  const lines = content.split(/\r?\n/);
  if (lines[0] !== '---') {
    return content.trim();
  }

  const closingIndex = lines.findIndex((line, index) => index > 0 && line.trim() === '---');
  if (closingIndex === -1) {
    return content.trim();
  }

  return lines.slice(closingIndex + 1).join('\n').trim();
}

function repoRelativePath(relativePath: string | undefined): string | null {
  if (!relativePath) {
    return null;
  }

  return path.join(REPO_ROOT, ...relativePath.split('/'));
}

function resolveCopilotCli(): { cliPath?: string; cliArgs?: string[] } {
  if (fs.existsSync(COPILOT_CLI_SHIM_PATH)) {
    return { cliPath: COPILOT_CLI_SHIM_PATH };
  }

  if (fs.existsSync(COPILOT_CLI_PACKAGE_PATH)) {
    return { cliPath: COPILOT_CLI_PACKAGE_PATH };
  }

  if (fs.existsSync(COPILOT_NPM_LOADER_PATH)) {
    return {
      cliPath: process.execPath,
      cliArgs: [COPILOT_NPM_LOADER_PATH],
    };
  }

  return {};
}

function extractSdkEventPayload(event: any): any {
  if (event && typeof event === 'object' && 'data' in event) {
    return event.data;
  }

  return event;
}

function loadAgentManifestEntry(agentId: string): AgentSourceManifestEntry | null {
  if (!fs.existsSync(AGENT_MANIFEST_PATH)) {
    return null;
  }

  const manifest = JSON.parse(
    fs.readFileSync(AGENT_MANIFEST_PATH, 'utf8')
  ) as AgentSourceManifest;
  return manifest.entries.find((entry) => entry.id === agentId) || null;
}

function readAgentDefinitionFromManifest(
  agentId: string,
  fallbackName: string,
  fallbackDescription: string,
  fallbackTools: string[]
): LoadedAgentDefinition | null {
  const manifestEntry = loadAgentManifestEntry(agentId);
  if (!manifestEntry) {
    return null;
  }

  const candidatePaths = [
    repoRelativePath(manifestEntry.mirror_path),
    repoRelativePath(manifestEntry.source_path),
  ].filter((candidate): candidate is string => Boolean(candidate && fs.existsSync(candidate)));

  const content = candidatePaths.length > 0
    ? fs.readFileSync(candidatePaths[0], 'utf8')
    : null;
  const frontmatter = content ? parseFrontmatter(content) : {};
  const tools = parseFrontmatterList(frontmatter.tools);
  const displayName = frontmatter.name || manifestEntry.display_name || fallbackName;
  const description = frontmatter.description
    || manifestEntry.description
    || fallbackDescription;
  const prompt = content
    ? stripFrontmatter(content)
    : `# ${displayName}\n\n${description}`;

  return {
    id: manifestEntry.id,
    name: displayName,
    displayName,
    description,
    tools: tools.length > 0 ? tools : (manifestEntry.tools || fallbackTools || null),
    prompt,
  };
}

interface OrchestratorPacketRequest {
  sessionId: string;
  manager: string;
  summary: string;
  originMessageId?: string;
  workOrderId?: string;
  lifecycleThreadId?: string;
  idempotencyKey: string;
}

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
  sendAndWait(options: { prompt: string }): Promise<string>;
  send(prompt: string): void;
}

export interface SessionInfo {
  id: string;
  agent?: string;
  createdAt: string;
  lastActivity?: string;
}

class CopilotSessionAdapter implements ClientSession {
  readonly id: string;
  readonly agent?: string;
  private readonly subscriptions = new Map<string, Map<(event: any) => void, () => void>>();

  constructor(private readonly session: RuntimeCopilotSession, agent?: string) {
    this.id = session.sessionId;
    this.agent = agent;
  }

  on(event: string, handler: (event: any) => void): void {
    const unsubscribe = this.session.on(event, (sdkEvent: any) => {
      handler(extractSdkEventPayload(sdkEvent));
    });

    let eventSubscriptions = this.subscriptions.get(event);
    if (!eventSubscriptions) {
      eventSubscriptions = new Map();
      this.subscriptions.set(event, eventSubscriptions);
    }

    eventSubscriptions.set(handler, unsubscribe);
  }

  off(event: string, handler?: (event: any) => void): void {
    const eventSubscriptions = this.subscriptions.get(event);
    if (!eventSubscriptions) {
      return;
    }

    if (handler) {
      const unsubscribe = eventSubscriptions.get(handler);
      if (unsubscribe) {
        unsubscribe();
        eventSubscriptions.delete(handler);
      }
    } else {
      for (const unsubscribe of eventSubscriptions.values()) {
        unsubscribe();
      }
      eventSubscriptions.clear();
    }

    if (eventSubscriptions.size === 0) {
      this.subscriptions.delete(event);
    }
  }

  async sendAndWait(options: { prompt: string }): Promise<string> {
    const response = await this.session.sendAndWait({ prompt: options.prompt });
    return typeof response?.data?.content === 'string' ? response.data.content : '';
  }

  send(prompt: string): void {
    void this.session.send({ prompt }).catch((error) => {
      console.error(`[SessionManager] Failed to send prompt asynchronously: ${error}`);
    });
  }

  dispose(): void {
    for (const eventSubscriptions of this.subscriptions.values()) {
      for (const unsubscribe of eventSubscriptions.values()) {
        unsubscribe();
      }
    }
    this.subscriptions.clear();
  }
}

class DefaultCopilotSDKClient implements CopilotSDKClient {
  private clientPromise: Promise<RuntimeCopilotClient> | null = null;
  private readonly sessions = new Map<string, CopilotSessionAdapter>();

  private async createRuntimeClient(): Promise<RuntimeCopilotClient> {
    const { CopilotClient } = await importCopilotSdk('@github/copilot-sdk');
    const cliConfig = resolveCopilotCli();
    const client = new CopilotClient({
      autoStart: false,
      cwd: REPO_ROOT,
      logLevel: 'info',
      ...cliConfig,
    });

    try {
      await client.start();
      return client;
    } catch (error) {
      try {
        if (typeof client.forceStop === 'function') {
          await client.forceStop();
        } else {
          await client.stop();
        }
      } catch {
        // Ignore cleanup failures from a partially-started client.
      }

      throw error;
    }
  }

  private async getRuntimeClient(): Promise<RuntimeCopilotClient> {
    if (!this.clientPromise) {
      this.clientPromise = this.createRuntimeClient().catch((error) => {
        this.clientPromise = null;
        throw error;
      });
    }

    return this.clientPromise;
  }

  private trackSession(runtimeSession: RuntimeCopilotSession, agent?: string): ClientSession {
    const session = new CopilotSessionAdapter(runtimeSession, agent);
    this.sessions.set(session.id, session);
    return session;
  }

  async createSession(options: any): Promise<ClientSession> {
    const client = await this.getRuntimeClient();
    const runtimeSession = await client.createSession(options as Record<string, unknown>);
    return this.trackSession(runtimeSession, typeof options?.agent === 'string' ? options.agent : undefined);
  }

  async deleteSession(sessionId: string): Promise<void> {
    const client = await this.getRuntimeClient();
    this.sessions.get(sessionId)?.dispose();
    this.sessions.delete(sessionId);
    await client.deleteSession(sessionId);
  }

  async resumeSession(sessionId: string): Promise<ClientSession> {
    const client = await this.getRuntimeClient();
    const runtimeSession = await client.resumeSession(sessionId);
    return this.trackSession(runtimeSession, this.sessions.get(sessionId)?.agent);
  }

  async listSessions(): Promise<SessionInfo[]> {
    if (!this.clientPromise) {
      return [];
    }

    const client = await this.getRuntimeClient();
    const sessions = await client.listSessions();
    return sessions.map((session) => ({
      id: session.sessionId,
      createdAt: session.startTime.toISOString(),
      lastActivity: session.modifiedTime.toISOString(),
    }));
  }

  async shutdown(): Promise<void> {
    if (!this.clientPromise) {
      return;
    }

    const clientPromise = this.clientPromise;
    this.clientPromise = null;

    try {
      const client = await clientPromise;
      const errors = await client.stop();
      if (errors.length > 0) {
        console.warn(`[SessionManager] Copilot SDK client stop returned ${errors.length} errors`);
      }
    } finally {
      for (const session of this.sessions.values()) {
        session.dispose();
      }
      this.sessions.clear();
    }
  }
}

/**
 * Session Manager: Manages SDK session lifecycle
 */
export class SessionManager {
  private sdkClient: CopilotSDKClient;
  private permissionHandler?: (agent: string, request: any) => Promise<{ approved: boolean }>;
  private activeSessions: Map<string, ClientSession> = new Map();
  private orchestratorBaseUrl?: string;
  private fetchImpl?: FetchLike;
  private sessionModel?: string;
  private sessionReasoningEffort?: SessionReasoningEffort;

  constructor(options: SessionManagerOptions) {
    this.sdkClient = options.sdk_client || this.getDefaultSDKClient();
    this.permissionHandler = options.permission_handler;
    this.orchestratorBaseUrl = options.orchestrator_base_url;
    this.fetchImpl = options.fetch_impl;
    this.sessionModel = options.session_model;
    this.sessionReasoningEffort = options.session_reasoning_effort;
  }

  /**
   * Get default SDK client.
   * Lazily loads the ESM-only Copilot SDK so CommonJS coordinator builds keep working.
   */
  private getDefaultSDKClient(): CopilotSDKClient {
    return new DefaultCopilotSDKClient();
  }

  /**
   * Create a new session targeting a specific agent
   */
  async createSession(targetAgent: TargetAgent, packet?: Packet): Promise<ClientSession> {
    try {
      const agentDefinition = this.getAgentDefinition(targetAgent);
      const agentName = agentDefinition.name;
      const onPermissionRequest = async (request: any) => {
        if (!this.permissionHandler) {
          return { kind: 'approved' };
        }

        const decision = await this.permissionHandler(agentName, request);
        return decision?.approved
          ? { kind: 'approved' }
          : { kind: 'denied-interactively-by-user' };
      };
      const sessionOptions: Record<string, unknown> = {
        agent: agentName,
        customAgents: [agentDefinition],
        workingDirectory: REPO_ROOT,
        onPermissionRequest,
      };
      if (this.sessionModel) {
        sessionOptions.model = this.sessionModel;
      }
      if (this.sessionReasoningEffort) {
        sessionOptions.reasoningEffort = this.sessionReasoningEffort;
      }

      // Create session with target agent
      const session = await this.sdkClient.createSession(sessionOptions);

      try {
        await this.enqueueOrchestratorPacket(session, targetAgent, packet);
      } catch (error) {
        try {
          await this.sdkClient.deleteSession(session.id);
        } catch (cleanupError) {
          console.error(
            `[SessionManager] Failed to cleanup session ${session.id} after orchestrator enqueue error: ${cleanupError}`
          );
        }
        throw error;
      }

      // Track session
      this.activeSessions.set(session.id, session);

      console.log(`[SessionManager] Created session ${session.id} for agent ${agentName}`);

      return session;
    } catch (error) {
      console.error(`[SessionManager] Failed to create session: ${error}`);
      throw error;
    }
  }

  private getAgentName(targetAgent: TargetAgent): string {
    return this.getAgentDefinition(targetAgent).name;
  }

  /**
   * Get agent definition (Core or SubCore)
   */
  private getAgentDefinition(targetAgent: TargetAgent): SessionAgentDefinition {
    const fallbackDefinitions: Record<TargetAgent, LoadedAgentDefinition> = {
      core: {
        id: 'ferros-core',
        name: 'FERROS Core Agent',
        displayName: 'FERROS Core Agent',
        description: 'Executes lanes for the main FERROS package across platform-neutral and cross-platform runtime surfaces.',
        tools: ['read', 'search', 'todo'],
        prompt: 'You are FERROS Core Agent. Execute bounded core-runtime work and report truthful results.'
      },
      subcore: {
        id: 'ferros-subcore',
        name: 'FERROS SubCore Agent',
        displayName: 'FERROS SubCore Agent',
        description: 'Executes lanes for ADR-025 x86_64 FERROS-root incubation.',
        tools: ['read', 'search', 'todo'],
        prompt: 'You are FERROS SubCore Agent. Execute bounded subcore-runtime work and report truthful results.'
      }
    };

    const fallback = fallbackDefinitions[targetAgent];
    const manifestDefinition = readAgentDefinitionFromManifest(
      TARGET_AGENT_IDS[targetAgent],
      fallback.name,
      fallback.description,
      fallback.tools || ['read', 'search', 'todo']
    );
    const definition = manifestDefinition || fallback;

    return {
      name: definition.name,
      displayName: definition.displayName,
      description: definition.description,
      tools: definition.tools,
      prompt: definition.prompt,
      infer: true,
    };
  }

  private resolveFetch(): FetchLike | undefined {
    if (this.fetchImpl) {
      return this.fetchImpl;
    }

    const candidate = (globalThis as { fetch?: unknown }).fetch;
    if (typeof candidate === 'function') {
      return candidate as FetchLike;
    }

    return undefined;
  }

  private buildOrchestratorPacketRequest(
    session: ClientSession,
    targetAgent: TargetAgent,
    packet: Packet
  ): OrchestratorPacketRequest {
    const workOrderId = packet.metadata?.lifecycle_contract?.work_order_id;
    const lifecycleThreadId = packet.metadata?.lifecycle_contract?.cycle_id;
    const idempotencyKey = this.buildOrchestratorIdempotencyKey(packet, targetAgent, workOrderId);

    return {
      sessionId: session.id,
      manager: this.getAgentName(targetAgent),
      summary: workOrderId
        ? `Coordinator handoff to ${this.getAgentName(targetAgent)} for ${workOrderId}`
        : `Coordinator handoff to ${this.getAgentName(targetAgent)} for run ${packet.route_token.run_id}`,
      originMessageId: packet.route_token.run_id,
      workOrderId,
      lifecycleThreadId,
      idempotencyKey
    };
  }

  private buildOrchestratorIdempotencyKey(
    packet: Packet,
    targetAgent: TargetAgent,
    workOrderId?: string
  ): string {
    return [
      'ferros-coordinator',
      targetAgent,
      packet.route_token.run_id,
      workOrderId || 'no-work-order'
    ].join(':');
  }

  private async enqueueOrchestratorPacket(
    session: ClientSession,
    targetAgent: TargetAgent,
    packet?: Packet
  ): Promise<void> {
    if (!this.orchestratorBaseUrl || !packet) {
      return;
    }

    const fetchImpl = this.resolveFetch();
    if (!fetchImpl) {
      throw new Error('Fetch implementation is required when orchestrator_base_url is configured.');
    }

    const request = this.buildOrchestratorPacketRequest(session, targetAgent, packet);
    const response = await fetchImpl(
      `${this.orchestratorBaseUrl.replace(/\/+$/, '')}/orchestrator/packets`,
      {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Idempotency-Key': request.idempotencyKey
        },
        body: JSON.stringify(request)
      }
    );

    if (!response.ok) {
      const detail = await response.text();
      throw new Error(
        `Failed to enqueue orchestrator packet (${response.status} ${response.statusText}): ${detail}`
      );
    }
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
      if (this.activeSessions.size === 0) {
        await this.sdkClient.shutdown?.();
      }
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

  async shutdownIfIdle(): Promise<void> {
    if (this.activeSessions.size > 0) {
      return;
    }

    await this.sdkClient.shutdown?.();
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
