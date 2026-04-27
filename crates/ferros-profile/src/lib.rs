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

const LOCAL_KEY_PAIR_STATE_FORMAT: &str = "ferros.local.key-pair.v0";
const LOCAL_SIGNED_GRANT_STATE_FORMAT: &str = "ferros.local.signed-grants.v0";
const LOCAL_PROFILE_BUNDLE_FORMAT: &str = "ferros.local.profile-bundle.v0";

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

    #[must_use]
    pub fn from_public_key(verifying_key: &VerifyingKey) -> Self {
        Self(encode_hex(&verifying_key.to_bytes()))
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
enum HexDecodeError {
    InvalidHex {
        field: &'static str,
    },
    InvalidHexLength {
        field: &'static str,
        expected: usize,
        actual: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapabilityGrantSignatureError {
    CanonicalSerialization(String),
    InvalidHex {
        field: &'static str,
    },
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
                write!(
                    f,
                    "canonical capability grant serialization failed: {error}"
                )
            }
            Self::InvalidHex { field } => write!(f, "{field} must be valid hexadecimal"),
            Self::InvalidHexLength {
                field,
                expected,
                actual,
            } => write!(f, "{field} must be {expected} hex characters, got {actual}"),
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

impl From<HexDecodeError> for CapabilityGrantSignatureError {
    fn from(value: HexDecodeError) -> Self {
        match value {
            HexDecodeError::InvalidHex { field } => Self::InvalidHex { field },
            HexDecodeError::InvalidHexLength {
                field,
                expected,
                actual,
            } => Self::InvalidHexLength {
                field,
                expected,
                actual,
            },
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
        let public_key_bytes =
            decode_hex_array::<32>(&self.signer_public_key, "signer_public_key")?;
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

fn json_string_literal(value: &str) -> Result<String, serde_json::Error> {
    serde_json::to_string(value)
}

fn canonical_json_string(value: &str) -> Result<String, CapabilityGrantSignatureError> {
    json_string_literal(value)
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
) -> Result<[u8; N], HexDecodeError> {
    if value.len() != N * 2 {
        return Err(HexDecodeError::InvalidHexLength {
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

fn decode_hex_nibble(nibble: u8, field: &'static str) -> Result<u8, HexDecodeError> {
    match nibble {
        b'0'..=b'9' => Ok(nibble - b'0'),
        b'a'..=b'f' => Ok(nibble - b'a' + 10),
        b'A'..=b'F' => Ok(nibble - b'A' + 10),
        _ => Err(HexDecodeError::InvalidHex { field }),
    }
}

#[derive(Clone)]
pub struct KeyPair {
    device_label: String,
    signing_key: SigningKey,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyPairError {
    EmptyDeviceLabel,
    EntropyUnavailable,
    InvalidHex {
        field: &'static str,
    },
    InvalidHexLength {
        field: &'static str,
        expected: usize,
        actual: usize,
    },
}

impl fmt::Debug for KeyPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("KeyPair")
            .field("device_label", &self.device_label)
            .field("profile_id", &self.profile_id())
            .field("public_key", &self.public_key_hex())
            .finish()
    }
}

impl fmt::Display for KeyPairError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyDeviceLabel => write!(f, "device label cannot be empty"),
            Self::EntropyUnavailable => {
                write!(
                    f,
                    "operating-system entropy is unavailable for Ed25519 key generation"
                )
            }
            Self::InvalidHex { field } => write!(f, "{field} must be valid hexadecimal"),
            Self::InvalidHexLength {
                field,
                expected,
                actual,
            } => write!(f, "{field} must be {expected} hex characters, got {actual}"),
        }
    }
}

impl From<HexDecodeError> for KeyPairError {
    fn from(value: HexDecodeError) -> Self {
        match value {
            HexDecodeError::InvalidHex { field } => Self::InvalidHex { field },
            HexDecodeError::InvalidHexLength {
                field,
                expected,
                actual,
            } => Self::InvalidHexLength {
                field,
                expected,
                actual,
            },
        }
    }
}

impl KeyPair {
    pub fn generate(device_label: impl Into<String>) -> Result<Self, KeyPairError> {
        let mut secret_key_bytes = [0_u8; 32];
        getrandom::getrandom(&mut secret_key_bytes)
            .map_err(|_| KeyPairError::EntropyUnavailable)?;

        Self::from_secret_key_bytes(device_label, secret_key_bytes)
    }

    pub fn from_secret_key_hex(
        device_label: impl Into<String>,
        secret_key_hex: &str,
    ) -> Result<Self, KeyPairError> {
        let secret_key_bytes = decode_hex_array::<32>(secret_key_hex, "secret_key")?;
        Self::from_secret_key_bytes(device_label, secret_key_bytes)
    }

    fn from_secret_key_bytes(
        device_label: impl Into<String>,
        secret_key_bytes: [u8; 32],
    ) -> Result<Self, KeyPairError> {
        let device_label = device_label.into();

        if device_label.trim().is_empty() {
            return Err(KeyPairError::EmptyDeviceLabel);
        }

        Ok(Self {
            device_label,
            signing_key: SigningKey::from_bytes(&secret_key_bytes),
        })
    }

    #[must_use]
    pub fn device_label(&self) -> &str {
        &self.device_label
    }

    #[must_use]
    pub fn profile_id(&self) -> ProfileId {
        ProfileId::from_public_key(&self.signing_key.verifying_key())
    }

    #[must_use]
    pub fn public_key_hex(&self) -> String {
        encode_hex(&self.signing_key.verifying_key().to_bytes())
    }

    #[must_use]
    pub fn secret_key_hex(&self) -> String {
        encode_hex(&self.signing_key.to_bytes())
    }

    pub fn sign_profile(
        &self,
        profile: &ProfileDocument,
    ) -> Result<SignedProfileDocument, ProfileSignatureError> {
        SignedProfileDocument::new(profile.clone(), self)
    }

    pub fn sign_grant(
        &self,
        grant: &CapabilityGrant,
    ) -> Result<SignedCapabilityGrant, CapabilityGrantSignatureError> {
        grant.sign(self.signing_key())
    }

    pub fn revoke_grant(
        &self,
        grant: &mut SignedCapabilityGrant,
        revoked_at: impl Into<String>,
        revocation_reason: impl Into<String>,
    ) -> Result<bool, CapabilityGrantSignatureError> {
        grant.revoke(self.signing_key(), revoked_at, revocation_reason)
    }

    fn signing_key(&self) -> &SigningKey {
        &self.signing_key
    }
}

#[derive(Debug, Clone)]
pub struct LocalProfileState {
    pub profile: ProfileDocument,
    pub key_pair: KeyPair,
    pub signed_grants: Vec<SignedCapabilityGrant>,
}

impl LocalProfileState {
    pub fn new(
        profile: ProfileDocument,
        key_pair: KeyPair,
        signed_grants: Vec<SignedCapabilityGrant>,
    ) -> Result<Self, ProfileStoreError> {
        let state = Self {
            profile,
            key_pair,
            signed_grants,
        };

        state.validate()?;
        Ok(state)
    }

    fn validate(&self) -> Result<(), ProfileStoreError> {
        let expected_profile_id = self.key_pair.profile_id();
        let expected_signer_public_key = self.key_pair.public_key_hex();
        let mut capabilities = BTreeSet::new();

        for grant in &self.signed_grants {
            grant.verify()?;

            if grant.grant.profile_id != expected_profile_id {
                return Err(ProfileStoreError::InvalidLocalState(format!(
                    "grant {} profile id does not match local key pair",
                    grant.grant.capability
                )));
            }

            if grant.signer_public_key != expected_signer_public_key {
                return Err(ProfileStoreError::InvalidLocalState(format!(
                    "grant {} signer_public_key does not match local key pair",
                    grant.grant.capability
                )));
            }

            if !capabilities.insert(grant.grant.capability.clone()) {
                return Err(ProfileStoreError::InvalidLocalState(format!(
                    "duplicate capability in local grant state: {}",
                    grant.grant.capability
                )));
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct StoredKeyPair {
    format: String,
    device_label: String,
    secret_key: String,
}

impl StoredKeyPair {
    fn from_key_pair(key_pair: &KeyPair) -> Self {
        Self {
            format: LOCAL_KEY_PAIR_STATE_FORMAT.to_owned(),
            device_label: key_pair.device_label().to_owned(),
            secret_key: key_pair.secret_key_hex(),
        }
    }

    fn into_key_pair(self) -> Result<KeyPair, ProfileStoreError> {
        if self.format != LOCAL_KEY_PAIR_STATE_FORMAT {
            return Err(ProfileStoreError::InvalidLocalState(format!(
                "unsupported local key pair state format: {}",
                self.format
            )));
        }

        KeyPair::from_secret_key_hex(self.device_label, &self.secret_key)
            .map_err(ProfileStoreError::from)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct StoredSignedGrantState {
    format: String,
    #[serde(default)]
    grants: Vec<SignedCapabilityGrant>,
}

impl StoredSignedGrantState {
    fn from_signed_grants(grants: &[SignedCapabilityGrant]) -> Self {
        Self {
            format: LOCAL_SIGNED_GRANT_STATE_FORMAT.to_owned(),
            grants: grants.to_vec(),
        }
    }

    fn into_signed_grants(self) -> Result<Vec<SignedCapabilityGrant>, ProfileStoreError> {
        if self.format != LOCAL_SIGNED_GRANT_STATE_FORMAT {
            return Err(ProfileStoreError::InvalidLocalState(format!(
                "unsupported local signed grant state format: {}",
                self.format
            )));
        }

        for grant in &self.grants {
            grant.verify()?;
        }

        Ok(self.grants)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct StoredProfileBundle {
    format: String,
    profile: ProfileDocument,
    key_pair: StoredKeyPair,
    #[serde(default)]
    grants: Vec<SignedCapabilityGrant>,
}

impl StoredProfileBundle {
    fn from_local_state(state: &LocalProfileState) -> Self {
        Self {
            format: LOCAL_PROFILE_BUNDLE_FORMAT.to_owned(),
            profile: state.profile.clone(),
            key_pair: StoredKeyPair::from_key_pair(&state.key_pair),
            grants: state.signed_grants.clone(),
        }
    }

    fn into_local_state(self) -> Result<LocalProfileState, ProfileStoreError> {
        if self.format != LOCAL_PROFILE_BUNDLE_FORMAT {
            return Err(ProfileStoreError::InvalidLocalState(format!(
                "unsupported local profile bundle format: {}",
                self.format
            )));
        }

        LocalProfileState::new(self.profile, self.key_pair.into_key_pair()?, self.grants)
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
        self.grants
            .iter()
            .filter(|grant| !grant.is_revoked())
            .collect()
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

    pub fn sign(&self, key_pair: &KeyPair) -> Result<SignedProfileDocument, ProfileSignatureError> {
        key_pair.sign_profile(self)
    }

    #[must_use]
    pub fn has_genesis_seal(&self) -> bool {
        self.seal_chain
            .first()
            .is_some_and(|seal| seal.previous_seal == "genesis")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileSignatureError {
    CanonicalSerialization(String),
    InvalidHex {
        field: &'static str,
    },
    InvalidHexLength {
        field: &'static str,
        expected: usize,
        actual: usize,
    },
    InvalidPublicKey,
    ProfileIdMismatch,
    SignerPublicKeyMismatch,
    SignatureMismatch,
}

impl fmt::Display for ProfileSignatureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CanonicalSerialization(error) => {
                write!(f, "canonical profile serialization failed: {error}")
            }
            Self::InvalidHex { field } => write!(f, "{field} must be valid hexadecimal"),
            Self::InvalidHexLength {
                field,
                expected,
                actual,
            } => write!(f, "{field} must be {expected} hex characters, got {actual}"),
            Self::InvalidPublicKey => write!(f, "signer_public_key is not a valid Ed25519 key"),
            Self::ProfileIdMismatch => {
                write!(f, "profile_id does not match signer_public_key")
            }
            Self::SignerPublicKeyMismatch => {
                write!(f, "signing key does not match signer_public_key")
            }
            Self::SignatureMismatch => write!(f, "profile signature verification failed"),
        }
    }
}

impl From<HexDecodeError> for ProfileSignatureError {
    fn from(value: HexDecodeError) -> Self {
        match value {
            HexDecodeError::InvalidHex { field } => Self::InvalidHex { field },
            HexDecodeError::InvalidHexLength {
                field,
                expected,
                actual,
            } => Self::InvalidHexLength {
                field,
                expected,
                actual,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignedProfileDocument {
    pub profile_id: ProfileId,
    pub profile: ProfileDocument,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_reason: Option<String>,
    pub signer_public_key: String,
    pub signature: String,
}

impl SignedProfileDocument {
    pub fn new(
        profile: ProfileDocument,
        key_pair: &KeyPair,
    ) -> Result<Self, ProfileSignatureError> {
        let mut signed_profile = Self {
            profile_id: key_pair.profile_id(),
            profile,
            revoked_at: None,
            revocation_reason: None,
            signer_public_key: key_pair.public_key_hex(),
            signature: String::new(),
        };

        signed_profile.signature = encode_hex(
            &key_pair
                .signing_key()
                .sign(&signed_profile.canonical_bytes()?)
                .to_bytes(),
        );

        Ok(signed_profile)
    }

    #[must_use]
    pub fn profile(&self) -> &ProfileDocument {
        &self.profile
    }

    #[must_use]
    pub fn profile_id(&self) -> &ProfileId {
        &self.profile_id
    }

    #[must_use]
    pub fn is_revoked(&self) -> bool {
        self.revoked_at.is_some()
    }

    pub fn verify(&self) -> Result<(), ProfileSignatureError> {
        let message = self.canonical_bytes()?;
        let public_key_bytes =
            decode_hex_array::<32>(&self.signer_public_key, "signer_public_key")?;
        let signature_bytes = decode_hex_array::<64>(&self.signature, "signature")?;
        let verifying_key = VerifyingKey::from_bytes(&public_key_bytes)
            .map_err(|_| ProfileSignatureError::InvalidPublicKey)?;
        let expected_profile_id = ProfileId::from_public_key(&verifying_key);
        let signature = Signature::from_bytes(&signature_bytes);

        if self.profile_id != expected_profile_id {
            return Err(ProfileSignatureError::ProfileIdMismatch);
        }

        verifying_key
            .verify_strict(&message, &signature)
            .map_err(|_| ProfileSignatureError::SignatureMismatch)
    }

    pub fn revoke(
        &mut self,
        key_pair: &KeyPair,
        revoked_at: impl Into<String>,
        revocation_reason: impl Into<String>,
    ) -> Result<bool, ProfileSignatureError> {
        if self.signer_public_key != key_pair.public_key_hex() {
            return Err(ProfileSignatureError::SignerPublicKeyMismatch);
        }

        if self.profile_id != key_pair.profile_id() {
            return Err(ProfileSignatureError::ProfileIdMismatch);
        }

        if self.is_revoked() {
            return Ok(false);
        }

        self.revoked_at = Some(revoked_at.into());
        self.revocation_reason = Some(revocation_reason.into());
        self.signature = encode_hex(
            &key_pair
                .signing_key()
                .sign(&self.canonical_bytes()?)
                .to_bytes(),
        );

        Ok(true)
    }

    fn canonical_signing_payload(&self) -> Result<String, ProfileSignatureError> {
        let mut payload = String::from("{\"profile_id\":");
        payload.push_str(&profile_json_string(self.profile_id.as_str())?);
        payload.push_str(",\"profile\":");
        payload.push_str(&canonical_profile_json(&self.profile)?);

        if let Some(revoked_at) = &self.revoked_at {
            payload.push_str(",\"revoked_at\":");
            payload.push_str(&profile_json_string(revoked_at)?);
        }

        if let Some(revocation_reason) = &self.revocation_reason {
            payload.push_str(",\"revocation_reason\":");
            payload.push_str(&profile_json_string(revocation_reason)?);
        }

        payload.push('}');
        Ok(payload)
    }

    fn canonical_bytes(&self) -> Result<Vec<u8>, ProfileSignatureError> {
        self.canonical_signing_payload().map(String::into_bytes)
    }
}

#[derive(Debug)]
pub enum ProfileStoreError {
    AlreadyExists(PathBuf),
    CapabilityGrantAlreadyExists(String),
    CapabilityGrantNotFound(String),
    CapabilityGrantSignature(CapabilityGrantSignatureError),
    InvalidProfile(ProfileDocumentError),
    InvalidLocalState(String),
    Io(io::Error),
    KeyPair(KeyPairError),
    Serde(serde_json::Error),
}

impl fmt::Display for ProfileStoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyExists(path) => {
                write!(f, "profile already exists at {}", path.display())
            }
            Self::CapabilityGrantAlreadyExists(capability) => {
                write!(f, "capability grant already exists: {capability}")
            }
            Self::CapabilityGrantNotFound(capability) => {
                write!(f, "capability grant not found: {capability}")
            }
            Self::CapabilityGrantSignature(error) => write!(f, "{error}"),
            Self::InvalidProfile(error) => write!(f, "{error}"),
            Self::InvalidLocalState(message) => write!(f, "invalid local profile state: {message}"),
            Self::Io(error) => write!(f, "{error}"),
            Self::KeyPair(error) => write!(f, "{error}"),
            Self::Serde(error) => write!(f, "{error}"),
        }
    }
}

impl From<CapabilityGrantSignatureError> for ProfileStoreError {
    fn from(value: CapabilityGrantSignatureError) -> Self {
        Self::CapabilityGrantSignature(value)
    }
}

impl From<ProfileDocumentError> for ProfileStoreError {
    fn from(value: ProfileDocumentError) -> Self {
        Self::InvalidProfile(value)
    }
}

impl From<KeyPairError> for ProfileStoreError {
    fn from(value: KeyPairError) -> Self {
        Self::KeyPair(value)
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

    fn save_profile(&self, path: &Path, profile: &ProfileDocument)
        -> Result<(), ProfileStoreError>;

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

pub trait LocalProfileStore: ProfileStore {
    fn create_local_profile(
        &self,
        path: &Path,
        profile: &ProfileDocument,
        key_pair: &KeyPair,
    ) -> Result<(), ProfileStoreError>;

    fn load_local_profile(&self, path: &Path) -> Result<LocalProfileState, ProfileStoreError>;

    fn save_signed_grants(
        &self,
        path: &Path,
        grants: &[SignedCapabilityGrant],
    ) -> Result<(), ProfileStoreError>;

    fn export_profile_bundle(
        &self,
        profile_path: &Path,
        bundle_path: &Path,
    ) -> Result<(), ProfileStoreError>;

    fn import_profile_bundle(
        &self,
        bundle_path: &Path,
        profile_path: &Path,
    ) -> Result<LocalProfileState, ProfileStoreError>;
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

pub fn init_local_profile<S: LocalProfileStore>(
    store: &S,
    path: &Path,
    display_name: impl Into<String>,
    created_at: impl Into<String>,
    device_label: impl Into<String>,
) -> Result<LocalProfileState, ProfileStoreError> {
    let profile = ProfileDocument::fresh(display_name, created_at)?;
    let key_pair = KeyPair::generate(device_label)?;

    store.create_local_profile(path, &profile, &key_pair)?;
    store.load_local_profile(path)
}

pub fn grant_profile_capability<S: LocalProfileStore>(
    store: &S,
    path: &Path,
    capability: impl Into<String>,
) -> Result<SignedCapabilityGrant, ProfileStoreError> {
    let mut state = store.load_local_profile(path)?;
    let capability = capability.into();

    if state
        .signed_grants
        .iter()
        .any(|grant| grant.grant.capability == capability)
    {
        return Err(ProfileStoreError::CapabilityGrantAlreadyExists(capability));
    }

    let signed_grant = state.key_pair.sign_grant(&CapabilityGrant::new(
        state.key_pair.profile_id(),
        capability,
    ))?;

    state.signed_grants.push(signed_grant.clone());
    store.save_signed_grants(path, &state.signed_grants)?;

    Ok(signed_grant)
}

pub fn revoke_profile_capability<S: LocalProfileStore>(
    store: &S,
    path: &Path,
    capability: &str,
    revoked_at: impl Into<String>,
    revocation_reason: impl Into<String>,
) -> Result<SignedCapabilityGrant, ProfileStoreError> {
    let mut state = store.load_local_profile(path)?;
    let revoked_at = revoked_at.into();
    let revocation_reason = revocation_reason.into();
    let grant = state
        .signed_grants
        .iter_mut()
        .find(|grant| grant.grant.capability == capability)
        .ok_or_else(|| ProfileStoreError::CapabilityGrantNotFound(capability.to_owned()))?;

    state
        .key_pair
        .revoke_grant(grant, revoked_at, revocation_reason)?;
    let updated_grant = grant.clone();

    store.save_signed_grants(path, &state.signed_grants)?;

    Ok(updated_grant)
}

fn validate_created_at(created_at: String) -> Result<String, ProfileDocumentError> {
    OffsetDateTime::parse(&created_at, &Rfc3339)
        .map(|_| created_at.clone())
        .map_err(|_| ProfileDocumentError::InvalidCreatedAt(created_at))
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct FileSystemProfileStore;

impl FileSystemProfileStore {
    fn key_pair_path(profile_path: &Path) -> PathBuf {
        profile_path.with_extension("key.json")
    }

    fn signed_grants_path(profile_path: &Path) -> PathBuf {
        profile_path.with_extension("grants.json")
    }

    fn write_json_pretty<T: Serialize>(
        &self,
        path: &Path,
        value: &T,
    ) -> Result<(), ProfileStoreError> {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }

        let serialized = serde_json::to_string_pretty(value)?;
        fs::write(path, serialized)?;
        Ok(())
    }

    fn write_json_pretty_new<T: Serialize>(
        &self,
        path: &Path,
        value: &T,
    ) -> Result<(), ProfileStoreError> {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }

        let serialized = serde_json::to_string_pretty(value)?;
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

    fn remove_local_profile_artifacts(&self, profile_path: &Path) {
        let _ = fs::remove_file(profile_path);
        let _ = fs::remove_file(Self::key_pair_path(profile_path));
        let _ = fs::remove_file(Self::signed_grants_path(profile_path));
    }

    fn load_key_pair(&self, profile_path: &Path) -> Result<KeyPair, ProfileStoreError> {
        let contents = fs::read_to_string(Self::key_pair_path(profile_path))?;
        let stored: StoredKeyPair = serde_json::from_str(&contents)?;
        stored.into_key_pair()
    }

    fn load_signed_grants(
        &self,
        profile_path: &Path,
    ) -> Result<Vec<SignedCapabilityGrant>, ProfileStoreError> {
        let grants_path = Self::signed_grants_path(profile_path);
        let contents = match fs::read_to_string(grants_path) {
            Ok(contents) => contents,
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(error) => return Err(ProfileStoreError::Io(error)),
        };
        let stored: StoredSignedGrantState = serde_json::from_str(&contents)?;
        stored.into_signed_grants()
    }
}

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

impl LocalProfileStore for FileSystemProfileStore {
    fn create_local_profile(
        &self,
        path: &Path,
        profile: &ProfileDocument,
        key_pair: &KeyPair,
    ) -> Result<(), ProfileStoreError> {
        let key_pair_path = Self::key_pair_path(path);

        if path.exists() {
            return Err(ProfileStoreError::AlreadyExists(path.to_path_buf()));
        }

        if key_pair_path.exists() {
            return Err(ProfileStoreError::AlreadyExists(key_pair_path));
        }

        self.create_profile(path, profile)?;

        if let Err(error) = self.write_json_pretty_new(
            &Self::key_pair_path(path),
            &StoredKeyPair::from_key_pair(key_pair),
        ) {
            let _ = fs::remove_file(path);
            return Err(error);
        }

        Ok(())
    }

    fn load_local_profile(&self, path: &Path) -> Result<LocalProfileState, ProfileStoreError> {
        LocalProfileState::new(
            self.load_profile(path)?,
            self.load_key_pair(path)?,
            self.load_signed_grants(path)?,
        )
    }

    fn save_signed_grants(
        &self,
        path: &Path,
        grants: &[SignedCapabilityGrant],
    ) -> Result<(), ProfileStoreError> {
        for grant in grants {
            grant.verify()?;
        }

        self.write_json_pretty(
            &Self::signed_grants_path(path),
            &StoredSignedGrantState::from_signed_grants(grants),
        )
    }

    fn export_profile_bundle(
        &self,
        profile_path: &Path,
        bundle_path: &Path,
    ) -> Result<(), ProfileStoreError> {
        let state = self.load_local_profile(profile_path)?;
        self.write_json_pretty_new(bundle_path, &StoredProfileBundle::from_local_state(&state))
    }

    fn import_profile_bundle(
        &self,
        bundle_path: &Path,
        profile_path: &Path,
    ) -> Result<LocalProfileState, ProfileStoreError> {
        let contents = fs::read_to_string(bundle_path)?;
        let bundle: StoredProfileBundle = serde_json::from_str(&contents)?;
        let state = bundle.into_local_state()?;

        self.create_local_profile(profile_path, &state.profile, &state.key_pair)?;

        if let Err(error) = self.save_signed_grants(profile_path, &state.signed_grants) {
            self.remove_local_profile_artifacts(profile_path);
            return Err(error);
        }

        match self.load_local_profile(profile_path) {
            Ok(local_state) => Ok(local_state),
            Err(error) => {
                self.remove_local_profile_artifacts(profile_path);
                Err(error)
            }
        }
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

fn profile_json_string(value: &str) -> Result<String, ProfileSignatureError> {
    json_string_literal(value)
        .map_err(|error| ProfileSignatureError::CanonicalSerialization(error.to_string()))
}

fn canonical_profile_json(profile: &ProfileDocument) -> Result<String, ProfileSignatureError> {
    let value = serde_json::to_value(profile)
        .map_err(|error| ProfileSignatureError::CanonicalSerialization(error.to_string()))?;

    canonical_json_value(&value)
        .map_err(|error| ProfileSignatureError::CanonicalSerialization(error.to_string()))
}

fn canonical_json_value(value: &Value) -> Result<String, serde_json::Error> {
    let mut encoded = String::new();
    write_canonical_json_value(value, &mut encoded)?;
    Ok(encoded)
}

fn write_canonical_json_value(
    value: &Value,
    encoded: &mut String,
) -> Result<(), serde_json::Error> {
    match value {
        Value::Null => encoded.push_str("null"),
        Value::Bool(boolean) => {
            encoded.push_str(if *boolean { "true" } else { "false" });
        }
        Value::Number(number) => encoded.push_str(&number.to_string()),
        Value::String(string) => encoded.push_str(&json_string_literal(string)?),
        Value::Array(items) => {
            encoded.push('[');

            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    encoded.push(',');
                }

                write_canonical_json_value(item, encoded)?;
            }

            encoded.push(']');
        }
        Value::Object(object) => {
            encoded.push('{');

            let mut entries = object.iter().collect::<Vec<_>>();
            entries.sort_by_key(|(left, _)| *left);

            for (index, (key, nested_value)) in entries.into_iter().enumerate() {
                if index > 0 {
                    encoded.push(',');
                }

                encoded.push_str(&json_string_literal(key)?);
                encoded.push(':');
                write_canonical_json_value(nested_value, encoded)?;
            }

            encoded.push('}');
        }
    }

    Ok(())
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
    use std::path::{Path, PathBuf};
    use std::time::{SystemTime, UNIX_EPOCH};

    use ed25519_dalek::SigningKey;

    use super::{
        encode_hex, foundation_contract_preview, init_profile, CapabilityGrant,
        CapabilityGrantSignatureError, ConsentManifest, ConsentManifestError,
        FileSystemProfileStore, KeyPair, KeyPairError, LocalProfileStore, ProfileDocument,
        ProfileDocumentError, ProfileId, ProfileIdError, ProfileSignatureError, ProfileStore,
        ProfileStoreError, SignedCapabilityGrant, SignedProfileDocument,
    };
    use serde_json::Value;

    const MINIMAL_STAGE0_FIXTURE: &str =
        include_str!("../../../schemas/fixtures/minimal-stage0-profile.json");
    const PROFILE_VALID_FIXTURE: &str =
        include_str!("../../../schemas/fixtures/profile-valid.json");
    const SIGNED_PROFILE_VALID_FIXTURE: &str =
        include_str!("../../../schemas/fixtures/signed-profile-valid.json");
    const GRANT_VALID_FIXTURE: &str = include_str!("../../../schemas/fixtures/grant-valid.json");
    const GRANT_INVALID_SIGNATURE_FIXTURE: &str =
        include_str!("../../../schemas/fixtures/grant-invalid-sig.json");
    const PROFILE_V0_SCHEMA: &str = include_str!("../../../schemas/profile.v0.json");
    const CAPABILITY_GRANT_V0_SCHEMA: &str =
        include_str!("../../../schemas/capability-grant.v0.json");
    const SIGNED_PROFILE_ENVELOPE_ONLY_FIELDS: [&str; 5] = [
        "profile_id",
        "revoked_at",
        "revocation_reason",
        "signer_public_key",
        "signature",
    ];
    const TEST_SIGNING_KEY_BYTES: [u8; 32] = [
        0x10, 0x21, 0x32, 0x43, 0x54, 0x65, 0x76, 0x87, 0x98, 0xa9, 0xba, 0xcb, 0xdc, 0xed, 0xfe,
        0x0f, 0x1f, 0x2e, 0x3d, 0x4c, 0x5b, 0x6a, 0x79, 0x88, 0x97, 0xa6, 0xb5, 0xc4, 0xd3, 0xe2,
        0xf1, 0x01,
    ];

    fn test_signing_key() -> SigningKey {
        SigningKey::from_bytes(&TEST_SIGNING_KEY_BYTES)
    }

    fn test_key_pair() -> KeyPair {
        KeyPair::from_secret_key_hex("local-test-device", &encode_hex(&TEST_SIGNING_KEY_BYTES))
            .expect("test key pair should decode")
    }

    fn signed_test_grant() -> SignedCapabilityGrant {
        CapabilityGrant::new(
            ProfileId::new("profile-alpha").expect("valid profile id"),
            "consent.read",
        )
        .sign(&test_signing_key())
        .expect("test grant should sign")
    }

    fn signed_test_profile() -> SignedProfileDocument {
        ProfileDocument::fresh("Wave Pilot", "2026-04-23T10:00:00Z")
            .expect("fresh profile should build")
            .sign(&test_key_pair())
            .expect("fresh profile should sign")
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
            current = root_schema
                .pointer(pointer)
                .unwrap_or_else(|| panic!("{label} should resolve schema ref {reference}"));
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

    fn cleanup_local_profile_artifacts(path: &Path) {
        let _ = std::fs::remove_file(path);
        let _ = std::fs::remove_file(FileSystemProfileStore::key_pair_path(path));
        let _ = std::fs::remove_file(FileSystemProfileStore::signed_grants_path(path));

        if let Some(parent) = path.parent() {
            let _ = std::fs::remove_dir(parent);
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

        let serialized =
            serde_json::to_string_pretty(&signed_grant).expect("signed grant should serialize");
        let mut reparsed = serde_json::from_str::<SignedCapabilityGrant>(&serialized)
            .expect("serialized signed grant should deserialize");

        reparsed
            .verify()
            .expect("reparsed signed grant should verify");
        assert!(reparsed
            .revoke(&signing_key, "2026-04-23T00:00:00Z", "manual revoke",)
            .expect("revoked signed grant should re-sign"));
        reparsed
            .verify()
            .expect("revoked signed grant should still verify");
        assert!(reparsed.grant().is_revoked());
        assert_eq!(
            reparsed.grant().revoked_at.as_deref(),
            Some("2026-04-23T00:00:00Z")
        );
        assert_eq!(
            reparsed.grant().revocation_reason.as_deref(),
            Some("manual revoke")
        );
        assert!(!ferros_core::CapabilityGrantView::is_active(
            reparsed.grant()
        ));
        assert!(!reparsed
            .revoke(&signing_key, "2026-04-23T00:05:00Z", "duplicate revoke",)
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
            vec![
                "profile_id",
                "capability",
                "revoked_at",
                "revocation_reason"
            ]
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

        let serialized =
            serde_json::to_value(&signed_grant).expect("signed grant should convert to JSON");

        assert_eq!(serialized, fixture);
        assert_matches_capability_grant_v0_contract(&serialized);
    }

    #[test]
    fn invalid_signature_fixture_is_rejected() {
        let fixture =
            serde_json::from_str::<SignedCapabilityGrant>(GRANT_INVALID_SIGNATURE_FIXTURE)
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

        let serialized =
            serde_json::to_value(&signed_grant).expect("signed grant should convert to JSON");

        assert_matches_capability_grant_v0_contract(&serialized);
        assert_eq!(serialized["revoked_at"], "2026-04-23T00:00:00Z");
        assert_eq!(serialized["revocation_reason"], "manual revoke");
    }

    #[test]
    fn signed_capability_grant_rejects_mismatched_resigning_key() {
        let mut signed_grant = signed_test_grant();
        let other_signing_key = SigningKey::from_bytes(&[0x55; 32]);

        assert_eq!(
            signed_grant.revoke(&other_signing_key, "2026-04-23T00:00:00Z", "manual revoke",),
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
        let serialized = serde_json::to_value(profile).expect("profile should convert to JSON");

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
    fn profile_v0_schema_freezes_the_unsigned_boundary() {
        let schema = parse_schema(PROFILE_V0_SCHEMA, "profile.v0");
        let description = schema
            .get("description")
            .and_then(Value::as_str)
            .expect("profile.v0 schema should describe its frozen boundary");
        let comment = schema
            .get("$comment")
            .and_then(Value::as_str)
            .expect("profile.v0 schema should document the signed-profile freeze boundary");
        let properties = schema
            .get("properties")
            .and_then(Value::as_object)
            .expect("profile.v0 schema should declare root properties");

        assert!(
            description.contains("Frozen S2-owned unsigned FERROS profile schema"),
            "profile.v0 schema should identify itself as the frozen unsigned v0 contract"
        );
        assert!(
            comment.contains("SignedProfileDocument stays Rust-local at v0"),
            "profile.v0 schema should keep SignedProfileDocument Rust-local at v0"
        );
        assert!(
            !properties.contains_key("profile"),
            "profile.v0 schema should describe the profile payload directly, not a signed wrapper"
        );

        for field in SIGNED_PROFILE_ENVELOPE_ONLY_FIELDS {
            assert!(
                !properties.contains_key(field),
                "profile.v0 schema should stay unsigned and omit envelope-only field {field}"
            );
        }
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
    fn key_pair_rejects_empty_device_label() {
        assert!(matches!(
            KeyPair::from_secret_key_hex("   ", &encode_hex(&TEST_SIGNING_KEY_BYTES)),
            Err(KeyPairError::EmptyDeviceLabel)
        ));
    }

    #[test]
    fn key_pair_generates_and_round_trips_secret_key_hex() {
        let generated = KeyPair::generate("local-device").expect("key generation should succeed");
        let reparsed =
            KeyPair::from_secret_key_hex(generated.device_label(), &generated.secret_key_hex())
                .expect("generated secret key should round-trip");

        assert_eq!(generated.device_label(), "local-device");
        assert_eq!(generated.profile_id(), reparsed.profile_id());
        assert_eq!(generated.public_key_hex(), reparsed.public_key_hex());
    }

    #[test]
    fn signed_profile_document_round_trips_verify_and_revoke() {
        let key_pair = test_key_pair();
        let profile = ProfileDocument::fresh("Wave Pilot", "2026-04-23T10:00:00Z")
            .expect("fresh profile should build");
        let signed_profile = profile.sign(&key_pair).expect("fresh profile should sign");

        signed_profile
            .verify()
            .expect("signed profile should verify");

        let serialized =
            serde_json::to_string_pretty(&signed_profile).expect("signed profile should serialize");
        let mut reparsed = serde_json::from_str::<SignedProfileDocument>(&serialized)
            .expect("serialized signed profile should deserialize");

        reparsed
            .verify()
            .expect("reparsed signed profile should verify");
        assert_eq!(reparsed.profile_id(), &key_pair.profile_id());
        assert_eq!(reparsed.profile().identity.name, "Wave Pilot");
        assert!(reparsed
            .revoke(&key_pair, "2026-04-23T11:00:00Z", "rotated local device",)
            .expect("revoked signed profile should re-sign"));
        reparsed
            .verify()
            .expect("revoked signed profile should still verify");
        assert!(reparsed.is_revoked());
        assert_eq!(reparsed.revoked_at.as_deref(), Some("2026-04-23T11:00:00Z"));
        assert_eq!(
            reparsed.revocation_reason.as_deref(),
            Some("rotated local device")
        );
        assert!(!reparsed
            .revoke(&key_pair, "2026-04-23T12:00:00Z", "duplicate revoke",)
            .expect("duplicate revoke should stay idempotent"));
    }

    #[test]
    fn signed_profile_valid_fixture_round_trips_and_verifies() {
        let fixture = serde_json::from_str::<Value>(SIGNED_PROFILE_VALID_FIXTURE)
            .expect("signed profile fixture should parse");
        let signed_profile = serde_json::from_value::<SignedProfileDocument>(fixture.clone())
            .expect("signed profile fixture should deserialize");

        signed_profile
            .verify()
            .expect("signed profile fixture should verify");
        assert_eq!(
            signed_profile.profile_id().as_str(),
            test_key_pair().profile_id().as_str()
        );
        assert_eq!(signed_profile.profile().identity.name, "Wave Pilot");

        let serialized = serde_json::to_value(&signed_profile)
            .expect("signed profile fixture should convert to JSON");
        let serialized_profile = serde_json::to_value(signed_profile.profile())
            .expect("embedded profile should convert to JSON");
        let embedded_profile = fixture
            .get("profile")
            .expect("signed profile fixture should contain an embedded profile");

        assert_eq!(serialized, fixture);
        assert_eq!(serialized_profile, *embedded_profile);
        assert_matches_profile_v0_contract(&serialized_profile);
    }

    #[test]
    fn revoked_signed_profile_keeps_embedded_profile_within_profile_v0_contract() {
        let key_pair = test_key_pair();
        let mut signed_profile = signed_test_profile();

        assert!(signed_profile
            .revoke(&key_pair, "2026-04-23T11:00:00Z", "rotated local device",)
            .expect("revoked signed profile should re-sign"));
        signed_profile
            .verify()
            .expect("revoked signed profile should still verify");

        let serialized_profile = serde_json::to_value(signed_profile.profile())
            .expect("revoked signed profile should expose the embedded profile as JSON");

        assert_matches_profile_v0_contract(&serialized_profile);
    }

    #[test]
    fn signed_profile_document_rejects_tampering() {
        let mut signed_profile = signed_test_profile();
        "Tampered Pilot".clone_into(&mut signed_profile.profile.identity.name);

        assert_eq!(
            signed_profile.verify(),
            Err(ProfileSignatureError::SignatureMismatch)
        );
    }

    #[test]
    fn signed_profile_document_rejects_mismatched_resigning_key() {
        let mut signed_profile = signed_test_profile();
        let other_key_pair =
            KeyPair::from_secret_key_hex("backup-device", &encode_hex(&[0x55; 32]))
                .expect("other key pair should decode");

        assert_eq!(
            signed_profile.revoke(
                &other_key_pair,
                "2026-04-23T11:00:00Z",
                "rotated local device",
            ),
            Err(ProfileSignatureError::SignerPublicKeyMismatch)
        );
    }

    #[test]
    fn init_profile_creates_new_profile_document_in_store() {
        let store = FileSystemProfileStore;
        let path = unique_temp_profile_path("init");

        let profile = init_profile(&store, &path, "Wave Pilot", "2026-04-23T10:00:00Z")
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

        init_profile(&store, &path, "Wave Pilot", "2026-04-23T10:00:00Z")
            .expect("initial profile should create");

        let error = init_profile(&store, &path, "Wave Pilot", "2026-04-23T10:01:00Z")
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

    #[test]
    fn reload_boundary_load_local_profile_round_trips_valid_persisted_state() {
        let store = FileSystemProfileStore;
        let path = unique_temp_profile_path("local-state");
        let profile = ProfileDocument::fresh("Wave Pilot", "2026-04-23T10:00:00Z")
            .expect("fresh profile should build");
        let key_pair = test_key_pair();
        let signed_grant = key_pair
            .sign_grant(&CapabilityGrant::new(key_pair.profile_id(), "consent.read"))
            .expect("grant should sign");

        store
            .create_local_profile(&path, &profile, &key_pair)
            .expect("local profile should persist");
        store
            .save_signed_grants(&path, std::slice::from_ref(&signed_grant))
            .expect("signed grants should persist");

        let loaded = store
            .load_local_profile(&path)
            .expect("local profile state should load");

        assert_eq!(loaded.profile, profile);
        assert_eq!(loaded.key_pair.device_label(), key_pair.device_label());
        assert_eq!(loaded.key_pair.public_key_hex(), key_pair.public_key_hex());
        assert_eq!(loaded.key_pair.secret_key_hex(), key_pair.secret_key_hex());
        assert_eq!(loaded.signed_grants, vec![signed_grant]);

        cleanup_local_profile_artifacts(&path);
    }

    #[test]
    fn reload_boundary_load_local_profile_defaults_missing_signed_grants_to_empty() {
        let store = FileSystemProfileStore;
        let path = unique_temp_profile_path("local-state-missing-grants");
        let profile = ProfileDocument::fresh("Wave Pilot", "2026-04-23T10:00:00Z")
            .expect("fresh profile should build");
        let key_pair = test_key_pair();

        store
            .create_local_profile(&path, &profile, &key_pair)
            .expect("local profile should persist");

        let loaded = store
            .load_local_profile(&path)
            .expect("missing signed grants should default to an empty list");

        assert_eq!(loaded.profile, profile);
        assert_eq!(loaded.key_pair.public_key_hex(), key_pair.public_key_hex());
        assert!(loaded.signed_grants.is_empty());

        cleanup_local_profile_artifacts(&path);
    }

    #[test]
    fn reload_boundary_load_local_profile_rejects_grants_for_a_different_local_signer() {
        let store = FileSystemProfileStore;
        let path = unique_temp_profile_path("local-state-invalid-signer");
        let profile = ProfileDocument::fresh("Wave Pilot", "2026-04-23T10:00:00Z")
            .expect("fresh profile should build");
        let key_pair = test_key_pair();
        let foreign_key_pair =
            KeyPair::from_secret_key_hex("backup-device", &encode_hex(&[0x55; 32]))
                .expect("backup key pair should decode");
        let foreign_signed_grant = foreign_key_pair
            .sign_grant(&CapabilityGrant::new(
                foreign_key_pair.profile_id(),
                "consent.read",
            ))
            .expect("foreign grant should sign");

        store
            .create_local_profile(&path, &profile, &key_pair)
            .expect("local profile should persist");
        store
            .save_signed_grants(&path, std::slice::from_ref(&foreign_signed_grant))
            .expect("foreign signed grant should persist");

        let error = store
            .load_local_profile(&path)
            .expect_err("foreign grants should be rejected during reload");

        assert!(matches!(
            error,
            ProfileStoreError::InvalidLocalState(message)
                if message.contains("grant consent.read")
                    && message.contains("does not match local key pair")
        ));

        cleanup_local_profile_artifacts(&path);
    }

    #[test]
    fn file_system_profile_store_exports_and_imports_local_bundle() {
        let store = FileSystemProfileStore;
        let source_path = unique_temp_profile_path("bundle-source");
        let bundle_path = unique_temp_profile_path("bundle").with_extension("bundle.json");
        let imported_path = unique_temp_profile_path("bundle-imported");

        let profile = ProfileDocument::fresh("Wave Pilot", "2026-04-23T10:00:00Z")
            .expect("fresh profile should build");
        let key_pair = test_key_pair();
        let signed_grant = key_pair
            .sign_grant(&CapabilityGrant::new(key_pair.profile_id(), "consent.read"))
            .expect("grant should sign");

        store
            .create_local_profile(&source_path, &profile, &key_pair)
            .expect("source state should persist");
        store
            .save_signed_grants(&source_path, std::slice::from_ref(&signed_grant))
            .expect("source grants should persist");
        store
            .export_profile_bundle(&source_path, &bundle_path)
            .expect("bundle should export");

        let imported = store
            .import_profile_bundle(&bundle_path, &imported_path)
            .expect("bundle should import");

        assert_eq!(imported.profile, profile);
        assert_eq!(
            imported.key_pair.public_key_hex(),
            key_pair.public_key_hex()
        );
        assert_eq!(
            imported.key_pair.secret_key_hex(),
            key_pair.secret_key_hex()
        );
        assert_eq!(imported.signed_grants, vec![signed_grant]);

        let _ = std::fs::remove_file(&source_path);
        let _ = std::fs::remove_file(FileSystemProfileStore::key_pair_path(&source_path));
        let _ = std::fs::remove_file(FileSystemProfileStore::signed_grants_path(&source_path));
        let _ = std::fs::remove_file(&bundle_path);
        let _ = std::fs::remove_file(&imported_path);
        let _ = std::fs::remove_file(FileSystemProfileStore::key_pair_path(&imported_path));
        let _ = std::fs::remove_file(FileSystemProfileStore::signed_grants_path(&imported_path));
        if let Some(parent) = source_path.parent() {
            let _ = std::fs::remove_dir(parent);
        }
    }

    #[test]
    fn file_system_profile_store_import_rolls_back_partial_state_when_bundle_grants_are_invalid() {
        let store = FileSystemProfileStore;
        let source_path = unique_temp_profile_path("bundle-source-invalid-grant");
        let bundle_path =
            unique_temp_profile_path("bundle-invalid-grant").with_extension("bundle.json");
        let imported_path = unique_temp_profile_path("bundle-imported-invalid-grant");

        let profile = ProfileDocument::fresh("Wave Pilot", "2026-04-23T10:00:00Z")
            .expect("fresh profile should build");
        let key_pair = test_key_pair();
        let signed_grant = key_pair
            .sign_grant(&CapabilityGrant::new(key_pair.profile_id(), "consent.read"))
            .expect("grant should sign");

        store
            .create_local_profile(&source_path, &profile, &key_pair)
            .expect("source state should persist");
        store
            .save_signed_grants(&source_path, std::slice::from_ref(&signed_grant))
            .expect("source grants should persist");
        store
            .export_profile_bundle(&source_path, &bundle_path)
            .expect("bundle should export");

        let mut bundle_json: Value = serde_json::from_str(
            &std::fs::read_to_string(&bundle_path).expect("bundle should be readable"),
        )
        .expect("bundle JSON should parse");
        bundle_json["grants"][0]["signature"] = Value::String("00".repeat(64));
        std::fs::write(
            &bundle_path,
            serde_json::to_string_pretty(&bundle_json).expect("bundle JSON should serialize"),
        )
        .expect("mutated bundle should persist");

        let error = store
            .import_profile_bundle(&bundle_path, &imported_path)
            .expect_err("invalid bundle grants should be rejected");

        assert!(matches!(
            error,
            ProfileStoreError::CapabilityGrantSignature(
                CapabilityGrantSignatureError::SignatureMismatch
            )
        ));
        assert!(!imported_path.exists());
        assert!(!FileSystemProfileStore::key_pair_path(&imported_path).exists());
        assert!(!FileSystemProfileStore::signed_grants_path(&imported_path).exists());

        cleanup_local_profile_artifacts(&source_path);
        let _ = std::fs::remove_file(&bundle_path);
        cleanup_local_profile_artifacts(&imported_path);
        if let Some(parent) = source_path.parent() {
            let _ = std::fs::remove_dir(parent);
        }
    }
}
