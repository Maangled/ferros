Home Assistant Fork Architectural Audit — Reference Material for ferros-hub
Scope of repository: This fork (Maangled/home-assistant) is a Home Assistant configuration fork — not the HA core engine. Its primary artefacts are: (1) config/ — declarative YAML configuration for a running HA instance; (2) config/custom_components/hacs/ — HACS (Home Assistant Community Store), a production-grade HA integration that exemplifies every major HA integration pattern; (3) docker-compose.yml — a multi-service deployment including Mosquitto, Piper TTS, Faster Whisper, OpenWakeWord, and LocalAI; and (4) templates/ + blueprints/ — Lovelace dashboard and automation blueprint YAML. The HACS component is by far the most architecturally rich source.

Section 1 — Entity Model
1A. Device → Entity relationship and registry identity
What HA does: A Device is a logical grouping in the device registry, identified by a tuple {(domain, unique_id)}. Entities attach to it via device_info. The DeviceEntryType.SERVICE flag marks non-physical (software-defined) devices.

Code
entity.py lines 22–32 (system_info):
    "identifiers": {(DOMAIN, HACS_SYSTEM_ID)},
    "entry_type": DeviceEntryType.SERVICE,

entity.py lines 117–124 (HacsRepositoryEntity.device_info):
    "identifiers": {(DOMAIN, str(self.repository.data.id))},
    "model": self.repository.data.category,
    "manufacturer": _manufacturer(),
A single HACS "device" (a GitHub repository) fans out into multiple typed entities: one UpdateEntity (update.py lines 31–158) and one SwitchEntity (switch.py lines 32–73), each with its own _attr_unique_id.

Why it matters for ferros-hub: A FERROS S6/S7 node is also a logical device that exposes heterogeneous capabilities. The one-device-many-entities pattern is the correct shape: the node is the device; its services (sensors, actuators, health checks) are entities. The identifier scheme decouples the address/transport layer from the logical object, which is important if nodes change network addresses.

Bucket: Reference Adapt the concept of a typed, registry-resident device identity with a domain-scoped unique ID. The categorical DeviceEntryType.SERVICE is directly applicable to FERROS virtual nodes. Discard the Python-specific DeviceEntryType enum literal; model an equivalent first-class concept in ferros-hub's type system.

1B. Entity categories and UI surfacing
What HA does: _attr_entity_category = EntityCategory.DIAGNOSTIC (switch.py line 35) marks entities that are not primary control surfaces — they appear in a separate section and can be hidden. _attr_has_entity_name = True + _attr_translation_key provide i18n-aware naming.

Why it matters for ferros-hub: Distinguishing primary operational surfaces from diagnostic or configuration surfaces reduces cognitive load in the ferros-hub UI. FERROS S6/S7 nodes will expose telemetry, control, and health data simultaneously.

Bucket: Adapt The three-tier entity category (primary / configuration / diagnostic) is worth preserving as a semantic annotation in ferros-hub. The mechanism is HA-specific; adapt conceptually.

1C. Availability as a first-class property
What HA does:

Code
entity.py lines 102–104:
    @property
    def available(self) -> bool:
        return self.hacs.repositories.is_downloaded(repository_id=...)
Availability is a live, derived property checked on every state write, not a stored flag.

Why it matters for ferros-hub: FERROS nodes can become unreachable. Making availability a derived predicate (not a stored field) ensures freshness without requiring a separate "mark unavailable" code path.

Bucket: Adapt Model availability as a derived property over live system state, not a persisted boolean.

Section 2 — Integration Semantics
2A. Setup lifecycle and staged initialization
What HA does: async_setup_entry (__init__.py lines 182–187) delegates to _async_initialize_integration, which builds the domain object, wires clients, then schedules async_try_startup. Startup is deferred to async_at_start (line 154), meaning platform forwarding happens only after HA is fully started. The stage machine (HacsStage: SETUP → WAITING → STARTUP → RUNNING) is explicit and inspectable.

