use std::collections::BTreeMap;
use std::fmt;

use ferros_core::CapabilityGrantView;
use ferros_profile::{CapabilityGrant, ProfileId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct AgentName(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentNameError {
    Empty,
    ContainsWhitespace,
}

impl fmt::Display for AgentNameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "agent name cannot be empty"),
            Self::ContainsWhitespace => write!(f, "agent name cannot contain whitespace"),
        }
    }
}

impl AgentName {
    pub fn new(value: impl Into<String>) -> Result<Self, AgentNameError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(AgentNameError::Empty);
        }
        if value.chars().any(char::is_whitespace) {
            return Err(AgentNameError::ContainsWhitespace);
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for AgentName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityRequirement {
    pub profile_id: ProfileId,
    pub capability: String,
}

impl CapabilityRequirement {
    #[must_use]
    pub fn new(profile_id: ProfileId, capability: impl Into<String>) -> Self {
        Self {
            profile_id,
            capability: capability.into(),
        }
    }

    #[must_use]
    pub fn is_satisfied_by(&self, grants: &[CapabilityGrant]) -> bool {
        grants.iter().any(|grant| {
            grant.is_active()
                && grant.profile_id == self.profile_id
                && grant.capability == self.capability
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentRuntime {
    InProcess,
    Subprocess,
    CoordinatorSdk,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentManifest {
    pub name: AgentName,
    pub version: String,
    pub required_capabilities: Vec<CapabilityRequirement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtime: Option<AgentRuntime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub env: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan_template: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route_target_stream: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route_target_family: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_target_agent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selection_tokens: Vec<String>,
}

impl AgentManifest {
    #[must_use]
    pub fn new(
        name: AgentName,
        version: impl Into<String>,
        required_capabilities: Vec<CapabilityRequirement>,
    ) -> Self {
        Self {
            name,
            version: version.into(),
            required_capabilities,
            runtime: None,
            command: None,
            args: Vec::new(),
            env: BTreeMap::new(),
            plan_template: None,
            route_target_stream: None,
            route_target_family: None,
            lifecycle_target_agent_id: None,
            selection_tokens: Vec::new(),
        }
    }

    #[must_use]
    pub fn with_runtime_metadata(
        mut self,
        runtime: AgentRuntime,
        command: Option<String>,
        args: Vec<String>,
        env: BTreeMap<String, String>,
        plan_template: Option<String>,
        route_target_stream: Option<String>,
        route_target_family: Option<String>,
        lifecycle_target_agent_id: Option<String>,
        selection_tokens: Vec<String>,
    ) -> Self {
        self.runtime = Some(runtime);
        self.command = command;
        self.args = args;
        self.env = env;
        self.plan_template = plan_template;
        self.route_target_stream = route_target_stream;
        self.route_target_family = route_target_family;
        self.lifecycle_target_agent_id = lifecycle_target_agent_id;
        self.selection_tokens = selection_tokens;
        self
    }

    #[must_use]
    pub fn missing_capabilities(&self, grants: &[CapabilityGrant]) -> Vec<CapabilityRequirement> {
        self.required_capabilities
            .iter()
            .filter(|requirement| !requirement.is_satisfied_by(grants))
            .cloned()
            .collect()
    }

    #[must_use]
    pub fn authorization(&self, grants: &[CapabilityGrant]) -> AuthorizationDecision {
        let missing = self.missing_capabilities(grants);
        if missing.is_empty() {
            AuthorizationDecision::Authorized
        } else {
            AuthorizationDecision::Denied { missing }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthorizationDecision {
    Authorized,
    Denied { missing: Vec<CapabilityRequirement> },
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::{
        AgentManifest, AgentName, AgentNameError, AgentRuntime, AuthorizationDecision,
        CapabilityRequirement,
    };
    use ferros_profile::{CapabilityGrant, ProfileId};

    #[test]
    fn agent_name_rejects_invalid_values() {
        assert_eq!(AgentName::new(""), Err(AgentNameError::Empty));
        assert_eq!(
            AgentName::new("echo agent"),
            Err(AgentNameError::ContainsWhitespace)
        );
    }

    #[test]
    fn manifest_authorization_is_deny_by_default() {
        let profile_id = ProfileId::new("profile-alpha").expect("valid profile id");
        let read_requirement = CapabilityRequirement::new(profile_id.clone(), "consent.read");
        let write_requirement = CapabilityRequirement::new(profile_id.clone(), "consent.write");
        let manifest = AgentManifest::new(
            AgentName::new("echo").expect("valid agent name"),
            "0.1.0",
            vec![read_requirement.clone(), write_requirement.clone()],
        );

        let decision = manifest.authorization(&[CapabilityGrant::new(profile_id, "consent.read")]);

        assert_eq!(
            decision,
            AuthorizationDecision::Denied {
                missing: vec![write_requirement],
            }
        );
    }

    #[test]
    fn manifest_authorization_requires_matching_profile_id() {
        let required_profile = ProfileId::new("profile-alpha").expect("valid profile id");
        let granted_profile = ProfileId::new("profile-beta").expect("valid profile id");
        let read_requirement = CapabilityRequirement::new(required_profile.clone(), "consent.read");
        let manifest = AgentManifest::new(
            AgentName::new("echo").expect("valid agent name"),
            "0.1.0",
            vec![read_requirement.clone()],
        );

        let decision =
            manifest.authorization(&[CapabilityGrant::new(granted_profile, "consent.read")]);

        assert_eq!(
            decision,
            AuthorizationDecision::Denied {
                missing: vec![read_requirement],
            }
        );
    }

    #[test]
    fn manifest_authorization_rejects_revoked_grants() {
        let profile_id = ProfileId::new("profile-alpha").expect("valid profile id");
        let requirement = CapabilityRequirement::new(profile_id.clone(), "consent.read");
        let manifest = AgentManifest::new(
            AgentName::new("echo").expect("valid agent name"),
            "0.1.0",
            vec![requirement.clone()],
        );
        let mut revoked_grant = CapabilityGrant::new(profile_id, "consent.read");

        assert!(revoked_grant.revoke("2026-04-23T00:00:00Z", "manual revoke"));

        assert_eq!(
            manifest.authorization(&[revoked_grant]),
            AuthorizationDecision::Denied {
                missing: vec![requirement],
            }
        );
    }

    #[test]
    fn manifest_round_trips_through_json() {
        let profile_id = ProfileId::new("profile-alpha").expect("valid profile id");
        let manifest = AgentManifest::new(
            AgentName::new("timer").expect("valid agent name"),
            "0.1.0",
            vec![CapabilityRequirement::new(profile_id, "clock.tick")],
        );

        let encoded =
            serde_json::to_string_pretty(&manifest).expect("agent manifest should serialize");
        let decoded: AgentManifest =
            serde_json::from_str(&encoded).expect("agent manifest should deserialize");

        assert_eq!(decoded, manifest);
    }

    #[test]
    fn manifest_runtime_metadata_round_trips_through_json() {
        let profile_id = ProfileId::new("profile-alpha").expect("valid profile id");
        let mut env = BTreeMap::new();
        env.insert("FERROS_MODE".to_owned(), "local".to_owned());
        let manifest = AgentManifest::new(
            AgentName::new("worker").expect("valid agent name"),
            "0.1.0",
            vec![CapabilityRequirement::new(profile_id, "runtime.dispatch")],
        )
        .with_runtime_metadata(
            AgentRuntime::Subprocess,
            Some("ferros-worker".to_owned()),
            vec!["--once".to_owned(), "--json".to_owned()],
            env,
            Some("default-worker-plan".to_owned()),
            None,
            Some("coding".to_owned()),
            Some("worker".to_owned()),
            vec!["worker".to_owned(), "dispatch".to_owned()],
        );

        let encoded =
            serde_json::to_string_pretty(&manifest).expect("agent manifest should serialize");
        let decoded: AgentManifest =
            serde_json::from_str(&encoded).expect("agent manifest should deserialize");

        assert_eq!(decoded, manifest);
        assert_eq!(decoded.runtime, Some(AgentRuntime::Subprocess));
        assert_eq!(decoded.command.as_deref(), Some("ferros-worker"));
        assert_eq!(decoded.args, vec!["--once".to_owned(), "--json".to_owned()]);
        assert_eq!(
            decoded.env.get("FERROS_MODE").map(String::as_str),
            Some("local")
        );
        assert_eq!(decoded.plan_template.as_deref(), Some("default-worker-plan"));
        assert_eq!(decoded.route_target_stream, None);
        assert_eq!(decoded.route_target_family.as_deref(), Some("coding"));
        assert_eq!(decoded.lifecycle_target_agent_id.as_deref(), Some("worker"));
        assert_eq!(decoded.selection_tokens, vec!["worker".to_owned(), "dispatch".to_owned()]);
    }
}
