#![forbid(unsafe_code)]

use serde::Serialize;
use std::fs;
use std::path::{Component, Path};

pub const ADR_REFERENCE: &str = "ADR-020";
pub const MIGRATION_AUTHORITY: &str = "sql-migrations";
pub const BASELINE_MIGRATION_PATH: &str = "migrations/0001_revision_base.sql";
pub const BASELINE_MIGRATION_SQL: &str = include_str!("../migrations/0001_revision_base.sql");
pub const ORDERED_CHILD_SINGLE_PARENT_SCOPE_MIGRATION_PATH: &str =
    "migrations/0002_ordered_child_single_parent_scope.sql";
pub const MIGRATION_PATHS: [&str; 2] = [
    BASELINE_MIGRATION_PATH,
    ORDERED_CHILD_SINGLE_PARENT_SCOPE_MIGRATION_PATH,
];
pub const LOCAL_PUSH_AUDIT_ENVELOPE_SCHEMA_PATH: &str =
    "schemas/local-push-audit-envelope.schema.json";
pub const LOCAL_PUSH_AUDIT_ENVELOPE_SCHEMA_ID: &str =
    "https://ferros.local/schemas/local-push-audit-envelope.schema.json";
pub const LOCAL_PUSH_AUDIT_ENVELOPE_VERSION: &str = "1.0";
pub const LOCAL_PUSH_DIGEST_ROOT: &str = ".tmp/push/";
pub const BURST_LOCAL_PUSH_ENVELOPE_PATH: &str = ".tmp/push/burst-local-push-envelope.json";
pub const LOCAL_ONRAMP_PROPOSAL_ARTIFACT_ROOT: &str = ".tmp/hub/";
pub const LOCAL_ONRAMP_PROPOSAL_ARTIFACT_PATH: &str = ".tmp/hub/local-onramp-proposal.json";
pub const LOCAL_ONRAMP_PROPOSAL_SCOPE: &str = "local-only";
pub const LOCAL_ONRAMP_PROPOSAL_EVIDENCE: &str = "non-evidentiary";

#[cfg(test)]
const ORDERED_CHILD_SINGLE_PARENT_SCOPE_MIGRATION_SQL: &str =
    include_str!("../migrations/0002_ordered_child_single_parent_scope.sql");
