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
    use std::collections::BTreeSet;

    use super::{
        foundation_contract_preview, CapabilityGrant, ProfileDocument, ProfileId, ProfileIdError,
    };
    use serde_json::Value;

    const MINIMAL_STAGE0_FIXTURE: &str =
        include_str!("../../../schemas/fixtures/minimal-stage0-profile.json");
    const GRANT_VALID_FIXTURE: &str = include_str!("../../../schemas/fixtures/grant-valid.json");
    const PROFILE_V0_SCHEMA: &str = include_str!("../../../schemas/profile.v0.json");
    const CAPABILITY_GRANT_V0_SCHEMA: &str =
        include_str!("../../../schemas/capability-grant.v0.json");

    fn parse_schema(schema_source: &str, schema_name: &str) -> Value {
        serde_json::from_str(schema_source)
            .unwrap_or_else(|_| panic!("{schema_name} schema should parse"))
    }

    fn resolve_schema_node<'a>(
        root_schema: &'a Value,
        schema_node: &'a Value,
        label: &str,
    ) -> &'a Value {
        let mut current = schema_node;

        while let Some(reference) = current.get("$ref").and_then(Value::as_str) {
            let pointer = reference.strip_prefix('#').unwrap_or_else(|| {
                panic!("{label} should only use local schema refs: {reference}")
            });
            current = root_schema.pointer(pointer).unwrap_or_else(|| {
                panic!("{label} should resolve schema ref {reference}")
            });
        }

        current
    }

    fn json_kind(value: &Value) -> &'static str {
        match value {
            Value::Null => "null",
            Value::Bool(_) => "boolean",
            Value::Number(number) if number.is_i64() || number.is_u64() => "integer",
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
        }
    }

    fn matches_schema_type(value: &Value, schema_type: &str) -> bool {
        match schema_type {
            "null" => value.is_null(),
            "boolean" => value.is_boolean(),
            "integer" => {
                matches!(value, Value::Number(number) if number.is_i64() || number.is_u64())
            }
            "number" => value.is_number(),
            "string" => value.is_string(),
            "array" => value.is_array(),
            "object" => value.is_object(),
            _ => false,
        }
    }

    fn assert_value_kind_matches_schema(value: &Value, schema_node: &Value, label: &str) {
        let expected_types = schema_node
            .get("type")
            .unwrap_or_else(|| panic!("{label} should declare a schema type"));

        let matches = match expected_types {
            Value::String(schema_type) => matches_schema_type(value, schema_type),
            Value::Array(schema_types) => schema_types
                .iter()
                .filter_map(Value::as_str)
                .any(|schema_type| matches_schema_type(value, schema_type)),
            _ => false,
        };

        assert!(
            matches,
            "{label} should match schema type {expected_types}, got {}",
            json_kind(value)
        );
    }

    // This is a deliberate subset contract check: required fields, declared properties, and JSON value kinds.
    fn assert_matches_schema_contract(
        instance: &Value,
        schema_source: &str,
        schema_name: &str,
        root_label: &str,
    ) {
        fn visit(instance: &Value, schema_node: &Value, root_schema: &Value, label: &str) {
            let schema_node = resolve_schema_node(root_schema, schema_node, label);

            if schema_node.get("type").is_some() {
                assert_value_kind_matches_schema(instance, schema_node, label);
            }

            match instance {
                Value::Object(object) => {
                    let properties = schema_node.get("properties").and_then(Value::as_object);

                    if schema_node
                        .get("additionalProperties")
                        .and_then(Value::as_bool)
                        == Some(false)
                    {
                        let properties = properties.unwrap_or_else(|| {
                            panic!(
                                "{label} should define properties when additionalProperties is false"
                            )
                        });

                        let undeclared_fields = object
                            .keys()
                            .filter(|field| !properties.contains_key(*field))
                            .cloned()
                            .collect::<BTreeSet<_>>();

                        assert!(
                            undeclared_fields.is_empty(),
                            "{label} contains fields missing from the declared schema: {}",
                            undeclared_fields.into_iter().collect::<Vec<_>>().join(", ")
                        );
                    }

                    if let Some(required) = schema_node.get("required").and_then(Value::as_array) {
                        let missing_fields = required
                            .iter()
                            .filter_map(Value::as_str)
                            .filter(|field| !object.contains_key(*field))
                            .collect::<Vec<_>>();

                        assert!(
                            missing_fields.is_empty(),
                            "{label} is missing required schema fields: {}",
                            missing_fields.join(", ")
                        );
                    }

                    if let Some(properties) = properties {
                        for (field, field_value) in object {
                            if let Some(field_schema) = properties.get(field) {
                                let nested_label = format!("{label}.{field}");
                                visit(field_value, field_schema, root_schema, &nested_label);
                            }
                        }
                    }
                }
                Value::Array(items) => {
                    if let Some(item_schema) = schema_node.get("items") {
                        for (index, item) in items.iter().enumerate() {
                            let nested_label = format!("{label}[{index}]");
                            visit(item, item_schema, root_schema, &nested_label);
                        }
                    }
                }
                Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
            }
        }

        let schema = parse_schema(schema_source, schema_name);
        visit(instance, &schema, &schema, root_label);
    }

    fn assert_matches_profile_v0_contract(instance: &Value) {
        assert_matches_schema_contract(instance, PROFILE_V0_SCHEMA, "profile.v0", "profile");
    }

    fn assert_matches_capability_grant_v0_contract(instance: &Value) {
        assert_matches_schema_contract(
            instance,
            CAPABILITY_GRANT_V0_SCHEMA,
            "capability-grant.v0",
            "grant",
        );
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
    fn grant_valid_fixture_round_trips_and_matches_capability_grant_v0_contract() {
        let fixture =
            serde_json::from_str::<Value>(GRANT_VALID_FIXTURE).expect("grant fixture should parse");
        let grant = serde_json::from_value::<CapabilityGrant>(fixture.clone())
            .expect("grant fixture should deserialize");

        assert_eq!(grant.profile_id.as_str(), "profile-alpha");
        assert_eq!(grant.capability, "consent.read");

        let serialized = serde_json::to_value(&grant).expect("grant should convert to JSON");

        assert_eq!(serialized, fixture);
        assert_matches_capability_grant_v0_contract(&serialized);
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
    fn serialized_profile_document_matches_profile_v0_contract() {
        let profile =
            ProfileDocument::from_json_str(MINIMAL_STAGE0_FIXTURE).expect("fixture should parse");
        let serialized = serde_json::to_value(&profile).expect("profile should convert to JSON");

        assert_matches_profile_v0_contract(&serialized);
    }
}
