use serde::{Deserialize, Serialize};

use crate::manifest::CapabilityRequirement;

pub const JSON_RPC_VERSION: &str = "2.0";
pub const METHOD_AGENT_LIST: &str = "agent.list";
pub const METHOD_AGENT_DESCRIBE: &str = "agent.describe";
pub const METHOD_AGENT_RUN: &str = "agent.run";
pub const METHOD_AGENT_STOP: &str = "agent.stop";
pub const METHOD_AGENT_SNAPSHOT: &str = "agent.snapshot";
pub const METHOD_GRANT_LIST: &str = "grant.list";
pub const METHOD_DENY_LOG_LIST: &str = "denyLog.list";

pub const JSON_RPC_INVALID_REQUEST: i32 = -32600;
pub const JSON_RPC_METHOD_NOT_FOUND: i32 = -32601;
pub const JSON_RPC_INVALID_PARAMS: i32 = -32602;
pub const JSON_RPC_AUTHORIZATION_DENIED: i32 = -32003;
pub const JSON_RPC_AGENT_NOT_FOUND: i32 = -32004;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentJsonRpcRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    #[serde(default, skip_serializing_if = "AgentJsonRpcParams::is_empty")]
    pub params: AgentJsonRpcParams,
}

impl AgentJsonRpcRequest {
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        method: impl Into<String>,
        params: AgentJsonRpcParams,
    ) -> Self {
        Self {
            jsonrpc: JSON_RPC_VERSION.to_owned(),
            id: id.into(),
            method: method.into(),
            params,
        }
    }

    #[must_use]
    pub fn is_version_supported(&self) -> bool {
        self.jsonrpc == JSON_RPC_VERSION
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentJsonRpcParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_path: Option<String>,
}

