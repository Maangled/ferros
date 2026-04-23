#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::convert::Infallible;
use std::fmt;

use ferros_agents::{
    Agent, AgentManifest, AgentRegistry, EchoAgent, InMemoryAgentRegistry,
    ReferenceAgentError, RegistryError, TimerAgent,
};
use ferros_core::{
    Capability, CapabilityError, CapabilityRequest, DenyByDefaultPolicy, MessageEnvelope,
    MessageEnvelopeError, PolicyDecision, PolicyEngine, RequesterProfileIdError,
};
use ferros_profile::{CapabilityGrant, ProfileId, ProfileIdError};
use ferros_runtime::{Executor, InMemoryExecutor, InMemoryMessageBus, MessageBus};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DemoSummary {
    pub started_agents: Vec<String>,
    pub echo_response: String,
    pub timer_event: String,
    pub denied_requests: usize,
    pub log_entries: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DemoError {
    UnknownAgent(String),
    ManifestMissingCapabilities(String),
    AuthorizationDenied(String),
    MissingEchoResponse,
    MissingTimerEvent,
    Profile(ProfileIdError),
    Capability(CapabilityError),
    RequesterProfile(RequesterProfileIdError),
    Envelope(MessageEnvelopeError),
    Registry(RegistryError),
    Agent(ReferenceAgentError),
}

impl fmt::Display for DemoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownAgent(name) => write!(f, "unknown agent: {name}"),
            Self::ManifestMissingCapabilities(name) => {
                write!(f, "agent {name} has no declared capabilities")
            }
            Self::AuthorizationDenied(message) => write!(f, "authorization denied: {message}"),
            Self::MissingEchoResponse => write!(f, "echo agent did not return a response"),
            Self::MissingTimerEvent => write!(f, "timer agent did not emit an event"),
            Self::Profile(error) => write!(f, "{error}"),
            Self::Capability(error) => write!(f, "{error}"),
            Self::RequesterProfile(error) => write!(f, "{error}"),
            Self::Envelope(error) => write!(f, "{error}"),
            Self::Registry(error) => write!(f, "{error}"),
            Self::Agent(error) => write!(f, "{error}"),
        }
    }
}

impl From<ProfileIdError> for DemoError {
    fn from(value: ProfileIdError) -> Self {
        Self::Profile(value)
    }
}

impl From<CapabilityError> for DemoError {
    fn from(value: CapabilityError) -> Self {
        Self::Capability(value)
    }
}

impl From<RequesterProfileIdError> for DemoError {
    fn from(value: RequesterProfileIdError) -> Self {
        Self::RequesterProfile(value)
    }
}

impl From<MessageEnvelopeError> for DemoError {
    fn from(value: MessageEnvelopeError) -> Self {
        Self::Envelope(value)
    }
}

impl From<RegistryError> for DemoError {
    fn from(value: RegistryError) -> Self {
        Self::Registry(value)
    }
}

impl From<ReferenceAgentError> for DemoError {
    fn from(value: ReferenceAgentError) -> Self {
        Self::Agent(value)
    }
}

struct HostedAgent {
    manifest: AgentManifest,
    agent: Box<dyn Agent<Error = ReferenceAgentError>>,
}

pub struct DemoRuntime {
    registry: InMemoryAgentRegistry,
    agents: BTreeMap<String, HostedAgent>,
    grants: Vec<CapabilityGrant>,
    policy: DenyByDefaultPolicy,
    executor: InMemoryExecutor<MessageEnvelope>,
    bus: InMemoryMessageBus,
    log_entries: Vec<String>,
    next_nonce: u64,
}

impl DemoRuntime {
    #[must_use]
    pub fn new(grants: Vec<CapabilityGrant>) -> Self {
        Self {
            registry: InMemoryAgentRegistry::default(),
            agents: BTreeMap::new(),
            grants,
            policy: DenyByDefaultPolicy,
            executor: InMemoryExecutor::new(),
            bus: InMemoryMessageBus::new(),
            log_entries: Vec::new(),
            next_nonce: 1,
        }
    }

    pub fn register(
        &mut self,
        manifest: AgentManifest,
        agent: Box<dyn Agent<Error = ReferenceAgentError>>,
    ) -> Result<(), DemoError> {
        self.registry.register(manifest.clone())?;
        self.agents.insert(
            manifest.name.as_str().to_owned(),
            HostedAgent { manifest, agent },
        );
        Ok(())
    }

