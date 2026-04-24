use ferros_core::MessageEnvelope;
use std::collections::VecDeque;
use std::convert::Infallible;

/// Routes FERROS-owned message envelopes between hosted recipients.
pub trait MessageBus {
    type Error;

    fn send(&mut self, envelope: MessageEnvelope) -> Result<(), Self::Error>;

    fn try_recv(&mut self, recipient: &str) -> Result<Option<MessageEnvelope>, Self::Error>;
}

#[derive(Debug, Default)]
pub struct InMemoryMessageBus {
    queue: VecDeque<MessageEnvelope>,
}

impl InMemoryMessageBus {
    #[must_use]
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }
}

impl MessageBus for InMemoryMessageBus {
    type Error = Infallible;

    fn send(&mut self, envelope: MessageEnvelope) -> Result<(), Self::Error> {
        self.queue.push_back(envelope);
        Ok(())
    }

    fn try_recv(&mut self, recipient: &str) -> Result<Option<MessageEnvelope>, Self::Error> {
        let Some(position) = self
            .queue
            .iter()
            .position(|envelope| envelope.recipient() == recipient)
        else {
            return Ok(None);
        };

        Ok(self.queue.remove(position))
    }
}
