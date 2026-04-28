use std::env;
use std::ffi::OsString;
use std::process::{Command, ExitCode};

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
            print!("{BURST_TEXT}");
            ExitCode::SUCCESS
        }
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
    fn burst_text_mentions_queue_clear_opener_and_shell_validation() {
        assert!(super::BURST_TEXT.contains("WAVE-2026-04-28-22"));
        assert!(super::BURST_TEXT.contains("shell_route_serves_local_shell_html"));
    }
}
