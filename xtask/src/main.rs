use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::Path;
use std::process::{Command, ExitCode};

use ferros_data::{
    LocalArtifactRole, LocalEnvelopeKind, LocalPushArtifact, LocalPushAuditEnvelope,
    LocalPushObservation, LocalPushScope, LocalPushSurface,
    BURST_LOCAL_PUSH_ENVELOPE_PATH, LOCAL_ONRAMP_DECISION_RECEIPT_ARTIFACT_PATH,
    LOCAL_ONRAMP_PROPOSAL_ARTIFACT_PATH,
};
use ferros_hub::{
    LocalBridgeStatus, LocalHubReloadStatus, LocalHubRuntimeSummary,
    LOCAL_HUB_STATE_SNAPSHOT_PATH,
    SIMULATED_LOCAL_BRIDGE_ARTIFACT_PATH,
};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

fn main() -> ExitCode {
    match parse_command(env::args_os().skip(1)) {
        CommandKind::Ci => match run_ci() {
            Ok(()) => ExitCode::SUCCESS,
            Err(message) => {
                eprintln!("xtask ci failed: {message}");
                ExitCode::FAILURE
            }
        },
        CommandKind::Burst => {
            match run_burst() {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(message) => {
                    eprintln!("xtask burst failed: {message}");
                    ExitCode::FAILURE
                }
            }
        }
        CommandKind::HubRunway { keep_artifacts } => match run_hub_runway(keep_artifacts) {
            Ok(output) => {
                println!("{output}");
                ExitCode::SUCCESS
            }
            Err(message) => {
                eprintln!("xtask hub-runway failed: {message}");
                ExitCode::FAILURE
            }
        },
        CommandKind::Help => {
            print!("{HELP_TEXT}");
            ExitCode::SUCCESS
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum CommandKind {
    Ci,
    Burst,
    HubRunway { keep_artifacts: bool },
    Help,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct HubRunwayArtifactSpec {
    label: &'static str,
    path: &'static str,
}

#[derive(Debug)]
struct HubArtifactSnapshot {
    spec: HubRunwayArtifactSpec,
    original_contents: Option<Vec<u8>>,
}

#[derive(Debug)]
struct HubArtifactCleanup {
    snapshots: Vec<HubArtifactSnapshot>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum HubArtifactCleanupMode {
    Restored,
    Kept,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum HubArtifactCleanupAction {
    RestoredPrevious,
    RemovedGenerated,
    KeptForInspection,
}

#[derive(Debug)]
struct HubArtifactCleanupSummary {
    mode: HubArtifactCleanupMode,
    actions: Vec<(HubRunwayArtifactSpec, HubArtifactCleanupAction)>,
}

const HUB_RUNWAY_ARTIFACTS: [HubRunwayArtifactSpec; 4] = [
    HubRunwayArtifactSpec {
        label: "hubBridgeArtifact",
        path: SIMULATED_LOCAL_BRIDGE_ARTIFACT_PATH,
    },
    HubRunwayArtifactSpec {
        label: "hubRestartSnapshotArtifact",
        path: LOCAL_HUB_STATE_SNAPSHOT_PATH,
    },
    HubRunwayArtifactSpec {
        label: "hubOnrampProposalArtifact",
        path: LOCAL_ONRAMP_PROPOSAL_ARTIFACT_PATH,
    },
    HubRunwayArtifactSpec {
        label: "hubOnrampDecisionArtifact",
        path: LOCAL_ONRAMP_DECISION_RECEIPT_ARTIFACT_PATH,
    },
];

fn parse_command<I>(args: I) -> CommandKind
where
    I: IntoIterator<Item = OsString>,
{
    let mut args = args.into_iter();
    match args.next().and_then(|arg| arg.into_string().ok()) {
        Some(command) if command == "ci" => CommandKind::Ci,
        Some(command) if command == "burst" => CommandKind::Burst,
        Some(command) if command == "hub-runway" => {
            let mut keep_artifacts = false;

            for arg in args {
                match arg.to_string_lossy().as_ref() {
                    "--keep-artifacts" => keep_artifacts = true,
                    _ => return CommandKind::Help,
                }
            }

            CommandKind::HubRunway { keep_artifacts }
        }
        _ => CommandKind::Help,
    }
}

impl HubArtifactCleanup {
    fn capture(workspace_root: &Path) -> Result<Self, String> {
        let mut snapshots = Vec::with_capacity(HUB_RUNWAY_ARTIFACTS.len());

        for spec in HUB_RUNWAY_ARTIFACTS {
            let artifact_path = workspace_root.join(spec.path);
            let original_contents = match fs::read(&artifact_path) {
                Ok(contents) => Some(contents),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
                Err(error) => {
                    return Err(format!(
                        "could not snapshot {} before hub-runway: {error}",
                        spec.path
                    ));
                }
            };

            snapshots.push(HubArtifactSnapshot {
                spec,
                original_contents,
            });
        }

        Ok(Self { snapshots })
    }

    fn keep(self) -> HubArtifactCleanupSummary {
        HubArtifactCleanupSummary {
            mode: HubArtifactCleanupMode::Kept,
            actions: self
                .snapshots
                .into_iter()
                .map(|snapshot| (snapshot.spec, HubArtifactCleanupAction::KeptForInspection))
                .collect(),
        }
    }

    fn restore(self, workspace_root: &Path) -> Result<HubArtifactCleanupSummary, String> {
        let mut actions = Vec::with_capacity(self.snapshots.len());

        for snapshot in self.snapshots {
            let artifact_path = workspace_root.join(snapshot.spec.path);
            let action = match snapshot.original_contents {
                Some(contents) => {
                    if let Some(parent) = artifact_path.parent() {
                        fs::create_dir_all(parent).map_err(|error| {
                            format!(
                                "could not recreate artifact directory for {}: {error}",
                                snapshot.spec.path
                            )
                        })?;
                    }

                    fs::write(&artifact_path, contents).map_err(|error| {
                        format!(
                            "could not restore {} after hub-runway: {error}",
                            snapshot.spec.path
                        )
                    })?;
                    HubArtifactCleanupAction::RestoredPrevious
                }
                None => {
                    match fs::remove_file(&artifact_path) {
                        Ok(()) => {}
                        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                        Err(error) => {
                            return Err(format!(
                                "could not remove generated {} after hub-runway: {error}",
                                snapshot.spec.path
                            ));
                        }
                    }
                    HubArtifactCleanupAction::RemovedGenerated
                }
            };

            actions.push((snapshot.spec, action));
        }

        Ok(HubArtifactCleanupSummary {
            mode: HubArtifactCleanupMode::Restored,
            actions,
        })
    }
}

impl HubArtifactCleanupMode {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Restored => "restored",
            Self::Kept => "kept",
        }
    }
}

impl HubArtifactCleanupAction {
    fn as_str(&self) -> &'static str {
        match self {
            Self::RestoredPrevious => "restored-previous",
            Self::RemovedGenerated => "removed-generated",
            Self::KeptForInspection => "kept-for-inspection",
        }
    }
}

impl HubArtifactCleanupSummary {
    fn render(&self) -> String {
        let mut lines = vec![format!("hubArtifactCleanupMode: {}", self.mode.as_str())];

        for (spec, action) in &self.actions {
            lines.push(format!("{}Cleanup: {}", spec.label, action.as_str()));
        }

        lines.join("\n")
    }
}

fn run_ci() -> Result<(), String> {
    run_step("cargo", &["fmt", "--all", "--check"])?;
    run_step(
        "cargo",
        &[
            "clippy",
            "--workspace",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ],
    )?;
    run_step("cargo", &["build", "--workspace", "--all-targets"])?;
    run_step("cargo", &["test", "--workspace", "--all-targets"])?;
    Ok(())
}

fn run_burst() -> Result<String, String> {
    let created_at = OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .map_err(|error| format!("could not format burst timestamp: {error}"))?;
    let envelope = LocalPushAuditEnvelope::new(
        LocalEnvelopeKind::LocalPush,
        created_at,
        LocalPushScope {
            batch_id: Some("BATCH-2026-04-28-OWNER".to_owned()),
            wave_id: Some("WAVE-2026-04-28-32".to_owned()),
            lane_id: Some("xtask-burst".to_owned()),
            stream: "S6".to_owned(),
            surface: LocalPushSurface::PushDigest,
            reason: "burst helper emitted typed local-push envelope output".to_owned(),
        },
        vec![
            LocalPushArtifact {
                path: "xtask/src/main.rs".to_owned(),
                role: LocalArtifactRole::Anchor,
                digest_ref: None,
            },
            LocalPushArtifact {
                path: ".tmp/push/PUSH-MANIFEST.md".to_owned(),
                role: LocalArtifactRole::Input,
                digest_ref: None,
            },
            LocalPushArtifact {
                path: BURST_LOCAL_PUSH_ENVELOPE_PATH.to_owned(),
                role: LocalArtifactRole::Output,
                digest_ref: None,
            },
        ],
    )
    .map_err(|error| format!("could not build local-push envelope: {error:?}"))?
    .with_observation(LocalPushObservation {
        target: "burst-helper".to_owned(),
        status: "observed",
        summary: Some("typed local-push envelope emitted under .tmp/push".to_owned()),
    })
    .with_note("local-only non-partner-facing burst helper output");

    let output_path = env::current_dir()
        .map_err(|error| format!("could not resolve workspace root: {error}"))?
        .join(BURST_LOCAL_PUSH_ENVELOPE_PATH);

    envelope
        .write_json(&output_path)
        .map_err(|error| format!("could not write local-push envelope: {error:?}"))?;

    Ok(format!(
        "{BURST_TEXT}\nTyped local-push helper output:\n    - {BURST_LOCAL_PUSH_ENVELOPE_PATH}\n"
    ))
}

fn run_hub_runway(keep_artifacts: bool) -> Result<String, String> {
    let workspace_root = env::current_dir()
        .map_err(|error| format!("could not resolve workspace root: {error}"))?;
    let cleanup = HubArtifactCleanup::capture(&workspace_root)?;

    let run_result = run_hub_runway_inner(&workspace_root);
    let cleanup_result = if keep_artifacts {
        Ok(cleanup.keep())
    } else {
        cleanup.restore(&workspace_root)
    };
    let inventory_result = validate_no_unexpected_hub_artifacts(&workspace_root);

    match (run_result, cleanup_result, inventory_result) {
        (Ok(output), Ok(cleanup_summary), Ok(())) => Ok(format!(
            "{output}\n{}\nhubUnexpectedArtifacts: none",
            cleanup_summary.render()
        )),
        (Ok(_), Err(cleanup_error), Ok(())) => Err(cleanup_error),
        (Ok(_), Ok(_), Err(inventory_error)) => Err(inventory_error),
        (Ok(_), Err(cleanup_error), Err(inventory_error)) => Err(format!(
            "{cleanup_error}; unexpected artifact inventory failed: {inventory_error}"
        )),
        (Err(run_error), Ok(_), Ok(())) => Err(run_error),
        (Err(run_error), Err(cleanup_error), Ok(())) => {
            Err(format!("{run_error}; cleanup failed: {cleanup_error}"))
        }
        (Err(run_error), Ok(_), Err(inventory_error)) => Err(format!(
            "{run_error}; unexpected artifact inventory failed: {inventory_error}"
        )),
        (Err(run_error), Err(cleanup_error), Err(inventory_error)) => Err(format!(
            "{run_error}; cleanup failed: {cleanup_error}; unexpected artifact inventory failed: {inventory_error}"
        )),
    }
}

fn validate_no_unexpected_hub_artifacts(workspace_root: &Path) -> Result<(), String> {
    let hub_dir = workspace_root.join(".tmp/hub");
    if !hub_dir.exists() {
        return Ok(());
    }

    let mut discovered = Vec::new();
    collect_relative_file_paths(&hub_dir, workspace_root, &mut discovered)?;
    discovered.sort();

    let expected_paths: Vec<&str> = HUB_RUNWAY_ARTIFACTS.iter().map(|spec| spec.path).collect();
    let unexpected: Vec<String> = discovered
        .into_iter()
        .filter(|path| !expected_paths.iter().any(|expected| expected == &path.as_str()))
        .collect();

    if unexpected.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "unexpected .tmp/hub artifacts remained after hub-runway: {}",
            unexpected.join(", ")
        ))
    }
}

