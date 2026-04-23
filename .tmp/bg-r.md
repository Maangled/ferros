FERROS ADR-018 Input: Architectural Audit of Maangled/botgen-rust
Executive Summary
The botgen-rust repository is a multi-agent Discord bot platform written in Rust. It consists of a workspace of ~15 crates across core/, services/, and bots/. Its domain closely overlaps with the five FERROS S6 concerns: agent lifecycle, registry shape, work queue/dispatch, materialize-from-description, and history/provenance/audit trail. The codebase is in a mid-construction state: type definitions and in-memory structures are well-developed, but the persistence binding for several critical subsystems (most notably the work-queue persistence module, the agent-repository SQL implementations, and the manager AgentManager struct itself) are commented out or stubbed with todo!(). Cross-file synthesis reveals meaningful gaps in the lifecycle→registry→queue integration chain, and the two separate agent-history surfaces (task audit trail in the queue vs. AgentEvent in the doc-spec command file) are never connected.

Section 1 — Agent Lifecycle
Key Files
File	Role
core/shared/src/agent.rs	Minimal shared Agent trait + AgentHealth, AgentStatus
services/agents/src/agent.rs	Richer Agent trait re-declaration, AgentHandle, AgentStatus
services/agents/src/agents/base_agent/types.rs	AgentState (7 states), BaseAgentConfig, BaseAgentExt trait
services/agents/src/agents/base_agent/agent.rs	Concrete BaseAgent with three tokio subtasks
services/agents/src/agents/base_agent/lifecycle.rs	Bridges BaseAgentExt → Agent (MainAgentTrait)
services/agents/src/registry.rs	AgentRegistry — HashMap<String, AgentHandle>
services/agents/src/manager.rs	AgentManager — health-check loop, emit_event, register_agent
core/gateway/src/agent.rs	Concrete GatewayAgent implementing lifecycle
What the System Does
BaseAgent (services/agents/src/agents/base_agent/agent.rs:15–27) spawns three independent tokio tasks on start():

A task-processing loop that polls WorkQueueApi::get_next_task in a tight poll (100 ms sleep on empty), updates status to Completed or Failed via update_task_status (lines 46–98).
A message-handling loop that subscribes to AgentCommProtocol with a MessageFilter targeting its own UUID, receives messages, dispatches to handle_message_internal (which returns Ok(None) — stubbed), and sends optional responses (lines 100–134).
A health-monitoring loop that ticks on health_check_interval (lines 136–149).
Lifecycle states in AgentState (types.rs:12–19): Uninitialized → Initializing → Ready → Running → Paused → Shutting → Terminated | Failed(String). Paused is declared but never transitioned into in the current implementation.

Shutdown uses a watch::channel(false) broadcast to all three loops.

AgentManager (manager.rs) conceptually sits above AgentRegistry with a config-driven agent_cache (using moka) and a periodic health-check loop, plus an emit_event method that forwards to an event_bus. However, AgentManager exists only as a doc-spec prototype in docs/checklists/01-setup-agents/04-agent-manager/command.rs — it is not compiled as a library crate. The concrete operational registry is services/agents/src/registry.rs.

AgentHandle (services/agents/src/agent.rs:122–241) wraps Arc<RwLock<Box<dyn Agent>>> and a JoinHandle. Its AgentHandleTrait::id() implementation returns the hard-coded string "agent" (line 248), breaking any keyed lookup — a concreteness gap.

