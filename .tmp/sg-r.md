FERROS S6 Audit: sheetgen-rust (Refractov 1.2) — ADR-020 Input
1. Repository Overview
Sheetgen-rust (codename Refractov 1.2) is an AI-first Rust backend for managing architectural drawing sheets. A GPT-4 assistant is the sole UI; it discovers and executes backend commands via POST /command/{name}. The architecture uses: Axum (HTTP), tokio-postgres + deadpool (DB), Redis (cache), Tigris (blob storage), and a custom GPT function-call integration baked around a definitions.yaml contract.

Key layers: gpt_interface/ (schema+OpenAPI), commands/ (CQRS handlers), storage/ (repos + models), domain/ (pure types), migrations/ (DDL), docs/ (logical schema JSONC).

2. Candidate Patterns, Components, and Workflows
2.1 — definitions.yaml ↔ Command Registry ↔ OpenAPI Runtime Generation
Files: src/gpt_interface/definitions.yaml, src/gpt_interface/gen_openapi.rs (lines 6–217), src/commands/utils/command_system.rs (COMMAND_REGISTRY, command! macro, lines 16–176), src/commands/utils/command_types.rs (CommandMetadata, CommandParam, lines 22–60), docs/api_contract.md

Description: Commands are declared with the command! and param! macros at startup using the #[ctor::ctor] attribute — they self-register into a global Lazy<Mutex<HashMap>> at binary load time. The registry is then serialized to definitions.yaml on boot and converted to a full OpenAPI 3.0.0 document served at /static/openapi.yaml. This effectively implements a runtime schema-first workflow: definitions.yaml is the single source of truth for what the GPT agent can call, and the OpenAPI spec is emitted as a side effect.

Evidence: COMMAND_REGISTRY (command_system.rs:16), register_command (line 20), generate_definitions_yaml (lines 138–157), build_openapi_yaml (gen_openapi.rs:6–217), .OpenAPI.yaml (auto-written from a test, openapi_tests.rs:24).

What is generic vs. domain-coupled: The registry, macro system, and YAML/OpenAPI generation are entirely generic dispatch infrastructure. The command contents (workspace_create, drawing_create, etc.) are domain-specific. The OpenAPI generation in gen_openapi.rs is mechanical boilerplate assembled with serde_yaml::Mapping — it does not reuse any standard OpenAPI crate.

Classification: Adapt — The self-registering command registry pattern is directly useful for FERROS S6 as a dynamic command/action catalogue mechanism. The OpenAPI generation should be replaced with a proper utoipa, aide, or okapi crate rather than hand-rolling YAML maps.

Plumbing vs. invariants-sensitive: Mechanical plumbing (the registry, macro, YAML serializer). The fact that definitions.yaml is the GPT agent contract is invariants-sensitive (it must stay in sync with the command registry).

2.2 — Migration-First Schema with SQL Enum Types and PL/pgSQL Converters
Files: migrations/000_enums.sql, migrations/001_init.sql, src/storage/migrations.rs

Description: The project separates enum type DDL (000_enums.sql) from table DDL (001_init.sql). PG enums are defined as CREATE TYPE ... AS ENUM (...). PL/pgSQL helper functions (text_to_workspace_type, text_to_template_type, etc.) perform safe text→enum conversion inside SQL, allowing tokio-postgres to pass plain TEXT parameters through the helpers and avoid needing full ORM-level enum mapping. The migration runner in storage/migrations.rs is hand-rolled (reads sorted .sql files, uses a migrations tracking table, wraps each migration in a transaction) rather than using the refinery crate that is declared as a dependency but appears unused.

Evidence: 000_enums.sql lines 31–104 (4 converter functions), 001_init.sql lines 137–189 (indexes, triggers, composite unique constraints for z-ordering), storage/migrations.rs (full custom runner), Cargo.toml line 47 (refinery dep unused).

What is generic vs. domain-coupled: The migration runner is generic. The PL/pgSQL text→enum converters are a repeatable pattern applicable to any multi-enum schema. The z-index uniqueness constraints (unique composite indexes per parent) are domain-specific but the z-ordering pattern is generic. The update_timestamp trigger function is entirely generic.

Classification: Adopt (migration runner pattern); Reference (PL/pgSQL text→enum converter approach — note that sqlx/refinery handle this more elegantly); Adapt (z-index ordering via DB-enforced unique composite indexes — useful for any ordered hierarchy).