    pub fn list_agents(&self) -> Vec<String> {
        self.registry
            .list()
            .into_iter()
            .map(|summary| summary.name.as_str().to_owned())
            .collect()
    }

    pub fn start_agent(&mut self, name: &str) -> Result<(), DemoError> {
        let decision = {
            let hosted = self
                .agents
                .get(name)
                .ok_or_else(|| DemoError::UnknownAgent(name.to_owned()))?;
            hosted.manifest.authorization(&self.grants)
        };

        match decision {
            ferros_agents::AuthorizationDecision::Authorized => {
                let hosted = self
                    .agents
                    .get_mut(name)
                    .ok_or_else(|| DemoError::UnknownAgent(name.to_owned()))?;
                hosted.agent.start()?;
                self.log_entries.push(format!("started:{name}"));
                Ok(())
            }
            ferros_agents::AuthorizationDecision::Denied { missing } => {
                let message = format!(
                    "{name} missing {}",
                    missing
                        .iter()
                        .map(|requirement| requirement.capability.as_str())
                        .collect::<Vec<_>>()
                        .join(",")
                );
                self.log_entries.push(format!("denied-start:{message}"));
                Err(DemoError::AuthorizationDenied(message))
            }
        }
    }

    pub fn stop_agent(&mut self, name: &str) -> Result<(), DemoError> {
        let hosted = self
            .agents
            .get_mut(name)
            .ok_or_else(|| DemoError::UnknownAgent(name.to_owned()))?;
        hosted.agent.stop()?;
        self.log_entries.push(format!("stopped:{name}"));
        Ok(())
    }

    pub fn send_message(
        &mut self,
        sender: &str,
        recipient: &str,
        capability: &str,
        payload: &[u8],
    ) -> Result<Option<Vec<u8>>, DemoError> {
        self.authorize(sender, capability)?;

        let envelope = MessageEnvelope::new(
            sender,
            recipient,
            Capability::new(capability)?,
            payload.to_vec(),
            self.allocate_nonce(),
        )?;

        self.executor
            .submit(envelope)
            .map_err(Self::map_infallible_executor)?;
        let Some(queued) = self
            .executor
            .pop_next()
            .map_err(Self::map_infallible_executor)?
        else {
            return Ok(None);
        };

        self.bus.send(queued).map_err(Self::map_infallible_bus)?;
        let Some(inbound) = self
            .bus
            .try_recv(recipient)
            .map_err(Self::map_infallible_bus)?
        else {
            return Ok(None);
        };

        let response_payload = {
            let hosted = self
                .agents
                .get_mut(recipient)
                .ok_or_else(|| DemoError::UnknownAgent(recipient.to_owned()))?;
            hosted.agent.handle_message(&inbound)?
        };

        let Some(response_payload) = response_payload else {
            return Ok(None);
        };

        let response = MessageEnvelope::new(
            recipient,
            sender,
            inbound.capability().clone(),
            response_payload,
            self.allocate_nonce(),
        )?;
        self.bus.send(response).map_err(Self::map_infallible_bus)?;

        Ok(self
            .bus
            .try_recv(sender)
            .map_err(Self::map_infallible_bus)?
            .map(|envelope| envelope.payload().to_vec()))
    }

    pub fn poll_agent(&mut self, name: &str) -> Result<Vec<Vec<u8>>, DemoError> {
        let capability = {
            let hosted = self
                .agents
                .get(name)
                .ok_or_else(|| DemoError::UnknownAgent(name.to_owned()))?;
            hosted
                .manifest
                .required_capabilities
                .first()
                .map(|requirement| requirement.capability.clone())
                .ok_or_else(|| DemoError::ManifestMissingCapabilities(name.to_owned()))?
        };

        self.authorize(name, &capability)?;

        let hosted = self
            .agents
            .get_mut(name)
            .ok_or_else(|| DemoError::UnknownAgent(name.to_owned()))?;
        hosted.agent.poll().map_err(Into::into)
    }

    #[must_use]
    pub fn log_entries(&self) -> &[String] {
        &self.log_entries
    }