fn collect_relative_file_paths(
    current_dir: &Path,
    workspace_root: &Path,
    discovered: &mut Vec<String>,
) -> Result<(), String> {
    for entry in fs::read_dir(current_dir)
        .map_err(|error| format!("could not read {}: {error}", current_dir.display()))?
    {
        let entry = entry.map_err(|error| {
            format!("could not read directory entry in {}: {error}", current_dir.display())
        })?;
        let entry_path = entry.path();

        if entry_path.is_dir() {
            collect_relative_file_paths(&entry_path, workspace_root, discovered)?;
            continue;
        }

        let relative_path = entry_path
            .strip_prefix(workspace_root)
            .map_err(|error| {
                format!(
                    "could not convert {} into a workspace-relative path: {error}",
                    entry_path.display()
                )
            })?
            .to_string_lossy()
            .replace('\\', "/");
        discovered.push(relative_path);
    }

    Ok(())
}

fn run_hub_runway_inner(workspace_root: &Path) -> Result<String, String> {
    let first_summary = ferros_hub::default_local_runtime_summary()
        .map_err(|error| format!("could not build local hub runtime summary: {error}"))?;
    validate_hub_runway_summary("first", &first_summary, workspace_root)?;

    if !matches!(
        first_summary.restart_observation.reload_status,
        LocalHubReloadStatus::FreshStart | LocalHubReloadStatus::Reloaded
    ) {
        return Err(format!(
            "expected first summary restart reload status to be fresh-start or reloaded, got {}",
            first_summary.restart_observation.reload_status.as_str()
        ));
    }

    let second_summary = ferros_hub::default_local_runtime_summary()
        .map_err(|error| format!("could not build local hub runtime summary: {error}"))?;
    validate_hub_runway_summary("second", &second_summary, workspace_root)?;

    if second_summary.restart_observation.reload_status != LocalHubReloadStatus::Reloaded {
        return Err(format!(
            "expected second summary restart reload status to be reloaded, got {}",
            second_summary.restart_observation.reload_status.as_str()
        ));
    }

    let proposal = second_summary
        .local_onramp_proposal
        .as_ref()
        .ok_or_else(|| "expected second summary to include a local onramp proposal".to_owned())?;
    let decision_receipt = second_summary
        .local_onramp_decision_receipt
        .as_ref()
        .ok_or_else(|| {
            "expected second summary to include a local onramp decision receipt".to_owned()
        })?;
    let summary_output = ferros_hub::summary_command_output()
        .map_err(|error| format!("could not build local hub runtime summary: {error}"))?;

    Ok(format!(
        "{}\nhubBridgeArtifact: {}\nhubRestartSnapshotArtifact: {}\nhubOnrampProposalStatus: {}\nhubOnrampProposalArtifact: {}\nhubOnrampProposalSource: {}\nhubOnrampDecisionLabel: {}\nhubOnrampDecisionArtifact: {}\nhubOnrampDecisionProposalId: {}",
        summary_output,
        SIMULATED_LOCAL_BRIDGE_ARTIFACT_PATH,
        LOCAL_HUB_STATE_SNAPSHOT_PATH,
        proposal.quarantine_status.as_str(),
        proposal.local_artifact_path,
        proposal.source,
        decision_receipt.decision_label.as_str(),
        decision_receipt.local_artifact_path,
        decision_receipt.proposal_id,
    ))
}

