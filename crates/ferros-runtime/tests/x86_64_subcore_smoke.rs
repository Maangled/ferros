use ferros_core::{Capability, MessageEnvelope};
use ferros_runtime::{
    Executor, InMemoryExecutor, InMemoryMessageBus, LocalRunwayAdapter, LocalRunwayAdapterError,
    LocalRunwayIntent, LocalRunwayState, MessageBus,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SmokeBusFailure {
    TransientRouteBlocked,
}

struct SmokeRetryableBus {
    queued: Vec<MessageEnvelope>,
    transient_failures_remaining: usize,
}

impl SmokeRetryableBus {
    fn with_failures(transient_failures_remaining: usize) -> Self {
        Self {
            queued: Vec::new(),
            transient_failures_remaining,
        }
    }
}

impl MessageBus for SmokeRetryableBus {
    type Error = SmokeBusFailure;

    fn send(&mut self, envelope: MessageEnvelope) -> Result<(), Self::Error> {
        if self.transient_failures_remaining > 0 {
            self.transient_failures_remaining -= 1;
            return Err(SmokeBusFailure::TransientRouteBlocked);
        }

        self.queued.push(envelope);
        Ok(())
    }

    fn try_recv(&mut self, recipient: &str) -> Result<Option<MessageEnvelope>, Self::Error> {
        let position = self
            .queued
            .iter()
            .position(|envelope| envelope.recipient() == recipient);

        Ok(position.map(|index| self.queued.remove(index)))
    }
}

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
    let mut adapter = LocalRunwayAdapter::new(InMemoryExecutor::new(), InMemoryMessageBus::new());

    for _ in 0..4 {
        adapter
            .advance(LocalRunwayIntent::Start)
            .expect("start path should advance");
    }

    adapter.submit("boot").expect("submit should succeed");
    adapter
        .submit("dispatch-message")
        .expect("submit should succeed");
    adapter.route(message(7)).expect("send should succeed");

    let (state, mut executor, mut bus) = adapter.into_parts();

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

#[test]
fn x86_64_subcore_host_smoke_requires_explicit_retry_after_transient_route_failure() {
    let mut adapter = LocalRunwayAdapter::new(InMemoryExecutor::new(), SmokeRetryableBus::with_failures(1));

    adapter
        .advance(LocalRunwayIntent::Start)
        .expect("pending -> profile should advance");
    adapter
        .advance(LocalRunwayIntent::Start)
        .expect("profile -> consent should advance");
    adapter
        .advance(LocalRunwayIntent::Start)
        .expect("consent -> runtime should advance");

    let first_attempt = adapter
        .advance_submit_and_route(LocalRunwayIntent::Start, "deliver", message(12))
        .expect_err("first route should fail transiently");

    match first_attempt {
        LocalRunwayAdapterError::Bus(error) => {
            assert_eq!(error, SmokeBusFailure::TransientRouteBlocked);
        }
        other => panic!("expected transient bus error, got {other:?}"),
    }

    assert_eq!(adapter.state(), LocalRunwayState::Active);
    assert_eq!(
        adapter
            .try_recv("subcore.runtime")
            .expect("receive should succeed before explicit retry"),
        None
    );

    adapter
        .route(message(12))
        .expect("explicit caller retry should succeed");

    let (state, mut executor, mut bus) = adapter.into_parts();

    assert_eq!(state, LocalRunwayState::Active);
    assert_eq!(executor.pop_next().expect("pop should succeed"), Some("deliver"));
    assert_eq!(executor.pop_next().expect("pop should succeed"), None);

    let delivered = bus
        .try_recv("subcore.runtime")
        .expect("receive should succeed after explicit retry")
        .expect("message should be delivered after explicit retry");
    assert_eq!(delivered.nonce(), 12);
}

#[test]
fn x86_64_subcore_host_smoke_requires_repeated_explicit_retries_for_repeated_transient_failures() {
    let mut adapter = LocalRunwayAdapter::new(InMemoryExecutor::new(), SmokeRetryableBus::with_failures(2));

    adapter
        .advance(LocalRunwayIntent::Start)
        .expect("pending -> profile should advance");
    adapter
        .advance(LocalRunwayIntent::Start)
        .expect("profile -> consent should advance");
    adapter
        .advance(LocalRunwayIntent::Start)
        .expect("consent -> runtime should advance");

    let first_attempt = adapter
        .advance_submit_and_route(LocalRunwayIntent::Start, "deliver", message(13))
        .expect_err("first route should fail transiently");

    match first_attempt {
        LocalRunwayAdapterError::Bus(error) => {
            assert_eq!(error, SmokeBusFailure::TransientRouteBlocked);
        }
        other => panic!("expected transient bus error, got {other:?}"),
    }

    assert_eq!(adapter.state(), LocalRunwayState::Active);
    assert_eq!(
        adapter
            .try_recv("subcore.runtime")
            .expect("receive should be empty before retries"),
        None
    );

    let retry_error = adapter
        .route(message(13))
        .expect_err("first explicit retry should still fail transiently");
    assert_eq!(retry_error, SmokeBusFailure::TransientRouteBlocked);

    assert_eq!(
        adapter
            .try_recv("subcore.runtime")
            .expect("receive should remain empty while failures remain"),
        None
    );

    adapter
        .route(message(13))
        .expect("second explicit retry should succeed");

    let (state, mut executor, mut bus) = adapter.into_parts();

    assert_eq!(state, LocalRunwayState::Active);
    assert_eq!(executor.pop_next().expect("pop should succeed"), Some("deliver"));
    assert_eq!(executor.pop_next().expect("pop should succeed"), None);

    let delivered = bus
        .try_recv("subcore.runtime")
        .expect("receive should succeed after second retry")
        .expect("message should be delivered after repeated retries");
    assert_eq!(delivered.nonce(), 13);
}