Code
__init__.py lines 146–156:
    await hass.config_entries.async_forward_entry_setups(config_entry, PLATFORMS)
    hacs.set_stage(HacsStage.SETUP)
    hacs.set_stage(HacsStage.WAITING)
    async_at_start(hass=hass, at_start_cb=hacs.startup_tasks)
Why it matters for ferros-hub: FERROS nodes have their own boot ordering (S6 runit stages). An integration managing FERROS nodes also needs to distinguish between "setup complete" and "system operationally ready". A deferred startup gate prevents premature polling before dependencies (MQTT bus, device registration, supervisor) are online.

Bucket: Reference The stage machine and deferred startup are sound. Model an analogous multi-stage lifecycle for ferros-hub's integration adapter. Discard HA-specific async_at_start / ConfigEntry mechanisms.

2B. Retry and back-off on transient startup failures
What HA does:

Code
__init__.py lines 162–169:
    if not startup_result:
        if hacs.system.disabled_reason != HacsDisabledReason.INVALID_TOKEN:
            hacs.log.info("Could not setup HACS, trying again in 15 min")
            async_call_later(hass, 900, async_try_startup)
        return
A 15-minute fixed retry is used for transient failures. INVALID_TOKEN is excluded from retry (permanent error) — a notable distinction.

Why it matters for ferros-hub: Network-dependent FERROS integrations must distinguish permanent from transient failures and apply appropriate retry strategies. Retrying forever on a bad credential wastes resources and creates confusing log noise.

Bucket: Adapt The permanent-vs-transient error class distinction with different retry policies is the key pattern. Adapt; replace fixed 900s with configurable or exponential back-off suited to FERROS network characteristics.

2C. Config entry data / options split
What HA does: config_entry.data holds credentials ("token") that are immutable until an explicit reauth flow. config_entry.options holds user-tunable settings (sidepanel_title, country, appdaemon). They are merged at runtime (lines 49–55 of __init__.py) but stored separately, so options changes can trigger a reload without credential re-entry.

Code
__init__.py lines 49–55:
    hacs.configuration.update_from_dict({
        "config_entry": config_entry,
        **config_entry.data,
        **config_entry.options,
    })
config_flow.py lines 188–191 registers a dedicated OptionsFlow handler, and __init__.py line 184 installs an update listener that triggers reload on options change.

Why it matters for ferros-hub: Node credentials (certs, API keys, device tokens) should not be re-entered when adjusting operational parameters (polling intervals, log levels). A two-tier config object with explicit reauth for credentials is correct.

Bucket: Adapt Conceptually adopt the split: immutable credential tier + mutable options tier. Reload-on-options-change (where safe) is a useful UX pattern.

2D. Coordinator pattern — push vs. pull
What HA does: HACS does NOT use the standard DataUpdateCoordinator (which polls on a timer). Instead it implements BaseDataUpdateCoordinatorProtocol (coordinator.py lines 12–38): a pure listener-dispatch model. Updates are pushed by calling coordinator.async_update_listeners() at dispatch sites (e.g., base.py line 889). Per-category coordinators allow independent dispatch granularity.

Code
base.py lines 376, 422–423:
    self.coordinators: dict[HacsCategory, HacsUpdateCoordinator] = {}
    self.coordinators[category] = HacsUpdateCoordinator()

coordinator.py lines 35–38:
    def async_update_listeners(self) -> None:
        for update_callback, _ in list(self._listeners.values()):
            update_callback()
Why it matters for ferros-hub: FERROS S6/S7 likely delivers state changes via events or MQTT messages rather than polled reads. A push-style coordinator (listener registry, explicit notify_listeners()) is the right abstraction. The per-category (per-service-type) granularity limits unnecessary UI re-renders.

Bucket: Adapt Push-style coordinator with per-domain listener groups is directly applicable. The BaseDataUpdateCoordinatorProtocol shape — add_listener → remove_listener → notify — is a good abstraction to model regardless of language/runtime.

