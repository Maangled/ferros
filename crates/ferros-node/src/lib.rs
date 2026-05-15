#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::convert::Infallible;
use std::fmt;
use std::fs;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use ferros_agents::{
    Agent, AgentJsonRpcRequest, AgentJsonRpcResponse, AgentJsonRpcResult, AgentManifest,
    AgentName, AgentRegistry, AgentRpcAgentDetail, AgentRpcAgentSummary, AgentRpcSnapshot,
    AgentStatus, CapabilityRequirement, DenyLogEntry, EchoAgent, GrantStateRecord,
    InMemoryAgentRegistry, ReferenceAgentError, RegistryError, TimerAgent,
    JSON_RPC_AGENT_NOT_FOUND,
    JSON_RPC_AUTHORIZATION_DENIED, JSON_RPC_INVALID_PARAMS, JSON_RPC_INVALID_REQUEST,
    JSON_RPC_METHOD_NOT_FOUND, METHOD_AGENT_DESCRIBE, METHOD_AGENT_LIST, METHOD_AGENT_RUN,
    METHOD_AGENT_SNAPSHOT, METHOD_AGENT_STOP, METHOD_DENY_LOG_LIST, METHOD_GRANT_LIST,
};
use ferros_core::{
    Capability, CapabilityError, CapabilityGrantView, CapabilityRequest, DenyByDefaultPolicy,
    MessageEnvelope, MessageEnvelopeError, PolicyDecision, PolicyEngine, RequesterProfileIdError,
};
use ferros_hub::{
    default_local_runtime_summary, local_bridge_profile_id,
    local_hub_relative_json_path_is_valid, local_onramp_banned_wording,
    local_runway_evidence_is_non_evidentiary, local_runway_scope_is_local_only,
    local_runway_text_looks_remote_like_url, LocalBridgeAgent,
    LocalBridgeRegistrationError, LocalHubRuntimeSummary, LocalOnrampProposal,
    LOCAL_HUB_STATE_SNAPSHOT_PATH,
};
use ferros_profile::{
    grant_profile_capability, init_local_profile, revoke_profile_capability, CapabilityGrant,
    FileSystemProfileStore, LocalProfileStore, ProfileId, ProfileIdError, ProfileStoreError,
};
use ferros_runtime::{
    DequeEnvelopeQueue, DequeJobQueue, Executor, InMemoryExecutor, InMemoryMessageBus,
    LocalRunwayState, MessageBus,
};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

const DEFAULT_PROFILE_ID: &str = "profile-alpha";
const DEFAULT_PROFILE_NAME: &str = "Fresh Start";
const DEFAULT_PROFILE_DEVICE_LABEL: &str = "ferros-cli";
const CLI_STATE_DIRECTORY: &str = "ferros";
const CLI_STATE_FILE: &str = "agent-center.state";
const CLI_PROFILE_DIRECTORY: &str = ".ferros";
const CLI_PROFILE_FILE: &str = "profile.json";
const MONITOR_STATE_FILE: &str = "monitor-state.json";
const MONITOR_STATE_SCHEMA_VERSION: u32 = 1;
const PROFILE_REVOKE_REASON: &str = "revoked via ferros profile revoke";
const LOCAL_SHELL_DEFAULT_PORT: u16 = 4317;
const MAX_HTTP_REQUEST_BYTES: usize = 64 * 1024;
const MONITOR_MAX_TIMELINE_EVENTS: usize = 200;
const MONITOR_MAX_ARCHIVED_SESSIONS: usize = 120;
const MONITOR_MAX_LIFECYCLE_THREADS: usize = 96;
const MONITOR_MAX_THREAD_ENTRIES: usize = 64;
const LOCAL_SHELL_HTML: &str = include_str!("../../../site/agent-center-shell.html");
const LOCAL_SHELL_ACCEPTANCE_HARNESS_HTML: &str =
    include_str!("../../../harnesses/localhost-shell-acceptance-harness.html");

static MONITOR_STATE: OnceLock<Mutex<MonitorState>> = OnceLock::new();