Plumbing vs. invariants-sensitive: The migration runner is mechanical plumbing. The check_annotation_single_parent CHECK constraint (001_init.sql:111–118) is invariants-sensitive — it enforces the "exactly one parent" rule at the DB level. The unique z-index constraints are domain-sensitive correctness constraints, not pure plumbing.

2.3 — Dual-Layer Domain Model (Domain Structs vs. Storage Models)
Files: src/domain/workspace.rs, src/domain/types.rs, src/storage/models/workspace_model.rs, src/storage/models/rendering_model.rs, docs/schema_notes.md

Description: The project separates domain types (domain/) from storage models (storage/models/). Domain types carry proper Rust enums (e.g., WorkspaceType enum), while storage models carry String for the same field for "serialization compatibility." The From<&Row> impl in storage models bridges tokio_postgres::Row to the model struct. Timestamps are eagerly serialized to String (via .to_rfc3339()) in the model rather than kept as chrono::DateTime.

Evidence: domain/workspace.rs:5 (workspace_type: WorkspaceType), storage/models/workspace_model.rs:10 (workspace_type: String with comment "Keep as string for serialization compatibility"), workspace_model.rs:21–29 (try_get fallback to custom type handler).

What is generic vs. domain-coupled: The pattern of having separate domain and persistence representations is generic. The specific try_get fallback mechanism for PG enums, and premature string serialization, are accidental complexity.

Classification: Reference — The separation is correct intent. The implementation (String workspace_type in the model, eager timestamp stringification) is an anti-pattern FERROS should avoid. FERROS should keep timestamps as DateTime<Utc> through the full stack and serialize only at the API boundary.

Plumbing vs. invariants-sensitive: The dual-layer separation is architecture-sensitive. The string workaround is accidental plumbing debt.

2.4 — Custom tokio-postgres Enum Type Handlers
Files: src/storage/type_handlers.rs

Description: WorkspaceTypeEnum and TemplateTypeEnum are manual wrappers that implement FromSql/ToSql for tokio-postgres by matching type name strings (ty.name() == "workspace_type_enum"). This is needed because tokio-postgres cannot natively deserialize custom PG enum types without either derive macros (postgres-types crate feature) or this manual approach.

Evidence: type_handlers.rs:24–55 (WorkspaceTypeEnum impl), type_handlers.rs:71–102 (TemplateTypeEnum impl). Cargo.toml:46 (postgres-types with derive feature is present but unused for this purpose).

What is generic vs. domain-coupled: This is a recurring infrastructure pattern. The postgres-types crate #[derive(ToSql, FromSql)] would eliminate this boilerplate — the crate's derive feature is already in Cargo.toml but not used.

Classification: Discard — FERROS should use the postgres-types derive macros or sqlx's native enum support instead of manual byte-level FromSql/ToSql implementations. This pattern is a source of silent bugs.

Plumbing vs. invariants-sensitive: Mechanical plumbing. The type safety it attempts to enforce is better handled at the ORM/derive level.

2.5 — Dynamic Query Builder (No Query Macro Checking)
Files: src/storage/repositories/workspace_repo.rs (lines 155–216), src/storage/repositories/drawing_repo.rs (lines 222–298), and all other *_repo.rs files

Description: All UPDATE operations use a hand-rolled dynamic query builder: a Vec<String> accumulates "field = $N" fragments, a parallel Vec<&dyn ToSql + Sync> accumulates parameter values, and param_index is manually incremented. This is replicated verbatim in every repository (~10 repos). The .sqlx/ directory contains 7 JSON query cache files targeting SQLite (not PostgreSQL), suggesting an abandoned earlier attempt to use sqlx::query! macro with compile-time checking before the project switched to tokio-postgres.

Evidence: workspace_repo.rs:155–216, drawing_repo.rs:222–298, .sqlx/query-0ecdbf5d...json (db_name: "SQLite"), Cargo.toml (no sqlx in current deps, tokio-postgres used directly).

What is generic vs. domain-coupled: The pattern is generic boilerplate copied for every entity. The abandoned .sqlx/ artifacts indicate compile-time query checking was once desired but abandoned.

Classification: Discard — FERROS should adopt sqlx query! macros (or SeaORM/diesel) for compile-time query type checking. The current dynamic builder is error-prone (parameter indexing off-by-one, no compile-time column-type verification) and is pure undifferentiated plumbing.