2E. Repository/service factory and validation
What HA does: async_register_repository (base.py lines 524–606) uses a REPOSITORY_CLASSES[category] factory, runs async_registration validation, collects validate.errors, and only registers on success. Validation errors are accumulated, not thrown immediately.

Code
base.py lines 558–571:
    repository = REPOSITORY_CLASSES[category](self, repository_full_name)
    if check:
        await repository.async_registration(ref)
        if repository.validate.errors:
            self.common.skip.add(repository.data.full_name)
            ...
            return repository.validate.errors
Why it matters for ferros-hub: Node capability registration in ferros-hub should similarly separate discovery (enumerate candidates) from validation (check manifest/capability contract) from registration (commit to registry). This makes each step independently testable and allows partial failures without poisoning the whole set.

Bucket: Reference Three-phase: discover → validate → register. Discard GitHub-specific validation logic; adapt the structural separation.

2F. Service exposure and dispatcher pattern
What HA does: Internal signals are published with async_dispatcher_send and consumed with async_dispatcher_connect (entity.py lines 49–57; base.py line 764). Entities subscribe in async_added_to_hass and unsubscribe via async_on_remove.

Code
entity.py lines 49–57:
    async def async_added_to_hass(self) -> None:
        self.async_on_remove(
            async_dispatcher_connect(self.hass, HacsDispatchEvent.REPOSITORY, ...)
        )
Why it matters for ferros-hub: The subscribe-on-attach / unsubscribe-on-detach lifecycle ensures no dangling callbacks after removal. This pattern is important for FERROS nodes that may be hot-added or removed.

Bucket: Reference The lifecycle-scoped subscription pattern is sound. Adapt to ferros-hub's event bus (MQTT, internal bus, or otherwise); discard HA dispatcher internals.

Section 3 — Configuration Shape
3A. YAML/UI boundary
What HA does: config/configuration.yaml (lines 1–15) is minimal — just default_config, frontend, and file includes. The substantive integration config lives in the UI (config entries). manifest.json line "config_flow": true is the machine-readable flag that moves configuration to UI. The YAML layer handles only structural wiring (include files) and cannot carry credentials.

Code
config/configuration.yaml lines 1–15:
    default_config:
    frontend:
      themes: !include_dir_merge_named themes
    automation: !include automations.yaml
Why it matters for ferros-hub: Having a thin declarative YAML layer for structural topology (which services exist, where files live) while putting dynamic/secret config in a store (credentials, runtime options) is the right separation for an IoT hub. YAML is version-controllable; credentials are not.

Bucket: Adapt The YAML-for-structure, store-for-credentials boundary is worth preserving. The HA !include directive and !include_dir_merge_named are file-composition primitives — a simpler equivalent (explicit file path config, environment variable injection) is more portable for ferros-hub.

3B. Secrets handling
What HA does: config/secrets.yaml (a flat key-value YAML) stores credential aliases. The token itself is stored in the config entry (not YAML), retrieved at runtime. Diagnostics redact the token: async_redact_data(data, ("token",)) (diagnostics.py line 80). Re-authentication triggers a guided UI flow (async_step_reauth, config_flow.py lines 174–186).

Why it matters for ferros-hub: FERROS S6/S7 credentials (device tokens, TLS keys) must never appear in diagnostics or logs. The explicit redact-list pattern is minimal and auditable.

Bucket: Adapt Maintain an explicit redaction list for any diagnostic/logging path. The reauth flow concept — separate from initial setup — is important for long-lived deployments where credentials rotate without requiring full reinstall.

3C. Options flow and runtime reconfiguration
What HA does: HacsOptionsFlowHandler (config_flow.py lines 194–225) presents a form for mutable settings. On submit it creates a new options entry which triggers async_reload_entry. It guards: if pending tasks, abort with reason="pending_tasks" (line 215).