impl AgentJsonRpcParams {
    #[must_use]
    pub fn for_agent(agent_name: impl Into<String>) -> Self {
        Self {
            agent_name: Some(agent_name.into()),
            profile_path: None,
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.agent_name.is_none() && self.profile_path.is_none()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentJsonRpcResponse {
    pub jsonrpc: String,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<AgentJsonRpcResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<AgentJsonRpcError>,
}

impl AgentJsonRpcResponse {
    #[must_use]
    pub fn success(id: impl Into<String>, result: AgentJsonRpcResult) -> Self {
        Self {
            jsonrpc: JSON_RPC_VERSION.to_owned(),
            id: id.into(),
            result: Some(result),
            error: None,
        }
    }

    #[must_use]
    pub fn error(id: impl Into<String>, code: i32, message: impl Into<String>) -> Self {
        Self {
            jsonrpc: JSON_RPC_VERSION.to_owned(),
            id: id.into(),
            result: None,
            error: Some(AgentJsonRpcError {
                code,
                message: message.into(),
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentJsonRpcError {
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum AgentJsonRpcResult {
    AgentList { agents: Vec<AgentRpcAgentSummary> },
    AgentDetail { agent: AgentRpcAgentDetail },
    AgentLifecycle { agent: AgentRpcAgentDetail },
    AgentSnapshot { snapshot: AgentRpcSnapshot },
    GrantList { grants: Vec<GrantStateRecord> },
    DenyLog { entries: Vec<DenyLogEntry> },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentRpcAgentSummary {
    pub name: String,
    pub version: String,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentRpcAgentDetail {
    pub name: String,
    pub version: String,
    pub status: String,
    pub required_capabilities: Vec<CapabilityRequirement>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentRpcSnapshot {
    pub agents: Vec<AgentRpcAgentDetail>,
    pub grants: Vec<GrantStateRecord>,
    pub deny_log: Vec<DenyLogEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrantStateRecord {
    pub profile_id: String,
    pub capability: String,
    pub is_active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DenyLogEntry {
    pub entry_id: usize,
    pub kind: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::{
        AgentJsonRpcParams, AgentJsonRpcRequest, AgentJsonRpcResponse, AgentJsonRpcResult,
        AgentRpcAgentDetail, AgentRpcAgentSummary, AgentRpcSnapshot, DenyLogEntry,
        GrantStateRecord, METHOD_AGENT_LIST, METHOD_AGENT_RUN,
    };
    use crate::manifest::CapabilityRequirement;
    use ferros_profile::ProfileId;

    #[test]
    fn request_round_trips_through_json_with_empty_params_omitted() {
        let request =
            AgentJsonRpcRequest::new("req-1", METHOD_AGENT_LIST, AgentJsonRpcParams::default());
        let encoded = serde_json::to_value(&request).expect("request should serialize");
        let decoded: AgentJsonRpcRequest =
            serde_json::from_value(encoded.clone()).expect("request should deserialize");

        assert_eq!(decoded, request);
        assert!(encoded.get("params").is_none());
    }

    #[test]
    fn response_round_trips_typed_result_payloads() {
        let response = AgentJsonRpcResponse::success(
            "req-2",
            AgentJsonRpcResult::DenyLog {
                entries: vec![DenyLogEntry {
                    entry_id: 1,
                    kind: "denied".to_owned(),
                    message: "echo:agent.admin:Denied(NoGrantsPresented)".to_owned(),
                    agent_name: Some("echo".to_owned()),
                    capability: Some("agent.admin".to_owned()),
                }],
            },
        );
        let encoded = serde_json::to_string_pretty(&response).expect("response should serialize");
        let decoded: AgentJsonRpcResponse =
            serde_json::from_str(&encoded).expect("response should deserialize");

        assert_eq!(decoded, response);
    }

    #[test]
    fn response_round_trips_agent_snapshot_payloads() {
        let response = AgentJsonRpcResponse::success(
            "req-snapshot",
            AgentJsonRpcResult::AgentSnapshot {
                snapshot: AgentRpcSnapshot {
                    agents: vec![AgentRpcAgentDetail {
                        name: "echo".to_owned(),
                        version: "0.1.0".to_owned(),
                        status: "running".to_owned(),
                        required_capabilities: vec![CapabilityRequirement::new(
                            ProfileId::new("profile-alpha").expect("valid profile id"),
                            "agent.echo",
                        )],
                    }],
                    grants: vec![GrantStateRecord {
                        profile_id: "profile-alpha".to_owned(),
                        capability: "agent.echo".to_owned(),
                        is_active: true,
                        revoked_at: None,
                        revocation_reason: None,
                    }],
                    deny_log: vec![DenyLogEntry {
                        entry_id: 2,
                        kind: "denied".to_owned(),
                        message: "echo:agent.admin:Denied(NoGrantsPresented)".to_owned(),
                        agent_name: Some("echo".to_owned()),
                        capability: Some("agent.admin".to_owned()),
                    }],
                },
            },
        );
        let encoded = serde_json::to_string_pretty(&response).expect("response should serialize");
        let decoded: AgentJsonRpcResponse =
            serde_json::from_str(&encoded).expect("response should deserialize");

        assert_eq!(decoded, response);
    }

    #[test]
    fn response_round_trips_agent_lifecycle_payloads() {
        let response = AgentJsonRpcResponse::success(
            "req-lifecycle",
            AgentJsonRpcResult::AgentLifecycle {
                agent: AgentRpcAgentDetail {
                    name: "echo".to_owned(),
                    version: "0.1.0".to_owned(),
                    status: "running".to_owned(),
                    required_capabilities: vec![CapabilityRequirement::new(
                        ProfileId::new("profile-alpha").expect("valid profile id"),
                        "agent.echo",
                    )],
                },
            },
        );
        let encoded = serde_json::to_string_pretty(&response).expect("response should serialize");
        let decoded: AgentJsonRpcResponse =
            serde_json::from_str(&encoded).expect("response should deserialize");

        assert_eq!(decoded, response);
    }

    #[test]
    fn result_tags_agent_list_payloads_with_kind() {
        let response = AgentJsonRpcResponse::success(
            "req-3",
            AgentJsonRpcResult::AgentList {
                agents: vec![AgentRpcAgentSummary {
                    name: "echo".to_owned(),
                    version: "0.1.0".to_owned(),
                    status: "running".to_owned(),
                }],
            },
        );
        let encoded = serde_json::to_value(response).expect("response should serialize to JSON");

        assert_eq!(encoded["result"]["kind"], "agentList");
        assert_eq!(encoded["result"]["agents"][0]["name"], "echo");
    }

    #[test]
    fn request_round_trips_run_method_with_agent_name() {
        let request = AgentJsonRpcRequest::new(
            "req-run",
            METHOD_AGENT_RUN,
            AgentJsonRpcParams::for_agent("echo"),
        );
        let encoded = serde_json::to_value(&request).expect("request should serialize");
        let decoded: AgentJsonRpcRequest =
            serde_json::from_value(encoded.clone()).expect("request should deserialize");

        assert_eq!(decoded, request);
        assert_eq!(encoded["method"], METHOD_AGENT_RUN);
        assert_eq!(encoded["params"]["agentName"], "echo");
    }

    #[test]
    fn result_tags_agent_snapshot_payloads_with_kind_and_camel_case_fields() {
        let response = AgentJsonRpcResponse::success(
            "req-4",
            AgentJsonRpcResult::AgentSnapshot {
                snapshot: AgentRpcSnapshot {
                    agents: vec![AgentRpcAgentDetail {
                        name: "echo".to_owned(),
                        version: "0.1.0".to_owned(),
                        status: "registered".to_owned(),
                        required_capabilities: Vec::new(),
                    }],
                    grants: Vec::new(),
                    deny_log: vec![DenyLogEntry {
                        entry_id: 1,
                        kind: "deniedStart".to_owned(),
                        message: "echo missing agent.echo".to_owned(),
                        agent_name: Some("echo".to_owned()),
                        capability: Some("agent.echo".to_owned()),
                    }],
                },
            },
        );
        let encoded = serde_json::to_value(response).expect("response should serialize to JSON");

        assert_eq!(encoded["result"]["kind"], "agentSnapshot");
        assert!(encoded["result"]["snapshot"].get("denyLog").is_some());
        assert_eq!(encoded["result"]["snapshot"]["agents"][0]["name"], "echo");
    }
}
