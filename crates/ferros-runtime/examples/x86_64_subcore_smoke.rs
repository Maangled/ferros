use ferros_core::{Capability, MessageEnvelope};
use ferros_runtime::{
    Executor, InMemoryExecutor, InMemoryMessageBus, LocalRunwayIntent, LocalRunwayState,
    MessageBus,
};

fn main() {
    let mut state = LocalRunwayState::Pending;
    let mut executor = InMemoryExecutor::new();
    let mut bus = InMemoryMessageBus::new();

    for _ in 0..4 {
        state = state
            .advance(LocalRunwayIntent::Start)
            .expect("start path should advance to active");
    }

    executor.submit("boot").expect("submit should succeed");
    executor
        .submit("dispatch-message")
        .expect("submit should succeed");

    let envelope = MessageEnvelope::new(
        "subcore.boot",
        "subcore.runtime",
        Capability::new("runtime.dispatch").expect("capability should parse"),
        b"hello-from-x86-subcore".to_vec(),
        1,
    )
    .expect("message should parse");

    bus.send(envelope).expect("send should succeed");

    let first_job = executor
        .pop_next()
        .expect("pop should succeed")
        .expect("boot job should exist");
    let second_job = executor
        .pop_next()
        .expect("pop should succeed")
        .expect("dispatch job should exist");
    let delivered = bus
        .try_recv("subcore.runtime")
        .expect("receive should succeed")
        .expect("runtime recipient should have a message");

    println!("state={}", state.as_str());
    println!("first_job={first_job}");
    println!("second_job={second_job}");
    println!("recipient={}", delivered.recipient());
    println!("capability={}", delivered.capability().as_str());
    println!("payload={}", String::from_utf8_lossy(delivered.payload()));
}