Classification for FERROS S6
Component	Classification	Rationale
AgentState 7-value enum + AgentCapabilities	adopt	Clean state machine shape; Paused variant is a bonus alignment point for FERROS suspend semantics. Consuming stream: FERROS S6 agent runtime. Violates: no explicit Resuming re-entry state (FERROS invariant: every state transition must be reversible or explicitly terminal). Post-G2? No — needed at G2.
BaseAgent three-task pattern	adapt	The pattern (task loop + message loop + health loop driven by watch::Sender) maps well to FERROS's concurrent subtask model. Must be adapted: process_task_internal is a stub returning static JSON; handle_message_internal always returns Ok(None); neither implements the actual capability dispatch FERROS needs. Consuming stream: FERROS S6 agent executor.
AgentHandle	discard	The hard-coded id() is structurally broken for registry key lookups. Replace entirely with FERROS's typed handle.
AgentRegistry (services/agents/src/registry.rs)	adapt	Correct shape: HashMap<String, AgentHandle> behind Arc<RwLock>. Violates: key type is String, not a typed AgentId; start_all iterates in HashMap-arbitrary order, risking dependency-order violations. Consuming stream: FERROS S6 registry. Post-G2? No.
AgentManager (doc-spec in docs/checklists/)	reference	Documents intent: health-check loop, event emission, cache TTL. Is not executable code; should inform FERROS design but not be adopted.
GatewayAgent	reference	A complete concrete example of the lifecycle contract. Useful as a reference implementation test fixture for FERROS.
Section 2 — Registry Shape
Key Files
File	Role
services/agents/src/registry.rs	Primary agent registry
core/routing/src/registry.rs	CommandRegistry — agent-to-command mapping
services/storage/src/function_registry/structure_registry.rs	StructureRegistry — per-bot table schema registry
services/storage/src/function_registry/mod.rs + generator.rs	FunctionRegistry — per-bot CRUD function metadata
migrations/20250724120000_registry_table.sql	DB tables: table_structure_registry, function_registry
core/database/src/repositories/agent_repository.rs	AgentRepository trait + PgAgentRepository (all methods todo!())
What the System Does
There are four logically distinct registries, none of which talk to each other:

Agent Registry (services/agents/src/registry.rs): In-memory HashMap<String, AgentHandle>. Supports register, get, start-all, stop-all, health-all. No persistence; agent identity is a String.

Command Registry (core/routing/src/registry.rs): In-memory HashMap<String, CommandInfo> where CommandInfo links a command name to an agent_id: String. Supports register, unregister, query by agent. No persistence.

Table Structure Registry (services/storage/src/function_registry/structure_registry.rs): DB-backed (table_structure_registry table). Stores column schemas as JSONB per (bot_id, table_name). Used for code generation.

Function Registry (services/storage/src/function_registry/mod.rs): DB-backed (function_registry table). Stores generated CRUD function metadata per (bot_id, table_name, function_name).

The PgAgentRepository (core/database/src/repositories/agent_repository.rs) defines the AgentRepository trait with find_by_id, create, update_status, update_heartbeat, find_all — all five methods are todo!() macros (lines 43–66). This means agent persistence to the database has no working implementation. The in-memory AgentRegistry is authoritative but volatile.

There is no cross-registry join: the agent registry does not know about command registrations, and no component resolves agent_id → capabilities across both registries.

Classification for FERROS S6
Component	Classification	Rationale
AgentRegistry structure	adapt	Shape is right. Must be adapted: String key must become typed AgentId (UUID); persistence binding is absent; no TTL eviction for dead entries. Consuming stream: FERROS S6 registry. Violates: no durable identity. Post-G2? Registry itself is G2; persistence can be post-G2.
CommandRegistry	adapt	Directly maps to FERROS command-capability index. The commands_for_agent query (by agent string ID) is precisely what S6 needs for capability lookup. Must adapt key to typed ID. Post-G2.
table_structure_registry + function_registry DB tables	reference	These are domain-specific artifacts of botgen's code-gen pipeline. Not applicable to FERROS agent registry. Useful only as an example of how per-bot JSONB schemas can be stored durably.
PgAgentRepository	discard	All methods todo!(). Write fresh with typed IDs and proper transaction semantics.
Agent DB model (core/database/src/models/agent.rs)	adapt	Schema (id UUID, name, agent_type, config JSONB, status, last_heartbeat) is a reasonable baseline. The create_agent function is unimplemented!() (line 37). Adapt the schema; discard the non-compiling function stubs.
Section 3 — Work Queue and Dispatch
Key Files
File	Role
core/shared/src/work_queue/types.rs	Task, TaskStatus, Priority, TaskHistoryEntry, AgentInfo, TaskMetadata
core/shared/src/work_queue/queue.rs	WorkQueue — all in-memory state and transitions (~500 lines)
core/shared/src/work_queue/scheduler.rs	TaskScheduler with SchedulingPolicy enum (PriorityFirst, FairShare, LoadBalanced, Specialized)
core/shared/src/work_queue/worker.rs	Worker, WorkerPool, WorkerTrait
core/shared/src/work_queue/api.rs	WorkQueueApi — async facade wrapping SharedWorkQueue + SharedTaskScheduler
core/shared/src/work_queue/persistence.rs	QueuePersistence — save/load queue to Postgres
core/shared/src/work_queue/mod.rs	Module root — persistence module commented out
core/gateway/src/api/work_queue.rs	Gateway HTTP API surface for the queue
What the System Does
WorkQueue (queue.rs) is a multi-index in-memory structure:

