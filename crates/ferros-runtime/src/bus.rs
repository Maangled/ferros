use ferros_core::MessageEnvelope;

/// Routes FERROS-owned message envelopes between hosted recipients.
pub trait MessageBus {
    type Error;

    fn send(&mut self, envelope: MessageEnvelope) -> Result<(), Self::Error>;

    fn try_recv(&mut self, recipient: &str) -> Result<Option<MessageEnvelope>, Self::Error>;
}