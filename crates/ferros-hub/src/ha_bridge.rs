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
pub const LOCAL_HUB_STATE_SNAPSHOT_PATH: &str = ".tmp/hub/local-hub-state-snapshot.json";
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
pub enum LocalHubStateSnapshotError {
    WorkspaceRootUnavailable,
    InvalidRelativeOutputPath(String),
    InvalidLocalState(String),
    Io(std::io::ErrorKind),
}

impl fmt::Display for LocalHubStateSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WorkspaceRootUnavailable => {
                write!(f, "workspace root is unavailable for local hub state output")
            }
            Self::InvalidRelativeOutputPath(path) => {
                write!(f, "invalid local hub state path: {}", path)
            }
            Self::InvalidLocalState(reason) => write!(f, "invalid local hub state: {}", reason),
            Self::Io(kind) => write!(f, "local hub state IO failed: {}", kind),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalHubReloadStatus {
    NotChecked,
    FreshStart,
    Reloaded,
    Unavailable,
}

impl LocalHubReloadStatus {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotChecked => "not-checked",
            Self::FreshStart => "fresh-start",
            Self::Reloaded => "reloaded",
            Self::Unavailable => "unavailable",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalHubRestartObservation {
    pub reload_status: LocalHubReloadStatus,
    pub prior_bridge_manifest_identity: Option<String>,
    pub prior_policy_decision_label: Option<String>,
    pub prior_artifact_relative_output_path: Option<String>,
}

impl LocalHubRestartObservation {
    fn not_checked() -> Self {
        Self {
            reload_status: LocalHubReloadStatus::NotChecked,
            prior_bridge_manifest_identity: None,
            prior_policy_decision_label: None,
            prior_artifact_relative_output_path: None,
        }
    }

    fn fresh_start() -> Self {
        Self {
            reload_status: LocalHubReloadStatus::FreshStart,
            prior_bridge_manifest_identity: None,
            prior_policy_decision_label: None,
            prior_artifact_relative_output_path: None,
        }
    }

    fn reloaded(snapshot: &LocalHubStateSnapshot) -> Self {
        Self {
            reload_status: LocalHubReloadStatus::Reloaded,
            prior_bridge_manifest_identity: Some(snapshot.bridge_manifest_identity.clone()),
            prior_policy_decision_label: Some(snapshot.policy_decision_label.clone()),
            prior_artifact_relative_output_path: snapshot.artifact_relative_output_path.clone(),
        }
    }

    fn unavailable() -> Self {
        Self {
            reload_status: LocalHubReloadStatus::Unavailable,
            prior_bridge_manifest_identity: None,
            prior_policy_decision_label: None,
            prior_artifact_relative_output_path: None,
        }
    }
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
    pub restart_observation: LocalHubRestartObservation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalHubStateSnapshot {
    pub bridge_manifest_identity: String,
    pub policy_decision_label: String,
    pub artifact_relative_output_path: Option<String>,
    pub scope: String,
    pub evidence: String,
    pub last_local_summary: String,
}

impl LocalHubStateSnapshot {
    pub fn from_runtime_summary(
        summary: &LocalHubRuntimeSummary,
    ) -> Result<Self, LocalHubStateSnapshotError> {
        let snapshot = Self {
            bridge_manifest_identity: format!(
                "{}@{}",
                summary.bridge_agent_name, summary.bridge_agent_version
            ),
            policy_decision_label: local_policy_decision_label(summary.policy_decision).to_string(),
            artifact_relative_output_path: summary.artifact_relative_output_path.clone(),
            scope: summary.scope.clone(),
            evidence: summary.evidence.clone(),
            last_local_summary: summary.summary.clone(),
        };

        snapshot.validate()?;
        Ok(snapshot)
    }

    pub fn render_json(&self) -> Result<String, LocalHubStateSnapshotError> {
        self.validate()?;

        Ok(format!(
            concat!(
                "{{\n",
                "  \"bridgeManifestIdentity\": \"{}\",\n",
                "  \"policyDecisionLabel\": \"{}\",\n",
                "  \"artifactRelativeOutputPath\": {},\n",
                "  \"scope\": \"{}\",\n",
                "  \"evidence\": \"{}\",\n",
                "  \"lastLocalSummary\": \"{}\"\n",
                "}}"
            ),
            escape_json_string(&self.bridge_manifest_identity),
            escape_json_string(&self.policy_decision_label),
            self.artifact_relative_output_path
                .as_deref()
                .map(|path| format!("\"{}\"", escape_json_string(path)))
                .unwrap_or_else(|| "null".to_string()),
            escape_json_string(&self.scope),
            escape_json_string(&self.evidence),
            escape_json_string(&self.last_local_summary)
        ))
    }

