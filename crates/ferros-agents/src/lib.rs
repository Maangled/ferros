#![forbid(unsafe_code)]

pub mod agent;
pub mod manifest;
pub mod registry;

pub use agent::{Agent, AgentStatus};
pub use manifest::{
    AgentManifest, AgentName, AgentNameError, AuthorizationDecision, CapabilityRequirement,
};
pub use registry::{AgentRegistry, AgentSummary, InMemoryAgentRegistry, RegistryError};