fn validate_hub_runway_summary(
    summary_label: &str,
    summary: &LocalHubRuntimeSummary,
    workspace_root: &Path,
) -> Result<(), String> {
    if summary.status != LocalBridgeStatus::Allowed {
        return Err(format!(
            "expected allowed local hub status in {summary_label} summary, got {}",
            summary.status.as_str()
        ));
    }

    if summary.artifact_relative_output_path.as_deref()
        != Some(SIMULATED_LOCAL_BRIDGE_ARTIFACT_PATH)
    {
        return Err(format!(
            "expected artifact path {} in {summary_label} summary, got {:?}",
            SIMULATED_LOCAL_BRIDGE_ARTIFACT_PATH,
            summary.artifact_relative_output_path
        ));
    }

    if summary.scope != "local-only" {
        return Err(format!(
            "expected local-only scope in {summary_label} summary, got {}",
            summary.scope
        ));
    }

    if summary.evidence != "non-evidentiary" {
        return Err(format!(
            "expected non-evidentiary evidence in {summary_label} summary, got {}",
            summary.evidence
        ));
    }

    let proposal = summary.local_onramp_proposal.as_ref().ok_or_else(|| {
        format!(
            "expected local onramp proposal in {summary_label} summary, got none"
        )
    })?;
    let decision_receipt = summary.local_onramp_decision_receipt.as_ref().ok_or_else(|| {
        format!(
            "expected local onramp decision receipt in {summary_label} summary, got none"
        )
    })?;

    if proposal.source != SIMULATED_LOCAL_BRIDGE_ARTIFACT_PATH {
        return Err(format!(
            "expected proposal source {} in {summary_label} summary, got {}",
            SIMULATED_LOCAL_BRIDGE_ARTIFACT_PATH,
            proposal.source
        ));
    }

    if proposal.quarantine_status.as_str() != "quarantined-pending-consent" {
        return Err(format!(
            "expected quarantined-pending-consent proposal status in {summary_label} summary, got {}",
            proposal.quarantine_status.as_str()
        ));
    }

    if proposal.scope != "local-only" {
        return Err(format!(
            "expected local-only proposal scope in {summary_label} summary, got {}",
            proposal.scope
        ));
    }

    if proposal.evidence != "non-evidentiary" {
        return Err(format!(
            "expected non-evidentiary proposal evidence in {summary_label} summary, got {}",
            proposal.evidence
        ));
    }

    if proposal.local_artifact_path != LOCAL_ONRAMP_PROPOSAL_ARTIFACT_PATH {
        return Err(format!(
            "expected proposal artifact path {} in {summary_label} summary, got {}",
            LOCAL_ONRAMP_PROPOSAL_ARTIFACT_PATH,
            proposal.local_artifact_path
        ));
    }

    if decision_receipt.proposal_id != proposal.proposal_id {
        return Err(format!(
            "expected decision receipt proposal id {} in {summary_label} summary, got {}",
            proposal.proposal_id,
            decision_receipt.proposal_id
        ));
    }

    if decision_receipt.proposal_artifact_path != LOCAL_ONRAMP_PROPOSAL_ARTIFACT_PATH {
        return Err(format!(
            "expected decision receipt proposal artifact path {} in {summary_label} summary, got {}",
            LOCAL_ONRAMP_PROPOSAL_ARTIFACT_PATH,
            decision_receipt.proposal_artifact_path
        ));
    }

    if decision_receipt.decision_label.as_str() != "allowed" {
        return Err(format!(
            "expected allowed decision label in {summary_label} summary, got {}",
            decision_receipt.decision_label.as_str()
        ));
    }

    let decision_detail = decision_receipt.decision_detail.as_deref().ok_or_else(|| {
        format!(
            "expected decision detail in {summary_label} summary, got none"
        )
    })?;

    if !decision_detail.contains(&proposal.proposal_id) {
        return Err(format!(
            "expected decision detail in {summary_label} summary to mention proposal id {}, got {}",
            proposal.proposal_id,
            decision_detail
        ));
    }

    if decision_receipt.scope != "local-only" {
        return Err(format!(
            "expected local-only decision scope in {summary_label} summary, got {}",
            decision_receipt.scope
        ));
    }

    if decision_receipt.evidence != "non-evidentiary" {
        return Err(format!(
            "expected non-evidentiary decision evidence in {summary_label} summary, got {}",
            decision_receipt.evidence
        ));
    }

    if decision_receipt.local_artifact_path != LOCAL_ONRAMP_DECISION_RECEIPT_ARTIFACT_PATH {
        return Err(format!(
            "expected decision artifact path {} in {summary_label} summary, got {}",
            LOCAL_ONRAMP_DECISION_RECEIPT_ARTIFACT_PATH,
            decision_receipt.local_artifact_path
        ));
    }

    let proposal_output_path = workspace_root.join(&proposal.local_artifact_path);
    let proposal_content = fs::read_to_string(&proposal_output_path)
        .map_err(|error| format!("could not read local onramp proposal artifact: {error}"))?;

    if !proposal_content.contains("\"quarantineStatus\": \"quarantined-pending-consent\"") {
        return Err(format!(
            "expected quarantined proposal status in {summary_label} artifact, got {}",
            proposal_content
        ));
    }

    if !proposal_content.contains(&format!(
        "\"localArtifactPath\": \"{}\"",
        LOCAL_ONRAMP_PROPOSAL_ARTIFACT_PATH
    )) {
        return Err(format!(
            "expected proposal artifact path {} in {summary_label} artifact payload",
            LOCAL_ONRAMP_PROPOSAL_ARTIFACT_PATH
        ));
    }

    for banned_wording in [
        "hardware",
        "proof",
        "launch",
        "accepted",
        "canonical",
        "granted",
        "://",
    ] {
        if proposal_content.contains(banned_wording) {
            return Err(format!(
                "proposal artifact in {summary_label} summary contained banned wording {}",
                banned_wording
            ));
        }
    }

    let decision_output_path = workspace_root.join(&decision_receipt.local_artifact_path);
    let decision_content = fs::read_to_string(&decision_output_path)
        .map_err(|error| format!("could not read local onramp decision artifact: {error}"))?;

    if !decision_content.contains("\"decisionLabel\": \"allowed\"") {
        return Err(format!(
            "expected allowed decision label in {summary_label} artifact, got {}",
            decision_content
        ));
    }

    if !decision_content.contains(&format!(
        "\"proposalId\": \"{}\"",
        proposal.proposal_id
    )) {
        return Err(format!(
            "expected proposal id {} in {summary_label} decision artifact payload",
            proposal.proposal_id
        ));
    }

    if !decision_content.contains(&format!(
        "\"proposalArtifactPath\": \"{}\"",
        LOCAL_ONRAMP_PROPOSAL_ARTIFACT_PATH
    )) {
        return Err(format!(
            "expected proposal artifact path {} in {summary_label} decision artifact payload",
            LOCAL_ONRAMP_PROPOSAL_ARTIFACT_PATH
        ));
    }

    if !decision_content.contains(&format!(
        "\"localArtifactPath\": \"{}\"",
        LOCAL_ONRAMP_DECISION_RECEIPT_ARTIFACT_PATH
    )) {
        return Err(format!(
            "expected decision artifact path {} in {summary_label} decision artifact payload",
            LOCAL_ONRAMP_DECISION_RECEIPT_ARTIFACT_PATH
        ));
    }

    if !decision_content.contains(decision_detail) {
        return Err(format!(
            "expected decision detail {} in {summary_label} artifact payload",
            decision_detail
        ));
    }

    for banned_wording in [
        "hardware",
        "proof",
        "launch",
        "accepted",
        "canonical",
        "granted",
        "://",
    ] {
        if decision_content.contains(banned_wording) {
            return Err(format!(
                "decision artifact in {summary_label} summary contained banned wording {}",
                banned_wording
            ));
        }
    }

    Ok(())
}