Why it matters for ferros-hub: Live reconfiguration (changing node polling intervals, log levels) without full restart is essential for production deployments. Guarding reconfiguration against in-flight operations prevents race conditions.

Bucket: Adapt Guard-before-reconfigure is critical. The pattern of detecting in-flight operations (task queue, active downloads) and deferring or aborting reconfiguration is directly applicable to ferros-hub nodes executing critical operations.

3D. Versioned persistent storage
What HA does: HACSStore (utils/store.py lines 14–32) extends HA's Store with version checking. If the loaded version doesn't match VERSION_STORAGE, it returns None (migration signal). Writes are atomic (atomic_writes=True, line 42) and content-diffed before writing (lines 65–72).

Code
utils/store.py lines 30–32:
    if data == {} or data["version"] != self.version:
        return None
    return data["data"]
Why it matters for ferros-hub: Node state and configuration must survive process restarts and upgrades. A versioned store with explicit migration hooks prevents silent data corruption after schema changes.

Bucket: Reference Version-checked load with explicit migration path, content-diff before write (avoid unnecessary I/O), atomic writes. These are table-stakes for any persistent configuration store. Adapt to ferros-hub's persistence backend (SQLite, Redis, flat files).

Section 4 — Operational Patterns
4A. Polling vs. push and scheduled work
What HA does: HACS entities are push-only (_attr_should_poll = False, entity.py line 39). Background polling of the GitHub API is done via async_track_time_interval (base.py lines 622–653) with explicit intervals (48h, 6h, 5min, 10min). This is registered in startup_tasks and cancellation tokens stored in self.recurring_tasks for clean unload.

Code
base.py lines 622–653:
    self.recurring_tasks.append(
        async_track_time_interval(self.hass, self.async_load_hacs_from_github, timedelta(hours=48))
    )
    ...
    for task in hacs.recurring_tasks:
        task()   # cancels all on unload (__init__.py line 203)
Why it matters for ferros-hub: FERROS nodes will have heterogeneous update rates — some telemetry is event-driven, some requires periodic polling (health checks, firmware queries). The pattern of registering cancellable recurring tasks and tracking them for clean teardown is critical for proper unload/reload.

Bucket: Adapt Model a cancellable task registry (handles, not raw coroutines) with explicit unload sweep. The interval registry pattern is language-agnostic. Discard async_track_time_interval specifically; adapt the concept.

4B. Error classification and disable/enable state machine
What HA does: HacsDisabledReason (enums.py lines 65–71) enumerates all system-disabling conditions: RATE_LIMIT, REMOVED, INVALID_TOKEN, CONSTRAINS, LOAD_HACS, RESTORE. The async_github_api_method wrapper (base.py lines 491–522) maps exception types to disable reasons:

Code
base.py lines 503–521:
    except GitHubAuthenticationException:
        self.disable_hacs(HacsDisabledReason.INVALID_TOKEN)
    except GitHubRatelimitException:
        self.disable_hacs(HacsDisabledReason.RATE_LIMIT)
    except GitHubNotModifiedException as exception:
        raise exception   # NOT a disable condition — data unchanged
    except GitHubException:
        _exception = exception
async_check_rate_limit (base.py lines 891–900) periodically re-enables if the rate limit has lifted.

Why it matters for ferros-hub: FERROS nodes may go offline (network), reject connections (bad credentials), or be administratively removed. A typed disable/enable state machine makes system health observable, automation-friendly, and recoverable without manual intervention.

Bucket: Adapt Enumerate all disable conditions as distinct, named reasons. Implement exception-to-reason mapping at every external API boundary. Add auto-recovery probes for conditions that are self-healing (rate limits, network outage). Discard GitHub-specific reason values; adapt the structure.

4C. Retry logic and download resilience
What HA does: async_download_file (base.py lines 678–735) implements a manual retry loop: up to 5 timeout retries, 1-second sleep between attempts, detailed timeout logging that explicitly names the real cause (network/host problem, not HACS):