Plumbing vs. invariants-sensitive: Mechanical plumbing. The risk is that manual index tracking silently mismatches parameters to columns.

2.6 — Z-Index Ordering Invariant (Transactional Shift-and-Set)
Files: src/storage/repositories/workspace_repo.rs (reorder_workspace, lines 241–276), src/storage/repositories/drawing_repo.rs (reorder_drawing, lines 367–407), every other *_repo.rs, src/storage/repositories/error_helpers.rs

Description: Every entity has a z_index: INT column with a DB-enforced unique composite index (e.g., UNIQUE(view_uuid, z_index)). Reorder operations use a two-step transactional approach: (1) shift all sibling z-indexes ≥ target by +1, (2) update the target entity's z-index. Duplicate z-index insertion retries with random offsets in test environments. error_helpers.rs extracts constraint names from PG error messages to give user-friendly errors.

Evidence: 001_init.sql:137–145 (6 composite z-index unique indexes), workspace_repo.rs:241–276 (transaction-based reorder), workspace_repo.rs:56–82 (random z-index retry loop), error_helpers.rs:5–49.

What is generic vs. domain-coupled: The z-ordering mechanism is entirely generic — any ordered hierarchical collection needs this. The "shift siblings" approach is a known ordered list pattern.

Classification: Adapt — Extract as a generic reorder_siblings(pool, table, parent_col, parent_id, target_id, new_z) function or a reusable trait for ferros-data. The random-retry logic for test isolation is a workaround for a real problem (missing sequence/gap strategy) and should be redesigned with gap-based ordering (e.g., fractional indexes or gap sequences) in ferros-data.

Plumbing vs. invariants-sensitive: Invariants-sensitive — the uniqueness of z-index per parent is a domain correctness invariant. The transaction-based shift is necessary to maintain it.

2.7 — Logical Schema JSONC with Mixins and Feature Flags
Files: docs/logical_schema.jsonc, docs/combined_logical_schema.jsonc, docs/schema_notes.md

Description: The project maintains a docs/combined_logical_schema.jsonc that defines the full intended data model with: (a) reusable mixins (revision_base, styleable, geometric), (b) feature-flag-gated enum values and fields, (c) explicit implementation status tracking per entity. This is a design-time schema contract document, not machine-executed, but it is referenced in migration documentation as a "must keep in sync" artefact.

Evidence: combined_logical_schema.jsonc:42–63 (mixins), combined_logical_schema.jsonc:4–41 (feature_flags), schema_notes.md:165 ("Each migration must update docs/combined_logical_schema.jsonc and this doc").

What is generic vs. domain-coupled: The mixin pattern (revision_base = uuid + timestamps + parent/origin lineage; styleable = style_over + custom_props JSONB) is domain-neutral and directly applicable to FERROS S6's data layer. The feature-flag annotation is a useful pattern for managing incremental schema evolution.

Classification: Adapt — The mixin vocabulary (revision_base, styleable, geometric) is well-conceived and should be formalized in ferros-data as either Rust trait bounds or macro-derived field sets. The feature-flag gating in JSONC is a manual process that could be replaced by Rust feature flags or compile-time conditional migrations.

Plumbing vs. invariants-sensitive: Invariants-sensitive — the revision_base mixin's origin_uuid/parent_uuid fields encode derivation lineage, which is a core data integrity concern for versioned entities.

2.8 — Test Generator Binary (definitions.yaml → Generated Test Files)
Files: src/bin/test_generator.rs, build.rs.old, build.rs.new, tests/test_utils/build/ (commands/, edge_cases/, enums/, fixups.rs)

Description: A dedicated Rust binary reads definitions.yaml, categorizes commands by prefix (workspace_, sheet_, etc.), and writes out async integration test files (tests/commands/{category}/functions.rs) using templated code generation. The test generator produces create/get/list/delete/reorder tests for every command, including UUID dependency resolution for the hierarchical entity graph. build.rs.old contained the original inline generator; build.rs.new delegates to the binary. This is a definitions-first test generation workflow.

Evidence: test_generator.rs:1–48 (entry point, calls build module), tests/test_utils/build/commands/writer.rs:30–80 (file writer with per-command test generation), build.rs.old:43–101 (original inline generator from definitions.yaml), build.rs.new:10–11 (rerun-if-changed=definitions.yaml).

