use std::fs;
use std::path::PathBuf;

use ferros_core::{PolicyDecision, PolicyDenialReason};
use ferros_profile::{CapabilityGrant, ProfileId};
use ferros_hub::{
    default_local_runtime_summary, default_local_runtime_summary_with_snapshot_path,
    deny_demo_command_output, evaluate_local_bridge_policy, execute_local_bridge_request,
    execute_local_bridge_request_with_output_path, LocalHubReloadStatus,
    LocalHubStateSnapshot, LocalHubStateSnapshotError,
    local_bridge_profile_id, prove_bridge_command_output, simulated_local_bridge_artifact,
    summarize_local_bridge_runway, summary_command_output, LocalBridgeAgent,
    LocalBridgeExecutionError, LocalBridgeRequest, LocalBridgeRegistrationError,
    LocalBridgeRegistry, LocalBridgeStatus, LocalCapabilitySnapshot,
    LOCAL_HUB_STATE_SNAPSHOT_PATH, SIMULATED_LOCAL_BRIDGE_ARTIFACT_PATH,
};

fn local_snapshot(capabilities: &[&str]) -> LocalCapabilitySnapshot {
    let requester_profile_id = local_bridge_profile_id();
    let grants = capabilities
        .iter()
        .map(|capability| CapabilityGrant::new(requester_profile_id.clone(), *capability))
        .collect();

    LocalCapabilitySnapshot::new(requester_profile_id, grants)
}

fn foreign_snapshot(capabilities: &[&str]) -> LocalCapabilitySnapshot {
    let requester_profile_id = local_bridge_profile_id();
    let foreign_profile_id =
        ProfileId::new("hub-foreign-bridge").expect("foreign bridge profile id should be valid");
    let grants = capabilities
        .iter()
        .map(|capability| CapabilityGrant::new(foreign_profile_id.clone(), *capability))
        .collect();

    LocalCapabilitySnapshot::new(requester_profile_id, grants)
}

fn revoked_snapshot(capability: &str) -> LocalCapabilitySnapshot {
    let requester_profile_id = local_bridge_profile_id();
    let mut grant = CapabilityGrant::new(requester_profile_id.clone(), capability);
    assert!(grant.revoke("2026-04-28T00:00:00Z", "local test revoke"));

    LocalCapabilitySnapshot::new(requester_profile_id, vec![grant])
}

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

fn denied_local_runtime_summary() -> ferros_hub::LocalHubRuntimeSummary {
    let bridge_agent = LocalBridgeAgent::new_default();
    let request = LocalBridgeRequest::new(
        "simulated-bridge-entity",
        "bridge.observe",
        "report-state",
    );

    summarize_local_bridge_runway(
        &bridge_agent,
        &LocalCapabilitySnapshot::new(local_bridge_profile_id(), Vec::new()),
        &request,
    )
    .expect("denied local runtime summary should build successfully")
}

fn allowed_local_runtime_summary() -> ferros_hub::LocalHubRuntimeSummary {
    let bridge_agent = LocalBridgeAgent::new_default();
    let request = LocalBridgeRequest::new(
        "simulated-bridge-entity",
        "bridge.observe",
        "report-state",
    );

    summarize_local_bridge_runway(&bridge_agent, &local_snapshot(&["bridge.observe"]), &request)
        .expect("allowed local runtime summary should build successfully")
}

fn local_state_snapshot_path(name: &str) -> (String, PathBuf) {
    let relative_path = format!(".tmp/hub/{name}-local-state.json");
    let absolute_path = repo_root().join(&relative_path);
    (relative_path, absolute_path)
}

fn write_local_state_fixture(name: &str, content: &str) -> (String, PathBuf) {
    let (relative_path, absolute_path) = local_state_snapshot_path(name);
    if let Some(parent) = absolute_path.parent() {
        fs::create_dir_all(parent).expect("local state fixture directory should be creatable");
    }
    fs::write(&absolute_path, content).expect("local state fixture should be writable");

    (relative_path, absolute_path)
}

struct LocalStateFixtureGuard {
    absolute_path: PathBuf,
}

impl LocalStateFixtureGuard {
    fn new(absolute_path: PathBuf) -> Self {
        Self { absolute_path }
    }
}

