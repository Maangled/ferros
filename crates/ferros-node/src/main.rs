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
            let mut port = ferros_node::local_shell_default_port();
            let mut bind_addr = "127.0.0.1";
            let mut remaining_args = Vec::new();

            while let Some(arg) = args.next() {
                match arg.as_str() {
                    "--port" => {
                        if let Some(port_str) = args.next() {
                            match port_str.parse::<u16>() {
                                Ok(p) => port = p,
                                Err(_) => {
                                    eprintln!("invalid port: {port_str}");
                                    std::process::exit(2);
                                }
                            }
                        } else {
                            eprintln!("--port requires a value");
                            std::process::exit(2);
                        }
                    }
                    "--bind" => {
                        if let Some(addr) = args.next() {
                            // BRIDGE-WORKAROUND: LAN bind — pre-auth, superseded when native bridge lands
                            bind_addr = Box::leak(addr.into_boxed_str());
                        } else {
                            eprintln!("--bind requires a value");
                            std::process::exit(2);
                        }
                    }
                    "--lan" => {
                        // BRIDGE-WORKAROUND: LAN bind — pre-auth, superseded when native bridge lands
                        bind_addr = "0.0.0.0";
                    }
                    val if val.parse::<u16>().is_ok() => {
                        port = val.parse::<u16>().unwrap();
                    }
                    _ => {
                        remaining_args.push(arg);
                    }
                }
            }

            if !remaining_args.is_empty() {
                eprintln!("usage: ferros-node shell [--port <port>] [--bind <addr>] [--lan]");
                std::process::exit(2);
            }

            println!(
                "serving FERROS local shell at http://{}:{}/",
                bind_addr, port
            );

            // BRIDGE-WORKAROUND: LAN bind — pre-auth, superseded when native bridge lands
            if let Err(error) = ferros_node::serve_local_shell_with_bind(port, bind_addr) {
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
