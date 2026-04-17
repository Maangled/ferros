# ADR-013: Legacy Integration Strategy

## Status
Accepted

## Context

FERROS is a clean-start rebuild. Three predecessor projects contain useful design
patterns, API shapes, and hard-won lessons that should inform FERROS's architecture
without being forked wholesale into it.

### Predecessor repos

**botgen-rust** — Modular Discord bot platform (Rust workspace). Structure: `core/`
(cache, database, gateway, routing, logging, shared) and `services/` (agents,
llm_client, moderation, storage, generator, render, discord). Key patterns: Agent
trait (init → handle_message → shutdown), command bus with transaction-wrapped audit
trails, service registry, bot template engine.

**sheetgen-rust** — AI-powered architectural documentation backend (Axum + PostgreSQL
+ GPT-4 function-call interface). Key patterns: YAML-driven command definitions with
auto-generated tests, command bus routing, domain/storage/API three-layer separation,
recursive dependency resolution with caching.

**workpace-rust** — Prototype card website with 16+ modules (home, hud, agents,
voting, task_lists, user_settings, indexeddb, smart_home, pagedata, ui_components,
websockets, etc.). Key patterns: Module decomposition matching FERROS surfaces, WASM
build pipeline (Rust → WASM → browser), IndexedDB storage layer.

## Decision

### 1. Port interfaces, not implementations

Extract design patterns and API shapes from the predecessor repos. Do not port
Postgres-specific code, Docker configs, or Discord-specific integrations. The storage
patterns (repository pattern, transaction wrapping, audit logging) are valuable; the
platform-specific implementations are not.

### 2. No server dependencies in Phase 0

FERROS Phase 0 is `file://` with localStorage. Postgres, Redis, S3/Tigris — none of
these apply until post-Phase 0. Any pattern from a predecessor repo that requires a
server is deferred until the appropriate wave.

### 3. Anti-pattern: premature backend generation

workpace-rust "got too complicated having everything generate from the Rust back end."
This is formally recorded as the reason FERROS uses HTML-first prototyping with
contract-driven development. Do not allow the Forge or Arena to generate everything
from Rust before the contracts are stable.

### 4. Schema-first, always

Both botgen and sheetgen independently converged on schema-driven development. FERROS
extends this: JSON schemas (C1–C10) are the single source of truth. They generate JS
validators (via `generate-harness-constants.ps1`), and will eventually generate Rust
structs for the OS layer. One schema, multiple targets.

### 5. Wave-aligned integration

Legacy patterns are only ported when the FERROS wave that needs them is active. No
premature ports. The tracking table below is the authoritative mapping.

## Pattern-to-Wave Mapping

| Pattern | Source Repo | Target Wave | FERROS Component |
|---------|------------|-------------|-----------------|
| Agent trait (init/handle/shutdown) | botgen-rust `core/shared/src/agent.rs` | Wave 3 (P1–P3) | Agent Command Center, C8 runtime host |
| Command bus + audit trail | botgen-rust `docs/architecture.md` | Wave 3 (P1) | C7 audit record enforcement |
| Domain/storage/API layering | sheetgen-rust `src/{domain,storage,api}/` | Wave 1 (V5–V7) | FerrosCore extraction from monolith |
| YAML→test auto-generation | sheetgen-rust `src/gpt_interface/definitions.yaml` | Wave 0 (extend existing) | Harness drift detection |
| Template engine + generator | botgen-rust `bots/botgen_bot/src/template_engine.rs` | Wave 1 Track B (V5–V7) | Forge Workbench card authoring |
| Recursive dependency resolution | sheetgen-rust `tests/test_utils/build/dependencies.rs` | Wave 1 Track B (V5) | Card→Template→Identity chains |
| Work queue/scheduling | botgen-rust `WORK_QUEUE_IMPLEMENTATION_COMPLETE.md` | Wave 2 (S1) | Schedule Ledger |
| Voting/ranking mechanics | workpace-rust `modules/voting/` | Wave 2 (S2) | Arena ranking system |
| Full agent routing + registry | botgen-rust `core/routing/`, `services/agents/` | Wave 3 (P1–P3) | Agent Command Center |
| Moderation service | botgen-rust `services/moderation/` | Wave 3 (P2) | Permission model enforcement |
| WASM build pipeline | workpace-rust `build-wasm.sh` | Research track | Browser-side contract validators |
| LLM client service | botgen-rust `services/llm_client/` | Post-Phase 0 | AI agent hosting layer |
| Module decomposition map | workpace-rust `modules/` | Reference only | Surface architecture validation |

## Consequences

### Positive

- Prevents repeating the complexity trap from workpace-rust.
- Ensures legacy knowledge is captured formally before the source repos diverge.
- Keeps Phase 0 scope tight — no server dependencies, no premature Rust code.

### Negative

- Some useful code (e.g., botgen's full agent implementation) will not be ported for
  months. This is intentional.

### Risks

If legacy repos are archived or diverge before their patterns are ported, the source
context is lost. Mitigation: this ADR and the integration tracking in
[PROGRESS.md](../progress/PROGRESS.md) capture the mapping now, independent of whether
the source repos remain accessible.

## Related

- [ADR-014](./ADR-014-three-layer-decomposition.md) — three-layer decomposition for
  future Rust code, derived from sheetgen-rust
- [ADR-0001](./ADR-0001-start-new-do-not-fork.md) — the founding decision to build
  FERROS from scratch rather than forking any predecessor
- [Legacy Repo Reference](../legacy/README.md)

---

## Wave 0 Closure Addendum (2026-04-17)

**Added:** PR 6 — Docs/ADR reconciliation

### Wave 0 closure state: no legacy ports

Wave 0 (PRs 1–5) closed with **zero legacy ports**. No patterns from botgen-rust, sheetgen-rust, or workpace-rust were ported into Wave 0 scope. This is correct and intentional per the wave-aligned integration rule.

The only Wave 0 legacy item was **L1 (harness drift detection)** from the tracking table, which was fulfilled indirectly: `tools/generate-harness-constants.ps1` and `tools/generate-ferros-core.ps1` implement deterministic regeneration, and a clean diff after regeneration is the drift detection check. No YAML→test auto-generation from sheetgen-rust was ported — the PowerShell generators serve the same function in the Wave 0 HTML-first context.

### Boundary between Wave 0 contract spine and Wave 1+ legacy integration

The Wave 0 contract spine is:

```
schemas/          (C1–C10 JSON schemas)
schemas/fixtures/ (golden fixtures)
docs/contracts/   (contract documents + manifest)
docs/assets/_core/ferros-core.js  (shared IIFE core)
harnesses/        (H1–H8 proof harnesses)
```

**No legacy patterns cross this boundary in Wave 0.** The first legacy port is L4 (three-layer decomposition from sheetgen-rust), which activates when the first Rust source file lands in Wave 1.

The tracking table in PROGRESS.md (Legacy Integration Tracking section) remains the authoritative record of which legacy patterns are ready to port at which wave.