fn run_step(program: &str, args: &[&str]) -> Result<(), String> {
    let status = Command::new(program)
        .args(args)
        .status()
        .map_err(|error| format!("could not start `{program} {}`: {error}", args.join(" ")))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "`{program} {}` exited with status {status}",
            args.join(" ")
        ))
    }
}

const HELP_TEXT: &str = "\
FERROS xtask

Usage:
    cargo xtask ci      Run fmt, clippy, build, and test for the current workspace
    cargo xtask burst   Print the current queue-clear opener surfaces and focused validation commands
    cargo xtask hub-runway [--keep-artifacts]   Validate the restart-aware hub seam plus local onramp rehearsal proposal and decision artifacts, print the summary output, and restore `.tmp/hub` artifacts unless keep mode is requested
";

const BURST_TEXT: &str = "\
FERROS queue-clear opener burst support

Ready opener waves:
    - WAVE-2026-04-28-18 runtime local runway checkpoint helpers
    - WAVE-2026-04-28-20 typed local-push audit envelope boundary
    - WAVE-2026-04-28-22 queue-clear focused xtask support
    - WAVE-2026-04-28-23 shell deny and lifecycle outcome rendering

Focused validation:
    - cargo test -p ferros-runtime
    - cargo test -p ferros-data
    - cargo check -p xtask
    - cargo xtask burst
    - cargo test -p ferros-node shell_route_serves_local_shell_html