tasks: HashMap<TaskId, Task> — task store
priority_queues: HashMap<Priority, VecDeque<TaskId>> — four-tier FIFO per priority
status_index: HashMap<TaskStatusKind, HashSet<TaskId>> — fast status-based filtering
dependencies / dependents: HashMap<TaskId, HashSet<TaskId>> — full bidirectional DAG
history: Vec<TaskHistoryEntry> — append-only status-change log
agents: HashMap<AgentId, AgentInfo> — agent capability + capacity tracking
assignments: HashMap<TaskId, TaskAssignment> — current task-agent bindings
completed_tasks: HashSet<TaskId> — dependency resolution sentinel
Key operations are well-implemented: add_task (circular-dependency check + DAG registration + history record), assign_task (capability match + capacity guard + Running status transition + history entry), update_task_status (state-machine transition validation, dependent-unblocking on completion), get_next_task (priority-descending scan with can_start() + capability match).

TaskScheduler (scheduler.rs) wraps SharedWorkQueue and implements assign_next_task(agent_id) using the configured SchedulingPolicy. The Specialized policy selects the best-matching agent for a task; LoadBalanced uses weighted preference scores. This is the most sophisticated piece in the repository.

WorkQueueApi (api.rs) exposes submit_task, get_next_task, update_task_status, register_agent, unregister_agent, get_task, get_metrics as async fn. These are what BaseAgent calls from its task-processing loop.

Critical gap: The persistence module (persistence.rs) compiles correctly with full SQL implementations but is commented out of mod.rs with the note "Temporarily disabled — requires database for compile-time checks" (mod.rs:16). The WorkQueueApi::persistence field is Option<()> (api.rs:157). This means the queue is fully ephemeral at runtime: all task state is lost on restart.

Worker/WorkerPool (worker.rs) provides a thin abstraction above the queue — workers register as AgentInfo, spawn a run loop that calls scheduler.assign_next_task, and simulate work with a sleep(100ms) (line 79). The actual processing logic is a placeholder.

