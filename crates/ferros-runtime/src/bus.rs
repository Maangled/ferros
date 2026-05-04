use ferros_core::MessageEnvelope;
use alloc::collections::VecDeque;
use core::convert::Infallible;

/// Routes FERROS-owned message envelopes between hosted recipients.
pub trait MessageBus {
    type Error;

    fn send(&mut self, envelope: MessageEnvelope) -> Result<(), Self::Error>;

    fn try_recv(&mut self, recipient: &str) -> Result<Option<MessageEnvelope>, Self::Error>;
}

/// Abstract queue backing for hosted and future non-std message routing.
pub trait EnvelopeQueue {
    fn push_back(&mut self, envelope: MessageEnvelope);

    fn remove_recipient(&mut self, recipient: &str) -> Option<MessageEnvelope>;
}

#[derive(Debug)]
pub struct DequeEnvelopeQueue {
    queue: VecDeque<MessageEnvelope>,
}

impl Default for DequeEnvelopeQueue {
    fn default() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }
}

impl EnvelopeQueue for DequeEnvelopeQueue {
    fn push_back(&mut self, envelope: MessageEnvelope) {
        self.queue.push_back(envelope);
    }

    fn remove_recipient(&mut self, recipient: &str) -> Option<MessageEnvelope> {
        let position = self
            .queue
            .iter()
            .position(|envelope| envelope.recipient() == recipient)?;

        self.queue.remove(position)
    }
}

#[derive(Debug)]
pub struct InMemoryMessageBus<Q> {
    queue: Q,
}

impl<Q> Default for InMemoryMessageBus<Q>
where
    Q: Default,
{
    fn default() -> Self {
        Self { queue: Q::default() }
    }
}

impl InMemoryMessageBus<DequeEnvelopeQueue> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            queue: DequeEnvelopeQueue::default(),
        }
    }
}

impl<Q> InMemoryMessageBus<Q> {
    #[must_use]
    pub fn from_queue(queue: Q) -> Self {
        Self { queue }
    }
}

impl<Q> MessageBus for InMemoryMessageBus<Q>
where
    Q: EnvelopeQueue,
{
    type Error = Infallible;

    fn send(&mut self, envelope: MessageEnvelope) -> Result<(), Self::Error> {
        self.queue.push_back(envelope);
        Ok(())
    }

    fn try_recv(&mut self, recipient: &str) -> Result<Option<MessageEnvelope>, Self::Error> {
        Ok(self.queue.remove_recipient(recipient))
    }
}
