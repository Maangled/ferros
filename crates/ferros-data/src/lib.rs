#![forbid(unsafe_code)]

pub const ADR_REFERENCE: &str = "ADR-020";
pub const MIGRATION_AUTHORITY: &str = "sql-migrations";
pub const BASELINE_MIGRATION_PATH: &str = "migrations/0001_revision_base.sql";
pub const BASELINE_MIGRATION_SQL: &str = include_str!("../migrations/0001_revision_base.sql");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthoritySource {
    SqlMigrations,
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

#[must_use]
pub const fn ferros_data_boundary() -> DataBoundary {
    DataBoundary::migration_first()
}

#[cfg(test)]
mod tests {
    use super::{
        ferros_data_boundary, ADR_REFERENCE, BASELINE_MIGRATION_PATH, BASELINE_MIGRATION_SQL,
        MIGRATION_AUTHORITY,
    };

    #[test]
    fn metadata_stays_aligned_with_adr_020() {
        assert_eq!(ADR_REFERENCE, "ADR-020");
        assert_eq!(MIGRATION_AUTHORITY, "sql-migrations");
        assert_eq!(BASELINE_MIGRATION_PATH, "migrations/0001_revision_base.sql");
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
        assert!(BASELINE_MIGRATION_SQL.contains("check (parent_card_id is not null or parent_deck_id is not null)"));
    }
}
