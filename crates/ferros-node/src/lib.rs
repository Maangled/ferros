#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::convert::Infallible;
use std::fmt;
use std::fs;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use ferros_agents::{
    Agent, AgentJsonRpcRequest, AgentJsonRpcResponse, AgentJsonRpcResult, AgentManifest,
    AgentRegistry, AgentRpcAgentDetail, AgentRpcAgentSummary, AgentRpcSnapshot, AgentStatus,
    CapabilityRequirement, DenyLogEntry, EchoAgent, GrantStateRecord, InMemoryAgentRegistry,
    ReferenceAgentError, RegistryError, TimerAgent, JSON_RPC_AGENT_NOT_FOUND,
    JSON_RPC_AUTHORIZATION_DENIED, JSON_RPC_INVALID_PARAMS, JSON_RPC_INVALID_REQUEST,
    JSON_RPC_METHOD_NOT_FOUND, METHOD_AGENT_DESCRIBE, METHOD_AGENT_LIST, METHOD_AGENT_RUN,
    METHOD_AGENT_SNAPSHOT, METHOD_AGENT_STOP, METHOD_DENY_LOG_LIST, METHOD_GRANT_LIST,
};
use ferros_core::{
    Capability, CapabilityError, CapabilityGrantView, CapabilityRequest, DenyByDefaultPolicy,
    MessageEnvelope, MessageEnvelopeError, PolicyDecision, PolicyEngine, RequesterProfileIdError,
};
use ferros_hub::{
    default_local_runtime_summary, LocalBridgeRegistrationError, LocalHubRuntimeSummary,
    LOCAL_HUB_STATE_SNAPSHOT_PATH,
};
use ferros_profile::{
    grant_profile_capability, init_local_profile, revoke_profile_capability, CapabilityGrant,
    FileSystemProfileStore, LocalProfileStore, ProfileId, ProfileIdError, ProfileStoreError,
};
use ferros_runtime::{
    Executor, InMemoryExecutor, InMemoryMessageBus, LocalRunwayState, MessageBus,
};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

const DEFAULT_PROFILE_ID: &str = "profile-alpha";
const DEFAULT_PROFILE_NAME: &str = "Fresh Start";
const DEFAULT_PROFILE_DEVICE_LABEL: &str = "ferros-cli";
const CLI_STATE_DIRECTORY: &str = "ferros";
const CLI_STATE_FILE: &str = "agent-center.state";
const CLI_PROFILE_DIRECTORY: &str = ".ferros";
const CLI_PROFILE_FILE: &str = "profile.json";
const PROFILE_REVOKE_REASON: &str = "revoked via ferros profile revoke";
const LOCAL_SHELL_DEFAULT_PORT: u16 = 4317;
const MAX_HTTP_REQUEST_BYTES: usize = 64 * 1024;
const LOCAL_SHELL_HTML: &str = include_str!("../../../site/agent-center-shell.html");
const LOCAL_SHELL_ACCEPTANCE_HARNESS_HTML: &str =
    include_str!("../../../harnesses/localhost-shell-acceptance-harness.html");

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
    executor: InMemoryExecutor<MessageEnvelope>,
    bus: InMemoryMessageBus,
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
        let timer = TimerAgent::new(profile_id);

        runtime.register(echo.manifest(), Box::new(echo))?;
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

pub fn serve_local_shell(port: u16) -> io::Result<()> {
    let listener = TcpListener::bind(("127.0.0.1", port))?;

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
    let (request_path, request_query) = split_request_path(&request.path);

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
    let header = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\nCache-Control: no-store\r\n\r\n",
        response.status_code,
        response.status_text,
        response.content_type,
        response.body.len()
    );

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

