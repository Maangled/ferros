use std::fmt;

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
            grant.profile_id == self.profile_id && grant.capability == self.capability
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentManifest {
    pub name: AgentName,
    pub version: String,
    pub required_capabilities: Vec<CapabilityRequirement>,
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
        }
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
    use super::{AgentManifest, AgentName, AgentNameError, AuthorizationDecision, CapabilityRequirement};
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

        let decision = manifest.authorization(&[CapabilityGrant::new(
            profile_id,
            "consent.read",
        )]);

        assert_eq!(
            decision,
            AuthorizationDecision::Denied {
                missing: vec![write_requirement],
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

        let encoded = serde_json::to_string_pretty(&manifest)
            .expect("agent manifest should serialize");
        let decoded: AgentManifest =
            serde_json::from_str(&encoded).expect("agent manifest should deserialize");

        assert_eq!(decoded, manifest);
    }
}