Classification for FERROS S6
Component	Classification	Rationale
Task struct with dependencies: Vec<TaskId>, required_capabilities: Vec<String>, crate_scope: Option<String>	adopt	Precisely the shape FERROS S6 needs. crate_scope is a direct predecessor of FERROS's namespace/domain concept. Consuming stream: FERROS S6 task type. No FERROS invariant violations if TaskId is re-typed to FERROS's ID space. G2.
Priority 4-tier enum (Critical/High/Normal/Low)	adopt	Mirrors MessageSeverity in agent_comm; FERROS should unify these into a single severity type. G2.
TaskStatus variant enum with embedded agent IDs and timestamps	adopt	Rich, inspectable state. The Completed { result: serde_json::Value } variant provides the result-embedding FERROS needs. G2.
TaskHistoryEntry	adopt	Timestamp + task_id + agent_id + old/new status + details string. Clean provenance atom. See Section 5. G2.
WorkQueue in-memory multi-index	adapt	The structure is excellent; the shared-state model (Arc<RwLock<WorkQueue>>) will contend under FERROS's higher concurrency. Must be adapted: replace single RwLock with per-partition locking or a MPSC-based actor model. The DAG implementation is adoptable as-is. Consuming stream: FERROS S6 queue core.
TaskScheduler with SchedulingPolicy	adapt	The Specialized and LoadBalanced policies are directly useful. Must be adapted: assign_next_task currently scans WorkQueue synchronously while holding an RwLock::write — will deadlock under FERROS's concurrent worker model. Consuming stream: FERROS S6 dispatcher. Post-G2.
WorkerPool	discard	Placeholder implementations (100 ms sleep for "processing"). Does not generalize to FERROS's typed task executor model.
QueuePersistence	adapt	SQL is well-written and transactional (single BEGIN/COMMIT wrapping all saves). Must be re-enabled (comment out removal is trivial). Violates FERROS invariant: history capped at last-1000 entries (persistence.rs:61) — FERROS requires unbounded append-only history. Consuming stream: FERROS S6 durable queue. Post-G2 except save_task/load_task which are G2.
WorkQueueApi async facade	adapt	Clean separation point. Re-wire persistence: Option<()> to real QueuePersistence. Consuming stream: FERROS S6 queue API. G2.
Section 4 — Materialize-from-Description Flow
Key Files
File	Role
bots/botgen_bot/src/lib.rs	BotSpec struct + BotInstance + ServiceType enum
bots/botgen_bot/src/generator.rs	BotGenerator::generate_bot(spec: BotSpec)
bots/botgen_bot/src/template_engine.rs	TemplateEngine::generate_files
bots/botgen_bot/src/service_registry.rs	ServiceRegistry::configure_bot
bots/botgen_bot/src/commands.rs:420	Discord /generate command building BotSpec and calling generate_bot
bots/ainpc_bot/src/personality_generator.rs	PersonalityGenerator::generate(channel) → LLM → GeneratedPersonality
bots/botgen_bot/src/ai_npc/manager.rs	AINPCManager::deploy_to_channel(channel) — LLM → character → intro post
bots/botgen_bot/src/ai_npc/personality_generator.rs	Duplicate of ainpc_bot generator in botgen context
bots/ainpc_bot/src/storage/ainpc_storage.rs	AINPCStorageService::create_character + deploy_to_channel
services/generator/src/lib.rs	SchemaSpec → EntitySpec → ColumnSpec (YAML-driven code gen)
schema_spec/bots/ainpc.yaml	Declarative bot spec (entities, CRUD declarations)
What the System Does — Two Distinct Pipelines
Pipeline A: Bot Crate Generation (botgen_bot)

Code
Discord command (/generate name description)
  → BotSpec { name, description, command_prefix, template, required_services, custom_config }
  → BotGenerator::validate_spec()
  → create_dir_all(output_dir/bot_name)
  → TemplateEngine::generate_files(template, TemplateParams, bot_dir)
  → ServiceRegistry::configure_bot(bot_dir, spec)  ← modifies Cargo.toml
  → Command::new("cargo").args(["check", "--all-features"]).output()
  → BotInstance { name, path, command_prefix, id: Uuid }
This generates a new Rust crate from templates. It is synchronous (uses std::process::Command, not tokio::process) and invokes the compiler to verify the output. The apply_changes method (for modification) returns Ok(()) as a placeholder.

Pipeline B: AI NPC Character Materialization (ainpc_bot / botgen_bot)

Code
Channel event (new channel / Discord command)
  → AINPCManager::deploy_to_channel(channel_info)
  → PersonalityGenerator::generate(channel)
      → PersonalityTemplate::for_channel(channel_name) → template
      → LlmClient::generate_text(prompt) → personality_details
      → GeneratedPersonality { name, role, expertise, personality, avatar_prompt }
  → AINPCStorageService::create_character(CreateCharacterRequest)
  → AINPCStorageService::deploy_to_channel(character_id, channel_id)
  → IntroductionGenerator::create_introduction(npc)
  → DiscordService::send_message / send_webhook_message
