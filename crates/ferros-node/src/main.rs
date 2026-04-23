use std::env;

fn main() {
    let mut args = env::args().skip(1);

    match args.next().as_deref() {
        Some("demo") => match ferros_node::run_demo() {
            Ok(summary) => {
                println!("started: {}", summary.started_agents.join(","));
                println!("echo: {}", summary.echo_response);
                println!("timer: {}", summary.timer_event);
                println!("denied: {}", summary.denied_requests);
            }
            Err(error) => {
                eprintln!("demo failed: {error}");
                std::process::exit(1);
            }
        },
        _ => {
            eprintln!("usage: ferros-node demo");
            std::process::exit(2);
        }
    }
}