Queued serial follow-ons:
    - WAVE-2026-04-28-26 ferros-node runway summary consumes LocalRunwayState
    - WAVE-2026-04-28-27 shell runway route honors checkpoint progress
    - WAVE-2026-04-28-29 profile adapter returns structured local status payloads
    - WAVE-2026-04-28-32 typed local-push envelope emission now lands at .tmp/push/burst-local-push-envelope.json
";

#[cfg(test)]
mod tests {
    use super::{
        parse_command, CommandKind, HubArtifactCleanup, HubArtifactCleanupAction,
        HubArtifactCleanupMode, HUB_RUNWAY_ARTIFACTS,
    };
    use ferros_data::{
        LOCAL_ONRAMP_DECISION_RECEIPT_ARTIFACT_PATH, LOCAL_ONRAMP_PROPOSAL_ARTIFACT_PATH,
    };
    use ferros_hub::{LOCAL_HUB_STATE_SNAPSHOT_PATH, SIMULATED_LOCAL_BRIDGE_ARTIFACT_PATH};
    use std::ffi::OsString;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn parses_ci_command() {
        let args = vec![OsString::from("ci")];
        assert_eq!(parse_command(args), CommandKind::Ci);
    }

    #[test]
    fn parses_burst_command() {
        let args = vec![OsString::from("burst")];
        assert_eq!(parse_command(args), CommandKind::Burst);
    }

