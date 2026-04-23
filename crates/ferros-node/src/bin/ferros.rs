use std::env;

use ferros_node::{execute_agent_cli, AgentCliCommand, CliError};

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

    if scope != "agent" {
        return Err(usage());
    }

    let Some(verb) = args.get(1).map(String::as_str) else {
        return Err(usage());
    };

    let command = match verb {
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
    };

    execute_agent_cli(command)
}

fn usage() -> CliError {
    CliError::Usage(
        "usage: ferros agent list | describe <name> | run <name> | stop <name> | logs [name]",
    )
}

fn exit_code(error: &CliError) -> i32 {
    match error {
        CliError::Usage(_) => 2,
        _ => 1,
    }
}