    pub fn write_under_repo_root(
        &self,
        relative_output_path: &str,
    ) -> Result<PathBuf, LocalHubStateSnapshotError> {
        validate_local_snapshot_output_path(relative_output_path)?;

        let repo_root = ferros_repo_root().map_err(map_bridge_error_to_snapshot_error)?;
        let absolute_path = repo_root.join(relative_output_path);
        let parent = absolute_path
            .parent()
            .ok_or(LocalHubStateSnapshotError::WorkspaceRootUnavailable)?;
        let rendered = self.render_json()?;

        fs::create_dir_all(parent).map_err(|error| LocalHubStateSnapshotError::Io(error.kind()))?;
        fs::write(&absolute_path, format!("{}\n", rendered))
            .map_err(|error| LocalHubStateSnapshotError::Io(error.kind()))?;

        Ok(absolute_path)
    }

    pub fn load_under_repo_root(
        relative_output_path: &str,
    ) -> Result<Self, LocalHubStateSnapshotError> {
        validate_local_snapshot_output_path(relative_output_path)?;

        let repo_root = ferros_repo_root().map_err(map_bridge_error_to_snapshot_error)?;
        let content = fs::read_to_string(repo_root.join(relative_output_path))
            .map_err(|error| LocalHubStateSnapshotError::Io(error.kind()))?;

        Self::parse_json(&content)
    }

    fn parse_json(content: &str) -> Result<Self, LocalHubStateSnapshotError> {
        let trimmed = content.trim();
        if !trimmed.starts_with('{') || !trimmed.ends_with('}') {
            return Err(invalid_local_hub_state("snapshot must be a JSON object"));
        }

        let inner = &trimmed[1..trimmed.len() - 1];
        let mut fields = BTreeMap::new();

        for entry in split_json_object_entries(inner)? {
            let (key_raw, value_raw) = split_json_entry(entry)?;
            let key = parse_json_string_literal("field name", key_raw)?;

            if fields
                .insert(key.clone(), value_raw.trim().to_string())
                .is_some()
            {
                return Err(invalid_local_hub_state(format!(
                    "duplicate field {}",
                    key
                )));
            }
        }

        let snapshot = Self {
            bridge_manifest_identity: parse_required_json_string(
                &mut fields,
                "bridgeManifestIdentity",
            )?,
            policy_decision_label: parse_required_json_string(
                &mut fields,
                "policyDecisionLabel",
            )?,
            artifact_relative_output_path: parse_optional_json_string(
                &mut fields,
                "artifactRelativeOutputPath",
            )?,
            scope: parse_required_json_string(&mut fields, "scope")?,
            evidence: parse_required_json_string(&mut fields, "evidence")?,
            last_local_summary: parse_required_json_string(&mut fields, "lastLocalSummary")?,
        };

        if let Some(unexpected_field) = fields.keys().next() {
            return Err(invalid_local_hub_state(format!(
                "unexpected field {}",
                unexpected_field
            )));
        }

        snapshot.validate()?;
        Ok(snapshot)
    }

    fn validate(&self) -> Result<(), LocalHubStateSnapshotError> {
        validate_manifest_identity(&self.bridge_manifest_identity)?;

        validate_snapshot_text_field("policyDecisionLabel", &self.policy_decision_label)?;
        if !matches!(
            self.policy_decision_label.as_str(),
            "allowed" | "denied:no-grants" | "denied:profile" | "denied:capability"
        ) {
            return Err(invalid_local_hub_state(
                "policyDecisionLabel must match the local bridge policy labels",
            ));
        }

        match self.artifact_relative_output_path.as_deref() {
            Some(path) => validate_local_snapshot_artifact_path(path)?,
            None => {}
        }

        if self.scope != "local-only" {
            return Err(invalid_local_hub_state("scope must remain local-only"));
        }
        if self.evidence != "non-evidentiary" {
            return Err(invalid_local_hub_state(
                "evidence must remain non-evidentiary",
            ));
        }

        validate_snapshot_text_field("scope", &self.scope)?;
        validate_snapshot_text_field("evidence", &self.evidence)?;
        validate_snapshot_text_field("lastLocalSummary", &self.last_local_summary)?;

        Ok(())
    }
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
        restart_observation: LocalHubRestartObservation::not_checked(),
    })
}

pub fn default_local_runtime_summary() -> Result<LocalHubRuntimeSummary, LocalBridgeRegistrationError> {
    default_local_runtime_summary_with_snapshot_path(LOCAL_HUB_STATE_SNAPSHOT_PATH)
}

pub fn default_local_runtime_summary_with_snapshot_path(
    snapshot_relative_output_path: &str,
) -> Result<LocalHubRuntimeSummary, LocalBridgeRegistrationError> {
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

    let summary = summarize_local_bridge_runway(&bridge_agent, &snapshot, &request)?;

    Ok(observe_local_hub_restart(summary, snapshot_relative_output_path))
}