Code
base.py lines 697–726:
    while timeouts < 5:
        try:
            request = await self.session.get(url=url, timeout=ClientTimeout(total=60), ...)
            if request.status == 200:
                return await request.read()
            raise HacsException(...)
        except TimeoutError:
            timeouts += 1
            await asyncio.sleep(1)
            continue
        except BaseException as exception:
            self.log.exception("Download failed - %s", exception)
        return None
Why it matters for ferros-hub: Network calls to FERROS nodes or cloud services will time out. Fixed retry counts with sleep are the minimum; exponential backoff with jitter would be better. The explicit "not our fault" log message is a good practice for network-dependent integrations.

Bucket: Adapt Use the 5-retry-with-sleep as a minimum template; upgrade to exponential backoff with jitter for production ferros-hub. The "attribution" in log messages (whose problem is this?) is worth preserving as a logging convention.

4D. Concurrent task management
What HA does: The @concurrent(concurrenttasks=10, backoff_time=5) decorator (utils/decorator.py lines 16–43) wraps update methods with an asyncio.Semaphore. Combined with QueueManager (utils/queue_manager.py), which gathers coroutines in batches and logs per-task exceptions:

Code
queue_manager.py lines 66–68:
    result = await asyncio.gather(*local_queue, return_exceptions=True)
    for entry in result:
        if isinstance(entry, Exception):
            _LOGGER.error("<QueueManager> %s", entry)
This allows batch concurrent execution while isolating individual failures and preventing queue poisoning.

Why it matters for ferros-hub: When discovering or updating many FERROS nodes simultaneously, naive gather fails entirely if one node errors. The return_exceptions=True + per-result inspection pattern is essential.

Bucket: Adapt gather(..., return_exceptions=True) with per-result inspection is the correct primitive for fan-out operations over unreliable endpoints. Semaphore-bounded concurrency protects against network saturation. Both patterns are directly adaptable to ferros-hub.

4E. Diagnostics and system health
What HA does: Two complementary patterns:

Config-entry diagnostics (diagnostics.py lines 16–80): structured dump of integration state (stage, version, categories, repo list) with explicit token redaction. Designed for issue reporting.

System health endpoint (system_health.py lines 17–52): live checks of external URL reachability (async_check_can_reach_url) plus resource metrics (API calls remaining, stage, repo counts). Consumed by HA's Developer Tools → System Health panel.

Code
system_health.py lines 30–47:
    "GitHub API": async_check_can_reach_url(hass, BASE_API_URL, GITHUB_STATUS),
    "GitHub API Calls Remaining": response.data.resources.core.remaining,
    "Stage": hacs.stage,
    "Available Repositories": len(hacs.repositories.list_all),
    "Downloaded Repositories": len(hacs.repositories.list_downloaded),
Why it matters for ferros-hub: ferros-hub needs analogous health surfaces: per-node connectivity status, supervisor/MQTT health, credential validity, resource limits. The two-tier approach (diagnostics for debugging, health for monitoring) maps cleanly to ferros-hub's operational needs.

Bucket: Reference Two-tier: redacted diagnostic dump (for issue reports) + live health checks (for monitoring). Adapt the structure; discard HA-specific async_check_can_reach_url helpers and SystemHealthRegistration hooks.

4F. Issue registry and user-visible repair flows
What HA does: async_create_issue (repositories/integration.py lines 51–62) creates an actionable issue in HA's issue registry when a post-install restart is required. The RestartRequiredFixFlow (repairs.py lines 17–45) presents a guided repair step.

Code
repositories/integration.py lines 51–62:
    async_create_issue(
        hass=self.hacs.hass,
        domain=DOMAIN,
        issue_id=f"restart_required_{self.data.id}_{self.ref}",
        is_fixable=True,
        severity=IssueSeverity.WARNING,
        translation_key="restart_required",
    )
