mod ha_bridge;

use ferros_core::{PolicyDecision, PolicyDenialReason};

pub use ferros_data::{
    local_hub_relative_json_path_is_valid, local_onramp_banned_wording,
    local_runway_evidence_is_non_evidentiary, local_runway_launch_overclaim_wording,
    local_runway_scope_is_local_only, local_runway_text_looks_remote_like_url,
};

pub use ha_bridge::{
    default_local_runtime_summary,
    default_local_runtime_summary_with_snapshot_path,
    execute_local_bridge_request,
    execute_local_bridge_request_with_output_path,
    evaluate_local_bridge_policy,
    local_bridge_profile_id,
    simulated_local_bridge_artifact,
    LocalBridgeAgent,
    LocalBridgeExecution,
    LocalBridgeExecutionError,
    LocalBridgeRegistrationError,
    LocalBridgeRegistry,
    LocalBridgeReport,
    LocalBridgeRequest,
    LocalBridgeStatus,
    LocalCapabilitySnapshot,
    LocalHubReloadStatus,
    LocalHubRestartObservation,
    LocalHubRuntimeSummary,
    LocalHubStateSnapshot,
    LocalHubStateSnapshotError,
    SimulatedBridgeArtifact,
    summarize_local_bridge_runway,
    LOCAL_HUB_ARTIFACT_ROOT,
    LOCAL_HUB_STATE_SNAPSHOT_PATH,
    SIMULATED_LOCAL_BRIDGE_ARTIFACT_PATH,
};

pub fn cli_help_text() -> &'static str {
    concat!(
        "ferros-hub local proof commands:\n",
        "  summary      Print the typed local runtime summary\n",
        "  prove-bridge Run the allowed local bridge proof and emit the local artifact\n",
        "  deny-demo    Run the denied local bridge proof without emitting an artifact"
    )
}

pub fn summary_command_output() -> Result<String, LocalBridgeRegistrationError> {
    Ok(format_local_runtime_summary(&default_local_runtime_summary()?))
}

pub fn prove_bridge_command_output() -> Result<String, LocalBridgeRegistrationError> {
    let summary = default_local_runtime_summary()?;
    let decision_label = summary
        .local_onramp_decision_receipt
        .as_ref()
        .map(|receipt| receipt.decision_label.as_str())
        .unwrap_or("none");
    let decision_artifact = summary
        .local_onramp_decision_receipt
        .as_ref()
        .map(|receipt| receipt.local_artifact_path.as_str())
        .unwrap_or("none");

    Ok(format!(
        "ferros-hub bridge-proof: {} for {} with artifact {} [{}; {}; restart {}; decision {}; receipt {}]",
        policy_decision_label(summary.policy_decision),
        summary.stand_in_name,
        summary
            .artifact_relative_output_path
            .as_deref()
            .unwrap_or("none"),
        summary.scope,
        summary.evidence,
        summary.restart_observation.reload_status.as_str(),
        decision_label,
        decision_artifact
    ))
}

pub fn deny_demo_command_output() -> Result<String, LocalBridgeRegistrationError> {
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
    )?;

    Ok(format!(
        "ferros-hub deny-demo: {} for {} without artifact [{}; {}]",
        policy_decision_label(summary.policy_decision),
        summary.stand_in_name,
        summary.scope,
        summary.evidence
    ))
}

fn format_local_runtime_summary(summary: &LocalHubRuntimeSummary) -> String {
    format!(
        concat!(
            "ferros-hub summary\n",
            "registeredBridgeAgents: {}\n",
            "bridgeAgent: {}@{}\n",
            "requesterProfileId: {}\n",
            "standInName: {}\n",
            "requestedCapability: {}\n",
            "requestedAction: {}\n",
            "policyDecision: {}\n",
            "bridgeStatus: {}\n",
            "artifact: {}\n",
            "onrampDecisionLabel: {}\n",
            "onrampDecisionArtifact: {}\n",
            "scope: {}\n",
            "evidence: {}\n",
            "restartReload: {}\n",
            "restartPriorBridgeManifest: {}\n",
            "restartPriorPolicyDecision: {}\n",
            "restartPriorArtifact: {}\n",
            "summary: {}"
        ),
        summary.registered_bridge_agents,
        summary.bridge_agent_name,
        summary.bridge_agent_version,
        summary.requester_profile_id,
        summary.stand_in_name,
        summary.requested_capability,
        summary.requested_action,
        policy_decision_label(summary.policy_decision),
        summary.status.as_str(),
        summary
            .artifact_relative_output_path
            .as_deref()
            .unwrap_or("none"),
        summary
            .local_onramp_decision_receipt
            .as_ref()
            .map(|receipt| receipt.decision_label.as_str())
            .unwrap_or("none"),
        summary
            .local_onramp_decision_receipt
            .as_ref()
            .map(|receipt| receipt.local_artifact_path.as_str())
            .unwrap_or("none"),
        summary.scope,
        summary.evidence,
        summary.restart_observation.reload_status.as_str(),
        summary
            .restart_observation
            .prior_bridge_manifest_identity
            .as_deref()
            .unwrap_or("none"),
        summary
            .restart_observation
            .prior_policy_decision_label
            .as_deref()
            .unwrap_or("none"),
        summary
            .restart_observation
            .prior_artifact_relative_output_path
            .as_deref()
            .unwrap_or("none"),
        summary.summary
    )
}

fn policy_decision_label(decision: PolicyDecision) -> &'static str {
    match decision {
        PolicyDecision::Allowed => "allowed",
        PolicyDecision::Denied(PolicyDenialReason::NoGrantsPresented) => "denied:no-grants",
        PolicyDecision::Denied(PolicyDenialReason::ProfileNotGranted) => "denied:profile",
        PolicyDecision::Denied(PolicyDenialReason::CapabilityNotGranted) => {
            "denied:capability"
        }
    }
}

pub fn prepare_default_local_runway(
) -> Result<(usize, SimulatedBridgeArtifact), LocalBridgeRegistrationError> {
    let mut registry = LocalBridgeRegistry::default();
    let bridge_agent = LocalBridgeAgent::new_default();
    registry.register(bridge_agent.clone())?;

    let artifact = simulated_local_bridge_artifact(&bridge_agent);
    Ok((registry.list().len(), artifact))
}