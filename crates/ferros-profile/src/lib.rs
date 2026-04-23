#![forbid(unsafe_code)]

use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileId(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileIdError {
    Empty,
    ContainsWhitespace,
}

impl fmt::Display for ProfileIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "profile id cannot be empty"),
            Self::ContainsWhitespace => write!(f, "profile id cannot contain whitespace"),
        }
    }
}

impl ProfileId {
    pub fn new(value: impl Into<String>) -> Result<Self, ProfileIdError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(ProfileIdError::Empty);
        }
        if value.chars().any(char::is_whitespace) {
            return Err(ProfileIdError::ContainsWhitespace);
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityGrant {
    pub profile_id: ProfileId,
    pub capability: String,
}

impl CapabilityGrant {
    #[must_use]
    pub fn new(profile_id: ProfileId, capability: impl Into<String>) -> Self {
        Self {
            profile_id,
            capability: capability.into(),
        }
    }
}

#[must_use]
pub fn foundation_contract_preview() -> &'static str {
    ferros_core::FOUNDATION_MARKER
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfileDocument {
    pub meta: ProfileMeta,
    pub identity: ProfileIdentity,
    pub attributes: Value,
    pub skills: Value,
    pub achievements: Vec<Value>,
    pub journal: Vec<Value>,
    pub credentials: Vec<Value>,
    #[serde(rename = "sealChain")]
    pub seal_chain: Vec<SealEntry>,
}

impl ProfileDocument {
    pub fn from_json_str(input: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(input)
    }

    pub fn to_json_string_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    #[must_use]
    pub fn has_genesis_seal(&self) -> bool {
        self.seal_chain
            .first()
            .is_some_and(|seal| seal.previous_seal == "genesis")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileMeta {
    pub version: String,
    pub created: String,
    #[serde(rename = "lastModified")]
    pub last_modified: String,
    #[serde(rename = "assistanceLevel")]
    pub assistance_level: u64,
    #[serde(rename = "genesisHash")]
    pub genesis_hash: String,
    #[serde(rename = "currentSeal")]
    pub current_seal: String,
    #[serde(rename = "sealCount")]
    pub seal_count: u64,
    pub stage: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileIdentity {
    pub name: String,
    pub avatar: String,
    pub class: Option<String>,
    #[serde(rename = "streamAffinity")]
    pub stream_affinity: Option<String>,
    pub title: String,
    #[serde(rename = "joinedDate")]
    pub joined_date: String,
    #[serde(rename = "streakDays")]
    pub streak_days: u64,
    #[serde(rename = "longestStreak")]
    pub longest_streak: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SealEntry {
    #[serde(rename = "taskId")]
    pub task_id: String,
    pub seal: String,
    #[serde(rename = "previousSeal")]
    pub previous_seal: String,
    pub timestamp: String,
    pub data: Value,
    #[serde(rename = "hashAlgorithm")]
    pub hash_algorithm: String,
    pub nonce: u64,
}

#[cfg(test)]
mod tests {
    use super::{
        foundation_contract_preview, CapabilityGrant, ProfileDocument, ProfileId, ProfileIdError,
    };
    use serde_json::Value;

    const MINIMAL_STAGE0_FIXTURE: &str =
        include_str!("../../../schemas/fixtures/minimal-stage0-profile.json");
    const PROFILE_V0_SCHEMA: &str = include_str!("../../../schemas/profile.v0.json");

    fn assert_matches_profile_v0_schema(instance: &Value) {
        let schema: Value =
            serde_json::from_str(PROFILE_V0_SCHEMA).expect("profile.v0 schema should parse");
        let config = jsonschema_valid::Config::from_schema(
            &schema,
            Some(jsonschema_valid::schemas::Draft::Draft7),
        )
        .expect("profile.v0 schema should compile");

        config
            .validate_schema()
            .expect("profile.v0 schema should be valid");

        if let Err(errors) = config.validate(instance) {
            let messages = errors.map(|error| error.to_string()).collect::<Vec<_>>();
            panic!(
                "profile instance should satisfy profile.v0.json: {}",
                messages.join("; ")
            );
        }
    }

    #[test]
    fn profile_id_rejects_empty_values() {
        assert_eq!(ProfileId::new(""), Err(ProfileIdError::Empty));
    }

    #[test]
    fn profile_id_rejects_whitespace() {
        assert_eq!(
            ProfileId::new("profile one"),
            Err(ProfileIdError::ContainsWhitespace)
        );
    }

    #[test]
    fn capability_grant_keeps_profile_identity() {
        let profile_id = ProfileId::new("profile-alpha").expect("valid profile id");
        let grant = CapabilityGrant::new(profile_id.clone(), "consent.read");

        assert_eq!(grant.profile_id, profile_id);
        assert_eq!(grant.capability, "consent.read");
    }

    #[test]
    fn profile_crate_exposes_foundation_marker() {
        assert_eq!(foundation_contract_preview(), "foundation-ready");
    }

    #[test]
    fn minimal_stage0_fixture_deserializes() {
        let profile =
            ProfileDocument::from_json_str(MINIMAL_STAGE0_FIXTURE).expect("fixture should parse");

        assert_eq!(profile.meta.stage, 0);
        assert_eq!(profile.identity.name, "Fresh Start");
        assert!(profile.has_genesis_seal());
    }

    #[test]
    fn minimal_stage0_fixture_round_trips_through_json() {
        let profile =
            ProfileDocument::from_json_str(MINIMAL_STAGE0_FIXTURE).expect("fixture should parse");
        let serialized = profile
            .to_json_string_pretty()
            .expect("profile should serialize");
        let reparsed =
            ProfileDocument::from_json_str(&serialized).expect("serialized profile should parse");

        assert_eq!(reparsed.meta.version, "1.0");
        assert_eq!(reparsed.seal_chain.len(), 1);
        assert_eq!(reparsed.identity.title, "Newcomer");
    }

    #[test]
    fn serialized_profile_document_matches_profile_v0_schema() {
        let profile =
            ProfileDocument::from_json_str(MINIMAL_STAGE0_FIXTURE).expect("fixture should parse");
        let serialized = serde_json::to_value(&profile).expect("profile should convert to JSON");

        assert_matches_profile_v0_schema(&serialized);
    }
}
