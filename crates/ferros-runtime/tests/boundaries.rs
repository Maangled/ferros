use std::convert::Infallible;

use ferros_core::{Capability, MessageEnvelope};
use ferros_runtime::{
    EnvelopeQueue, Executor, InMemoryExecutor, InMemoryMessageBus, JobQueue, LocalRunwayAdapter,
    LocalRunwayAdapterError, LocalRunwayIntent, LocalRunwayState, MessageBus,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExecutorFailure {
    SubmitBlocked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BusFailure {
    RouteBlocked,
}

struct StubExecutor {
    pending: Vec<&'static str>,
}

impl StubExecutor {
    fn new() -> Self {
        Self { pending: Vec::new() }
    }
}

impl Executor for StubExecutor {
    type Job = &'static str;
    type Error = Infallible;

    fn submit(&mut self, job: Self::Job) -> Result<(), Self::Error> {
        self.pending.push(job);
        Ok(())
    }

    fn pop_next(&mut self) -> Result<Option<Self::Job>, Self::Error> {
        if self.pending.is_empty() {
            Ok(None)
        } else {
            Ok(Some(self.pending.remove(0)))
        }
    }

    fn pending_jobs(&self) -> usize {
        self.pending.len()
    }
}

fn message(sender: &str, recipient: &str, nonce: u64) -> MessageEnvelope {
    MessageEnvelope::new(
        sender,
        recipient,
        Capability::new("runtime.dispatch").expect("capability should parse"),
        format!("payload-{nonce}").into_bytes(),
        nonce,
    )
    .expect("message envelope should parse")
}

#[test]
fn executor_runs_jobs_in_submission_order() {
    let mut executor = StubExecutor::new();

    executor.submit("boot").expect("submit should succeed");
    executor.submit("deliver").expect("submit should succeed");

    assert_eq!(executor.pending_jobs(), 2);
    assert_eq!(
        executor.pop_next().expect("pop should succeed"),
        Some("boot")
    );
    assert_eq!(
        executor.pop_next().expect("pop should succeed"),
        Some("deliver")
    );
    assert_eq!(
        executor.pop_next().expect("idle executor should be empty"),
        None
    );
    assert_eq!(executor.pending_jobs(), 0);
}

#[test]
fn in_memory_executor_preserves_submission_order() {
    let mut executor = InMemoryExecutor::new();

    executor.submit("boot").expect("submit should succeed");
    executor.submit("deliver").expect("submit should succeed");

    assert_eq!(
        executor.pop_next().expect("pop should succeed"),
        Some("boot")
    );
    assert_eq!(
        executor.pop_next().expect("pop should succeed"),
        Some("deliver")
    );
    assert_eq!(
        executor.pop_next().expect("idle executor should be empty"),
        None
    );
}

#[test]
fn message_bus_routes_messages_by_recipient() {
    let mut bus = InMemoryMessageBus::new();

    bus.send(message("agent.alpha", "agent.bravo", 11))
        .expect("send should succeed");
    bus.send(message("agent.alpha", "agent.charlie", 12))
        .expect("send should succeed");

    let received = bus
        .try_recv("agent.charlie")
        .expect("receive should succeed")
        .expect("recipient should have a queued message");

    assert_eq!(received.sender(), "agent.alpha");
    assert_eq!(received.recipient(), "agent.charlie");
    assert_eq!(received.capability().as_str(), "runtime.dispatch");
    assert_eq!(received.payload(), b"payload-12");
    assert_eq!(received.nonce(), 12);

    let remaining = bus
        .try_recv("agent.bravo")
        .expect("receive should succeed")
        .expect("other recipient should still have its message");
    assert_eq!(remaining.nonce(), 11);
}

#[test]
fn message_bus_reports_empty_queue_for_unknown_recipient() {
    let mut bus = InMemoryMessageBus::new();

    bus.send(message("agent.alpha", "agent.bravo", 11))
        .expect("send should succeed");

    let missing = bus
        .try_recv("agent.charlie")
        .expect("receive should succeed");

    assert!(missing.is_none());
}

struct VecJobQueue {
    pending: Vec<&'static str>,
}

impl VecJobQueue {
    fn new() -> Self {
        Self { pending: Vec::new() }
    }
}

impl JobQueue for VecJobQueue {
    type Job = &'static str;

    fn push_back(&mut self, job: Self::Job) {
        self.pending.push(job);
    }

    fn pop_front(&mut self) -> Option<Self::Job> {
        if self.pending.is_empty() {
            None
        } else {
            Some(self.pending.remove(0))
        }
    }

    fn len(&self) -> usize {
        self.pending.len()
    }
}

struct VecEnvelopeQueue {
    queue: Vec<MessageEnvelope>,
}

impl VecEnvelopeQueue {
    fn new() -> Self {
        Self { queue: Vec::new() }
    }
}

impl EnvelopeQueue for VecEnvelopeQueue {
    fn push_back(&mut self, envelope: MessageEnvelope) {
        self.queue.push(envelope);
    }

    fn remove_recipient(&mut self, recipient: &str) -> Option<MessageEnvelope> {
        let position = self
            .queue
            .iter()
            .position(|envelope| envelope.recipient() == recipient)?;

        Some(self.queue.remove(position))
    }
}

struct FailingExecutor {
    pending: Vec<&'static str>,
}

impl FailingExecutor {
    fn new() -> Self {
        Self { pending: Vec::new() }
    }
}

impl Executor for FailingExecutor {
    type Job = &'static str;
    type Error = ExecutorFailure;

    fn submit(&mut self, _job: Self::Job) -> Result<(), Self::Error> {
        Err(ExecutorFailure::SubmitBlocked)
    }

    fn pop_next(&mut self) -> Result<Option<Self::Job>, Self::Error> {
        if self.pending.is_empty() {
            Ok(None)
        } else {
            Ok(Some(self.pending.remove(0)))
        }
    }

    fn pending_jobs(&self) -> usize {
        self.pending.len()
    }
}

struct FailingBus {
    queued: Vec<MessageEnvelope>,
}

impl FailingBus {
    fn new() -> Self {
        Self { queued: Vec::new() }
    }
}

impl MessageBus for FailingBus {
    type Error = BusFailure;

    fn send(&mut self, _envelope: MessageEnvelope) -> Result<(), Self::Error> {
        Err(BusFailure::RouteBlocked)
    }

    fn try_recv(&mut self, recipient: &str) -> Result<Option<MessageEnvelope>, Self::Error> {
        let position = self
            .queued
            .iter()
            .position(|envelope| envelope.recipient() == recipient);

        Ok(position.map(|index| self.queued.remove(index)))
    }
}

#[test]
fn in_memory_executor_accepts_custom_job_queue_backing() {
    let mut executor = InMemoryExecutor::from_queue(VecJobQueue::new());

    executor.submit("boot").expect("submit should succeed");
    executor.submit("deliver").expect("submit should succeed");

    assert_eq!(executor.pending_jobs(), 2);
    assert_eq!(executor.pop_next().expect("pop should succeed"), Some("boot"));
    assert_eq!(executor.pop_next().expect("pop should succeed"), Some("deliver"));
    assert_eq!(executor.pop_next().expect("pop should succeed"), None);
}

#[test]
fn in_memory_message_bus_accepts_custom_queue_backing() {
    let mut bus = InMemoryMessageBus::from_queue(VecEnvelopeQueue::new());

    bus.send(message("agent.alpha", "agent.bravo", 11))
        .expect("send should succeed");
    bus.send(message("agent.alpha", "agent.charlie", 12))
        .expect("send should succeed");

    let received = bus
        .try_recv("agent.charlie")
        .expect("receive should succeed")
        .expect("recipient should have a queued message");

    assert_eq!(received.nonce(), 12);

    let remaining = bus
        .try_recv("agent.bravo")
        .expect("receive should succeed")
        .expect("other recipient should still have its message");

    assert_eq!(remaining.nonce(), 11);
}

#[test]
fn local_runway_adapter_composes_transition_submit_and_route_with_custom_queues() {
    let mut adapter = LocalRunwayAdapter::new(
        InMemoryExecutor::from_queue(VecJobQueue::new()),
        InMemoryMessageBus::from_queue(VecEnvelopeQueue::new()),
    );

    adapter
        .advance(LocalRunwayIntent::Start)
        .expect("pending -> profile should succeed");
    adapter
        .advance(LocalRunwayIntent::Start)
        .expect("profile -> consent should succeed");
    adapter
        .advance(LocalRunwayIntent::Start)
        .expect("consent -> runtime should succeed");

    let next = adapter
        .advance_submit_and_route(
            LocalRunwayIntent::Start,
            "deliver",
            message("agent.alpha", "agent.bravo", 21),
        )
        .expect("runtime -> active adapter step should succeed");

    let (state, mut executor, mut bus) = adapter.into_parts();

    assert_eq!(next, LocalRunwayState::Active);
    assert_eq!(state, LocalRunwayState::Active);
    assert_eq!(executor.pop_next().expect("pop should succeed"), Some("deliver"));
    assert_eq!(executor.pop_next().expect("pop should succeed"), None);

    let delivered = bus
        .try_recv("agent.bravo")
        .expect("receive should succeed")
        .expect("recipient should have a message");
    assert_eq!(delivered.nonce(), 21);
}

#[test]
fn local_runway_adapter_maps_transition_errors_without_side_effects() {
    let mut adapter = LocalRunwayAdapter::new(InMemoryExecutor::new(), InMemoryMessageBus::new());

    let error = adapter
        .advance_submit_and_route(
            LocalRunwayIntent::Stop,
            "deliver",
            message("agent.alpha", "agent.bravo", 22),
        )
        .expect_err("pending -> stop should fail before submit or route");

    match error {
        LocalRunwayAdapterError::Transition(transition) => {
            assert_eq!(transition.from, LocalRunwayState::Pending);
            assert_eq!(transition.intent, LocalRunwayIntent::Stop);
        }
        other => panic!("expected transition error, got {other:?}"),
    }

    let (state, mut executor, mut bus) = adapter.into_parts();

    assert_eq!(state, LocalRunwayState::Pending);
    assert_eq!(executor.pending_jobs(), 0);
    assert_eq!(executor.pop_next().expect("pop should succeed"), None);
    assert_eq!(bus.try_recv("agent.bravo").expect("receive should succeed"), None);
}

#[test]
fn local_runway_adapter_maps_executor_errors_after_transition() {
    let mut adapter = LocalRunwayAdapter::with_state(
        LocalRunwayState::RuntimeReady,
        FailingExecutor::new(),
        InMemoryMessageBus::new(),
    );

    let error = adapter
        .advance_submit_and_route(
            LocalRunwayIntent::Start,
            "deliver",
            message("agent.alpha", "agent.bravo", 23),
        )
        .expect_err("executor failure should be surfaced");

    match error {
        LocalRunwayAdapterError::Executor(executor_error) => {
            assert_eq!(executor_error, ExecutorFailure::SubmitBlocked);
        }
        other => panic!("expected executor error, got {other:?}"),
    }

    let (state, mut executor, mut bus) = adapter.into_parts();

    assert_eq!(state, LocalRunwayState::Active);
    assert_eq!(executor.pending_jobs(), 0);
    assert_eq!(executor.pop_next().expect("pop should succeed"), None);
    assert_eq!(bus.try_recv("agent.bravo").expect("receive should succeed"), None);
}

#[test]
fn local_runway_adapter_maps_bus_errors_after_executor_submission() {
    let mut adapter = LocalRunwayAdapter::with_state(
        LocalRunwayState::RuntimeReady,
        InMemoryExecutor::new(),
        FailingBus::new(),
    );

    let error = adapter
        .advance_submit_and_route(
            LocalRunwayIntent::Start,
            "deliver",
            message("agent.alpha", "agent.bravo", 24),
        )
        .expect_err("bus failure should be surfaced");

    match error {
        LocalRunwayAdapterError::Bus(bus_error) => {
            assert_eq!(bus_error, BusFailure::RouteBlocked);
        }
        other => panic!("expected bus error, got {other:?}"),
    }

    let (state, mut executor, mut bus) = adapter.into_parts();

    assert_eq!(state, LocalRunwayState::Active);
    assert_eq!(executor.pending_jobs(), 1);
    assert_eq!(executor.pop_next().expect("pop should succeed"), Some("deliver"));
    assert_eq!(executor.pop_next().expect("pop should succeed"), None);
    assert_eq!(bus.try_recv("agent.bravo").expect("receive should succeed"), None);
}