#[cfg(test)]
const LOCAL_PUSH_AUDIT_ENVELOPE_SCHEMA: &str =
    include_str!("../../../schemas/local-push-audit-envelope.schema.json");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthoritySource {
    SqlMigrations,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum LocalEnvelopeAuthority {
    LocalOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConsentBoundary {
    ExplicitOperatorConsent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum LocalEnvelopeKind {
    LocalPush,
    LocalAudit,
}

impl LocalEnvelopeKind {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalPush => "local-push",
            Self::LocalAudit => "local-audit",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum LocalPushSurface {
    PushDigest,
    RunwayObservation,
    AuditTrace,
}

impl LocalPushSurface {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PushDigest => "push-digest",
            Self::RunwayObservation => "runway-observation",
            Self::AuditTrace => "audit-trace",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum LocalArtifactRole {
    Anchor,
    Input,
    Output,
    Evidence,
}

impl LocalArtifactRole {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Anchor => "anchor",
            Self::Input => "input",
            Self::Output => "output",
            Self::Evidence => "evidence",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum LocalDigestAlgorithm {
    Sha256,
    Blake3,
    Other,
}

impl LocalDigestAlgorithm {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Sha256 => "sha256",
            Self::Blake3 => "blake3",
            Self::Other => "other",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalPushScope {
    pub batch_id: Option<String>,
    pub wave_id: Option<String>,
    pub lane_id: Option<String>,
    pub stream: String,
    pub surface: LocalPushSurface,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalPushArtifact {
    pub path: String,
    pub role: LocalArtifactRole,
    pub digest_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalPushDigest {
    pub label: String,
    pub algorithm: LocalDigestAlgorithm,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalPushObservation {
    pub target: String,
    pub status: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct LocalPushAuditAuthority {
    pub mode: LocalEnvelopeAuthority,
    pub consent: ConsentBoundary,
}

impl LocalPushAuditAuthority {
    #[must_use]
    pub const fn doctrine_safe() -> Self {
        Self {
            mode: LocalEnvelopeAuthority::LocalOnly,
            consent: ConsentBoundary::ExplicitOperatorConsent,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalPushAuditEnvelope {
    pub envelope_version: &'static str,
    pub envelope_type: LocalEnvelopeKind,
    pub created_at: String,
    pub authority: LocalPushAuditAuthority,
    pub scope: LocalPushScope,
    pub artifacts: Vec<LocalPushArtifact>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub digests: Vec<LocalPushDigest>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub observations: Vec<LocalPushObservation>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocalPushAuditEnvelopeError {
    EmptyArtifacts,
    InvalidRelativePath(String),
    InvalidStream(String),
    EmptyReason,
}

#[derive(Debug)]
pub enum LocalPushAuditEnvelopeWriteError {
    InvalidEnvelope(LocalPushAuditEnvelopeError),
    Serialize(serde_json::Error),
    Io(std::io::Error),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum LocalOnrampQuarantineStatus {
    QuarantinedPendingConsent,
}

impl LocalOnrampQuarantineStatus {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QuarantinedPendingConsent => "quarantined-pending-consent",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalOnrampProposal {
    pub source: String,
    pub proposal_id: String,
    pub bridge_agent_name: String,
    pub stand_in_entity_name: String,
    pub requested_capability: String,
    pub requested_action: String,
    pub quarantine_status: LocalOnrampQuarantineStatus,
    pub scope: String,
    pub evidence: String,
    pub local_artifact_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocalOnrampProposalError {
    EmptyField(&'static str),
    InvalidRelativePath(String),
    InvalidScope(String),
    InvalidEvidence(String),
    InvalidTextField {
        field: &'static str,
        value: String,
        reason: &'static str,
    },
}

#[derive(Debug)]
pub enum LocalOnrampProposalWriteError {
    InvalidProposal(LocalOnrampProposalError),
    Serialize(serde_json::Error),
    Io(std::io::Error),
}

impl LocalPushAuditEnvelope {
    pub fn new(
        envelope_type: LocalEnvelopeKind,
        created_at: impl Into<String>,
        scope: LocalPushScope,
        artifacts: Vec<LocalPushArtifact>,
    ) -> Result<Self, LocalPushAuditEnvelopeError> {
        let envelope = Self {
            envelope_version: LOCAL_PUSH_AUDIT_ENVELOPE_VERSION,
            envelope_type,
            created_at: created_at.into(),
            authority: LocalPushAuditAuthority::doctrine_safe(),
            scope,
            artifacts,
            digests: Vec::new(),
            observations: Vec::new(),
            notes: Vec::new(),
        };

        envelope.validate()?;
        Ok(envelope)
    }

    pub fn validate(&self) -> Result<(), LocalPushAuditEnvelopeError> {
        if self.artifacts.is_empty() {
            return Err(LocalPushAuditEnvelopeError::EmptyArtifacts);
        }

        if self.scope.reason.trim().is_empty() {
            return Err(LocalPushAuditEnvelopeError::EmptyReason);
        }

        match self.scope.stream.as_str() {
            "S1" | "S2" | "S3" | "S4" | "S5" | "S6" | "S7" | "S8" => {}
            other => {
                return Err(LocalPushAuditEnvelopeError::InvalidStream(
                    other.to_owned(),
                ))
            }
        }

        for artifact in &self.artifacts {
            if artifact.path.starts_with('/')
                || artifact.path.contains(':')
                || artifact.path.trim().is_empty()
            {
                return Err(LocalPushAuditEnvelopeError::InvalidRelativePath(
                    artifact.path.clone(),
                ));
            }
        }

        Ok(())
    }

    #[must_use]
    pub fn with_digest(mut self, digest: LocalPushDigest) -> Self {
        self.digests.push(digest);
        self
    }

    #[must_use]
    pub fn with_observation(mut self, observation: LocalPushObservation) -> Self {
        self.observations.push(observation);
        self
    }

    #[must_use]
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    pub fn to_pretty_json(&self) -> Result<String, LocalPushAuditEnvelopeWriteError> {
        self.validate()
            .map_err(LocalPushAuditEnvelopeWriteError::InvalidEnvelope)?;

        serde_json::to_string_pretty(self).map_err(LocalPushAuditEnvelopeWriteError::Serialize)
    }

    pub fn write_json(&self, path: impl AsRef<Path>) -> Result<(), LocalPushAuditEnvelopeWriteError> {
        let path = path.as_ref();
        let json = self.to_pretty_json()?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(LocalPushAuditEnvelopeWriteError::Io)?;
        }

        fs::write(path, format!("{json}\n")).map_err(LocalPushAuditEnvelopeWriteError::Io)
    }
}

impl LocalOnrampProposal {
    pub fn new(
        source: impl Into<String>,
        proposal_id: impl Into<String>,
        bridge_agent_name: impl Into<String>,
        stand_in_entity_name: impl Into<String>,
        requested_capability: impl Into<String>,
        requested_action: impl Into<String>,
        local_artifact_path: impl Into<String>,
    ) -> Result<Self, LocalOnrampProposalError> {
        let proposal = Self {
            source: source.into(),
            proposal_id: proposal_id.into(),
            bridge_agent_name: bridge_agent_name.into(),
            stand_in_entity_name: stand_in_entity_name.into(),
            requested_capability: requested_capability.into(),
            requested_action: requested_action.into(),
            quarantine_status: LocalOnrampQuarantineStatus::QuarantinedPendingConsent,
            scope: LOCAL_ONRAMP_PROPOSAL_SCOPE.to_owned(),
            evidence: LOCAL_ONRAMP_PROPOSAL_EVIDENCE.to_owned(),
            local_artifact_path: local_artifact_path.into(),
        };

        proposal.validate()?;
        Ok(proposal)
    }

    pub fn validate(&self) -> Result<(), LocalOnrampProposalError> {
        validate_onramp_text_field("source", &self.source)?;
        validate_onramp_text_field("proposalId", &self.proposal_id)?;
        validate_onramp_text_field("bridgeAgentName", &self.bridge_agent_name)?;
        validate_onramp_text_field("standInEntityName", &self.stand_in_entity_name)?;
        validate_onramp_text_field("requestedCapability", &self.requested_capability)?;
        validate_onramp_text_field("requestedAction", &self.requested_action)?;

        if self.scope != LOCAL_ONRAMP_PROPOSAL_SCOPE {
            return Err(LocalOnrampProposalError::InvalidScope(self.scope.clone()));
        }

        if self.evidence != LOCAL_ONRAMP_PROPOSAL_EVIDENCE {
            return Err(LocalOnrampProposalError::InvalidEvidence(self.evidence.clone()));
        }

        validate_local_onramp_artifact_path(&self.local_artifact_path)?;

        Ok(())
    }

    pub fn to_pretty_json(&self) -> Result<String, LocalOnrampProposalWriteError> {
        self.validate()
            .map_err(LocalOnrampProposalWriteError::InvalidProposal)?;

        serde_json::to_string_pretty(self).map_err(LocalOnrampProposalWriteError::Serialize)
    }

    pub fn write_json(&self, path: impl AsRef<Path>) -> Result<(), LocalOnrampProposalWriteError> {
        let path = path.as_ref();
        let json = self.to_pretty_json()?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(LocalOnrampProposalWriteError::Io)?;
        }

        fs::write(path, format!("{json}\n")).map_err(LocalOnrampProposalWriteError::Io)
    }
}

fn validate_onramp_text_field(
    field: &'static str,
    value: &str,
) -> Result<(), LocalOnrampProposalError> {
    if value.trim().is_empty() {
        return Err(LocalOnrampProposalError::EmptyField(field));
    }

    let lowered = value.to_ascii_lowercase();
    if looks_remote_like_url(&lowered) {
        return Err(LocalOnrampProposalError::InvalidTextField {
            field,
            value: value.to_owned(),
            reason: "must not contain remote-looking URLs",
        });
    }

    if let Some(wording) = banned_onramp_wording(&lowered) {
        return Err(LocalOnrampProposalError::InvalidTextField {
            field,
            value: value.to_owned(),
            reason: wording,
        });
    }

    Ok(())
}

fn validate_local_onramp_artifact_path(path: &str) -> Result<(), LocalOnrampProposalError> {
    if !path.starts_with(LOCAL_ONRAMP_PROPOSAL_ARTIFACT_ROOT)
        || !path.ends_with(".json")
        || Path::new(path).is_absolute()
        || path.contains(':')
        || path.trim().is_empty()
    {
        return Err(LocalOnrampProposalError::InvalidRelativePath(path.to_owned()));
    }

    for component in Path::new(path).components() {
        match component {
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return Err(LocalOnrampProposalError::InvalidRelativePath(path.to_owned()));
            }
            Component::CurDir | Component::Normal(_) => {}
        }
    }

    let lowered = path.to_ascii_lowercase();
    if looks_remote_like_url(&lowered) || banned_onramp_wording(&lowered).is_some() {
        return Err(LocalOnrampProposalError::InvalidRelativePath(path.to_owned()));
    }

    Ok(())
}

fn looks_remote_like_url(value: &str) -> bool {
    value.contains("://")
        || value.starts_with("//")
        || value.contains("http://")
        || value.contains("https://")
        || value.contains("ws://")
        || value.contains("wss://")
        || value.contains("ftp://")
        || value.contains("file://")
}

fn banned_onramp_wording(value: &str) -> Option<&'static str> {
    [
        "hardware",
        "proof",
        "launch",
        "accepted",
        "canonical",
        "granted",
    ]
    .into_iter()
    .find(|wording| value.contains(wording))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RevisionBaseFields {
    pub lineage: &'static str,
    pub identity: &'static str,
    pub created_at: &'static str,
    pub updated_at: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DataBoundary {
    pub authority: AuthoritySource,
    pub snapshots_use_jsonb: bool,
    pub application_prevalidation_required: bool,
    pub revision_base: RevisionBaseFields,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalPushAuditEnvelopeBoundary {
    pub authority: LocalEnvelopeAuthority,
    pub consent_boundary: ConsentBoundary,
    pub supports_digests: bool,
    pub supports_runway_observation: bool,
    pub kinds: [LocalEnvelopeKind; 2],
}

impl DataBoundary {
    #[must_use]
    pub const fn migration_first() -> Self {
        Self {
            authority: AuthoritySource::SqlMigrations,
            snapshots_use_jsonb: true,
            application_prevalidation_required: true,
            revision_base: RevisionBaseFields {
                lineage: "lineage_id",
                identity: "entity_id",
                created_at: "created_at",
                updated_at: "updated_at",
            },
        }
    }

    #[must_use]
    pub const fn migrations_are_authoritative(self) -> bool {
        matches!(self.authority, AuthoritySource::SqlMigrations)
    }
}

impl LocalPushAuditEnvelopeBoundary {
    #[must_use]
    pub const fn doctrine_safe() -> Self {
        Self {
            authority: LocalEnvelopeAuthority::LocalOnly,
            consent_boundary: ConsentBoundary::ExplicitOperatorConsent,
            supports_digests: true,
            supports_runway_observation: true,
            kinds: [LocalEnvelopeKind::LocalPush, LocalEnvelopeKind::LocalAudit],
        }
    }
}

#[must_use]
pub const fn ferros_data_boundary() -> DataBoundary {
    DataBoundary::migration_first()
}

#[must_use]
pub const fn local_push_audit_boundary() -> LocalPushAuditEnvelopeBoundary {
    LocalPushAuditEnvelopeBoundary::doctrine_safe()
}

#[cfg(test)]
mod tests {
    use super::{
        ferros_data_boundary, local_push_audit_boundary, BURST_LOCAL_PUSH_ENVELOPE_PATH,
        LocalArtifactRole, LocalDigestAlgorithm, LocalEnvelopeAuthority, LocalEnvelopeKind,
        LocalOnrampProposal, LocalOnrampProposalError, LocalOnrampQuarantineStatus,
        LocalPushArtifact, LocalPushAuditEnvelope, LocalPushAuditEnvelopeError,
        LocalPushDigest, LocalPushObservation, LocalPushScope, LocalPushSurface,
        ADR_REFERENCE, BASELINE_MIGRATION_PATH, BASELINE_MIGRATION_SQL,
        LOCAL_ONRAMP_PROPOSAL_ARTIFACT_PATH, LOCAL_ONRAMP_PROPOSAL_EVIDENCE,
        LOCAL_ONRAMP_PROPOSAL_SCOPE,
        LOCAL_PUSH_AUDIT_ENVELOPE_SCHEMA, LOCAL_PUSH_AUDIT_ENVELOPE_SCHEMA_ID,
        LOCAL_PUSH_AUDIT_ENVELOPE_SCHEMA_PATH, LOCAL_PUSH_AUDIT_ENVELOPE_VERSION,
        LOCAL_PUSH_DIGEST_ROOT, MIGRATION_AUTHORITY, MIGRATION_PATHS,
        ORDERED_CHILD_SINGLE_PARENT_SCOPE_MIGRATION_PATH,
        ORDERED_CHILD_SINGLE_PARENT_SCOPE_MIGRATION_SQL,
    };

    fn normalized_sql(sql: &str) -> String {
        sql.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    #[test]
    fn metadata_stays_aligned_with_adr_020() {
        assert_eq!(ADR_REFERENCE, "ADR-020");
        assert_eq!(MIGRATION_AUTHORITY, "sql-migrations");
        assert_eq!(BASELINE_MIGRATION_PATH, "migrations/0001_revision_base.sql");
        assert_eq!(
            ORDERED_CHILD_SINGLE_PARENT_SCOPE_MIGRATION_PATH,
            "migrations/0002_ordered_child_single_parent_scope.sql"
        );
    }

    #[test]
    fn migration_manifest_preserves_the_expected_order() {
        assert_eq!(
            MIGRATION_PATHS,
            [
                "migrations/0001_revision_base.sql",
                "migrations/0002_ordered_child_single_parent_scope.sql",
            ]
        );
    }

    #[test]
    fn boundary_requires_sql_authority_jsonb_snapshots_and_prevalidation() {
        let boundary = ferros_data_boundary();

        assert!(boundary.migrations_are_authoritative());
        assert!(boundary.snapshots_use_jsonb);
        assert!(boundary.application_prevalidation_required);
        assert_eq!(boundary.revision_base.lineage, "lineage_id");
        assert_eq!(boundary.revision_base.identity, "entity_id");
    }

    #[test]
    fn baseline_migration_proves_revision_base_and_database_invariants() {
        assert!(BASELINE_MIGRATION_SQL.contains("create table if not exists revision_base"));
        assert!(BASELINE_MIGRATION_SQL.contains("snapshot jsonb not null"));
        assert!(BASELINE_MIGRATION_SQL.contains("check (jsonb_typeof(snapshot) = 'object')"));
        assert!(BASELINE_MIGRATION_SQL
            .contains("check (parent_card_id is not null or parent_deck_id is not null)"));
    }

    #[test]
    fn ordered_child_parent_scope_tightening_yields_exactly_one_parent() {
        let baseline_sql = normalized_sql(BASELINE_MIGRATION_SQL);
        let tightening_sql = normalized_sql(ORDERED_CHILD_SINGLE_PARENT_SCOPE_MIGRATION_SQL);
        let combined_sql = format!("{baseline_sql} {tightening_sql}");

        assert!(baseline_sql
            .contains("check (parent_card_id is not null or parent_deck_id is not null)"));
        assert!(tightening_sql.contains(
            "alter table ordered_child add constraint ordered_child_single_parent_scope"
        ));
        assert!(tightening_sql.contains(
            "check ( not ( parent_card_id is not null and parent_deck_id is not null ) )"
        ));
        assert!(combined_sql
            .contains("check (parent_card_id is not null or parent_deck_id is not null)"));
        assert!(combined_sql.contains(
            "check ( not ( parent_card_id is not null and parent_deck_id is not null ) )"
        ));
    }

    #[test]
    fn local_push_audit_boundary_stays_local_and_consent_first() {
        let boundary = local_push_audit_boundary();

        assert!(boundary.supports_digests);
        assert!(boundary.supports_runway_observation);
        assert_eq!(boundary.kinds.len(), 2);
    }

    #[test]
    fn local_push_audit_schema_metadata_stays_aligned() {
        assert_eq!(
            LOCAL_PUSH_AUDIT_ENVELOPE_SCHEMA_PATH,
            "schemas/local-push-audit-envelope.schema.json"
        );
        assert_eq!(
            LOCAL_PUSH_AUDIT_ENVELOPE_SCHEMA_ID,
            "https://ferros.local/schemas/local-push-audit-envelope.schema.json"
        );
        assert_eq!(LOCAL_PUSH_AUDIT_ENVELOPE_VERSION, "1.0");
        assert_eq!(LOCAL_PUSH_DIGEST_ROOT, ".tmp/push/");
        assert_eq!(BURST_LOCAL_PUSH_ENVELOPE_PATH, ".tmp/push/burst-local-push-envelope.json");
        assert!(LOCAL_PUSH_AUDIT_ENVELOPE_SCHEMA.contains(LOCAL_PUSH_AUDIT_ENVELOPE_SCHEMA_ID));
        assert!(LOCAL_PUSH_AUDIT_ENVELOPE_SCHEMA.contains("local-only"));
        assert!(LOCAL_PUSH_AUDIT_ENVELOPE_SCHEMA.contains("explicit-operator-consent"));
    }

    #[test]
    fn local_push_audit_envelope_accepts_local_scope_and_relative_artifacts() {
        let envelope = LocalPushAuditEnvelope::new(
            LocalEnvelopeKind::LocalPush,
            "2026-04-28T12:00:00Z",
            LocalPushScope {
                batch_id: Some("BATCH-2026-04-28-G".to_owned()),
                wave_id: Some("WAVE-2026-04-28-20".to_owned()),
                lane_id: Some("L5".to_owned()),
                stream: "S6".to_owned(),
                surface: LocalPushSurface::PushDigest,
                reason: "queue-clear local digest emission".to_owned(),
            },
            vec![LocalPushArtifact {
                path: ".tmp/push/batch-1-digest.md".to_owned(),
                role: LocalArtifactRole::Output,
                digest_ref: Some("batch-1".to_owned()),
            }],
        )
        .expect("local envelope should validate")
        .with_digest(LocalPushDigest {
            label: "batch-1".to_owned(),
            algorithm: LocalDigestAlgorithm::Sha256,
            value: "deadbeef".to_owned(),
        })
        .with_observation(LocalPushObservation {
            target: "runway-summary".to_owned(),
            status: "planned",
            summary: Some("consumer pending".to_owned()),
        })
        .with_note("local-only artifact");

        assert_eq!(envelope.envelope_version, "1.0");
        assert_eq!(envelope.envelope_type.as_str(), "local-push");
        assert_eq!(envelope.authority.mode, LocalEnvelopeAuthority::LocalOnly);
        assert_eq!(envelope.scope.surface.as_str(), "push-digest");
        assert_eq!(envelope.artifacts[0].role.as_str(), "output");
        assert_eq!(envelope.digests[0].algorithm.as_str(), "sha256");
    }

    #[test]
    fn local_push_audit_envelope_serializes_with_schema_field_names() {
        let envelope = LocalPushAuditEnvelope::new(
            LocalEnvelopeKind::LocalPush,
            "2026-04-28T12:00:00Z",
            LocalPushScope {
                batch_id: Some("BATCH-2026-04-28-G".to_owned()),
                wave_id: Some("WAVE-2026-04-28-32".to_owned()),
                lane_id: Some("burst".to_owned()),
                stream: "S6".to_owned(),
                surface: LocalPushSurface::PushDigest,
                reason: "typed local envelope emission".to_owned(),
            },
            vec![LocalPushArtifact {
                path: BURST_LOCAL_PUSH_ENVELOPE_PATH.to_owned(),
                role: LocalArtifactRole::Output,
                digest_ref: None,
            }],
        )
        .expect("local envelope should validate")
        .with_observation(LocalPushObservation {
            target: "burst-helper".to_owned(),
            status: "observed",
            summary: Some("typed local envelope emitted".to_owned()),
        });

        let payload = serde_json::to_value(&envelope).expect("envelope should serialize");

        assert_eq!(payload["envelopeVersion"], "1.0");
        assert_eq!(payload["envelopeType"], "local-push");
        assert_eq!(payload["authority"]["mode"], "local-only");
        assert_eq!(payload["authority"]["consent"], "explicit-operator-consent");
        assert_eq!(payload["scope"]["waveId"], "WAVE-2026-04-28-32");
        assert_eq!(payload["artifacts"][0]["path"], BURST_LOCAL_PUSH_ENVELOPE_PATH);
        assert_eq!(payload["observations"][0]["status"], "observed");
    }

    #[test]
    fn local_push_audit_envelope_write_json_creates_output_file() {
        let output_path = std::env::temp_dir().join(format!(
            "ferros-local-push-envelope-{}.json",
            std::process::id()
        ));
        let envelope = LocalPushAuditEnvelope::new(
            LocalEnvelopeKind::LocalPush,
            "2026-04-28T12:00:00Z",
            LocalPushScope {
                batch_id: None,
                wave_id: Some("WAVE-2026-04-28-32".to_owned()),
                lane_id: Some("test".to_owned()),
                stream: "S6".to_owned(),
                surface: LocalPushSurface::PushDigest,
                reason: "write local test artifact".to_owned(),
            },
            vec![LocalPushArtifact {
                path: ".tmp/push/test-envelope.json".to_owned(),
                role: LocalArtifactRole::Output,
                digest_ref: None,
            }],
        )
        .expect("local envelope should validate");

        envelope
            .write_json(&output_path)
            .expect("json output should write");

        let written = std::fs::read_to_string(&output_path).expect("output file should read");
        assert!(written.contains("\"authority\""));
        assert!(written.contains("\"mode\": \"local-only\""));
        assert!(written.contains("\"consent\": \"explicit-operator-consent\""));

        let _ = std::fs::remove_file(output_path);
    }

    #[test]
    fn local_push_audit_envelope_rejects_absolute_paths_and_unknown_streams() {
        let absolute_path = LocalPushAuditEnvelope::new(
            LocalEnvelopeKind::LocalAudit,
            "2026-04-28T12:00:00Z",
            LocalPushScope {
                batch_id: None,
                wave_id: None,
                lane_id: None,
                stream: "S6".to_owned(),
                surface: LocalPushSurface::AuditTrace,
                reason: "absolute path must fail".to_owned(),
            },
            vec![LocalPushArtifact {
                path: "/tmp/push.md".to_owned(),
                role: LocalArtifactRole::Output,
                digest_ref: None,
            }],
        );
        let invalid_stream = LocalPushAuditEnvelope::new(
            LocalEnvelopeKind::LocalPush,
            "2026-04-28T12:00:00Z",
            LocalPushScope {
                batch_id: None,
                wave_id: None,
                lane_id: None,
                stream: "SX".to_owned(),
                surface: LocalPushSurface::PushDigest,
                reason: "invalid stream must fail".to_owned(),
            },
            vec![LocalPushArtifact {
                path: ".tmp/push/batch-1.md".to_owned(),
                role: LocalArtifactRole::Output,
                digest_ref: None,
            }],
        );

        assert_eq!(
            absolute_path,
            Err(LocalPushAuditEnvelopeError::InvalidRelativePath(
                "/tmp/push.md".to_owned()
            ))
        );
        assert_eq!(
            invalid_stream,
            Err(LocalPushAuditEnvelopeError::InvalidStream("SX".to_owned()))
        );
    }

    #[test]
    fn onramp_proposal_accepts_quarantined_local_bridge_material() {
        let proposal = LocalOnrampProposal::new(
            "ha-local-bridge",
            "proposal-simulated-bridge-entity",
            "ha-local-bridge",
            "simulated-bridge-entity",
            "bridge.observe",
            "report-state",
            LOCAL_ONRAMP_PROPOSAL_ARTIFACT_PATH,
        )
        .expect("proposal should validate");

        let payload = serde_json::to_value(&proposal).expect("proposal should serialize");

        assert_eq!(proposal.quarantine_status, LocalOnrampQuarantineStatus::QuarantinedPendingConsent);
        assert_eq!(proposal.scope, LOCAL_ONRAMP_PROPOSAL_SCOPE);
        assert_eq!(proposal.evidence, LOCAL_ONRAMP_PROPOSAL_EVIDENCE);
        assert_eq!(payload["quarantineStatus"], "quarantined-pending-consent");
        assert_eq!(payload["localArtifactPath"], LOCAL_ONRAMP_PROPOSAL_ARTIFACT_PATH);
    }

    #[test]
    fn onramp_proposal_write_json_creates_output_file() {
        let output_path = std::env::temp_dir().join(format!(
            "ferros-local-onramp-proposal-{}.json",
            std::process::id()
        ));
        let proposal = LocalOnrampProposal::new(
            "ha-local-bridge",
            "proposal-simulated-bridge-entity",
            "ha-local-bridge",
            "simulated-bridge-entity",
            "bridge.observe",
            "report-state",
            LOCAL_ONRAMP_PROPOSAL_ARTIFACT_PATH,
        )
        .expect("proposal should validate");

        proposal
            .write_json(&output_path)
            .expect("proposal json should write");

        let written = std::fs::read_to_string(&output_path).expect("output file should read");
        assert!(written.contains("\"proposalId\""));
        assert!(written.contains("\"quarantineStatus\": \"quarantined-pending-consent\""));
        assert!(written.contains("\"scope\": \"local-only\""));

        let _ = std::fs::remove_file(output_path);
    }

    #[test]
    fn onramp_proposal_rejects_remote_looking_text() {
        let proposal = LocalOnrampProposal::new(
            "https://ha.local/entity/bridge",
            "proposal-simulated-bridge-entity",
            "ha-local-bridge",
            "simulated-bridge-entity",
            "bridge.observe",
            "report-state",
            LOCAL_ONRAMP_PROPOSAL_ARTIFACT_PATH,
        );

        assert_eq!(
            proposal,
            Err(LocalOnrampProposalError::InvalidTextField {
                field: "source",
                value: "https://ha.local/entity/bridge".to_owned(),
                reason: "must not contain remote-looking URLs",
            })
        );
    }

    #[test]
    fn onramp_proposal_rejects_hardware_proof_launch_wording() {
        let proposal = LocalOnrampProposal::new(
            "ha-local-bridge",
            "proposal-hardware-launch-proof",
            "ha-local-bridge",
            "simulated-bridge-entity",
            "bridge.observe",
            "report-state",
            LOCAL_ONRAMP_PROPOSAL_ARTIFACT_PATH,
        );

        assert_eq!(
            proposal,
            Err(LocalOnrampProposalError::InvalidTextField {
                field: "proposalId",
                value: "proposal-hardware-launch-proof".to_owned(),
                reason: "hardware",
            })
        );
    }

    #[test]
    fn onramp_proposal_rejects_accepted_canonical_granted_wording() {
        let proposal = LocalOnrampProposal::new(
            "ha-local-bridge",
            "proposal-simulated-bridge-entity",
            "ha-local-bridge",
            "accepted-bridge-entity",
            "bridge.observe",
            "report-state",
            LOCAL_ONRAMP_PROPOSAL_ARTIFACT_PATH,
        );

        assert_eq!(
            proposal,
            Err(LocalOnrampProposalError::InvalidTextField {
                field: "standInEntityName",
                value: "accepted-bridge-entity".to_owned(),
                reason: "accepted",
            })
        );
    }

    #[test]
    fn onramp_proposal_rejects_non_local_artifact_paths() {
        let proposal = LocalOnrampProposal::new(
            "ha-local-bridge",
            "proposal-simulated-bridge-entity",
            "ha-local-bridge",
            "simulated-bridge-entity",
            "bridge.observe",
            "report-state",
            ".tmp/hub/../accepted-canonical.json",
        );

        assert_eq!(
            proposal,
            Err(LocalOnrampProposalError::InvalidRelativePath(
                ".tmp/hub/../accepted-canonical.json".to_owned()
            ))
        );
    }
}
