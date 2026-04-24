use std::fmt;

use ferros_core::MessageEnvelope;
use ferros_profile::ProfileId;

use crate::agent::{Agent, AgentStatus};
use crate::manifest::{AgentManifest, AgentName, CapabilityRequirement};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReferenceAgentError {
    NotRunning,
}

impl fmt::Display for ReferenceAgentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotRunning => write!(f, "agent must be running"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EchoAgent {
    id: AgentName,
    capabilities: Vec<CapabilityRequirement>,
    status: AgentStatus,
}

impl EchoAgent {
    pub fn new(profile_id: ProfileId) -> Self {
        Self {
            id: AgentName::new("echo").expect("echo name should be valid"),
            capabilities: vec![CapabilityRequirement::new(profile_id, "agent.echo")],
            status: AgentStatus::Registered,
        }
    }

    #[must_use]
    pub fn manifest(&self) -> AgentManifest {
        AgentManifest::new(self.id.clone(), "0.1.0", self.capabilities.clone())
    }
}

impl Agent for EchoAgent {
    type Error = ReferenceAgentError;

    fn id(&self) -> &AgentName {
        &self.id
    }

    fn capabilities(&self) -> &[CapabilityRequirement] {
        &self.capabilities
    }

    fn start(&mut self) -> Result<(), Self::Error> {
        self.status = AgentStatus::Running;
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Self::Error> {
        self.status = AgentStatus::Stopped;
        Ok(())
    }

    fn status(&self) -> AgentStatus {
        self.status
    }

    fn handle_message(
        &mut self,
        envelope: &MessageEnvelope,
    ) -> Result<Option<Vec<u8>>, Self::Error> {
        if self.status != AgentStatus::Running {
            return Err(ReferenceAgentError::NotRunning);
        }

        Ok(Some(envelope.payload().to_vec()))
    }
}

#[derive(Debug, Clone)]
pub struct TimerAgent {
    id: AgentName,
    capabilities: Vec<CapabilityRequirement>,
    status: AgentStatus,
    tick_count: u64,
}

impl TimerAgent {
    pub fn new(profile_id: ProfileId) -> Self {
        Self {
            id: AgentName::new("timer").expect("timer name should be valid"),
            capabilities: vec![CapabilityRequirement::new(profile_id, "agent.timer")],
            status: AgentStatus::Registered,
            tick_count: 0,
        }
    }

    #[must_use]
    pub fn manifest(&self) -> AgentManifest {
        AgentManifest::new(self.id.clone(), "0.1.0", self.capabilities.clone())
    }
}

impl Agent for TimerAgent {
    type Error = ReferenceAgentError;

    fn id(&self) -> &AgentName {
        &self.id
    }

    fn capabilities(&self) -> &[CapabilityRequirement] {
        &self.capabilities
    }

    fn start(&mut self) -> Result<(), Self::Error> {
        self.status = AgentStatus::Running;
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Self::Error> {
        self.status = AgentStatus::Stopped;
        Ok(())
    }

    fn status(&self) -> AgentStatus {
        self.status
    }

    fn poll(&mut self) -> Result<Vec<Vec<u8>>, Self::Error> {
        if self.status != AgentStatus::Running {
            return Err(ReferenceAgentError::NotRunning);
        }

        self.tick_count += 1;
        Ok(vec![format!("tick-{}", self.tick_count).into_bytes()])
    }
}

#[cfg(test)]
mod tests {
    use super::{EchoAgent, ReferenceAgentError, TimerAgent};
    use crate::agent::{Agent, AgentStatus};
    use ferros_core::{Capability, MessageEnvelope};
    use ferros_profile::ProfileId;

    #[test]
    fn echo_agent_reflects_payload_when_running() {
        let profile_id = ProfileId::new("profile-alpha").expect("valid profile id");
        let mut agent = EchoAgent::new(profile_id);
        agent.start().expect("start should succeed");

        let envelope = MessageEnvelope::new(
            "echo",
            "echo",
            Capability::new("agent.echo").expect("capability should parse"),
            b"hello".to_vec(),
            1,
        )
        .expect("message should parse");

        let response = agent
            .handle_message(&envelope)
            .expect("echo should handle its payload");

        assert_eq!(response, Some(b"hello".to_vec()));
    }

    #[test]
    fn timer_agent_emits_deterministic_ticks_when_running() {
        let profile_id = ProfileId::new("profile-alpha").expect("valid profile id");
        let mut agent = TimerAgent::new(profile_id);
        agent.start().expect("start should succeed");

        assert_eq!(agent.status(), AgentStatus::Running);
        assert_eq!(
            agent.poll().expect("first poll should work"),
            vec![b"tick-1".to_vec()]
        );
        assert_eq!(
            agent.poll().expect("second poll should work"),
            vec![b"tick-2".to_vec()]
        );
    }

    #[test]
    fn reference_agents_reject_work_while_stopped() {
        let profile_id = ProfileId::new("profile-alpha").expect("valid profile id");
        let mut timer = TimerAgent::new(profile_id);

        assert_eq!(timer.poll(), Err(ReferenceAgentError::NotRunning));
    }
}