    #[test]
    fn parses_hub_runway_command() {
        let args = vec![OsString::from("hub-runway")];
        assert_eq!(
            parse_command(args),
            CommandKind::HubRunway {
                keep_artifacts: false,
            }
        );
    }

    #[test]
    fn parses_hub_runway_keep_artifacts_flag() {
        let args = vec![OsString::from("hub-runway"), OsString::from("--keep-artifacts")];
        assert_eq!(
            parse_command(args),
            CommandKind::HubRunway {
                keep_artifacts: true,
            }
        );
    }

    #[test]
    fn defaults_to_help_for_unknown_hub_runway_flag() {
        let args = vec![OsString::from("hub-runway"), OsString::from("--unknown")];
        assert_eq!(parse_command(args), CommandKind::Help);
    }

    #[test]
    fn defaults_to_help_without_arguments() {
        let args: Vec<OsString> = Vec::new();
        assert_eq!(parse_command(args), CommandKind::Help);
    }

    #[test]
    fn defaults_to_help_for_unknown_command() {
        let args = vec![OsString::from("unknown")];
        assert_eq!(parse_command(args), CommandKind::Help);
    }

    #[test]
    fn help_text_mentions_burst_command() {
        assert!(super::HELP_TEXT.contains("cargo xtask burst"));
    }