Why it matters for ferros-hub: FERROS node operations (firmware updates, config changes) may require user action (reboot, acknowledge). An issue registry with fixable-flow semantics surfaces these clearly rather than burying them in logs.

Bucket: Adapt Model a user-visible action registry (issue + guided fix) for ferros-hub. The is_fixable flag distinguishes informational issues from actionable ones. Discard HA's RepairsFlow machinery; adapt the conceptual separation.

4G. Reload/unload lifecycle discipline
What HA does: async_unload_entry (__init__.py lines 190–222) follows a strict order: (1) guard pending tasks, (2) clear queue, (3) cancel recurring tasks, (4) force-write state, (5) remove UI panel, (6) unload platforms, (7) reset stage, (8) mark removed, (9) pop domain from hass.data. Each step is ordered to prevent data loss and dangling references.

Code
__init__.py lines 190–222: full unload sequence
async_reload_entry (line 225–229) is simply unload + setup, relying on the correctness of each phase.

Why it matters for ferros-hub: Hot-reload/unload of node adapters without data loss or resource leaks requires the same disciplined teardown order. Skipping force-write before unload is a common source of state loss on restart.

Bucket: Reference Ordered teardown is a convention, not a mechanism. The exact ordering (guard → cancel → persist → teardown) is the adaptation target. Discard HA-specific unload APIs; preserve the ordering principle.

4H. Blueprint / automation template pattern
What HA does: blueprints/automation/homeassistant/motion_light.yaml defines a reusable automation template with typed input selectors, trigger/condition/action YAML, and execution mode (restart). Users instantiate blueprints with concrete values; the template is shared without modification.

Code
motion_light.yaml lines 9–17:
    input:
      motion_entity:
        selector:
          entity:
            filter:
              - device_class: occupancy
                domain: binary_sensor
      light_target:
        name: Light
        selector:
          target:
            entity:
              domain: light
Why it matters for ferros-hub: ferros-hub (if it exposes automation or rule logic) benefits from a template/blueprint pattern that separates the reusable logic shape from the instance-specific bindings. This avoids copy-paste rule duplication across nodes.

Bucket: Adapt The input-selector → instantiation pattern is applicable if ferros-hub has a rule/automation layer. Discard HA-specific selectors and YAML schema; adapt the blueprint-as-parameterized-template concept.

4I. Dashboard configuration shape (Lovelace YAML)
What HA does: templates/spicydash.yaml demonstrates HA's Lovelace YAML format: views with paths, themes, custom card types (custom:layout-card, custom:mushroom-*), entity references as strings (light.living_room), and !input template expansion. Dashboard state is driven by input_select entities used as routing variables.

Why it matters for ferros-hub: If ferros-hub needs a configurable dashboard layer, HA's pattern of entity-string references (decoupled from transport), per-view config, and theme separation is clean. The input_select as UI state variable pattern is clever but complex.

Bucket: Discard The Lovelace YAML format is deeply HA-specific (custom card ecosystem, Polymer/LitElement rendering, websocket state subscription). ferros-hub should not import this format. The conceptual lesson — that dashboard state can be driven by entities like any other sensor — is worth noting but does not need HA's format.

4J. Multi-service Docker Compose deployment
What HA does: docker-compose.yml (lines 1–101) co-deploys HA, Mosquitto MQTT, Piper TTS, Faster Whisper, OpenWakeWord, and LocalAI. FERROS-relevant observations: HA uses network_mode: host and privileged: true for device access; GPU is passed through to TTS/STT/LLM containers; MQTT broker is a first-class service (not embedded); NEST_SUBSCRIBER_ID environment variable is passed to HA.

Why it matters for ferros-hub: The multi-service topology (hub + MQTT + AI inference stack + hub) mirrors what ferros-hub likely needs. MQTT as an external, independently scalable broker (rather than embedded) is the right architectural separation.