PersonalityTemplate::for_channel does channel-name prefix matching to select an expert archetype. The CreateCharacterRequest and Character structs (ainpc_storage.rs) capture name, description, personality: serde_json::Value, voice_config, guild_id, channel_id, created_by. The deploy_to_channel method in the concrete implementation (ainpc_storage_impl.rs:155) writes to Postgres.

Pipeline C: Schema-to-CRUD Code Generation (schema_spec + services/generator)

Code
ainpc.yaml (SchemaSpec)
  → EntitySpec[] → ColumnSpec[] → CrudSpec
  → GeneratorService::generate_from_schema()  ← partially implemented
  → SQL migration files (manual)
  → function_registry entries (via generate_and_register_table_crud)
  → StructureRegistry::register_table
The ainpc.yaml schema spec declares entities, column types, CRUD operations, and indexes declaratively. The comment at the bottom explicitly states: "Future: generator will emit migrations + function registry Rust code from this file." This pipeline is aspirational; the actual migration files are manually authored.

Cross-Pipeline Observation
The three pipelines share no common abstraction. BotSpec (Pipeline A) has no relation to SchemaSpec (Pipeline C), and neither is consulted by the NPC materialization flow (Pipeline B). This is the key structural gap: there is no unified "agent description → materialized agent" contract.

Classification for FERROS S6
Component	Classification	Rationale
BotSpec struct	adapt	The fields (name, description, required_services, custom_config) are a reasonable "intent description" shape. Must be adapted: add agent_type, capabilities, schema_ref fields; remove Discord-specific fields. Consuming stream: FERROS S6 agent descriptor. G2.
BotGenerator::generate_bot	reference	The step sequence (validate → scaffold → configure → verify) is the right pattern. The synchronous cargo check invocation is incompatible with FERROS's async agent runtime. Reference for process design only.
PersonalityGenerator (ainpc_bot)	reference	Domain-specific LLM prompt template pattern. Not applicable to FERROS unless FERROS S6 materializes AI personas; useful as a reference for how LLM output is structured into a typed GeneratedPersonality.
AINPCStorageService::create_character	reference	The CreateCharacterRequest → Character round-trip is a working example of persist-on-materialize. Reference for FERROS agent-creation persistence pattern.
SchemaSpec / ainpc.yaml	adopt	The YAML declarative schema format (version, bot, schema, entities, CRUD intents, indexes) is close to FERROS's desired "agent manifest" format. Adopt the YAML structure; replace bot: with agent_type: and add capability declarations. Consuming stream: FERROS S6 agent manifest. Post-G2.
generate_and_register_table_crud	reference	Demonstrates introspect-then-register pattern for generated functions. Reference for FERROS capability auto-registration post-materialize. Post-G2.
DeploymentManager (ainpc_bot)	discard	is_deployed always returns Ok(true), fetch_deployed_character always returns Ok(None). Placeholder-only.
Section 5 — Agent History, Audit Trail, Provenance, Event Log
Key Files
File	Role
core/shared/src/work_queue/types.rs:224–259	TaskHistoryEntry struct definition
core/shared/src/work_queue/queue.rs:31, 87, 131, 214, 301, 343, 520–529	In-queue history accumulation
core/shared/src/work_queue/persistence.rs:57–64, 251–400	DB save/load of TaskHistoryEntry → work_queue_history table
core/shared/src/agent_comm/event.rs	EventBus — in-memory event pub/sub
docs/checklists/01-setup-agents/04-agent-manager/command.rs:195–255	AgentEvent enum (Registered, Initialized, Started, Stopped, Failed, RecoveryAttempt, etc.)
bots/ainpc_bot/src/storage/ainpc_storage.rs:27–31	Memory struct — per-character interaction memory
migrations/20240101000000_ai_npc_deployments.sql	ai_npc_deployments — deployment record with created_at / updated_at
migrations/20250624000000_bot_deployments.sql	bot_deployments — deployment_state, health_status JSONB, removed_at
What the System Does — Three Separate History Surfaces
Surface 1: Work Queue Audit Trail (most mature)