const MONITOR_AGENT_MANIFEST_PATH: &str = "agents/manifest.json";
const MONITOR_AGENT_SOURCE_ROOT: &str = "agents/source";
const MONITOR_AGENT_MIRROR_ROOT: &str = ".github/agents";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct MonitorMessage {
    id: String,
    speaker: String,
    who: String,
    text: String,
    at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct MonitorNotification {
    id: String,
    packet_id: Option<String>,
    session_id: Option<String>,
    lifecycle_thread_id: Option<String>,
    severity: String,
    title: String,
    summary: String,
    action: String,
    created_at: String,
    status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct MonitorSession {
    id: String,
    label: String,
    active_agent: String,
    thread_id: Option<String>,
    created_at: String,
    messages: Vec<MonitorMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct MonitorArchivedSession {
    id: String,
    label: String,
    reason: String,
    archived_at: String,
    preview: Vec<MonitorMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct MonitorLoop {
    id: String,
    agent: String,
    state: String,
    category: String,
    status: String,
    status_reason: String,
    status_detail: String,
    description: String,
    started_at: String,
    updated_at: String,
    stale_after: Option<String>,
    progress: Option<u8>,
    thread_id: Option<String>,
    work_order_id: Option<String>,
    escalation_id: Option<String>,
    source_agent_id: Option<String>,
    target_agent_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct MonitorEvent {
    id: String,
    kind: String,
    text: String,
    at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct MonitorLifecycleEntry {
    id: String,
    kind: String,
    speaker: String,
    who: String,
    text: String,
    at: String,
    status: Option<String>,
    next_action: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct MonitorLifecycleThread {
    id: String,
    title: String,
    kind: String,
    status: String,
    owner_agent: String,
    source_agent_id: Option<String>,
    target_agent_id: Option<String>,
    work_order_id: Option<String>,
    escalation_id: Option<String>,
    created_at: String,
    updated_at: String,
    entries: Vec<MonitorLifecycleEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct MonitorAgentDirectoryEntry {
    id: String,
    display_name: String,
    description: String,
    family: String,
    role: String,
    lane: String,
    source_path: String,
    mirror_path: String,
    sync_state: String,
    user_invocable: bool,
    tools: Vec<String>,
    child_agents: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct MonitorAgentSourceTreeStatus {
    manifest_path: String,
    canonical_root: String,
    mirror_root: String,
    sync_state: String,
    entry_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct MonitorPacket {
    id: String,
    session_id: String,
    origin_message_id: String,
    work_order_id: Option<String>,
    manager: String,
    state: String,
    lifecycle_thread_id: Option<String>,
    notification_id: Option<String>,
    created_at: String,
    updated_at: String,
    summary: String,
    last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct MonitorStateSnapshot {
    open_chats: Vec<MonitorSession>,
    archived_chats: Vec<MonitorArchivedSession>,
    notifications: Vec<MonitorNotification>,
    running_loops: Vec<MonitorLoop>,
    timeline: Vec<MonitorEvent>,
    lifecycle_threads: Vec<MonitorLifecycleThread>,
    packets: Vec<MonitorPacket>,
    agent_directory: Vec<MonitorAgentDirectoryEntry>,
    agent_source_tree: MonitorAgentSourceTreeStatus,
    selected_chat_id: Option<String>,
    selected_lifecycle_thread_id: Option<String>,
    next_id: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct MonitorState {
    open_chats: Vec<MonitorSession>,
    archived_chats: Vec<MonitorArchivedSession>,
    notifications: Vec<MonitorNotification>,
    running_loops: Vec<MonitorLoop>,
    timeline: Vec<MonitorEvent>,
    lifecycle_threads: Vec<MonitorLifecycleThread>,
    packets: Vec<MonitorPacket>,
    selected_chat_id: Option<String>,
    selected_lifecycle_thread_id: Option<String>,
    next_id: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct PersistedMonitorState {
    schema_version: u32,
    state: MonitorState,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MonitorCreateSessionRequest {
    label: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MonitorMessageRequest {
    speaker: String,
    who: String,
    text: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MonitorLifecycleMessageRequest {
    speaker: String,
    who: String,
    text: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MonitorRouteRequest {
    target: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MonitorLoopTransitionRequest {
    action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum MonitorDispatchStatus {
    Routed,
    Resolved,
    HumanInterventionRequired,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct MonitorDispatchRequest {
    session_id: String,
    message_id: String,
    operator_text: String,
    active_agent: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct MonitorDispatchResult {
    ferros_reply: String,
    packet_id: Option<String>,
    manager: Option<String>,
    lifecycle_thread_id: Option<String>,
    notification_id: Option<String>,
    /// Backend that accepted the dispatch: "scaffold" | "runtime.bus" | "coordinator.sdk"
    backend: Option<String>,
    status: MonitorDispatchStatus,
}

/// Result returned by a dispatch backend implementation.
#[derive(Debug, Clone, PartialEq, Eq)]
struct MonitorDispatchBackendResult {
    /// Whether the backend accepted the dispatch request.
    accepted: bool,
    /// Identifier for the backend implementation (e.g. "scaffold", "coordinator.sdk").
    backend: String,
    /// Optional message to surface in the monitor reply.
    message: String,
    /// Error detail if accepted is false.
    error: Option<String>,
}

/// Interface for dispatch backends. The scaffold implementation is the only concrete
/// backend today. Future implementations will call runtime bus or coordinator SDK.
trait MonitorDispatchBackend: Send + Sync {
    fn handle_dispatch(
        &self,
        session_id: &str,
        target: &DispatchTarget,
        operator_text: &str,
    ) -> MonitorDispatchBackendResult;
}

/// Scaffold backend: accepts all dispatches without calling any external runtime.
/// Use until runtime/coordinator handoff is actually wired.
struct ScaffoldMonitorDispatchBackend;

impl MonitorDispatchBackend for ScaffoldMonitorDispatchBackend {
    fn handle_dispatch(
        &self,
        _session_id: &str,
        _target: &DispatchTarget,
        _operator_text: &str,
    ) -> MonitorDispatchBackendResult {
        MonitorDispatchBackendResult {
            accepted: true,
            backend: "scaffold".to_owned(),
            message: "Packet staged for manager. Live execution is not yet connected.".to_owned(),
            error: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AgentSourceTreeManifest {
    entries: Vec<AgentSourceTreeManifestEntry>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AgentSourceTreeManifestEntry {
    id: String,
    display_name: String,
    description: Option<String>,
    family: String,
    role: String,
    lane: Option<String>,
    source_path: String,
    mirror_path: String,
    user_invocable: Option<bool>,
    tools: Option<Vec<String>>,
    child_agents: Option<Vec<String>>,
}

impl Default for MonitorState {
    fn default() -> Self {
        Self {
            open_chats: Vec::new(),
            archived_chats: Vec::new(),
            notifications: Vec::new(),
            running_loops: Vec::new(),
            timeline: Vec::new(),
            lifecycle_threads: Vec::new(),
            packets: Vec::new(),
            selected_chat_id: None,
            selected_lifecycle_thread_id: None,
            next_id: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DemoSummary {
    pub started_agents: Vec<String>,
    pub echo_response: String,
    pub timer_event: String,
    pub denied_requests: usize,
    pub log_entries: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationDenyDetail {
    pub summary: String,
    pub missing_requirements: Vec<CapabilityRequirement>,
}

impl AuthorizationDenyDetail {
    fn from_missing_requirements(
        agent_name: &str,
        missing_requirements: Vec<CapabilityRequirement>,
    ) -> Self {
        let summary = format!(
            "{agent_name} missing {}",
            missing_requirements
                .iter()
                .map(|requirement| requirement.capability.as_str())
                .collect::<Vec<_>>()
                .join(",")
        );

        Self {
            summary,
            missing_requirements,
        }
    }

    fn from_summary(summary: impl Into<String>) -> Self {
        Self {
            summary: summary.into(),
            missing_requirements: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DemoError {
    UnknownAgent(String),
    ManifestMissingCapabilities(String),
    AuthorizationDenied(AuthorizationDenyDetail),
    MissingEchoResponse,
    MissingTimerEvent,
    Profile(ProfileIdError),
    Capability(CapabilityError),
    RequesterProfile(RequesterProfileIdError),
    Envelope(MessageEnvelopeError),
    Registry(RegistryError),
    Agent(ReferenceAgentError),
}

impl fmt::Display for DemoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownAgent(name) => write!(f, "unknown agent: {name}"),
            Self::ManifestMissingCapabilities(name) => {
                write!(f, "agent {name} has no declared capabilities")
            }
            Self::AuthorizationDenied(detail) => {
                write!(f, "authorization denied: {}", detail.summary)
            }
            Self::MissingEchoResponse => write!(f, "echo agent did not return a response"),
            Self::MissingTimerEvent => write!(f, "timer agent did not emit an event"),
            Self::Profile(error) => write!(f, "{error}"),
            Self::Capability(error) => write!(f, "{error}"),
            Self::RequesterProfile(error) => write!(f, "{error}"),
            Self::Envelope(error) => write!(f, "{error}"),
            Self::Registry(error) => write!(f, "{error}"),
            Self::Agent(error) => write!(f, "{error}"),
        }
    }
}

impl From<ProfileIdError> for DemoError {
    fn from(value: ProfileIdError) -> Self {
        Self::Profile(value)
    }
}

impl From<CapabilityError> for DemoError {
    fn from(value: CapabilityError) -> Self {
        Self::Capability(value)
    }
}

impl From<RequesterProfileIdError> for DemoError {
    fn from(value: RequesterProfileIdError) -> Self {
        Self::RequesterProfile(value)
    }
}

impl From<MessageEnvelopeError> for DemoError {
    fn from(value: MessageEnvelopeError) -> Self {
        Self::Envelope(value)
    }
}

impl From<RegistryError> for DemoError {
    fn from(value: RegistryError) -> Self {
        Self::Registry(value)
    }
}

impl From<ReferenceAgentError> for DemoError {
    fn from(value: ReferenceAgentError) -> Self {
        Self::Agent(value)
    }
}

struct HostedAgent {
    manifest: AgentManifest,
    agent: Box<dyn Agent<Error = ReferenceAgentError>>,
}

#[derive(Debug, Clone)]
struct LocalBridgeStandInAgent {
    manifest: AgentManifest,
    status: AgentStatus,
}

impl LocalBridgeStandInAgent {
    fn new_default() -> Self {
        let bridge_agent = LocalBridgeAgent::new_default();
        let agent_name = AgentName::new(bridge_agent.name)
            .expect("default local bridge name should be valid");
        let profile_id = local_bridge_profile_id();
        let required_capabilities = bridge_agent
            .required_local_capabilities
            .into_iter()
            .map(|capability| CapabilityRequirement::new(profile_id.clone(), capability))
            .collect();

        Self {
            manifest: AgentManifest::new(agent_name, bridge_agent.version, required_capabilities),
            status: AgentStatus::Registered,
        }
    }

    fn manifest(&self) -> AgentManifest {
        self.manifest.clone()
    }
}

impl Agent for LocalBridgeStandInAgent {
    type Error = ReferenceAgentError;

    fn id(&self) -> &AgentName {
        &self.manifest.name
    }

    fn capabilities(&self) -> &[CapabilityRequirement] {
        &self.manifest.required_capabilities
    }

    fn start(&mut self) -> Result<(), Self::Error> {
        self.status = AgentStatus::Running;
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Self::Error> {
        self.status = AgentStatus::Stopped;
        Ok(())
    }

    fn status(&self) -> AgentStatus {
        self.status
    }
}

struct ActiveGrantView<'a> {
    grant: &'a CapabilityGrant,
}

impl CapabilityGrantView for ActiveGrantView<'_> {
    fn profile_id(&self) -> &str {
        self.grant.profile_id.as_str()
    }

    fn capability(&self) -> &str {
        &self.grant.capability
    }

    fn is_active(&self) -> bool {
        !self.grant.is_revoked()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentRecord {
    pub manifest: AgentManifest,
    pub status: AgentStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentCliCommand {
    List,
    Describe { name: String },
    Run { name: String },
    Stop { name: String },
    Logs { name: Option<String> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocalAgentApiCommand {
    List,
    Describe { name: String },
    Run { name: String },
    Stop { name: String },
    Logs { name: Option<String> },
}

impl From<AgentCliCommand> for LocalAgentApiCommand {
    fn from(value: AgentCliCommand) -> Self {
        match value {
            AgentCliCommand::List => Self::List,
            AgentCliCommand::Describe { name } => Self::Describe { name },
            AgentCliCommand::Run { name } => Self::Run { name },
            AgentCliCommand::Stop { name } => Self::Stop { name },
            AgentCliCommand::Logs { name } => Self::Logs { name },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocalAgentApiResponse {
    AgentList { agents: Vec<AgentRecord> },
    AgentDetail { agent: AgentRecord },
    AgentLifecycle { agent: AgentRecord },
    AgentLogs { entries: Vec<String> },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalRunwaySummary {
    pub surface: String,
    pub scope: String,
    pub evidence: String,
    pub checkpoint_state: String,
    pub checkpoint_detail: String,
    pub checkpoint_position: usize,
    pub checkpoint_total: usize,
    pub profile_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<String>,
    pub agent_count: usize,
    pub agents: Vec<LocalRunwayAgentSummary>,
    pub grant_count: usize,
    pub active_grant_count: usize,
    pub revoked_grant_count: usize,
    pub deny_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_deny: Option<LocalRunwayDenySummary>,
    pub checklist: Vec<LocalRunwayChecklistItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hub_restart: Option<LocalRunwayHubRestartSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hub_onramp_proposal: Option<LocalRunwayHubOnrampProposalSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hub_onramp_decision_receipt: Option<LocalRunwayHubOnrampDecisionReceiptSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalRunwayAgentSummary {
    pub name: String,
    pub version: String,
    pub status: String,
    pub required_capabilities: Vec<String>,
}

impl From<AgentRecord> for LocalRunwayAgentSummary {
    fn from(value: AgentRecord) -> Self {
        Self {
            name: value.manifest.name.as_str().to_owned(),
            version: value.manifest.version.to_string(),
            status: format_agent_status(value.status).to_owned(),
            required_capabilities: value
                .manifest
                .required_capabilities
                .into_iter()
                .map(|requirement| requirement.capability)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalRunwayDenySummary {
    pub entry_id: usize,
    pub kind: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability: Option<String>,
}

impl From<DenyLogEntry> for LocalRunwayDenySummary {
    fn from(value: DenyLogEntry) -> Self {
        Self {
            entry_id: value.entry_id,
            kind: value.kind,
            message: value.message,
            agent_name: value.agent_name,
            capability: value.capability,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalRunwayHubRestartSummary {
    pub reload_status: String,
    pub snapshot_path: String,
    pub scope: String,
    pub evidence: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prior_bridge_manifest_identity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prior_policy_decision_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prior_artifact_relative_output_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalRunwayHubOnrampProposalSummary {
    pub source: String,
    pub proposal_id: String,
    pub bridge_agent_name: String,
    pub stand_in_entity_name: String,
    pub requested_capability: String,
    pub requested_action: String,
    pub quarantine_status: String,
    pub scope: String,
    pub evidence: String,
    pub local_artifact_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalRunwayHubOnrampDecisionReceiptSummary {
    pub proposal_id: String,
    pub decision_label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_detail: Option<String>,
    pub local_artifact_path: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LocalRunwayChecklistStatus {
    Observed,
    Pending,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalRunwayChecklistItem {
    pub item: String,
    pub status: LocalRunwayChecklistStatus,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalAgentApi {
    state_path: PathBuf,
}

impl Default for LocalAgentApi {
    fn default() -> Self {
        Self::new()
    }
}

impl LocalAgentApi {
    #[must_use]
    pub fn new() -> Self {
        Self {
            state_path: cli_state_path(),
        }
    }

    #[must_use]
    pub fn at_state_path(state_path: impl Into<PathBuf>) -> Self {
        Self {
            state_path: state_path.into(),
        }
    }

    pub fn execute(
        &self,
        command: LocalAgentApiCommand,
    ) -> Result<LocalAgentApiResponse, CliError> {
        execute_local_agent_api_with_state_path(command, &self.state_path)
    }

    pub fn runway_summary(&self) -> Result<LocalRunwaySummary, CliError> {
        self.runway_summary_with_store_and_profile_path(
            &default_profile_path(),
            &FileSystemProfileStore,
        )
    }

    fn runway_summary_with_store_and_profile_path<S: LocalProfileStore>(
        &self,
        profile_path: &Path,
        store: &S,
    ) -> Result<LocalRunwaySummary, CliError> {
        build_local_runway_summary_with_store(&self.state_path, profile_path, store)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileCliCommand {
    Init { path: PathBuf },
    Show { path: PathBuf },
    Export { path: PathBuf, bundle_path: PathBuf },
    Import { path: PathBuf, bundle_path: PathBuf },
    Grant { path: PathBuf, capability: String },
    Revoke { path: PathBuf, capability: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProfileShellRequest {
    action: String,
    #[serde(default)]
    profile_path: Option<String>,
    #[serde(default)]
    bundle_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProfileShellStatus {
    kind: String,
    summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProfileShellErrorDetail {
    code: String,
    message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProfileShellResponse {
    ok: bool,
    action: String,
    status: ProfileShellStatus,
    profile_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    bundle_path: Option<String>,
    lines: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    profile: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_detail: Option<ProfileShellErrorDetail>,
}

#[derive(Debug)]
pub enum CliError {
    Usage(&'static str),
    InvalidState(String),
    Io(io::Error),
    Profile(ProfileStoreError),
    Runtime(DemoError),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Usage(message) => write!(f, "{message}"),
            Self::InvalidState(message) => write!(f, "invalid CLI state: {message}"),
            Self::Io(error) => write!(f, "{error}"),
            Self::Profile(error) => write!(f, "{error}"),
            Self::Runtime(error) => write!(f, "{error}"),
        }
    }
}

impl From<io::Error> for CliError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<ProfileStoreError> for CliError {
    fn from(value: ProfileStoreError) -> Self {
        Self::Profile(value)
    }
}

impl From<DemoError> for CliError {
    fn from(value: DemoError) -> Self {
        Self::Runtime(value)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct CliState {
    agent_statuses: BTreeMap<String, AgentStatus>,
    log_entries: Vec<String>,
}

impl CliState {
    fn load(path: &Path) -> Result<Self, CliError> {
        match fs::read_to_string(path) {
            Ok(contents) => Self::parse(&contents),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(Self::default()),
            Err(error) => Err(CliError::Io(error)),
        }
    }

    fn parse(contents: &str) -> Result<Self, CliError> {
        let mut state = Self::default();

        for line in contents.lines() {
            if line.is_empty() {
                continue;
            }

            let Some((kind, rest)) = line.split_once('\t') else {
                return Err(CliError::InvalidState(format!(
                    "malformed state line: {line}"
                )));
            };

            match kind {
                "status" => {
                    let Some((name, status_label)) = rest.split_once('\t') else {
                        return Err(CliError::InvalidState(format!(
                            "malformed status entry: {line}"
                        )));
                    };

                    let status = parse_agent_status(status_label).ok_or_else(|| {
                        CliError::InvalidState(format!(
                            "unsupported status {status_label} for {name}"
                        ))
                    })?;

                    state.set_status(name, status);
                }
                "log" => state.log_entries.push(rest.to_owned()),
                _ => {
                    return Err(CliError::InvalidState(format!(
                        "unknown state entry kind: {kind}"
                    )));
                }
            }
        }

        Ok(state)
    }

    fn save(&self, path: &Path) -> Result<(), CliError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(path, self.encode())?;
        Ok(())
    }

    fn encode(&self) -> String {
        let mut lines = Vec::new();

        for (name, status) in &self.agent_statuses {
            lines.push(format!("status\t{name}\t{}", format_agent_status(*status)));
        }

        for entry in &self.log_entries {
            lines.push(format!("log\t{entry}"));
        }

        lines.join("\n")
    }

    fn set_status(&mut self, name: &str, status: AgentStatus) {
        if status == AgentStatus::Registered {
            self.agent_statuses.remove(name);
        } else {
            self.agent_statuses.insert(name.to_owned(), status);
        }
    }
}

pub struct DemoRuntime {
    registry: InMemoryAgentRegistry,
    agents: BTreeMap<String, HostedAgent>,
    grants: Vec<CapabilityGrant>,
    policy: DenyByDefaultPolicy,
    executor: InMemoryExecutor<DequeJobQueue<MessageEnvelope>>,
    bus: InMemoryMessageBus<DequeEnvelopeQueue>,
    log_entries: Vec<String>,
    next_nonce: u64,
}

impl DemoRuntime {
    #[must_use]
    pub fn new(grants: Vec<CapabilityGrant>) -> Self {
        Self {
            registry: InMemoryAgentRegistry::default(),
            agents: BTreeMap::new(),
            grants,
            policy: DenyByDefaultPolicy,
            executor: InMemoryExecutor::new(),
            bus: InMemoryMessageBus::new(),
            log_entries: Vec::new(),
            next_nonce: 1,
        }
    }

    pub fn reference_host() -> Result<Self, DemoError> {
        let profile_id = ProfileId::new(DEFAULT_PROFILE_ID)?;
        let grants = vec![
            CapabilityGrant::new(profile_id.clone(), "agent.echo"),
            CapabilityGrant::new(profile_id.clone(), "agent.timer"),
        ];

        Self::reference_host_with_profile_id_and_grants(profile_id, grants)
    }

    #[cfg(test)]
    fn reference_host_with_grants(grants: Vec<CapabilityGrant>) -> Result<Self, DemoError> {
        let profile_id = ProfileId::new(DEFAULT_PROFILE_ID)?;

        Self::reference_host_with_profile_id_and_grants(profile_id, grants)
    }

    fn reference_host_with_profile_id_and_grants(
        profile_id: ProfileId,
        grants: Vec<CapabilityGrant>,
    ) -> Result<Self, DemoError> {
        let mut runtime = Self::new(grants);

        let echo = EchoAgent::new(profile_id.clone());
        let local_bridge = LocalBridgeStandInAgent::new_default();
        let timer = TimerAgent::new(profile_id);

        runtime.register(echo.manifest(), Box::new(echo))?;
        runtime.register(local_bridge.manifest(), Box::new(local_bridge))?;
        runtime.register(timer.manifest(), Box::new(timer))?;

        Ok(runtime)
    }

    pub fn register(
        &mut self,
        manifest: AgentManifest,
        agent: Box<dyn Agent<Error = ReferenceAgentError>>,
    ) -> Result<(), DemoError> {
        self.registry.register(manifest.clone())?;
        self.agents.insert(
            manifest.name.as_str().to_owned(),
            HostedAgent { manifest, agent },
        );
        Ok(())
    }

    pub fn list_agents(&self) -> Vec<String> {
        self.registry
            .list()
            .into_iter()
            .map(|summary| summary.name.as_str().to_owned())
            .collect()
    }

    pub fn start_agent(&mut self, name: &str) -> Result<(), DemoError> {
        self.start_agent_internal(name, true)
    }

    fn start_agent_internal(&mut self, name: &str, record_log: bool) -> Result<(), DemoError> {
        let required_capabilities = {
            let hosted = self
                .agents
                .get(name)
                .ok_or_else(|| DemoError::UnknownAgent(name.to_owned()))?;
            hosted.manifest.required_capabilities.clone()
        };

        let mut missing = Vec::new();

        for requirement in required_capabilities {
            let decision = self.evaluate_policy(
                requirement.profile_id.as_str(),
                requirement.capability.as_str(),
            )?;

            if !decision.is_allowed() {
                missing.push(requirement);
            }
        }

        if missing.is_empty() {
            let hosted = self
                .agents
                .get_mut(name)
                .ok_or_else(|| DemoError::UnknownAgent(name.to_owned()))?;
            hosted.agent.start()?;
            if record_log {
                self.log_entries.push(format!("started:{name}"));
            }
            return Ok(());
        }

        let detail = AuthorizationDenyDetail::from_missing_requirements(name, missing);
        self.log_entries
            .push(format!("denied-start:{}", detail.summary));
        Err(DemoError::AuthorizationDenied(detail))
    }

    pub fn stop_agent(&mut self, name: &str) -> Result<(), DemoError> {
        self.stop_agent_internal(name, true)
    }

    fn stop_agent_internal(&mut self, name: &str, record_log: bool) -> Result<(), DemoError> {
        let hosted = self
            .agents
            .get_mut(name)
            .ok_or_else(|| DemoError::UnknownAgent(name.to_owned()))?;
        hosted.agent.stop()?;
        if record_log {
            self.log_entries.push(format!("stopped:{name}"));
        }
        Ok(())
    }

    #[must_use]
    pub fn agent_records(&self) -> Vec<AgentRecord> {
        self.agents
            .values()
            .map(|hosted| AgentRecord {
                manifest: hosted.manifest.clone(),
                status: hosted.agent.status(),
            })
            .collect()
    }

    #[must_use]
    pub fn describe_agent(&self, name: &str) -> Option<AgentRecord> {
        self.agents.get(name).map(|hosted| AgentRecord {
            manifest: hosted.manifest.clone(),
            status: hosted.agent.status(),
        })
    }

    pub fn send_message(
        &mut self,
        sender: &str,
        recipient: &str,
        capability: &str,
        payload: &[u8],
    ) -> Result<Option<Vec<u8>>, DemoError> {
        self.authorize(sender, capability)?;

        let envelope = MessageEnvelope::new(
            sender,
            recipient,
            Capability::new(capability)?,
            payload.to_vec(),
            self.allocate_nonce(),
        )?;

        self.executor
            .submit(envelope)
            .map_err(Self::map_infallible_executor)?;
        let Some(queued) = self
            .executor
            .pop_next()
            .map_err(Self::map_infallible_executor)?
        else {
            return Ok(None);
        };

        self.bus.send(queued).map_err(Self::map_infallible_bus)?;
        let Some(inbound) = self
            .bus
            .try_recv(recipient)
            .map_err(Self::map_infallible_bus)?
        else {
            return Ok(None);
        };

        let response_payload = {
            let hosted = self
                .agents
                .get_mut(recipient)
                .ok_or_else(|| DemoError::UnknownAgent(recipient.to_owned()))?;
            hosted.agent.handle_message(&inbound)?
        };

        let Some(response_payload) = response_payload else {
            return Ok(None);
        };

        let response = MessageEnvelope::new(
            recipient,
            sender,
            inbound.capability().clone(),
            response_payload,
            self.allocate_nonce(),
        )?;
        self.bus.send(response).map_err(Self::map_infallible_bus)?;

        Ok(self
            .bus
            .try_recv(sender)
            .map_err(Self::map_infallible_bus)?
            .map(|envelope| envelope.payload().to_vec()))
    }

    pub fn poll_agent(&mut self, name: &str) -> Result<Vec<Vec<u8>>, DemoError> {
        let capability = {
            let hosted = self
                .agents
                .get(name)
                .ok_or_else(|| DemoError::UnknownAgent(name.to_owned()))?;
            hosted
                .manifest
                .required_capabilities
                .first()
                .map(|requirement| requirement.capability.clone())
                .ok_or_else(|| DemoError::ManifestMissingCapabilities(name.to_owned()))?
        };

        self.authorize(name, &capability)?;

        let hosted = self
            .agents
            .get_mut(name)
            .ok_or_else(|| DemoError::UnknownAgent(name.to_owned()))?;
        hosted.agent.poll().map_err(Into::into)
    }

    pub fn run_reference_demo_cycle(&mut self) -> Result<DemoSummary, DemoError> {
        self.start_agent("echo")?;
        self.start_agent("timer")?;

        let echo_response = self
            .send_message("echo", "echo", "agent.echo", b"hello")?
            .ok_or(DemoError::MissingEchoResponse)?;

        let denied_requests = match self.send_message("echo", "echo", "agent.admin", b"nope") {
            Ok(_) => 0,
            Err(DemoError::AuthorizationDenied(_)) => 1,
            Err(error) => return Err(error),
        };

        let timer_event = self
            .poll_agent("timer")?
            .into_iter()
            .next()
            .ok_or(DemoError::MissingTimerEvent)?;

        self.stop_agent("echo")?;
        self.stop_agent("timer")?;

        Ok(DemoSummary {
            started_agents: self.list_agents(),
            echo_response: String::from_utf8_lossy(&echo_response).into_owned(),
            timer_event: String::from_utf8_lossy(&timer_event).into_owned(),
            denied_requests,
            log_entries: self.log_entries().to_vec(),
        })
    }

    #[must_use]
    pub fn log_entries(&self) -> &[String] {
        &self.log_entries
    }

    fn authorize(&mut self, name: &str, capability: &str) -> Result<(), DemoError> {
        let requester_profile_id = {
            let hosted = self
                .agents
                .get(name)
                .ok_or_else(|| DemoError::UnknownAgent(name.to_owned()))?;
            hosted
                .manifest
                .required_capabilities
                .first()
                .map(|requirement| requirement.profile_id.as_str().to_owned())
                .ok_or_else(|| DemoError::ManifestMissingCapabilities(name.to_owned()))?
        };

        let decision = self.evaluate_policy(&requester_profile_id, capability)?;

        if decision == PolicyDecision::Allowed {
            return Ok(());
        }

        let summary = format!("{name}:{capability}:{decision:?}");
        self.log_entries.push(format!("denied:{summary}"));
        Err(DemoError::AuthorizationDenied(
            AuthorizationDenyDetail::from_summary(summary),
        ))
    }

    fn evaluate_policy(
        &self,
        requester_profile_id: &str,
        capability: &str,
    ) -> Result<PolicyDecision, DemoError> {
        let request = CapabilityRequest::new(requester_profile_id, Capability::new(capability)?)?;
        let grants = self
            .grants
            .iter()
            .map(|grant| ActiveGrantView { grant })
            .collect::<Vec<_>>();

        Ok(self.policy.evaluate(&request, &grants))
    }

    fn allocate_nonce(&mut self) -> u64 {
        let nonce = self.next_nonce;
        self.next_nonce += 1;
        nonce
    }

    fn replay_cli_state(&mut self, state: &CliState) -> Result<(), CliError> {
        for (name, status) in &state.agent_statuses {
            match status {
                AgentStatus::Registered => {}
                AgentStatus::Running => self.start_agent_internal(name, false)?,
                AgentStatus::Stopped => self.stop_agent_internal(name, false)?,
                _ => {
                    return Err(CliError::InvalidState(format!(
                        "unsupported persisted status for {name}: {}",
                        format_agent_status(*status)
                    )));
                }
            }
        }

        Ok(())
    }

    fn map_infallible_executor(error: Infallible) -> DemoError {
        match error {}
    }

    fn map_infallible_bus(error: Infallible) -> DemoError {
        match error {}
    }
}

pub fn build_reference_runtime() -> Result<DemoRuntime, DemoError> {
    DemoRuntime::reference_host()
}

pub fn execute_local_agent_api(
    command: LocalAgentApiCommand,
) -> Result<LocalAgentApiResponse, CliError> {
    LocalAgentApi::new().execute(command)
}

pub fn execute_agent_cli(command: AgentCliCommand) -> Result<Vec<String>, CliError> {
    execute_agent_cli_with_state_path(command, &cli_state_path())
}

pub fn execute_profile_cli(command: ProfileCliCommand) -> Result<Vec<String>, CliError> {
    execute_profile_cli_with_store(command, &FileSystemProfileStore)
}

pub fn execute_agent_read_rpc(
    request: AgentJsonRpcRequest,
) -> Result<AgentJsonRpcResponse, CliError> {
    execute_agent_read_rpc_with_store_and_paths(
        request,
        &cli_state_path(),
        &default_profile_path(),
        &FileSystemProfileStore,
    )
}

pub fn execute_agent_read_rpc_json(request_json: &str) -> Result<String, CliError> {
    let request: AgentJsonRpcRequest = serde_json::from_str(request_json)
        .map_err(|error| CliError::InvalidState(format!("invalid JSON-RPC request: {error}")))?;
    let response = execute_agent_read_rpc(request)?;

    serde_json::to_string_pretty(&response).map_err(|error| {
        CliError::InvalidState(format!("failed to serialize JSON-RPC response: {error}"))
    })
}

#[must_use]
pub fn local_shell_default_port() -> u16 {
    LOCAL_SHELL_DEFAULT_PORT
}

#[must_use]
pub fn local_shell_url(port: u16) -> String {
    format!("http://127.0.0.1:{port}/")
}

// BRIDGE-WORKAROUND: LAN bind — pre-auth, superseded when native bridge lands
pub fn serve_local_shell(port: u16) -> io::Result<()> {
    serve_local_shell_with_bind(port, "127.0.0.1")
}

// BRIDGE-WORKAROUND: LAN bind — pre-auth, superseded when native bridge lands
pub fn serve_local_shell_with_bind(port: u16, bind_addr: &str) -> io::Result<()> {
    let listener = TcpListener::bind((bind_addr, port))?;

    serve_local_shell_with_listener(listener, None)
}

fn serve_local_shell_with_listener(
    listener: TcpListener,
    max_connections: Option<usize>,
) -> io::Result<()> {
    serve_local_shell_with_store_and_paths(
        listener,
        max_connections,
        &cli_state_path(),
        &default_profile_path(),
        &FileSystemProfileStore,
    )
}

fn serve_local_shell_with_store_and_paths<S: LocalProfileStore>(
    listener: TcpListener,
    max_connections: Option<usize>,
    state_path: &Path,
    default_profile_path: &Path,
    store: &S,
) -> io::Result<()> {
    for (handled_connections, incoming) in listener.incoming().enumerate() {
        let mut stream = incoming?;

        if let Err(error) = handle_shell_connection_with_store_and_paths(
            &mut stream,
            state_path,
            default_profile_path,
            store,
        ) {
            let response = text_response(
                500,
                "Internal Server Error",
                format!("FERROS shell server error: {error}"),
            );
            let _ = write_http_response(&mut stream, response);
        }

        if max_connections.is_some_and(|limit| handled_connections + 1 >= limit) {
            break;
        }
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HttpRequest {
    method: String,
    path: String,
    body: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HttpResponse {
    status_code: u16,
    status_text: &'static str,
    content_type: &'static str,
    body: Vec<u8>,
}

fn handle_shell_connection_with_store_and_paths<S: LocalProfileStore>(
    stream: &mut TcpStream,
    state_path: &Path,
    default_profile_path: &Path,
    store: &S,
) -> io::Result<()> {
    let Some(request) = read_http_request(stream)? else {
        return Ok(());
    };
    let response =
        route_shell_request_with_store_and_paths(request, state_path, default_profile_path, store);
    write_http_response(stream, response)
}

fn route_shell_request_with_store_and_paths<S: LocalProfileStore>(
    request: HttpRequest,
    state_path: &Path,
    default_profile_path: &Path,
    store: &S,
) -> HttpResponse {
    if request.method == "OPTIONS" {
        return text_response(200, "OK", "");
    }
    let (request_path, request_query) = split_request_path(&request.path);

    if let Some(response) =
        route_monitor_request(request.method.as_str(), request_path, request.body.clone())
    {
        return response;
    }

    match (request.method.as_str(), request_path) {
        ("GET", "/") | ("GET", "/index.html") => HttpResponse {
            status_code: 200,
            status_text: "OK",
            content_type: "text/html; charset=utf-8",
            body: LOCAL_SHELL_HTML.as_bytes().to_vec(),
        },
        ("GET", "/harnesses/localhost-shell-acceptance.html") => HttpResponse {
            status_code: 200,
            status_text: "OK",
            content_type: "text/html; charset=utf-8",
            body: LOCAL_SHELL_ACCEPTANCE_HARNESS_HTML.as_bytes().to_vec(),
        },
        ("GET", "/runway-summary") | ("GET", "/runway-summary.json") => {
            let requested_profile_path =
                requested_profile_path_from_query(request_query, default_profile_path);
            route_shell_runway_summary_request(state_path, &requested_profile_path, store)
        }
        ("POST", "/rpc") => {
            route_shell_rpc_request(request.body, state_path, default_profile_path, store)
        }
        ("POST", "/profile") => {
            route_shell_profile_request(request.body, default_profile_path, store)
        }
        _ => text_response(404, "Not Found", "FERROS local shell route not found"),
    }
}

fn enable_cors(response: HttpResponse) -> HttpResponse {
    response
}

fn route_monitor_request(method: &str, request_path: &str, body: Vec<u8>) -> Option<HttpResponse> {
    if request_path.starts_with("/monitor") && method == "OPTIONS" {
        return Some(enable_cors(text_response(200, "OK", "")));
    }

    if request_path == "/monitor/state" && method == "GET" {
        let state = monitor_state();
        let guard = state.lock().map_err(|_| ()).ok()?;
        let snapshot = guard.snapshot();
        return Some(enable_cors(json_response(200, "OK", &snapshot)));
    }

    if request_path == "/monitor/events" && method == "GET" {
        let state = monitor_state();
        let guard = state.lock().map_err(|_| ()).ok()?;
        return Some(enable_cors(json_response(200, "OK", &guard.timeline)));
    }

    if request_path == "/monitor/lifecycle" && method == "GET" {
        let state = monitor_state();
        let guard = state.lock().map_err(|_| ()).ok()?;
        return Some(enable_cors(json_response(200, "OK", &guard.lifecycle_threads)));
    }

    if request_path == "/monitor/agent-directory" && method == "GET" {
        let directory = load_monitor_agent_directory();
        return Some(enable_cors(json_response(200, "OK", &directory)));
    }

    if request_path == "/monitor/sessions" && method == "GET" {
        let state = monitor_state();
        let guard = state.lock().map_err(|_| ()).ok()?;
        return Some(enable_cors(json_response(200, "OK", &guard.open_chats)));
    }

    if request_path == "/monitor/sessions" && method == "POST" {
        let request: MonitorCreateSessionRequest = match serde_json::from_slice(&body) {
            Ok(parsed) => parsed,
            Err(error) => {
                return Some(enable_cors(text_response(
                    400,
                    "Bad Request",
                    format!("invalid monitor session payload: {error}"),
                )));
            }
        };

        let state = monitor_state();
        let mut guard = match state.lock() {
            Ok(guard) => guard,
            Err(_) => {
                return Some(text_response(
                    500,
                    "Internal Server Error",
                    "monitor state lock poisoned",
                ));
            }
        };
        let session = guard.create_session(request.label);
        persist_monitor_state_best_effort(&mut guard, "session.create persistence warning");
        return Some(enable_cors(json_response(200, "OK", &session)));
    }

    if request_path == "/monitor/loops/transition" && method == "POST" {
        let request: MonitorLoopTransitionRequest = match serde_json::from_slice(&body) {
            Ok(parsed) => parsed,
            Err(error) => {
                return Some(enable_cors(text_response(
                    400,
                    "Bad Request",
                    format!("invalid monitor loop payload: {error}"),
                )));
            }
        };

        let state = monitor_state();
        let mut guard = match state.lock() {
            Ok(guard) => guard,
            Err(_) => {
                return Some(enable_cors(text_response(
                    500,
                    "Internal Server Error",
                    "monitor state lock poisoned",
                )));
            }
        };
        guard.apply_loop_transition(&request.action);
        persist_monitor_state_best_effort(&mut guard, "loop.transition persistence warning");
        return Some(enable_cors(json_response(200, "OK", &guard.snapshot())));
    }

    if let Some((thread_id, trailing)) = parse_monitor_lifecycle_subroute(request_path) {
        if trailing == "messages" && method == "POST" {
            let request: MonitorLifecycleMessageRequest = match serde_json::from_slice(&body) {
                Ok(parsed) => parsed,
                Err(error) => {
                    return Some(enable_cors(text_response(
                        400,
                        "Bad Request",
                        format!("invalid lifecycle message payload: {error}"),
                    )));
                }
            };

            let state = monitor_state();
            let mut guard = match state.lock() {
                Ok(guard) => guard,
                Err(_) => {
                    return Some(enable_cors(text_response(
                        500,
                        "Internal Server Error",
                        "monitor state lock poisoned",
                    )));
                }
            };

            if !guard.add_lifecycle_message(thread_id, request) {
                return Some(enable_cors(text_response(404, "Not Found", "monitor lifecycle thread not found")));
            }

            persist_monitor_state_best_effort(&mut guard, "lifecycle.message persistence warning");

            return Some(enable_cors(json_response(200, "OK", &guard.snapshot())));
        }
    }

    if let Some((notification_id, trailing)) = parse_monitor_notification_subroute(request_path) {
        if method == "POST" {
            let state = monitor_state();
            let mut guard = match state.lock() {
                Ok(guard) => guard,
                Err(_) => {
                    return Some(enable_cors(text_response(
                        500,
                        "Internal Server Error",
                        "monitor state lock poisoned",
                    )));
                }
            };

            if !matches!(trailing, "open" | "opened" | "resolve") {
                return Some(enable_cors(text_response(
                    400,
                    "Bad Request",
                    format!("unknown notification action: {trailing}"),
                )));
            }

            let updated = match trailing {
                "open" => guard.open_notification(notification_id),
                "opened" => guard.update_notification_status(notification_id, "opened"),
                "resolve" => guard.update_notification_status(notification_id, "resolved"),
                _ => unreachable!(),
            };

            if !updated {
                return Some(enable_cors(text_response(
                    404,
                    "Not Found",
                    "monitor notification not found",
                )));
            }

            persist_monitor_state_best_effort(&mut guard, "notification.action persistence warning");
            return Some(enable_cors(json_response(200, "OK", &guard.snapshot())));
        }
    }

    let Some((session_id, trailing)) = parse_monitor_session_subroute(request_path) else {
        return None;
    };

    if trailing == "messages" && method == "POST" {
        let request: MonitorMessageRequest = match serde_json::from_slice(&body) {
            Ok(parsed) => parsed,
            Err(error) => {
                return Some(enable_cors(text_response(
                    400,
                    "Bad Request",
                    format!("invalid monitor message payload: {error}"),
                )));
            }
        };

        let state = monitor_state();
        let mut guard = match state.lock() {
            Ok(guard) => guard,
            Err(_) => {
                return Some(enable_cors(text_response(
                    500,
                    "Internal Server Error",
                    "monitor state lock poisoned",
                )));
            }
        };

        let message_id = match guard.add_message(session_id, request.clone()) {
            Some(message_id) => message_id,
            None => {
                return Some(enable_cors(text_response(404, "Not Found", "monitor session not found")));
            }
        };

        if request.speaker.eq_ignore_ascii_case("user")
            && guard.session_active_agent(session_id).is_some_and(|agent| agent == "FERROS Agent")
        {
            let _ = guard.ferros_agent_handle_human_message(session_id, &message_id, &request.text);
        }

        persist_monitor_state_best_effort(&mut guard, "session.message persistence warning");

        return Some(enable_cors(json_response(200, "OK", &guard.snapshot())));
    }

    if trailing == "route" && method == "POST" {
        let request: MonitorRouteRequest = match serde_json::from_slice(&body) {
            Ok(parsed) => parsed,
            Err(error) => {
                return Some(enable_cors(text_response(
                    400,
                    "Bad Request",
                    format!("invalid monitor route payload: {error}"),
                )));
            }
        };

        let state = monitor_state();
        let mut guard = match state.lock() {
            Ok(guard) => guard,
            Err(_) => {
                return Some(enable_cors(text_response(
                    500,
                    "Internal Server Error",
                    "monitor state lock poisoned",
                )));
            }
        };
        if !guard.route_session(session_id, &request.target) {
            return Some(enable_cors(text_response(404, "Not Found", "monitor session not found")));
        }
        persist_monitor_state_best_effort(&mut guard, "session.route persistence warning");
        return Some(enable_cors(json_response(200, "OK", &guard.snapshot())));
    }

    if trailing == "archive" && method == "POST" {
        let state = monitor_state();
        let mut guard = match state.lock() {
            Ok(guard) => guard,
            Err(_) => {
                return Some(enable_cors(text_response(
                    500,
                    "Internal Server Error",
                    "monitor state lock poisoned",
                )));
            }
        };
        if !guard.archive_session(session_id) {
            return Some(enable_cors(text_response(404, "Not Found", "monitor session not found")));
        }
        persist_monitor_state_best_effort(&mut guard, "session.archive persistence warning");
        return Some(enable_cors(json_response(200, "OK", &guard.snapshot())));
    }

    None
}

fn parse_monitor_session_subroute(path: &str) -> Option<(&str, &str)> {
    let prefix = "/monitor/sessions/";
    if !path.starts_with(prefix) {
        return None;
    }

    let tail = &path[prefix.len()..];
    let (session_id, subroute) = tail.split_once('/')?;
    if session_id.is_empty() || subroute.is_empty() {
        return None;
    }

    Some((session_id, subroute))
}

fn parse_monitor_lifecycle_subroute(path: &str) -> Option<(&str, &str)> {
    let prefix = "/monitor/lifecycle/";
    if !path.starts_with(prefix) {
        return None;
    }

    let tail = &path[prefix.len()..];
    let (thread_id, subroute) = tail.split_once('/')?;
    if thread_id.is_empty() || subroute.is_empty() {
        return None;
    }

    Some((thread_id, subroute))
}

fn parse_monitor_notification_subroute(path: &str) -> Option<(&str, &str)> {
    let prefix = "/monitor/notifications/";
    if !path.starts_with(prefix) {
        return None;
    }

    let tail = &path[prefix.len()..];
    let (notification_id, subroute) = tail.split_once('/')?;
    if notification_id.is_empty() || subroute.is_empty() {
        return None;
    }

    Some((notification_id, subroute))
}

/// Test-only: route monitor requests against a caller-supplied `Mutex<MonitorState>` so
/// tests can use isolated state instead of the global singleton.
#[cfg(test)]
fn route_monitor_request_with_state(
    method: &str,
    request_path: &str,
    body: Vec<u8>,
    state: &std::sync::Mutex<MonitorState>,
) -> Option<HttpResponse> {
    if request_path == "/monitor/sessions" && method == "POST" {
        let request: MonitorCreateSessionRequest = match serde_json::from_slice(&body) {
            Ok(r) => r,
            Err(e) => {
                return Some(enable_cors(text_response(
                    400,
                    "Bad Request",
                    format!("invalid monitor session payload: {e}"),
                )));
            }
        };
        let mut guard = state.lock().ok()?;
        let session = guard.create_session(request.label);
        return Some(enable_cors(json_response(200, "OK", &session)));
    }

    if let Some((session_id, trailing)) = parse_monitor_session_subroute(request_path) {
        if trailing == "messages" && method == "POST" {
            let request: MonitorMessageRequest = match serde_json::from_slice(&body) {
                Ok(r) => r,
                Err(e) => {
                    return Some(enable_cors(text_response(
                        400,
                        "Bad Request",
                        format!("invalid monitor message payload: {e}"),
                    )));
                }
            };
            let mut guard = state.lock().ok()?;
            let message_id = match guard.add_message(session_id, request.clone()) {
                Some(id) => id,
                None => {
                    return Some(enable_cors(text_response(
                        404,
                        "Not Found",
                        "monitor session not found",
                    )));
                }
            };
            if request.speaker.eq_ignore_ascii_case("user")
                && guard
                    .session_active_agent(session_id)
                    .is_some_and(|agent| agent == "FERROS Agent")
            {
                let _ =
                    guard.ferros_agent_handle_human_message(session_id, &message_id, &request.text);
            }
            return Some(enable_cors(json_response(200, "OK", &guard.snapshot())));
        }
    }

    if let Some((notification_id, trailing)) = parse_monitor_notification_subroute(request_path) {
        if method == "POST" {
            if !matches!(trailing, "open" | "opened" | "resolve") {
                return Some(enable_cors(text_response(
                    400,
                    "Bad Request",
                    format!("unknown notification action: {trailing}"),
                )));
            }
            let mut guard = state.lock().ok()?;
            let updated = match trailing {
                "open" => guard.open_notification(notification_id),
                "opened" => guard.update_notification_status(notification_id, "opened"),
                "resolve" => guard.update_notification_status(notification_id, "resolved"),
                _ => unreachable!(),
            };
            if !updated {
                return Some(enable_cors(text_response(
                    404,
                    "Not Found",
                    "monitor notification not found",
                )));
            }
            return Some(enable_cors(json_response(200, "OK", &guard.snapshot())));
        }
    }

    None
}

fn split_request_path(path: &str) -> (&str, Option<&str>) {
    match path.split_once('?') {
        Some((request_path, request_query)) => (request_path, Some(request_query)),
        None => (path, None),
    }
}

fn requested_profile_path_from_query(
    request_query: Option<&str>,
    default_profile_path: &Path,
) -> PathBuf {
    request_query
        .and_then(|query| query.split('&').find_map(query_param_profile_path))
        .map(PathBuf::from)
        .unwrap_or_else(|| default_profile_path.to_path_buf())
}

fn query_param_profile_path(segment: &str) -> Option<String> {
    let (key, value) = segment.split_once('=')?;
    if key != "profilePath" {
        return None;
    }

    decode_query_component(value).ok()
}

fn decode_query_component(value: &str) -> Result<String, ()> {
    let mut bytes = Vec::with_capacity(value.len());
    let raw = value.as_bytes();
    let mut index = 0;

    while index < raw.len() {
        match raw[index] {
            b'%' => {
                if index + 2 >= raw.len() {
                    return Err(());
                }

                let high = decode_query_hex(raw[index + 1])?;
                let low = decode_query_hex(raw[index + 2])?;
                bytes.push((high << 4) | low);
                index += 3;
            }
            b'+' => {
                bytes.push(b' ');
                index += 1;
            }
            byte => {
                bytes.push(byte);
                index += 1;
            }
        }
    }

    String::from_utf8(bytes).map_err(|_| ())
}

fn decode_query_hex(value: u8) -> Result<u8, ()> {
    match value {
        b'0'..=b'9' => Ok(value - b'0'),
        b'a'..=b'f' => Ok(value - b'a' + 10),
        b'A'..=b'F' => Ok(value - b'A' + 10),
        _ => Err(()),
    }
}

fn route_shell_profile_request<S: LocalProfileStore>(
    body: Vec<u8>,
    default_profile_path: &Path,
    store: &S,
) -> HttpResponse {
    let request_json = match String::from_utf8(body) {
        Ok(request_json) => request_json,
        Err(_) => {
            return text_response(400, "Bad Request", "request body must be valid UTF-8");
        }
    };

    let request: ProfileShellRequest = match serde_json::from_str(&request_json) {
        Ok(request) => request,
        Err(error) => {
            return text_response(
                400,
                "Bad Request",
                format!("invalid profile request: {error}"),
            );
        }
    };

    let response = execute_profile_shell_request_with_store(request, default_profile_path, store);
    match serde_json::to_string_pretty(&response) {
        Ok(body) => HttpResponse {
            status_code: 200,
            status_text: "OK",
            content_type: "application/json; charset=utf-8",
            body: body.into_bytes(),
        },
        Err(error) => text_response(
            500,
            "Internal Server Error",
            format!("failed to serialize profile response: {error}"),
        ),
    }
}

fn route_shell_runway_summary_request<S: LocalProfileStore>(
    state_path: &Path,
    default_profile_path: &Path,
    store: &S,
) -> HttpResponse {
    match LocalAgentApi::at_state_path(state_path)
        .runway_summary_with_store_and_profile_path(default_profile_path, store)
    {
        Ok(summary) => match serde_json::to_string_pretty(&summary) {
            Ok(body) => HttpResponse {
                status_code: 200,
                status_text: "OK",
                content_type: "application/json; charset=utf-8",
                body: body.into_bytes(),
            },
            Err(error) => text_response(
                500,
                "Internal Server Error",
                format!("failed to serialize runway summary: {error}"),
            ),
        },
        Err(error) => text_response(
            500,
            "Internal Server Error",
            format!("failed to build runway summary: {error}"),
        ),
    }
}

fn route_shell_rpc_request<S: LocalProfileStore>(
    body: Vec<u8>,
    state_path: &Path,
    default_profile_path: &Path,
    store: &S,
) -> HttpResponse {
    let request_json = match String::from_utf8(body) {
        Ok(request_json) => request_json,
        Err(_) => {
            return text_response(400, "Bad Request", "request body must be valid UTF-8");
        }
    };

    let request: AgentJsonRpcRequest = match serde_json::from_str(&request_json) {
        Ok(request) => request,
        Err(error) => {
            return text_response(
                400,
                "Bad Request",
                format!("invalid JSON-RPC request: {error}"),
            );
        }
    };

    match execute_agent_read_rpc_with_store_and_paths(
        request,
        state_path,
        default_profile_path,
        store,
    ) {
        Ok(response) => match serde_json::to_string_pretty(&response) {
            Ok(body) => HttpResponse {
                status_code: 200,
                status_text: "OK",
                content_type: "application/json; charset=utf-8",
                body: body.into_bytes(),
            },
            Err(error) => text_response(
                500,
                "Internal Server Error",
                format!("failed to serialize JSON-RPC response: {error}"),
            ),
        },
        Err(error) => text_response(500, "Internal Server Error", error.to_string()),
    }
}

fn read_http_request(stream: &mut TcpStream) -> io::Result<Option<HttpRequest>> {
    let mut buffer = Vec::new();
    let mut chunk = [0_u8; 1024];
    let mut header_end = None;
    let mut content_length = 0_usize;

    loop {
        let bytes_read = stream.read(&mut chunk)?;
        if bytes_read == 0 {
            if buffer.is_empty() {
                return Ok(None);
            }
            break;
        }

        buffer.extend_from_slice(&chunk[..bytes_read]);

        if buffer.len() > MAX_HTTP_REQUEST_BYTES {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "HTTP request exceeded FERROS local shell limit",
            ));
        }

        if header_end.is_none() {
            if let Some(index) = find_http_header_end(&buffer) {
                header_end = Some(index);
                content_length = parse_content_length(&buffer[..index])?;
            }
        }

        if let Some(index) = header_end {
            let expected_len = index + 4 + content_length;
            if buffer.len() >= expected_len {
                return parse_http_request(&buffer[..expected_len]).map(Some);
            }
        }
    }

    if let Some(index) = header_end {
        let expected_len = index + 4 + content_length;
        if buffer.len() >= expected_len {
            return parse_http_request(&buffer[..expected_len]).map(Some);
        }
    }

    Err(io::Error::new(
        io::ErrorKind::UnexpectedEof,
        "incomplete HTTP request for FERROS local shell",
    ))
}

fn parse_http_request(bytes: &[u8]) -> io::Result<HttpRequest> {
    let Some(header_end) = find_http_header_end(bytes) else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "missing HTTP header terminator",
        ));
    };

    let header_text = std::str::from_utf8(&bytes[..header_end])
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "HTTP headers must be UTF-8"))?;
    let mut lines = header_text.lines();
    let request_line = lines
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "missing HTTP request line"))?;
    let mut parts = request_line.split_whitespace();
    let method = parts
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "missing HTTP method"))?;
    let path = parts
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "missing HTTP path"))?;

    Ok(HttpRequest {
        method: method.to_owned(),
        path: path.to_owned(),
        body: bytes[header_end + 4..].to_vec(),
    })
}

fn parse_content_length(header_bytes: &[u8]) -> io::Result<usize> {
    let header_text = std::str::from_utf8(header_bytes)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "HTTP headers must be UTF-8"))?;

    for line in header_text.lines() {
        if let Some(value) = line.strip_prefix("Content-Length:") {
            return value.trim().parse::<usize>().map_err(|_| {
                io::Error::new(io::ErrorKind::InvalidData, "invalid Content-Length header")
            });
        }

        if let Some(value) = line.strip_prefix("content-length:") {
            return value.trim().parse::<usize>().map_err(|_| {
                io::Error::new(io::ErrorKind::InvalidData, "invalid Content-Length header")
            });
        }
    }

    Ok(0)
}

fn find_http_header_end(bytes: &[u8]) -> Option<usize> {
    bytes.windows(4).position(|window| window == b"\r\n\r\n")
}

fn write_http_response(stream: &mut TcpStream, response: HttpResponse) -> io::Result<()> {
    let mut header = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\nCache-Control: no-store",
        response.status_code,
        response.status_text,
        response.content_type,
        response.body.len()
    );

    header.push_str("\r\nAccess-Control-Allow-Origin: *");
    header.push_str("\r\nAccess-Control-Allow-Methods: GET, POST, OPTIONS");
    header.push_str("\r\nAccess-Control-Allow-Headers: Content-Type");

    header.push_str("\r\n\r\n");

    stream.write_all(header.as_bytes())?;
    stream.write_all(&response.body)?;
    stream.flush()
}

fn text_response(
    status_code: u16,
    status_text: &'static str,
    message: impl Into<String>,
) -> HttpResponse {
    HttpResponse {
        status_code,
        status_text,
        content_type: "text/plain; charset=utf-8",
        body: message.into().into_bytes(),
    }
}

fn json_response<T: Serialize>(status_code: u16, status_text: &'static str, payload: &T) -> HttpResponse {
    match serde_json::to_string_pretty(payload) {
        Ok(body) => HttpResponse {
            status_code,
            status_text,
            content_type: "application/json; charset=utf-8",
            body: body.into_bytes(),
        },
        Err(error) => text_response(
            500,
            "Internal Server Error",
            format!("failed to serialize monitor response: {error}"),
        ),
    }
}

fn monitor_state() -> &'static Mutex<MonitorState> {
    MONITOR_STATE.get_or_init(|| Mutex::new(load_monitor_state().unwrap_or_default()))
}

fn load_monitor_state() -> Option<MonitorState> {
    let path = monitor_state_path();
    let bytes = fs::read(&path).ok()?;
    let persisted: PersistedMonitorState = serde_json::from_slice(&bytes).ok()?;
    if persisted.schema_version != MONITOR_STATE_SCHEMA_VERSION {
        return None;
    }

    let mut state = persisted.state;
    normalize_monitor_state(&mut state);
    Some(state)
}

fn persist_monitor_state(state: &MonitorState) -> io::Result<()> {
    let path = monitor_state_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let payload = PersistedMonitorState {
        schema_version: MONITOR_STATE_SCHEMA_VERSION,
        state: state.clone(),
    };
    let bytes = serde_json::to_vec_pretty(&payload)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    fs::write(path, bytes)
}

fn persist_monitor_state_best_effort(state: &mut MonitorState, context: &str) {
    if let Err(error) = persist_monitor_state(state) {
        state.push_event("monitor.persistence.warning", format!("{context}: {error}"));
    }
}

fn normalize_monitor_state(state: &mut MonitorState) {
    for session in &mut state.open_chats {
        for message in &mut session.messages {
            if message.id.is_empty() {
                message.id = format!("msg-{}", state.next_id + 1);
                state.next_id += 1;
            }
        }
    }

    for archive in &mut state.archived_chats {
        for message in &mut archive.preview {
            if message.id.is_empty() {
                message.id = format!("msg-{}", state.next_id + 1);
                state.next_id += 1;
            }
        }
    }

    for notification in &mut state.notifications {
        if notification.id.is_empty() {
            notification.id = format!("ntf-{}", state.next_id + 1);
            state.next_id += 1;
        }
    }

    for packet in &mut state.packets {
        if packet.id.is_empty() {
            packet.id = format!("pkt-{}", state.next_id + 1);
            state.next_id += 1;
        }
    }
}

fn monitor_now() -> String {
    match OffsetDateTime::now_utc().format(&Rfc3339) {
        Ok(value) => value,
        Err(_) => "1970-01-01T00:00:00Z".to_owned(),
    }
}

fn monitor_at_plus_seconds(seconds: i64) -> Option<String> {
    let timestamp = OffsetDateTime::now_utc() + time::Duration::seconds(seconds);
    timestamp.format(&Rfc3339).ok()
}

fn monitor_category_for_agent(agent: &str) -> String {
    let normalized = agent.to_ascii_lowercase();
    if normalized.contains("business")
        || normalized.contains("business manager")
        || normalized.contains("operations")
        || normalized.contains("sales")
        || normalized.contains("finance")
        || normalized.contains("hr")
    {
        return "business".to_owned();
    }

    if normalized.contains("software architect")
        || normalized.contains("core")
        || normalized.contains("subcore")
        || normalized.contains("coding")
    {
        return "software".to_owned();
    }

    if normalized.contains("architect") {
        return "architect".to_owned();
    }

    if normalized.contains("ferros") || normalized.contains("escalation") {
        return "administration".to_owned();
    }

    "service".to_owned()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DispatchTarget {
    Software,
    Business,
    FerrosArchitect,
    CodingArchitect,
    BusinessArchitect,
}

fn infer_dispatch_target(operator_text: &str) -> DispatchTarget {
    let lowered = operator_text.to_ascii_lowercase();

    if lowered.contains("business architect") {
        return DispatchTarget::BusinessArchitect;
    }

    if lowered.contains("coding architect") {
        return DispatchTarget::CodingArchitect;
    }

    if lowered.contains("ferros architect") || lowered.contains("agent architect") {
        return DispatchTarget::FerrosArchitect;
    }

    if lowered.contains("business") || lowered.contains("sales") || lowered.contains("finance") {
        return DispatchTarget::Business;
    }

    DispatchTarget::Software
}

fn monitor_status_detail_for(agent: &str, state: &str, description: &str) -> (String, String, String, Option<u8>) {
    let category = monitor_category_for_agent(agent);
    match state {
        "running" => (
            "running".to_owned(),
            "work-order-active".to_owned(),
            description.to_owned(),
            Some(48),
        ),
        "waiting" => {
            let status = if category == "administration" || agent.to_ascii_lowercase().contains("escalation") {
                "attention"
            } else {
                "escalating"
            };
            (
                status.to_owned(),
                "awaiting-upstream-or-operator".to_owned(),
                description.to_owned(),
                Some(if status == "attention" { 82 } else { 62 }),
            )
        }
        "stopped" | "failed" => (
            "stopped".to_owned(),
            "stopped-before-closure".to_owned(),
            description.to_owned(),
            Some(18),
        ),
        _ => (
            "idle".to_owned(),
            "no-active-cycle".to_owned(),
            description.to_owned(),
            Some(10),
        ),
    }
}

fn monitor_loop_is_stale(loop_state: &MonitorLoop) -> bool {
    let Some(stale_after) = loop_state.stale_after.as_deref() else {
        return false;
    };

    let Ok(stale_after) = OffsetDateTime::parse(stale_after, &Rfc3339) else {
        return false;
    };

    OffsetDateTime::now_utc() > stale_after
}

fn normalize_monitor_loop(loop_state: &MonitorLoop) -> MonitorLoop {
    let mut normalized = loop_state.clone();
    if monitor_loop_is_stale(loop_state)
        && normalized.status != "stopped"
        && normalized.status != "attention"
    {
        normalized.status = "stalled".to_owned();
        normalized.status_reason = "stale-timeout".to_owned();
        normalized.status_detail = format!("No lifecycle update since {}", normalized.updated_at);
        normalized.progress = Some(76);
    }

    normalized
}

fn parse_agent_frontmatter(content: &str) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    let mut lines = content.lines();
    if lines.next() != Some("---") {
        return map;
    }

    let mut current_key: Option<String> = None;
    let mut list_accumulator: Vec<String> = Vec::new();
    for line in lines {
        if line.trim() == "---" {
            if let Some(key) = current_key.take() {
                map.insert(key, list_accumulator.join("\n"));
            }
            break;
        }

        let trimmed = line.trim_end();
        if let Some((key, value)) = trimmed.split_once(':') {
            if let Some(previous_key) = current_key.take() {
                map.insert(previous_key, list_accumulator.join("\n"));
                list_accumulator.clear();
            }

            let key = key.trim().to_owned();
            let value = value.trim();
            if value.is_empty() {
                current_key = Some(key);
            } else {
                map.insert(key, value.to_owned());
            }
            continue;
        }

        if trimmed.trim_start().starts_with('-') {
            list_accumulator.push(trimmed.trim().trim_start_matches('-').trim().to_owned());
        }
    }

    map
}

fn parse_frontmatter_list(value: Option<&String>) -> Vec<String> {
    let Some(value) = value else {
        return Vec::new();
    };

    let trimmed = value.trim();
    if trimmed.starts_with('[') && trimmed.ends_with(']') {
        return trimmed[1..trimmed.len() - 1]
            .split(',')
            .map(|item| item.trim().trim_matches('"').trim_matches('\''))
            .filter(|item| !item.is_empty())
            .map(str::to_owned)
            .collect();
    }

    trimmed
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_owned)
        .collect()
}

fn fallback_directory_entry_from_path(path: &Path) -> Option<MonitorAgentDirectoryEntry> {
    let file_name = path.file_name()?.to_string_lossy().to_string();
    let content = fs::read_to_string(path).ok()?;
    let frontmatter = parse_agent_frontmatter(&content);
    let display_name = frontmatter
        .get("name")
        .cloned()
        .unwrap_or_else(|| file_name.replace(".agent.md", ""));
    let description = frontmatter
        .get("description")
        .cloned()
        .unwrap_or_else(|| "No description published in frontmatter.".to_owned());
    let family = monitor_category_for_agent(&display_name);
    let role = if display_name.to_ascii_lowercase().contains("architect") {
        "architect"
    } else if display_name.to_ascii_lowercase().contains("officer") {
        "officer"
    } else if display_name.to_ascii_lowercase().contains("core") || display_name.to_ascii_lowercase().contains("subcore") {
        "execution"
    } else {
        "agent"
    };
    Some(MonitorAgentDirectoryEntry {
        id: file_name.replace(".agent.md", ""),
        display_name,
        description,
        family,
        role: role.to_owned(),
        lane: role.to_owned(),
        source_path: path.display().to_string(),
        mirror_path: path.display().to_string(),
        sync_state: "mirror-only".to_owned(),
        user_invocable: frontmatter
            .get("user-invocable")
            .map(|value| value == "true")
            .unwrap_or(false),
        tools: parse_frontmatter_list(frontmatter.get("tools")),
        child_agents: parse_frontmatter_list(frontmatter.get("agents")),
    })
}

fn load_monitor_agent_directory() -> Vec<MonitorAgentDirectoryEntry> {
    let manifest_path = Path::new(MONITOR_AGENT_MANIFEST_PATH);
    if let Ok(content) = fs::read_to_string(manifest_path) {
        if let Ok(manifest) = serde_json::from_str::<AgentSourceTreeManifest>(&content) {
            let mut entries = manifest
                .entries
                .into_iter()
                .map(|entry| MonitorAgentDirectoryEntry {
                    id: entry.id,
                    display_name: entry.display_name,
                    description: entry.description.unwrap_or_else(|| "No description published.".to_owned()),
                    family: entry.family,
                    role: entry.role.clone(),
                    lane: entry.lane.unwrap_or(entry.role),
                    source_path: entry.source_path,
                    mirror_path: entry.mirror_path,
                    sync_state: "mirrored".to_owned(),
                    user_invocable: entry.user_invocable.unwrap_or(false),
                    tools: entry.tools.unwrap_or_default(),
                    child_agents: entry.child_agents.unwrap_or_default(),
                })
                .collect::<Vec<_>>();
            entries.sort_by(|left, right| left.display_name.cmp(&right.display_name));
            return entries;
        }
    }

    let mut entries = Vec::new();
    if let Ok(read_dir) = fs::read_dir(MONITOR_AGENT_MIRROR_ROOT) {
        for entry in read_dir.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "md")
                && path.file_name().is_some_and(|name| name.to_string_lossy().ends_with(".agent.md"))
            {
                if let Some(directory_entry) = fallback_directory_entry_from_path(&path) {
                    entries.push(directory_entry);
                }
            }
        }
    }
    entries.sort_by(|left, right| left.display_name.cmp(&right.display_name));
    entries
}

fn monitor_agent_source_tree_status(entry_count: usize) -> MonitorAgentSourceTreeStatus {
    let source_root = Path::new(MONITOR_AGENT_SOURCE_ROOT);
    let mirror_root = Path::new(MONITOR_AGENT_MIRROR_ROOT);
    let manifest_path = Path::new(MONITOR_AGENT_MANIFEST_PATH);
    let sync_state = if manifest_path.exists() && source_root.exists() {
        "mirrored"
    } else if mirror_root.exists() {
        "mirror-only"
    } else {
        "missing"
    };

    MonitorAgentSourceTreeStatus {
        manifest_path: manifest_path.display().to_string(),
        canonical_root: source_root.display().to_string(),
        mirror_root: mirror_root.display().to_string(),
        sync_state: sync_state.to_owned(),
        entry_count,
    }
}

impl MonitorState {
    fn snapshot(&self) -> MonitorStateSnapshot {
        let lifecycle_threads = self.lifecycle_threads.clone();
        let agent_directory = load_monitor_agent_directory();
        MonitorStateSnapshot {
            open_chats: self.open_chats.clone(),
            archived_chats: self.archived_chats.clone(),
            notifications: self.notifications.clone(),
            running_loops: self.running_loops.iter().map(normalize_monitor_loop).collect(),
            timeline: self.timeline.clone(),
            lifecycle_threads: lifecycle_threads.clone(),
            packets: self.packets.clone(),
            agent_directory: agent_directory.clone(),
            agent_source_tree: monitor_agent_source_tree_status(agent_directory.len()),
            selected_chat_id: self.selected_chat_id.clone(),
            selected_lifecycle_thread_id: self
                .selected_lifecycle_thread_id
                .clone()
                .or_else(|| lifecycle_threads.first().map(|thread| thread.id.clone())),
            next_id: self.next_id,
        }
    }

    fn create_lifecycle_thread(
        &mut self,
        title: String,
        kind: &str,
        owner_agent: &str,
        status: &str,
        source_agent_id: Option<String>,
        target_agent_id: Option<String>,
        work_order_id: Option<String>,
        escalation_id: Option<String>,
        initial_entry: MonitorLifecycleEntry,
    ) -> String {
        let id = self.next_identifier("thr");
        let thread = MonitorLifecycleThread {
            id: id.clone(),
            title,
            kind: kind.to_owned(),
            status: status.to_owned(),
            owner_agent: owner_agent.to_owned(),
            source_agent_id,
            target_agent_id,
            work_order_id,
            escalation_id,
            created_at: initial_entry.at.clone(),
            updated_at: initial_entry.at.clone(),
            entries: vec![initial_entry],
        };
        self.selected_lifecycle_thread_id = Some(id.clone());
        self.lifecycle_threads.insert(0, thread);
        self.lifecycle_threads.truncate(MONITOR_MAX_LIFECYCLE_THREADS);
        id
    }

    fn append_thread_entry(
        &mut self,
        thread_id: &str,
        kind: &str,
        speaker: &str,
        who: &str,
        text: String,
        status: Option<&str>,
        next_action: Option<&str>,
    ) -> bool {
        let entry = MonitorLifecycleEntry {
            id: self.next_identifier("lfe"),
            kind: kind.to_owned(),
            speaker: speaker.to_owned(),
            who: who.to_owned(),
            text,
            at: monitor_now(),
            status: status.map(str::to_owned),
            next_action: next_action.map(str::to_owned),
        };

        let Some(index) = self.lifecycle_threads.iter().position(|thread| thread.id == thread_id) else {
            return false;
        };

        let thread = &mut self.lifecycle_threads[index];
        if let Some(status) = status {
            thread.status = status.to_owned();
        }
        thread.updated_at = entry.at.clone();
        thread.entries.push(entry);
        if thread.entries.len() > MONITOR_MAX_THREAD_ENTRIES {
            let drain_count = thread.entries.len() - MONITOR_MAX_THREAD_ENTRIES;
            thread.entries.drain(0..drain_count);
        }
        self.selected_lifecycle_thread_id = Some(thread_id.to_owned());
        true
    }

    fn create_lifecycle_entry(
        &mut self,
        kind: &str,
        speaker: &str,
        who: &str,
        text: String,
        status: Option<&str>,
        next_action: Option<&str>,
    ) -> MonitorLifecycleEntry {
        MonitorLifecycleEntry {
            id: self.next_identifier("lfe"),
            kind: kind.to_owned(),
            speaker: speaker.to_owned(),
            who: who.to_owned(),
            text,
            at: monitor_now(),
            status: status.map(str::to_owned),
            next_action: next_action.map(str::to_owned),
        }
    }

    fn create_session(&mut self, label: Option<String>) -> MonitorSession {
        let id = self.next_identifier("chat");
        let now = monitor_now();
        let label = label.unwrap_or_else(|| format!("User -> FERROS #{}", self.next_id));
        let initial_entry = self.create_lifecycle_entry(
            "chat.opened",
            "agent",
            "FERROS Agent",
            format!("Lifecycle chat opened for {label}"),
            Some("running"),
            Some("Await operator message or route request"),
        );
        let thread_id = self.create_lifecycle_thread(
            label.clone(),
            "chat",
            "FERROS Agent",
            "running",
            Some("operator".to_owned()),
            Some("FERROS Agent".to_owned()),
            None,
            None,
            initial_entry,
        );
        let session = MonitorSession {
            id: id.clone(),
            label,
            active_agent: "FERROS Agent".to_owned(),
            thread_id: Some(thread_id),
            created_at: now.clone(),
            messages: vec![MonitorMessage {
                id: self.next_identifier("msg"),
                speaker: "agent".to_owned(),
                who: "FERROS Agent".to_owned(),
                text: "Session ready. Route requests into coding, business, or architect lanes.".to_owned(),
                at: now,
            }],
        };
        self.selected_chat_id = Some(id);
        self.open_chats.push(session.clone());
        self.push_event("open-chat", format!("Opened {}", session.label));
        session
    }

    fn add_message(&mut self, session_id: &str, request: MonitorMessageRequest) -> Option<String> {
        let MonitorMessageRequest { speaker, who, text } = request;
        let message_id = self.next_identifier("msg");
        let message = MonitorMessage {
            id: message_id.clone(),
            speaker: speaker.clone(),
            who: who.clone(),
            text: text.clone(),
            at: monitor_now(),
        };

        let Some(index) = self.open_chats.iter().position(|session| session.id == session_id) else {
            return None;
        };

        let (session_label, thread_id) = {
            let session = &mut self.open_chats[index];
            session.messages.push(message);
            (session.label.clone(), session.thread_id.clone())
        };
        if let Some(thread_id) = thread_id {
            let _ = self.append_thread_entry(
                &thread_id,
                "chat.message",
                &speaker,
                &who,
                text,
                Some("running"),
                Some("Await route or reply"),
            );
        }
        self.push_event(
            "chat.message.added",
            format!("Message {message_id} appended in {}", session_label),
        );
        Some(message_id)
    }

    fn session_active_agent(&self, session_id: &str) -> Option<&str> {
        self.open_chats
            .iter()
            .find(|session| session.id == session_id)
            .map(|session| session.active_agent.as_str())
    }

    fn ferros_agent_handle_human_message(
        &mut self,
        session_id: &str,
        message_id: &str,
        text: &str,
    ) -> MonitorDispatchResult {
        let dispatch_request = MonitorDispatchRequest {
            session_id: session_id.to_owned(),
            message_id: message_id.to_owned(),
            operator_text: text.to_owned(),
            active_agent: "FERROS Agent".to_owned(),
        };

        if self
            .open_chats
            .iter()
            .all(|session| session.id != dispatch_request.session_id)
        {
            return MonitorDispatchResult {
                ferros_reply: "Session was not found; dispatch could not run.".to_owned(),
                packet_id: None,
                manager: None,
                lifecycle_thread_id: None,
                notification_id: None,
                backend: None,
                status: MonitorDispatchStatus::Failed,
            };
        }

        let lowered = dispatch_request.operator_text.to_ascii_lowercase();
        if lowered.contains("human intervention") || lowered.contains("needs operator") || lowered.contains("escalate") {
            let packet_id = self.create_packet(
                dispatch_request.session_id.clone(),
                dispatch_request.message_id.clone(),
                None,
                "FERROS Agent".to_owned(),
                "human_intervention_required",
                None,
                None,
                "Escalation: operator review required".to_owned(),
            );
            let notification_id = self.create_notification(
                Some(packet_id.clone()),
                Some(dispatch_request.session_id.clone()),
                None,
                "high",
                "Human intervention required",
                "FERROS Agent could not resolve automatically. Open FERROS chat to continue.",
            );
            // Back-link the notification onto the packet
            if let Some(p) = self.packets.iter_mut().find(|p| p.id == packet_id) {
                p.notification_id = Some(notification_id.clone());
            }
            let ferros_reply = "I could not safely resolve this request automatically. I created an operator notification and paused downstream execution pending your guidance.".to_owned();
            let _ = self.add_message(
                &dispatch_request.session_id,
                MonitorMessageRequest {
                    speaker: "agent".to_owned(),
                    who: "FERROS Agent".to_owned(),
                    text: ferros_reply.clone(),
                },
            );
            return MonitorDispatchResult {
                ferros_reply,
                packet_id: Some(packet_id),
                manager: None,
                lifecycle_thread_id: None,
                notification_id: Some(notification_id),
                backend: None,
                status: MonitorDispatchStatus::HumanInterventionRequired,
            };
        }

        let target = infer_dispatch_target(&dispatch_request.operator_text);

        // Consult the dispatch backend before committing state mutations.
        let backend_result = ScaffoldMonitorDispatchBackend.handle_dispatch(
            &dispatch_request.session_id,
            &target,
            &dispatch_request.operator_text,
        );

        if !backend_result.accepted {
            return MonitorDispatchResult {
                ferros_reply: backend_result
                    .error
                    .unwrap_or_else(|| "Backend rejected dispatch.".to_owned()),
                packet_id: None,
                manager: None,
                lifecycle_thread_id: None,
                notification_id: None,
                backend: Some(backend_result.backend),
                status: MonitorDispatchStatus::Failed,
            };
        }

        let Some((packet_id, manager, lifecycle_thread_id)) =
            self.dispatch_session_to_manager(
                &dispatch_request.session_id,
                &dispatch_request.message_id,
                target,
            )
        else {
            return MonitorDispatchResult {
                ferros_reply: "Dispatch failed because the session was unavailable.".to_owned(),
                packet_id: None,
                manager: None,
                lifecycle_thread_id: None,
                notification_id: None,
                backend: Some(backend_result.backend),
                status: MonitorDispatchStatus::Failed,
            };
        };

        let ferros_reply = format!(
            "Packet staged for {manager} as {packet_id}. Live manager execution is not yet connected, but this Administration liaison chat will stay open for updates."
        );
        let _ = self.add_message(
            &dispatch_request.session_id,
            MonitorMessageRequest {
                speaker: "agent".to_owned(),
                who: "FERROS Agent".to_owned(),
                text: ferros_reply.clone(),
            },
        );

        MonitorDispatchResult {
            ferros_reply,
            packet_id: Some(packet_id),
            manager: Some(manager),
            lifecycle_thread_id: Some(lifecycle_thread_id),
            notification_id: None,
            backend: Some(backend_result.backend),
            status: MonitorDispatchStatus::Routed,
        }
    }

    fn dispatch_session_to_manager(
        &mut self,
        session_id: &str,
        origin_message_id: &str,
        target: DispatchTarget,
    ) -> Option<(String, String, String)> {
        let (session_label, session_thread_id) = {
            let session = self.open_chats.iter().find(|session| session.id == session_id)?;
            (session.label.clone(), session.thread_id.clone())
        };

        let work_order_id = self.next_identifier("wo");
        let (route_label, loop_agent, loop_desc) = match target {
            DispatchTarget::Business => (
                "Business Agent",
                "Business Agent",
                "Executing business packet and coordinating department loops.",
            ),
            DispatchTarget::FerrosArchitect => (
                "FERROS Agent Architect Agent",
                "FERROS Agent Architect Agent",
                "Coordinating architect-family delegation to coding/business architects.",
            ),
            DispatchTarget::CodingArchitect => (
                "Coding Agent Architect",
                "Coding Agent Architect",
                "Designing coding-family topology and lane architecture.",
            ),
            DispatchTarget::BusinessArchitect => (
                "Business Agent Architect",
                "Business Agent Architect",
                "Designing business-company and departmental architecture.",
            ),
            DispatchTarget::Software => (
                "Software Architect",
                "Software Architect",
                "Executing software work order and preparing Core/SubCore branch packets.",
            ),
        };

        if let Some(thread_id) = session_thread_id.as_deref() {
            let _ = self.append_thread_entry(
                thread_id,
                "packet.sent",
                "agent",
                "FERROS Agent",
                format!("Dispatched to {route_label} as work order {work_order_id}"),
                Some("work_order_emitted"),
                Some("Follow downstream packet lifecycle thread"),
            );
        }

        let packet_entry = self.create_lifecycle_entry(
            "packet.created",
            "agent",
            "FERROS Agent",
            format!("Staged {work_order_id} for {route_label} from {session_label}"),
            Some("dispatched_to_manager"),
            Some("Await manager-level lifecycle updates"),
        );
        let packet_thread_id = self.create_lifecycle_thread(
            format!("{route_label} packet {work_order_id}"),
            "packet",
            route_label,
            "dispatched_to_manager",
            Some("FERROS Agent".to_owned()),
            Some(route_label.to_owned()),
            Some(work_order_id.clone()),
            None,
            packet_entry,
        );

        self.upsert_loop(loop_agent, "running", loop_desc);
        self.push_event("packet.sent", format!("{} -> {route_label}", session_label));

        let packet_id = self.create_packet(
            session_id.to_owned(),
            origin_message_id.to_owned(),
            Some(work_order_id.clone()),
            route_label.to_owned(),
            "dispatched_to_manager",
            Some(packet_thread_id.clone()),
            None,
            format!("Staged from {session_label} → {route_label}"),
        );

        Some((packet_id, route_label.to_owned(), packet_thread_id))
    }

    fn create_packet(
        &mut self,
        session_id: String,
        origin_message_id: String,
        work_order_id: Option<String>,
        manager: String,
        state: &str,
        lifecycle_thread_id: Option<String>,
        notification_id: Option<String>,
        summary: String,
    ) -> String {
        let id = self.next_identifier("pkt");
        let now = monitor_now();
        self.packets.push(MonitorPacket {
            id: id.clone(),
            session_id,
            origin_message_id,
            work_order_id,
            manager,
            state: state.to_owned(),
            lifecycle_thread_id,
            notification_id,
            created_at: now.clone(),
            updated_at: now,
            summary,
            last_error: None,
        });
        self.push_event("packet.created", format!("Packet {id} registered"));
        id
    }

    #[allow(dead_code)]
    fn update_packet_state(&mut self, packet_id: &str, new_state: &str, detail: Option<String>) -> bool {
        let Some(packet) = self.packets.iter_mut().find(|p| p.id == packet_id) else {
            return false;
        };
        packet.state = new_state.to_owned();
        packet.updated_at = monitor_now();
        if let Some(err) = detail {
            packet.last_error = Some(err);
        }
        self.push_event("packet.state_changed", format!("{packet_id} -> {new_state}"));
        true
    }

    #[allow(dead_code)]
    fn packet_by_id(&self, packet_id: &str) -> Option<&MonitorPacket> {
        self.packets.iter().find(|p| p.id == packet_id)
    }

    fn create_notification(
        &mut self,
        packet_id: Option<String>,
        session_id: Option<String>,
        lifecycle_thread_id: Option<String>,
        severity: &str,
        title: &str,
        summary: &str,
    ) -> String {
        let id = self.next_identifier("ntf");
        self.notifications.insert(
            0,
            MonitorNotification {
                id: id.clone(),
                packet_id,
                session_id,
                lifecycle_thread_id,
                severity: severity.to_owned(),
                title: title.to_owned(),
                summary: summary.to_owned(),
                action: "open_ferros_chat".to_owned(),
                created_at: monitor_now(),
                status: "unread".to_owned(),
            },
        );
        self.notifications.truncate(200);
        self.push_event("notification.created", format!("{title}: {summary}"));
        id
    }

    fn open_notification(&mut self, notification_id: &str) -> bool {
        let Some(notification) = self
            .notifications
            .iter_mut()
            .find(|n| n.id == notification_id)
        else {
            return false;
        };

        let session_id = notification.session_id.clone();
        let lifecycle_thread_id = notification.lifecycle_thread_id.clone();
        let title = notification.title.clone();
        notification.status = "opened".to_owned();

        if let Some(sid) = session_id {
            if self.open_chats.iter().any(|c| c.id == sid) {
                self.selected_chat_id = Some(sid);
            }
        }

        if let Some(tid) = lifecycle_thread_id {
            if self.lifecycle_threads.iter().any(|t| t.id == tid) {
                self.selected_lifecycle_thread_id = Some(tid);
            }
        }

        self.push_event("notification.opened", format!("{title} acknowledged by operator"));
        true
    }

    fn update_notification_status(&mut self, notification_id: &str, status: &str) -> bool {
        let Some(notification) = self
            .notifications
            .iter_mut()
            .find(|notification| notification.id == notification_id)
        else {
            return false;
        };

        let title = notification.title.clone();
        notification.status = status.to_owned();
        self.push_event("notification.updated", format!("{title} -> {status}"));
        true
    }

    fn add_lifecycle_message(&mut self, thread_id: &str, request: MonitorLifecycleMessageRequest) -> bool {
        self.append_thread_entry(
            thread_id,
            "lifecycle.note",
            &request.speaker,
            &request.who,
            request.text,
            None,
            None,
        )
    }

    fn route_session(&mut self, session_id: &str, target: &str) -> bool {
        let target = match target {
            "business" => DispatchTarget::Business,
            "ferros-architect" => DispatchTarget::FerrosArchitect,
            "coding-architect" => DispatchTarget::CodingArchitect,
            "business-architect" => DispatchTarget::BusinessArchitect,
            _ => DispatchTarget::Software,
        };

        self.dispatch_session_to_manager(session_id, "", target).is_some()
    }

    fn archive_session(&mut self, session_id: &str) -> bool {
        let Some(index) = self.open_chats.iter().position(|s| s.id == session_id) else {
            return false;
        };
        let session = self.open_chats.remove(index);
        self.archived_chats.insert(
            0,
            MonitorArchivedSession {
                id: session.id,
                label: session.label.clone(),
                reason: "Manually archived by operator".to_owned(),
                archived_at: monitor_now(),
                preview: session.messages.iter().rev().take(3).cloned().collect(),
            },
        );
        self.archived_chats.truncate(MONITOR_MAX_ARCHIVED_SESSIONS);
        self.selected_chat_id = self.open_chats.first().map(|c| c.id.clone());
        if let Some(thread_id) = session.thread_id.as_deref() {
            let _ = self.append_thread_entry(
                thread_id,
                "chat.archived",
                "system",
                "Monitor",
                format!("{} archived by operator", session.label),
                Some("archived"),
                Some("See archive drawer for retained preview"),
            );
        }
        self.push_event("session.archived", format!("{} archived", session.label));
        true
    }

    fn apply_loop_transition(&mut self, action: &str) {
        match action {
            "architect-split" => {
                self.upsert_loop(
                    "FERROS Agent Architect Agent",
                    "running",
                    "Delegating packets across coding and business architect lanes.",
                );
                self.upsert_loop(
                    "Coding Agent Architect",
                    "running",
                    "Coding architect recursion in progress.",
                );
                self.upsert_loop(
                    "Business Agent Architect",
                    "running",
                    "Business architect recursion in progress.",
                );
                self.push_event(
                    "loop.started",
                    "FERROS Agent Architect delegated to Coding and Business architects.".to_string(),
                );
            }
            "coding-split" => {
                self.upsert_loop(
                    "Software Architect",
                    "waiting",
                    "Waiting for Core/SubCore completion packets.",
                );
                self.upsert_loop("Core Agent", "running", "Core lane executing routed packet.");
                self.upsert_loop(
                    "SubCore Agent",
                    "running",
                    "SubCore lane executing routed packet.",
                );
                self.push_event(
                    "loop.started",
                    "Software Architect split into Core and SubCore loops.".to_string(),
                );
            }
            "coding-merge" => {
                self.remove_loop("Core Agent");
                self.remove_loop("SubCore Agent");
                self.upsert_loop(
                    "Software Architect",
                    "running",
                    "Core/SubCore responses merged. Software Architect continuation active.",
                );
                self.push_event(
                    "loop.merged",
                    "Core/SubCore returned to Software Architect.".to_string(),
                );
            }
            "business-split" => {
                self.upsert_loop(
                    "Business Agent Architect",
                    "waiting",
                    "Awaiting department loop completions.",
                );
                self.upsert_loop(
                    "Operations Department",
                    "running",
                    "Operations directives executing.",
                );
                self.upsert_loop("Sales Department", "running", "Sales directives executing.");
                self.upsert_loop(
                    "Finance Department",
                    "running",
                    "Finance directives executing.",
                );
                self.upsert_loop("HR Department", "running", "HR directives executing.");
                self.push_event(
                    "loop.started",
                    "Business architect split into Operations/Sales/Finance/HR.".to_string(),
                );
            }
            "business-merge" => {
                self.remove_loop("Operations Department");
                self.remove_loop("Sales Department");
                self.remove_loop("Finance Department");
                self.remove_loop("HR Department");
                self.upsert_loop(
                    "Business Agent Architect",
                    "running",
                    "Department outputs merged; business architecture loop resumed.",
                );
                self.push_event("loop.merged", "Department loops returned to Business Architect.".to_string());
            }
            "escalate" => {
                self.running_loops.clear();
                self.upsert_loop(
                    "FERROS Escalation",
                    "waiting",
                    "Escalation reached Administration and needs operator-visible clarification.",
                );
                self.push_event(
                    "escalation.opened",
                    "Escalation opened. FERROS chat moved to operator-visible queue.".to_string(),
                );
                let session = self.create_session(Some("Escalation -> FERROS".to_owned()));
                let escalation_message_id = self.next_identifier("msg");
                if let Some(open) = self.open_chats.iter_mut().find(|chat| chat.id == session.id) {
                    open.messages.push(MonitorMessage {
                        id: escalation_message_id,
                        speaker: "agent".to_owned(),
                        who: "FERROS Agent".to_owned(),
                        text: "Escalation received. Human decision is required before continuing.".to_owned(),
                        at: monitor_now(),
                    });
                }
                if let Some(thread_id) = session.thread_id.as_deref() {
                    let _ = self.append_thread_entry(
                        thread_id,
                        "escalation.opened",
                        "agent",
                        "FERROS Agent",
                        "Escalation received. Human decision is required before continuing.".to_owned(),
                        Some("attention"),
                        Some("Open or explain from Administration"),
                    );
                }
            }
            _ => {}
        }
    }

    fn upsert_loop(&mut self, agent: &str, state: &str, description: &str) {
        let id = agent.to_ascii_lowercase().replace(' ', "-");
        let now = monitor_now();
        let category = monitor_category_for_agent(agent);
        let (status, status_reason, status_detail, progress) =
            monitor_status_detail_for(agent, state, description);
        let stale_after = monitor_at_plus_seconds(if status == "running" { 300 } else { 180 });

        if let Some(existing) = self.running_loops.iter_mut().find(|loop_state| loop_state.id == id) {
            let thread_id = existing.thread_id.clone();
            existing.state = state.to_owned();
            existing.category = category;
            existing.status = status.clone();
            existing.status_reason = status_reason.clone();
            existing.status_detail = status_detail.clone();
            existing.description = description.to_owned();
            existing.progress = progress;
            existing.stale_after = stale_after;
            existing.updated_at = now;
            if let Some(thread_id) = thread_id {
                let _ = self.append_thread_entry(
                    &thread_id,
                    "loop.status",
                    "system",
                    "Monitor",
                    format!("{agent}: {description}"),
                    Some(&status),
                    Some("Await report, merge, or escalation"),
                );
            }
            return;
        }

        let work_order_id = if status == "running" {
            Some(self.next_identifier("wo"))
        } else {
            None
        };
        let escalation_id = if status == "attention" || status == "escalating" {
            Some(self.next_identifier("esc"))
        } else {
            None
        };
        let initial_entry = self.create_lifecycle_entry(
            "loop.created",
            "system",
            "Monitor",
            format!("{agent}: {description}"),
            Some(&status),
            Some("Await report, merge, or escalation"),
        );
        let thread_id = self.create_lifecycle_thread(
            agent.to_owned(),
            "loop",
            agent,
            &status,
            Some(category.clone()),
            Some(agent.to_owned()),
            work_order_id.clone(),
            escalation_id.clone(),
            initial_entry,
        );

        self.running_loops.push(MonitorLoop {
            id,
            agent: agent.to_owned(),
            state: state.to_owned(),
            category,
            status,
            status_reason,
            status_detail,
            description: description.to_owned(),
            started_at: now.clone(),
            updated_at: now,
            stale_after,
            progress,
            thread_id: Some(thread_id),
            work_order_id,
            escalation_id,
            source_agent_id: Some("FERROS Monitor".to_owned()),
            target_agent_id: Some(agent.to_owned()),
        });
    }

    fn remove_loop(&mut self, agent: &str) {
        let id = agent.to_ascii_lowercase().replace(' ', "-");
        let removed = self
            .running_loops
            .iter()
            .find(|loop_state| loop_state.id == id)
            .cloned();
        self.running_loops.retain(|loop_state| loop_state.id != id);
        if let Some(removed) = removed {
            if let Some(thread_id) = removed.thread_id.as_deref() {
                let _ = self.append_thread_entry(
                    thread_id,
                    "loop.closed",
                    "system",
                    "Monitor",
                    format!("{} removed from the active loop roster", removed.agent),
                    Some("reported"),
                    Some("Inspect merged parent or archive history"),
                );
            }
        }
    }

    fn push_event(&mut self, kind: &str, text: String) {
        let event = MonitorEvent {
            id: self.next_identifier("evt"),
            kind: kind.to_owned(),
            text,
            at: monitor_now(),
        };
        self.timeline.insert(0, event);
        self.timeline.truncate(MONITOR_MAX_TIMELINE_EVENTS);
    }

    fn next_identifier(&mut self, prefix: &str) -> String {
        self.next_id += 1;
        format!("{prefix}-{}", self.next_id)
    }
}

fn execute_profile_shell_request_with_store<S: LocalProfileStore>(
    request: ProfileShellRequest,
    default_profile_path: &Path,
    store: &S,
) -> ProfileShellResponse {
    let action = request.action.trim().to_ascii_lowercase();
    let profile_path =
        requested_profile_path(request.profile_path.as_deref(), default_profile_path);
    let bundle_path = request
        .bundle_path
        .as_deref()
        .and_then(non_empty_trimmed)
        .map(PathBuf::from);

    let command = match action.as_str() {
        "init" => Ok(ProfileCliCommand::Init {
            path: profile_path.clone(),
        }),
        "show" => Ok(ProfileCliCommand::Show {
            path: profile_path.clone(),
        }),
        "export" => match bundle_path.clone() {
            Some(bundle_path) => Ok(ProfileCliCommand::Export {
                path: profile_path.clone(),
                bundle_path,
            }),
            None => Err("bundlePath is required for profile export".to_owned()),
        },
        "import" => match bundle_path.clone() {
            Some(bundle_path) => Ok(ProfileCliCommand::Import {
                path: profile_path.clone(),
                bundle_path,
            }),
            None => Err("bundlePath is required for profile import".to_owned()),
        },
        "grant" | "revoke" => Err(
            "profile grant and revoke are not exposed through the localhost profile surface"
                .to_owned(),
        ),
        _ => Err(format!("unsupported profile action: {action}")),
    };

    match command {
        Ok(command) => match execute_profile_cli_with_store(command, store) {
            Ok(lines) => {
                let profile = profile_value_for_action(action.as_str(), store, &profile_path);

                ProfileShellResponse {
                    ok: true,
                    status: profile_shell_success_status(action.as_str()),
                    action,
                    profile_path: profile_path.display().to_string(),
                    bundle_path: bundle_path.map(|path| path.display().to_string()),
                    profile,
                    lines,
                    error: None,
                    error_detail: None,
                }
            }
            Err(error) => {
                let error_message = error.to_string();
                let error_detail = profile_shell_error_detail(&error);

                ProfileShellResponse {
                    ok: false,
                    status: profile_shell_blocked_status(action.as_str()),
                    action,
                    profile_path: profile_path.display().to_string(),
                    bundle_path: bundle_path.map(|path| path.display().to_string()),
                    lines: Vec::new(),
                    profile: None,
                    error: Some(error_message),
                    error_detail: Some(error_detail),
                }
            }
        },
        Err(error) => {
            let error_detail = profile_shell_request_error_detail(action.as_str(), &error);

            ProfileShellResponse {
                ok: false,
                status: profile_shell_blocked_status(action.as_str()),
                action,
                profile_path: profile_path.display().to_string(),
                bundle_path: bundle_path.map(|path| path.display().to_string()),
                lines: Vec::new(),
                profile: None,
                error: Some(error),
                error_detail: Some(error_detail),
            }
        }
    }
}

fn profile_shell_success_status(action: &str) -> ProfileShellStatus {
    let summary = match action {
        "init" => "Local profile initialized through /profile.",
        "show" => "Local profile document loaded through /profile.",
        "export" => "Local profile bundle exported through /profile.",
        "import" => "Local profile bundle imported through /profile.",
        _ => "Local profile action completed through /profile.",
    };

    ProfileShellStatus {
        kind: "complete".to_owned(),
        summary: summary.to_owned(),
    }
}

fn profile_shell_blocked_status(action: &str) -> ProfileShellStatus {
    let summary = match action {
        "init" => "Local profile initialization was blocked on /profile.",
        "show" => "Local profile read was blocked on /profile.",
        "export" => "Local profile export was blocked on /profile.",
        "import" => "Local profile import was blocked on /profile.",
        _ => "Local profile action was blocked on /profile.",
    };

    ProfileShellStatus {
        kind: "blocked".to_owned(),
        summary: summary.to_owned(),
    }
}

fn profile_shell_request_error_detail(action: &str, error: &str) -> ProfileShellErrorDetail {
    let code = match action {
        "export" | "import" if error.contains("bundlePath is required") => "bundle_path_required",
        "grant" | "revoke" => "mutation_not_exposed",
        _ if error.starts_with("unsupported profile action:") => "unsupported_action",
        _ => "invalid_request",
    };

    ProfileShellErrorDetail {
        code: code.to_owned(),
        message: error.to_owned(),
    }
}

fn profile_shell_error_detail(error: &CliError) -> ProfileShellErrorDetail {
    let code = match error {
        CliError::Usage(_) => "usage",
        CliError::InvalidState(_) => "invalid_state",
        CliError::Io(io_error) if io_error.kind() == io::ErrorKind::NotFound => "not_found",
        CliError::Io(_) => "io",
        CliError::Profile(ProfileStoreError::AlreadyExists(_)) => "already_exists",
        CliError::Profile(ProfileStoreError::CapabilityGrantAlreadyExists(_)) => {
            "capability_grant_exists"
        }
        CliError::Profile(ProfileStoreError::CapabilityGrantNotFound(_)) => {
            "capability_grant_not_found"
        }
        CliError::Profile(ProfileStoreError::CapabilityGrantSignature(_)) => {
            "capability_grant_signature"
        }
        CliError::Profile(ProfileStoreError::InvalidProfile(_)) => "invalid_profile",
        CliError::Profile(ProfileStoreError::InvalidLocalState(_)) => "invalid_local_state",
        CliError::Profile(ProfileStoreError::Io(io_error))
            if io_error.kind() == io::ErrorKind::NotFound =>
        {
            "not_found"
        }
        CliError::Profile(ProfileStoreError::Io(_)) => "io",
        CliError::Profile(ProfileStoreError::KeyPair(_)) => "key_pair_error",
        CliError::Profile(ProfileStoreError::Serde(_)) => "invalid_json",
        CliError::Runtime(_) => "runtime_error",
    };

    ProfileShellErrorDetail {
        code: code.to_owned(),
        message: error.to_string(),
    }
}

fn requested_profile_path(value: Option<&str>, default_profile_path: &Path) -> PathBuf {
    value
        .and_then(non_empty_trimmed)
        .map(PathBuf::from)
        .unwrap_or_else(|| default_profile_path.to_path_buf())
}

fn non_empty_trimmed(value: &str) -> Option<&str> {
    let trimmed = value.trim();
    (!trimmed.is_empty()).then_some(trimmed)
}

fn map_local_runway_hub_restart_summary(
    summary: &LocalHubRuntimeSummary,
) -> LocalRunwayHubRestartSummary {
    let restart_observation = &summary.restart_observation;

    LocalRunwayHubRestartSummary {
        reload_status: restart_observation.reload_status.as_str().to_owned(),
        snapshot_path: LOCAL_HUB_STATE_SNAPSHOT_PATH.to_owned(),
        scope: summary.scope.clone(),
        evidence: summary.evidence.clone(),
        prior_bridge_manifest_identity: restart_observation.prior_bridge_manifest_identity.clone(),
        prior_policy_decision_label: restart_observation.prior_policy_decision_label.clone(),
        prior_artifact_relative_output_path: restart_observation
            .prior_artifact_relative_output_path
            .clone(),
    }
}

fn map_local_runway_hub_onramp_proposal_summary(
    summary: &LocalHubRuntimeSummary,
) -> Option<LocalRunwayHubOnrampProposalSummary> {
    summary
        .local_onramp_proposal
        .as_ref()
        .filter(|proposal| has_valid_local_onramp_proposal_projection(proposal))
        .map(|proposal| LocalRunwayHubOnrampProposalSummary {
            source: proposal.source.clone(),
            proposal_id: proposal.proposal_id.clone(),
            bridge_agent_name: proposal.bridge_agent_name.clone(),
            stand_in_entity_name: proposal.stand_in_entity_name.clone(),
            requested_capability: proposal.requested_capability.clone(),
            requested_action: proposal.requested_action.clone(),
            quarantine_status: proposal.quarantine_status.as_str().to_owned(),
            scope: proposal.scope.clone(),
            evidence: proposal.evidence.clone(),
            local_artifact_path: proposal.local_artifact_path.clone(),
        })
}

fn map_local_runway_hub_onramp_decision_receipt_summary(
    summary: &LocalHubRuntimeSummary,
) -> Option<LocalRunwayHubOnrampDecisionReceiptSummary> {
    summary
        .local_onramp_decision_receipt
        .as_ref()
        .filter(|receipt| {
            has_valid_local_onramp_decision_projection(
                &receipt.proposal_id,
                &receipt.proposal_artifact_path,
                receipt.decision_detail.as_deref(),
                &receipt.scope,
                &receipt.evidence,
                &receipt.local_artifact_path,
            )
        })
        .map(|receipt| LocalRunwayHubOnrampDecisionReceiptSummary {
            proposal_id: receipt.proposal_id.clone(),
            decision_label: receipt.decision_label.as_str().to_owned(),
            decision_detail: receipt.decision_detail.clone(),
            local_artifact_path: receipt.local_artifact_path.clone(),
        })
}

fn observe_local_runway_hub_observations_with<HubSummaryLoader>(
    hub_summary_loader: HubSummaryLoader,
) -> (
    Option<LocalRunwayHubRestartSummary>,
    Option<LocalRunwayHubOnrampProposalSummary>,
    Option<LocalRunwayHubOnrampDecisionReceiptSummary>,
)
where
    HubSummaryLoader: FnOnce() -> Result<LocalHubRuntimeSummary, LocalBridgeRegistrationError>,
{
    match hub_summary_loader() {
        Ok(summary) => (
            Some(map_local_runway_hub_restart_summary(&summary)),
            map_local_runway_hub_onramp_proposal_summary(&summary),
            map_local_runway_hub_onramp_decision_receipt_summary(&summary),
        ),
        Err(_) => (None, None, None),
    }
}

fn has_valid_local_onramp_proposal_projection(proposal: &LocalOnrampProposal) -> bool {
    local_hub_relative_json_path_is_valid(&proposal.source)
        && local_hub_relative_json_path_is_valid(&proposal.local_artifact_path)
        && local_runway_scope_is_local_only(&proposal.scope)
        && local_runway_evidence_is_non_evidentiary(&proposal.evidence)
        && is_valid_local_onramp_text(&proposal.proposal_id)
        && is_valid_local_onramp_text(&proposal.bridge_agent_name)
        && is_valid_local_onramp_text(&proposal.stand_in_entity_name)
        && is_valid_local_onramp_text(&proposal.requested_capability)
        && is_valid_local_onramp_text(&proposal.requested_action)
}

fn has_valid_local_onramp_decision_projection(
    proposal_id: &str,
    proposal_artifact_path: &str,
    decision_detail: Option<&str>,
    scope: &str,
    evidence: &str,
    local_artifact_path: &str,
) -> bool {
    local_hub_relative_json_path_is_valid(proposal_artifact_path)
        && local_hub_relative_json_path_is_valid(local_artifact_path)
        && local_runway_scope_is_local_only(scope)
        && local_runway_evidence_is_non_evidentiary(evidence)
        && is_valid_local_onramp_text(proposal_id)
        && decision_detail
            .map(is_valid_local_onramp_text)
            .unwrap_or(true)
}

fn is_valid_local_onramp_text(value: &str) -> bool {
    !value.trim().is_empty()
        && !local_runway_text_looks_remote_like_url(value)
        && local_onramp_banned_wording(value).is_none()
}

fn profile_value_for_action<S: LocalProfileStore>(
    action: &str,
    store: &S,
    profile_path: &Path,
) -> Option<serde_json::Value> {
    match action {
        "init" | "show" | "import" => store
            .load_profile(profile_path)
            .ok()
            .and_then(|profile| serde_json::to_value(profile).ok()),
        _ => None,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LocalRunwayProfileObservation {
    profile_name: Option<String>,
    grant_count: usize,
    active_grant_count: usize,
    revoked_grant_count: usize,
    checklist_item: LocalRunwayChecklistItem,
}

fn build_local_runway_summary_with_store<S: LocalProfileStore>(
    state_path: &Path,
    profile_path: &Path,
    store: &S,
) -> Result<LocalRunwaySummary, CliError> {
    build_local_runway_summary_with_store_and_hub_summary_loader(
        state_path,
        profile_path,
        store,
        default_local_runtime_summary,
    )
}

fn build_local_runway_summary_with_store_and_hub_summary_loader<S, HubSummaryLoader>(
    state_path: &Path,
    profile_path: &Path,
    store: &S,
    hub_summary_loader: HubSummaryLoader,
) -> Result<LocalRunwaySummary, CliError>
where
    S: LocalProfileStore,
    HubSummaryLoader: FnOnce() -> Result<LocalHubRuntimeSummary, LocalBridgeRegistrationError>,
{
    let state = CliState::load(state_path)?;
    let runtime = runtime_with_state_from_loaded_state(&state)?;
    let agent_records = runtime.agent_records();
    let deny_entries = deny_log_entries(&state, None);
    let deny_count = deny_entries.len();
    let latest_deny = deny_entries
        .into_iter()
        .last()
        .map(LocalRunwayDenySummary::from);
    let profile_observation = observe_local_runway_profile(store, profile_path);
    let (hub_restart, hub_onramp_proposal, hub_onramp_decision_receipt) =
        observe_local_runway_hub_observations_with(hub_summary_loader);

    let stand_in_agent = match runtime.describe_agent("echo") {
        Some(agent) => LocalRunwayChecklistItem {
            item: "namedStandInAgent".to_owned(),
            status: LocalRunwayChecklistStatus::Observed,
            detail: format!(
                "echo available with status {}",
                format_agent_status(agent.status)
            ),
        },
        None => LocalRunwayChecklistItem {
            item: "namedStandInAgent".to_owned(),
            status: LocalRunwayChecklistStatus::Error,
            detail: "echo stand-in agent missing from local reference host".to_owned(),
        },
    };

    let checkpoint_state = derive_local_runway_state(
        &profile_observation,
        &stand_in_agent,
        latest_deny.as_ref(),
        &agent_records,
        &state.log_entries,
    );

    let agents = agent_records
        .into_iter()
        .map(LocalRunwayAgentSummary::from)
        .collect::<Vec<_>>();

    let consent_flow = match latest_deny.as_ref() {
        Some(deny) => LocalRunwayChecklistItem {
            item: "consentFlowVisibility".to_owned(),
            status: LocalRunwayChecklistStatus::Observed,
            detail: format_local_runway_deny_detail(deny),
        },
        None => {
            let (status, detail) = match profile_observation.checklist_item.status {
                LocalRunwayChecklistStatus::Error => (
                    LocalRunwayChecklistStatus::Error,
                    "profile observation failed before deny visibility could be checked".to_owned(),
                ),
                LocalRunwayChecklistStatus::Observed => (
                    LocalRunwayChecklistStatus::Pending,
                    format!(
                        "{} loaded; no deny entry observed yet",
                        profile_observation
                            .profile_name
                            .as_deref()
                            .unwrap_or("local profile")
                    ),
                ),
                LocalRunwayChecklistStatus::Pending => (
                    LocalRunwayChecklistStatus::Pending,
                    "awaiting local profile initialization or deny observation".to_owned(),
                ),
            };

            LocalRunwayChecklistItem {
                item: "consentFlowVisibility".to_owned(),
                status,
                detail,
            }
        }
    };

    let power_cycle = LocalRunwayChecklistItem {
        item: "powerCycleStatus".to_owned(),
        status: LocalRunwayChecklistStatus::Pending,
        detail: "power-cycle observation remains pending on this local-only surface".to_owned(),
    };

    Ok(LocalRunwaySummary {
        surface: "local-runway-summary".to_owned(),
        scope: "local-only".to_owned(),
        evidence: "non-evidentiary".to_owned(),
        checkpoint_state: checkpoint_state.as_str().to_owned(),
        checkpoint_detail: checkpoint_state.shell_detail().to_owned(),
        checkpoint_position: checkpoint_state.ordinal() + 1,
        checkpoint_total: LocalRunwayState::ALL.len(),
        profile_path: profile_path.display().to_string(),
        profile_name: profile_observation.profile_name,
        agent_count: agents.len(),
        agents,
        grant_count: profile_observation.grant_count,
        active_grant_count: profile_observation.active_grant_count,
        revoked_grant_count: profile_observation.revoked_grant_count,
        deny_count,
        latest_deny,
        checklist: vec![
            profile_observation.checklist_item,
            stand_in_agent,
            consent_flow,
            power_cycle,
        ],
        hub_restart,
        hub_onramp_proposal,
        hub_onramp_decision_receipt,
    })
}

fn derive_local_runway_state(
    profile_observation: &LocalRunwayProfileObservation,
    stand_in_agent: &LocalRunwayChecklistItem,
    latest_deny: Option<&LocalRunwayDenySummary>,
    agent_records: &[AgentRecord],
    log_entries: &[String],
) -> LocalRunwayState {
    let mut checkpoint = LocalRunwayState::Pending;
    let has_profile = matches!(
        profile_observation.checklist_item.status,
        LocalRunwayChecklistStatus::Observed
    );
    let has_consent_observation = latest_deny.is_some() || profile_observation.grant_count > 0;
    let has_runtime_observation =
        matches!(stand_in_agent.status, LocalRunwayChecklistStatus::Observed);
    let running_agent_count = agent_records
        .iter()
        .filter(|record| matches!(record.status, AgentStatus::Running))
        .count();

    if has_profile {
        checkpoint = checkpoint
            .advance(ferros_runtime::LocalRunwayIntent::Start)
            .expect("pending -> profile-ready should be valid");
    }

    if has_consent_observation {
        checkpoint = checkpoint
            .advance(ferros_runtime::LocalRunwayIntent::Start)
            .expect("profile-ready -> consent-ready should be valid");
    }

    if has_runtime_observation && checkpoint.ordinal() >= LocalRunwayState::ConsentReady.ordinal() {
        checkpoint = checkpoint
            .advance(ferros_runtime::LocalRunwayIntent::Start)
            .expect("consent-ready -> runtime-ready should be valid");
    }

    if running_agent_count > 0 && checkpoint.ordinal() >= LocalRunwayState::RuntimeReady.ordinal() {
        checkpoint = checkpoint
            .advance(ferros_runtime::LocalRunwayIntent::Start)
            .expect("runtime-ready -> active should be valid");
    } else if checkpoint == LocalRunwayState::RuntimeReady
        && latest_lifecycle_marker(log_entries) == Some("stopped")
    {
        checkpoint = LocalRunwayState::Halted;
    }

    checkpoint
}

fn latest_lifecycle_marker(log_entries: &[String]) -> Option<&'static str> {
    log_entries.iter().rev().find_map(|entry| {
        if entry.starts_with("started:") {
            Some("started")
        } else if entry.starts_with("stopped:") {
            Some("stopped")
        } else {
            None
        }
    })
}

fn observe_local_runway_profile<S: LocalProfileStore>(
    store: &S,
    profile_path: &Path,
) -> LocalRunwayProfileObservation {
    match store.load_local_profile(profile_path) {
        Ok(state) => {
            let profile_name = state.profile.identity.name.clone();
            let grants = state
                .signed_grants
                .into_iter()
                .map(|signed_grant| grant_state_record(signed_grant.grant))
                .collect::<Vec<_>>();
            let grant_count = grants.len();
            let active_grant_count = grants.iter().filter(|grant| grant.is_active).count();
            let revoked_grant_count = grant_count.saturating_sub(active_grant_count);

            LocalRunwayProfileObservation {
                profile_name: Some(profile_name.clone()),
                grant_count,
                active_grant_count,
                revoked_grant_count,
                checklist_item: LocalRunwayChecklistItem {
                    item: "profileInit".to_owned(),
                    status: LocalRunwayChecklistStatus::Observed,
                    detail: format!("loaded {profile_name} with {grant_count} local grants"),
                },
            }
        }
        Err(ProfileStoreError::Io(error)) if error.kind() == io::ErrorKind::NotFound => {
            LocalRunwayProfileObservation {
                profile_name: None,
                grant_count: 0,
                active_grant_count: 0,
                revoked_grant_count: 0,
                checklist_item: LocalRunwayChecklistItem {
                    item: "profileInit".to_owned(),
                    status: LocalRunwayChecklistStatus::Pending,
                    detail: format!("no local profile observed at {}", profile_path.display()),
                },
            }
        }
        Err(error) => LocalRunwayProfileObservation {
            profile_name: None,
            grant_count: 0,
            active_grant_count: 0,
            revoked_grant_count: 0,
            checklist_item: LocalRunwayChecklistItem {
                item: "profileInit".to_owned(),
                status: LocalRunwayChecklistStatus::Error,
                detail: format!("failed to load local profile: {error}"),
            },
        },
    }
}

fn format_local_runway_deny_detail(deny: &LocalRunwayDenySummary) -> String {
    match (deny.agent_name.as_deref(), deny.capability.as_deref()) {
        (Some(agent_name), Some(capability)) => {
            format!("latest deny observed for {agent_name} on {capability}")
        }
        (Some(agent_name), None) => format!("latest deny observed for {agent_name}"),
        _ => format!("latest deny observed: {}", deny.message),
    }
}

fn execute_agent_read_rpc_with_store_and_paths<S: LocalProfileStore>(
    request: AgentJsonRpcRequest,
    state_path: &Path,
    default_profile_path: &Path,
    store: &S,
) -> Result<AgentJsonRpcResponse, CliError> {
    execute_agent_rpc_with_store_and_paths_and_runtime_loader(
        request,
        state_path,
        default_profile_path,
        store,
        |state, profile_path, store| {
            runtime_with_state_and_profile_path_from_loaded_state(state, profile_path, store)
        },
    )
}

fn execute_agent_rpc_with_store_and_paths_and_runtime_loader<S, RuntimeLoader>(
    request: AgentJsonRpcRequest,
    state_path: &Path,
    default_profile_path: &Path,
    store: &S,
    runtime_loader: RuntimeLoader,
) -> Result<AgentJsonRpcResponse, CliError>
where
    S: LocalProfileStore,
    RuntimeLoader: Fn(&CliState, &Path, &S) -> Result<DemoRuntime, CliError> + Copy,
{
    let AgentJsonRpcRequest {
        jsonrpc,
        id,
        method,
        params,
    } = request;

    if jsonrpc != ferros_agents::JSON_RPC_VERSION {
        return Ok(AgentJsonRpcResponse::error(
            id,
            JSON_RPC_INVALID_REQUEST,
            format!("unsupported JSON-RPC version: {jsonrpc}"),
        ));
    }

    match method.as_str() {
        METHOD_AGENT_LIST => {
            let state = CliState::load(state_path)?;
            let profile_path =
                requested_profile_path(params.profile_path.as_deref(), default_profile_path);
            let runtime = runtime_loader(&state, &profile_path, store)?;
            let agents = runtime
                .agent_records()
                .into_iter()
                .map(agent_record_to_rpc_summary)
                .collect();

            Ok(AgentJsonRpcResponse::success(
                id,
                AgentJsonRpcResult::AgentList { agents },
            ))
        }
        METHOD_AGENT_DESCRIBE => {
            let Some(agent_name) = params.agent_name.as_deref() else {
                return Ok(AgentJsonRpcResponse::error(
                    id,
                    JSON_RPC_INVALID_PARAMS,
                    "agentName parameter is required for agent.describe",
                ));
            };

            let state = CliState::load(state_path)?;
            let profile_path =
                requested_profile_path(params.profile_path.as_deref(), default_profile_path);
            let runtime = runtime_loader(&state, &profile_path, store)?;
            let Some(agent) = runtime.describe_agent(agent_name) else {
                return Ok(AgentJsonRpcResponse::error(
                    id,
                    JSON_RPC_AGENT_NOT_FOUND,
                    format!("agent not found: {agent_name}"),
                ));
            };

            Ok(AgentJsonRpcResponse::success(
                id,
                AgentJsonRpcResult::AgentDetail {
                    agent: agent_record_to_rpc_detail(agent),
                },
            ))
        }
        METHOD_AGENT_RUN | METHOD_AGENT_STOP => {
            let Some(agent_name) = params.agent_name.as_deref() else {
                return Ok(AgentJsonRpcResponse::error(
                    id,
                    JSON_RPC_INVALID_PARAMS,
                    format!("agentName parameter is required for {method}"),
                ));
            };

            let command = if method == METHOD_AGENT_RUN {
                LocalAgentApiCommand::Run {
                    name: agent_name.to_owned(),
                }
            } else {
                LocalAgentApiCommand::Stop {
                    name: agent_name.to_owned(),
                }
            };
            let profile_path =
                requested_profile_path(params.profile_path.as_deref(), default_profile_path);

            match execute_local_agent_lifecycle_with_profile_path(
                command,
                state_path,
                &profile_path,
                store,
                runtime_loader,
            ) {
                Ok(LocalAgentApiResponse::AgentLifecycle { agent }) => {
                    Ok(AgentJsonRpcResponse::success(
                        id,
                        AgentJsonRpcResult::AgentLifecycle {
                            agent: agent_record_to_rpc_detail(agent),
                        },
                    ))
                }
                Ok(other) => Err(CliError::InvalidState(format!(
                    "unexpected local agent API response for {method}: {other:?}"
                ))),
                Err(CliError::Runtime(DemoError::UnknownAgent(name))) => {
                    Ok(AgentJsonRpcResponse::error(
                        id,
                        JSON_RPC_AGENT_NOT_FOUND,
                        format!("agent not found: {name}"),
                    ))
                }
                Err(CliError::Runtime(DemoError::AuthorizationDenied(detail))) => {
                    Ok(AgentJsonRpcResponse::error(
                        id,
                        JSON_RPC_AUTHORIZATION_DENIED,
                        format!("authorization denied: {}", detail.summary),
                    ))
                }
                Err(error) => Err(error),
            }
        }
        METHOD_AGENT_SNAPSHOT => {
            let state = CliState::load(state_path)?;
            let profile_path =
                requested_profile_path(params.profile_path.as_deref(), default_profile_path);
            let runtime = runtime_loader(&state, &profile_path, store)?;
            let agent_name = params.agent_name.as_deref();
            let agents = if let Some(agent_name) = agent_name {
                let Some(agent) = runtime.describe_agent(agent_name) else {
                    return Ok(AgentJsonRpcResponse::error(
                        id,
                        JSON_RPC_AGENT_NOT_FOUND,
                        format!("agent not found: {agent_name}"),
                    ));
                };

                vec![agent_record_to_rpc_detail(agent)]
            } else {
                runtime
                    .agent_records()
                    .into_iter()
                    .map(agent_record_to_rpc_detail)
                    .collect()
            };
            let grants = load_grant_state_records(store, &profile_path)?;
            let deny_log = deny_log_entries(&state, agent_name);

            Ok(AgentJsonRpcResponse::success(
                id,
                AgentJsonRpcResult::AgentSnapshot {
                    snapshot: AgentRpcSnapshot {
                        agents,
                        grants,
                        deny_log,
                    },
                },
            ))
        }
        METHOD_GRANT_LIST => {
            let profile_path =
                requested_profile_path(params.profile_path.as_deref(), default_profile_path);
            let grants = load_grant_state_records(store, &profile_path)?;

            Ok(AgentJsonRpcResponse::success(
                id,
                AgentJsonRpcResult::GrantList { grants },
            ))
        }
        METHOD_DENY_LOG_LIST => {
            let state = CliState::load(state_path)?;
            let entries = deny_log_entries(&state, params.agent_name.as_deref());

            Ok(AgentJsonRpcResponse::success(
                id,
                AgentJsonRpcResult::DenyLog { entries },
            ))
        }
        _ => Ok(AgentJsonRpcResponse::error(
            id,
            JSON_RPC_METHOD_NOT_FOUND,
            format!("unknown JSON-RPC method: {method}"),
        )),
    }
}

#[cfg(test)]
fn runtime_with_state(state_path: &Path) -> Result<DemoRuntime, CliError> {
    let state = CliState::load(state_path)?;
    runtime_with_state_from_loaded_state(&state)
}

fn runtime_with_state_from_loaded_state(state: &CliState) -> Result<DemoRuntime, CliError> {
    let mut runtime = DemoRuntime::reference_host().map_err(CliError::from)?;
    runtime.replay_cli_state(state)?;
    Ok(runtime)
}

fn runtime_with_state_and_profile_path_from_loaded_state<S: LocalProfileStore>(
    state: &CliState,
    profile_path: &Path,
    store: &S,
) -> Result<DemoRuntime, CliError> {
    match store.load_local_profile(profile_path) {
        Ok(local_profile) => {
            let profile_id = local_profile.key_pair.profile_id();
            let grants = local_profile
                .signed_grants
                .into_iter()
                .map(|signed_grant| signed_grant.grant)
                .collect();
            let mut runtime =
                DemoRuntime::reference_host_with_profile_id_and_grants(profile_id, grants)
                    .map_err(CliError::from)?;
            runtime.replay_cli_state(state)?;
            Ok(runtime)
        }
        Err(ProfileStoreError::Io(error)) if error.kind() == io::ErrorKind::NotFound => {
            let mut runtime = DemoRuntime::reference_host().map_err(CliError::from)?;
            runtime.replay_cli_state(state)?;
            Ok(runtime)
        }
        Err(error) => Err(CliError::from(error)),
    }
}

fn cli_state_path() -> PathBuf {
    std::env::temp_dir()
        .join(CLI_STATE_DIRECTORY)
        .join(CLI_STATE_FILE)
}

fn monitor_state_path() -> PathBuf {
    if let Some(explicit_path) = std::env::var_os("FERROS_MONITOR_STATE_PATH") {
        let explicit_path = PathBuf::from(explicit_path);
        if !explicit_path.as_os_str().is_empty() {
            return explicit_path;
        }
    }

    profile_home_dir()
        .unwrap_or_else(std::env::temp_dir)
        .join(CLI_PROFILE_DIRECTORY)
        .join(MONITOR_STATE_FILE)
}

pub fn default_profile_path() -> PathBuf {
    if let Some(explicit_path) = std::env::var_os("FERROS_PROFILE_PATH") {
        let explicit_path = PathBuf::from(explicit_path);

        if !explicit_path.as_os_str().is_empty() {
            return explicit_path;
        }
    }

    profile_home_dir()
        .unwrap_or_else(std::env::temp_dir)
        .join(CLI_PROFILE_DIRECTORY)
        .join(CLI_PROFILE_FILE)
}

fn profile_home_dir() -> Option<PathBuf> {
    std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from)
}

struct LocalAgentController<'a> {
    state_path: &'a Path,
    runtime_loader: fn(&CliState) -> Result<DemoRuntime, CliError>,
}

impl<'a> LocalAgentController<'a> {
    fn new(
        state_path: &'a Path,
        runtime_loader: fn(&CliState) -> Result<DemoRuntime, CliError>,
    ) -> Self {
        Self {
            state_path,
            runtime_loader,
        }
    }

    fn execute(&self, command: LocalAgentApiCommand) -> Result<LocalAgentApiResponse, CliError> {
        match command {
            LocalAgentApiCommand::List => self.list(),
            LocalAgentApiCommand::Describe { name } => self.describe(&name),
            LocalAgentApiCommand::Run { name } => self.run(&name),
            LocalAgentApiCommand::Stop { name } => self.stop(&name),
            LocalAgentApiCommand::Logs { name } => self.logs(name.as_deref()),
        }
    }

    fn list(&self) -> Result<LocalAgentApiResponse, CliError> {
        let state = CliState::load(self.state_path)?;
        let runtime = (self.runtime_loader)(&state)?;

        Ok(LocalAgentApiResponse::AgentList {
            agents: runtime.agent_records(),
        })
    }

    fn describe(&self, name: &str) -> Result<LocalAgentApiResponse, CliError> {
        let state = CliState::load(self.state_path)?;
        let runtime = (self.runtime_loader)(&state)?;
        let agent = runtime
            .describe_agent(name)
            .ok_or_else(|| DemoError::UnknownAgent(name.to_owned()))?;

        Ok(LocalAgentApiResponse::AgentDetail { agent })
    }

    fn run(&self, name: &str) -> Result<LocalAgentApiResponse, CliError> {
        let mut state = CliState::load(self.state_path)?;
        let mut runtime = (self.runtime_loader)(&state)?;
        let record = runtime
            .describe_agent(name)
            .ok_or_else(|| DemoError::UnknownAgent(name.to_owned()))?;

        if record.status != AgentStatus::Running {
            let start_result = runtime.start_agent(name);

            if !runtime.log_entries().is_empty() {
                state
                    .log_entries
                    .extend(runtime.log_entries().iter().cloned());
            }

            match start_result {
                Ok(()) => {
                    state.set_status(name, AgentStatus::Running);
                    state.save(self.state_path)?;
                }
                Err(error) => {
                    if !runtime.log_entries().is_empty() {
                        state.save(self.state_path)?;
                    }
                    return Err(error.into());
                }
            }
        }

        let agent = runtime
            .describe_agent(name)
            .ok_or_else(|| DemoError::UnknownAgent(name.to_owned()))?;

        Ok(LocalAgentApiResponse::AgentLifecycle { agent })
    }

    fn stop(&self, name: &str) -> Result<LocalAgentApiResponse, CliError> {
        let mut state = CliState::load(self.state_path)?;
        let mut runtime = (self.runtime_loader)(&state)?;
        runtime
            .describe_agent(name)
            .ok_or_else(|| DemoError::UnknownAgent(name.to_owned()))?;

        if runtime
            .describe_agent(name)
            .map(|record| record.status)
            .ok_or_else(|| DemoError::UnknownAgent(name.to_owned()))?
            != AgentStatus::Stopped
        {
            runtime.stop_agent(name)?;
            state.set_status(name, AgentStatus::Stopped);
            state
                .log_entries
                .extend(runtime.log_entries().iter().cloned());
            state.save(self.state_path)?;
        }

        let agent = runtime
            .describe_agent(name)
            .ok_or_else(|| DemoError::UnknownAgent(name.to_owned()))?;

        Ok(LocalAgentApiResponse::AgentLifecycle { agent })
    }

    fn logs(&self, name: Option<&str>) -> Result<LocalAgentApiResponse, CliError> {
        let state = CliState::load(self.state_path)?;
        let entries = if let Some(name) = name {
            state
                .log_entries
                .into_iter()
                .filter(|entry| entry.contains(name))
                .collect::<Vec<_>>()
        } else {
            state.log_entries
        };

        Ok(LocalAgentApiResponse::AgentLogs { entries })
    }
}

fn execute_local_agent_api_with_state_path(
    command: LocalAgentApiCommand,
    state_path: &Path,
) -> Result<LocalAgentApiResponse, CliError> {
    execute_local_agent_api_with_runtime_loader(
        command,
        state_path,
        runtime_with_state_from_loaded_state,
    )
}

fn execute_local_agent_api_with_runtime_loader(
    command: LocalAgentApiCommand,
    state_path: &Path,
    runtime_loader: fn(&CliState) -> Result<DemoRuntime, CliError>,
) -> Result<LocalAgentApiResponse, CliError> {
    LocalAgentController::new(state_path, runtime_loader).execute(command)
}

fn execute_local_agent_lifecycle_with_profile_path<S, RuntimeLoader>(
    command: LocalAgentApiCommand,
    state_path: &Path,
    profile_path: &Path,
    store: &S,
    runtime_loader: RuntimeLoader,
) -> Result<LocalAgentApiResponse, CliError>
where
    S: LocalProfileStore,
    RuntimeLoader: Fn(&CliState, &Path, &S) -> Result<DemoRuntime, CliError> + Copy,
{
    match command {
        LocalAgentApiCommand::Run { name } => {
            let mut state = CliState::load(state_path)?;
            let mut runtime = runtime_loader(&state, profile_path, store)?;
            let record = runtime
                .describe_agent(&name)
                .ok_or_else(|| DemoError::UnknownAgent(name.clone()))?;

            if record.status != AgentStatus::Running {
                let start_result = runtime.start_agent(&name);

                if !runtime.log_entries().is_empty() {
                    state
                        .log_entries
                        .extend(runtime.log_entries().iter().cloned());
                }

                match start_result {
                    Ok(()) => {
                        state.set_status(&name, AgentStatus::Running);
                        state.save(state_path)?;
                    }
                    Err(error) => {
                        if !runtime.log_entries().is_empty() {
                            state.save(state_path)?;
                        }
                        return Err(error.into());
                    }
                }
            }

            let agent = runtime
                .describe_agent(&name)
                .ok_or_else(|| DemoError::UnknownAgent(name.clone()))?;

            Ok(LocalAgentApiResponse::AgentLifecycle { agent })
        }
        LocalAgentApiCommand::Stop { name } => {
            let mut state = CliState::load(state_path)?;
            let mut runtime = runtime_loader(&state, profile_path, store)?;
            runtime
                .describe_agent(&name)
                .ok_or_else(|| DemoError::UnknownAgent(name.clone()))?;

            if runtime
                .describe_agent(&name)
                .map(|record| record.status)
                .ok_or_else(|| DemoError::UnknownAgent(name.clone()))?
                != AgentStatus::Stopped
            {
                runtime.stop_agent(&name)?;
                state.set_status(&name, AgentStatus::Stopped);
                state
                    .log_entries
                    .extend(runtime.log_entries().iter().cloned());
                state.save(state_path)?;
            }

            let agent = runtime
                .describe_agent(&name)
                .ok_or_else(|| DemoError::UnknownAgent(name.clone()))?;

            Ok(LocalAgentApiResponse::AgentLifecycle { agent })
        }
        other => Err(CliError::InvalidState(format!(
            "unexpected profile-scoped lifecycle command: {other:?}"
        ))),
    }
}

fn execute_agent_cli_with_state_path(
    command: AgentCliCommand,
    state_path: &Path,
) -> Result<Vec<String>, CliError> {
    execute_agent_cli_with_runtime_loader(command, state_path, runtime_with_state_from_loaded_state)
}

fn execute_agent_cli_with_runtime_loader(
    command: AgentCliCommand,
    state_path: &Path,
    runtime_loader: fn(&CliState) -> Result<DemoRuntime, CliError>,
) -> Result<Vec<String>, CliError> {
    execute_local_agent_api_with_runtime_loader(command.into(), state_path, runtime_loader)
        .map(format_local_agent_api_response)
}

fn execute_profile_cli_with_store<S: LocalProfileStore>(
    command: ProfileCliCommand,
    store: &S,
) -> Result<Vec<String>, CliError> {
    match command {
        ProfileCliCommand::Init { path } => {
            let state = init_local_profile(
                store,
                &path,
                DEFAULT_PROFILE_NAME,
                current_profile_timestamp(),
                DEFAULT_PROFILE_DEVICE_LABEL,
            )?;

            Ok(vec![
                format!("initialized profile at {}", path.display()),
                format!("profile id: {}", state.key_pair.profile_id().as_str()),
                format!("profile name: {}", state.profile.identity.name),
            ])
        }
        ProfileCliCommand::Show { path } => {
            let profile = store.load_profile(&path)?;
            let rendered = profile
                .to_json_string_pretty()
                .map_err(ProfileStoreError::Serde)?;

            Ok(rendered.lines().map(str::to_owned).collect())
        }
        ProfileCliCommand::Export { path, bundle_path } => {
            store.export_profile_bundle(&path, &bundle_path)?;

            Ok(vec![format!(
                "exported profile bundle to {}",
                bundle_path.display()
            )])
        }
        ProfileCliCommand::Import { path, bundle_path } => {
            let state = store.import_profile_bundle(&bundle_path, &path)?;

            Ok(vec![
                format!("imported profile at {}", path.display()),
                format!("profile id: {}", state.key_pair.profile_id().as_str()),
                format!("grant count: {}", state.signed_grants.len()),
            ])
        }
        ProfileCliCommand::Grant { path, capability } => {
            let signed_grant = grant_profile_capability(store, &path, capability)?;

            Ok(vec![format!(
                "granted {} to {}",
                signed_grant.grant.capability,
                signed_grant.grant.profile_id.as_str()
            )])
        }
        ProfileCliCommand::Revoke { path, capability } => {
            let signed_grant = revoke_profile_capability(
                store,
                &path,
                &capability,
                current_profile_timestamp(),
                PROFILE_REVOKE_REASON,
            )?;

            Ok(vec![format!(
                "revoked {} for {}",
                signed_grant.grant.capability,
                signed_grant.grant.profile_id.as_str()
            )])
        }
    }
}

fn current_profile_timestamp() -> String {
    OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .unwrap_or_else(|_| {
            let seconds = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();

            format!("1970-01-01T00:00:{:02}Z", seconds % 60)
        })
}

fn format_agent_summary(record: &AgentRecord) -> String {
    format!(
        "{}\t{}\t{}",
        record.manifest.name.as_str(),
        record.manifest.version,
        format_agent_status(record.status)
    )
}

fn format_local_agent_api_response(response: LocalAgentApiResponse) -> Vec<String> {
    match response {
        LocalAgentApiResponse::AgentList { agents } => {
            let mut lines = vec!["name\tversion\tstatus".to_owned()];
            lines.extend(agents.iter().map(format_agent_summary));
            lines
        }
        LocalAgentApiResponse::AgentDetail { agent } => format_agent_description(&agent),
        LocalAgentApiResponse::AgentLifecycle { agent } => vec![format_agent_lifecycle(&agent)],
        LocalAgentApiResponse::AgentLogs { entries } => {
            if entries.is_empty() {
                vec!["no log entries".to_owned()]
            } else {
                entries
            }
        }
    }
}

fn format_agent_lifecycle(record: &AgentRecord) -> String {
    format!(
        "{}\t{}",
        record.manifest.name.as_str(),
        format_agent_status(record.status)
    )
}

fn format_agent_description(record: &AgentRecord) -> Vec<String> {
    let mut lines = vec![
        format!("name: {}", record.manifest.name.as_str()),
        format!("version: {}", record.manifest.version),
        format!("status: {}", format_agent_status(record.status)),
    ];

    if record.manifest.required_capabilities.is_empty() {
        lines.push("required capabilities: none".to_owned());
        return lines;
    }

    lines.push("required capabilities:".to_owned());
    lines.extend(
        record
            .manifest
            .required_capabilities
            .iter()
            .map(|requirement| {
                format!(
                    "- {}:{}",
                    requirement.profile_id.as_str(),
                    requirement.capability
                )
            }),
    );
    lines
}

fn format_agent_status(status: AgentStatus) -> &'static str {
    match status {
        AgentStatus::Registered => "registered",
        AgentStatus::Starting => "starting",
        AgentStatus::Running => "running",
        AgentStatus::Paused => "paused",
        AgentStatus::Stopping => "stopping",
        AgentStatus::Stopped => "stopped",
        AgentStatus::Failed => "failed",
    }
}

fn parse_agent_status(value: &str) -> Option<AgentStatus> {
    match value {
        "registered" => Some(AgentStatus::Registered),
        "starting" => Some(AgentStatus::Starting),
        "running" => Some(AgentStatus::Running),
        "paused" => Some(AgentStatus::Paused),
        "stopping" => Some(AgentStatus::Stopping),
        "stopped" => Some(AgentStatus::Stopped),
        "failed" => Some(AgentStatus::Failed),
        _ => None,
    }
}

fn agent_record_to_rpc_summary(record: AgentRecord) -> AgentRpcAgentSummary {
    AgentRpcAgentSummary {
        name: record.manifest.name.as_str().to_owned(),
        version: record.manifest.version,
        status: format_agent_status(record.status).to_owned(),
    }
}

fn agent_record_to_rpc_detail(record: AgentRecord) -> AgentRpcAgentDetail {
    AgentRpcAgentDetail {
        name: record.manifest.name.as_str().to_owned(),
        version: record.manifest.version,
        status: format_agent_status(record.status).to_owned(),
        required_capabilities: record.manifest.required_capabilities,
    }
}

fn load_grant_state_records<S: LocalProfileStore>(
    store: &S,
    profile_path: &Path,
) -> Result<Vec<GrantStateRecord>, CliError> {
    match store.load_local_profile(profile_path) {
        Ok(state) => Ok(state
            .signed_grants
            .into_iter()
            .map(|signed_grant| grant_state_record(signed_grant.grant))
            .collect()),
        Err(ProfileStoreError::Io(error)) if error.kind() == io::ErrorKind::NotFound => {
            Ok(Vec::new())
        }
        Err(error) => Err(error.into()),
    }
}

fn grant_state_record(grant: CapabilityGrant) -> GrantStateRecord {
    let is_active = !grant.is_revoked();

    GrantStateRecord {
        profile_id: grant.profile_id.as_str().to_owned(),
        capability: grant.capability,
        is_active,
        revoked_at: grant.revoked_at,
        revocation_reason: grant.revocation_reason,
    }
}

fn deny_log_entries(state: &CliState, agent_name: Option<&str>) -> Vec<DenyLogEntry> {
    state
        .log_entries
        .iter()
        .enumerate()
        .filter_map(|(index, entry)| parse_deny_log_entry(index + 1, entry))
        .filter(|entry| agent_name.is_none() || entry.agent_name.as_deref() == agent_name)
        .collect()
}

fn parse_deny_log_entry(entry_id: usize, entry: &str) -> Option<DenyLogEntry> {
    if let Some(message) = entry.strip_prefix("denied-start:") {
        let (agent_name, capability) = match message.split_once(" missing ") {
            Some((agent_name, capability)) => {
                (Some(agent_name.to_owned()), Some(capability.to_owned()))
            }
            None => (None, None),
        };

        return Some(DenyLogEntry {
            entry_id,
            kind: "deniedStart".to_owned(),
            message: message.to_owned(),
            agent_name,
            capability,
        });
    }

    let message = entry.strip_prefix("denied:")?;
    let (agent_name, capability) = match message.split_once(':') {
        Some((agent_name, remainder)) => match remainder.split_once(':') {
            Some((capability, _)) => (Some(agent_name.to_owned()), Some(capability.to_owned())),
            None => (Some(agent_name.to_owned()), None),
        },
        None => (None, None),
    };

    Some(DenyLogEntry {
        entry_id,
        kind: "denied".to_owned(),
        message: message.to_owned(),
        agent_name,
        capability,
    })
}

pub fn run_demo() -> Result<DemoSummary, DemoError> {
    let mut runtime = DemoRuntime::reference_host()?;
    runtime.run_reference_demo_cycle()
}

#[cfg(test)]
mod tests {
    use super::{
        build_local_runway_summary_with_store_and_hub_summary_loader, default_profile_path,
        execute_agent_cli_with_runtime_loader, execute_agent_cli_with_state_path,
        execute_agent_read_rpc_json, execute_agent_read_rpc_with_store_and_paths,
        execute_agent_rpc_with_store_and_paths_and_runtime_loader,
        execute_local_agent_api_with_runtime_loader, execute_profile_cli_with_store,
        infer_dispatch_target, monitor_now, parse_http_request,
        route_monitor_request_with_state, route_shell_request_with_store_and_paths,
        run_demo, runtime_with_state, serve_local_shell_with_listener,
        serve_local_shell_with_store_and_paths, AgentCliCommand, AuthorizationDenyDetail,
        CliError, CliState, DemoError, DemoRuntime, DispatchTarget, HttpRequest, LocalAgentApi,
        LocalAgentApiCommand, LocalAgentApiResponse, LocalRunwayChecklistStatus,
        LocalRunwaySummary, MonitorLifecycleThread, MonitorMessageRequest, MonitorState,
        PersistedMonitorState, ProfileCliCommand,
        ProfileShellResponse, ScaffoldMonitorDispatchBackend,
        DEFAULT_PROFILE_NAME,
    };
    use ferros_agents::{
        AgentJsonRpcParams, AgentJsonRpcRequest, AgentJsonRpcResponse, AgentJsonRpcResult,
        AgentStatus, EchoAgent, JSON_RPC_AGENT_NOT_FOUND, JSON_RPC_AUTHORIZATION_DENIED,
        JSON_RPC_INVALID_PARAMS, JSON_RPC_INVALID_REQUEST, JSON_RPC_METHOD_NOT_FOUND,
        METHOD_AGENT_DESCRIBE, METHOD_AGENT_LIST, METHOD_AGENT_RUN, METHOD_AGENT_SNAPSHOT,
        METHOD_AGENT_STOP, METHOD_DENY_LOG_LIST, METHOD_GRANT_LIST,
    };
    use ferros_hub::{
        default_local_runtime_summary, LocalBridgeRegistrationError, LocalHubRuntimeSummary,
        LOCAL_HUB_STATE_SNAPSHOT_PATH, SIMULATED_LOCAL_BRIDGE_ARTIFACT_PATH,
    };
    use ferros_profile::{CapabilityGrant, FileSystemProfileStore, ProfileDocument, ProfileId};
    use std::fs;
    use std::io::{Read, Write};
    use std::net::{Shutdown, TcpListener, TcpStream};
    use std::path::{Path, PathBuf};
    use std::thread;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn demo_runs_deterministically_and_denies_unauthorized_work() {
        let summary = run_demo().expect("demo should succeed");

        assert_eq!(
            summary.started_agents,
            vec!["echo".to_string(), "ha-local-bridge".to_string(), "timer".to_string()]
        );
        assert_eq!(summary.echo_response, "hello");
        assert_eq!(summary.timer_event, "tick-1");
        assert_eq!(summary.denied_requests, 1);
        assert!(summary
            .log_entries
            .iter()
            .any(|entry| entry.starts_with("denied:")));
    }

    #[test]
    fn demo_host_bootstraps_reference_agents_and_reuses_demo_cycle() {
        let mut runtime = DemoRuntime::reference_host().expect("reference host should build");

        assert_eq!(
            runtime.list_agents(),
            vec![
                "echo".to_string(),
                "ha-local-bridge".to_string(),
                "timer".to_string()
            ]
        );

        let summary = runtime
            .run_reference_demo_cycle()
            .expect("reference demo cycle should succeed");

        assert_eq!(summary.echo_response, "hello");
        assert_eq!(summary.timer_event, "tick-1");
        assert_eq!(summary.denied_requests, 1);
    }

    #[test]
    fn runtime_lists_registered_reference_agents() {
        let runtime = DemoRuntime::reference_host().expect("reference host should build");

        assert_eq!(
            runtime.list_agents(),
            vec![
                "echo".to_string(),
                "ha-local-bridge".to_string(),
                "timer".to_string()
            ]
        );
    }

    #[test]
    fn runtime_describes_local_bridge_stand_in_agent() {
        let runtime = DemoRuntime::reference_host().expect("reference host should build");

        let agent = runtime
            .describe_agent("ha-local-bridge")
            .expect("local bridge stand-in should be registered");

        assert_eq!(agent.manifest.name.as_str(), "ha-local-bridge");
        assert_eq!(agent.manifest.version, "0.1.0");
        assert_eq!(agent.manifest.required_capabilities.len(), 1);
        assert_eq!(agent.manifest.required_capabilities[0].capability, "bridge.observe");
        assert_eq!(
            agent.manifest.required_capabilities[0].profile_id.as_str(),
            "hub-local-bridge"
        );
        assert_eq!(agent.status, AgentStatus::Registered);
    }

    #[test]
    fn infer_dispatch_target_maps_keywords_to_expected_lane() {
        assert_eq!(infer_dispatch_target("please send to business"), DispatchTarget::Business);
        assert_eq!(
            infer_dispatch_target("route this through coding architect"),
            DispatchTarget::CodingArchitect
        );
        assert_eq!(
            infer_dispatch_target("need ferros architect review"),
            DispatchTarget::FerrosArchitect
        );
        assert_eq!(
            infer_dispatch_target("general software work order"),
            DispatchTarget::Software
        );
    }

    #[test]
    fn routing_session_does_not_archive_liaison_chat() {
        let mut state = MonitorState::default();
        let session = state.create_session(Some("User -> FERROS".to_owned()));

        assert!(state.route_session(&session.id, "business"));
        assert!(state.open_chats.iter().any(|chat| chat.id == session.id));
        assert!(state.archived_chats.is_empty());
    }

    #[test]
    fn notification_actions_update_status() {
        let mut state = MonitorState::default();
        let notification_id = state.create_notification(
            Some("wo-1".to_owned()),
            None,
            None,
            "high",
            "Human intervention required",
            "Pause and ask Administration",
        );

        assert!(state.update_notification_status(&notification_id, "opened"));
        assert_eq!(state.notifications[0].status, "opened");
        assert!(state.update_notification_status(&notification_id, "resolved"));
        assert_eq!(state.notifications[0].status, "resolved");
    }

    #[test]
    fn notification_open_focuses_linked_session_and_thread() {
        let mut state = MonitorState::default();
        let session = state.create_session(Some("FERROS Agent".to_owned()));

        // Push a minimal lifecycle thread directly.
        let thread_id = "thr-test-01".to_owned();
        state.lifecycle_threads.push(MonitorLifecycleThread {
            id: thread_id.clone(),
            title: "Test thread".to_owned(),
            kind: "packet".to_owned(),
            status: "running".to_owned(),
            owner_agent: "Software Architect".to_owned(),
            source_agent_id: None,
            target_agent_id: None,
            work_order_id: None,
            escalation_id: None,
            created_at: monitor_now(),
            updated_at: monitor_now(),
            entries: vec![],
        });

        let notification_id = state.create_notification(
            None,
            Some(session.id.clone()),
            Some(thread_id.clone()),
            "high",
            "Needs operator",
            "FERROS escalated",
        );

        // Before open: notification should be unread; lifecycle thread not yet selected.
        assert_eq!(state.notifications[0].status, "unread");
        // Clear selection state so we can verify open_notification applies notification linkage.
        state.selected_chat_id = None;
        state.selected_lifecycle_thread_id = None;

        let opened = state.open_notification(&notification_id);
        assert!(opened, "open_notification should return true");
        assert_eq!(state.notifications[0].status, "opened");
        assert_eq!(state.selected_chat_id.as_deref(), Some(session.id.as_str()));
        assert_eq!(state.selected_lifecycle_thread_id.as_deref(), Some(thread_id.as_str()));
    }

    #[test]
    fn notification_open_unknown_id_returns_false() {
        let mut state = MonitorState::default();
        assert!(!state.open_notification("ntf-does-not-exist"));
    }

    #[test]
    fn dispatch_creates_packet_and_lifecycle_thread() {
        let mut state = MonitorState::default();
        let session = state.create_session(Some("Admin liaison".to_owned()));
        let origin_message_id = state.add_message(
            &session.id,
            super::MonitorMessageRequest {
                speaker: "user".to_owned(),
                who: "Human".to_owned(),
                text: "please route this to software".to_owned(),
            },
        )
        .expect("user message should be added");

        let result = state.ferros_agent_handle_human_message(
            &session.id,
            &origin_message_id,
            "please route this to software",
        );

        // Dispatch should succeed and produce a packet + lifecycle thread
        assert!(
            matches!(result.status, super::MonitorDispatchStatus::Routed),
            "expected Routed, got {:?}",
            result.status
        );
        assert!(result.packet_id.is_some(), "packet_id should be set");
        assert!(result.lifecycle_thread_id.is_some(), "lifecycle_thread_id should be set");

        // Packet should appear in state
        let packet_id = result.packet_id.unwrap();
        let packet = state.packet_by_id(&packet_id);
        assert!(packet.is_some(), "packet should be registered in state");
        let packet = packet.unwrap();
        assert_eq!(packet.state, "dispatched_to_manager");
        assert_eq!(packet.session_id, session.id);
        assert_eq!(packet.origin_message_id, origin_message_id);
        assert!(packet.work_order_id.is_some(), "packet should carry work order id");

        // Lifecycle thread should exist
        let thread_id = result.lifecycle_thread_id.unwrap();
        assert!(
            state.lifecycle_threads.iter().any(|t| t.id == thread_id),
            "lifecycle thread should exist"
        );
    }

    #[test]
    fn packet_state_is_persisted_in_snapshot_roundtrip() {
        let mut state = MonitorState::default();
        let session = state.create_session(Some("Snapshot test".to_owned()));
        let packet_id = state.create_packet(
            session.id.clone(),
            "msg-origin".to_owned(),
            None,
            "Software Architect".to_owned(),
            "dispatched_to_manager",
            None,
            None,
            "Test summary".to_owned(),
        );

        let payload = PersistedMonitorState {
            schema_version: super::MONITOR_STATE_SCHEMA_VERSION,
            state,
        };
        let persisted =
            serde_json::to_string_pretty(&payload).expect("should serialize");
        let loaded: PersistedMonitorState =
            serde_json::from_str(&persisted).expect("should deserialize");

        let packet = loaded.state.packets.iter().find(|p| p.id == packet_id);
        assert!(packet.is_some(), "packet should survive roundtrip");
        assert_eq!(packet.unwrap().state, "dispatched_to_manager");
    }

    #[test]
    fn human_intervention_notification_links_to_packet_when_available() {
        let mut state = MonitorState::default();
        let session = state.create_session(Some("Admin liaison".to_owned()));

        let result = state.ferros_agent_handle_human_message(
            &session.id,
            "msg-1",
            "escalate this immediately",
        );

        assert!(
            matches!(result.status, super::MonitorDispatchStatus::HumanInterventionRequired),
            "expected HumanInterventionRequired"
        );
        assert!(result.packet_id.is_some(), "packet_id should be set on escalation");
        assert!(result.notification_id.is_some(), "notification_id should be set");

        let packet_id = result.packet_id.unwrap();
        let notification_id = result.notification_id.unwrap();

        let packet = state.packet_by_id(&packet_id).expect("packet should exist");
        assert_eq!(packet.state, "human_intervention_required");
        assert_eq!(packet.notification_id.as_deref(), Some(notification_id.as_str()));

        let notification = state
            .notifications
            .iter()
            .find(|n| n.id == notification_id)
            .expect("notification should exist");
        assert_eq!(notification.packet_id.as_deref(), Some(packet_id.as_str()));
    }

    // -----------------------------------------------------------------------
    // Sprint 3 — route-level HTTP tests
    // -----------------------------------------------------------------------

    fn make_state() -> std::sync::Mutex<MonitorState> {
        std::sync::Mutex::new(MonitorState::default())
    }

    fn body(json: serde_json::Value) -> Vec<u8> {
        serde_json::to_vec(&json).expect("test JSON serialization should not fail")
    }

    #[test]
    fn route_create_session_returns_session_with_id_and_ferros_agent() {
        let state = make_state();
        let response = route_monitor_request_with_state(
            "POST",
            "/monitor/sessions",
            body(serde_json::json!({})),
            &state,
        )
        .expect("route should return a response");

        assert_eq!(response.status_code, 200);
        let value: serde_json::Value =
            serde_json::from_slice(&response.body).expect("body should be valid JSON");
        assert!(value["id"].as_str().map(|id| !id.is_empty()).unwrap_or(false), "session id should be set");
        assert_eq!(value["activeAgent"].as_str(), Some("FERROS Agent"));
        let first_msg_id = value["messages"][0]["id"].as_str();
        assert!(first_msg_id.map(|id| !id.is_empty()).unwrap_or(false), "first message id should be set");
    }

    #[test]
    fn route_send_message_keeps_chat_open_and_creates_packet() {
        let state = make_state();
        // First create a session
        let create_resp = route_monitor_request_with_state(
            "POST",
            "/monitor/sessions",
            body(serde_json::json!({})),
            &state,
        )
        .expect("create session should return a response");
        let session: serde_json::Value =
            serde_json::from_slice(&create_resp.body).expect("body should be valid JSON");
        let session_id = session["id"].as_str().expect("session id should be a string").to_owned();

        // Send a user message
        let msg_resp = route_monitor_request_with_state(
            "POST",
            &format!("/monitor/sessions/{session_id}/messages"),
            body(serde_json::json!({
                "speaker": "user",
                "who": "Human",
                "text": "please route to software"
            })),
            &state,
        )
        .expect("send message should return a response");

        assert_eq!(msg_resp.status_code, 200);
        let snapshot: serde_json::Value =
            serde_json::from_slice(&msg_resp.body).expect("body should be valid JSON");

        // Chat should still be open, not archived
        let empty = vec![];
        let open_ids: Vec<&str> = snapshot["openChats"]
            .as_array()
            .unwrap_or(&empty)
            .iter()
            .filter_map(|c| c["id"].as_str())
            .collect();
        assert!(open_ids.contains(&session_id.as_str()), "session should remain open");

        let archived_ids: Vec<&str> = snapshot["archivedChats"]
            .as_array()
            .unwrap_or(&empty)
            .iter()
            .filter_map(|c| c["id"].as_str())
            .collect();
        assert!(!archived_ids.contains(&session_id.as_str()), "session should not be archived");

        // FERROS Agent should have replied
        let messages = snapshot["openChats"]
            .as_array()
            .and_then(|chats| chats.iter().find(|c| c["id"].as_str() == Some(&session_id)))
            .and_then(|chat| chat["messages"].as_array())
            .cloned()
            .unwrap_or_default();
        let has_ferros_reply = messages.iter().any(|m| m["who"].as_str() == Some("FERROS Agent"));
        assert!(has_ferros_reply, "FERROS Agent should have replied");

        // A packet should have been created
        let packets = snapshot["packets"].as_array().cloned().unwrap_or_default();
        assert!(!packets.is_empty(), "at least one packet should be created");
    }

    #[test]
    fn route_notification_open_updates_selection() {
        let state = make_state();
        // Create a session and a notification linked to it
        let session = {
            let mut guard = state.lock().unwrap();
            guard.create_session(None)
        };
        let notification_id = {
            let mut guard = state.lock().unwrap();
            guard.create_notification(
                None,
                Some(session.id.clone()),
                None,
                "high",
                "Test",
                "Test summary",
            )
        };

        let response = route_monitor_request_with_state(
            "POST",
            &format!("/monitor/notifications/{notification_id}/open"),
            vec![],
            &state,
        )
        .expect("route should return a response");

        assert_eq!(response.status_code, 200);
        let snapshot: serde_json::Value =
            serde_json::from_slice(&response.body).expect("body should be valid JSON");
        assert_eq!(
            snapshot["selectedChatId"].as_str(),
            Some(session.id.as_str()),
            "selectedChatId should be updated to linked session"
        );
    }

    #[test]
    fn route_notification_unknown_action_returns_400() {
        let state = make_state();
        let response = route_monitor_request_with_state(
            "POST",
            "/monitor/notifications/ntf-1/not-real",
            vec![],
            &state,
        )
        .expect("route should return a response");

        assert_eq!(response.status_code, 400, "unknown action should be 400 Bad Request");
    }

    #[test]
    fn route_notification_bad_id_returns_404() {
        let state = make_state();
        let response = route_monitor_request_with_state(
            "POST",
            "/monitor/notifications/not-real/open",
            vec![],
            &state,
        )
        .expect("route should return a response");

        assert_eq!(response.status_code, 404, "missing notification should be 404 Not Found");
    }

    // -----------------------------------------------------------------------
    // Sprint 4 — dispatch backend seam tests
    // -----------------------------------------------------------------------

    #[test]
    fn scaffold_backend_returns_accepted_and_monitor_state_updates() {
        use super::{DispatchTarget, MonitorDispatchBackend};

        let backend = ScaffoldMonitorDispatchBackend;
        let result = backend.handle_dispatch("chat-1", &DispatchTarget::Software, "build a thing");

        assert!(result.accepted, "scaffold backend should always accept");
        assert_eq!(result.backend, "scaffold");
        assert!(result.error.is_none());

        // Verify that a full dispatch through ferros_agent_handle_human_message
        // records the backend name on the result.
        let mut state = MonitorState::default();
        let session = state.create_session(None);
        let dispatch_result =
            state.ferros_agent_handle_human_message(&session.id, "msg-1", "route to software");

        assert!(
            matches!(dispatch_result.status, super::MonitorDispatchStatus::Routed),
            "expected Routed"
        );
        assert_eq!(
            dispatch_result.backend.as_deref(),
            Some("scaffold"),
            "dispatch result should carry backend label"
        );
    }

    #[test]
    fn monitor_state_persistence_roundtrip_preserves_messages_and_notifications() {
        let mut state = MonitorState::default();
        let session = state.create_session(Some("Persisted chat".to_owned()));
        let _ = state.add_message(
            &session.id,
            super::MonitorMessageRequest {
                speaker: "user".to_owned(),
                who: "Human".to_owned(),
                text: "please route this through business".to_owned(),
            },
        );
        let notification_id = state.create_notification(
            Some("wo-2".to_owned()),
            None,
            None,
            "medium",
            "Follow-up needed",
            "Operator should review this packet",
        );
        let _ = state.update_notification_status(&notification_id, "opened");

        let payload = PersistedMonitorState {
            schema_version: super::MONITOR_STATE_SCHEMA_VERSION,
            state,
        };
        let persisted = serde_json::to_string_pretty(&payload).expect("persisted snapshot should serialize");
        let loaded: PersistedMonitorState = serde_json::from_str(&persisted).expect("persisted snapshot should deserialize");

        assert_eq!(loaded.schema_version, super::MONITOR_STATE_SCHEMA_VERSION);
        assert!(loaded
            .state
            .open_chats
            .iter()
            .flat_map(|chat| chat.messages.iter())
            .all(|message| !message.id.is_empty()));
        assert_eq!(loaded.state.notifications.len(), 1);
        assert_eq!(loaded.state.notifications[0].status, "opened");
        assert_eq!(loaded.state.notifications[0].packet_id.as_deref(), Some("wo-2"));
    }

    #[test]
    fn revoked_grant_does_not_authorize_agent_start() {
        let profile_id = ProfileId::new("profile-alpha").expect("valid profile id");
        let mut revoked_grant = CapabilityGrant::new(profile_id.clone(), "agent.echo");
        assert!(revoked_grant.revoke("2026-04-23T00:00:00Z", "manual revoke"));

        let mut runtime = DemoRuntime::new(vec![revoked_grant]);
        let echo = EchoAgent::new(profile_id);

        runtime
            .register(echo.manifest(), Box::new(echo))
            .expect("echo should register");

        let error = runtime
            .start_agent("echo")
            .expect_err("revoked grant should deny start");

        match error {
            DemoError::AuthorizationDenied(detail) => {
                assert_eq!(detail.summary, "echo missing agent.echo");
                assert_eq!(detail.missing_requirements.len(), 1);
                assert_eq!(detail.missing_requirements[0].capability, "agent.echo");
                assert_eq!(
                    detail.missing_requirements[0].profile_id.as_str(),
                    "profile-alpha"
                );
            }
            other => panic!("unexpected authorization result: {other:?}"),
        }
        assert!(runtime
            .log_entries()
            .iter()
            .any(|entry| entry == "denied-start:echo missing agent.echo"));
    }

    #[test]
    fn revoked_grant_does_not_authorize_runtime_messages() {
        let profile_id = ProfileId::new("profile-alpha").expect("valid profile id");
        let mut revoked_grant = CapabilityGrant::new(profile_id.clone(), "agent.echo");
        assert!(revoked_grant.revoke("2026-04-23T00:00:00Z", "manual revoke"));

        let mut runtime = DemoRuntime::new(vec![revoked_grant]);
        let echo = EchoAgent::new(profile_id);

        runtime
            .register(echo.manifest(), Box::new(echo))
            .expect("echo should register");

        let error = runtime
            .send_message("echo", "echo", "agent.echo", b"hello")
            .expect_err("revoked grant should deny runtime authorization");

        assert_eq!(
            error,
            DemoError::AuthorizationDenied(AuthorizationDenyDetail::from_summary(
                "echo:agent.echo:Denied(NoGrantsPresented)"
            ),)
        );
        assert!(runtime
            .log_entries()
            .iter()
            .any(|entry| entry == "denied:echo:agent.echo:Denied(NoGrantsPresented)"));
    }

    #[test]
    fn agent_cli_lists_reference_agents_with_status() {
        let state_path = unique_state_path("list");

        let lines = execute_agent_cli_with_state_path(AgentCliCommand::List, &state_path)
            .expect("list should succeed");

        assert_eq!(
            lines,
            vec![
                "name\tversion\tstatus".to_string(),
                "echo\t0.1.0\tregistered".to_string(),
                "ha-local-bridge\t0.1.0\tregistered".to_string(),
                "timer\t0.1.0\tregistered".to_string(),
            ]
        );

        cleanup_state_path(&state_path);
    }

    #[test]
    fn local_agent_api_lists_reference_agents_without_cli_formatting() {
        let state_path = unique_state_path("local-api-list");

        let response = LocalAgentApi::at_state_path(&state_path)
            .execute(LocalAgentApiCommand::List)
            .expect("local agent API list should succeed");

        match response {
            LocalAgentApiResponse::AgentList { agents } => {
                assert_eq!(agents.len(), 3);
                assert_eq!(agents[0].manifest.name.as_str(), "echo");
                assert_eq!(agents[0].status, AgentStatus::Registered);
                assert_eq!(agents[1].manifest.name.as_str(), "ha-local-bridge");
                assert_eq!(agents[1].status, AgentStatus::Registered);
                assert_eq!(agents[2].manifest.name.as_str(), "timer");
                assert_eq!(agents[2].status, AgentStatus::Registered);
            }
            other => panic!("unexpected local agent API result: {other:?}"),
        }

        cleanup_state_path(&state_path);
    }

    #[test]
    fn agent_cli_persists_run_stop_and_logs() {
        let state_path = unique_state_path("lifecycle");

        execute_agent_cli_with_state_path(
            AgentCliCommand::Run {
                name: "echo".to_string(),
            },
            &state_path,
        )
        .expect("run should succeed");

        let describe_running = execute_agent_cli_with_state_path(
            AgentCliCommand::Describe {
                name: "echo".to_string(),
            },
            &state_path,
        )
        .expect("describe should succeed");

        assert!(describe_running
            .iter()
            .any(|line| line == "status: running"));

        execute_agent_cli_with_state_path(
            AgentCliCommand::Stop {
                name: "echo".to_string(),
            },
            &state_path,
        )
        .expect("stop should succeed");

        let describe_stopped = execute_agent_cli_with_state_path(
            AgentCliCommand::Describe {
                name: "echo".to_string(),
            },
            &state_path,
        )
        .expect("describe after stop should succeed");

        assert!(describe_stopped
            .iter()
            .any(|line| line == "status: stopped"));

        let logs =
            execute_agent_cli_with_state_path(AgentCliCommand::Logs { name: None }, &state_path)
                .expect("logs should succeed");

        assert_eq!(
            logs,
            vec!["started:echo".to_string(), "stopped:echo".to_string()]
        );

        cleanup_state_path(&state_path);
    }

    #[test]
    fn local_agent_api_persists_run_stop_and_logs_without_cli_formatting() {
        let state_path = unique_state_path("local-api-lifecycle");
        let api = LocalAgentApi::at_state_path(&state_path);

        let running = api
            .execute(LocalAgentApiCommand::Run {
                name: "echo".to_string(),
            })
            .expect("local agent API run should succeed");

        match running {
            LocalAgentApiResponse::AgentLifecycle { agent } => {
                assert_eq!(agent.manifest.name.as_str(), "echo");
                assert_eq!(agent.status, AgentStatus::Running);
            }
            other => panic!("unexpected local agent API run result: {other:?}"),
        }

        let stopped = api
            .execute(LocalAgentApiCommand::Stop {
                name: "echo".to_string(),
            })
            .expect("local agent API stop should succeed");

        match stopped {
            LocalAgentApiResponse::AgentLifecycle { agent } => {
                assert_eq!(agent.manifest.name.as_str(), "echo");
                assert_eq!(agent.status, AgentStatus::Stopped);
            }
            other => panic!("unexpected local agent API stop result: {other:?}"),
        }

        let logs = api
            .execute(LocalAgentApiCommand::Logs { name: None })
            .expect("local agent API logs should succeed");

        match logs {
            LocalAgentApiResponse::AgentLogs { entries } => {
                assert_eq!(
                    entries,
                    vec!["started:echo".to_string(), "stopped:echo".to_string()]
                );
            }
            other => panic!("unexpected local agent API log result: {other:?}"),
        }

        cleanup_state_path(&state_path);
    }

    #[test]
    fn local_agent_api_runway_summary_serializes_and_tracks_profile_and_deny_observation() {
        let state_path = unique_state_path("local-api-runway-summary");
        let profile_path = unique_profile_path("local-api-runway-summary");
        let store = FileSystemProfileStore;
        let api = LocalAgentApi::at_state_path(&state_path);

        let missing_summary = api
            .runway_summary_with_store_and_profile_path(&profile_path, &store)
            .expect("runway summary should load without a profile");
        let missing_profile = missing_summary
            .checklist
            .iter()
            .find(|item| item.item == "profileInit")
            .expect("profile checklist item should be present");
        let missing_consent = missing_summary
            .checklist
            .iter()
            .find(|item| item.item == "consentFlowVisibility")
            .expect("consent checklist item should be present");
        let missing_hub_restart = missing_summary
            .hub_restart
            .as_ref()
            .expect("hub restart observation should be present");

        assert_eq!(missing_summary.surface, "local-runway-summary");
        assert_eq!(missing_summary.scope, "local-only");
        assert_eq!(missing_summary.evidence, "non-evidentiary");
        assert_eq!(missing_summary.checkpoint_state, "pending");
        assert_eq!(
            missing_summary.checkpoint_detail,
            "Local shell runway not initialized yet."
        );
        assert_eq!(missing_summary.checkpoint_position, 1);
        assert_eq!(missing_summary.checkpoint_total, 7);
        assert_eq!(
            missing_summary.profile_path,
            profile_path.display().to_string()
        );
        assert_eq!(missing_summary.agent_count, 3);
        assert_eq!(missing_summary.deny_count, 0);
        assert!(missing_summary.latest_deny.is_none());
        assert_eq!(
            missing_hub_restart.snapshot_path,
            LOCAL_HUB_STATE_SNAPSHOT_PATH
        );
        assert_eq!(missing_hub_restart.scope, "local-only");
        assert_eq!(missing_hub_restart.evidence, "non-evidentiary");
        assert!(matches!(
            missing_hub_restart.reload_status.as_str(),
            "fresh-start" | "reloaded" | "unavailable"
        ));
        assert_eq!(missing_profile.status, LocalRunwayChecklistStatus::Pending);
        assert_eq!(missing_consent.status, LocalRunwayChecklistStatus::Pending);

        execute_profile_cli_with_store(
            ProfileCliCommand::Init {
                path: profile_path.clone(),
            },
            &store,
        )
        .expect("profile init should succeed");

        let error = execute_local_agent_api_with_runtime_loader(
            LocalAgentApiCommand::Run {
                name: "echo".to_string(),
            },
            &state_path,
            denied_reference_runtime_from_loaded_state,
        )
        .expect_err("denied runtime should persist a deny log entry");
        assert!(matches!(
            error,
            CliError::Runtime(DemoError::AuthorizationDenied(_))
        ));

        let observed_summary = api
            .runway_summary_with_store_and_profile_path(&profile_path, &store)
            .expect("runway summary should load after profile init and deny");
        let observed_profile = observed_summary
            .checklist
            .iter()
            .find(|item| item.item == "profileInit")
            .expect("profile checklist item should be present");
        let observed_consent = observed_summary
            .checklist
            .iter()
            .find(|item| item.item == "consentFlowVisibility")
            .expect("consent checklist item should be present");
        let payload = serde_json::to_value(&observed_summary)
            .expect("runway summary should serialize to JSON");

        assert_eq!(
            observed_summary.profile_name.as_deref(),
            Some(DEFAULT_PROFILE_NAME)
        );
        assert_eq!(observed_summary.grant_count, 0);
        assert_eq!(observed_summary.deny_count, 1);
        assert_eq!(observed_summary.checkpoint_state, "runtime-ready");
        assert_eq!(
            observed_summary.checkpoint_detail,
            "Consent checkpoint observed; runtime activation pending."
        );
        assert_eq!(observed_summary.checkpoint_position, 4);
        assert_eq!(observed_summary.checkpoint_total, 7);
        assert_eq!(
            observed_profile.status,
            LocalRunwayChecklistStatus::Observed
        );
        assert_eq!(
            observed_consent.status,
            LocalRunwayChecklistStatus::Observed
        );
        assert_eq!(
            observed_summary
                .latest_deny
                .as_ref()
                .and_then(|deny| deny.agent_name.as_deref()),
            Some("echo")
        );
        assert_eq!(payload["surface"].as_str(), Some("local-runway-summary"));
        assert_eq!(payload["latestDeny"]["kind"].as_str(), Some("deniedStart"));
        assert_eq!(
            payload["hubRestart"]["snapshotPath"].as_str(),
            Some(LOCAL_HUB_STATE_SNAPSHOT_PATH)
        );
        assert_eq!(payload["hubRestart"]["scope"].as_str(), Some("local-only"));
        assert_eq!(
            payload["hubRestart"]["evidence"].as_str(),
            Some("non-evidentiary")
        );
        assert!(matches!(
            payload["hubRestart"]["reloadStatus"].as_str(),
            Some("fresh-start" | "reloaded" | "unavailable")
        ));
        assert!(payload["checklist"].is_array());

        cleanup_state_path(&state_path);
        cleanup_profile_artifacts(&profile_path);
        cleanup_parent_dir(&profile_path);
    }

    #[test]
    fn local_agent_api_runway_summary_omits_hub_restart_when_hub_summary_loader_fails() {
        let state_path = unique_state_path("local-api-runway-summary-hub-restart-fallback");
        let profile_path = unique_profile_path("local-api-runway-summary-hub-restart-fallback");
        let store = FileSystemProfileStore;

        let summary = build_local_runway_summary_with_store_and_hub_summary_loader(
            &state_path,
            &profile_path,
            &store,
            || -> Result<LocalHubRuntimeSummary, LocalBridgeRegistrationError> {
                Err(LocalBridgeRegistrationError::AlreadyRegistered(
                    "ha-local-bridge".to_owned(),
                ))
            },
        )
        .expect("runway summary should load when the hub summary is unavailable");
        let payload = serde_json::to_value(&summary)
            .expect("runway summary should serialize without the hub restart child");

        assert!(summary.hub_restart.is_none());
        assert!(summary.hub_onramp_proposal.is_none());
        assert!(payload.get("hubRestart").is_none());
        assert!(payload.get("hubOnrampProposal").is_none());

        cleanup_state_path(&state_path);
        cleanup_profile_artifacts(&profile_path);
        cleanup_parent_dir(&profile_path);
    }

    #[test]
    fn onramp_shell_route_gets_local_runway_summary_json() {
        let state_path = unique_state_path("onramp-shell-runway-summary");
        let profile_path = unique_profile_path("onramp-shell-runway-summary");
        let store = FileSystemProfileStore;

        execute_profile_cli_with_store(
            ProfileCliCommand::Init {
                path: profile_path.clone(),
            },
            &store,
        )
        .expect("profile init should succeed");

        execute_local_agent_api_with_runtime_loader(
            LocalAgentApiCommand::Run {
                name: "echo".to_string(),
            },
            &state_path,
            denied_reference_runtime_from_loaded_state,
        )
        .expect_err("denied runtime should record a deny entry before summary read");

        let response = route_shell_request_with_store_and_paths(
            HttpRequest {
                method: "GET".to_owned(),
                path: format!(
                    "/runway-summary.json?profilePath={}",
                    profile_path.display()
                ),
                body: Vec::new(),
            },
            &state_path,
            &profile_path,
            &store,
        );
        let payload: LocalRunwaySummary =
            serde_json::from_slice(&response.body).expect("runway summary response should parse");
        let hub_onramp_proposal = payload
            .hub_onramp_proposal
            .as_ref()
            .expect("shell runway summary should include a hub onramp proposal child");

        assert_eq!(response.status_code, 200);
        assert_eq!(
            hub_onramp_proposal.source,
            SIMULATED_LOCAL_BRIDGE_ARTIFACT_PATH
        );
        assert_eq!(
            hub_onramp_proposal.proposal_id,
            "proposal-ha-local-bridge-simulated-bridge-entity-report-state"
        );
        assert_eq!(hub_onramp_proposal.bridge_agent_name, "ha-local-bridge");
        assert_eq!(
            hub_onramp_proposal.stand_in_entity_name,
            "simulated-bridge-entity"
        );
        assert_eq!(hub_onramp_proposal.requested_capability, "bridge.observe");
        assert_eq!(hub_onramp_proposal.requested_action, "report-state");
        assert_eq!(
            hub_onramp_proposal.quarantine_status,
            "quarantined-pending-consent"
        );
        assert_eq!(hub_onramp_proposal.scope, "local-only");
        assert_eq!(hub_onramp_proposal.evidence, "non-evidentiary");
        assert_eq!(
            hub_onramp_proposal.local_artifact_path,
            ".tmp/hub/local-onramp-proposal.json"
        );

        cleanup_state_path(&state_path);
        cleanup_profile_artifacts(&profile_path);
        cleanup_parent_dir(&profile_path);
    }

    #[test]
    fn onramp_runway_summary_omits_hub_onramp_proposal_when_hub_summary_child_is_absent() {
        let state_path = unique_state_path("onramp-runway-summary-no-child");
        let profile_path = unique_profile_path("onramp-runway-summary-no-child");
        let store = FileSystemProfileStore;

        let summary = build_local_runway_summary_with_store_and_hub_summary_loader(
            &state_path,
            &profile_path,
            &store,
            || -> Result<LocalHubRuntimeSummary, LocalBridgeRegistrationError> {
                let mut summary = default_local_runtime_summary()?;
                summary.local_onramp_proposal = None;
                Ok(summary)
            },
        )
        .expect("runway summary should load when the hub onramp proposal child is absent");
        let payload = serde_json::to_value(&summary)
            .expect("runway summary should serialize without the hub onramp child");

        assert!(summary.hub_restart.is_some());
        assert!(summary.hub_onramp_proposal.is_none());
        assert!(payload.get("hubRestart").is_some());
        assert!(payload.get("hubOnrampProposal").is_none());

        cleanup_state_path(&state_path);
        cleanup_profile_artifacts(&profile_path);
        cleanup_parent_dir(&profile_path);
    }

    #[test]
    fn onramp_runway_summary_omits_hub_onramp_proposal_when_hub_summary_child_is_invalid() {
        let state_path = unique_state_path("onramp-runway-summary-invalid-child");
        let profile_path = unique_profile_path("onramp-runway-summary-invalid-child");
        let store = FileSystemProfileStore;

        let summary = build_local_runway_summary_with_store_and_hub_summary_loader(
            &state_path,
            &profile_path,
            &store,
            || -> Result<LocalHubRuntimeSummary, LocalBridgeRegistrationError> {
                let mut summary = default_local_runtime_summary()?;
                summary
                    .local_onramp_proposal
                    .as_mut()
                    .expect("default hub summary should include an onramp proposal")
                    .local_artifact_path =
                    "https://example.com/local-onramp-proposal.json".to_owned();
                Ok(summary)
            },
        )
        .expect(
            "runway summary should load when the invalid hub onramp proposal child is filtered",
        );
        let payload = serde_json::to_value(&summary)
            .expect("runway summary should serialize without the invalid hub onramp child");

        assert!(summary.hub_restart.is_some());
        assert!(summary.hub_onramp_proposal.is_none());
        assert!(summary.hub_onramp_decision_receipt.is_some());
        assert!(payload.get("hubRestart").is_some());
        assert!(payload.get("hubOnrampProposal").is_none());
        assert!(payload.get("hubOnrampDecisionReceipt").is_some());

        cleanup_state_path(&state_path);
        cleanup_profile_artifacts(&profile_path);
        cleanup_parent_dir(&profile_path);
    }

    #[test]
    fn onramp_decision_shell_route_gets_local_runway_summary_json() {
        let state_path = unique_state_path("onramp-decision-shell-runway-summary");
        let profile_path = unique_profile_path("onramp-decision-shell-runway-summary");
        let store = FileSystemProfileStore;

        execute_profile_cli_with_store(
            ProfileCliCommand::Init {
                path: profile_path.clone(),
            },
            &store,
        )
        .expect("profile init should succeed");

        execute_local_agent_api_with_runtime_loader(
            LocalAgentApiCommand::Run {
                name: "echo".to_string(),
            },
            &state_path,
            denied_reference_runtime_from_loaded_state,
        )
        .expect_err("denied runtime should record a deny entry before summary read");

        let response = route_shell_request_with_store_and_paths(
            HttpRequest {
                method: "GET".to_owned(),
                path: format!(
                    "/runway-summary.json?profilePath={}",
                    profile_path.display()
                ),
                body: Vec::new(),
            },
            &state_path,
            &profile_path,
            &store,
        );
        let payload: LocalRunwaySummary =
            serde_json::from_slice(&response.body).expect("runway summary response should parse");
        let hub_onramp_decision_receipt = payload
            .hub_onramp_decision_receipt
            .as_ref()
            .expect("shell runway summary should include a hub onramp decision receipt child");

        assert_eq!(response.status_code, 200);
        assert_eq!(
            hub_onramp_decision_receipt.proposal_id,
            "proposal-ha-local-bridge-simulated-bridge-entity-report-state"
        );
        assert_eq!(hub_onramp_decision_receipt.decision_label, "allowed");
        assert_eq!(
            hub_onramp_decision_receipt.decision_detail.as_deref(),
            Some(
                "local-only operator rehearsal allowed report-state for proposal proposal-ha-local-bridge-simulated-bridge-entity-report-state"
            )
        );
        assert_eq!(
            hub_onramp_decision_receipt.local_artifact_path,
            ".tmp/hub/local-onramp-decision-receipt.json"
        );

        cleanup_state_path(&state_path);
        cleanup_profile_artifacts(&profile_path);
        cleanup_parent_dir(&profile_path);
    }

    #[test]
    fn onramp_decision_runway_summary_omits_hub_onramp_decision_receipt_when_hub_summary_child_is_absent(
    ) {
        let state_path = unique_state_path("onramp-decision-runway-summary-no-child");
        let profile_path = unique_profile_path("onramp-decision-runway-summary-no-child");
        let store = FileSystemProfileStore;

        let summary = build_local_runway_summary_with_store_and_hub_summary_loader(
            &state_path,
            &profile_path,
            &store,
            || -> Result<LocalHubRuntimeSummary, LocalBridgeRegistrationError> {
                let mut summary = default_local_runtime_summary()?;
                summary.local_onramp_decision_receipt = None;
                Ok(summary)
            },
        )
        .expect("runway summary should load when the hub onramp decision child is absent");
        let payload = serde_json::to_value(&summary)
            .expect("runway summary should serialize without the hub onramp decision child");

        assert!(summary.hub_restart.is_some());
        assert!(summary.hub_onramp_proposal.is_some());
        assert!(summary.hub_onramp_decision_receipt.is_none());
        assert!(payload.get("hubRestart").is_some());
        assert!(payload.get("hubOnrampProposal").is_some());
        assert!(payload.get("hubOnrampDecisionReceipt").is_none());

        cleanup_state_path(&state_path);
        cleanup_profile_artifacts(&profile_path);
        cleanup_parent_dir(&profile_path);
    }

    #[test]
    fn onramp_decision_runway_summary_omits_hub_onramp_decision_receipt_when_hub_summary_child_is_invalid(
    ) {
        let state_path = unique_state_path("onramp-decision-runway-summary-invalid-child");
        let profile_path = unique_profile_path("onramp-decision-runway-summary-invalid-child");
        let store = FileSystemProfileStore;

        let summary = build_local_runway_summary_with_store_and_hub_summary_loader(
            &state_path,
            &profile_path,
            &store,
            || -> Result<LocalHubRuntimeSummary, LocalBridgeRegistrationError> {
                let mut summary = default_local_runtime_summary()?;
                summary
                    .local_onramp_decision_receipt
                    .as_mut()
                    .expect("default hub summary should include an onramp decision receipt")
                    .decision_detail = Some("local-only gate closure rehearsal".to_owned());
                Ok(summary)
            },
        )
        .expect(
            "runway summary should load when the invalid hub onramp decision child is filtered",
        );
        let payload = serde_json::to_value(&summary).expect(
            "runway summary should serialize without the invalid hub onramp decision child",
        );

        assert!(summary.hub_restart.is_some());
        assert!(summary.hub_onramp_proposal.is_some());
        assert!(summary.hub_onramp_decision_receipt.is_none());
        assert!(payload.get("hubRestart").is_some());
        assert!(payload.get("hubOnrampProposal").is_some());
        assert!(payload.get("hubOnrampDecisionReceipt").is_none());

        cleanup_state_path(&state_path);
        cleanup_profile_artifacts(&profile_path);
        cleanup_parent_dir(&profile_path);
    }

    #[test]
    fn agent_read_rpc_observes_cli_lifecycle_state_after_local_run_and_stop() {
        let state_path = unique_state_path("rpc-cli-lifecycle");
        let profile_path = unique_profile_path("rpc-cli-lifecycle");

        execute_agent_cli_with_state_path(
            AgentCliCommand::Run {
                name: "echo".to_string(),
            },
            &state_path,
        )
        .expect("run should succeed");

        let running_detail = execute_agent_read_rpc_with_store_and_paths(
            AgentJsonRpcRequest::new(
                "req-cli-lifecycle-running-detail",
                METHOD_AGENT_DESCRIBE,
                AgentJsonRpcParams::for_agent("echo"),
            ),
            &state_path,
            &profile_path,
            &FileSystemProfileStore,
        )
        .expect("describe RPC should succeed after local run");

        match running_detail.result.expect("result should be present") {
            AgentJsonRpcResult::AgentDetail { agent } => {
                assert_eq!(agent.name, "echo");
                assert_eq!(agent.status, "running");
            }
            other => panic!("unexpected RPC result: {other:?}"),
        }

        let running_snapshot = execute_agent_read_rpc_with_store_and_paths(
            AgentJsonRpcRequest::new(
                "req-cli-lifecycle-running-snapshot",
                METHOD_AGENT_SNAPSHOT,
                AgentJsonRpcParams::for_agent("echo"),
            ),
            &state_path,
            &profile_path,
            &FileSystemProfileStore,
        )
        .expect("snapshot RPC should succeed after local run");

        match running_snapshot.result.expect("result should be present") {
            AgentJsonRpcResult::AgentSnapshot { snapshot } => {
                assert_eq!(snapshot.agents.len(), 1);
                assert_eq!(snapshot.agents[0].name, "echo");
                assert_eq!(snapshot.agents[0].status, "running");
            }
            other => panic!("unexpected RPC result: {other:?}"),
        }

        execute_agent_cli_with_state_path(
            AgentCliCommand::Stop {
                name: "echo".to_string(),
            },
            &state_path,
        )
        .expect("stop should succeed");

        let stopped_snapshot = execute_agent_read_rpc_with_store_and_paths(
            AgentJsonRpcRequest::new(
                "req-cli-lifecycle-stopped-snapshot",
                METHOD_AGENT_SNAPSHOT,
                AgentJsonRpcParams::for_agent("echo"),
            ),
            &state_path,
            &profile_path,
            &FileSystemProfileStore,
        )
        .expect("snapshot RPC should succeed after local stop");

        match stopped_snapshot.result.expect("result should be present") {
            AgentJsonRpcResult::AgentSnapshot { snapshot } => {
                assert_eq!(snapshot.agents.len(), 1);
                assert_eq!(snapshot.agents[0].name, "echo");
                assert_eq!(snapshot.agents[0].status, "stopped");
            }
            other => panic!("unexpected RPC result: {other:?}"),
        }

        cleanup_state_path(&state_path);
        cleanup_profile_artifacts(&profile_path);
    }

    fn denied_reference_runtime_from_loaded_state(
        state: &CliState,
    ) -> Result<DemoRuntime, CliError> {
        let mut runtime =
            DemoRuntime::reference_host_with_grants(Vec::new()).map_err(CliError::from)?;
        runtime.replay_cli_state(state)?;
        Ok(runtime)
    }

    #[test]
    fn agent_cli_denied_run_persists_deny_start_without_mutating_agent_state() {
        let state_path = unique_state_path("lifecycle-denied-start");

        let error = execute_agent_cli_with_runtime_loader(
            AgentCliCommand::Run {
                name: "echo".to_string(),
            },
            &state_path,
            denied_reference_runtime_from_loaded_state,
        )
        .expect_err("run should be denied without grants");

        assert!(matches!(
            error,
            CliError::Runtime(DemoError::AuthorizationDenied(AuthorizationDenyDetail {
                summary,
                missing_requirements,
            }))
                if summary == "echo missing agent.echo"
                && missing_requirements.len() == 1
                && missing_requirements[0].capability == "agent.echo"
                && missing_requirements[0].profile_id.as_str() == "profile-alpha"
        ));

        let describe = execute_agent_cli_with_runtime_loader(
            AgentCliCommand::Describe {
                name: "echo".to_string(),
            },
            &state_path,
            denied_reference_runtime_from_loaded_state,
        )
        .expect("describe should still succeed after denied run");

        assert!(describe.iter().any(|line| line == "status: registered"));

        let logs =
            execute_agent_cli_with_state_path(AgentCliCommand::Logs { name: None }, &state_path)
                .expect("logs should reflect the denied lifecycle attempt");

        assert_eq!(
            logs,
            vec!["denied-start:echo missing agent.echo".to_string()]
        );

        cleanup_state_path(&state_path);
    }

    #[test]
    fn local_agent_api_denied_run_persists_deny_start_without_mutating_agent_state() {
        let state_path = unique_state_path("local-api-denied-start");

        let error = execute_local_agent_api_with_runtime_loader(
            LocalAgentApiCommand::Run {
                name: "echo".to_string(),
            },
            &state_path,
            denied_reference_runtime_from_loaded_state,
        )
        .expect_err("local agent API run should be denied without grants");

        assert!(matches!(
            error,
            CliError::Runtime(DemoError::AuthorizationDenied(AuthorizationDenyDetail {
                summary,
                missing_requirements,
            }))
                if summary == "echo missing agent.echo"
                && missing_requirements.len() == 1
                && missing_requirements[0].capability == "agent.echo"
                && missing_requirements[0].profile_id.as_str() == "profile-alpha"
        ));

        let describe = execute_local_agent_api_with_runtime_loader(
            LocalAgentApiCommand::Describe {
                name: "echo".to_string(),
            },
            &state_path,
            denied_reference_runtime_from_loaded_state,
        )
        .expect("local agent API describe should succeed after denied run");

        match describe {
            LocalAgentApiResponse::AgentDetail { agent } => {
                assert_eq!(agent.manifest.name.as_str(), "echo");
                assert_eq!(agent.status, AgentStatus::Registered);
            }
            other => panic!("unexpected local agent API detail result: {other:?}"),
        }

        let logs = LocalAgentApi::at_state_path(&state_path)
            .execute(LocalAgentApiCommand::Logs { name: None })
            .expect("local agent API logs should reflect the denied lifecycle attempt");

        match logs {
            LocalAgentApiResponse::AgentLogs { entries } => {
                assert_eq!(
                    entries,
                    vec!["denied-start:echo missing agent.echo".to_string()]
                );
            }
            other => panic!("unexpected local agent API log result: {other:?}"),
        }

        cleanup_state_path(&state_path);
    }

    #[test]
    fn reload_boundary_cli_state_load_reads_exact_path_and_defaults_missing_path() {
        let state_path = unique_state_path("reload-cli-state");
        let other_path = unique_state_path("reload-cli-state-other");
        let missing_path = unique_state_path("reload-cli-state-missing");

        let mut state = CliState::default();
        state.set_status("echo", AgentStatus::Running);
        state.log_entries.push("started:echo".to_owned());
        state.save(&state_path).expect("state should save");

        let mut other_state = CliState::default();
        other_state.set_status("timer", AgentStatus::Stopped);
        other_state
            .save(&other_path)
            .expect("other state should save");

        let loaded = CliState::load(&state_path).expect("state should load from the provided path");
        let missing = CliState::load(&missing_path).expect("missing state should default to empty");

        assert_eq!(loaded, state);
        assert_ne!(loaded, other_state);
        assert_eq!(missing, CliState::default());

        cleanup_state_path(&state_path);
        cleanup_state_path(&other_path);
        cleanup_state_path(&missing_path);
    }

    #[test]
    fn reload_boundary_runtime_with_state_rebuilds_reference_runtime_without_replaying_logs() {
        let state_path = unique_state_path("reload-runtime");
        let mut state = CliState::default();
        state.set_status("echo", AgentStatus::Running);
        state.set_status("timer", AgentStatus::Stopped);
        state.log_entries.push("started:echo".to_owned());
        state.log_entries.push("stopped:timer".to_owned());
        state.save(&state_path).expect("state should save");

        let runtime = runtime_with_state(&state_path)
            .expect("runtime should rebuild from persisted reference agent state");
        let records = runtime
            .agent_records()
            .into_iter()
            .map(|record| (record.manifest.name.as_str().to_owned(), record.status))
            .collect::<Vec<_>>();

        assert_eq!(
            records,
            vec![
                ("echo".to_owned(), AgentStatus::Running),
                ("ha-local-bridge".to_owned(), AgentStatus::Registered),
                ("timer".to_owned(), AgentStatus::Stopped),
            ]
        );
        assert!(runtime.log_entries().is_empty());

        cleanup_state_path(&state_path);
    }

    #[test]
    fn reload_boundary_runtime_with_state_preserves_unknown_agent_error_path() {
        let state_path = unique_state_path("reload-runtime-unknown-agent");
        let mut state = CliState::default();
        state.set_status("missing", AgentStatus::Running);
        state.save(&state_path).expect("state should save");

        let error = match runtime_with_state(&state_path) {
            Ok(_) => panic!("unknown persisted agents should fail"),
            Err(error) => error,
        };

        assert!(matches!(
            error,
            CliError::Runtime(DemoError::UnknownAgent(name)) if name == "missing"
        ));

        cleanup_state_path(&state_path);
    }

    #[test]
    fn reload_boundary_runtime_with_state_rejects_unsupported_persisted_status() {
        let state_path = unique_state_path("reload-runtime-unsupported-status");
        let mut state = CliState::default();
        state.set_status("echo", AgentStatus::Failed);
        state.save(&state_path).expect("state should save");

        let error = match runtime_with_state(&state_path) {
            Ok(_) => panic!("unsupported persisted status should be rejected"),
            Err(error) => error,
        };

        assert!(matches!(
            error,
            CliError::InvalidState(message)
                if message == "unsupported persisted status for echo: failed"
        ));

        cleanup_state_path(&state_path);
    }

    #[test]
    fn agent_read_rpc_lists_reference_agents() {
        let state_path = unique_state_path("rpc-list");
        let profile_path = unique_profile_path("rpc-list");

        let response = execute_agent_read_rpc_with_store_and_paths(
            AgentJsonRpcRequest::new("req-1", METHOD_AGENT_LIST, AgentJsonRpcParams::default()),
            &state_path,
            &profile_path,
            &FileSystemProfileStore,
        )
        .expect("list RPC should succeed");

        match response.result.expect("result should be present") {
            AgentJsonRpcResult::AgentList { agents } => {
                assert_eq!(agents.len(), 3);
                assert_eq!(agents[0].name, "echo");
                assert_eq!(agents[0].status, "registered");
                assert_eq!(agents[1].name, "ha-local-bridge");
                assert_eq!(agents[2].name, "timer");
            }
            other => panic!("unexpected RPC result: {other:?}"),
        }

        cleanup_state_path(&state_path);
        cleanup_profile_artifacts(&profile_path);
    }

    #[test]
    fn agent_read_rpc_rejects_unsupported_jsonrpc_version() {
        let state_path = unique_state_path("rpc-invalid-version");
        let profile_path = unique_profile_path("rpc-invalid-version");

        let response = execute_agent_read_rpc_with_store_and_paths(
            AgentJsonRpcRequest {
                jsonrpc: "1.0".to_owned(),
                id: "req-invalid-version".to_owned(),
                method: METHOD_AGENT_LIST.to_owned(),
                params: AgentJsonRpcParams::default(),
            },
            &state_path,
            &profile_path,
            &FileSystemProfileStore,
        )
        .expect("invalid version RPC should return a JSON-RPC error");

        assert_rpc_error(
            &response,
            JSON_RPC_INVALID_REQUEST,
            "unsupported JSON-RPC version: 1.0",
        );

        cleanup_state_path(&state_path);
        cleanup_profile_artifacts(&profile_path);
    }

    #[test]
    fn agent_read_rpc_requires_agent_name_for_describe() {
        let state_path = unique_state_path("rpc-describe-missing-params");
        let profile_path = unique_profile_path("rpc-describe-missing-params");

        let response = execute_agent_read_rpc_with_store_and_paths(
            AgentJsonRpcRequest::new(
                "req-describe-missing-params",
                METHOD_AGENT_DESCRIBE,
                AgentJsonRpcParams::default(),
            ),
            &state_path,
            &profile_path,
            &FileSystemProfileStore,
        )
        .expect("missing describe params should return a JSON-RPC error");

        assert_rpc_error(
            &response,
            JSON_RPC_INVALID_PARAMS,
            "agentName parameter is required for agent.describe",
        );

        cleanup_state_path(&state_path);
        cleanup_profile_artifacts(&profile_path);
    }

    #[test]
    fn agent_read_rpc_returns_not_found_for_unknown_agent() {
        let state_path = unique_state_path("rpc-agent-not-found");
        let profile_path = unique_profile_path("rpc-agent-not-found");

        let response = execute_agent_read_rpc_with_store_and_paths(
            AgentJsonRpcRequest::new(
                "req-agent-not-found",
                METHOD_AGENT_DESCRIBE,
                AgentJsonRpcParams::for_agent("missing"),
            ),
            &state_path,
            &profile_path,
            &FileSystemProfileStore,
        )
        .expect("unknown agent RPC should return a JSON-RPC error");

        assert_rpc_error(
            &response,
            JSON_RPC_AGENT_NOT_FOUND,
            "agent not found: missing",
        );

        cleanup_state_path(&state_path);
        cleanup_profile_artifacts(&profile_path);
    }

    #[test]
    fn agent_read_rpc_rejects_unknown_method_names() {
        let state_path = unique_state_path("rpc-unknown-method");
        let profile_path = unique_profile_path("rpc-unknown-method");

        let response = execute_agent_read_rpc_with_store_and_paths(
            AgentJsonRpcRequest::new(
                "req-unknown-method",
                "agent.write",
                AgentJsonRpcParams::default(),
            ),
            &state_path,
            &profile_path,
            &FileSystemProfileStore,
        )
        .expect("unknown methods should return a JSON-RPC error");

        assert_rpc_error(
            &response,
            JSON_RPC_METHOD_NOT_FOUND,
            "unknown JSON-RPC method: agent.write",
        );

        cleanup_state_path(&state_path);
        cleanup_profile_artifacts(&profile_path);
    }

    #[test]
    fn agent_read_rpc_describes_agent_with_capability_requirements() {
        let state_path = unique_state_path("rpc-describe");
        let profile_path = unique_profile_path("rpc-describe");

        let response = execute_agent_read_rpc_with_store_and_paths(
            AgentJsonRpcRequest::new(
                "req-2",
                METHOD_AGENT_DESCRIBE,
                AgentJsonRpcParams::for_agent("echo"),
            ),
            &state_path,
            &profile_path,
            &FileSystemProfileStore,
        )
        .expect("describe RPC should succeed");

        match response.result.expect("result should be present") {
            AgentJsonRpcResult::AgentDetail { agent } => {
                assert_eq!(agent.name, "echo");
                assert_eq!(agent.status, "registered");
                assert_eq!(agent.required_capabilities.len(), 1);
                assert_eq!(agent.required_capabilities[0].capability, "agent.echo");
            }
            other => panic!("unexpected RPC result: {other:?}"),
        }

        cleanup_state_path(&state_path);
        cleanup_profile_artifacts(&profile_path);
    }

    #[test]
    fn agent_read_rpc_snapshot_filters_agent_and_deny_log_and_uses_profile_path() {
        let store = FileSystemProfileStore;
        let state_path = unique_state_path("rpc-snapshot-filtered");
        let profile_path = unique_profile_path("rpc-snapshot-filtered");
        let mut state = CliState::default();

        execute_profile_cli_with_store(
            ProfileCliCommand::Init {
                path: profile_path.clone(),
            },
            &store,
        )
        .expect("profile init should succeed");
        execute_profile_cli_with_store(
            ProfileCliCommand::Grant {
                path: profile_path.clone(),
                capability: "agent.echo".to_owned(),
            },
            &store,
        )
        .expect("profile grant should succeed");

        state.set_status("echo", AgentStatus::Running);
        state
            .log_entries
            .push("denied:echo:agent.admin:Denied(NoGrantsPresented)".to_owned());
        state
            .log_entries
            .push("denied-start:timer missing agent.timer".to_owned());
        state.save(&state_path).expect("state file should save");

        let response = execute_agent_read_rpc_with_store_and_paths(
            AgentJsonRpcRequest::new(
                "req-snapshot-filtered",
                METHOD_AGENT_SNAPSHOT,
                AgentJsonRpcParams {
                    agent_name: Some("echo".to_owned()),
                    profile_path: Some(profile_path.display().to_string()),
                },
            ),
            &state_path,
            &unique_profile_path("rpc-snapshot-filtered-default"),
            &store,
        )
        .expect("snapshot RPC should succeed");

        match response.result.expect("result should be present") {
            AgentJsonRpcResult::AgentSnapshot { snapshot } => {
                assert_eq!(snapshot.agents.len(), 1);
                assert_eq!(snapshot.agents[0].name, "echo");
                assert_eq!(snapshot.agents[0].status, "running");
                assert_eq!(snapshot.agents[0].required_capabilities.len(), 1);
                assert_eq!(snapshot.grants.len(), 1);
                assert_eq!(snapshot.grants[0].capability, "agent.echo");
                assert_eq!(snapshot.deny_log.len(), 1);
                assert_eq!(snapshot.deny_log[0].agent_name.as_deref(), Some("echo"));
                assert_eq!(
                    snapshot.deny_log[0].capability.as_deref(),
                    Some("agent.admin")
                );
            }
            other => panic!("unexpected RPC result: {other:?}"),
        }

        cleanup_state_path(&state_path);
        cleanup_profile_artifacts(&profile_path);
        cleanup_profile_artifacts(&unique_profile_path("rpc-snapshot-filtered-default"));
    }

    #[test]
    fn agent_read_rpc_snapshot_returns_not_found_for_unknown_agent() {
        let state_path = unique_state_path("rpc-snapshot-agent-not-found");
        let profile_path = unique_profile_path("rpc-snapshot-agent-not-found");

        let response = execute_agent_read_rpc_with_store_and_paths(
            AgentJsonRpcRequest::new(
                "req-snapshot-agent-not-found",
                METHOD_AGENT_SNAPSHOT,
                AgentJsonRpcParams::for_agent("missing"),
            ),
            &state_path,
            &profile_path,
            &FileSystemProfileStore,
        )
        .expect("unknown snapshot agent RPC should return a JSON-RPC error");

        assert_rpc_error(
            &response,
            JSON_RPC_AGENT_NOT_FOUND,
            "agent not found: missing",
        );

        cleanup_state_path(&state_path);
        cleanup_profile_artifacts(&profile_path);
    }

    #[test]
    fn agent_read_rpc_lists_signed_grant_state_from_local_profile() {
        let store = FileSystemProfileStore;
        let state_path = unique_state_path("rpc-grants");
        let profile_path = unique_profile_path("rpc-grants");

        execute_profile_cli_with_store(
            ProfileCliCommand::Init {
                path: profile_path.clone(),
            },
            &store,
        )
        .expect("profile init should succeed");
        execute_profile_cli_with_store(
            ProfileCliCommand::Grant {
                path: profile_path.clone(),
                capability: "agent.echo".to_owned(),
            },
            &store,
        )
        .expect("profile grant should succeed");

        let response = execute_agent_read_rpc_with_store_and_paths(
            AgentJsonRpcRequest::new(
                "req-3",
                METHOD_GRANT_LIST,
                AgentJsonRpcParams {
                    agent_name: None,
                    profile_path: Some(profile_path.display().to_string()),
                },
            ),
            &state_path,
            &profile_path,
            &store,
        )
        .expect("grant list RPC should succeed");

        match response.result.expect("result should be present") {
            AgentJsonRpcResult::GrantList { grants } => {
                assert_eq!(grants.len(), 1);
                assert_eq!(grants[0].capability, "agent.echo");
                assert!(grants[0].is_active);
            }
            other => panic!("unexpected RPC result: {other:?}"),
        }

        cleanup_state_path(&state_path);
        cleanup_profile_artifacts(&profile_path);
    }

    #[test]
    fn agent_read_rpc_lists_only_deny_entries_and_supports_agent_filter() {
        let state_path = unique_state_path("rpc-deny-log");
        let profile_path = unique_profile_path("rpc-deny-log");
        let state = CliState {
            agent_statuses: Default::default(),
            log_entries: vec![
                "started:echo".to_owned(),
                "denied:echo:agent.admin:Denied(NoGrantsPresented)".to_owned(),
                "denied-start:timer missing agent.timer".to_owned(),
            ],
        };

        state.save(&state_path).expect("state file should save");

        let response = execute_agent_read_rpc_with_store_and_paths(
            AgentJsonRpcRequest::new(
                "req-4",
                METHOD_DENY_LOG_LIST,
                AgentJsonRpcParams::for_agent("echo"),
            ),
            &state_path,
            &profile_path,
            &FileSystemProfileStore,
        )
        .expect("deny log RPC should succeed");

        match response.result.expect("result should be present") {
            AgentJsonRpcResult::DenyLog { entries } => {
                assert_eq!(entries.len(), 1);
                assert_eq!(entries[0].agent_name.as_deref(), Some("echo"));
                assert_eq!(entries[0].capability.as_deref(), Some("agent.admin"));
                assert_eq!(entries[0].kind, "denied");
            }
            other => panic!("unexpected RPC result: {other:?}"),
        }

        cleanup_state_path(&state_path);
        cleanup_profile_artifacts(&profile_path);
    }

    #[test]
    fn agent_read_rpc_exposes_denied_lifecycle_entries_from_cli_state() {
        let state_path = unique_state_path("rpc-denied-lifecycle");
        let profile_path = unique_profile_path("rpc-denied-lifecycle");

        execute_agent_cli_with_runtime_loader(
            AgentCliCommand::Run {
                name: "echo".to_string(),
            },
            &state_path,
            denied_reference_runtime_from_loaded_state,
        )
        .expect_err("run should be denied without grants");

        let response = execute_agent_read_rpc_with_store_and_paths(
            AgentJsonRpcRequest::new(
                "req-denied-lifecycle",
                METHOD_DENY_LOG_LIST,
                AgentJsonRpcParams::for_agent("echo"),
            ),
            &state_path,
            &profile_path,
            &FileSystemProfileStore,
        )
        .expect("deny log RPC should succeed");

        match response.result.expect("result should be present") {
            AgentJsonRpcResult::DenyLog { entries } => {
                assert_eq!(entries.len(), 1);
                assert_eq!(entries[0].kind, "deniedStart");
                assert_eq!(entries[0].agent_name.as_deref(), Some("echo"));
                assert_eq!(entries[0].capability.as_deref(), Some("agent.echo"));
            }
            other => panic!("unexpected RPC result: {other:?}"),
        }

        cleanup_state_path(&state_path);
        cleanup_profile_artifacts(&profile_path);
    }

    #[test]
    fn agent_read_rpc_json_round_trips_serialized_requests() {
        let request_json = serde_json::to_string(&AgentJsonRpcRequest::new(
            "req-5",
            METHOD_AGENT_LIST,
            AgentJsonRpcParams::default(),
        ))
        .expect("request should serialize");

        let response_json = execute_agent_read_rpc_json(&request_json)
            .expect("JSON-RPC wrapper should return a JSON response");
        let response: ferros_agents::AgentJsonRpcResponse =
            serde_json::from_str(&response_json).expect("response JSON should deserialize");

        match response.result.expect("result should be present") {
            AgentJsonRpcResult::AgentList { agents } => {
                assert_eq!(agents.len(), 3);
            }
            other => panic!("unexpected RPC result: {other:?}"),
        }
    }

    #[test]
    fn agent_write_rpc_runs_and_stops_agent_over_local_state_path() {
        let state_path = unique_state_path("rpc-write-lifecycle");
        let profile_path = unique_profile_path("rpc-write-lifecycle");
        let profile_path_string = profile_path.display().to_string();

        execute_profile_cli_with_store(
            ProfileCliCommand::Init {
                path: profile_path.clone(),
            },
            &FileSystemProfileStore,
        )
        .expect("profile init should succeed");
        execute_profile_cli_with_store(
            ProfileCliCommand::Grant {
                path: profile_path.clone(),
                capability: "agent.echo".to_owned(),
            },
            &FileSystemProfileStore,
        )
        .expect("profile grant should succeed");

        let run_response = execute_agent_read_rpc_with_store_and_paths(
            AgentJsonRpcRequest::new(
                "req-write-run",
                METHOD_AGENT_RUN,
                AgentJsonRpcParams {
                    agent_name: Some("echo".to_owned()),
                    profile_path: Some(profile_path_string.clone()),
                },
            ),
            &state_path,
            &profile_path,
            &FileSystemProfileStore,
        )
        .expect("run RPC should succeed");

        match run_response.result.expect("result should be present") {
            AgentJsonRpcResult::AgentLifecycle { agent } => {
                assert_eq!(agent.name, "echo");
                assert_eq!(agent.status, "running");
            }
            other => panic!("unexpected RPC result: {other:?}"),
        }

        let running_snapshot = execute_agent_read_rpc_with_store_and_paths(
            AgentJsonRpcRequest::new(
                "req-write-run-snapshot",
                METHOD_AGENT_SNAPSHOT,
                AgentJsonRpcParams {
                    agent_name: Some("echo".to_owned()),
                    profile_path: Some(profile_path_string.clone()),
                },
            ),
            &state_path,
            &profile_path,
            &FileSystemProfileStore,
        )
        .expect("snapshot RPC should succeed after run");

        match running_snapshot.result.expect("result should be present") {
            AgentJsonRpcResult::AgentSnapshot { snapshot } => {
                assert_eq!(snapshot.agents.len(), 1);
                assert_eq!(snapshot.agents[0].name, "echo");
                assert_eq!(snapshot.agents[0].status, "running");
            }
            other => panic!("unexpected RPC result: {other:?}"),
        }

        let stop_response = execute_agent_read_rpc_with_store_and_paths(
            AgentJsonRpcRequest::new(
                "req-write-stop",
                METHOD_AGENT_STOP,
                AgentJsonRpcParams {
                    agent_name: Some("echo".to_owned()),
                    profile_path: Some(profile_path_string.clone()),
                },
            ),
            &state_path,
            &profile_path,
            &FileSystemProfileStore,
        )
        .expect("stop RPC should succeed");

        match stop_response.result.expect("result should be present") {
            AgentJsonRpcResult::AgentLifecycle { agent } => {
                assert_eq!(agent.name, "echo");
                assert_eq!(agent.status, "stopped");
            }
            other => panic!("unexpected RPC result: {other:?}"),
        }

        let stopped_snapshot = execute_agent_read_rpc_with_store_and_paths(
            AgentJsonRpcRequest::new(
                "req-write-stop-snapshot",
                METHOD_AGENT_SNAPSHOT,
                AgentJsonRpcParams {
                    agent_name: Some("echo".to_owned()),
                    profile_path: Some(profile_path_string.clone()),
                },
            ),
            &state_path,
            &profile_path,
            &FileSystemProfileStore,
        )
        .expect("snapshot RPC should succeed after stop");

        match stopped_snapshot.result.expect("result should be present") {
            AgentJsonRpcResult::AgentSnapshot { snapshot } => {
                assert_eq!(snapshot.agents.len(), 1);
                assert_eq!(snapshot.agents[0].name, "echo");
                assert_eq!(snapshot.agents[0].status, "stopped");
            }
            other => panic!("unexpected RPC result: {other:?}"),
        }

        cleanup_state_path(&state_path);
        cleanup_profile_artifacts(&profile_path);
        cleanup_parent_dir(&profile_path);
    }

    #[test]
    fn agent_write_rpc_denies_run_without_selected_profile_grant() {
        let state_path = unique_state_path("rpc-write-profile-denied");
        let profile_path = unique_profile_path("rpc-write-profile-denied");

        execute_profile_cli_with_store(
            ProfileCliCommand::Init {
                path: profile_path.clone(),
            },
            &FileSystemProfileStore,
        )
        .expect("profile init should succeed");

        let response = execute_agent_read_rpc_with_store_and_paths(
            AgentJsonRpcRequest::new(
                "req-write-profile-denied",
                METHOD_AGENT_RUN,
                AgentJsonRpcParams {
                    agent_name: Some("echo".to_owned()),
                    profile_path: Some(profile_path.display().to_string()),
                },
            ),
            &state_path,
            &profile_path,
            &FileSystemProfileStore,
        )
        .expect("run RPC should return a JSON-RPC error envelope");

        assert_rpc_error(
            &response,
            JSON_RPC_AUTHORIZATION_DENIED,
            "authorization denied: echo missing agent.echo",
        );

        cleanup_state_path(&state_path);
        cleanup_profile_artifacts(&profile_path);
        cleanup_parent_dir(&profile_path);
    }

    #[test]
    fn agent_write_rpc_requires_agent_name_for_run() {
        let state_path = unique_state_path("rpc-write-missing-agent-name");
        let profile_path = unique_profile_path("rpc-write-missing-agent-name");

        let response = execute_agent_read_rpc_with_store_and_paths(
            AgentJsonRpcRequest::new(
                "req-write-missing-agent-name",
                METHOD_AGENT_RUN,
                AgentJsonRpcParams::default(),
            ),
            &state_path,
            &profile_path,
            &FileSystemProfileStore,
        )
        .expect("missing run params should return a JSON-RPC error");

        assert_rpc_error(
            &response,
            JSON_RPC_INVALID_PARAMS,
            "agentName parameter is required for agent.run",
        );

        cleanup_state_path(&state_path);
        cleanup_profile_artifacts(&profile_path);
    }

    #[test]
    fn agent_write_rpc_denied_run_persists_deny_start_without_mutating_agent_state() {
        let state_path = unique_state_path("rpc-write-denied-start");
        let profile_path = unique_profile_path("rpc-write-denied-start");

        let response = execute_agent_rpc_with_store_and_paths_and_runtime_loader(
            AgentJsonRpcRequest::new(
                "req-write-denied-start",
                METHOD_AGENT_RUN,
                AgentJsonRpcParams::for_agent("echo"),
            ),
            &state_path,
            &profile_path,
            &FileSystemProfileStore,
            |state, _, _| denied_reference_runtime_from_loaded_state(state),
        )
        .expect("denied run should return a JSON-RPC error envelope");

        assert_rpc_error(
            &response,
            JSON_RPC_AUTHORIZATION_DENIED,
            "authorization denied: echo missing agent.echo",
        );

        let describe = execute_agent_read_rpc_with_store_and_paths(
            AgentJsonRpcRequest::new(
                "req-write-denied-describe",
                METHOD_AGENT_DESCRIBE,
                AgentJsonRpcParams::for_agent("echo"),
            ),
            &state_path,
            &profile_path,
            &FileSystemProfileStore,
        )
        .expect("describe RPC should still succeed after denied run");

        match describe.result.expect("result should be present") {
            AgentJsonRpcResult::AgentDetail { agent } => {
                assert_eq!(agent.name, "echo");
                assert_eq!(agent.status, "registered");
            }
            other => panic!("unexpected RPC result: {other:?}"),
        }

        let deny_log = execute_agent_read_rpc_with_store_and_paths(
            AgentJsonRpcRequest::new(
                "req-write-denied-log",
                METHOD_DENY_LOG_LIST,
                AgentJsonRpcParams::for_agent("echo"),
            ),
            &state_path,
            &profile_path,
            &FileSystemProfileStore,
        )
        .expect("deny log RPC should succeed after denied run");

        match deny_log.result.expect("result should be present") {
            AgentJsonRpcResult::DenyLog { entries } => {
                assert_eq!(entries.len(), 1);
                assert_eq!(entries[0].kind, "deniedStart");
                assert_eq!(entries[0].agent_name.as_deref(), Some("echo"));
                assert_eq!(entries[0].capability.as_deref(), Some("agent.echo"));
            }
            other => panic!("unexpected RPC result: {other:?}"),
        }

        cleanup_state_path(&state_path);
        cleanup_profile_artifacts(&profile_path);
    }

    #[test]
    fn shell_route_serves_local_shell_html() {
        let state_path = unique_state_path("shell-html");
        let profile_path = unique_profile_path("shell-html");

        let response = route_shell_request_with_store_and_paths(
            HttpRequest {
                method: "GET".to_owned(),
                path: "/".to_owned(),
                body: Vec::new(),
            },
            &state_path,
            &profile_path,
            &FileSystemProfileStore,
        );

        let html = String::from_utf8(response.body).expect("shell HTML should be valid UTF-8");

        assert_eq!(response.status_code, 200);
        assert!(html.contains("FERROS Local Shell"));
        assert!(html.contains("/rpc"));
        assert!(html.contains("/profile"));
        assert!(html.contains("data-profile-action=\"show\""));
        assert!(html.contains("lifecycle-submit-button"));
        assert!(html.contains("lifecycle-arm-checkbox"));
        assert!(html.contains("data-module=\"LifecycleControlCard\""));
        assert!(html.contains("data-module=\"LifecycleOutcomeCard\""));
        assert!(html.contains("data-module=\"ProposedMaterialCard\""));
        assert!(html.contains("data-module=\"ReceiptStrip\""));
        assert!(html.contains("data-module=\"ConsentBoundaryCard\""));
        assert!(html.contains("data-module=\"RecoveryStateCard\""));
        assert!(html.contains("data-module=\"TouchAnchorStrip\""));
        assert!(html.contains("id=\"home-hub-surface\""));
        assert!(html.contains("id=\"forge-surface\""));
        assert!(html.contains("id=\"arena-surface\""));
        assert!(html.contains("id: 'homeHub'"));
        assert!(html.contains("id: 'forge'"));
        assert!(html.contains("id: 'arena'"));
        assert!(html.contains("data-module=\"HomeHubTopologyCard\""));
        assert!(html.contains("data-module=\"HomeHubBridgeProposalGroup\""));
        assert!(html.contains("data-module=\"ForgePreviewCard\""));
        assert!(html.contains("data-module=\"ForgeAuthoringStripCard\""));
        assert!(html.contains("data-module=\"ExportReadinessCard\""));
        assert!(html.contains("data-module=\"ArenaPreviewCard\""));
        assert!(html.contains("data-module=\"ArenaLifecycleRehearsalStageCard\""));
        assert!(html.contains("data-touch-jump=\"${escapeHtml(anchor.target)}\""));
        assert!(html.contains("target: 'registry-panel'"));
        assert!(html.contains("data-module=\"EvidenceBadge\""));
        assert!(html.contains("data-module=\"SourceLineageCard\""));
        assert!(html.contains("data-module=\"RunwayChecklistRowCard\""));
        assert!(html.contains("data-module=\"ToolLaneCard\""));
        assert!(html.contains("data-module=\"AgentStatusBadge\""));
        assert!(html.contains("data-module=\"CapabilityPillList\""));
        assert!(html.contains("data-module=\"AgentListRowCard\""));
        assert!(html.contains("data-module=\"AgentDetailCard\""));
        assert!(html.contains("data-module=\"GrantDetailCard\""));
        assert!(html.contains("data-module=\"DenyEventDetailCard\""));
        assert!(html.contains("data-module=\"HomeHubSurfaceCard\""));
        assert!(html.contains("data-module=\"ForgeSurfaceCard\""));
        assert!(html.contains("data-module=\"ArenaSurfaceCard\""));
        assert!(html.contains("data-module=\"AgentRowCard\""));
        assert!(html.contains("data-module=\"GrantRowCard\""));
        assert!(html.contains("data-module=\"DenyLogRowCard\""));
        assert!(html.contains("data-module=\"MetricCard\""));
        assert!(html.contains("data-module=\"GrantsSurfaceCard\""));
        assert!(html.contains("data-module=\"ProfileSurfaceCard\""));
        assert!(html.contains("data-surface-state=\"empty\""));
        for marker in [
            "data-protected-chrome=\"top-edge\"",
            "data-protected-chrome=\"bottom-edge\"",
            "data-status-rail=\"primary\"",
            "data-status-rail=\"secondary\"",
            "data-panel-anchor=\"registry\"",
            "data-panel-anchor=\"center\"",
            "data-panel-anchor=\"inspector\"",
            "data-panel-anchor=\"tools\"",
            "data-panel-anchor=\"audit\"",
            "data-panel-collapse=\"registry\"",
            "data-panel-collapse=\"inspector\"",
            "data-panel-collapse=\"tools\"",
            "data-panel-collapse=\"audit\"",
        ] {
            assert!(
                html.contains(marker),
                "expected local shell html to include marker {marker}"
            );
        }

        cleanup_state_path(&state_path);
        cleanup_profile_artifacts(&profile_path);
    }

    #[test]
    fn shell_route_posts_json_rpc_agent_list() {
        let state_path = unique_state_path("shell-rpc");
        let profile_path = unique_profile_path("shell-rpc");
        let request_body = serde_json::to_vec(&AgentJsonRpcRequest::new(
            "req-shell",
            METHOD_AGENT_LIST,
            AgentJsonRpcParams::default(),
        ))
        .expect("request JSON should serialize");

        let response = route_shell_request_with_store_and_paths(
            HttpRequest {
                method: "POST".to_owned(),
                path: "/rpc".to_owned(),
                body: request_body,
            },
            &state_path,
            &profile_path,
            &FileSystemProfileStore,
        );

        let payload: ferros_agents::AgentJsonRpcResponse = serde_json::from_slice(&response.body)
            .expect("shell RPC response should be valid JSON");

        assert_eq!(response.status_code, 200);
        match payload.result.expect("result should be present") {
            AgentJsonRpcResult::AgentList { agents } => {
                assert_eq!(agents.len(), 3);
                assert_eq!(agents[0].name, "echo");
                assert_eq!(agents[1].name, "ha-local-bridge");
            }
            other => panic!("unexpected RPC result: {other:?}"),
        }

        cleanup_state_path(&state_path);
        cleanup_profile_artifacts(&profile_path);
    }

    #[test]
    fn shell_route_gets_local_runway_summary_json() {
        let state_path = unique_state_path("shell-runway-summary");
        let profile_path = unique_profile_path("shell-runway-summary");
        let store = FileSystemProfileStore;

        execute_profile_cli_with_store(
            ProfileCliCommand::Init {
                path: profile_path.clone(),
            },
            &store,
        )
        .expect("profile init should succeed");

        execute_local_agent_api_with_runtime_loader(
            LocalAgentApiCommand::Run {
                name: "echo".to_string(),
            },
            &state_path,
            denied_reference_runtime_from_loaded_state,
        )
        .expect_err("denied runtime should record a deny entry before summary read");

        let response = route_shell_request_with_store_and_paths(
            HttpRequest {
                method: "GET".to_owned(),
                path: format!(
                    "/runway-summary.json?profilePath={}",
                    profile_path.display()
                ),
                body: Vec::new(),
            },
            &state_path,
            &profile_path,
            &store,
        );
        let payload: LocalRunwaySummary =
            serde_json::from_slice(&response.body).expect("runway summary response should parse");

        assert_eq!(response.status_code, 200);
        assert_eq!(response.content_type, "application/json; charset=utf-8");
        assert_eq!(payload.surface, "local-runway-summary");
        assert_eq!(payload.scope, "local-only");
        assert_eq!(payload.checkpoint_state, "runtime-ready");
        assert_eq!(payload.checkpoint_position, 4);
        assert_eq!(payload.checkpoint_total, 7);
        assert_eq!(payload.profile_path, profile_path.display().to_string());
        assert_eq!(payload.deny_count, 1);
        assert!(payload.agent_count >= 2);
        let hub_restart = payload
            .hub_restart
            .as_ref()
            .expect("shell runway summary should include hub restart observation");
        assert_eq!(hub_restart.snapshot_path, LOCAL_HUB_STATE_SNAPSHOT_PATH);
        assert_eq!(hub_restart.scope, "local-only");
        assert_eq!(hub_restart.evidence, "non-evidentiary");
        assert!(matches!(
            hub_restart.reload_status.as_str(),
            "fresh-start" | "reloaded" | "unavailable"
        ));
        assert!(payload
            .checklist
            .iter()
            .any(|item| item.item == "powerCycleStatus"));

        cleanup_state_path(&state_path);
        cleanup_profile_artifacts(&profile_path);
        cleanup_parent_dir(&profile_path);
    }

    #[test]
    fn shell_route_posts_profile_init_and_show_through_local_adapter() {
        let state_path = unique_state_path("shell-profile-init");
        let profile_path = unique_profile_path("shell-profile-init");
        let profile_path_string = profile_path.display().to_string();
        let init_body = serde_json::to_vec(&serde_json::json!({
            "action": "init",
            "profilePath": profile_path_string,
        }))
        .expect("profile init request should serialize");

        let init_response = route_shell_request_with_store_and_paths(
            HttpRequest {
                method: "POST".to_owned(),
                path: "/profile".to_owned(),
                body: init_body,
            },
            &state_path,
            &profile_path,
            &FileSystemProfileStore,
        );
        let init_payload: ProfileShellResponse =
            serde_json::from_slice(&init_response.body).expect("profile response should parse");

        assert_eq!(init_response.status_code, 200);
        assert!(init_payload.ok);
        assert_eq!(init_payload.action, "init");
        assert_eq!(init_payload.status.kind, "complete");
        assert_eq!(
            init_payload.status.summary,
            "Local profile initialized through /profile."
        );
        assert!(init_payload.error_detail.is_none());
        assert_eq!(
            init_payload
                .profile
                .as_ref()
                .expect("profile should return")["identity"]["name"]
                .as_str(),
            Some(DEFAULT_PROFILE_NAME)
        );

        let show_body = serde_json::to_vec(&serde_json::json!({
            "action": "show",
            "profilePath": profile_path.display().to_string(),
        }))
        .expect("profile show request should serialize");
        let show_response = route_shell_request_with_store_and_paths(
            HttpRequest {
                method: "POST".to_owned(),
                path: "/profile".to_owned(),
                body: show_body,
            },
            &state_path,
            &profile_path,
            &FileSystemProfileStore,
        );
        let show_payload: ProfileShellResponse =
            serde_json::from_slice(&show_response.body).expect("profile response should parse");

        assert_eq!(show_response.status_code, 200);
        assert!(show_payload.ok);
        assert_eq!(show_payload.action, "show");
        assert_eq!(show_payload.status.kind, "complete");
        assert_eq!(
            show_payload.status.summary,
            "Local profile document loaded through /profile."
        );
        assert!(show_payload.error_detail.is_none());
        assert!(show_payload
            .lines
            .iter()
            .any(|line| line.contains("\"name\": \"Fresh Start\"")));

        cleanup_state_path(&state_path);
        cleanup_profile_artifacts(&profile_path);
        cleanup_parent_dir(&profile_path);
    }

    #[test]
    fn shell_route_posts_profile_export_and_import_through_local_adapter() {
        let state_path = unique_state_path("shell-profile-bundle");
        let source_path = unique_profile_path("shell-profile-bundle-source");
        let imported_path = unique_profile_path("shell-profile-bundle-imported");
        let bundle_path = unique_profile_path("shell-profile-bundle").with_extension("bundle.json");
        let store = FileSystemProfileStore;

        execute_profile_cli_with_store(
            ProfileCliCommand::Init {
                path: source_path.clone(),
            },
            &store,
        )
        .expect("source profile init should succeed");

        let export_body = serde_json::to_vec(&serde_json::json!({
            "action": "export",
            "profilePath": source_path.display().to_string(),
            "bundlePath": bundle_path.display().to_string(),
        }))
        .expect("profile export request should serialize");
        let export_response = route_shell_request_with_store_and_paths(
            HttpRequest {
                method: "POST".to_owned(),
                path: "/profile".to_owned(),
                body: export_body,
            },
            &state_path,
            &source_path,
            &store,
        );
        let export_payload: ProfileShellResponse =
            serde_json::from_slice(&export_response.body).expect("profile response should parse");

        assert_eq!(export_response.status_code, 200);
        assert!(export_payload.ok);
        assert_eq!(export_payload.action, "export");
        assert_eq!(export_payload.status.kind, "complete");
        assert_eq!(
            export_payload.status.summary,
            "Local profile bundle exported through /profile."
        );
        assert!(export_payload.error_detail.is_none());
        assert!(bundle_path.exists());

        let import_body = serde_json::to_vec(&serde_json::json!({
            "action": "import",
            "profilePath": imported_path.display().to_string(),
            "bundlePath": bundle_path.display().to_string(),
        }))
        .expect("profile import request should serialize");
        let import_response = route_shell_request_with_store_and_paths(
            HttpRequest {
                method: "POST".to_owned(),
                path: "/profile".to_owned(),
                body: import_body,
            },
            &state_path,
            &source_path,
            &store,
        );
        let import_payload: ProfileShellResponse =
            serde_json::from_slice(&import_response.body).expect("profile response should parse");

        assert_eq!(import_response.status_code, 200);
        assert!(import_payload.ok);
        assert_eq!(import_payload.action, "import");
        assert_eq!(import_payload.status.kind, "complete");
        assert_eq!(
            import_payload.status.summary,
            "Local profile bundle imported through /profile."
        );
        assert!(import_payload.error_detail.is_none());
        assert_eq!(
            import_payload
                .profile
                .as_ref()
                .expect("profile should return")["identity"]["name"]
                .as_str(),
            Some(DEFAULT_PROFILE_NAME)
        );

        cleanup_state_path(&state_path);
        cleanup_profile_artifacts(&source_path);
        cleanup_profile_artifacts(&imported_path);
        cleanup_parent_dir(&source_path);
        cleanup_parent_dir(&imported_path);
        let _ = fs::remove_file(&bundle_path);
    }

    #[test]
    fn shell_route_profile_adapter_rejects_grant_mutation_actions() {
        let state_path = unique_state_path("shell-profile-reject-grant");
        let profile_path = unique_profile_path("shell-profile-reject-grant");
        let request_body = serde_json::to_vec(&serde_json::json!({
            "action": "grant",
            "profilePath": profile_path.display().to_string(),
        }))
        .expect("profile grant request should serialize");

        let response = route_shell_request_with_store_and_paths(
            HttpRequest {
                method: "POST".to_owned(),
                path: "/profile".to_owned(),
                body: request_body,
            },
            &state_path,
            &profile_path,
            &FileSystemProfileStore,
        );
        let payload: ProfileShellResponse =
            serde_json::from_slice(&response.body).expect("profile response should parse");

        assert_eq!(response.status_code, 200);
        assert!(!payload.ok);
        assert_eq!(payload.action, "grant");
        assert_eq!(payload.status.kind, "blocked");
        assert_eq!(
            payload.status.summary,
            "Local profile action was blocked on /profile."
        );
        assert_eq!(
            payload
                .error_detail
                .as_ref()
                .expect("error detail should be present")
                .code,
            "mutation_not_exposed"
        );
        assert!(payload
            .error
            .expect("error should be present")
            .contains("not exposed through the localhost profile surface"));

        cleanup_state_path(&state_path);
        cleanup_profile_artifacts(&profile_path);
        cleanup_parent_dir(&profile_path);
    }

    #[test]
    fn shell_route_serves_localhost_acceptance_harness() {
        let state_path = unique_state_path("shell-harness-html");
        let profile_path = unique_profile_path("shell-harness-html");

        let response = route_shell_request_with_store_and_paths(
            HttpRequest {
                method: "GET".to_owned(),
                path: "/harnesses/localhost-shell-acceptance.html".to_owned(),
                body: Vec::new(),
            },
            &state_path,
            &profile_path,
            &FileSystemProfileStore,
        );

        let html = String::from_utf8(response.body).expect("harness HTML should be valid UTF-8");

        assert_eq!(response.status_code, 200);
        assert!(html.contains("FERROS Localhost Shell Acceptance Harness"));
        assert!(html.contains("iframe id=\"sut\" src=\"/\""));
        assert!(html.contains("Profile show uses /profile without sending JSON-RPC"));
        assert!(html
            .contains("Lifecycle gate blocks an unarmed or missing-grant click before write RPC"));
        assert!(html.contains("[data-protected-chrome=\"top-edge\"]"));
        assert!(html.contains("[data-protected-chrome=\"status-rail\"][data-status-rail=\"primary\"]"));
        assert!(html.contains("[data-protected-chrome=\"panel-header\"][data-panel-anchor=\""));
        assert!(html.contains("[data-protected-chrome=\"collapse\"][data-panel-collapse=\""));
        assert!(html.contains("Base shell exposes stable protected-chrome shell and status markers"));
        assert!(html.contains("Major panel headers expose stable extraction anchors"));
        assert!(html.contains("Collapse affordances expose stable extraction anchors"));
        assert!(html.contains("Touch anchor strip exposes persistent section jumps for touch posture"));
        assert!(html.contains("Eight route buttons are visible"));
        assert!(html.contains("Registry list rows expose AgentListRowCard markers with stable data-agent-name selectors"));
        assert!(html.contains("Runway surface exposes the consent boundary through the shared onramp boundary module"));
        assert!(html.contains("Runway surface exposes operator recovery posture through the shared recovery module"));
        assert!(html.contains("Runway checklist rows expose RunwayChecklistRowCard markers with stable data-runway-index mapping"));
        assert!(html.contains("Home-Hub route activates"));
        assert!(html.contains("Forge route activates"));
        assert!(html.contains("Arena route activates"));
        assert!(html.contains("Audit lifecycle surfaces expose shared lifecycle-card modules while preserving the existing lifecycle controls"));

        cleanup_state_path(&state_path);
        cleanup_profile_artifacts(&profile_path);
    }

    #[test]
    fn shell_route_rejects_retired_harness_alias_path() {
        let state_path = unique_state_path("shell-harness-alias-retired");
        let profile_path = unique_profile_path("shell-harness-alias-retired");

        let canonical = route_shell_request_with_store_and_paths(
            HttpRequest {
                method: "GET".to_owned(),
                path: "/harnesses/localhost-shell-acceptance.html".to_owned(),
                body: Vec::new(),
            },
            &state_path,
            &profile_path,
            &FileSystemProfileStore,
        );

        let retired_alias = route_shell_request_with_store_and_paths(
            HttpRequest {
                method: "GET".to_owned(),
                path: "/harnesses/localhost-shell-acceptance-harness.html".to_owned(),
                body: Vec::new(),
            },
            &state_path,
            &profile_path,
            &FileSystemProfileStore,
        );

        assert_eq!(canonical.status_code, 200);
        assert_eq!(retired_alias.status_code, 404);
        assert_eq!(retired_alias.content_type, "text/plain; charset=utf-8");
        assert!(String::from_utf8(retired_alias.body)
            .expect("retired alias error should be valid UTF-8")
            .contains("FERROS local shell route not found"));

        cleanup_state_path(&state_path);
        cleanup_profile_artifacts(&profile_path);
    }

    #[test]
    fn shell_listener_serves_local_shell_html_over_tcp() {
        let listener = TcpListener::bind(("127.0.0.1", 0)).expect("listener should bind");
        let address = listener
            .local_addr()
            .expect("listener should report local addr");

        let server = thread::spawn(move || {
            serve_local_shell_with_listener(listener, Some(1))
                .expect("shell listener should serve one request");
        });

        let mut stream = TcpStream::connect(address).expect("client should connect");
        stream
            .write_all(b"GET / HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n")
            .expect("request should write");
        stream
            .shutdown(Shutdown::Write)
            .expect("client write-half should shut down");

        let response = read_stream_to_string(&mut stream);

        assert!(response.starts_with("HTTP/1.1 200 OK\r\n"));
        assert!(response.contains("Content-Type: text/html; charset=utf-8"));
        assert!(response.contains("Cache-Control: no-store"));
        assert!(response.contains("FERROS Local Shell"));

        server.join().expect("listener thread should exit cleanly");
    }

    #[test]
    fn shell_listener_posts_json_rpc_agent_list_over_tcp() {
        let listener = TcpListener::bind(("127.0.0.1", 0)).expect("listener should bind");
        let address = listener
            .local_addr()
            .expect("listener should report local addr");
        let request = serde_json::to_string(&AgentJsonRpcRequest::new(
            "req-socket",
            METHOD_AGENT_LIST,
            AgentJsonRpcParams::default(),
        ))
        .expect("request should serialize");

        let server = thread::spawn(move || {
            serve_local_shell_with_listener(listener, Some(1))
                .expect("shell listener should serve one request");
        });

        let mut stream = TcpStream::connect(address).expect("client should connect");
        let request_bytes = format!(
            "POST /rpc HTTP/1.1\r\nHost: 127.0.0.1\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            request.len(),
            request,
        );
        stream
            .write_all(request_bytes.as_bytes())
            .expect("request should write");
        stream
            .shutdown(Shutdown::Write)
            .expect("client write-half should shut down");

        let response = read_stream_to_string(&mut stream);
        let payload = parse_http_response_json(&response);

        assert!(response.starts_with("HTTP/1.1 200 OK\r\n"));
        match payload.result.expect("result should be present") {
            AgentJsonRpcResult::AgentList { agents } => {
                assert_eq!(agents.len(), 3);
                assert_eq!(agents[0].name, "echo");
                assert_eq!(agents[1].name, "ha-local-bridge");
                assert_eq!(agents[2].name, "timer");
            }
            other => panic!("unexpected RPC result: {other:?}"),
        }

        server.join().expect("listener thread should exit cleanly");
    }

    #[test]
    fn shell_listener_binds_to_lan_address_over_tcp() {
        // BRIDGE-WORKAROUND: LAN bind — pre-auth, superseded when native bridge lands
        let listener = TcpListener::bind(("0.0.0.0", 0)).expect("listener should bind");
        let address = listener
            .local_addr()
            .expect("listener should report local addr");

        let server = thread::spawn(move || {
            serve_local_shell_with_listener(listener, Some(1))
                .expect("shell listener should serve one request");
        });

        let mut stream = TcpStream::connect(address).expect("client should connect to 0.0.0.0");
        stream
            .write_all(b"GET / HTTP/1.1\r\nHost: 0.0.0.0\r\nConnection: close\r\n\r\n")
            .expect("request should write");
        stream
            .shutdown(Shutdown::Write)
            .expect("client write-half should shut down");

        let response = read_stream_to_string(&mut stream);

        assert!(response.starts_with("HTTP/1.1 200 OK\r\n"));
        assert!(response.contains("FERROS Local Shell"));

        server.join().expect("listener thread should exit cleanly");
    }

    #[test]
    fn shell_listener_posts_json_rpc_agent_snapshot_over_tcp() {
        let listener = TcpListener::bind(("127.0.0.1", 0)).expect("listener should bind");
        let address = listener
            .local_addr()
            .expect("listener should report local addr");
        let request = serde_json::to_string(&AgentJsonRpcRequest::new(
            "req-snapshot-socket",
            METHOD_AGENT_SNAPSHOT,
            AgentJsonRpcParams::default(),
        ))
        .expect("request should serialize");

        let server = thread::spawn(move || {
            serve_local_shell_with_listener(listener, Some(1))
                .expect("shell listener should serve one request");
        });

        let mut stream = TcpStream::connect(address).expect("client should connect");
        let request_bytes = format!(
            "POST /rpc HTTP/1.1\r\nHost: 127.0.0.1\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            request.len(),
            request,
        );
        stream
            .write_all(request_bytes.as_bytes())
            .expect("request should write");
        stream
            .shutdown(Shutdown::Write)
            .expect("client write-half should shut down");

        let response = read_stream_to_string(&mut stream);
        let payload = parse_http_response_json(&response);

        assert!(response.starts_with("HTTP/1.1 200 OK\r\n"));
        match payload.result.expect("result should be present") {
            AgentJsonRpcResult::AgentSnapshot { snapshot } => {
                assert_eq!(snapshot.agents.len(), 3);
                assert_eq!(snapshot.agents[0].name, "echo");
                assert_eq!(snapshot.agents[1].name, "ha-local-bridge");
                assert_eq!(snapshot.agents[2].name, "timer");
            }
            other => panic!("unexpected RPC result: {other:?}"),
        }

        server.join().expect("listener thread should exit cleanly");
    }

    #[test]
    fn shell_listener_posts_json_rpc_invalid_params_error_over_tcp() {
        let listener = TcpListener::bind(("127.0.0.1", 0)).expect("listener should bind");
        let address = listener
            .local_addr()
            .expect("listener should report local addr");
        let request = serde_json::to_string(&AgentJsonRpcRequest::new(
            "req-invalid-params",
            METHOD_AGENT_DESCRIBE,
            AgentJsonRpcParams::default(),
        ))
        .expect("request should serialize");

        let server = thread::spawn(move || {
            serve_local_shell_with_listener(listener, Some(1))
                .expect("shell listener should serve one request");
        });

        let mut stream = TcpStream::connect(address).expect("client should connect");
        let request_bytes = format!(
            "POST /rpc HTTP/1.1\r\nHost: 127.0.0.1\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            request.len(),
            request,
        );
        stream
            .write_all(request_bytes.as_bytes())
            .expect("request should write");
        stream
            .shutdown(Shutdown::Write)
            .expect("client write-half should shut down");

        let response = read_stream_to_string(&mut stream);
        let payload = parse_http_response_json(&response);

        assert!(response.starts_with("HTTP/1.1 200 OK\r\n"));
        assert_rpc_error(
            &payload,
            JSON_RPC_INVALID_PARAMS,
            "agentName parameter is required for agent.describe",
        );

        server.join().expect("listener thread should exit cleanly");
    }

    #[test]
    fn shell_listener_posts_json_rpc_agent_run_over_tcp() {
        let listener = TcpListener::bind(("127.0.0.1", 0)).expect("listener should bind");
        let address = listener
            .local_addr()
            .expect("listener should report local addr");
        let state_path = unique_state_path("shell-rpc-run");
        let profile_path = unique_profile_path("shell-rpc-run");
        execute_profile_cli_with_store(
            ProfileCliCommand::Init {
                path: profile_path.clone(),
            },
            &FileSystemProfileStore,
        )
        .expect("profile init should succeed");
        execute_profile_cli_with_store(
            ProfileCliCommand::Grant {
                path: profile_path.clone(),
                capability: "agent.echo".to_owned(),
            },
            &FileSystemProfileStore,
        )
        .expect("profile grant should succeed");
        let request = serde_json::to_string(&AgentJsonRpcRequest::new(
            "req-run-socket",
            METHOD_AGENT_RUN,
            AgentJsonRpcParams {
                agent_name: Some("echo".to_owned()),
                profile_path: Some(profile_path.display().to_string()),
            },
        ))
        .expect("request should serialize");

        let server_state_path = state_path.clone();
        let server_profile_path = profile_path.clone();
        let server = thread::spawn(move || {
            serve_local_shell_with_store_and_paths(
                listener,
                Some(1),
                &server_state_path,
                &server_profile_path,
                &FileSystemProfileStore,
            )
            .expect("shell listener should serve one request");
        });

        let mut stream = TcpStream::connect(address).expect("client should connect");
        let request_bytes = format!(
            "POST /rpc HTTP/1.1\r\nHost: 127.0.0.1\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            request.len(),
            request,
        );
        stream
            .write_all(request_bytes.as_bytes())
            .expect("request should write");
        stream
            .shutdown(Shutdown::Write)
            .expect("client write-half should shut down");

        let response = read_stream_to_string(&mut stream);
        let payload = parse_http_response_json(&response);

        assert!(response.starts_with("HTTP/1.1 200 OK\r\n"));
        match payload.result.expect("result should be present") {
            AgentJsonRpcResult::AgentLifecycle { agent } => {
                assert_eq!(agent.name, "echo");
                assert_eq!(agent.status, "running");
            }
            other => panic!("unexpected RPC result: {other:?}"),
        }

        server.join().expect("listener thread should exit cleanly");
        cleanup_state_path(&state_path);
        cleanup_profile_artifacts(&profile_path);
        cleanup_parent_dir(&profile_path);
    }

    #[test]
    fn shell_route_returns_not_found_for_unknown_paths() {
        let state_path = unique_state_path("shell-404");
        let profile_path = unique_profile_path("shell-404");

        let response = route_shell_request_with_store_and_paths(
            HttpRequest {
                method: "GET".to_owned(),
                path: "/missing".to_owned(),
                body: Vec::new(),
            },
            &state_path,
            &profile_path,
            &FileSystemProfileStore,
        );

        assert_eq!(response.status_code, 404);

        cleanup_state_path(&state_path);
        cleanup_profile_artifacts(&profile_path);
    }

    #[test]
    fn parse_http_request_preserves_query_string_for_shell_routes() {
        let request = parse_http_request(
            b"GET /runway-summary.json?profilePath=%2Ftmp%2Fprofile.json HTTP/1.1\r\nHost: localhost\r\n\r\n",
        )
        .expect("request should parse");

        assert_eq!(request.method, "GET");
        assert_eq!(
            request.path,
            "/runway-summary.json?profilePath=%2Ftmp%2Fprofile.json"
        );
    }

    #[test]
    fn profile_cli_init_and_show_round_trip_profile_document() {
        let store = FileSystemProfileStore;
        let profile_path = unique_profile_path("init-show");

        let init_lines = execute_profile_cli_with_store(
            ProfileCliCommand::Init {
                path: profile_path.clone(),
            },
            &store,
        )
        .expect("profile init should succeed");

        assert!(init_lines
            .iter()
            .any(|line| line == &format!("initialized profile at {}", profile_path.display())));
        assert!(init_lines
            .iter()
            .any(|line| line == &format!("profile name: {DEFAULT_PROFILE_NAME}")));

        let show_lines = execute_profile_cli_with_store(
            ProfileCliCommand::Show {
                path: profile_path.clone(),
            },
            &store,
        )
        .expect("profile show should succeed");
        let rendered = show_lines.join("\n");
        let profile =
            ProfileDocument::from_json_str(&rendered).expect("show output should be valid JSON");

        assert_eq!(profile.identity.name, DEFAULT_PROFILE_NAME);
        assert!(profile.has_genesis_seal());

        cleanup_state_path(&profile_path);
        cleanup_parent_dir(&profile_path);
    }

    #[test]
    fn profile_cli_init_rejects_existing_profile_file() {
        let store = FileSystemProfileStore;
        let profile_path = unique_profile_path("init-existing");

        execute_profile_cli_with_store(
            ProfileCliCommand::Init {
                path: profile_path.clone(),
            },
            &store,
        )
        .expect("first init should succeed");

        let error = execute_profile_cli_with_store(
            ProfileCliCommand::Init {
                path: profile_path.clone(),
            },
            &store,
        )
        .expect_err("second init should fail");

        assert!(matches!(
            error,
            CliError::Profile(ferros_profile::ProfileStoreError::AlreadyExists(existing_path))
                if existing_path == profile_path
        ));

        cleanup_state_path(&profile_path);
        cleanup_parent_dir(&profile_path);
    }

    #[test]
    fn default_profile_path_uses_profile_file_name() {
        let path = default_profile_path();

        assert_eq!(
            path.file_name().and_then(|value| value.to_str()),
            Some("profile.json")
        );
    }

    fn unique_state_path(test_name: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be after epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("ferros-node-{test_name}-{nonce}.state"))
    }

    fn unique_profile_path(test_name: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be after epoch")
            .as_nanos();

        std::env::temp_dir()
            .join(format!("ferros-node-{test_name}-{nonce}"))
            .join(format!("{test_name}.json"))
    }

    fn cleanup_state_path(path: &Path) {
        let _ = fs::remove_file(path);
    }

    fn cleanup_parent_dir(path: &Path) {
        if let Some(parent) = path.parent() {
            let _ = fs::remove_dir(parent);
        }
    }

    fn cleanup_profile_artifacts(path: &Path) {
        let _ = fs::remove_file(path);
        let _ = fs::remove_file(path.with_extension("key.json"));
        let _ = fs::remove_file(path.with_extension("grants.json"));
        cleanup_parent_dir(path);
    }

    fn read_stream_to_string(stream: &mut TcpStream) -> String {
        let mut response = String::new();
        stream
            .read_to_string(&mut response)
            .expect("response should read as UTF-8");
        response
    }

    fn parse_http_response_json(response: &str) -> AgentJsonRpcResponse {
        let (_, body) = response
            .split_once("\r\n\r\n")
            .expect("HTTP response should contain a header terminator");
        serde_json::from_str(body).expect("HTTP response JSON should deserialize")
    }

    fn assert_rpc_error(response: &AgentJsonRpcResponse, code: i32, message: &str) {
        assert!(
            response.result.is_none(),
            "expected no JSON-RPC result payload"
        );

        let error = response
            .error
            .as_ref()
            .expect("expected JSON-RPC error payload");

        assert_eq!(error.code, code);
        assert_eq!(error.message, message);
    }
}
