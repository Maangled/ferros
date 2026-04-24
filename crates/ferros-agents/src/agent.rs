use ferros_core::MessageEnvelope;

use crate::manifest::{AgentName, CapabilityRequirement};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentStatus {
    Registered,
    Starting,
    Running,
    Paused,
    Stopping,
    Stopped,
    Failed,
}

/// Pre-G3 execution stays host-agnostic so S3 can freeze naming and lifecycle
/// vocabulary before S4 publishes the executor surface.
pub trait Agent {
    type Error;

    fn id(&self) -> &AgentName;
    fn capabilities(&self) -> &[CapabilityRequirement];
    fn start(&mut self) -> Result<(), Self::Error>;
    fn stop(&mut self) -> Result<(), Self::Error>;
    fn status(&self) -> AgentStatus;

    fn handle_message(
        &mut self,
        _envelope: &MessageEnvelope,
    ) -> Result<Option<Vec<u8>>, Self::Error> {
        Ok(None)
    }

    fn poll(&mut self) -> Result<Vec<Vec<u8>>, Self::Error> {
        Ok(Vec::new())
    }
}
