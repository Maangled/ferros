use std::env;

fn main() {
    let mut args = env::args().skip(1);

    match args.next().as_deref() {
        Some("demo") if args.next().is_none() => match ferros_node::run_demo() {
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
        Some("shell") => {
            let port = match args.next() {
                Some(value) => match value.parse::<u16>() {
                    Ok(port) => port,
                    Err(_) => {
                        eprintln!("invalid port: {value}");
                        std::process::exit(2);
                    }
                },
                None => ferros_node::local_shell_default_port(),
            };

            if args.next().is_some() {
                eprintln!("usage: ferros-node demo\n       ferros-node shell [port]");
                std::process::exit(2);
            }

            println!(
                "serving FERROS local shell at {}",
                ferros_node::local_shell_url(port)
            );

            if let Err(error) = ferros_node::serve_local_shell(port) {
                eprintln!("shell server failed: {error}");
                std::process::exit(1);
            }
        }
        _ => {
            eprintln!("usage: ferros-node demo\n       ferros-node shell [port]");
            std::process::exit(2);
        }
    }
}