    #[test]
    fn help_text_mentions_hub_runway_command() {
        assert!(super::HELP_TEXT.contains("cargo xtask hub-runway"));
    }

    #[test]
    fn help_text_mentions_keep_artifacts_flag() {
        assert!(super::HELP_TEXT.contains("--keep-artifacts"));
    }

    #[test]
    fn burst_text_mentions_queue_clear_opener_and_shell_validation() {
        assert!(super::BURST_TEXT.contains("WAVE-2026-04-28-22"));
        assert!(super::BURST_TEXT.contains("shell_route_serves_local_shell_html"));
        assert!(super::BURST_TEXT.contains("burst-local-push-envelope.json"));
    }

    #[test]
    fn artifact_cleanup_restore_reinstates_previous_state() {
        let temp_dir = create_temp_dir("restore");
        let existing_bridge_path = temp_dir.join(SIMULATED_LOCAL_BRIDGE_ARTIFACT_PATH);
        write_text(&existing_bridge_path, "bridge-before");

        let cleanup = HubArtifactCleanup::capture(&temp_dir).expect("capture should succeed");

        for spec in HUB_RUNWAY_ARTIFACTS {
            write_text(&temp_dir.join(spec.path), spec.label);
        }

        let summary = cleanup.restore(&temp_dir).expect("restore should succeed");
        assert_eq!(summary.mode, HubArtifactCleanupMode::Restored);
        assert_eq!(
            fs::read_to_string(&existing_bridge_path).expect("bridge artifact should exist"),
            "bridge-before"
        );
        assert!(!temp_dir.join(LOCAL_HUB_STATE_SNAPSHOT_PATH).exists());
        assert!(!temp_dir.join(LOCAL_ONRAMP_PROPOSAL_ARTIFACT_PATH).exists());
        assert!(!temp_dir
            .join(LOCAL_ONRAMP_DECISION_RECEIPT_ARTIFACT_PATH)
            .exists());
        assert!(summary.actions.contains(&(
            HUB_RUNWAY_ARTIFACTS[0],
            HubArtifactCleanupAction::RestoredPrevious,
        )));
        assert!(summary.actions.contains(&(
            HUB_RUNWAY_ARTIFACTS[1],
            HubArtifactCleanupAction::RemovedGenerated,
        )));

        fs::remove_dir_all(temp_dir).expect("temp dir cleanup should succeed");
    }

