#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::convert::Infallible;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use ferros_agents::{
    Agent, AgentManifest, AgentRegistry, AgentStatus, EchoAgent, InMemoryAgentRegistry,
    ReferenceAgentError, RegistryError, TimerAgent,
};
use ferros_core::{
    Capability, CapabilityError, CapabilityGrantView, CapabilityRequest, DenyByDefaultPolicy,
    MessageEnvelope, MessageEnvelopeError, PolicyDecision, PolicyEngine,
    RequesterProfileIdError,
};
use ferros_profile::{
    grant_profile_capability, init_local_profile, revoke_profile_capability, CapabilityGrant,
    FileSystemProfileStore, LocalProfileStore, ProfileId, ProfileIdError, ProfileStoreError,
};
use ferros_runtime::{Executor, InMemoryExecutor, InMemoryMessageBus, MessageBus};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

const DEFAULT_PROFILE_ID: &str = "profile-alpha";
const DEFAULT_PROFILE_NAME: &str = "Fresh Start";
const DEFAULT_PROFILE_DEVICE_LABEL: &str = "ferros-cli";
const CLI_STATE_DIRECTORY: &str = "ferros";
const CLI_STATE_FILE: &str = "agent-center.state";
const CLI_PROFILE_DIRECTORY: &str = ".ferros";
const CLI_PROFILE_FILE: &str = "profile.json";
const PROFILE_REVOKE_REASON: &str = "revoked via ferros profile revoke";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DemoSummary {
    pub started_agents: Vec<String>,
    pub echo_response: String,
    pub timer_event: String,
    pub denied_requests: usize,
    pub log_entries: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DemoError {
    UnknownAgent(String),
    ManifestMissingCapabilities(String),
    AuthorizationDenied(String),
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
            Self::AuthorizationDenied(message) => write!(f, "authorization denied: {message}"),
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
pub enum ProfileCliCommand {
    Init { path: PathBuf },
    Show { path: PathBuf },
    Export { path: PathBuf, bundle_path: PathBuf },
    Import { path: PathBuf, bundle_path: PathBuf },
    Grant { path: PathBuf, capability: String },
    Revoke { path: PathBuf, capability: String },
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

        let message = format!(
            "{name} missing {}",
            missing
                .iter()
                .map(|requirement| requirement.capability.as_str())
                .collect::<Vec<_>>()
                .join(",")
        );
        self.log_entries.push(format!("denied-start:{message}"));
        Err(DemoError::AuthorizationDenied(message))
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

        let message = format!("{name}:{capability}:{decision:?}");
        self.log_entries.push(format!("denied:{message}"));
        Err(DemoError::AuthorizationDenied(message))
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

    fn map_infallible_executor(error: Infallible) -> DemoError {
        match error {}
    }

    fn map_infallible_bus(error: Infallible) -> DemoError {
        match error {}
    }
}

pub fn build_reference_runtime() -> Result<DemoRuntime, DemoError> {
    let profile_id = ProfileId::new(DEFAULT_PROFILE_ID)?;
    let grants = vec![
        CapabilityGrant::new(profile_id.clone(), "agent.echo"),
        CapabilityGrant::new(profile_id.clone(), "agent.timer"),
    ];
    let mut runtime = DemoRuntime::new(grants);

    let echo = EchoAgent::new(profile_id.clone());
    let timer = TimerAgent::new(profile_id);

    runtime.register(echo.manifest(), Box::new(echo))?;
    runtime.register(timer.manifest(), Box::new(timer))?;

    Ok(runtime)
}

pub fn execute_agent_cli(command: AgentCliCommand) -> Result<Vec<String>, CliError> {
    execute_agent_cli_with_state_path(command, &cli_state_path())
}

pub fn execute_profile_cli(command: ProfileCliCommand) -> Result<Vec<String>, CliError> {
    execute_profile_cli_with_store(command, &FileSystemProfileStore)
}

fn execute_agent_cli_with_state_path(
    command: AgentCliCommand,
    state_path: &Path,
) -> Result<Vec<String>, CliError> {
    match command {
        AgentCliCommand::List => {
            let runtime = runtime_with_state(state_path)?;
            let mut lines = vec!["name\tversion\tstatus".to_owned()];

            lines.extend(
                runtime
                    .agent_records()
                    .into_iter()
                    .map(|record| format_agent_summary(&record)),
            );

            Ok(lines)
        }
        AgentCliCommand::Describe { name } => {
            let runtime = runtime_with_state(state_path)?;
            let record = runtime
                .describe_agent(&name)
                .ok_or_else(|| DemoError::UnknownAgent(name.clone()))?;

            Ok(format_agent_description(&record))
        }
        AgentCliCommand::Run { name } => {
            let mut state = CliState::load(state_path)?;
            let mut runtime = runtime_with_state_from_loaded_state(&state)?;
            let record = runtime
                .describe_agent(&name)
                .ok_or_else(|| DemoError::UnknownAgent(name.clone()))?;

            if record.status != AgentStatus::Running {
                runtime.start_agent(&name)?;
                state.set_status(&name, AgentStatus::Running);
                state
                    .log_entries
                    .extend(runtime.log_entries().iter().cloned());
                state.save(state_path)?;
            }

            let status = runtime
                .describe_agent(&name)
                .map(|updated| updated.status)
                .ok_or_else(|| DemoError::UnknownAgent(name.clone()))?;

            Ok(vec![format!("{}\t{}", name, format_agent_status(status))])
        }
        AgentCliCommand::Stop { name } => {
            let mut state = CliState::load(state_path)?;
            let mut runtime = runtime_with_state_from_loaded_state(&state)?;
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

            let status = runtime
                .describe_agent(&name)
                .map(|record| record.status)
                .ok_or_else(|| DemoError::UnknownAgent(name.clone()))?;

            Ok(vec![format!("{}\t{}", name, format_agent_status(status))])
        }
        AgentCliCommand::Logs { name } => {
            let state = CliState::load(state_path)?;
            let entries = if let Some(name) = name {
                state
                    .log_entries
                    .into_iter()
                    .filter(|entry| entry.contains(&name))
                    .collect::<Vec<_>>()
            } else {
                state.log_entries
            };

            if entries.is_empty() {
                Ok(vec!["no log entries".to_owned()])
            } else {
                Ok(entries)
            }
        }
    }
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

fn runtime_with_state(state_path: &Path) -> Result<DemoRuntime, CliError> {
    let state = CliState::load(state_path)?;
    runtime_with_state_from_loaded_state(&state)
}

fn runtime_with_state_from_loaded_state(state: &CliState) -> Result<DemoRuntime, CliError> {
    let mut runtime = build_reference_runtime()?;

    for (name, status) in &state.agent_statuses {
        match status {
            AgentStatus::Registered => {}
            AgentStatus::Running => runtime.start_agent_internal(name, false)?,
            AgentStatus::Stopped => runtime.stop_agent_internal(name, false)?,
            _ => {
                return Err(CliError::InvalidState(format!(
                    "unsupported persisted status for {name}: {}",
                    format_agent_status(*status)
                )));
            }
        }
    }

    Ok(runtime)
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

pub fn run_demo() -> Result<DemoSummary, DemoError> {
    let mut runtime = build_reference_runtime()?;

    runtime.start_agent("echo")?;
    runtime.start_agent("timer")?;

    let echo_response = runtime
        .send_message("echo", "echo", "agent.echo", b"hello")?
        .ok_or(DemoError::MissingEchoResponse)?;

    let denied_requests = match runtime.send_message("echo", "echo", "agent.admin", b"nope") {
        Ok(_) => 0,
        Err(DemoError::AuthorizationDenied(_)) => 1,
        Err(error) => return Err(error),
    };

    let timer_event = runtime
        .poll_agent("timer")?
        .into_iter()
        .next()
        .ok_or(DemoError::MissingTimerEvent)?;

    runtime.stop_agent("echo")?;
    runtime.stop_agent("timer")?;

    Ok(DemoSummary {
        started_agents: runtime.list_agents(),
        echo_response: String::from_utf8_lossy(&echo_response).into_owned(),
        timer_event: String::from_utf8_lossy(&timer_event).into_owned(),
        denied_requests,
        log_entries: runtime.log_entries().to_vec(),
    })
}

#[cfg(test)]
mod tests {
    use super::{
        default_profile_path, execute_agent_cli_with_state_path, execute_profile_cli_with_store,
        run_demo, AgentCliCommand, CliError, DemoError, DemoRuntime, ProfileCliCommand,
        DEFAULT_PROFILE_NAME,
    };
    use ferros_agents::{EchoAgent, TimerAgent};
    use ferros_profile::{CapabilityGrant, FileSystemProfileStore, ProfileDocument, ProfileId};
    use std::fs;
    use std::path::PathBuf;
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
    fn runtime_lists_registered_reference_agents() {
        let profile_id = ProfileId::new("profile-alpha").expect("valid profile id");
        let grants = vec![
            CapabilityGrant::new(profile_id.clone(), "agent.echo"),
            CapabilityGrant::new(profile_id.clone(), "agent.timer"),
        ];
        let mut runtime = DemoRuntime::new(grants);

        let echo = EchoAgent::new(profile_id.clone());
        let timer = TimerAgent::new(profile_id);

        runtime
            .register(echo.manifest(), Box::new(echo))
            .expect("echo should register");
        runtime
            .register(timer.manifest(), Box::new(timer))
            .expect("timer should register");

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

        assert_eq!(
            error,
            DemoError::AuthorizationDenied("echo missing agent.echo".to_string())
        );
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
            DemoError::AuthorizationDenied(
                "echo:agent.echo:Denied(NoGrantsPresented)".to_string(),
            )
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

        assert_eq!(path.file_name().and_then(|value| value.to_str()), Some("profile.json"));
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

    fn cleanup_state_path(path: &PathBuf) {
        let _ = fs::remove_file(path);
    }

    fn cleanup_parent_dir(path: &PathBuf) {
        if let Some(parent) = path.parent() {
            let _ = fs::remove_dir(parent);
        }
    }
}
