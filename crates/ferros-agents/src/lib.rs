#![forbid(unsafe_code)]

pub mod agent;
pub mod bus;
pub mod manifest;
pub mod registry;

pub use agent::{Agent, AgentStatus};
pub use bus::{
    BusChannel, BusEndpoint, BusEndpointError, BusListener, BusTransport,
    BusTransportKind,
};
pub use manifest::{
    AgentManifest, AgentName, AgentNameError, AuthorizationDecision, CapabilityRequirement,
};
pub use registry::{AgentRegistry, AgentSummary, InMemoryAgentRegistry, RegistryError};