impl Drop for LocalStateFixtureGuard {
    fn drop(&mut self) {
        if self.absolute_path.exists() {
            let _ = fs::remove_file(&self.absolute_path);
        }
    }
}

#[test]
fn hub_state_round_trip_persists_local_only_snapshot() {
    let (relative_path, absolute_path) = local_state_snapshot_path("hub_state_round_trip");
    let _cleanup = LocalStateFixtureGuard::new(absolute_path.clone());
    if absolute_path.exists() {
        fs::remove_file(&absolute_path).expect("stale local hub state snapshot should be removable");
    }

    let summary = default_local_runtime_summary()
        .expect("default local runtime summary should build successfully");
    let snapshot = LocalHubStateSnapshot::from_runtime_summary(&summary)
        .expect("local hub state snapshot should derive from the runtime summary");
    let rendered = snapshot
        .render_json()
        .expect("local hub state snapshot should render to JSON");

    assert!(LOCAL_HUB_STATE_SNAPSHOT_PATH.starts_with(".tmp/hub/"));
    assert_eq!(snapshot.bridge_manifest_identity, "ha-local-bridge@0.1.0");
    assert_eq!(snapshot.policy_decision_label, "allowed");
    assert_eq!(
        snapshot.artifact_relative_output_path,
        Some(SIMULATED_LOCAL_BRIDGE_ARTIFACT_PATH.to_string())
    );
    assert_eq!(snapshot.scope, "local-only");
    assert_eq!(snapshot.evidence, "non-evidentiary");
    assert_eq!(snapshot.last_local_summary, summary.summary);
    assert!(rendered.contains("\"bridgeManifestIdentity\": \"ha-local-bridge@0.1.0\""));
    assert!(rendered.contains("\"policyDecisionLabel\": \"allowed\""));
    assert!(rendered.contains("\"scope\": \"local-only\""));
    assert!(rendered.contains("\"evidence\": \"non-evidentiary\""));

    let written_path = snapshot
        .write_under_repo_root(&relative_path)
        .expect("local hub state snapshot should persist under .tmp/hub/");
    assert_eq!(written_path, absolute_path);

    let persisted = fs::read_to_string(&absolute_path)
        .expect("persisted local hub state snapshot should be readable");
    assert_eq!(persisted, format!("{}\n", rendered));

    let loaded = LocalHubStateSnapshot::load_under_repo_root(&relative_path)
        .expect("persisted local hub state snapshot should reload successfully");
    assert_eq!(loaded, snapshot);
}

#[test]
fn hub_state_renders_exact_restart_snapshot_json_contract() {
    let summary = allowed_local_runtime_summary();
    let snapshot = LocalHubStateSnapshot::from_runtime_summary(&summary)
        .expect("allowed local runtime summary should produce a restart snapshot");

    assert_eq!(
        snapshot
            .render_json()
            .expect("restart snapshot should render deterministically"),
        concat!(
            "{\n",
            "  \"bridgeManifestIdentity\": \"ha-local-bridge@0.1.0\",\n",
            "  \"policyDecisionLabel\": \"allowed\",\n",
            "  \"artifactRelativeOutputPath\": \".tmp/hub/simulated-local-bridge-artifact.json\",\n",
            "  \"scope\": \"local-only\",\n",
            "  \"evidence\": \"non-evidentiary\",\n",
            "  \"lastLocalSummary\": \"local-only bridge allowed report-state for simulated-bridge-entity\"\n",
            "}"
        )
    );
}

#[test]
fn hub_state_rejects_absolute_and_parent_traversal_paths() {
    let snapshot = LocalHubStateSnapshot::from_runtime_summary(&denied_local_runtime_summary())
        .expect("denied runtime summary should still produce a valid snapshot");

    assert_eq!(
        snapshot.write_under_repo_root(".tmp/hub/../escape.json"),
        Err(LocalHubStateSnapshotError::InvalidRelativeOutputPath(
            ".tmp/hub/../escape.json".to_string()
        ))
    );
    assert_eq!(
        snapshot.write_under_repo_root("C:\\hub\\local-state.json"),
        Err(LocalHubStateSnapshotError::InvalidRelativeOutputPath(
            "C:\\hub\\local-state.json".to_string()
        ))
    );
}