    fn authorize(&mut self, name: &str, capability: &str) -> Result<(), DemoError> {
        let requester_profile_id = {
            let hosted = self
                .agents
                .get(name)
                .ok_or_else(|| DemoError::UnknownAgent(name.to_owned()))?;
            hosted
                .manifest
                .required_capabilities
                .first()
                .map(|requirement| requirement.profile_id.as_str().to_owned())
                .ok_or_else(|| DemoError::ManifestMissingCapabilities(name.to_owned()))?
        };

        let request = CapabilityRequest::new(requester_profile_id, Capability::new(capability)?)?;
        let decision = self.policy.evaluate(&request, &self.grants);

        if decision == PolicyDecision::Allowed {
            return Ok(());
        }

        let message = format!("{name}:{capability}:{decision:?}");
        self.log_entries.push(format!("denied:{message}"));
        Err(DemoError::AuthorizationDenied(message))
    }

    fn allocate_nonce(&mut self) -> u64 {
        let nonce = self.next_nonce;
        self.next_nonce += 1;
        nonce
    }

    fn map_infallible_executor(error: Infallible) -> DemoError {
        match error {}
    }

    fn map_infallible_bus(error: Infallible) -> DemoError {
        match error {}
    }
}

pub fn run_demo() -> Result<DemoSummary, DemoError> {
    let profile_id = ProfileId::new("profile-alpha")?;
    let grants = vec![
        CapabilityGrant::new(profile_id.clone(), "agent.echo"),
        CapabilityGrant::new(profile_id.clone(), "agent.timer"),
    ];
    let mut runtime = DemoRuntime::new(grants);

    let echo = EchoAgent::new(profile_id.clone());
    let timer = TimerAgent::new(profile_id);

    runtime.register(echo.manifest(), Box::new(echo))?;
    runtime.register(timer.manifest(), Box::new(timer))?;

    runtime.start_agent("echo")?;
    runtime.start_agent("timer")?;

    let echo_response = runtime
        .send_message("echo", "echo", "agent.echo", b"hello")?
        .ok_or(DemoError::MissingEchoResponse)?;

    let denied_requests = match runtime.send_message("echo", "echo", "agent.admin", b"nope") {
        Ok(_) => 0,
        Err(DemoError::AuthorizationDenied(_)) => 1,
        Err(error) => return Err(error),
    };

    let timer_event = runtime
        .poll_agent("timer")?
        .into_iter()
        .next()
        .ok_or(DemoError::MissingTimerEvent)?;

    runtime.stop_agent("echo")?;
    runtime.stop_agent("timer")?;

    Ok(DemoSummary {
        started_agents: runtime.list_agents(),
        echo_response: String::from_utf8_lossy(&echo_response).into_owned(),
        timer_event: String::from_utf8_lossy(&timer_event).into_owned(),
        denied_requests,
        log_entries: runtime.log_entries().to_vec(),
    })
}

#[cfg(test)]
mod tests {
    use super::{run_demo, DemoRuntime};
    use ferros_agents::{EchoAgent, TimerAgent};
    use ferros_profile::{CapabilityGrant, ProfileId};

    #[test]
    fn demo_runs_deterministically_and_denies_unauthorized_work() {
        let summary = run_demo().expect("demo should succeed");

        assert_eq!(summary.started_agents, vec!["echo".to_string(), "timer".to_string()]);
        assert_eq!(summary.echo_response, "hello");
        assert_eq!(summary.timer_event, "tick-1");
        assert_eq!(summary.denied_requests, 1);
        assert!(summary
            .log_entries
            .iter()
            .any(|entry| entry.starts_with("denied:")));
    }

    #[test]
    fn runtime_lists_registered_reference_agents() {
        let profile_id = ProfileId::new("profile-alpha").expect("valid profile id");
        let grants = vec![
            CapabilityGrant::new(profile_id.clone(), "agent.echo"),
            CapabilityGrant::new(profile_id.clone(), "agent.timer"),
        ];
        let mut runtime = DemoRuntime::new(grants);

        let echo = EchoAgent::new(profile_id.clone());
        let timer = TimerAgent::new(profile_id);

        runtime
            .register(echo.manifest(), Box::new(echo))
            .expect("echo should register");
        runtime
            .register(timer.manifest(), Box::new(timer))
            .expect("timer should register");

        assert_eq!(runtime.list_agents(), vec!["echo".to_string(), "timer".to_string()]);
    }
}