Every call to add_task, update_task_status, assign_task, and unregister_agent appends a TaskHistoryEntry to queue.history. The entry contains:

timestamp: DateTime<Utc>
task_id: TaskId
agent_id: Option<AgentId>
old_status: TaskStatus
new_status: TaskStatus
details: String (free text, e.g. "Task assigned to agent X and set to Running")
get_task_history(task_id) provides a filtered view. The persistence layer (persistence.rs) saves to a work_queue_history table and loads it back, making this a durable audit trail in design. However, persistence is commented out (mod.rs:16), so at runtime it is ephemeral. Additionally, the persistence layer hard-caps the save to history.iter().rev().take(1000) (persistence.rs:61), which would silently discard older history entries on every save — a data-loss invariant violation for audit purposes.

Surface 2: Agent Lifecycle Event Bus (design-only)

EventBus (agent_comm/event.rs) is an in-memory pub/sub using HashMap<String, Vec<EventHandler>> where handlers receive &AgentMessage. It can emit arbitrary named events.

AgentEvent (docs/checklists/.../command.rs:195–255) defines a rich vocabulary: Registered { agent_id, agent_type }, Initialized, Started, Stopped, Failed { reason }, RecoveryAttempt { attempt }, Recovered, RecoveryFailed, StatusChanged { old_status, new_status }, Custom { event_type, payload }. This is documented in the checklist folder and referenced in AgentManager::emit_event (manager.rs:53–63), but AgentManager itself only exists in that checklist-code prototype. No running code emits or persists AgentEvent records.

Surface 3: Conversation/Memory (AINPC-domain)

Memory struct (ainpc_storage.rs:26–31) stores per-character user interactions: character_id, user_id, interaction_type, content, emotional_context JSONB, importance, timestamp. store_memory and get_recent_memories are defined in the AINPCStorageService trait and implemented in ainpc_storage_impl.rs. The memory_system.rs file in ainpc_bot is deprecated with a comment redirecting to MemoryStorage.

Missing pieces identified:

No agent lifecycle event log is persisted anywhere. Agent register/start/stop/fail transitions are not durable.
TaskHistoryEntry records are durable in design but ephemeral in the currently-compiled code.
No global provenance chain exists: given a BotInstance or Character, there is no path back to the original BotSpec or CreateCharacterRequest that produced it.
bot_deployments migration tracks deployment_state but the corresponding repository methods are todo!().
The EventBus pub/sub and the AgentEvent vocabulary are disconnected from TaskHistoryEntry — there is no unified event log.
Classification for FERROS S6
Component	Classification	Rationale
TaskHistoryEntry struct	adopt	The shape (timestamp, task_id, agent_id, old→new status, details) is precisely what FERROS S6's task audit log needs. Consuming stream: FERROS S6 audit/provenance layer. G2.
In-queue history: Vec<TaskHistoryEntry> accumulation	adopt	The pattern of recording on every state transition, not just on demand, is architecturally correct. Consuming stream: same. G2.
QueuePersistence::save_history_entry / load_history SQL	adapt	The SQL is correct. Must remove the 1000-entry cap (persistence.rs:61) before adopting — this cap silently destroys old history and violates any audit invariant. Consuming stream: FERROS S6 durable audit trail. Post-G2 (durability).
AgentEvent enum (command.rs doc-spec)	adopt	Rich, well-typed vocabulary. This enum should be the canonical lifecycle event type for FERROS. Reference file: docs/checklists/01-setup-agents/04-agent-manager/command.rs:195–255. Needs to be moved into a real compiled crate. Consuming stream: FERROS S6 lifecycle event log. G2 (type promotion), Post-G2 (storage).
EventBus (agent_comm/event.rs)	adapt	The HashMap<event_name, Vec<handler>> pattern works for in-process pub/sub. Must be adapted: add persistence sink; replace &AgentMessage handler signature with typed AgentEvent; add ordering guarantees. Consuming stream: FERROS S6 event bus. Post-G2.
Memory / store_memory (AINPC)	reference	Domain-specific interaction log. The importance and emotional_context fields are AINPC-specific. Reference for how FERROS might model agent-user interaction history in a specialized agent. Post-G2.
bot_deployments table	reference	Tracks deployment state transitions with created_at / updated_at / removed_at. A useful pattern for FERROS's agent deployment record, but the removed_at-based soft-delete should be adapted into an append-only event table for full provenance. Post-G2.
CommandRouter (doc-spec command.rs:289–530)	reference	Prometheus-instrumented command routing with pending-command map and direct-channel dispatch. The correlation_id-based response matching is valuable for FERROS request tracing. The Prometheus metric definitions can be adopted directly. Post-G2.
Section 6 — Cross-File Synthesis: Integration Gaps Relevant to FERROS S6
Gap 1: Lifecycle → Registry → Persistence Disconnect
The flow BaseAgent::start() → AgentRegistry::register() → PgAgentRepository::create() is broken at the last step: all PgAgentRepository methods are todo!(). An agent that starts successfully is registered only in the in-memory HashMap and vanishes on restart. FERROS S6 must close this gap before G2.

