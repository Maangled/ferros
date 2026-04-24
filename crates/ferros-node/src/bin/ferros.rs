use std::{env, path::PathBuf};

use ferros_node::{
    default_profile_path, execute_agent_cli, execute_profile_cli, run_demo, AgentCliCommand,
    CliError, ProfileCliCommand,
};

fn main() {
    match run(env::args().skip(1).collect()) {
        Ok(lines) => {
            for line in lines {
                println!("{line}");
            }
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(exit_code(&error));
        }
    }
}

fn run(args: Vec<String>) -> Result<Vec<String>, CliError> {
    let Some(scope) = args.first().map(String::as_str) else {
        return Err(usage());
    };

    match scope {
        "demo" if args.len() == 1 => run_demo().map(|summary| {
            vec![
                format!("started: {}", summary.started_agents.join(",")),
                format!("echo: {}", summary.echo_response),
                format!("timer: {}", summary.timer_event),
                format!("denied: {}", summary.denied_requests),
            ]
        }).map_err(CliError::from),
        "agent" => execute_agent_cli(parse_agent_command(&args)?),
        "profile" => execute_profile_cli(parse_profile_command(&args)?),
        _ => Err(usage()),
    }
}

fn parse_agent_command(args: &[String]) -> Result<AgentCliCommand, CliError> {
    let Some(verb) = args.get(1).map(String::as_str) else {
        return Err(usage());
    };

    Ok(match verb {
        "list" if args.len() == 2 => AgentCliCommand::List,
        "describe" if args.len() == 3 => AgentCliCommand::Describe {
            name: args[2].clone(),
        },
        "run" if args.len() == 3 => AgentCliCommand::Run {
            name: args[2].clone(),
        },
        "stop" if args.len() == 3 => AgentCliCommand::Stop {
            name: args[2].clone(),
        },
        "logs" if args.len() == 2 => AgentCliCommand::Logs { name: None },
        "logs" if args.len() == 3 => AgentCliCommand::Logs {
            name: Some(args[2].clone()),
        },
        _ => return Err(usage()),
    })
}

fn parse_profile_command(args: &[String]) -> Result<ProfileCliCommand, CliError> {
    let Some(verb) = args.get(1).map(String::as_str) else {
        return Err(usage());
    };

    let path = match args.get(2) {
        Some(path) if args.len() == 3 => PathBuf::from(path),
        None => default_profile_path(),
        _ => return Err(usage()),
    };

    match verb {
        "init" => Ok(ProfileCliCommand::Init { path }),
        "show" => Ok(ProfileCliCommand::Show { path }),
        _ => Err(usage()),
    }
}

fn usage() -> CliError {
    CliError::Usage(
        "usage: ferros demo\n       ferros agent list | describe <name> | run <name> | stop <name> | logs [name]\n       ferros profile init [path]\n       ferros profile show [path]",
    )
}

fn exit_code(error: &CliError) -> i32 {
    match error {
        CliError::Usage(_) => 2,
        _ => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::run;
    use std::{fs, path::PathBuf, time::{SystemTime, UNIX_EPOCH}};

    #[test]
    fn run_dispatches_profile_init_and_show_with_explicit_path() {
        let profile_path = unique_profile_path("bin-profile");
        let profile_path_string = profile_path.to_string_lossy().into_owned();

        let init_lines = run(vec![
            "profile".to_string(),
            "init".to_string(),
            profile_path_string.clone(),
        ])
        .expect("profile init should succeed");
        assert!(init_lines
            .iter()
            .any(|line| line == &format!("initialized profile at {}", profile_path.display())));

        let show_lines = run(vec![
            "profile".to_string(),
            "show".to_string(),
            profile_path_string,
        ])
        .expect("profile show should succeed");
        assert!(show_lines
            .iter()
            .any(|line| line.contains("\"name\": \"Fresh Start\"")));

        cleanup_profile_path(&profile_path);
    }

    #[test]
    fn run_dispatches_demo() {
        let lines = run(vec!["demo".to_string()]).expect("demo should succeed");

        assert!(lines.iter().any(|line| line.starts_with("started: ")));
        assert!(lines.iter().any(|line| line.starts_with("echo: ")));
        assert!(lines.iter().any(|line| line.starts_with("timer: ")));
        assert!(lines.iter().any(|line| line == "denied: 1"));
    }

    fn unique_profile_path(test_name: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be after epoch")
            .as_nanos();

        std::env::temp_dir()
            .join("ferros-bin-profile-tests")
            .join(format!("{test_name}-{nonce}.json"))
    }

    fn cleanup_profile_path(path: &PathBuf) {
        let _ = fs::remove_file(path);
        if let Some(parent) = path.parent() {
            let _ = fs::remove_dir(parent);
        }
    }
}