    #[test]
    fn artifact_cleanup_keep_reports_keep_mode() {
        let temp_dir = create_temp_dir("keep");
        let cleanup = HubArtifactCleanup::capture(&temp_dir).expect("capture should succeed");
        let summary = cleanup.keep();

        assert_eq!(summary.mode, HubArtifactCleanupMode::Kept);
        assert_eq!(summary.actions.len(), HUB_RUNWAY_ARTIFACTS.len());
        assert!(summary.actions.iter().all(|(_, action)| {
            *action == HubArtifactCleanupAction::KeptForInspection
        }));

        fs::remove_dir_all(temp_dir).expect("temp dir cleanup should succeed");
    }

    #[test]
    fn unexpected_hub_artifacts_are_reported() {
        let temp_dir = create_temp_dir("unexpected");
        write_text(
            &temp_dir.join(".tmp/hub/unexpected-extra.json"),
            "unexpected artifact",
        );

        let error = super::validate_no_unexpected_hub_artifacts(&temp_dir)
            .expect_err("unexpected artifacts should fail validation");
        assert!(error.contains("unexpected-extra.json"));

        fs::remove_dir_all(temp_dir).expect("temp dir cleanup should succeed");
    }

    fn create_temp_dir(label: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be after unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "ferros-xtask-hub-runway-{label}-{}-{unique}",
            std::process::id()
        ));
        fs::create_dir_all(&path).expect("temp dir should be created");
        path
    }

    fn write_text(path: &PathBuf, content: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("parent dir should exist");
        }

        fs::write(path, content).expect("file write should succeed");
    }
}