#[test]
fn hub_state_rejects_malformed_local_state() {
    let (relative_path, absolute_path) = write_local_state_fixture(
        "hub_state_malformed",
        concat!(
            "{\n",
            "  \"scope\": \"local-only\"\n",
            "}\n"
        ),
    );
    let _cleanup = LocalStateFixtureGuard::new(absolute_path);

    let error = LocalHubStateSnapshot::load_under_repo_root(&relative_path)
        .expect_err("missing snapshot fields should be rejected");
    match error {
        LocalHubStateSnapshotError::InvalidLocalState(message) => {
            assert!(message.contains("bridgeManifestIdentity"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn hub_state_rejects_trailing_comma_in_local_state() {
    let (relative_path, absolute_path) = write_local_state_fixture(
        "hub_state_trailing_comma",
        concat!(
            "{\n",
            "  \"bridgeManifestIdentity\": \"ha-local-bridge@0.1.0\",\n",
            "  \"policyDecisionLabel\": \"allowed\",\n",
            "  \"artifactRelativeOutputPath\": \".tmp/hub/simulated-local-bridge-artifact.json\",\n",
            "  \"scope\": \"local-only\",\n",
            "  \"evidence\": \"non-evidentiary\",\n",
            "  \"lastLocalSummary\": \"local-only bridge allowed report-state for simulated-bridge-entity\",\n",
            "}\n"
        ),
    );
    let _cleanup = LocalStateFixtureGuard::new(absolute_path);

    let error = LocalHubStateSnapshot::load_under_repo_root(&relative_path)
        .expect_err("trailing commas should be rejected as malformed local state");
    match error {
        LocalHubStateSnapshotError::InvalidLocalState(message) => {
            assert!(message.contains("trailing comma"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn hub_state_rejects_remote_looking_artifact_paths() {
    let (relative_path, absolute_path) = write_local_state_fixture(
        "hub_state_remote_path",
        concat!(
            "{\n",
            "  \"bridgeManifestIdentity\": \"ha-local-bridge@0.1.0\",\n",
            "  \"policyDecisionLabel\": \"allowed\",\n",
            "  \"artifactRelativeOutputPath\": \"https://example.com/artifact.json\",\n",
            "  \"scope\": \"local-only\",\n",
            "  \"evidence\": \"non-evidentiary\",\n",
            "  \"lastLocalSummary\": \"local-only bridge allowed report-state for simulated-bridge-entity\"\n",
            "}\n"
        ),
    );
    let _cleanup = LocalStateFixtureGuard::new(absolute_path);

    let error = LocalHubStateSnapshot::load_under_repo_root(&relative_path)
        .expect_err("remote-looking artifact paths should be rejected");
    match error {
        LocalHubStateSnapshotError::InvalidLocalState(message) => {
            assert!(message.contains("artifactRelativeOutputPath"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn hub_state_rejects_hardware_proof_launch_wording() {
    let (relative_path, absolute_path) = write_local_state_fixture(
        "hub_state_banned_wording",
        concat!(
            "{\n",
            "  \"bridgeManifestIdentity\": \"ha-local-bridge@0.1.0\",\n",
            "  \"policyDecisionLabel\": \"allowed\",\n",
            "  \"artifactRelativeOutputPath\": \".tmp/hub/simulated-local-bridge-artifact.json\",\n",
            "  \"scope\": \"local-only\",\n",
            "  \"evidence\": \"non-evidentiary\",\n",
            "  \"lastLocalSummary\": \"local-only hardware launch proof summary\"\n",
            "}\n"
        ),
    );
    let _cleanup = LocalStateFixtureGuard::new(absolute_path);

    let error = LocalHubStateSnapshot::load_under_repo_root(&relative_path)
        .expect_err("hardware, proof, and launch wording should be rejected");
    match error {
        LocalHubStateSnapshotError::InvalidLocalState(message) => {
            assert!(message.contains("lastLocalSummary"));
            assert!(
                message.contains("hardware")
                    || message.contains("launch")
                    || message.contains("proof")
            );
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn hub_reload_defaults_when_no_prior_snapshot_exists() {
    let (relative_path, absolute_path) =
        local_state_snapshot_path("hub_reload_defaults_when_no_prior_snapshot_exists");
    let _cleanup = LocalStateFixtureGuard::new(absolute_path.clone());
    if absolute_path.exists() {
        fs::remove_file(&absolute_path)
            .expect("stale local hub reload snapshot should be removable");
    }

    let summary = default_local_runtime_summary_with_snapshot_path(&relative_path)
        .expect("default local runtime summary should build successfully with a fresh snapshot");

    assert_eq!(summary.restart_observation.reload_status, LocalHubReloadStatus::FreshStart);
    assert_eq!(summary.restart_observation.prior_bridge_manifest_identity, None);
    assert_eq!(summary.restart_observation.prior_policy_decision_label, None);
    assert_eq!(summary.restart_observation.prior_artifact_relative_output_path, None);
    assert!(absolute_path.exists());

    let persisted = LocalHubStateSnapshot::load_under_repo_root(&relative_path)
        .expect("fresh summary should persist a snapshot for the next local run");
    assert_eq!(persisted.bridge_manifest_identity, "ha-local-bridge@0.1.0");
    assert_eq!(persisted.policy_decision_label, "allowed");
    assert_eq!(persisted.last_local_summary, summary.summary);
}

#[test]
fn hub_reload_reports_bounded_prior_snapshot_when_present() {
    let (relative_path, absolute_path) =
        local_state_snapshot_path("hub_reload_reports_bounded_prior_snapshot_when_present");
    let _cleanup = LocalStateFixtureGuard::new(absolute_path.clone());
    if absolute_path.exists() {
        fs::remove_file(&absolute_path)
            .expect("stale local hub reload snapshot should be removable");
    }

    let prior_snapshot = LocalHubStateSnapshot::from_runtime_summary(&allowed_local_runtime_summary())
        .expect("allowed local runtime summary should produce a prior snapshot fixture");
    prior_snapshot
        .write_under_repo_root(&relative_path)
        .expect("prior local hub reload snapshot should be writable");

    let summary = default_local_runtime_summary_with_snapshot_path(&relative_path)
        .expect("default local runtime summary should reload a prior snapshot");

    assert_eq!(summary.restart_observation.reload_status, LocalHubReloadStatus::Reloaded);
    assert_eq!(
        summary.restart_observation.prior_bridge_manifest_identity,
        Some(prior_snapshot.bridge_manifest_identity.clone())
    );
    assert_eq!(
        summary.restart_observation.prior_policy_decision_label,
        Some(prior_snapshot.policy_decision_label.clone())
    );
    assert_eq!(
        summary.restart_observation.prior_artifact_relative_output_path,
        prior_snapshot.artifact_relative_output_path.clone()
    );

    let persisted = LocalHubStateSnapshot::load_under_repo_root(&relative_path)
        .expect("reloaded summary should refresh the stored snapshot for the next local run");
    assert_eq!(persisted.policy_decision_label, "allowed");
    assert_eq!(
        persisted.artifact_relative_output_path,
        Some(SIMULATED_LOCAL_BRIDGE_ARTIFACT_PATH.to_string())
    );
    assert_eq!(persisted.last_local_summary, summary.summary);
}

#[test]
fn hub_reload_marks_unavailable_when_prior_snapshot_is_invalid() {
    let (relative_path, absolute_path) = write_local_state_fixture(
        "hub_reload_marks_unavailable_when_prior_snapshot_is_invalid",
        concat!(
            "{\n",
            "  \"bridgeManifestIdentity\": \"ha-local-bridge@0.1.0\",\n",
            "  \"policyDecisionLabel\": \"allowed\",\n",
            "  \"artifactRelativeOutputPath\": \"https://example.com/not-local.json\",\n",
            "  \"scope\": \"local-only\",\n",
            "  \"evidence\": \"non-evidentiary\",\n",
            "  \"lastLocalSummary\": \"local-only bridge allowed report-state for simulated-bridge-entity\"\n",
            "}\n"
        ),
    );
    let _cleanup = LocalStateFixtureGuard::new(absolute_path.clone());

    let summary = default_local_runtime_summary_with_snapshot_path(&relative_path)
        .expect("default local runtime summary should degrade to unavailable for invalid prior state");

    assert_eq!(summary.restart_observation.reload_status, LocalHubReloadStatus::Unavailable);
    assert_eq!(summary.restart_observation.prior_bridge_manifest_identity, None);
    assert_eq!(summary.restart_observation.prior_policy_decision_label, None);
    assert_eq!(summary.restart_observation.prior_artifact_relative_output_path, None);

    let persisted = LocalHubStateSnapshot::load_under_repo_root(&relative_path)
        .expect("invalid prior state should be replaced with a fresh bounded snapshot");
    assert_eq!(persisted.bridge_manifest_identity, "ha-local-bridge@0.1.0");
    assert_eq!(persisted.policy_decision_label, "allowed");
    assert_eq!(
        persisted.artifact_relative_output_path,
        Some(SIMULATED_LOCAL_BRIDGE_ARTIFACT_PATH.to_string())
    );
    assert_eq!(persisted.last_local_summary, summary.summary);
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

    let manifest = registry
        .manifest_for("ha-local-bridge")
        .expect("registered local bridge should expose a FERROS agent manifest");
    assert_eq!(manifest.name.as_str(), "ha-local-bridge");
    assert_eq!(manifest.version, "0.1.0");
    assert_eq!(manifest.required_capabilities.len(), 1);
    assert_eq!(manifest.required_capabilities[0].capability, "bridge.observe");

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
    let snapshot = local_snapshot(&["bridge.observe"]);
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
    let snapshot = LocalCapabilitySnapshot::new(local_bridge_profile_id(), Vec::new());
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
    let snapshot = local_snapshot(&["bridge.observe"]);
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

#[test]
fn bridge_policy_allows_active_matching_grant() {
    let request = LocalBridgeRequest::new(
        "simulated-bridge-entity",
        "bridge.observe",
        "report-state",
    );

    assert_eq!(
        evaluate_local_bridge_policy(&local_snapshot(&["bridge.observe"]), &request),
        PolicyDecision::Allowed
    );
}

#[test]
fn bridge_policy_denies_without_active_grants() {
    let request = LocalBridgeRequest::new(
        "simulated-bridge-entity",
        "bridge.observe",
        "report-state",
    );

    assert_eq!(
        evaluate_local_bridge_policy(
            &LocalCapabilitySnapshot::new(local_bridge_profile_id(), Vec::new()),
            &request,
        ),
        PolicyDecision::Denied(PolicyDenialReason::NoGrantsPresented)
    );
}

#[test]
fn bridge_policy_denies_when_grants_are_for_other_profiles() {
    let request = LocalBridgeRequest::new(
        "simulated-bridge-entity",
        "bridge.observe",
        "report-state",
    );

    assert_eq!(
        evaluate_local_bridge_policy(&foreign_snapshot(&["bridge.observe"]), &request),
        PolicyDecision::Denied(PolicyDenialReason::ProfileNotGranted)
    );
}

#[test]
fn bridge_policy_ignores_revoked_grants() {
    let request = LocalBridgeRequest::new(
        "simulated-bridge-entity",
        "bridge.observe",
        "report-state",
    );

    assert_eq!(
        evaluate_local_bridge_policy(&revoked_snapshot("bridge.observe"), &request),
        PolicyDecision::Denied(PolicyDenialReason::NoGrantsPresented)
    );
}

#[test]
fn hub_summary_records_default_allowed_runway() {
    let summary = default_local_runtime_summary()
        .expect("default local runtime summary should build successfully");

    assert_eq!(summary.registered_bridge_agents, 1);
    assert_eq!(summary.bridge_agent_name, "ha-local-bridge");
    assert_eq!(summary.bridge_agent_version, "0.1.0");
    assert_eq!(summary.requester_profile_id, local_bridge_profile_id().as_str());
    assert_eq!(summary.stand_in_name, "simulated-bridge-entity");
    assert_eq!(summary.requested_capability, "bridge.observe");
    assert_eq!(summary.requested_action, "report-state");
    assert_eq!(summary.policy_decision, PolicyDecision::Allowed);
    assert_eq!(summary.status, LocalBridgeStatus::Allowed);
    assert_eq!(
        summary.artifact_relative_output_path,
        Some(SIMULATED_LOCAL_BRIDGE_ARTIFACT_PATH.to_string())
    );
    assert_eq!(summary.scope, "local-only");
    assert_eq!(summary.evidence, "non-evidentiary");
    assert!(summary.summary.contains("local-only bridge allowed"));
}

#[test]
fn hub_summary_records_denied_policy_state() {
    let bridge_agent = LocalBridgeAgent::new_default();
    let request = LocalBridgeRequest::new(
        "simulated-bridge-entity",
        "bridge.observe",
        "report-state",
    );
    let summary = summarize_local_bridge_runway(
        &bridge_agent,
        &LocalCapabilitySnapshot::new(local_bridge_profile_id(), Vec::new()),
        &request,
    )
    .expect("denied local runtime summary should still build successfully");

    assert_eq!(summary.registered_bridge_agents, 1);
    assert_eq!(
        summary.policy_decision,
        PolicyDecision::Denied(PolicyDenialReason::NoGrantsPresented)
    );
    assert_eq!(summary.status, LocalBridgeStatus::Denied);
    assert_eq!(summary.artifact_relative_output_path, None);
    assert_eq!(summary.scope, "local-only");
    assert_eq!(summary.evidence, "non-evidentiary");
    assert_eq!(
        summary.restart_observation.reload_status,
        LocalHubReloadStatus::NotChecked
    );
    assert!(summary.summary.contains("not granted"));
}

#[test]
fn hub_cli_summary_output_stays_local_only() {
    let output = summary_command_output().expect("summary output should build successfully");

    assert!(output.contains("ferros-hub summary"));
    assert!(output.contains("policyDecision: allowed"));
    assert!(output.contains("scope: local-only"));
    assert!(output.contains("evidence: non-evidentiary"));
    assert!(output.contains("restartReload:"));
}

#[test]
fn hub_cli_prove_bridge_output_mentions_artifact() {
    let output =
        prove_bridge_command_output().expect("prove-bridge output should build successfully");

    assert!(output.contains("ferros-hub bridge-proof: allowed"));
    assert!(output.contains(SIMULATED_LOCAL_BRIDGE_ARTIFACT_PATH));
    assert!(output.contains("local-only"));
    assert!(output.contains("non-evidentiary"));
    assert!(output.contains("restart "));
}

#[test]
fn hub_cli_deny_demo_output_reports_denied_without_artifact() {
    let output = deny_demo_command_output().expect("deny-demo output should build successfully");

    assert!(output.contains("ferros-hub deny-demo: denied:no-grants"));
    assert!(output.contains("without artifact"));
    assert!(output.contains("local-only"));
    assert!(output.contains("non-evidentiary"));
}

#[test]
fn hub_contract_allowed_report_fields_stay_schema_ready() {
    let bridge_agent = LocalBridgeAgent::new_default();
    let request = LocalBridgeRequest::new(
        "simulated-bridge-entity",
        "bridge.observe",
        "report-state",
    );
    let execution = execute_local_bridge_request(&bridge_agent, &local_snapshot(&["bridge.observe"]), &request);

    assert_eq!(execution.report.bridge_agent_name, "ha-local-bridge");
    assert_eq!(execution.report.stand_in_name, "simulated-bridge-entity");
    assert_eq!(execution.report.requested_capability, "bridge.observe");
    assert_eq!(execution.report.requested_action, "report-state");
    assert_eq!(execution.report.status, LocalBridgeStatus::Allowed);
    assert_eq!(
        execution.report.artifact_relative_output_path,
        Some(SIMULATED_LOCAL_BRIDGE_ARTIFACT_PATH.to_string())
    );
    assert_eq!(execution.report.scope, "local-only");
    assert_eq!(execution.report.evidence, "non-evidentiary");
    assert!(execution.report.summary.contains("local-only bridge allowed"));
}

#[test]
fn hub_contract_denied_report_fields_stay_schema_ready() {
    let bridge_agent = LocalBridgeAgent::new_default();
    let request = LocalBridgeRequest::new(
        "simulated-bridge-entity",
        "bridge.observe",
        "report-state",
    );
    let execution = execute_local_bridge_request(
        &bridge_agent,
        &LocalCapabilitySnapshot::new(local_bridge_profile_id(), Vec::new()),
        &request,
    );

    assert_eq!(execution.report.bridge_agent_name, "ha-local-bridge");
    assert_eq!(execution.report.stand_in_name, "simulated-bridge-entity");
    assert_eq!(execution.report.requested_capability, "bridge.observe");
    assert_eq!(execution.report.requested_action, "report-state");
    assert_eq!(execution.report.status, LocalBridgeStatus::Denied);
    assert_eq!(execution.report.artifact_relative_output_path, None);
    assert_eq!(execution.report.scope, "local-only");
    assert_eq!(execution.report.evidence, "non-evidentiary");
    assert!(execution.report.summary.contains("not granted"));
}