What is generic vs. domain-coupled: The test generator framework (reading a YAML command schema, categorizing by name convention, writing typed async tests) is a generalized scaffolding approach. The dependency graph for "you must create a Workspace before creating a Sheet" is domain-specific but the resolution algorithm is generic.

Classification: Reference — The concept of deriving integration tests from a command schema is a strong idea for S8 tooling. However, the implementation is brittle (string pattern matching on command names, PowerShell scripts in scripts/old/, .rs.old artifacts) and the generated test quality relies on naming conventions rather than explicit metadata. FERROS S8 tooling should adopt the idea but implement it properly with typed descriptors.

Plumbing vs. invariants-sensitive: Mechanical plumbing (the generator itself). The dependency resolution in the generated tests is invariants-sensitive — it correctly models the parent-child creation chain.

2.9 — Redis Cache Layer with Pub/Sub Invalidation
Files: src/storage/repositories/cache_repo.rs, src/storage/models/cache_model.rs

Description: A RedisClient wrapper provides typed JSON set/get/delete with TTL, plus Redis pub/sub channels for cache invalidation messages (SheetUpdatedMessage, ProjectUpdatedMessage, DrawingUpdatedMessage). Channel names are defined as constants in cache_model.rs.

Evidence: cache_repo.rs:1–50 (RedisClient impl), Cargo.toml:51 (redis with aio and tokio-comp).

What is generic vs. domain-coupled: The RedisClient wrapper (set_json_ex, get_json, pub/sub) is generic. The invalidation message structs are domain-specific.

Classification: Adapt — Extract the generic JSON-typed Redis client wrapper into ferros-data as a reusable caching primitive. The pub/sub invalidation channel pattern is useful for FERROS S6 but the channel names and message types should be generated from the entity schema, not hand-coded per entity.

Plumbing vs. invariants-sensitive: Mechanical plumbing.

2.10 — Rendering Snapshot (JSONB Baked Sheet)
Files: src/storage/models/rendering_model.rs, migrations/001_init.sql:120–131, src/storage/repositories/rendering_repo.rs

Description: A renderings table stores a baked_sheet JSONB (the entire rendered sheet snapshot), render_version INT, and rendered_at. This is a denormalized snapshot pattern — rather than re-querying the full hierarchy to render, a baked JSON snapshot is stored alongside normalized records. postgres-types::Json wrapper is used for FromSql.

Evidence: rendering_model.rs:1–33, 001_init.sql:120–131, 001_init.sql:164 (CREATE INDEX idx_renderings_latest ON renderings (sheet_uuid, render_version DESC)).

What is generic vs. domain-coupled: The baked-snapshot pattern (store a JSONB snapshot alongside normalized data for read performance) is domain-neutral. The index on (sheet_uuid, render_version DESC) for "latest rendering" retrieval is a reusable idiom.

Classification: Adopt — The JSONB snapshot pattern with version tracking is directly useful in ferros-data as a "materialized view" or "read model" pattern. The use of postgres_types::Json<T> for JSONB deserialization is the correct approach and should be standardized.

Plumbing vs. invariants-sensitive: render_version is invariants-sensitive (determines canonical current state). The JSONB storage is mechanical plumbing.

2.11 — Annotation Single-Parent CHECK Constraint
Files: migrations/001_init.sql:111–118, src/storage/repositories/annotation_repo.rs:22–29

Description: The annotations table has a DB-level CHECK constraint enforcing that exactly one of five nullable parent FK columns is non-null (sheet_uuid, titleblock_uuid, viewport_uuid, view_uuid, drawing_uuid). The Rust insert function mirrors this validation (parent_count != 1 check before INSERT).

Evidence: 001_init.sql:111–118 (CHECK constraint), annotation_repo.rs:22–29 (Rust mirror validation).

What is generic vs. domain-coupled: The pattern of "exactly one of N nullable FKs" (polymorphic parent) is a common schema design problem. Both DB-level CHECK and application-level pre-validation is the correct defense-in-depth approach.

Classification: Adopt — FERROS should standardize this dual-validation pattern for polymorphic parent relationships. The DB constraint is the authoritative guard; the application check gives better error messages before hitting the DB.

Plumbing vs. invariants-sensitive: Invariants-sensitive — this is a core data integrity rule.

2.12 — Asset Upload with Placeholder + Transactional Replace
Files: src/storage/repositories/drawing_repo.rs:49–110, src/storage/repositories/asset_repo.rs

