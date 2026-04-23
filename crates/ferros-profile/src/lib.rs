#![forbid(unsafe_code)]

use std::{collections::BTreeSet, fmt, fs, io, path::Path};

use ferros_core::CapabilityGrantView;
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_reason: Option<String>,
}

impl CapabilityGrant {
    #[must_use]
    pub fn new(profile_id: ProfileId, capability: impl Into<String>) -> Self {
        Self {
            profile_id,
            capability: capability.into(),
            revoked_at: None,
            revocation_reason: None,
        }
    }

    #[must_use]
    pub fn is_revoked(&self) -> bool {
        self.revoked_at.is_some()
    }

    pub fn revoke(
        &mut self,
        revoked_at: impl Into<String>,
        revocation_reason: impl Into<String>,
    ) -> bool {
        if self.is_revoked() {
            return false;
        }

        self.revoked_at = Some(revoked_at.into());
        self.revocation_reason = Some(revocation_reason.into());
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsentManifest {
    pub profile_id: ProfileId,
    pub grants: Vec<CapabilityGrant>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsentManifestError {
    GrantProfileMismatch { expected: String, found: String },
    DuplicateCapability(String),
}

impl fmt::Display for ConsentManifestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GrantProfileMismatch { expected, found } => write!(
                f,
                "grant profile id {found} does not match consent manifest profile {expected}"
            ),
            Self::DuplicateCapability(capability) => {
                write!(f, "duplicate capability in consent manifest: {capability}")
            }
        }
    }
}

impl ConsentManifest {
    pub fn new(
        profile_id: ProfileId,
        grants: Vec<CapabilityGrant>,
    ) -> Result<Self, ConsentManifestError> {
        let mut seen_capabilities = BTreeSet::new();

        for grant in &grants {
            if grant.profile_id != profile_id {
                return Err(ConsentManifestError::GrantProfileMismatch {
                    expected: profile_id.as_str().to_owned(),
                    found: grant.profile_id.as_str().to_owned(),
                });
            }

            if !seen_capabilities.insert(grant.capability.clone()) {
                return Err(ConsentManifestError::DuplicateCapability(
                    grant.capability.clone(),
                ));
            }
        }

        Ok(Self { profile_id, grants })
    }

    #[must_use]
    pub fn active_grants(&self) -> Vec<&CapabilityGrant> {
        self.grants.iter().filter(|grant| !grant.is_revoked()).collect()
    }

    pub fn revoke_capability(
        &mut self,
        capability: &str,
        revoked_at: impl Into<String>,
        revocation_reason: impl Into<String>,
    ) -> bool {
        let revoked_at = revoked_at.into();
        let revocation_reason = revocation_reason.into();

        self.grants
            .iter_mut()
            .find(|grant| grant.capability == capability)
            .is_some_and(|grant| grant.revoke(revoked_at, revocation_reason))
    }
}

impl CapabilityGrantView for CapabilityGrant {
    fn profile_id(&self) -> &str {
        self.profile_id.as_str()
    }

    fn capability(&self) -> &str {
        &self.capability
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

#[derive(Debug)]
pub enum ProfileStoreError {
    Io(io::Error),
    Serde(serde_json::Error),
}

impl fmt::Display for ProfileStoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "{error}"),
            Self::Serde(error) => write!(f, "{error}"),
        }
    }
}

impl From<io::Error> for ProfileStoreError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_json::Error> for ProfileStoreError {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}

pub trait ProfileStore {
    fn load_profile(&self, path: &Path) -> Result<ProfileDocument, ProfileStoreError>;

    fn save_profile(
        &self,
        path: &Path,
        profile: &ProfileDocument,
    ) -> Result<(), ProfileStoreError>;
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct FileSystemProfileStore;

impl ProfileStore for FileSystemProfileStore {
    fn load_profile(&self, path: &Path) -> Result<ProfileDocument, ProfileStoreError> {
        let contents = fs::read_to_string(path)?;
        Ok(ProfileDocument::from_json_str(&contents)?)
    }

    fn save_profile(
        &self,
        path: &Path,
        profile: &ProfileDocument,
    ) -> Result<(), ProfileStoreError> {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }

