#![forbid(unsafe_code)]

pub mod agent;
pub mod bus;
pub mod manifest;
pub mod reference;
pub mod registry;
pub mod rpc;

pub use agent::{Agent, AgentStatus};
pub use bus::{
    BusChannel, BusEndpoint, BusEndpointError, BusListener, BusTransport, BusTransportKind,
};
pub use manifest::{
    AgentManifest, AgentName, AgentNameError, AuthorizationDecision, CapabilityRequirement,
};
pub use reference::{EchoAgent, ReferenceAgentError, TimerAgent};
pub use registry::{AgentRegistry, AgentSummary, InMemoryAgentRegistry, RegistryError};
pub use rpc::{
    AgentJsonRpcError, AgentJsonRpcParams, AgentJsonRpcRequest, AgentJsonRpcResponse,
    AgentJsonRpcResult, AgentRpcAgentDetail, AgentRpcAgentSummary, DenyLogEntry, GrantStateRecord,
    JSON_RPC_AGENT_NOT_FOUND, JSON_RPC_INVALID_PARAMS, JSON_RPC_INVALID_REQUEST,
    JSON_RPC_METHOD_NOT_FOUND, JSON_RPC_VERSION, METHOD_AGENT_DESCRIBE, METHOD_AGENT_LIST,
    METHOD_DENY_LOG_LIST, METHOD_GRANT_LIST,
};
