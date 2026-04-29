mod ha_bridge;

use ha_bridge::{simulated_local_bridge_artifact, LocalBridgeAgent, LocalBridgeRegistry};

fn main() {
    let mut registry = LocalBridgeRegistry::default();
    let bridge_agent = LocalBridgeAgent::new_default();
    registry
        .register(bridge_agent.clone())
        .expect("default local bridge agent should register");

    let artifact = simulated_local_bridge_artifact(&bridge_agent);

    println!(
        "ferros-hub runway: registered {} local bridge agent and staged simulated artifact at {}",
        registry.list().len(),
        artifact.relative_output_path
    );
}