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
        CommandKind::Help => {
            print_help();
            ExitCode::SUCCESS
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum CommandKind {
    Ci,
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

fn print_help() {
    println!("FERROS xtask");
    println!();
    println!("Usage:");
    println!("  cargo xtask ci      Run fmt, clippy, build, and test for the current workspace");
}

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
    fn defaults_to_help_without_arguments() {
        let args: Vec<OsString> = Vec::new();
        assert_eq!(parse_command(args), CommandKind::Help);
    }

    #[test]
    fn defaults_to_help_for_unknown_command() {
        let args = vec![OsString::from("unknown")];
        assert_eq!(parse_command(args), CommandKind::Help);
    }
}