fn observe_local_hub_restart(
    mut summary: LocalHubRuntimeSummary,
    snapshot_relative_output_path: &str,
) -> LocalHubRuntimeSummary {
    let mut restart_observation = load_local_hub_restart_observation(snapshot_relative_output_path);

    match LocalHubStateSnapshot::from_runtime_summary(&summary) {
        Ok(snapshot) => {
            if snapshot
                .write_under_repo_root(snapshot_relative_output_path)
                .is_err()
            {
                restart_observation = LocalHubRestartObservation::unavailable();
            }
        }
        Err(_) => restart_observation = LocalHubRestartObservation::unavailable(),
    }

    summary.restart_observation = restart_observation;
    summary
}

fn load_local_hub_restart_observation(
    snapshot_relative_output_path: &str,
) -> LocalHubRestartObservation {
    match LocalHubStateSnapshot::load_under_repo_root(snapshot_relative_output_path) {
        Ok(snapshot) => LocalHubRestartObservation::reloaded(&snapshot),
        Err(LocalHubStateSnapshotError::Io(std::io::ErrorKind::NotFound)) => {
            LocalHubRestartObservation::fresh_start()
        }
        Err(_) => LocalHubRestartObservation::unavailable(),
    }
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
    validate_local_hub_relative_path(path)
        .map_err(LocalBridgeExecutionError::InvalidRelativeOutputPath)
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

fn local_policy_decision_label(decision: PolicyDecision) -> &'static str {
    match decision {
        PolicyDecision::Allowed => "allowed",
        PolicyDecision::Denied(PolicyDenialReason::NoGrantsPresented) => "denied:no-grants",
        PolicyDecision::Denied(PolicyDenialReason::ProfileNotGranted) => "denied:profile",
        PolicyDecision::Denied(PolicyDenialReason::CapabilityNotGranted) => {
            "denied:capability"
        }
    }
}

fn validate_local_snapshot_output_path(path: &str) -> Result<(), LocalHubStateSnapshotError> {
    validate_local_hub_relative_path(path)
        .map_err(LocalHubStateSnapshotError::InvalidRelativeOutputPath)
}

fn validate_local_snapshot_artifact_path(path: &str) -> Result<(), LocalHubStateSnapshotError> {
    validate_local_hub_relative_path(path).map_err(|_| {
        invalid_local_hub_state(format!(
            "artifactRelativeOutputPath must remain under {}",
            LOCAL_HUB_ARTIFACT_ROOT
        ))
    })
}

fn validate_local_hub_relative_path(path: &str) -> Result<(), String> {
    if !path.starts_with(LOCAL_HUB_ARTIFACT_ROOT)
        || Path::new(path).is_absolute()
        || path.contains(':')
    {
        return Err(path.to_string());
    }

    for component in Path::new(path).components() {
        match component {
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return Err(path.to_string());
            }
            Component::CurDir | Component::Normal(_) => {}
        }
    }

    Ok(())
}

fn validate_manifest_identity(identity: &str) -> Result<(), LocalHubStateSnapshotError> {
    validate_snapshot_text_field("bridgeManifestIdentity", identity)?;

    let Some((name, version)) = identity.split_once('@') else {
        return Err(invalid_local_hub_state(
            "bridgeManifestIdentity must be formatted as name@version",
        ));
    };

    if name.is_empty() || version.is_empty() {
        return Err(invalid_local_hub_state(
            "bridgeManifestIdentity must be formatted as name@version",
        ));
    }

    Ok(())
}

fn validate_snapshot_text_field(
    field_name: &str,
    value: &str,
) -> Result<(), LocalHubStateSnapshotError> {
    if value.trim().is_empty() {
        return Err(invalid_local_hub_state(format!(
            "{} must not be empty",
            field_name
        )));
    }

    let lowered = value.to_ascii_lowercase();
    if looks_remote_like_url(&lowered) {
        return Err(invalid_local_hub_state(format!(
            "{} must not contain remote-looking URLs",
            field_name
        )));
    }

    if let Some(wording) = banned_local_state_wording(&lowered) {
        return Err(invalid_local_hub_state(format!(
            "{} must not contain {} wording",
            field_name, wording
        )));
    }

    Ok(())
}

fn looks_remote_like_url(value: &str) -> bool {
    value.contains("://")
        || value.starts_with("//")
        || value.contains("http://")
        || value.contains("https://")
        || value.contains("ws://")
        || value.contains("wss://")
        || value.contains("ftp://")
        || value.contains("file://")
}

fn banned_local_state_wording(value: &str) -> Option<&'static str> {
    ["hardware", "proof", "launch"]
        .into_iter()
        .find(|wording| value.contains(wording))
}

fn invalid_local_hub_state(reason: impl Into<String>) -> LocalHubStateSnapshotError {
    LocalHubStateSnapshotError::InvalidLocalState(reason.into())
}

