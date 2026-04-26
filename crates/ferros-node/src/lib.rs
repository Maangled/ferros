#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::convert::Infallible;
use std::fmt;
use std::fs;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use ferros_agents::{
    Agent, AgentJsonRpcRequest, AgentJsonRpcResponse, AgentJsonRpcResult, AgentManifest,
    AgentRegistry, AgentRpcAgentDetail, AgentRpcAgentSummary, AgentRpcSnapshot, AgentStatus,
    DenyLogEntry, EchoAgent, GrantStateRecord, InMemoryAgentRegistry, ReferenceAgentError,
    RegistryError, TimerAgent,
    JSON_RPC_AGENT_NOT_FOUND, JSON_RPC_INVALID_PARAMS, JSON_RPC_INVALID_REQUEST,
    JSON_RPC_METHOD_NOT_FOUND, METHOD_AGENT_DESCRIBE, METHOD_AGENT_LIST, METHOD_DENY_LOG_LIST,
    METHOD_AGENT_SNAPSHOT, METHOD_GRANT_LIST,
};
use ferros_core::{
    Capability, CapabilityError, CapabilityGrantView, CapabilityRequest, DenyByDefaultPolicy,
    MessageEnvelope, MessageEnvelopeError, PolicyDecision, PolicyEngine, RequesterProfileIdError,
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

    pub fn reference_host() -> Result<Self, DemoError> {
        let profile_id = ProfileId::new(DEFAULT_PROFILE_ID)?;
        let grants = vec![
            CapabilityGrant::new(profile_id.clone(), "agent.echo"),
            CapabilityGrant::new(profile_id.clone(), "agent.timer"),
        ];
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

    pub fn run_reference_demo_cycle(&mut self) -> Result<DemoSummary, DemoError> {
        self.start_agent("echo")?;
        self.start_agent("timer")?;

        let echo_response = self
            .send_message("echo", "echo", "agent.echo", b"hello")?
            .ok_or(DemoError::MissingEchoResponse)?;

        let denied_requests = match self.send_message("echo", "echo", "agent.admin", b"nope")
        {
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
    let mut handled_connections = 0_usize;

    for incoming in listener.incoming() {
        let mut stream = incoming?;

        if let Err(error) = handle_shell_connection(&mut stream) {
            let response = text_response(
                500,
                "Internal Server Error",
                format!("FERROS shell server error: {error}"),
            );
            let _ = write_http_response(&mut stream, response);
        }

        handled_connections += 1;

        if max_connections.is_some_and(|limit| handled_connections >= limit) {
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

fn handle_shell_connection(stream: &mut TcpStream) -> io::Result<()> {
    let Some(request) = read_http_request(stream)? else {
        return Ok(());
    };
    let response = route_shell_request(request);
    write_http_response(stream, response)
}

fn route_shell_request(request: HttpRequest) -> HttpResponse {
    route_shell_request_with_store_and_paths(
        request,
        &cli_state_path(),
        &default_profile_path(),
        &FileSystemProfileStore,
    )
}

fn route_shell_request_with_store_and_paths<S: LocalProfileStore>(
    request: HttpRequest,
    state_path: &Path,
    default_profile_path: &Path,
    store: &S,
) -> HttpResponse {
    match (request.method.as_str(), request.path.as_str()) {
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
        ("POST", "/rpc") => {
            route_shell_rpc_request(request.body, state_path, default_profile_path, store)
        }
        _ => text_response(404, "Not Found", "FERROS local shell route not found"),
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
        path: path.split('?').next().unwrap_or(path).to_owned(),
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

fn execute_agent_read_rpc_with_store_and_paths<S: LocalProfileStore>(
    request: AgentJsonRpcRequest,
    state_path: &Path,
    default_profile_path: &Path,
    store: &S,
) -> Result<AgentJsonRpcResponse, CliError> {
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
            let runtime = runtime_with_state(state_path)?;
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

            let runtime = runtime_with_state(state_path)?;
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
        METHOD_AGENT_SNAPSHOT => {
            let state = CliState::load(state_path)?;
            let runtime = runtime_with_state_from_loaded_state(&state)?;
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
            let profile_path = params
                .profile_path
                .as_deref()
                .map(PathBuf::from)
                .unwrap_or_else(|| default_profile_path.to_path_buf());
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
            let profile_path = params
                .profile_path
                .as_deref()
                .map(PathBuf::from)
                .unwrap_or_else(|| default_profile_path.to_path_buf());
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
    let mut runtime = DemoRuntime::reference_host()?;
    runtime.replay_cli_state(state)?;

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
        default_profile_path, execute_agent_cli_with_state_path, execute_agent_read_rpc_json,
        execute_agent_read_rpc_with_store_and_paths, execute_profile_cli_with_store,
        route_shell_request_with_store_and_paths, run_demo, runtime_with_state,
        serve_local_shell_with_listener, AgentCliCommand, CliError, CliState, DemoError,
        DemoRuntime, HttpRequest, ProfileCliCommand, DEFAULT_PROFILE_NAME,
    };
    use ferros_agents::{
        AgentJsonRpcParams, AgentJsonRpcRequest, AgentJsonRpcResponse, AgentJsonRpcResult,
        AgentStatus, EchoAgent, JSON_RPC_AGENT_NOT_FOUND, JSON_RPC_INVALID_PARAMS,
        JSON_RPC_INVALID_REQUEST, JSON_RPC_METHOD_NOT_FOUND, METHOD_AGENT_DESCRIBE,
        METHOD_AGENT_LIST, METHOD_AGENT_SNAPSHOT, METHOD_DENY_LOG_LIST, METHOD_GRANT_LIST,
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
            DemoError::AuthorizationDenied("echo:agent.echo:Denied(NoGrantsPresented)".to_string(),)
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
        other_state.save(&other_path).expect("other state should save");

        let loaded = CliState::load(&state_path).expect("state should load from the provided path");
        let missing =
            CliState::load(&missing_path).expect("missing state should default to empty");

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
        state.log_entries.push(
            "denied:echo:agent.admin:Denied(NoGrantsPresented)".to_owned(),
        );
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
                assert_eq!(snapshot.deny_log[0].capability.as_deref(), Some("agent.admin"));
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

        cleanup_state_path(&state_path);
        cleanup_profile_artifacts(&profile_path);
    }

    #[test]
    fn shell_listener_serves_local_shell_html_over_tcp() {
        let listener = TcpListener::bind(("127.0.0.1", 0)).expect("listener should bind");
        let address = listener.local_addr().expect("listener should report local addr");

        let server = thread::spawn(move || {
            serve_local_shell_with_listener(listener, Some(1)).expect("shell listener should serve one request");
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
        let address = listener.local_addr().expect("listener should report local addr");
        let request = serde_json::to_string(&AgentJsonRpcRequest::new(
            "req-socket",
            METHOD_AGENT_LIST,
            AgentJsonRpcParams::default(),
        ))
        .expect("request should serialize");

        let server = thread::spawn(move || {
            serve_local_shell_with_listener(listener, Some(1)).expect("shell listener should serve one request");
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
        let address = listener.local_addr().expect("listener should report local addr");
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
        let address = listener.local_addr().expect("listener should report local addr");
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
        assert!(response.result.is_none(), "expected no JSON-RPC result payload");

        let error = response
            .error
            .as_ref()
            .expect("expected JSON-RPC error payload");

        assert_eq!(error.code, code);
        assert_eq!(error.message, message);
    }
}