Description: Drawing insertion uses a two-phase pattern: (1) set file_path = "placeholder://{uuid}", (2) within a transaction, upload the asset to Tigris, then replace the placeholder with the real URL. On UPDATE, the old asset is deleted after the new asset is confirmed uploaded. The generate_asset_path function uses a date-partitioned path ({entity_type}/{yyyy}/{mm}/{dd}/{entity_id}_{uuid}_{filename}).

Evidence: drawing_repo.rs:49–110 (placeholder pattern, transactional commit), asset_repo.rs:41–57 (path generation), asset_repo.rs:60–103 (upload_asset).

What is generic vs. domain-coupled: The date-partitioned asset path pattern and the transactional placeholder-then-replace flow are generic for any blob-backed entity. The Tigris-specific API calls are domain-specific infrastructure.

Classification: Adapt — The asset path strategy and the placeholder pattern are worth extracting as ferros-data conventions. The Tigris client (asset_repo.rs) is app-specific infrastructure that should be abstracted behind a StorageBackend trait for FERROS.

Plumbing vs. invariants-sensitive: Mechanical plumbing (the path and upload logic). The transactional coupling of DB record + blob storage is invariants-sensitive (prevents orphaned records or dangling references).

3. Anti-Patterns and Accidental Complexity
Anti-pattern	Evidence	Risk for FERROS
Premature timestamp serialization	workspace_model.rs:35 — .to_rfc3339() at row-read time stores timestamps as String	Loses type safety; re-parsing needed; timezone handling fragile
Three-layer type confusion for enums	Domain WorkspaceType enum → Storage String → custom WorkspaceTypeEnum wrapper; postgres-types derive feature present but unused	Should be eliminated with one layer using postgres-types derive
Hand-rolled dynamic UPDATE builder	Duplicated across all 10 repos; param_index manual tracking	Error-prone; no compile-time verification; sqlx macros fix this
Random z-index retry in tests	workspace_repo.rs:71–76 (retry loop with rand)	Masks a missing gap-ordering strategy; flaky under load
Abandoned .sqlx/ SQLite artifacts	.sqlx/query-*.json all have "db_name": "SQLite"	Suggests mid-migration abandonment; .sqlx/ with stale SQLite queries will confuse new developers
schema.rs stub	storage/schema.rs:4 — single line WORKSPACE_TABLE that is never used	Dead code; creates false impression of schema management
refinery declared but unused	Cargo.toml:47, custom migration runner in storage/migrations.rs	Dependency bloat; custom runner has no checksum verification
Error message string-matching for routing	command_system.rs:207–217 — error_msg.contains("Missing required parameter")	Fragile; should use typed error variants with thiserror
Layer drift between domain, DB, and JSONC spec	schema_notes.rs:73–97 — "View: DB table: none (inline JSON)"	Three sources of truth create inevitable drift; needs a single authoritative source (code-first or migration-first)
global_registry writes a YAML file at startup	command_system.rs:172 — writes to src/gpt_interface/definitions.yaml at runtime	Writing source-controlled files at runtime from a binary is fragile and will silently reorder committed YAML
4. Evidence of Schema-First / Contract-Enforcement Workflows
Workflow	Evidence	Assessment
Runtime OpenAPI generation from registry	gen_openapi.rs:6–217, openapi_tests.rs	Present; not compile-time; no request validation against spec at runtime
Abandoned sqlx compile-time query check	.sqlx/ directory with SQLite JSON artifacts	Was attempted; abandoned when switching to tokio-postgres
Migration-first with enum DDL	000_enums.sql, 001_init.sql	Good: enums defined before tables, text converters enforce valid values at DB layer
definitions.yaml as command schema contract	definitions.yaml, api_contract.md	Explicit LLM-facing contract; partially enforced (no validation middleware)
JSONC logical schema with mixin inheritance	combined_logical_schema.jsonc	Design-time only; not machine-executed; drifts from DB and code
BUILD-time test generation from YAML	build.rs.old, src/bin/test_generator.rs	Code generation from schema is present; quality of generated tests is low
Key gap: There is no compile-time or request-time enforcement that POST /command/{name} params conform to the parameter schema in definitions.yaml. Validation is ad-hoc (params["name"].as_str().ok_or_else(...) in each handler).

