use std::collections::VecDeque;
use std::convert::Infallible;

use ferros_core::{Capability, MessageEnvelope};
use ferros_runtime::{Executor, InMemoryExecutor, InMemoryMessageBus, MessageBus};

struct StubExecutor {
    pending: VecDeque<&'static str>,
}

impl StubExecutor {
    fn new() -> Self {
        Self {
            pending: VecDeque::new(),
        }
    }
}

impl Executor for StubExecutor {
    type Job = &'static str;
    type Error = Infallible;

    fn submit(&mut self, job: Self::Job) -> Result<(), Self::Error> {
        self.pending.push_back(job);
        Ok(())
    }

    fn pop_next(&mut self) -> Result<Option<Self::Job>, Self::Error> {
        Ok(self.pending.pop_front())
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
