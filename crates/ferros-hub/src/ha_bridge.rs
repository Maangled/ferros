#![allow(dead_code)]

use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::path::{Component, Path, PathBuf};

use ferros_agents::{
    AgentManifest, AgentName, AgentRegistry, CapabilityRequirement, InMemoryAgentRegistry,
    RegistryError,
};
use ferros_core::{
    Capability, CapabilityRequest, DenyByDefaultPolicy, PolicyDecision, PolicyDenialReason,
    PolicyEngine,
};
use ferros_profile::{CapabilityGrant, ProfileId};

pub const LOCAL_HUB_ARTIFACT_ROOT: &str = ".tmp/hub/";
pub const SIMULATED_LOCAL_BRIDGE_ARTIFACT_PATH: &str =
    ".tmp/hub/simulated-local-bridge-artifact.json";

const LOCAL_BRIDGE_PROFILE_ID: &str = "hub-local-bridge";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalBridgeAgent {
    pub name: String,
    pub version: String,
    pub required_local_capabilities: Vec<String>,
    pub scope: String,
    pub evidence: String,
}

impl LocalBridgeAgent {
    #[must_use]
    pub fn new_default() -> Self {
        Self {
            name: "ha-local-bridge".to_string(),
            version: "0.1.0".to_string(),
            required_local_capabilities: vec!["bridge.observe".to_string()],
            scope: "local-only".to_string(),
            evidence: "non-evidentiary".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocalBridgeRegistrationError {
    AlreadyRegistered(String),
    InvalidAgentName(String),
}

impl fmt::Display for LocalBridgeRegistrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyRegistered(name) => {
                write!(f, "local bridge agent {} is already registered", name)
            }
            Self::InvalidAgentName(name) => {
                write!(f, "local bridge agent {} is not a valid FERROS agent name", name)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LocalBridgeMetadata {
    scope: String,
    evidence: String,
}

#[derive(Debug, Default)]
pub struct LocalBridgeRegistry {
    agents: InMemoryAgentRegistry,
    metadata: BTreeMap<AgentName, LocalBridgeMetadata>,
}

impl LocalBridgeRegistry {
    pub fn register(
        &mut self,
        agent: LocalBridgeAgent,
    ) -> Result<(), LocalBridgeRegistrationError> {
        let manifest = agent_manifest(&agent)?;
        let agent_name = manifest.name.clone();

        self.agents
            .register(manifest)
            .map_err(|error| match error {
                RegistryError::AlreadyRegistered(name) => {
                    LocalBridgeRegistrationError::AlreadyRegistered(name.as_str().to_string())
                }
            })?;

        self.metadata.insert(
            agent_name,
            LocalBridgeMetadata {
                scope: agent.scope,
                evidence: agent.evidence,
            },
        );

        Ok(())
    }

    #[must_use]
    pub fn list(&self) -> Vec<LocalBridgeAgent> {
        self.agents
            .list()
            .into_iter()
            .filter_map(|summary| {
                let manifest = self.agents.describe(&summary.name).ok().flatten()?;
                let metadata = self.metadata.get(&summary.name)?;

                Some(LocalBridgeAgent {
                    name: summary.name.as_str().to_string(),
                    version: summary.version,
                    required_local_capabilities: manifest
                        .required_capabilities
                        .into_iter()
                        .map(|requirement| requirement.capability)
                        .collect(),
                    scope: metadata.scope.clone(),
                    evidence: metadata.evidence.clone(),
                })
            })
            .collect()
    }

    #[must_use]
    pub fn manifest_for(&self, name: &str) -> Option<AgentManifest> {
        let agent_name = AgentName::new(name.to_string()).ok()?;
        self.agents.describe(&agent_name).ok().flatten()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulatedBridgeArtifact {
    pub bridge_agent_name: String,
    pub stand_in_name: String,
    pub relative_output_path: String,
    pub requested_capability: String,
    pub requested_action: String,
    pub status: LocalBridgeStatus,
    pub scope: String,
    pub evidence: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalBridgeRequest {
    pub stand_in_name: String,
    pub requested_capability: String,
    pub requested_action: String,
}

impl LocalBridgeRequest {
    #[must_use]
    pub fn new(
        stand_in_name: impl Into<String>,
        requested_capability: impl Into<String>,
        requested_action: impl Into<String>,
    ) -> Self {
        Self {
            stand_in_name: stand_in_name.into(),
            requested_capability: requested_capability.into(),
            requested_action: requested_action.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalCapabilitySnapshot {
    pub requester_profile_id: ProfileId,
    pub grants: Vec<CapabilityGrant>,
}

impl LocalCapabilitySnapshot {
    #[must_use]
    pub fn new(requester_profile_id: ProfileId, grants: Vec<CapabilityGrant>) -> Self {
        Self {
            requester_profile_id,
            grants,
        }
    }
}

#[must_use]
pub fn local_bridge_profile_id() -> ProfileId {
    ProfileId::new(LOCAL_BRIDGE_PROFILE_ID)
        .expect("static local bridge profile id should be valid")
}

#[must_use]
pub fn evaluate_local_bridge_policy(
    snapshot: &LocalCapabilitySnapshot,
    request: &LocalBridgeRequest,
) -> PolicyDecision {
    let Ok(capability) = Capability::new(request.requested_capability.clone()) else {
        return PolicyDecision::Denied(PolicyDenialReason::CapabilityNotGranted);
    };

    let policy_request = CapabilityRequest::new(
        snapshot.requester_profile_id.as_str().to_string(),
        capability,
    )
    .expect("validated profile ids should remain valid policy request ids");

    DenyByDefaultPolicy.evaluate(&policy_request, &snapshot.grants)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalBridgeStatus {
    Allowed,
    Denied,
    Error,
}

impl LocalBridgeStatus {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Allowed => "allowed",
            Self::Denied => "denied",
            Self::Error => "error",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocalBridgeExecutionError {
    WorkspaceRootUnavailable,
    InvalidRelativeOutputPath(String),
    Io(std::io::ErrorKind),
}

impl fmt::Display for LocalBridgeExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WorkspaceRootUnavailable => {
                write!(f, "workspace root is unavailable for local bridge artifact output")
            }
            Self::InvalidRelativeOutputPath(path) => {
                write!(f, "invalid local bridge output path: {}", path)
            }
            Self::Io(kind) => write!(f, "local bridge artifact IO failed: {}", kind),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalBridgeReport {
    pub bridge_agent_name: String,
    pub stand_in_name: String,
    pub requested_capability: String,
    pub requested_action: String,
    pub status: LocalBridgeStatus,
    pub artifact_relative_output_path: Option<String>,
    pub summary: String,
    pub scope: String,
    pub evidence: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalBridgeExecution {
    pub artifact: Option<SimulatedBridgeArtifact>,
    pub report: LocalBridgeReport,
    pub error: Option<LocalBridgeExecutionError>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalHubRuntimeSummary {
    pub registered_bridge_agents: usize,
    pub bridge_agent_name: String,
    pub bridge_agent_version: String,
    pub requester_profile_id: String,
    pub stand_in_name: String,
    pub requested_capability: String,
    pub requested_action: String,
    pub policy_decision: PolicyDecision,
    pub status: LocalBridgeStatus,
    pub artifact_relative_output_path: Option<String>,
    pub summary: String,
    pub scope: String,
    pub evidence: String,
}

impl SimulatedBridgeArtifact {
    #[must_use]
    pub fn render_json(&self) -> String {
        format!(
            concat!(
                "{{\n",
                "  \"bridgeAgentName\": \"{}\",\n",
                "  \"standInName\": \"{}\",\n",
                "  \"relativeOutputPath\": \"{}\",\n",
                "  \"requestedCapability\": \"{}\",\n",
                "  \"requestedAction\": \"{}\",\n",
                "  \"status\": \"{}\",\n",
                "  \"scope\": \"{}\",\n",
                "  \"evidence\": \"{}\"\n",
                "}}"
            ),
            self.bridge_agent_name,
            self.stand_in_name,
            self.relative_output_path,
            self.requested_capability,
            self.requested_action,
            self.status.as_str(),
            self.scope,
            self.evidence
        )
    }

    pub fn write_under_repo_root(&self) -> Result<PathBuf, LocalBridgeExecutionError> {
        validate_local_output_path(&self.relative_output_path)?;

        let repo_root = ferros_repo_root()?;
        let absolute_path = repo_root.join(&self.relative_output_path);
        let parent = absolute_path
            .parent()
            .ok_or(LocalBridgeExecutionError::WorkspaceRootUnavailable)?;

        fs::create_dir_all(parent).map_err(|error| LocalBridgeExecutionError::Io(error.kind()))?;
        fs::write(&absolute_path, format!("{}\n", self.render_json()))
            .map_err(|error| LocalBridgeExecutionError::Io(error.kind()))?;

        Ok(absolute_path)
    }
}

#[must_use]
pub fn simulated_local_bridge_artifact(agent: &LocalBridgeAgent) -> SimulatedBridgeArtifact {
    simulated_local_bridge_artifact_with_request(
        agent,
        &LocalBridgeRequest::new("simulated-bridge-entity", "bridge.observe", "report-state"),
        LocalBridgeStatus::Allowed,
        SIMULATED_LOCAL_BRIDGE_ARTIFACT_PATH.to_string(),
    )
}

#[must_use]
pub fn execute_local_bridge_request(
    agent: &LocalBridgeAgent,
    snapshot: &LocalCapabilitySnapshot,
    request: &LocalBridgeRequest,
) -> LocalBridgeExecution {
    execute_local_bridge_request_with_output_path(
        agent,
        snapshot,
        request,
        SIMULATED_LOCAL_BRIDGE_ARTIFACT_PATH.to_string(),
    )
}

pub fn summarize_local_bridge_runway(
    agent: &LocalBridgeAgent,
    snapshot: &LocalCapabilitySnapshot,
    request: &LocalBridgeRequest,
) -> Result<LocalHubRuntimeSummary, LocalBridgeRegistrationError> {
    let mut registry = LocalBridgeRegistry::default();
    registry.register(agent.clone())?;

    let policy_decision = evaluate_local_bridge_policy(snapshot, request);
    let execution = execute_local_bridge_request(agent, snapshot, request);

    Ok(LocalHubRuntimeSummary {
        registered_bridge_agents: registry.list().len(),
        bridge_agent_name: agent.name.clone(),
        bridge_agent_version: agent.version.clone(),
        requester_profile_id: snapshot.requester_profile_id.as_str().to_string(),
        stand_in_name: request.stand_in_name.clone(),
        requested_capability: request.requested_capability.clone(),
        requested_action: request.requested_action.clone(),
        policy_decision,
        status: execution.report.status,
        artifact_relative_output_path: execution.report.artifact_relative_output_path.clone(),
        summary: execution.report.summary.clone(),
        scope: execution.report.scope.clone(),
        evidence: execution.report.evidence.clone(),
    })
}

pub fn default_local_runtime_summary() -> Result<LocalHubRuntimeSummary, LocalBridgeRegistrationError> {
    let bridge_agent = LocalBridgeAgent::new_default();
    let requester_profile_id = local_bridge_profile_id();
    let snapshot = LocalCapabilitySnapshot::new(
        requester_profile_id.clone(),
        vec![CapabilityGrant::new(
            requester_profile_id,
            "bridge.observe",
        )],
    );
    let request = LocalBridgeRequest::new(
        "simulated-bridge-entity",
        "bridge.observe",
        "report-state",
    );

    summarize_local_bridge_runway(&bridge_agent, &snapshot, &request)
}

#[must_use]
pub fn execute_local_bridge_request_with_output_path(
    agent: &LocalBridgeAgent,
    snapshot: &LocalCapabilitySnapshot,
    request: &LocalBridgeRequest,
    relative_output_path: impl Into<String>,
) -> LocalBridgeExecution {
    let policy_decision = evaluate_local_bridge_policy(snapshot, request);
    if !policy_decision.is_allowed() {
        return LocalBridgeExecution {
            artifact: None,
            report: LocalBridgeReport {
                bridge_agent_name: agent.name.clone(),
                stand_in_name: request.stand_in_name.clone(),
                requested_capability: request.requested_capability.clone(),
                requested_action: request.requested_action.clone(),
                status: LocalBridgeStatus::Denied,
                artifact_relative_output_path: None,
                summary: denial_summary(policy_decision, snapshot, request),
                scope: "local-only".to_string(),
                evidence: "non-evidentiary".to_string(),
            },
            error: None,
        };
    }

    let relative_output_path = relative_output_path.into();
    let artifact = simulated_local_bridge_artifact_with_request(
        agent,
        request,
        LocalBridgeStatus::Allowed,
        relative_output_path.clone(),
    );

    match artifact.write_under_repo_root() {
        Ok(_) => LocalBridgeExecution {
            report: LocalBridgeReport {
                bridge_agent_name: agent.name.clone(),
                stand_in_name: request.stand_in_name.clone(),
                requested_capability: request.requested_capability.clone(),
                requested_action: request.requested_action.clone(),
                status: LocalBridgeStatus::Allowed,
                artifact_relative_output_path: Some(relative_output_path),
                summary: format!(
                    "local-only bridge allowed {} for {}",
                    request.requested_action, request.stand_in_name
                ),
                scope: "local-only".to_string(),
                evidence: "non-evidentiary".to_string(),
            },
            artifact: Some(artifact),
            error: None,
        },
        Err(error) => LocalBridgeExecution {
            artifact: None,
            report: LocalBridgeReport {
                bridge_agent_name: agent.name.clone(),
                stand_in_name: request.stand_in_name.clone(),
                requested_capability: request.requested_capability.clone(),
                requested_action: request.requested_action.clone(),
                status: LocalBridgeStatus::Error,
                artifact_relative_output_path: None,
                summary: error_summary(&error),
                scope: "local-only".to_string(),
                evidence: "non-evidentiary".to_string(),
            },
            error: Some(error),
        },
    }
}

fn simulated_local_bridge_artifact_with_request(
    agent: &LocalBridgeAgent,
    request: &LocalBridgeRequest,
    status: LocalBridgeStatus,
    relative_output_path: String,
) -> SimulatedBridgeArtifact {
    SimulatedBridgeArtifact {
        bridge_agent_name: agent.name.clone(),
        stand_in_name: request.stand_in_name.clone(),
        relative_output_path,
        requested_capability: request.requested_capability.clone(),
        requested_action: request.requested_action.clone(),
        status,
        scope: "local-only".to_string(),
        evidence: "non-evidentiary".to_string(),
    }
}

fn agent_manifest(agent: &LocalBridgeAgent) -> Result<AgentManifest, LocalBridgeRegistrationError> {
    let agent_name = AgentName::new(agent.name.clone())
        .map_err(|_| LocalBridgeRegistrationError::InvalidAgentName(agent.name.clone()))?;
    let profile_id = local_bridge_profile_id();
    let required_capabilities = agent
        .required_local_capabilities
        .iter()
        .cloned()
        .map(|capability| CapabilityRequirement::new(profile_id.clone(), capability))
        .collect();

    Ok(AgentManifest::new(
        agent_name,
        agent.version.clone(),
        required_capabilities,
    ))
}

fn denial_summary(
    decision: PolicyDecision,
    snapshot: &LocalCapabilitySnapshot,
    request: &LocalBridgeRequest,
) -> String {
    match decision.denial_reason() {
        Some(PolicyDenialReason::NoGrantsPresented) => format!(
            "local-only bridge denied because capability {} is not granted by any active local grant",
            request.requested_capability
        ),
        Some(PolicyDenialReason::ProfileNotGranted) => format!(
            "local-only bridge denied because capability {} is not granted for profile {}",
            request.requested_capability,
            snapshot.requester_profile_id.as_str()
        ),
        Some(PolicyDenialReason::CapabilityNotGranted) => format!(
            "local-only bridge denied because capability {} is not granted",
            request.requested_capability
        ),
        None => format!(
            "local-only bridge allowed {} for {}",
            request.requested_action, request.stand_in_name
        ),
    }
}

fn ferros_repo_root() -> Result<PathBuf, LocalBridgeExecutionError> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let crates_dir = manifest_dir
        .parent()
        .ok_or(LocalBridgeExecutionError::WorkspaceRootUnavailable)?;
    crates_dir
        .parent()
        .map(Path::to_path_buf)
        .ok_or(LocalBridgeExecutionError::WorkspaceRootUnavailable)
}

fn validate_local_output_path(path: &str) -> Result<(), LocalBridgeExecutionError> {
    if !path.starts_with(LOCAL_HUB_ARTIFACT_ROOT)
        || Path::new(path).is_absolute()
        || path.contains(':')
    {
        return Err(LocalBridgeExecutionError::InvalidRelativeOutputPath(
            path.to_string(),
        ));
    }

    for component in Path::new(path).components() {
        match component {
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return Err(LocalBridgeExecutionError::InvalidRelativeOutputPath(
                    path.to_string(),
                ));
            }
            Component::CurDir | Component::Normal(_) => {}
        }
    }

    Ok(())
}

fn error_summary(error: &LocalBridgeExecutionError) -> String {
    match error {
        LocalBridgeExecutionError::InvalidRelativeOutputPath(_) => {
            "local-only bridge rejected before write because the output path was invalid"
                .to_string()
        }
        LocalBridgeExecutionError::WorkspaceRootUnavailable => {
            "local-only bridge could not resolve the workspace root for reporting"
                .to_string()
        }
        LocalBridgeExecutionError::Io(_) => {
            "local-only bridge encountered an IO error while reporting".to_string()
        }
    }
}