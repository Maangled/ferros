#![forbid(unsafe_code)]

use serde::Serialize;
use std::fs;
use std::path::Path;

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
        LocalPushArtifact, LocalPushAuditEnvelope, LocalPushAuditEnvelopeError,
        LocalPushDigest, LocalPushObservation, LocalPushScope, LocalPushSurface,
        ADR_REFERENCE, BASELINE_MIGRATION_PATH, BASELINE_MIGRATION_SQL,
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
}
