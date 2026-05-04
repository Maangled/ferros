use ferros_core::{Capability, MessageEnvelope};
use ferros_runtime::{
    Executor, InMemoryExecutor, InMemoryMessageBus, LocalRunwayIntent, LocalRunwayState,
    MessageBus,
};

fn message(nonce: u64) -> MessageEnvelope {
    MessageEnvelope::new(
        "subcore.boot",
        "subcore.runtime",
        Capability::new("runtime.dispatch").expect("capability should parse"),
        format!("payload-{nonce}").into_bytes(),
        nonce,
    )
    .expect("message should parse")
}

#[test]
fn x86_64_subcore_host_smoke_advances_to_active_and_routes_message() {
    let mut state = LocalRunwayState::Pending;
    let mut executor = InMemoryExecutor::new();
    let mut bus = InMemoryMessageBus::new();

    for _ in 0..4 {
        state = state
            .advance(LocalRunwayIntent::Start)
            .expect("start path should advance");
    }

    executor.submit("boot").expect("submit should succeed");
    executor
        .submit("dispatch-message")
        .expect("submit should succeed");
    bus.send(message(7)).expect("send should succeed");

    assert_eq!(state, LocalRunwayState::Active);
    assert_eq!(
        executor.pop_next().expect("pop should succeed"),
        Some("boot")
    );
    assert_eq!(
        executor.pop_next().expect("pop should succeed"),
        Some("dispatch-message")
    );

    let delivered = bus
        .try_recv("subcore.runtime")
        .expect("receive should succeed")
        .expect("message should exist");
    assert_eq!(delivered.nonce(), 7);
    assert_eq!(delivered.payload(), b"payload-7");
}

#[test]
fn x86_64_subcore_resume_from_halt_restarts_at_profile_ready() {
    let halted = LocalRunwayState::Draining
        .advance(LocalRunwayIntent::Stop)
        .expect("draining stop should halt");
    let resumed = halted
        .advance(LocalRunwayIntent::Resume)
        .expect("halted resume should restart");

    assert_eq!(halted, LocalRunwayState::Halted);
    assert_eq!(resumed, LocalRunwayState::ProfileReady);
    assert!(resumed.can_observe_local_shell());
}

#[test]
fn x86_64_subcore_bus_preserves_unmatched_message_until_targeted_receive() {
    let mut bus = InMemoryMessageBus::new();

    bus.send(message(11)).expect("send should succeed");

    let missing = bus
        .try_recv("subcore.other")
        .expect("receive should succeed");
    let delivered = bus
        .try_recv("subcore.runtime")
        .expect("receive should succeed")
        .expect("message should still be queued");

    assert!(missing.is_none());
    assert_eq!(delivered.recipient(), "subcore.runtime");
    assert_eq!(delivered.capability().as_str(), "runtime.dispatch");
}