fn map_bridge_error_to_snapshot_error(
    error: LocalBridgeExecutionError,
) -> LocalHubStateSnapshotError {
    match error {
        LocalBridgeExecutionError::WorkspaceRootUnavailable => {
            LocalHubStateSnapshotError::WorkspaceRootUnavailable
        }
        LocalBridgeExecutionError::InvalidRelativeOutputPath(path) => {
            LocalHubStateSnapshotError::InvalidRelativeOutputPath(path)
        }
        LocalBridgeExecutionError::Io(kind) => LocalHubStateSnapshotError::Io(kind),
    }
}

fn parse_required_json_string(
    fields: &mut BTreeMap<String, String>,
    field_name: &str,
) -> Result<String, LocalHubStateSnapshotError> {
    let raw_value = fields.remove(field_name).ok_or_else(|| {
        invalid_local_hub_state(format!("missing field {}", field_name))
    })?;

    parse_json_string_literal(field_name, &raw_value)
}

fn parse_optional_json_string(
    fields: &mut BTreeMap<String, String>,
    field_name: &str,
) -> Result<Option<String>, LocalHubStateSnapshotError> {
    let raw_value = fields.remove(field_name).ok_or_else(|| {
        invalid_local_hub_state(format!("missing field {}", field_name))
    })?;

    if raw_value.trim() == "null" {
        return Ok(None);
    }

    parse_json_string_literal(field_name, &raw_value).map(Some)
}

fn split_json_object_entries<'a>(content: &'a str) -> Result<Vec<&'a str>, LocalHubStateSnapshotError> {
    let mut entries = Vec::new();
    let mut start = 0;
    let mut in_string = false;
    let mut escaped = false;

    for (index, character) in content.char_indices() {
        if in_string {
            if escaped {
                escaped = false;
                continue;
            }

            match character {
                '\\' => escaped = true,
                '"' => in_string = false,
                _ => {}
            }

            continue;
        }

        match character {
            '"' => in_string = true,
            ',' => {
                let entry = content[start..index].trim();
                if entry.is_empty() {
                    return Err(invalid_local_hub_state(
                        "snapshot JSON contained an empty field entry",
                    ));
                }

                entries.push(entry);
                start = index + character.len_utf8();
            }
            _ => {}
        }
    }

    if in_string || escaped {
        return Err(invalid_local_hub_state(
            "snapshot JSON contained an unterminated string",
        ));
    }

    let tail = content[start..].trim();
    if tail.is_empty() {
        if content.trim_end().ends_with(',') {
            return Err(invalid_local_hub_state(
                "snapshot JSON contained a trailing comma",
            ));
        }
    } else {
        entries.push(tail);
    }

    Ok(entries)
}

fn split_json_entry<'a>(entry: &'a str) -> Result<(&'a str, &'a str), LocalHubStateSnapshotError> {
    let mut in_string = false;
    let mut escaped = false;

    for (index, character) in entry.char_indices() {
        if in_string {
            if escaped {
                escaped = false;
                continue;
            }

            match character {
                '\\' => escaped = true,
                '"' => in_string = false,
                _ => {}
            }

            continue;
        }

        match character {
            '"' => in_string = true,
            ':' => return Ok((&entry[..index], &entry[index + character.len_utf8()..])),
            _ => {}
        }
    }

    Err(invalid_local_hub_state(
        "snapshot JSON field was missing a separator",
    ))
}

fn parse_json_string_literal(
    field_name: &str,
    value: &str,
) -> Result<String, LocalHubStateSnapshotError> {
    let trimmed = value.trim();
    if !trimmed.starts_with('"') || !trimmed.ends_with('"') || trimmed.len() < 2 {
        return Err(invalid_local_hub_state(format!(
            "{} must be a JSON string",
            field_name
        )));
    }

    let mut parsed = String::new();
    let mut characters = trimmed[1..trimmed.len() - 1].chars();

    while let Some(character) = characters.next() {
        if character != '\\' {
            parsed.push(character);
            continue;
        }

        let escaped = characters.next().ok_or_else(|| {
            invalid_local_hub_state(format!(
                "{} contained an unterminated escape sequence",
                field_name
            ))
        })?;

        match escaped {
            '"' => parsed.push('"'),
            '\\' => parsed.push('\\'),
            'n' => parsed.push('\n'),
            'r' => parsed.push('\r'),
            't' => parsed.push('\t'),
            _ => {
                return Err(invalid_local_hub_state(format!(
                    "{} contained an unsupported escape sequence",
                    field_name
                )));
            }
        }
    }

    Ok(parsed)
}

fn escape_json_string(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());

    for character in value.chars() {
        match character {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            _ => escaped.push(character),
        }
    }

    escaped
}