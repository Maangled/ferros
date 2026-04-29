use std::env;
use std::ffi::OsString;
use std::fs;
use std::process::{Command, ExitCode};

use ferros_data::{
    LocalArtifactRole, LocalEnvelopeKind, LocalPushArtifact, LocalPushAuditEnvelope,
    LocalPushObservation, LocalPushScope, LocalPushSurface,
    LOCAL_ONRAMP_PROPOSAL_ARTIFACT_PATH, BURST_LOCAL_PUSH_ENVELOPE_PATH,
};
use ferros_hub::{
    LocalBridgeStatus, LocalHubReloadStatus, LocalHubRuntimeSummary,
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
        CommandKind::HubRunway => match run_hub_runway() {
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
    HubRunway,
    Help,
}

fn parse_command<I>(args: I) -> CommandKind
where
    I: IntoIterator<Item = OsString>,
{
    match args
        .into_iter()
        .next()
        .and_then(|arg| arg.into_string().ok())
    {
        Some(command) if command == "ci" => CommandKind::Ci,
        Some(command) if command == "burst" => CommandKind::Burst,
        Some(command) if command == "hub-runway" => CommandKind::HubRunway,
        _ => CommandKind::Help,
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

fn run_hub_runway() -> Result<String, String> {
    let first_summary = ferros_hub::default_local_runtime_summary()
        .map_err(|error| format!("could not build local hub runtime summary: {error}"))?;
    validate_hub_runway_summary("first", &first_summary)?;

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
    validate_hub_runway_summary("second", &second_summary)?;

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
    let summary_output = ferros_hub::summary_command_output()
        .map_err(|error| format!("could not build local hub runtime summary: {error}"))?;

    Ok(format!(
        "{}\nhubOnrampProposalStatus: {}\nhubOnrampProposalArtifact: {}\nhubOnrampProposalSource: {}",
        summary_output,
        proposal.quarantine_status.as_str(),
        proposal.local_artifact_path,
        proposal.source,
    ))
}

fn validate_hub_runway_summary(
    summary_label: &str,
    summary: &LocalHubRuntimeSummary,
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

    let proposal_output_path = env::current_dir()
        .map_err(|error| format!("could not resolve workspace root: {error}"))?
        .join(&proposal.local_artifact_path);
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
    cargo xtask hub-runway   Validate the restart-aware hub seam plus local onramp rehearsal artifact and print the summary output
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
    use super::{parse_command, CommandKind};
    use std::ffi::OsString;

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
        assert_eq!(parse_command(args), CommandKind::HubRunway);
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
    fn burst_text_mentions_queue_clear_opener_and_shell_validation() {
        assert!(super::BURST_TEXT.contains("WAVE-2026-04-28-22"));
        assert!(super::BURST_TEXT.contains("shell_route_serves_local_shell_html"));
        assert!(super::BURST_TEXT.contains("burst-local-push-envelope.json"));
    }
}
