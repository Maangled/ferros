#![forbid(unsafe_code)]

use std::{
    collections::BTreeSet,
    fmt,
    fs::{self, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
};

use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use ferros_core::CapabilityGrantView;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

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

    pub fn sign(
        &self,
        signing_key: &SigningKey,
    ) -> Result<SignedCapabilityGrant, CapabilityGrantSignatureError> {
        SignedCapabilityGrant::new(self.clone(), signing_key)
    }

    fn canonical_signing_payload(&self) -> Result<String, CapabilityGrantSignatureError> {
        let mut payload = String::from("{\"profile_id\":");
        payload.push_str(&canonical_json_string(self.profile_id.as_str())?);
        payload.push_str(",\"capability\":");
        payload.push_str(&canonical_json_string(&self.capability)?);

        if let Some(revoked_at) = &self.revoked_at {
            payload.push_str(",\"revoked_at\":");
            payload.push_str(&canonical_json_string(revoked_at)?);
        }

        if let Some(revocation_reason) = &self.revocation_reason {
            payload.push_str(",\"revocation_reason\":");
            payload.push_str(&canonical_json_string(revocation_reason)?);
        }

        payload.push('}');
        Ok(payload)
    }

    fn canonical_bytes(&self) -> Result<Vec<u8>, CapabilityGrantSignatureError> {
        self.canonical_signing_payload().map(String::into_bytes)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapabilityGrantSignatureError {
    CanonicalSerialization(String),
    InvalidHex { field: &'static str },
    InvalidHexLength {
        field: &'static str,
        expected: usize,
        actual: usize,
    },
    InvalidPublicKey,
    SignerPublicKeyMismatch,
    SignatureMismatch,
}

impl fmt::Display for CapabilityGrantSignatureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CanonicalSerialization(error) => {
                write!(f, "canonical capability grant serialization failed: {error}")
            }
            Self::InvalidHex { field } => write!(f, "{field} must be valid hexadecimal"),
            Self::InvalidHexLength {
                field,
                expected,
                actual,
            } => write!(
                f,
                "{field} must be {expected} hex characters, got {actual}"
            ),
            Self::InvalidPublicKey => write!(f, "signer_public_key is not a valid Ed25519 key"),
            Self::SignerPublicKeyMismatch => {
                write!(f, "signing key does not match signer_public_key")
            }
            Self::SignatureMismatch => {
                write!(f, "capability grant signature verification failed")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignedCapabilityGrant {
    #[serde(flatten)]
    pub grant: CapabilityGrant,
    pub signer_public_key: String,
    pub signature: String,
}

impl SignedCapabilityGrant {
    pub fn new(
        grant: CapabilityGrant,
        signing_key: &SigningKey,
    ) -> Result<Self, CapabilityGrantSignatureError> {
        let signer_public_key = encode_hex(&signing_key.verifying_key().to_bytes());
        let signature = encode_hex(&signing_key.sign(&grant.canonical_bytes()?).to_bytes());

        Ok(Self {
            grant,
            signer_public_key,
            signature,
        })
    }

    #[must_use]
    pub fn grant(&self) -> &CapabilityGrant {
        &self.grant
    }

    pub fn verify(&self) -> Result<(), CapabilityGrantSignatureError> {
        let message = self.grant.canonical_bytes()?;
        let public_key_bytes = decode_hex_array::<32>(&self.signer_public_key, "signer_public_key")?;
        let signature_bytes = decode_hex_array::<64>(&self.signature, "signature")?;
        let verifying_key = VerifyingKey::from_bytes(&public_key_bytes)
            .map_err(|_| CapabilityGrantSignatureError::InvalidPublicKey)?;
        let signature = Signature::from_bytes(&signature_bytes);

        verifying_key
            .verify_strict(&message, &signature)
            .map_err(|_| CapabilityGrantSignatureError::SignatureMismatch)
    }

    pub fn revoke(
        &mut self,
        signing_key: &SigningKey,
        revoked_at: impl Into<String>,
        revocation_reason: impl Into<String>,
    ) -> Result<bool, CapabilityGrantSignatureError> {
        let expected_signer_public_key = encode_hex(&signing_key.verifying_key().to_bytes());

        if self.signer_public_key != expected_signer_public_key {
            return Err(CapabilityGrantSignatureError::SignerPublicKeyMismatch);
        }

        if !self.grant.revoke(revoked_at, revocation_reason) {
            return Ok(false);
        }

        self.signature = encode_hex(&signing_key.sign(&self.grant.canonical_bytes()?).to_bytes());
        Ok(true)
    }
}

fn canonical_json_string(value: &str) -> Result<String, CapabilityGrantSignatureError> {
    serde_json::to_string(value)
        .map_err(|error| CapabilityGrantSignatureError::CanonicalSerialization(error.to_string()))
}

fn encode_hex(bytes: &[u8]) -> String {
    let mut encoded = String::with_capacity(bytes.len() * 2);

    for byte in bytes {
        use fmt::Write as _;

        write!(&mut encoded, "{byte:02x}").expect("writing to a string should not fail");
    }

    encoded
}

fn decode_hex_array<const N: usize>(
    value: &str,
    field: &'static str,
) -> Result<[u8; N], CapabilityGrantSignatureError> {
    if value.len() != N * 2 {
        return Err(CapabilityGrantSignatureError::InvalidHexLength {
            field,
            expected: N * 2,
            actual: value.len(),
        });
    }

    let mut bytes = [0_u8; N];

    for (index, pair) in value.as_bytes().chunks_exact(2).enumerate() {
        let high = decode_hex_nibble(pair[0], field)?;
        let low = decode_hex_nibble(pair[1], field)?;
        bytes[index] = (high << 4) | low;
    }

    Ok(bytes)
}

fn decode_hex_nibble(
    nibble: u8,
    field: &'static str,
) -> Result<u8, CapabilityGrantSignatureError> {
    match nibble {
        b'0'..=b'9' => Ok(nibble - b'0'),
        b'a'..=b'f' => Ok(nibble - b'a' + 10),
        b'A'..=b'F' => Ok(nibble - b'A' + 10),
        _ => Err(CapabilityGrantSignatureError::InvalidHex { field }),
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

    fn is_active(&self) -> bool {
        !self.is_revoked()
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileDocumentError {
    InvalidCreatedAt(String),
}

impl fmt::Display for ProfileDocumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCreatedAt(created_at) => write!(
                f,
                "created_at must be a valid RFC3339 date-time, got {created_at:?}"
            ),
        }
    }
}

impl ProfileDocument {
    pub fn from_json_str(input: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(input)
    }

    pub fn fresh(
        display_name: impl Into<String>,
        created_at: impl Into<String>,
    ) -> Result<Self, ProfileDocumentError> {
        let display_name = display_name.into();
        let created_at = validate_created_at(created_at.into())?;
        let genesis_seal = initial_profile_seal(&display_name, &created_at);

        Ok(Self {
            meta: ProfileMeta {
                version: "1.0".to_owned(),
                created: created_at.clone(),
                last_modified: created_at.clone(),
                assistance_level: 1,
                genesis_hash: genesis_seal.clone(),
                current_seal: genesis_seal.clone(),
                seal_count: 1,
                stage: 0,
            },
            identity: ProfileIdentity {
                name: display_name.clone(),
                avatar: "star".to_owned(),
                class: None,
                stream_affinity: None,
                title: "Newcomer".to_owned(),
                joined_date: created_at.clone(),
                streak_days: 0,
                longest_streak: 0,
            },
            attributes: default_profile_attributes(),
            skills: default_profile_skills(),
            achievements: vec![json!({
                "id": "genesis_pioneer",
                "name": "Genesis Pioneer",
                "desc": "Created a FERROS profile",
                "icon": "trophy",
                "unlocked": true,
                "unlockedAt": created_at.clone(),
            })],
            journal: vec![json!({
                "ts": created_at.clone(),
                "text": format!("Profile created for {display_name}"),
                "type": "system",
            })],
            credentials: Vec::new(),
            seal_chain: vec![SealEntry {
                task_id: "genesis".to_owned(),
                seal: genesis_seal,
                previous_seal: "genesis".to_owned(),
                timestamp: created_at,
                data: json!({
                    "event": "profile_created",
                    "name": display_name,
                }),
                hash_algorithm: "sha256".to_owned(),
                nonce: 0,
            }],
        })
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
    AlreadyExists(PathBuf),
    InvalidProfile(ProfileDocumentError),
    Io(io::Error),
    Serde(serde_json::Error),
}

impl fmt::Display for ProfileStoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyExists(path) => {
                write!(f, "profile already exists at {}", path.display())
            }
            Self::InvalidProfile(error) => write!(f, "{error}"),
            Self::Io(error) => write!(f, "{error}"),
            Self::Serde(error) => write!(f, "{error}"),
        }
    }
}

impl From<ProfileDocumentError> for ProfileStoreError {
    fn from(value: ProfileDocumentError) -> Self {
        Self::InvalidProfile(value)
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

    fn create_profile(
        &self,
        path: &Path,
        profile: &ProfileDocument,
    ) -> Result<(), ProfileStoreError> {
        if path.exists() {
            return Err(ProfileStoreError::AlreadyExists(path.to_path_buf()));
        }

        self.save_profile(path, profile)
    }
}

pub fn init_profile<S: ProfileStore>(
    store: &S,
    path: &Path,
    display_name: impl Into<String>,
    created_at: impl Into<String>,
) -> Result<ProfileDocument, ProfileStoreError> {
    let profile = ProfileDocument::fresh(display_name, created_at)?;
    store.create_profile(path, &profile)?;
    store.load_profile(path)
}

fn validate_created_at(created_at: String) -> Result<String, ProfileDocumentError> {
    OffsetDateTime::parse(&created_at, &Rfc3339)
        .map(|_| created_at.clone())
        .map_err(|_| ProfileDocumentError::InvalidCreatedAt(created_at))
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

    fn create_profile(
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
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .map_err(|error| {
                if error.kind() == io::ErrorKind::AlreadyExists {
                    ProfileStoreError::AlreadyExists(path.to_path_buf())
                } else {
                    ProfileStoreError::Io(error)
                }
            })?;

        file.write_all(serialized.as_bytes())?;
        Ok(())
    }
}

fn default_profile_attributes() -> Value {
    json!({
        "Discipline": default_attribute("amber", "discipline"),
        "Knowledge": default_attribute("blue", "knowledge"),
        "Craft": default_attribute("cyan", "craft"),
        "Governance": default_attribute("purple", "governance"),
        "Wellness": default_attribute("green", "wellness"),
        "Community": default_attribute("pink", "community"),
    })
}

fn default_attribute(color: &str, icon: &str) -> Value {
    json!({
        "level": 1,
        "xp": 0,
        "xpToNext": 100,
        "color": color,
        "icon": icon,
    })
}

fn default_profile_skills() -> Value {
    json!({
        "A": [],
        "B": [],
        "C": [],
    })
}

fn initial_profile_seal(display_name: &str, created_at: &str) -> String {
    let mut slug = display_name
        .chars()
        .chain(created_at.chars())
        .filter_map(|character| {
            if character.is_ascii_alphanumeric() {
                Some(character.to_ascii_lowercase())
            } else if matches!(character, ' ' | '-' | '_' | ':') {
                Some('-')
            } else {
                None
            }
        })
        .collect::<String>();

    while slug.contains("--") {
        slug = slug.replace("--", "-");
    }

    let slug = slug.trim_matches('-');

    if slug.is_empty() {
        "profile-genesis".to_owned()
    } else {
        format!("profile-genesis-{slug}")
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

    use ed25519_dalek::SigningKey;

    use super::{
        foundation_contract_preview, init_profile, CapabilityGrant,
        CapabilityGrantSignatureError, ConsentManifest, ConsentManifestError,
        FileSystemProfileStore, ProfileDocument, ProfileDocumentError, ProfileId,
        ProfileIdError, ProfileStore, ProfileStoreError, SignedCapabilityGrant,
    };
    use serde_json::Value;

    const MINIMAL_STAGE0_FIXTURE: &str =
        include_str!("../../../schemas/fixtures/minimal-stage0-profile.json");
    const PROFILE_VALID_FIXTURE: &str =
        include_str!("../../../schemas/fixtures/profile-valid.json");
    const GRANT_VALID_FIXTURE: &str = include_str!("../../../schemas/fixtures/grant-valid.json");
    const GRANT_INVALID_SIGNATURE_FIXTURE: &str =
        include_str!("../../../schemas/fixtures/grant-invalid-sig.json");
    const PROFILE_V0_SCHEMA: &str = include_str!("../../../schemas/profile.v0.json");
    const CAPABILITY_GRANT_V0_SCHEMA: &str =
        include_str!("../../../schemas/capability-grant.v0.json");
    const TEST_SIGNING_KEY_BYTES: [u8; 32] = [
        0x10, 0x21, 0x32, 0x43, 0x54, 0x65, 0x76, 0x87, 0x98, 0xa9, 0xba, 0xcb, 0xdc, 0xed,
        0xfe, 0x0f, 0x1f, 0x2e, 0x3d, 0x4c, 0x5b, 0x6a, 0x79, 0x88, 0x97, 0xa6, 0xb5, 0xc4,
        0xd3, 0xe2, 0xf1, 0x01,
    ];

    fn test_signing_key() -> SigningKey {
        SigningKey::from_bytes(&TEST_SIGNING_KEY_BYTES)
    }

    fn signed_test_grant() -> SignedCapabilityGrant {
        CapabilityGrant::new(
            ProfileId::new("profile-alpha").expect("valid profile id"),
            "consent.read",
        )
        .sign(&test_signing_key())
        .expect("test grant should sign")
    }

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
    fn signed_capability_grant_round_trips_verify_and_revoke() {
        let signing_key = test_signing_key();
        let grant = CapabilityGrant::new(
            ProfileId::new("profile-alpha").expect("valid profile id"),
            "consent.read",
        );
        let signed_grant = grant.sign(&signing_key).expect("grant should sign");

        signed_grant.verify().expect("signed grant should verify");

        let serialized = serde_json::to_string_pretty(&signed_grant)
            .expect("signed grant should serialize");
        let mut reparsed = serde_json::from_str::<SignedCapabilityGrant>(&serialized)
            .expect("serialized signed grant should deserialize");

        reparsed.verify().expect("reparsed signed grant should verify");
        assert!(reparsed
            .revoke(
                &signing_key,
                "2026-04-23T00:00:00Z",
                "manual revoke",
            )
            .expect("revoked signed grant should re-sign"));
        reparsed
            .verify()
            .expect("revoked signed grant should still verify");
        assert!(reparsed.grant().is_revoked());
        assert_eq!(reparsed.grant().revoked_at.as_deref(), Some("2026-04-23T00:00:00Z"));
        assert_eq!(reparsed.grant().revocation_reason.as_deref(), Some("manual revoke"));
        assert!(!ferros_core::CapabilityGrantView::is_active(reparsed.grant()));
        assert!(!reparsed
            .revoke(
                &signing_key,
                "2026-04-23T00:05:00Z",
                "duplicate revoke",
            )
            .expect("duplicate revoke should stay idempotent"));
    }

    #[test]
    fn canonical_signing_payload_matches_published_active_contract() {
        let signed_grant = signed_test_grant();

        assert_eq!(
            signed_grant
                .grant()
                .canonical_signing_payload()
                .expect("canonical payload should serialize"),
            "{\"profile_id\":\"profile-alpha\",\"capability\":\"consent.read\"}"
        );
    }

    #[test]
    fn canonical_signing_payload_matches_published_revoked_contract() {
        let mut grant = CapabilityGrant::new(
            ProfileId::new("profile-alpha").expect("valid profile id"),
            "consent.read",
        );
        grant.revoke("2026-04-23T00:00:00Z", "manual revoke");

        assert_eq!(
            grant
                .canonical_signing_payload()
                .expect("canonical payload should serialize"),
            "{\"profile_id\":\"profile-alpha\",\"capability\":\"consent.read\",\"revoked_at\":\"2026-04-23T00:00:00Z\",\"revocation_reason\":\"manual revoke\"}"
        );
    }

    #[test]
    fn capability_grant_schema_publishes_current_signing_contract() {
        let schema = parse_schema(CAPABILITY_GRANT_V0_SCHEMA, "capability-grant.v0");
        let signing_contract = schema
            .get("x-ferros-signature")
            .and_then(Value::as_object)
            .expect("grant schema should publish a signing contract");
        let payload_fields = signing_contract
            .get("payload_fields_in_order")
            .and_then(Value::as_array)
            .expect("signing contract should list ordered payload fields")
            .iter()
            .map(|value| {
                value
                    .as_str()
                    .expect("payload field names should be strings")
            })
            .collect::<Vec<_>>();
        let optional_fields = signing_contract
            .get("payload_optional_fields")
            .and_then(Value::as_array)
            .expect("signing contract should list optional payload fields")
            .iter()
            .map(|value| {
                value
                    .as_str()
                    .expect("optional payload field names should be strings")
            })
            .collect::<Vec<_>>();
        let payload_examples = signing_contract
            .get("canonical_payload_examples")
            .and_then(Value::as_object)
            .expect("signing contract should publish payload examples");
        let mut revoked_grant = CapabilityGrant::new(
            ProfileId::new("profile-alpha").expect("valid profile id"),
            "consent.read",
        );
        revoked_grant.revoke("2026-04-23T00:00:00Z", "manual revoke");

        assert_eq!(
            payload_fields,
            vec!["profile_id", "capability", "revoked_at", "revocation_reason"]
        );
        assert_eq!(optional_fields, vec!["revoked_at", "revocation_reason"]);
        assert_eq!(
            payload_examples
                .get("active")
                .and_then(Value::as_str)
                .expect("signing contract should publish an active payload example"),
            signed_test_grant()
                .grant()
                .canonical_signing_payload()
                .expect("active payload should serialize")
        );
        assert_eq!(
            payload_examples
                .get("revoked")
                .and_then(Value::as_str)
                .expect("signing contract should publish a revoked payload example"),
            revoked_grant
                .canonical_signing_payload()
                .expect("revoked payload should serialize")
        );
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
    fn capability_grant_view_reports_revoked_grants_as_inactive() {
        let profile_id = ProfileId::new("profile-alpha").expect("valid profile id");
        let mut grant = CapabilityGrant::new(profile_id, "consent.read");

        assert!(ferros_core::CapabilityGrantView::is_active(&grant));

        grant.revoke("2026-04-23T00:00:00Z", "manual revoke");

        assert!(!ferros_core::CapabilityGrantView::is_active(&grant));
    }

    #[test]
    fn grant_valid_fixture_round_trips_and_matches_capability_grant_v0_contract() {
        let fixture =
            serde_json::from_str::<Value>(GRANT_VALID_FIXTURE).expect("grant fixture should parse");
        let signed_grant = serde_json::from_value::<SignedCapabilityGrant>(fixture.clone())
            .expect("grant fixture should deserialize");

        signed_grant
            .verify()
            .expect("signed grant fixture should verify");
        assert_eq!(signed_grant.grant().profile_id.as_str(), "profile-alpha");
        assert_eq!(signed_grant.grant().capability, "consent.read");

        let serialized = serde_json::to_value(&signed_grant)
            .expect("signed grant should convert to JSON");

        assert_eq!(serialized, fixture);
        assert_matches_capability_grant_v0_contract(&serialized);
    }

    #[test]
    fn invalid_signature_fixture_is_rejected() {
        let fixture = serde_json::from_str::<SignedCapabilityGrant>(GRANT_INVALID_SIGNATURE_FIXTURE)
            .expect("negative grant fixture should deserialize");

        assert_eq!(
            fixture.verify(),
            Err(CapabilityGrantSignatureError::SignatureMismatch)
        );
    }

    #[test]
    fn revoked_grant_matches_capability_grant_v0_contract() {
        let signing_key = test_signing_key();
        let mut signed_grant = signed_test_grant();
        signed_grant
            .revoke(&signing_key, "2026-04-23T00:00:00Z", "manual revoke")
            .expect("revoked signed grant should re-sign");

        let serialized = serde_json::to_value(&signed_grant)
            .expect("signed grant should convert to JSON");

        assert_matches_capability_grant_v0_contract(&serialized);
        assert_eq!(serialized["revoked_at"], "2026-04-23T00:00:00Z");
        assert_eq!(serialized["revocation_reason"], "manual revoke");
    }

    #[test]
    fn signed_capability_grant_rejects_mismatched_resigning_key() {
        let mut signed_grant = signed_test_grant();
        let other_signing_key = SigningKey::from_bytes(&[0x55; 32]);

        assert_eq!(
            signed_grant.revoke(
                &other_signing_key,
                "2026-04-23T00:00:00Z",
                "manual revoke",
            ),
            Err(CapabilityGrantSignatureError::SignerPublicKeyMismatch)
        );
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
    fn profile_valid_fixture_round_trips_and_matches_profile_v0_contract() {
        let fixture =
            serde_json::from_str::<Value>(PROFILE_VALID_FIXTURE).expect("fixture should parse");
        let profile = serde_json::from_value::<ProfileDocument>(fixture.clone())
            .expect("fixture should deserialize");
        let serialized = serde_json::to_value(&profile).expect("profile should convert to JSON");

        assert_eq!(profile.identity.name, "Fixture Pilot");
        assert!(profile.has_genesis_seal());
        assert_eq!(serialized, fixture);
        assert_matches_profile_v0_contract(&serialized);
    }

    #[test]
    fn serialized_profile_document_matches_profile_v0_contract() {
        let profile =
            ProfileDocument::from_json_str(MINIMAL_STAGE0_FIXTURE).expect("fixture should parse");
        let serialized = serde_json::to_value(&profile).expect("profile should convert to JSON");

        assert_matches_profile_v0_contract(&serialized);
    }

    #[test]
    fn fresh_profile_document_matches_profile_v0_contract() {
        let profile = ProfileDocument::fresh("Wave Pilot", "2026-04-23T10:00:00Z")
            .expect("valid RFC3339 timestamp should build a fresh profile");
        let serialized = serde_json::to_value(&profile).expect("profile should convert to JSON");

        assert_eq!(profile.identity.name, "Wave Pilot");
        assert!(profile.has_genesis_seal());
        assert_matches_profile_v0_contract(&serialized);
    }

    #[test]
    fn fresh_profile_document_rejects_invalid_created_at() {
        let error = ProfileDocument::fresh("Wave Pilot", "not-a-timestamp")
            .expect_err("invalid created_at should be rejected at the constructor boundary");

        assert_eq!(
            error,
            ProfileDocumentError::InvalidCreatedAt("not-a-timestamp".to_owned())
        );
    }

    #[test]
    fn init_profile_creates_new_profile_document_in_store() {
        let store = FileSystemProfileStore;
        let path = unique_temp_profile_path("init");

        let profile = init_profile(
            &store,
            &path,
            "Wave Pilot",
            "2026-04-23T10:00:00Z",
        )
        .expect("init should create a new profile");

        let loaded = store
            .load_profile(&path)
            .expect("new profile should load from disk");

        assert_eq!(profile, loaded);
        assert_eq!(loaded.identity.name, "Wave Pilot");
        assert!(loaded.has_genesis_seal());

        let _ = std::fs::remove_file(&path);
        if let Some(parent) = path.parent() {
            let _ = std::fs::remove_dir(parent);
        }
    }

    #[test]
    fn init_profile_rejects_existing_target_path() {
        let store = FileSystemProfileStore;
        let path = unique_temp_profile_path("init-existing");

        init_profile(
            &store,
            &path,
            "Wave Pilot",
            "2026-04-23T10:00:00Z",
        )
        .expect("initial profile should create");

        let error = init_profile(
            &store,
            &path,
            "Wave Pilot",
            "2026-04-23T10:01:00Z",
        )
        .expect_err("second init should not overwrite an existing profile");

        assert!(matches!(
            error,
            ProfileStoreError::AlreadyExists(existing_path) if existing_path == path
        ));

        let _ = std::fs::remove_file(&path);
        if let Some(parent) = path.parent() {
            let _ = std::fs::remove_dir(parent);
        }
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