Gap 2: Work Queue Dispatch → Worker Loop
BaseAgent's task-processing loop calls WorkQueueApi::get_next_task with a 100 ms polling interval. WorkQueueApi wraps SharedWorkQueue but the persistence field is Option<()>. The WorkerPool (separate path) also polls the scheduler. These are two independent dispatch paths that can both claim the same task, as get_next_task is a read-only scan without atomic claim (the claim only happens in assign_task or update_task_status). FERROS S6 must introduce an atomic claim operation (e.g., SELECT FOR UPDATE SKIP LOCKED) before G2.

Gap 3: Materialize-from-Description → Registry Registration
After BotGenerator::generate_bot(spec) returns a BotInstance, there is no call to AgentRegistry::register(). The generated bot crate is written to disk but never started or registered in the runtime. The AINPC pipeline (deploy_to_channel) persists the character to Postgres but does not create an Agent and register it. FERROS S6 must define the post-materialize registration contract as part of the G2 interface.

Gap 4: AgentEvent Vocabulary Exists in Wrong Layer
The richest event vocabulary (AgentEvent in command.rs) lives in the doc/checklist layer — it is not compiled. The compiled EventBus (agent_comm/event.rs) only handles generic AgentMessage. The compiled AgentManager health checks emit log lines but no persisted events. FERROS must promote AgentEvent into a first-class compiled type and wire it into the EventBus before G2 event-log requirements can be met.

Gap 5: Dual Agent Trait Definitions
core/shared/src/agent.rs and services/agents/src/agent.rs both define Agent, AgentStatus, AgentHealth, and AgentHandle. They are structurally identical but are separate types. BaseAgent's lifecycle.rs bridges them via impl MainAgentTrait for BaseAgent, but AgentRegistry uses the services/agents variants. FERROS S6 should unify to a single canonical Agent trait in core_shared to prevent downstream divergence.