fn execute_profile_shell_request_with_store<S: LocalProfileStore>(
    request: ProfileShellRequest,
    default_profile_path: &Path,
    store: &S,
) -> ProfileShellResponse {
    let action = request.action.trim().to_ascii_lowercase();
    let profile_path = requested_profile_path(request.profile_path.as_deref(), default_profile_path);
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
        "export" | "import" if error.contains("bundlePath is required") => {
            "bundle_path_required"
        }
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
    let latest_deny = deny_entries.into_iter().last().map(LocalRunwayDenySummary::from);
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
                    "profile observation failed before deny visibility could be checked"
                        .to_owned(),
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
        detail: "power-cycle observation remains pending on this local-only surface"
            .to_owned(),
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
    let has_runtime_observation = matches!(
        stand_in_agent.status,
        LocalRunwayChecklistStatus::Observed
    );
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

    if running_agent_count > 0 && checkpoint.ordinal() >= LocalRunwayState::RuntimeReady.ordinal()
    {
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
                    detail: format!(
                        "no local profile observed at {}",
                        profile_path.display()
                    ),
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
            let profile_path = requested_profile_path(params.profile_path.as_deref(), default_profile_path);
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
            let profile_path = requested_profile_path(params.profile_path.as_deref(), default_profile_path);
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
            let profile_path = requested_profile_path(params.profile_path.as_deref(), default_profile_path);

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
            let profile_path = requested_profile_path(params.profile_path.as_deref(), default_profile_path);
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
            let profile_path = requested_profile_path(params.profile_path.as_deref(), default_profile_path);
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
            let mut runtime =
                DemoRuntime::reference_host_with_grants(Vec::new()).map_err(CliError::from)?;
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
        build_local_runway_summary_with_store_and_hub_summary_loader,
        default_profile_path, execute_agent_cli_with_runtime_loader,
        execute_agent_cli_with_state_path, execute_agent_read_rpc_json,
        execute_agent_read_rpc_with_store_and_paths,
        execute_agent_rpc_with_store_and_paths_and_runtime_loader,
        execute_local_agent_api_with_runtime_loader, execute_profile_cli_with_store,
        parse_http_request,
        route_shell_request_with_store_and_paths, run_demo, runtime_with_state,
        serve_local_shell_with_listener, serve_local_shell_with_store_and_paths, AgentCliCommand,
        AuthorizationDenyDetail, CliError, CliState, DemoError, DemoRuntime, HttpRequest,
        LocalAgentApi, LocalAgentApiCommand, LocalAgentApiResponse,
        LocalRunwayChecklistStatus, LocalRunwaySummary, ProfileCliCommand,
        ProfileShellResponse,
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
            vec!["echo".to_string(), "timer".to_string()]
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
            vec!["echo".to_string(), "timer".to_string()]
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
            vec!["echo".to_string(), "timer".to_string()]
        );
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
                assert_eq!(agents.len(), 2);
                assert_eq!(agents[0].manifest.name.as_str(), "echo");
                assert_eq!(agents[0].status, AgentStatus::Registered);
                assert_eq!(agents[1].manifest.name.as_str(), "timer");
                assert_eq!(agents[1].status, AgentStatus::Registered);
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
        assert_eq!(missing_summary.profile_path, profile_path.display().to_string());
        assert_eq!(missing_summary.agent_count, 2);
        assert_eq!(missing_summary.deny_count, 0);
        assert!(missing_summary.latest_deny.is_none());
        assert_eq!(missing_hub_restart.snapshot_path, LOCAL_HUB_STATE_SNAPSHOT_PATH);
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

        assert_eq!(observed_summary.profile_name.as_deref(), Some(DEFAULT_PROFILE_NAME));
        assert_eq!(observed_summary.grant_count, 0);
        assert_eq!(observed_summary.deny_count, 1);
        assert_eq!(observed_summary.checkpoint_state, "runtime-ready");
        assert_eq!(
            observed_summary.checkpoint_detail,
            "Consent checkpoint observed; runtime activation pending."
        );
        assert_eq!(observed_summary.checkpoint_position, 4);
        assert_eq!(observed_summary.checkpoint_total, 7);
        assert_eq!(observed_profile.status, LocalRunwayChecklistStatus::Observed);
        assert_eq!(observed_consent.status, LocalRunwayChecklistStatus::Observed);
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
        let payload: LocalRunwaySummary = serde_json::from_slice(&response.body)
            .expect("runway summary response should parse");
        let hub_onramp_proposal = payload
            .hub_onramp_proposal
            .as_ref()
            .expect("shell runway summary should include a hub onramp proposal child");

        assert_eq!(response.status_code, 200);
        assert_eq!(hub_onramp_proposal.source, SIMULATED_LOCAL_BRIDGE_ARTIFACT_PATH);
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
        let payload: LocalRunwaySummary = serde_json::from_slice(&response.body)
            .expect("runway summary response should parse");
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
                assert_eq!(agents.len(), 2);
                assert_eq!(agents[0].name, "echo");
                assert_eq!(agents[0].status, "registered");
                assert_eq!(agents[1].name, "timer");
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
                assert_eq!(agents.len(), 2);
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
                assert_eq!(agents.len(), 2);
                assert_eq!(agents[0].name, "echo");
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
        let payload: LocalRunwaySummary = serde_json::from_slice(&response.body)
            .expect("runway summary response should parse");

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
            init_payload.profile.as_ref().expect("profile should return")["identity"]["name"]
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
            import_payload.profile.as_ref().expect("profile should return")["identity"]["name"]
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
                assert_eq!(agents.len(), 2);
                assert_eq!(agents[0].name, "echo");
                assert_eq!(agents[1].name, "timer");
            }
            other => panic!("unexpected RPC result: {other:?}"),
        }

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
                assert_eq!(snapshot.agents.len(), 2);
                assert_eq!(snapshot.agents[0].name, "echo");
                assert_eq!(snapshot.agents[1].name, "timer");
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
            .join("ferros-node-profiles")
            .join(format!("{test_name}-{nonce}.json"))
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