Bucket: Reference External MQTT broker with HA connecting as a client is the correct deployment topology. GPU sidecar containers for inference are a pattern worth noting for FERROS's AI subsystems. network_mode: host is a common IoT necessity for mDNS/Zeroconf discovery — document this as a likely requirement for ferros-hub as well. Discard HA's specific container image and volume layout.

Summary Table
Pattern	File(s)	Bucket	Key Adaptation Note
Device→Entity identity with registry	entity.py:22-124	Reference	Domain-scoped identifier tuple; DeviceEntryType.SERVICE for virtual nodes
Entity category tiers (primary/config/diagnostic)	switch.py:35, entity.py:35-45	Adapt	Three-tier annotation, not HA enum literals
Availability as derived property	entity.py:102-104	Adapt	Live predicate, not stored flag
Staged initialization state machine	__init__.py:107-156, enums.py:57-62	Reference	Stage names; deferred startup gate
Permanent vs. transient failure retry	__init__.py:162-169	Adapt	Replace fixed 900s with exponential backoff
Config data/options split + reauth	__init__.py:49-55, config_flow.py:150-168	Adapt	Credential-immutability until explicit reauth
Push coordinator with per-domain listeners	coordinator.py:12-38, base.py:376, 889	Adapt	Push > poll for event-driven nodes
Repository factory + validation phases	base.py:524-606	Reference	Discover → validate → register separation
Lifecycle-scoped dispatcher subscriptions	entity.py:49-57	Reference	Subscribe-on-attach, unsubscribe-on-detach
YAML-for-structure, store-for-credentials	config/configuration.yaml, utils/store.py	Adapt	Thin declarative YAML + versioned credential store
Explicit diagnostics redaction list	diagnostics.py:80	Adapt	redact_data(keys_to_redact) at all diagnostic egress
Versioned store with atomic writes	utils/store.py:14-42	Reference	Version-checked load, content-diff before write
Cancellable recurring task registry	base.py:622-659	Adapt	Track cancellation handles; sweep on unload
Exception-type → disable-reason mapping	base.py:491-522, enums.py:65-71	Adapt	Named disable reasons with auto-recovery for transient
Bounded concurrent task execution	utils/decorator.py, queue_manager.py	Adapt	Semaphore + gather(return_exceptions=True)
Two-tier diagnostics/health surface	diagnostics.py, system_health.py	Reference	Debug dump (redacted) + live health checks
Issue registry + guided repair flow	repositories/integration.py:51-62, repairs.py	Adapt	User-visible actionable issues with fix steps
Ordered unload discipline	__init__.py:190-222	Reference	Guard → cancel → persist → teardown ordering
Blueprint/template parameterization	blueprints/motion_light.yaml	Adapt	If ferros-hub has automation layer
External MQTT broker deployment	docker-compose.yml:2-20	Reference	External broker topology; network_mode: host for mDNS
Lovelace dashboard YAML	templates/spicydash.yaml	Discard	HA-specific card ecosystem; not portable
GPU sidecar for inference	docker-compose.yml:30-101	Reference	Pattern for FERROS AI subsystems
!include YAML composition	config/configuration.yaml:7-9	Discard	HA-specific; use explicit path config in ferros-hub
Overall Guidance
The HACS custom component is the highest-value reference artefact in this repo: it implements every major HA integration pattern in a production-quality way. The top three takeaways for ferros-hub architectural adaptation are:

The state machine matters more than the framework. HA's HacsStage, HacsDisabledReason, and lifecycle ordering are patterns expressible in any language/runtime. The exact asyncio/Python mechanics are discardable; the logical structure is not.

Push coordinators + cancellable task registries are the correct primitives for FERROS. Polling-based updates create unnecessary coupling to network availability. FERROS S6/S7's event-driven nature aligns with HACS's push-style coordinator, not the standard DataUpdateCoordinator.

Config layering (immutable credentials + mutable options + versioned store + diagnostic redaction) is a non-negotiable production requirement. FERROS deployments will run unattended; the credential lifecycle and diagnostic safety patterns from HACS should be modeled from the start, not retrofitted.