5. Classification Summary
Candidate	Classification	Layer	Plumbing / Invariant
Command self-registration registry + macros	Adapt	ferros-data / S8 tooling	Plumbing
definitions.yaml → OpenAPI runtime generation	Adapt	S8 tooling	Plumbing
Migration-first with SQL enum DDL	Adopt	ferros-data	Both
PL/pgSQL text→enum converters	Reference	ferros-data	Plumbing
Z-index composite unique index ordering	Adapt	ferros-data	Invariant
Transactional z-index shift-and-set	Adapt	ferros-data	Invariant
Dual-layer domain/storage model separation	Reference	ferros-data	Architecture
Custom tokio-postgres enum type handlers	Discard	— (use postgres-types derive)	Plumbing
Hand-rolled dynamic UPDATE builder	Discard	— (use sqlx macros)	Plumbing
Logical JSONC schema with mixins + feature flags	Adapt	ferros-data	Both
Test generator binary (definitions → tests)	Reference	S8 tooling	Plumbing
Redis JSON cache wrapper with pub/sub	Adapt	ferros-data	Plumbing
JSONB baked rendering snapshot pattern	Adopt	ferros-data	Invariant
Annotation single-parent CHECK + app mirror	Adopt	ferros-data	Invariant
Asset date-partitioned path + placeholder pattern	Adapt	ferros-data	Both
Startup YAML file-write from registry	Discard	—	Anti-pattern
String-matching error routing	Discard	—	Anti-pattern
Premature timestamp String serialization	Discard	—	Anti-pattern
6. Top Recommendations for ADR-020 (Decision-Oriented Bullets)
FERROS S6 must adopt a single authoritative schema source. sheetgen-rust's three-layer drift (domain Rust types / migration DDL / JSONC spec) is its main maintenance burden. ADR-020 should decide whether FERROS is migration-first (DDL is truth, types are derived) or code-first (Rust structs are truth, migrations are generated). The sheetgen pattern is nominally migration-first but has no code generation from DDL, creating drift.

Adopt the revision_base mixin as a standard ferros-data trait. sheetgen's JSONC mixins (uuid pk, parent_uuid, origin_uuid, version, created_at, updated_at) represent a well-reasoned baseline for all versioned entities. FERROS should formalize this as a derive macro or trait bound rather than repeating fields.

Adopt the JSONB rendering snapshot with version-indexed retrieval. The renderings table pattern (baked JSONB + render_version + composite index for latest) is ready to adopt as ferros-data's read-model or materialized-snapshot pattern.

Adopt the dual-validation guard for polymorphic parent constraints (DB CHECK + application pre-check). The annotation constraint (check_annotation_single_parent) is a model for how FERROS should handle "entity belongs to exactly one of N parents."

Adapt the z-index ordering invariant but replace the random-retry workaround. The transactional shift-and-set reorder is correct but the random-z-index collision handling reveals a design gap. FERROS should use fractional/gap-based ordering or a sequence table to avoid collisions entirely, then generalize this as a ferros-data Orderable<Parent> trait.

Replace hand-rolled tokio-postgres code with sqlx macros. The abandoned .sqlx/ SQLite artifacts indicate the intent was always compile-time query checking. FERROS S6 should not inherit the tokio-postgres + manual dynamic builders approach; use sqlx::query! with a PostgreSQL compile-time check environment.

The definitions.yaml → OpenAPI generation pattern is worth adopting for S8 tooling, but the implementation should use a standard OpenAPI crate (not hand-rolled serde_yaml::Mapping assembly). The self-registering command registry via #[ctor] is a useful pattern for dynamic command catalogues with LLM agents.

Discard the startup YAML file-write behavior. Writing to committed source files at binary startup (definitions.yaml) is an anti-pattern that should not be brought into FERROS. The contract document should be generated by a dedicated cargo xtask or build.rs step, not at runtime.

Generalize the asset storage layer behind a trait before adoption. The Tigris client in asset_repo.rs is useful reference for a date-partitioned blob storage abstraction, but it should be abstracted behind a BlobStorage trait with S3-compatible, local-filesystem, and in-memory test implementations before entering ferros-data.

The Redis cache wrapper is extractable as generic ferros-data infrastructure, but the invalidation message types should be generated from the entity schema (analogous to how migrations can be generated), not hand-coded per entity. Establish this discipline in ADR-020 before adding Redis to ferros-data.