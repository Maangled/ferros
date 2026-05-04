use ferros_hub::{
    cli_help_text, deny_demo_command_output, prepare_default_local_runway,
    prove_bridge_command_output, remote_fire_event_command_output,
    remote_report_state_command_output, remote_summary_command_output,
    summary_command_output,
};

fn main() {
    match std::env::args().nth(1).as_deref() {
        None => {
            let (registered_agents, artifact) = prepare_default_local_runway()
                .expect("default local runway should prepare successfully");

            println!(
                "ferros-hub runway: registered {} local bridge agent and staged simulated artifact at {}",
                registered_agents,
                artifact.relative_output_path
            );
        }
        Some("summary") => println!(
            "{}",
            summary_command_output().expect("summary command should succeed")
        ),
        Some("prove-bridge") => println!(
            "{}",
            prove_bridge_command_output().expect("prove-bridge command should succeed")
        ),
        Some("deny-demo") => println!(
            "{}",
            deny_demo_command_output().expect("deny-demo command should succeed")
        ),
        Some("remote-summary") => println!(
            "{}",
            remote_summary_command_output().expect("remote-summary command should succeed")
        ),
        Some("remote-fire-event") => println!(
            "{}",
            remote_fire_event_command_output().expect("remote-fire-event command should succeed")
        ),
        Some("remote-report-state") => println!(
            "{}",
            remote_report_state_command_output()
                .expect("remote-report-state command should succeed")
        ),
        Some("help") | Some("--help") | Some("-h") => {
            println!("{}", cli_help_text());
        }
        Some(command) => {
            eprintln!(
                "unknown ferros-hub command: {command}\n\n{}",
                cli_help_text()
            );
            std::process::exit(2);
        }
    }
}