Consolidated Classification Table
Module / Component	File(s)	Classify	FERROS Stream	G2?	Key Violation
AgentState 7-variant enum	base_agent/types.rs	adopt	S6 runtime	Yes	Missing Resuming
BaseAgent 3-task pattern	base_agent/agent.rs	adapt	S6 executor	Yes	Process methods stubbed
AgentCapabilities struct	base_agent/types.rs	adopt	S6 registry	Yes	None
AgentHandle (services/agents)	services/agents/src/agent.rs	discard	—	—	id() hard-coded
AgentRegistry	services/agents/src/registry.rs	adapt	S6 registry	Yes	String key, no persistence
AgentManager (doc-spec)	docs/checklists/…/command.rs	reference	S6 manager	No	Uncompiled
GatewayAgent	core/gateway/src/agent.rs	reference	S6 test fixture	No	None
PgAgentRepository	core/database/src/repositories/agent_repository.rs	discard	—	—	All todo!()
Agent DB model	core/database/src/models/agent.rs	adapt	S6 persistence	Yes	Functions unimplemented
CommandRegistry	core/routing/src/registry.rs	adapt	S6 capability index	Post-G2	String key
Task + TaskStatus + Priority	work_queue/types.rs	adopt	S6 task type	Yes	None
TaskHistoryEntry	work_queue/types.rs:224	adopt	S6 audit trail	Yes	None
AgentInfo + AgentMetrics	work_queue/types.rs:262	adopt	S6 capacity model	Yes	None
WorkQueue in-memory	work_queue/queue.rs	adapt	S6 queue core	Yes	Single RwLock contention
TaskScheduler (all policies)	work_queue/scheduler.rs	adapt	S6 dispatcher	Post-G2	Sync lock in async critical section
WorkerPool	work_queue/worker.rs	discard	—	—	Processing is placeholder
WorkQueueApi async facade	work_queue/api.rs	adapt	S6 queue API	Yes	Persistence field is Option<()>
QueuePersistence	work_queue/persistence.rs	adapt	S6 durable queue	Post-G2	1000-entry history cap
BotSpec	bots/botgen_bot/src/lib.rs	adapt	S6 agent descriptor	Yes	Discord-specific fields
SchemaSpec / ainpc.yaml	schema_spec/bots/ainpc.yaml	adopt	S6 agent manifest	Post-G2	None (aspirational)
PersonalityGenerator	bots/ainpc_bot/src/personality_generator.rs	reference	—	No	Domain-specific
AINPCStorageService trait	ainpc_storage.rs	reference	—	No	Domain-specific
DeploymentManager (ainpc_bot)	manager/deployment_manager.rs	discard	—	—	All methods placeholder
AgentEvent enum	docs/checklists/.../command.rs:195	adopt	S6 event log	Yes	Uncompiled
EventBus	agent_comm/event.rs	adapt	S6 event bus	Post-G2	No persistence, untyped
Memory / store_memory	ainpc_storage.rs:26	reference	—	No	Domain-specific
bot_deployments migration	migrations/20250624000000_bot_deployments.sql	reference	S6 deployment record	Post-G2	Mutable state, not append-only
CommandRouter (doc-spec)	docs/checklists/.../command.rs:289	reference	S6 tracing	Post-G2	Uncompiled
Dual Agent trait definitions	core/shared + services/agents	discard (one)	S6 trait unification	Yes	Structural duplication
Summary for ADR-018
What botgen-rust gets right that FERROS S6 should adopt:

The Task/TaskStatus/TaskHistoryEntry triple is production-quality and should be adopted verbatim as FERROS's task audit atom.
The AgentState 7-variant lifecycle enum with watch::channel shutdown broadcast maps cleanly to FERROS's lifecycle contract.
The AgentEvent vocabulary in the doc-spec layer is the right set of events for FERROS's lifecycle event log; it just needs to be promoted to a compiled crate.
The WorkQueue multi-index structure with dependency DAG and capability matching is the most mature piece of the codebase and should form the basis of FERROS S6's dispatch core.
The SchemaSpec / ainpc.yaml declarative format is a direct predecessor of the FERROS agent manifest concept.
What FERROS S6 must not inherit:

The 1000-entry history cap in QueuePersistence is an audit-trail data-loss hazard.
The commented-out persistence module leaves the entire queue ephemeral and is the single highest-risk gap.
The AgentHandle::id() hard-coding and the dual Agent trait definitions would propagate registry identity confusion.
The DeploymentManager and PgAgentRepository stub implementations (Ok(true), todo!()) create false confidence in the persistence layer.
The three lifecycle→registry→dispatch→provenance connections that botgen-rust is missing and FERROS S6 must define from scratch:

Atomic task claim (no race between concurrent workers).
Post-materialize registration of created agents into the durable registry.
A single unified event log wiring AgentEvent lifecycle transitions together with TaskHistoryEntry transitions into a queryable provenance chain per agent identity.