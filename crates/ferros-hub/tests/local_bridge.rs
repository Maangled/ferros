#[path = "../src/ha_bridge.rs"]
mod ha_bridge;

use std::fs;
use std::path::PathBuf;

use ha_bridge::{
    execute_local_bridge_request, execute_local_bridge_request_with_output_path,
    simulated_local_bridge_artifact, LocalBridgeAgent, LocalBridgeExecutionError,
    LocalBridgeRequest, LocalBridgeRegistrationError, LocalBridgeRegistry,
    LocalBridgeStatus, LocalCapabilitySnapshot, SIMULATED_LOCAL_BRIDGE_ARTIFACT_PATH,
};

fn repo_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .and_then(|path| path.parent())
        .expect("workspace root should be resolvable")
        .to_path_buf()
}

fn emitted_artifact_path() -> PathBuf {
    repo_root().join(SIMULATED_LOCAL_BRIDGE_ARTIFACT_PATH)
}

#[test]
fn bridge_agent_registers_locally() {
    let mut registry = LocalBridgeRegistry::default();
    let bridge_agent = LocalBridgeAgent::new_default();

    registry
        .register(bridge_agent.clone())
        .expect("default local bridge agent should register once");

    let registered = registry.list();
    assert_eq!(registered.len(), 1);
    assert_eq!(registered[0].name, "ha-local-bridge");
    assert_eq!(registered[0].required_local_capabilities, vec!["bridge.observe"]);
    assert_eq!(registered[0].scope, "local-only");
    assert_eq!(registered[0].evidence, "non-evidentiary");

    let error = registry
        .register(bridge_agent)
        .expect_err("duplicate register should fail");
    assert_eq!(
        error,
        LocalBridgeRegistrationError::AlreadyRegistered("ha-local-bridge".to_string())
    );
}

#[test]
fn bridge_allow_path_emits_local_artifact() {
    let artifact_path = emitted_artifact_path();
    if artifact_path.exists() {
        fs::remove_file(&artifact_path).expect("stale local bridge artifact should be removable");
    }

    let bridge_agent = LocalBridgeAgent::new_default();
    let snapshot = LocalCapabilitySnapshot::new(vec!["bridge.observe".to_string()]);
    let request = LocalBridgeRequest::new(
        "simulated-bridge-entity",
        "bridge.observe",
        "report-state",
    );

    let execution = execute_local_bridge_request(&bridge_agent, &snapshot, &request);
    let artifact = execution
        .artifact
        .as_ref()
        .expect("allow path should emit an artifact");

    assert_eq!(execution.report.status, LocalBridgeStatus::Allowed);
    assert_eq!(
        execution.report.artifact_relative_output_path,
        Some(SIMULATED_LOCAL_BRIDGE_ARTIFACT_PATH.to_string())
    );
    assert_eq!(execution.error, None);
    assert!(artifact_path.exists());

    let rendered = fs::read_to_string(&artifact_path)
        .expect("emitted local bridge artifact should be readable");
    let lowered = rendered.to_ascii_lowercase();

    assert_eq!(rendered, format!("{}\n", artifact.render_json()));
    assert!(rendered.contains("\"requestedCapability\": \"bridge.observe\""));
    assert!(rendered.contains("\"requestedAction\": \"report-state\""));
    assert!(rendered.contains("\"status\": \"allowed\""));
    assert!(rendered.contains("\"scope\": \"local-only\""));
    assert!(rendered.contains("\"evidence\": \"non-evidentiary\""));
    assert!(!lowered.contains("home assistant"));
    assert!(!lowered.contains("dashboard"));
    assert!(!lowered.contains("hardware"));
    assert!(!lowered.contains("launch"));
    assert!(!lowered.contains("proof"));
}

#[test]
fn bridge_denied_capability_reports_without_artifact() {
    let bridge_agent = LocalBridgeAgent::new_default();
    let snapshot = LocalCapabilitySnapshot::new(Vec::new());
    let request = LocalBridgeRequest::new(
        "simulated-bridge-entity",
        "bridge.observe",
        "report-state",
    );

    let execution = execute_local_bridge_request(&bridge_agent, &snapshot, &request);

    assert_eq!(execution.report.status, LocalBridgeStatus::Denied);
    assert_eq!(execution.artifact, None);
    assert_eq!(execution.error, None);
    assert_eq!(execution.report.artifact_relative_output_path, None);
    assert!(execution.report.summary.contains("not granted"));
}

#[test]
fn bridge_error_path_reports_invalid_output_path() {
    let bridge_agent = LocalBridgeAgent::new_default();
    let snapshot = LocalCapabilitySnapshot::new(vec!["bridge.observe".to_string()]);
    let request = LocalBridgeRequest::new(
        "simulated-bridge-entity",
        "bridge.observe",
        "report-state",
    );

    let execution = execute_local_bridge_request_with_output_path(
        &bridge_agent,
        &snapshot,
        &request,
        "../hub/rejected.json",
    );

    assert_eq!(execution.report.status, LocalBridgeStatus::Error);
    assert_eq!(execution.artifact, None);
    assert_eq!(execution.report.artifact_relative_output_path, None);
    assert!(execution.report.summary.contains("rejected before write"));
    assert_eq!(
        execution.error,
        Some(LocalBridgeExecutionError::InvalidRelativeOutputPath(
            "../hub/rejected.json".to_string()
        ))
    );
}

#[test]
fn simulated_bridge_artifact_stays_local_only() {
    let bridge_agent = LocalBridgeAgent::new_default();
    let artifact = simulated_local_bridge_artifact(&bridge_agent);

    assert_eq!(artifact.bridge_agent_name, bridge_agent.name);
    assert_eq!(artifact.stand_in_name, "simulated-bridge-entity");
    assert_eq!(artifact.relative_output_path, SIMULATED_LOCAL_BRIDGE_ARTIFACT_PATH);
    assert!(!artifact.relative_output_path.contains("://"));
    assert_eq!(artifact.requested_capability, "bridge.observe");
    assert_eq!(artifact.requested_action, "report-state");
    assert_eq!(artifact.status, LocalBridgeStatus::Allowed);
    assert_eq!(artifact.scope, "local-only");
    assert_eq!(artifact.evidence, "non-evidentiary");

    let rendered = artifact.render_json();
    let lowered = rendered.to_ascii_lowercase();

    assert!(rendered.contains("\"requestedCapability\": \"bridge.observe\""));
    assert!(rendered.contains("\"requestedAction\": \"report-state\""));
    assert!(rendered.contains("\"status\": \"allowed\""));
    assert!(rendered.contains("\"scope\": \"local-only\""));
    assert!(rendered.contains("\"evidence\": \"non-evidentiary\""));
    assert!(!lowered.contains("http"));
    assert!(!lowered.contains("dashboard"));
    assert!(!lowered.contains("hardware"));
    assert!(!lowered.contains("launch"));
    assert!(!lowered.contains("proof"));
}