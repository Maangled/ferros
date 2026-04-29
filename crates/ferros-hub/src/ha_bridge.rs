#![allow(dead_code)]

use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::path::{Component, Path, PathBuf};

pub const LOCAL_HUB_ARTIFACT_ROOT: &str = ".tmp/hub/";
pub const SIMULATED_LOCAL_BRIDGE_ARTIFACT_PATH: &str =
    ".tmp/hub/simulated-local-bridge-artifact.json";

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
}

impl fmt::Display for LocalBridgeRegistrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyRegistered(name) => {
                write!(f, "local bridge agent {} is already registered", name)
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct LocalBridgeRegistry {
    agents: BTreeMap<String, LocalBridgeAgent>,
}

impl LocalBridgeRegistry {
    pub fn register(
        &mut self,
        agent: LocalBridgeAgent,
    ) -> Result<(), LocalBridgeRegistrationError> {
        if self.agents.contains_key(&agent.name) {
            return Err(LocalBridgeRegistrationError::AlreadyRegistered(agent.name));
        }

        self.agents.insert(agent.name.clone(), agent);
        Ok(())
    }

    #[must_use]
    pub fn list(&self) -> Vec<LocalBridgeAgent> {
        self.agents.values().cloned().collect()
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
    pub granted_capabilities: Vec<String>,
}

impl LocalCapabilitySnapshot {
    #[must_use]
    pub fn new(granted_capabilities: Vec<String>) -> Self {
        Self {
            granted_capabilities,
        }
    }

    #[must_use]
    pub fn allows(&self, capability: &str) -> bool {
        self.granted_capabilities.iter().any(|granted| granted == capability)
    }
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

#[must_use]
pub fn execute_local_bridge_request_with_output_path(
    agent: &LocalBridgeAgent,
    snapshot: &LocalCapabilitySnapshot,
    request: &LocalBridgeRequest,
    relative_output_path: impl Into<String>,
) -> LocalBridgeExecution {
    if !snapshot.allows(&request.requested_capability) {
        return LocalBridgeExecution {
            artifact: None,
            report: LocalBridgeReport {
                bridge_agent_name: agent.name.clone(),
                stand_in_name: request.stand_in_name.clone(),
                requested_capability: request.requested_capability.clone(),
                requested_action: request.requested_action.clone(),
                status: LocalBridgeStatus::Denied,
                artifact_relative_output_path: None,
                summary: format!(
                    "local-only bridge denied because capability {} is not granted",
                    request.requested_capability
                ),
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