        let serialized = profile.to_json_string_pretty()?;
        fs::write(path, serialized)?;
        Ok(())
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
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::{
        foundation_contract_preview, CapabilityGrant, ConsentManifest, ConsentManifestError,
        FileSystemProfileStore, ProfileDocument, ProfileId, ProfileIdError, ProfileStore,
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

    fn unique_temp_profile_path(test_name: &str) -> PathBuf {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after epoch")
            .as_nanos();

        std::env::temp_dir()
            .join("ferros-profile-tests")
            .join(format!("{test_name}-{timestamp}.json"))
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
        assert!(!grant.is_revoked());
    }

    #[test]
    fn capability_grant_revocation_is_idempotent() {
        let profile_id = ProfileId::new("profile-alpha").expect("valid profile id");
        let mut grant = CapabilityGrant::new(profile_id, "consent.read");

        assert!(grant.revoke("2026-04-23T00:00:00Z", "manual revoke"));
        assert!(grant.is_revoked());
        assert_eq!(grant.revoked_at.as_deref(), Some("2026-04-23T00:00:00Z"));
        assert_eq!(grant.revocation_reason.as_deref(), Some("manual revoke"));
        assert!(!grant.revoke("2026-04-23T00:05:00Z", "second revoke"));
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
    fn revoked_grant_matches_capability_grant_v0_contract() {
        let profile_id = ProfileId::new("profile-alpha").expect("valid profile id");
        let mut grant = CapabilityGrant::new(profile_id, "consent.read");
        grant.revoke("2026-04-23T00:00:00Z", "manual revoke");

        let serialized = serde_json::to_value(&grant).expect("grant should convert to JSON");

        assert_matches_capability_grant_v0_contract(&serialized);
        assert_eq!(serialized["revoked_at"], "2026-04-23T00:00:00Z");
        assert_eq!(serialized["revocation_reason"], "manual revoke");
    }

    #[test]
    fn consent_manifest_rejects_grants_for_other_profiles() {
        let expected_profile = ProfileId::new("profile-alpha").expect("valid profile id");
        let other_profile = ProfileId::new("profile-beta").expect("valid profile id");

        let result = ConsentManifest::new(
            expected_profile,
            vec![CapabilityGrant::new(other_profile, "consent.read")],
        );

        assert_eq!(
            result,
            Err(ConsentManifestError::GrantProfileMismatch {
                expected: "profile-alpha".to_string(),
                found: "profile-beta".to_string(),
            })
        );
    }

    #[test]
    fn consent_manifest_rejects_duplicate_capabilities() {
        let profile_id = ProfileId::new("profile-alpha").expect("valid profile id");

        let result = ConsentManifest::new(
            profile_id.clone(),
            vec![
                CapabilityGrant::new(profile_id.clone(), "consent.read"),
                CapabilityGrant::new(profile_id, "consent.read"),
            ],
        );

        assert_eq!(
            result,
            Err(ConsentManifestError::DuplicateCapability(
                "consent.read".to_string(),
            ))
        );
    }

    #[test]
    fn consent_manifest_revoke_updates_active_grants() {
        let profile_id = ProfileId::new("profile-alpha").expect("valid profile id");
        let mut manifest = ConsentManifest::new(
            profile_id.clone(),
            vec![
                CapabilityGrant::new(profile_id.clone(), "consent.read"),
                CapabilityGrant::new(profile_id, "consent.write"),
            ],
        )
        .expect("manifest should build");

        assert_eq!(manifest.active_grants().len(), 2);
        assert!(manifest.revoke_capability(
            "consent.write",
            "2026-04-23T00:00:00Z",
            "user revoked write access",
        ));
        assert_eq!(manifest.active_grants().len(), 1);
        assert_eq!(manifest.active_grants()[0].capability, "consent.read");
        assert!(!manifest.revoke_capability(
            "consent.write",
            "2026-04-23T00:05:00Z",
            "duplicate revoke",
        ));
        assert!(!manifest.revoke_capability(
            "consent.admin",
            "2026-04-23T00:05:00Z",
            "missing capability",
        ));
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

    #[test]
    fn file_system_profile_store_round_trips_profile_document() {
        let store = FileSystemProfileStore;
        let path = unique_temp_profile_path("round-trip");
        let profile =
            ProfileDocument::from_json_str(MINIMAL_STAGE0_FIXTURE).expect("fixture should parse");

        store
            .save_profile(&path, &profile)
            .expect("profile should save to filesystem");
        let loaded = store
            .load_profile(&path)
            .expect("profile should load from filesystem");

        assert_eq!(loaded, profile);

        let _ = std::fs::remove_file(&path);
        if let Some(parent) = path.parent() {
            let _ = std::fs::remove_dir(parent);
        }
    }
}
