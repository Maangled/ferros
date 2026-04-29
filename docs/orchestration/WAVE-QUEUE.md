# FERROS Wave Queue — Code Track

This is the **code track** queue (`track: code`). It feeds the local driver for all code, shell, and code-adjacent docs waves. For system-track (legal/ledger/asset/onramp) work, see `docs/orchestration/SYSTEM-QUEUE.md`. For hardware-track work, see `docs/orchestration/HARDWARE-QUEUE.md`.

Process one wave per invocation (Interactive Mode) unless the user explicitly requests Batch Mode. See `docs/orchestration/BATCH-MODE.md` for Batch Mode rules.

## Queue item schema

Required fields: `Title`, `Status`, `Priority`, `Gate`, `Owning streams`, `Goal`, `Anchor files`, `Validation`, `Constraints`, `Last update`

Optional fields (additive, layered on top of the existing field order without breaking it):
- `size: S | L` — S means ≤3 anchor files, single stream, single-crate or docs-only. L means multi-crate, multi-stream, or schema-touching. Batch Mode default consumes only S.
- `parallel-safe-with: [WAVE-IDs]` — explicit non-overlap declarations.
- `serial-after: WAVE-ID` — must wait for a prior wave to complete.
- `solo: true | false` — must run alone (truth-sync, gate close, schema freeze, shared truth surfaces).
- `track: code | system | hardware` — which queue this belongs to.

## Ready

### WAVE-2026-04-29-63

- Title: Sync contracts overview after local onramp rehearsal packet
- Status: ready
- Priority: P2
- Gate: rolling shared-contract truth-sync after local onramp rehearsal packet
- Owning streams: S8 primary, S7 awareness, S4 awareness, S6 awareness
- Goal: Reconcile `docs/contracts/CONTRACTS-OVERVIEW.md` with the landed local onramp rehearsal packet so the shared contract index records the `onramp-proposal.schema.json` contract and the additive `hubOnrampProposal` runway-summary seam without blurring owner boundaries or overstating the consent, transport, or gate posture.
- Anchor files: `docs/contracts/CONTRACTS-OVERVIEW.md`
- Validation: `get_errors` clean on `docs/contracts/CONTRACTS-OVERVIEW.md`; claim rationalizer confirms no overclaim
- Constraints: Shared truth only. `docs/contracts/CONTRACTS-OVERVIEW.md` only. Preserve owner split between the S6 proposal model, the S7 emitted local artifact, and the S4/S5 read-only observation seams. No crate, schema, harness, or site edits. No G4 closure, no accept/reject flow, no canonical mutation, no remote transport, no Home Assistant proof, and no physical-device evidence.
- Last update: 2026-04-29
- size: S
- serial-after: WAVE-2026-04-29-62
- solo: true
- track: code

## In Progress

None.

## Blocked

None.

## Done

### WAVE-2026-04-29-62

- Title: Serial truth-sync after local onramp rehearsal packet
- Status: done
- Priority: P2
- Gate: rolling truth-sync after local onramp rehearsal owner waves
- Owning streams: S8 primary, S7 awareness, S5 awareness, S4 awareness, S6 awareness
- Goal: Reconcile only the minimal truthful shared surfaces after the local onramp rehearsal packet lands so the repo records exactly what is now true about proposed-material generation, read-only observation, schema/harness coverage, and xtask rehearsal without overstating the consent, transport, or gate posture.
- Anchor files: `STATUS.md`, `streams/S7-hub/README.md`, `streams/S7-hub/PROGRESS.md`, `streams/S7-hub/BACKLOG.md`, `streams/S7-hub/CONTRACTS.md`, `streams/S5-ux/README.md`, `streams/S5-ux/PROGRESS.md`, `streams/S5-ux/BACKLOG.md`, `streams/S4-runtime/CONTRACTS.md`, `streams/S6-harvest/README.md`, `streams/S6-harvest/PROGRESS.md`, `streams/S6-harvest/BACKLOG.md`
- Validation: `get_errors` clean on touched truth surfaces; claim rationalizer confirms no overclaim
- Constraints: Final serial wave only. Shared truth only; no crate, schema, harness, or site edits. Record exactly what is now true and exactly what remains not true: no G4 closure, no physical-device evidence, no real Home Assistant integration, no HA dashboard proof, no canonical profile/grant mutation, no remote transport, no durable target-hardware runtime proof, and no independent install evidence.
- Last update: 2026-04-29
- size: L
- serial-after: WAVE-2026-04-29-60
- solo: true
- track: code

### WAVE-2026-04-29-60

- Title: Extend localhost acceptance harness for proposed-material observation
- Status: done
- Priority: P1
- Gate: G4 local onramp observation proof
- Owning streams: S5 primary, S7 awareness
- Goal: Extend the same-origin localhost shell acceptance harness so it proves the local onramp proposal renders as quarantined proposed material on the existing route, stays local-only and non-evidentiary, and does not expose canonicalization, grant, revoke, remote transport, Home Assistant proof, hardware evidence, or G4 closure claims.
- Anchor files: `harnesses/localhost-shell-acceptance-harness.html`
- Validation: `get_errors` clean on `harnesses/localhost-shell-acceptance-harness.html`; `cargo test -p ferros-node shell_route_serves_localhost_acceptance_harness`; direct same-origin acceptance harness run stays green with onramp proposal checks
- Constraints: Harness only. Same-origin and read-only. No new route, no privileged browser controls, no accept/reject transport, no grant/revoke controls, no remote transport, no Home Assistant integration claim, no physical-device evidence, and no G4 closure wording.
- Last update: 2026-04-29
- size: S
- serial-after: WAVE-2026-04-29-59
- track: code

### WAVE-2026-04-29-59

- Title: Render proposed bridge material in the localhost shell
- Status: done
- Priority: P1
- Gate: G4 local onramp observation surface
- Owning streams: S5 primary, S4 support, S7 awareness
- Goal: Add a narrow local onramp proposal display area to the existing shell runway panel or inspector so the operator can read source, proposal id, bridge agent, stand-in entity, requested capability/action, quarantine status, scope, evidence, and artifact path as pending consent on the current read-only route.
- Anchor files: `site/agent-center-shell.html`
- Validation: `get_errors` clean on `site/agent-center-shell.html`; relevant `cargo test -p ferros-node shell_route_` tests pass; live local shell load shows pending-consent proposed material on the existing runway route
- Constraints: Render only. The UI must make clear this is proposed material only, not accepted, not canonical, not a grant, and not Home Assistant proof. No accept/reject controls, no privileged browser controls, no grant/revoke controls, no new route, no remote transport, and no G4 closure wording.
- Last update: 2026-04-29
- size: S
- serial-after: WAVE-2026-04-29-58
- track: code

### WAVE-2026-04-29-61

- Title: Extend xtask hub-runway into a local onramp rehearsal chain
- Status: done
- Priority: P1
- Gate: G4 local onramp rehearsal helper
- Owning streams: S7 primary, S1 support, S6 awareness
- Goal: Extend `cargo xtask hub-runway` so it reuses the landed hub-owned seams to write and reload the hub snapshot, emit the local onramp proposal artifact, validate its bounded local rehearsal shape if feasible, and print a compact local-only rehearsal report without duplicating hub logic.
- Anchor files: `xtask/src/main.rs`
- Validation: `cargo check -p xtask`; `cargo xtask hub-runway`; `cargo test -p ferros-hub onramp_proposal_`; targeted `cargo test -p ferros-node onramp_`
- Constraints: Helper only. Reuse published hub-owned seams only. Keep all emitted proposal and snapshot artifacts under `.tmp/hub/`. No new node, shell, schema, or harness routes here. No remote transport, no daemon or server mode, no canonical profile or grant mutation, and no G4 closure wording.
- Last update: 2026-04-29
- size: S
- parallel-safe-with: [WAVE-2026-04-29-57, WAVE-2026-04-29-58]
- serial-after: WAVE-2026-04-29-56
- track: code

### WAVE-2026-04-29-58

- Title: Extend node runway summary with optional local onramp proposal observation
- Status: done
- Priority: P1
- Gate: G4 local onramp observation seam
- Owning streams: S4 primary, S7 support, S6 awareness
- Goal: Extend the existing read-only `/runway-summary(.json)` seam additively with an optional local onramp proposal child sourced from the hub-owned summary seam so the current same-origin runway payload can surface proposed material without reading random files directly.
- Anchor files: `crates/ferros-node/src/lib.rs`
- Validation: targeted `cargo test -p ferros-node onramp_`; existing runway summary and hub restart tests still pass; `cargo check -p ferros-node`
- Constraints: Additive child only on the existing read-only route. Consume the hub-owned seam only; do not read `.tmp/hub/` files directly from node or shell. Keep it optional, read-only, same-origin, and display-only. No remote transport, no daemon or server mode, no canonical profile or grant mutation, and no G4 closure wording.
- Last update: 2026-04-29
- size: S
- parallel-safe-with: [WAVE-2026-04-29-57, WAVE-2026-04-29-61]
- serial-after: WAVE-2026-04-29-56
- track: code

### WAVE-2026-04-29-57

- Title: Add schema and contract-validator coverage for the local onramp proposal
- Status: done
- Priority: P1
- Gate: G4 local onramp contract validation
- Owning streams: S7 primary, S1 support, S6 awareness
- Goal: Add one bounded local-only schema for the onramp proposal artifact, regenerate harness constants, and add positive and negative H1 validator coverage for quarantined proposed material without widening frozen S2 or canonical FERROS state contracts.
- Anchor files: `crates/ferros-hub/tests/local_bridge.rs`, `schemas/onramp-proposal.schema.json`, `harnesses/_constants.js`, `harnesses/ferros-contract-validator.html`
- Validation: schema checks; `powershell -NoProfile -ExecutionPolicy Bypass -File tools/generate-harness-constants.ps1`; direct file-based run of `harnesses/ferros-contract-validator.html` remains green; targeted `cargo test -p ferros-hub onramp_proposal_`
- Constraints: New local-only schema only. Do not touch frozen S2 schemas. Do not widen existing hub-local schemas unless a validator-proven blocker forces replanning. Cover banned wording, remote-looking text, invalid paths, and accidental canonical/grant claims. No partner-facing claim, no remote transport, no Home Assistant integration claim, no physical-device evidence, and no G4 closure wording.
- Last update: 2026-04-29
- size: L
- parallel-safe-with: [WAVE-2026-04-29-58, WAVE-2026-04-29-61]
- serial-after: WAVE-2026-04-29-56
- track: code

### WAVE-2026-04-29-56

- Title: Emit local onramp proposal artifact from ferros-hub
- Status: done
- Priority: P1
- Gate: G4 local onramp rehearsal owner seam
- Owning streams: S7 primary, S6 support
- Goal: Extend the local hub proof path so the simulated bridge entity emits one bounded `.tmp/hub/local-onramp-proposal.json` artifact derived from the current bridge proof, keeping that artifact quarantined, pending consent, local-only, and non-evidentiary without mutating canonical profile, grants, or accepted state.
- Anchor files: `crates/ferros-hub/Cargo.toml`, `crates/ferros-hub/src/ha_bridge.rs`, `crates/ferros-hub/tests/local_bridge.rs`
- Validation: `cargo test -p ferros-hub onramp_proposal_`; `cargo run -p ferros-hub -- prove-bridge`; confirm artifact contents and banned wording checks; `cargo check -p ferros-hub`
- Constraints: Hub owner wave only. Consume the published W55 proposal model. Keep all proposal artifacts under `.tmp/hub/`. Do not mutate profile, grants, S2 schemas, agent lifecycle, or canonical state. No node, shell, schema, harness, or xtask edits here. No remote transport, no daemon or server mode, and no G4 closure wording.
- Last update: 2026-04-29
- size: L
- serial-after: WAVE-2026-04-29-55
- track: code

### WAVE-2026-04-29-55

- Title: Add local onramp proposal model for proposed bridge material
- Status: done
- Priority: P1
- Gate: G4 local onramp rehearsal model
- Owning streams: S6 primary, S7 support
- Goal: Add a bounded local proposed-material model in the existing stream-owned data/onramp surface so one simulated bridge entity can be represented as quarantined, non-canonical proposed material rather than canonical profile state or a grant.
- Anchor files: `crates/ferros-data/src/lib.rs`
- Validation: targeted cargo tests for `onramp_proposal_`; `cargo check` on touched crates
- Constraints: The model must include source, proposal id, bridge agent name, stand-in entity name, requested capability/action, quarantine status, scope, evidence, and local artifact path. Validation must reject remote-looking URLs, hardware/proof/launch wording, accepted/canonical/granted wording, and malformed local paths. No hub, node, shell, schema, harness, or xtask edits here. No remote transport, no daemon or server mode, and no canonical profile or grant mutation.
- Last update: 2026-04-29
- size: S
- serial-after: WAVE-2026-04-29-54
- track: code

### WAVE-2026-04-29-54

- Title: Sync the cross-stream contracts overview after the restart-aware local hub runway packet
- Status: done
- Priority: P2
- Gate: rolling contracts truth-sync after W53
- Owning streams: S8 primary, S7 awareness, S4 awareness
- Goal: Reconcile `docs/contracts/CONTRACTS-OVERVIEW.md` with the already-authoritative S7 and S4 contract docs so the cross-stream index names the owner-backed S7 local hub restart snapshot contract and the additive S4-owned `hubRestart` child on the existing read-only `/runway-summary(.json)` seam without widening the claim boundary.
- Anchor files: `docs/contracts/CONTRACTS-OVERVIEW.md`
- Validation: `get_errors` clean on touched file; diff review confirms no G4 closure, no hardware evidence claim, no Home Assistant integration claim, no remote transport claim, no daemon or server mode claim, and no durable published hub restart API claim
- Constraints: Overview only. Do not reopen `STATUS.md`, stream `CONTRACTS.md` files, schemas, harnesses, or crate files. Keep the S7 packet local-only, non-evidentiary, and non-gate-closing.
- Last update: 2026-04-29
- size: S
- serial-after: WAVE-2026-04-29-53
- solo: true
- track: code

### WAVE-2026-04-29-53

- Title: Final serial truth-sync after the restart-aware local hub runway packet
- Status: done
- Priority: P2
- Gate: rolling truth-sync after S7 owner waves
- Owning streams: S8 primary, S7 awareness, S4 awareness
- Goal: Reconcile the minimum truthful shared surfaces after the restart-aware local hub snapshot, node runway observation, shell visibility, schema validation, acceptance harness, and xtask helper slices land, while stating exactly what still remains open.
- Anchor files: `STATUS.md`, `streams/S7-hub/README.md`, `streams/S7-hub/PROGRESS.md`, `streams/S7-hub/BACKLOG.md`, `streams/S7-hub/CONTRACTS.md`, `streams/S4-runtime/CONTRACTS.md`
- Validation: `get_errors` clean on touched truth surfaces; diff review confirms no G4 closure, no hardware evidence claim, no Home Assistant integration claim, no remote transport claim, and no daemon or server mode claim
- Constraints: Final serial wave only. Shared truth only; no crate, schema, harness, or site edits. Do not close G4, do not claim physical-device evidence, do not claim Home Assistant integration, and keep queue and run-log updates append-only.
- Last update: 2026-04-29
- size: L
- serial-after: WAVE-2026-04-29-52
- solo: true
- track: code

### WAVE-2026-04-29-52

- Title: Extend xtask hub-runway helper to prove snapshot write and reload
- Status: done
- Priority: P1
- Gate: G4 local tooling proof helper
- Owning streams: S7 primary, S1 support
- Goal: Keep `cargo xtask hub-runway` aligned to the restart-aware hub seam so it proves the same local snapshot write and reload path and prints the same local-only, non-evidentiary hub observation without duplicating hub logic.
- Anchor files: `xtask/src/main.rs`
- Validation: `cargo check -p xtask`; `cargo xtask hub-runway`; `cargo run -p ferros-hub -- summary`
- Constraints: Helper only. Reuse the published hub seam from WAVE-2026-04-29-47. Do not reopen hub source files or add node, shell, schema, or harness routes here. Keep all output local-only and non-evidentiary under `.tmp/hub/`. No remote transport, no daemon or server mode, and no G4 closure wording.
- Last update: 2026-04-29
- size: S
- parallel-safe-with: [WAVE-2026-04-29-48, WAVE-2026-04-29-49, WAVE-2026-04-29-50, WAVE-2026-04-29-51]
- serial-after: WAVE-2026-04-29-47
- track: code

### WAVE-2026-04-29-51

- Title: Add schema-backed validation for the local hub restart snapshot contract
- Status: done
- Priority: P1
- Gate: G4 local restart contract validation
- Owning streams: S7 primary, S1 support
- Goal: Add one bounded local-only schema for the persisted hub restart snapshot and wire H1 validator coverage so the restart-aware local hub state stays schema-backed and drift-resistant without widening into a partner-facing or hardware-facing contract.
- Anchor files: `crates/ferros-hub/tests/local_bridge.rs`, `schemas/hub-local-state-snapshot.schema.json`, `harnesses/_constants.js`, `harnesses/ferros-contract-validator.html`
- Validation: `cargo test -p ferros-hub --test local_bridge hub_state_`; `powershell -NoProfile -ExecutionPolicy Bypass -File tools/generate-harness-constants.ps1`; direct file-based run of `harnesses/ferros-contract-validator.html` stays green with explicit restart snapshot positive and negative cases
- Constraints: Schema and validator only. Do not touch frozen S2 schemas. Use only the current validator subset. Do not widen `hub-local-runway-report` or `hub-local-bridge-artifact` unless a validator-proven blocker forces replanning. No partner-facing claim, no remote transport, no Home Assistant integration claim, no physical-device evidence, and no G4 closure wording.
- Last update: 2026-04-29
- size: L
- parallel-safe-with: [WAVE-2026-04-29-48, WAVE-2026-04-29-49, WAVE-2026-04-29-50, WAVE-2026-04-29-52]
- serial-after: WAVE-2026-04-29-47
- track: code

### WAVE-2026-04-29-50

- Title: Extend localhost shell acceptance harness for restart-aware hub runway observation
- Status: done
- Priority: P1
- Gate: G4 local shell acceptance proof
- Owning streams: S5 primary
- Goal: Extend the same-origin localhost shell acceptance harness so it proves the restart-aware hub observation is read through the existing `/runway-summary.json` seam and rendered without opening a new shell route.
- Anchor files: `harnesses/localhost-shell-acceptance-harness.html`
- Validation: `cargo test -p ferros-node shell_route_serves_localhost_acceptance_harness`; direct same-origin run of `/harnesses/localhost-shell-acceptance.html` against `ferros-node shell` stays green with restart-aware runway checks
- Constraints: Harness only. Keep observation same-origin and read-only. No new route, no privileged browser controls, no remote transport, no Home Assistant integration claim, no physical-device evidence, and no G4 closure wording.
- Last update: 2026-04-29
- size: S
- parallel-safe-with: [WAVE-2026-04-29-51, WAVE-2026-04-29-52]
- serial-after: WAVE-2026-04-29-49
- track: code

### WAVE-2026-04-29-49

- Title: Render local hub restart observation in the localhost shell runway panel
- Status: done
- Priority: P1
- Gate: G4 local shell runway observation
- Owning streams: S5 primary, S4 support
- Goal: Render the additive hub restart-aware fields already exposed through `/runway-summary.json` inside the existing runway panel and inspector so the operator sees restart-state observation on the same read-only shell path without new routes or privileged controls.
- Anchor files: `site/agent-center-shell.html`
- Validation: `cargo test -p ferros-node shell_route_gets_local_runway_summary_json`; `get_errors` clean on `site/agent-center-shell.html`; live local load of `ferros-node shell` shows hub restart observation through the existing runway panel and inspector
- Constraints: UI and read-only copy only. Treat `hubRestart` as optional and `reloadStatus` as display-only runway context; do not translate it into durability, power-cycle, hardware, or gate evidence. No new route, no browser-issued hub writes, no privileged browser controls, no remote transport, no Home Assistant integration claim, no physical-device evidence, and no G4 closure wording.
- Last update: 2026-04-29
- size: S
- parallel-safe-with: [WAVE-2026-04-29-51, WAVE-2026-04-29-52]
- serial-after: WAVE-2026-04-29-48
- track: code

### WAVE-2026-04-29-48

- Title: Extend ferros-node runway-summary payload with optional hub restart observation
- Status: done
- Priority: P1
- Gate: G4 local runway observation seam
- Owning streams: S4 primary, S7 support
- Goal: Consume the published restart-aware hub summary from `ferros-hub` inside the existing `/runway-summary(.json)` seam so the local node surface exposes additive hub restart observation through the current profile-scoped, read-only runway payload rather than a new route.
- Anchor files: `Cargo.lock`, `crates/ferros-node/Cargo.toml`, `crates/ferros-node/src/lib.rs`
- Validation: `cargo test -p ferros-node local_agent_api_runway_summary_serializes_and_tracks_profile_and_deny_observation`; `cargo test -p ferros-node shell_route_gets_local_runway_summary_json`; `cargo test -p ferros-node local_agent_api_runway_summary_omits_hub_restart_when_hub_summary_loader_fails`; `cargo check -p ferros-node`
- Constraints: Extend the existing `/runway-summary.json` payload additively only. Consume `restart_observation` from the default hub summary seam only; do not read the snapshot file directly or depend on the snapshot-path override helper. No daemon or server mode beyond the current localhost shell host, no remote transport, no privileged browser controls, no Home Assistant integration claim, no physical-device evidence, and no G4 closure wording.
- Last update: 2026-04-29
- size: L
- parallel-safe-with: [WAVE-2026-04-29-51, WAVE-2026-04-29-52]
- serial-after: WAVE-2026-04-29-47
- track: code

### WAVE-2026-04-29-47

- Title: Integrate restart-aware local hub snapshot reload into the typed runtime summary
- Status: done
- Priority: P1
- Gate: G4 local restart-aware runway
- Owning streams: S7 primary
- Goal: Wire the local hub snapshot seam into the existing typed hub runtime summary so summary and prove flows can record and reload restart-aware local runway state, default safely when no snapshot exists, and report bounded local reload status without widening into a public restart API.
- Anchor files: `crates/ferros-hub/src/lib.rs`, `crates/ferros-hub/src/ha_bridge.rs`, `crates/ferros-hub/tests/local_bridge.rs`
- Validation: `cargo test -p ferros-hub --test local_bridge hub_reload_`; `cargo run -p ferros-hub -- summary`; `cargo run -p ferros-hub -- prove-bridge`; `cargo check -p ferros-hub`; `cargo test -p ferros-hub --test local_bridge hub_summary_`
- Constraints: Consume only the WAVE-2026-04-29-46 snapshot seam. Keep persistence under `.tmp/hub/` and keep wording local-only, non-evidentiary, and non-gate-closing. No `ferros-node`, shell, schema, harness, or `xtask` edits.
- Last update: 2026-04-29
- size: S
- serial-after: WAVE-2026-04-29-46
- track: code

### WAVE-2026-04-29-46

- Title: Add typed local hub state snapshot model and bounded path guardrails
- Status: done
- Priority: P1
- Gate: G4 local restart-aware runway
- Owning streams: S7 primary
- Goal: Add a typed local hub state snapshot model under `crates/ferros-hub` that persists only local, non-evidentiary runway state under `.tmp/hub/`, capturing bridge manifest identity, policy decision label, artifact path, scope, evidence status, and last local summary while rejecting absolute paths, parent traversal, remote-looking URLs, hardware or proof or launch wording, and malformed local state.
- Anchor files: `crates/ferros-hub/src/lib.rs`, `crates/ferros-hub/src/ha_bridge.rs`, `crates/ferros-hub/tests/local_bridge.rs`
- Validation: `cargo test -p ferros-hub --test local_bridge hub_state_`; `cargo check -p ferros-hub`; `cargo test -p ferros-hub --test local_bridge bridge_`
- Constraints: Core owner wave only. Keep persisted output under `.tmp/hub/` and keep all wording local-only and non-evidentiary. No `ferros-node`, shell, schema, harness, or `xtask` edits. No remote transport, no daemon or server mode, no privileged browser controls, no Home Assistant integration claim, no physical-device evidence, and no G4 closure wording.
- Last update: 2026-04-29
- size: S
- serial-after: WAVE-2026-04-28-45
- track: code

### WAVE-2026-04-28-38

- Title: Promote ferros-hub from binary-backed scaffold to library-backed local runway crate
- Status: done
- Priority: P1
- Gate: G4 local executable runway
- Owning streams: S7 primary
- Goal: Add `crates/ferros-hub/src/lib.rs`, move reusable hub and bridge types into library-backed posture, and keep the binary thin without widening into hardware, Home Assistant, or remote transport claims.
- Anchor files: `crates/ferros-hub/src/lib.rs`, `crates/ferros-hub/src/main.rs`, `crates/ferros-hub/src/ha_bridge.rs`, `crates/ferros-hub/tests/local_bridge.rs`
- Validation: `cargo check -p ferros-hub`; `cargo test -p ferros-hub --test local_bridge`
- Constraints: Library promotion only. Keep local-only and non-evidentiary wording. No remote transport, no hardware evidence, no Home Assistant proof, and no G4 closure wording.
- Last update: 2026-04-28
- size: L
- track: code

### WAVE-2026-04-28-39

- Title: Replace crate-local bridge registration with ferros-agents registry primitives
- Status: done
- Priority: P1
- Gate: G4 local bridge composition
- Owning streams: S7 primary, S3 support
- Goal: Consume `AgentManifest`, `AgentName`, `AgentRegistry`, and `InMemoryAgentRegistry` from `ferros-agents` so the local bridge is represented as a real FERROS local manifest instead of only a crate-local registration struct.
- Anchor files: `Cargo.lock`, `crates/ferros-hub/Cargo.toml`, `crates/ferros-hub/src/lib.rs`, `crates/ferros-hub/src/ha_bridge.rs`, `crates/ferros-hub/tests/local_bridge.rs`
- Validation: `cargo test -p ferros-hub --test local_bridge bridge_agent_registers_locally`; `cargo test -p ferros-hub --test local_bridge`
- Constraints: Consume existing S3 primitives only. Do not edit `crates/ferros-agents/**`. Keep one bridge agent and one simulated stand-in entity. No lifecycle, RPC, remote transport, or Home Assistant proof widening.
- Last update: 2026-04-28
- size: L
- serial-after: WAVE-2026-04-28-38
- track: code

### WAVE-2026-04-28-40

- Title: Replace local hub capability checks with ferros-core policy primitives
- Status: done
- Priority: P1
- Gate: G4 local policy composition
- Owning streams: S7 primary, S4 support, S2 awareness
- Goal: Consume `Capability`, `CapabilityRequest`, `DenyByDefaultPolicy`, `PolicyDecision`, and `PolicyDenialReason` from `ferros-core` and drive local hub authorization from real FERROS policy primitives using `ferros_profile::CapabilityGrant` inputs.
- Anchor files: `Cargo.lock`, `crates/ferros-hub/Cargo.toml`, `crates/ferros-hub/src/lib.rs`, `crates/ferros-hub/src/ha_bridge.rs`, `crates/ferros-hub/tests/local_bridge.rs`
- Validation: `cargo test -p ferros-hub --test local_bridge bridge_policy_`; `cargo check -p ferros-hub`
- Constraints: Consume existing S4 and S2 primitives only. Do not edit `crates/ferros-core/**` or `crates/ferros-profile/**`. Keep proof local-only and non-evidentiary. No frozen schema touch, no remote transport, and no privilege widening.
- Last update: 2026-04-28
- size: L
- serial-after: WAVE-2026-04-28-39
- track: code

### WAVE-2026-04-28-41

- Title: Add typed local hub runtime summary model
- Status: done
- Priority: P1
- Gate: G4 local runtime reporting
- Owning streams: S7 primary, S4 awareness
- Goal: Add a deterministic local runtime summary that composes bridge registration state, policy decision, emitted artifact path, local-only scope, non-evidentiary evidence status, and deny/error summary without widening into remote transport or hardware claims.
- Anchor files: `crates/ferros-hub/src/lib.rs`, `crates/ferros-hub/src/ha_bridge.rs`, `crates/ferros-hub/tests/local_bridge.rs`
- Validation: `cargo check -p ferros-hub`; `cargo test -p ferros-hub --test local_bridge hub_summary_`
- Constraints: Local report only. Keep the model deterministic, local-only, and non-evidentiary. No JSON-RPC, no node shell route, no Home Assistant dashboard proof, and no gate-close wording.
- Last update: 2026-04-28
- size: S
- serial-after: WAVE-2026-04-28-40
- track: code

### WAVE-2026-04-28-42

- Title: Add thin ferros-hub CLI proof commands for local runway summary and bridge proof
- Status: done
- Priority: P1
- Gate: G4 local CLI proof surface
- Owning streams: S7 primary
- Goal: Extend the `ferros-hub` binary with a narrow proof CLI for local summary, bridge artifact emission, deny-path demonstration, and runtime summary print while keeping the binary a thin wrapper over reusable local hub logic.
- Anchor files: `crates/ferros-hub/src/lib.rs`, `crates/ferros-hub/src/main.rs`, `crates/ferros-hub/tests/local_bridge.rs`
- Validation: `cargo run -p ferros-hub -- summary`; `cargo run -p ferros-hub -- prove-bridge`; `cargo run -p ferros-hub -- deny-demo`; `cargo test -p ferros-hub --test local_bridge`
- Constraints: Thin proof CLI only. No daemon or server mode. Keep outputs local-only and non-evidentiary under `.tmp/hub/`. No Home Assistant proof, no hardware evidence, no remote transport, and no stable remote-facing contract widening.
- Last update: 2026-04-28
- size: S
- serial-after: WAVE-2026-04-28-41
- track: code

### WAVE-2026-04-28-43

- Title: Add schema-backed local hub artifact and report validation
- Status: done
- Priority: P1
- Gate: G4 local report contract validation
- Owning streams: S7 primary, S1 support
- Goal: Add a bounded local schema and harness-backed validator coverage for the hub runtime report and emitted local artifact so the local hub contract is executable and drift-resistant without widening into a partner-facing or hardware-facing surface.
- Anchor files: `crates/ferros-hub/tests/local_bridge.rs`, `schemas/hub-local-runway-report.schema.json`, `schemas/hub-local-bridge-artifact.schema.json`, `harnesses/_constants.js`, `harnesses/ferros-contract-validator.html`
- Validation: `cargo test -p ferros-hub --test local_bridge`; `powershell -NoProfile -ExecutionPolicy Bypass -File tools/generate-harness-constants.ps1`; direct file-based run of `harnesses/ferros-contract-validator.html` stays green with explicit local hub report and artifact positive cases
- Constraints: Local-only and non-evidentiary contract only. Do not touch frozen S2 schemas. Keep production hub code unchanged unless a validator-proven blocker forces replanning. Use the validator's supported subset only. No Home Assistant proof, no hardware evidence, no remote transport, and no partner-facing schema claim.
- Last update: 2026-04-28
- size: L
- serial-after: WAVE-2026-04-28-42
- track: code

### WAVE-2026-04-28-44

- Title: Add xtask hub-runway helper for local proof execution
- Status: done
- Priority: P1
- Gate: G4 local tooling proof helper
- Owning streams: S7 primary, S1 support
- Goal: Add a focused xtask helper such as `cargo xtask hub-runway` that calls the existing hub library proof seam directly and confirms the local `.tmp/hub` artifact without duplicating core proof logic.
- Anchor files: `Cargo.lock`, `xtask/Cargo.toml`, `xtask/src/main.rs`
- Validation: `cargo check -p xtask`; `cargo xtask hub-runway`; `cargo run -p ferros-hub -- prove-bridge`
- Constraints: Helper only. Keep outputs local-only and non-evidentiary under `.tmp/hub/`. The recursive Lane Architect pass is complete; treat `crates/ferros-hub/src/lib.rs`, `crates/ferros-hub/src/main.rs`, and `crates/ferros-hub/src/ha_bridge.rs` as stop surfaces in this wave. No queue mutation, no remote transport, and no gate-close wording.
- Last update: 2026-04-28
- size: L
- serial-after: WAVE-2026-04-28-43
- track: code

### WAVE-2026-04-28-45

- Title: Final serial truth-sync after local hub runway expansion packet
- Status: done
- Priority: P2
- Gate: rolling truth-sync after S7 owner waves
- Owning streams: S8 primary, S7 awareness
- Goal: Reconcile the minimum truthful shared surfaces after the local hub library, S3 registry composition, S4 policy composition, local report/artifact, CLI proof, schema validation, and xtask helper slices land, while stating exactly what still remains open.
- Anchor files: `STATUS.md`, `streams/S7-hub/README.md`, `streams/S7-hub/PROGRESS.md`, `streams/S7-hub/BACKLOG.md`
- Validation: `get_errors` clean on touched truth surfaces; diff review confirms no G4 closure, no hardware evidence claim, no Home Assistant dashboard proof claim, no remote transport claim, and no privilege-boundary widening
- Constraints: Final serial wave only. Shared truth only; no crate edits. Do not close G4, do not claim physical-device evidence, do not claim real Home Assistant integration, and keep queue/log updates append-only.
- Last update: 2026-04-28
- size: L
- serial-after: WAVE-2026-04-28-44
- solo: true
- track: code

### WAVE-2026-04-28-37

- Title: Serial truth-sync after non-gate-closing G4 executable runway slices
- Status: done
- Priority: P2
- Gate: rolling truth-sync after S7 owner waves
- Owning streams: S8 primary, S7 awareness
- Goal: Reconcile only the minimal S7 truth surfaces after the executable runway slices land: reflect local binary, bridge, and harness progress without promoting G4, hardware evidence, or Home Assistant proof beyond what the artifacts actually show.
- Anchor files: `STATUS.md`, `streams/S7-hub/README.md`, `streams/S7-hub/PROGRESS.md`, `streams/S7-hub/BACKLOG.md`
- Validation: `get_errors` clean on touched truth surfaces; diff review confirms no G4 closure, no hardware proof claim, and no Home Assistant integration claim
- Constraints: Final serial wave only. Do not run before WAVE-2026-04-28-36 lands. Shared truth only; no crate edits.
- Last update: 2026-04-28
- size: L
- serial-after: WAVE-2026-04-28-36
- solo: true
- track: code

### WAVE-2026-04-28-36

- Title: Focused local bridge harness proves allow deny and error paths
- Status: done
- Priority: P1
- Gate: G4 local bridge proof hardening
- Owning streams: S7 primary
- Goal: Add a focused local proof loop for the bridge seam so one simulated allow path, one denied capability path, and one error/reporting path are executable without widening into hardware or Home Assistant evidence.
- Anchor files: `crates/ferros-hub/tests/local_bridge.rs`, `crates/ferros-hub/src/ha_bridge.rs`, `.tmp/hub/`
- Validation: `cargo test -p ferros-hub bridge_`; simulated local artifact emitted under `.tmp/hub/`
- Constraints: Local-only proof only. No physical-device session, no Home Assistant dashboard claim, no gate-close wording, and no new remote-control surface.
- Last update: 2026-04-28
- size: S
- serial-after: WAVE-2026-04-28-35
- track: code

### WAVE-2026-04-28-35

- Title: Add first local-only HA bridge seam and simulated bridge artifact
- Status: done
- Priority: P1
- Gate: G4 local bridge executable runway
- Owning streams: S7 primary
- Goal: Add one bridge-agent seam inside `ferros-hub` plus a simulated local bridge artifact so S7 can prove bridge behavior through local executable surfaces before any hardware or Home Assistant claim.
- Anchor files: `crates/ferros-hub/src/main.rs`, `crates/ferros-hub/src/ha_bridge.rs`, `crates/ferros-hub/tests/local_bridge.rs`
- Validation: `cargo test -p ferros-hub bridge_agent_registers_locally`; `cargo test -p ferros-hub simulated_bridge_artifact_stays_local_only`
- Constraints: One bridge agent, one simulated entity/artifact, local-only evidence, no Home Assistant fork change, no remote transport, and no privileged-write widening.
- Last update: 2026-04-28
- size: S
- serial-after: WAVE-2026-04-28-34
- track: code

### WAVE-2026-04-28-34

- Title: Scaffold `ferros-hub` binary skeleton for G4 proof-ladder step 1
- Status: done
- Priority: P1
- Gate: G4 local executable runway
- Owning streams: S7 primary, S4 support awareness
- Goal: Revise the execution posture so subagent review is the default safety mechanism for this size-L wave, then create the first local-only `ferros-hub` crate and binary skeleton so G4 can advance from runway-only docs to an executable hub entry point without claiming hardware evidence, Home Assistant integration, or gate closure.
- Anchor files: `docs/orchestration/BATCH-MODE.md`, `docs/orchestration/LOCAL-DRIVER.md`, `Cargo.toml`, `Cargo.lock`, `crates/ferros-hub/Cargo.toml`, `crates/ferros-hub/src/main.rs`
- Validation: `cargo check -p ferros-hub`
- Constraints: Local-only scaffold only. No Home Assistant fork changes, no physical-device claim, no remote transport, no privileged-write expansion, and no G4 closure wording. Validation may refresh the root `Cargo.lock` only insofar as the local `ferros-hub` package stanza is required by the new workspace member.
- Last update: 2026-04-28
- size: L
- track: code

### WAVE-2026-04-28-33

- Title: Final serial truth-sync after runway, profile, and local-push owner waves
- Status: done
- Priority: P2
- Gate: rolling truth-sync after owner lanes
- Owning streams: S8 primary
- Goal: Reconcile shared truth only after substantive owner waves land: update the minimal set of progress and status surfaces needed to reflect the new runway, profile, shell, harness, and local-push reality without overstating D1 or G4.
- Anchor files: `STATUS.md`, `streams/S4-runtime/PROGRESS.md`, `streams/S5-ux/PROGRESS.md`, `streams/S6-harvest/PROGRESS.md`
- Validation: `get_errors` clean on all touched truth surfaces; diff review confirms no gate promotion and no hardware claim
- Constraints: Final serial wave only. Do not run before owner slices land. No gate closes, no ADR promotion, and no hardware or HA bridge claim.
- Last update: 2026-04-28
- size: L
- serial-after: WAVE-2026-04-28-32
- track: code

### WAVE-2026-04-28-32

- Title: Typed local-push envelope emission lands in burst-driven .tmp/push output
- Status: done
- Priority: P1
- Gate: G4 local push audit execution
- Owning streams: S6 primary, S1 support
- Goal: Use the typed local-push envelope boundary to emit real local envelope artifacts into `.tmp/push` and expose a focused burst helper path so queue-clear runs stop relying on markdown-only digests for that seam.
- Anchor files: `crates/ferros-data/src/lib.rs`, `xtask/src/main.rs`, `.tmp/push/`
- Validation: `cargo test -p ferros-data`; `cargo check -p xtask`; `cargo xtask burst`
- Constraints: Keep output local-only and non-partner-facing. No remote upload, no hardware session, and no frozen S2 schema touch.
- Last update: 2026-04-28
- size: L
- serial-after: WAVE-2026-04-28-20
- track: code

### WAVE-2026-04-28-25

- Title: Codify the first local profile-surface code slice in owner docs
- Status: done
- Priority: P2
- Gate: post-G3 local profile surface prep
- Owning streams: S5 primary, S2 consumer awareness
- Goal: Convert the profile-surface handoff into an owner-backed next-code slice so the queue can open a real adapter wave without reopening the frozen S2 contract or widening browser privileges.
- Anchor files: `streams/S5-ux/README.md`, `streams/S5-ux/BACKLOG.md`, `streams/S2-profile/README.md`
- Validation: `get_errors` clean on all 3 anchor files
- Constraints: Docs-only. Do not edit frozen S2 schemas. No browser grant or revoke actions and no remote profile access.
- Last update: 2026-04-28
- size: S
- parallel-safe-with: [WAVE-2026-04-28-18, WAVE-2026-04-28-19, WAVE-2026-04-28-20, WAVE-2026-04-28-21, WAVE-2026-04-28-22, WAVE-2026-04-28-23, WAVE-2026-04-28-24]
- track: code

### WAVE-2026-04-28-21

- Title: Admit local-push envelope into harness constants and contract validation
- Status: done
- Priority: P1
- Gate: G4 local push audit substrate
- Owning streams: S6 primary, S1 support
- Goal: Add the local-push audit envelope schema to the existing harness constant generation and contract validator so the new schema has an executable consumer before broader queue-clear use.
- Anchor files: `tools/generate-harness-constants.ps1`, `harnesses/_constants.js`, `harnesses/ferros-contract-validator.html`
- Validation: `get_errors` clean on touched tool and harness files; file:// run of `harnesses/ferros-contract-validator.html` stays green after regeneration
- Constraints: Additive harness admission only. Do not widen JSON Schema vocabulary unless required by the schema. No CI or shared-truth edits.
- Last update: 2026-04-28
- size: L
- parallel-safe-with: [WAVE-2026-04-28-18, WAVE-2026-04-28-19, WAVE-2026-04-28-20, WAVE-2026-04-28-22, WAVE-2026-04-28-23, WAVE-2026-04-28-24, WAVE-2026-04-28-25]
- track: code

### WAVE-2026-04-28-19

- Title: Local consent snapshot enrichment for browser-facing consumers
- Status: done
- Priority: P1
- Gate: G4 / D1 local consent runway
- Owning streams: S2 primary
- Goal: Extend the landed local consent snapshot so downstream local consumers can render grant and consent readiness without reopening frozen S2 contracts or inventing browser-local identity rules.
- Anchor files: `crates/ferros-profile/src/lib.rs`
- Validation: `cargo test -p ferros-profile local_consent_snapshot_`; `cargo test -p ferros-profile reload_boundary_load_local_profile_`
- Constraints: Do not edit `schemas/profile.v0.json` or `schemas/capability-grant.v0.json`. No browser grant or revoke mutation. Keep the surface local-only.
- Last update: 2026-04-28
- size: S
- parallel-safe-with: [WAVE-2026-04-28-18, WAVE-2026-04-28-20, WAVE-2026-04-28-21, WAVE-2026-04-28-22, WAVE-2026-04-28-23, WAVE-2026-04-28-24, WAVE-2026-04-28-25]
- track: code

### WAVE-2026-04-28-31

- Title: Acceptance harness closes the local lifecycle, deny, and profile adapter proof loop
- Status: done
- Priority: P1
- Gate: post-G3 local shell proof hardening
- Owning streams: S5 primary, S3 awareness
- Goal: Finish the local shell proof loop in the same-origin acceptance harness: lifecycle allow and deny behavior, refreshed deny visibility, and local profile adapter outcomes must all read back through existing local surfaces only.
- Anchor files: `harnesses/localhost-shell-acceptance-harness.html`
- Validation: `get_errors` clean on `harnesses/localhost-shell-acceptance-harness.html`; same-origin run of `/harnesses/localhost-shell-acceptance.html` passes the added lifecycle, deny, and profile checks
- Constraints: No new browser-side state cache, no remote sync claim, no hardware evidence, and no gate-close wording. If WAVE-2026-04-28-28 already landed, extend the same harness instead of forking a second acceptance path.
- Last update: 2026-04-28
- size: S
- serial-after: WAVE-2026-04-28-30
- track: code

### WAVE-2026-04-28-24

- Title: Acceptance harness proves allowed-write lifecycle path on the live shell
- Status: done
- Priority: P1
- Gate: post-G3 local shell acceptance proof
- Owning streams: S5 primary
- Goal: Extend the localhost shell acceptance harness so it proves the positive local-only lifecycle path: one armed allowed write, one post-write snapshot refresh, and no duplicate lifecycle RPC for a single click.
- Anchor files: `harnesses/localhost-shell-acceptance-harness.html`
- Validation: `get_errors` clean on `harnesses/localhost-shell-acceptance-harness.html`; same-origin run of `/harnesses/localhost-shell-acceptance.html` passes the added lifecycle checks
- Constraints: Prove the existing local-only path only. No hardware, no remote host, and no gate-closing claims.
- Last update: 2026-04-28
- size: S
- parallel-safe-with: [WAVE-2026-04-28-18, WAVE-2026-04-28-19, WAVE-2026-04-28-20, WAVE-2026-04-28-21, WAVE-2026-04-28-22, WAVE-2026-04-28-23, WAVE-2026-04-28-25]
- track: code

### WAVE-2026-04-28-30

- Title: Shell profile surface consumes structured local adapter results
- Status: done
- Priority: P1
- Gate: post-G3 local profile surface
- Owning streams: S5 primary, S2 consumer awareness
- Goal: Render the structured local profile adapter results in the shell so operator-selected `init`, `show`, `export`, and `import` outcomes are legible without exposing frozen-contract internals or widening privileges.
- Anchor files: `site/agent-center-shell.html`
- Validation: `cargo test -p ferros-node shell_route_posts_profile_init_and_show_through_local_adapter`; `cargo test -p ferros-node shell_route_posts_profile_export_and_import_through_local_adapter`; `cargo test -p ferros-node shell_route_serves_local_shell_html`; `get_errors` clean on `site/agent-center-shell.html`
- Constraints: Keep the surface local-only and same-origin. No grant or revoke controls, no remote profile access, and no schema edits.
- Last update: 2026-04-28
- size: S
- serial-after: WAVE-2026-04-28-29
- track: code

### WAVE-2026-04-28-29

- Title: Profile adapter returns structured local status and error payloads for the shell
- Status: done
- Priority: P1
- Gate: post-G3 local profile adapter
- Owning streams: S5 primary, S2 consumer awareness, S4 support awareness
- Goal: Refine the existing local `/profile` adapter so `init`, `show`, `export`, and `import` return structured success and rejection payloads the shell can render without inventing a new browser schema or mutating grant state.
- Anchor files: `crates/ferros-node/src/lib.rs`
- Validation: `cargo test -p ferros-node shell_route_posts_profile_init_and_show_through_local_adapter`; `cargo test -p ferros-node shell_route_posts_profile_export_and_import_through_local_adapter`; `cargo test -p ferros-node shell_route_profile_adapter_rejects_grant_mutation_actions`
- Constraints: Preserve S2 overwrite, parse, verify, and rollback behavior. No new shared write-side JSON-RPC contract. No grant or revoke mutation.
- Last update: 2026-04-28
- size: S
- serial-after: WAVE-2026-04-28-26
- track: code

### WAVE-2026-04-28-28

- Title: Acceptance harness proves runway profile-path and checkpoint rendering
- Status: done
- Priority: P1
- Gate: G4 / D1 runway acceptance proof
- Owning streams: S5 primary
- Goal: Extend the localhost acceptance harness so it proves the runway route reflects the selected profile path, renders checkpoint progress, and keeps the route non-evidentiary and local-only.
- Anchor files: `harnesses/localhost-shell-acceptance-harness.html`
- Validation: `get_errors` clean on `harnesses/localhost-shell-acceptance-harness.html`; same-origin run of `/harnesses/localhost-shell-acceptance.html` passes the added runway checks
- Constraints: Same-origin acceptance only. No D1 or G4 evidence claim. No hardware session assumptions.
- Last update: 2026-04-28
- size: S
- serial-after: WAVE-2026-04-28-27
- track: code

### WAVE-2026-04-28-27

- Title: Shell runway route honors explicit profile-path selection and checkpoint progress
- Status: done
- Priority: P1
- Gate: G4 / D1 runway shell observation
- Owning streams: S5 primary, S4 support
- Goal: Make the live shell runway route read the operator-selected local profile path and render checkpoint progress from the enriched runway summary without widening into write-side profile control.
- Anchor files: `site/agent-center-shell.html`
- Validation: `cargo test -p ferros-node shell_route_gets_local_runway_summary_json`; `cargo test -p ferros-node shell_route_serves_local_shell_html`; `get_errors` clean on `site/agent-center-shell.html`
- Constraints: Reuse the current local profile-path selection only. No browser grant writes, no remote profile access, and no gate claims.
- Last update: 2026-04-28
- size: S
- serial-after: WAVE-2026-04-28-26
- track: code

### WAVE-2026-04-28-26

- Title: ferros-node runway summary consumes LocalRunwayState
- Status: done
- Priority: P1
- Gate: G4 / D1 local runway observation
- Owning streams: S4 primary, S5 consumer awareness
- Goal: Consume the landed runtime runway state inside the `ferros-node` runway summary so checkpoint progress comes from one typed local state model instead of duplicated node-local checklist logic.
- Anchor files: `crates/ferros-node/src/lib.rs`
- Validation: `cargo test -p ferros-node local_agent_api_runway_summary_serializes_and_tracks_profile_and_deny_observation`; `cargo test -p ferros-node shell_route_gets_local_runway_summary_json`
- Constraints: Read-only local shell surface only. No remote transport, no hardware, and no gate-truth edits.
- Last update: 2026-04-28
- size: S
- serial-after: WAVE-2026-04-28-18
- track: code

### WAVE-2026-04-28-22

- Title: xtask burst helper grows queue-clear focused runway commands
- Status: done
- Priority: P1
- Gate: queue-clear tooling substrate
- Owning streams: S1 primary
- Goal: Expand the landed burst helper so aggressive code-track drains have focused commands for runway, profile, and local-push follow-ups instead of relying on one static burst text block.
- Anchor files: `xtask/src/main.rs`
- Validation: `cargo check -p xtask`; `cargo xtask burst`
- Constraints: Additive helper only. No workspace-wide CI policy changes. Keep commands local and code-track scoped.
- Last update: 2026-04-28
- size: S
- parallel-safe-with: [WAVE-2026-04-28-18, WAVE-2026-04-28-19, WAVE-2026-04-28-20, WAVE-2026-04-28-21, WAVE-2026-04-28-23, WAVE-2026-04-28-24, WAVE-2026-04-28-25]
- track: code

### WAVE-2026-04-28-23

- Title: Shell deny and lifecycle outcome rendering above existing local-only surfaces
- Status: done
- Priority: P1
- Gate: post-G3 local shell operator proof
- Owning streams: S5 primary, S3 awareness
- Goal: Use the current local-only `agent.run`, `agent.stop`, `agent.snapshot`, and `denyLog.list` surfaces to render specific blocked, denied, and post-refresh lifecycle states on the live shell without adding new RPC methods.
- Anchor files: `site/agent-center-shell.html`
- Validation: `cargo test -p ferros-node shell_route_serves_local_shell_html`; `get_errors` clean on `site/agent-center-shell.html`
- Constraints: No new RPC methods. No grant or revoke controls. No remote transport. No shadow browser audit store.
- Last update: 2026-04-28
- size: S
- parallel-safe-with: [WAVE-2026-04-28-18, WAVE-2026-04-28-19, WAVE-2026-04-28-20, WAVE-2026-04-28-21, WAVE-2026-04-28-22, WAVE-2026-04-28-24, WAVE-2026-04-28-25]
- track: code

### WAVE-2026-04-28-20

- Title: Typed local-push audit envelope boundary in ferros-data
- Status: done
- Priority: P1
- Gate: G4 local push audit substrate
- Owning streams: S6 primary, S4 awareness
- Goal: Turn the landed local-push audit envelope boundary into typed Rust-local data structures and boundary helpers so `.tmp/push` follow-ups do not rely on markdown-only stand-ins.
- Anchor files: `crates/ferros-data/src/lib.rs`, `schemas/local-push-audit-envelope.schema.json`
- Validation: `cargo test -p ferros-data`; `get_errors` clean on `schemas/local-push-audit-envelope.schema.json`
- Constraints: Preserve local-only and explicit-operator-consent semantics. No frozen S2 schema touch. No remote upload or HA-facing claim.
- Last update: 2026-04-28
- size: L
- parallel-safe-with: [WAVE-2026-04-28-18, WAVE-2026-04-28-19, WAVE-2026-04-28-21, WAVE-2026-04-28-22, WAVE-2026-04-28-23, WAVE-2026-04-28-24, WAVE-2026-04-28-25]
- track: code

### WAVE-2026-04-28-18

- Title: Runtime local runway checkpoint helpers and tests
- Status: done
- Priority: P1
- Gate: G4 / D1 local runway substrate
- Owning streams: S4 primary
- Goal: Tighten the landed `LocalRunwayState` checkpoint scaffold so it has shell-consumable helpers and focused tests before `ferros-node` consumes it. Keep the state machine local-only and non-evidentiary.
- Anchor files: `crates/ferros-runtime/src/local_runway.rs`, `crates/ferros-runtime/src/lib.rs`
- Validation: `cargo test -p ferros-runtime`
- Constraints: Runtime-only. No node, shell, hardware, remote transport, or gate-truth edits.
- Last update: 2026-04-28
- size: S
- parallel-safe-with: [WAVE-2026-04-28-19, WAVE-2026-04-28-20, WAVE-2026-04-28-21, WAVE-2026-04-28-22, WAVE-2026-04-28-23, WAVE-2026-04-28-24, WAVE-2026-04-28-25]
- track: code

### WAVE-2026-04-28-01

- Title: Recursive G4 push scaffolding and breadth probe
- Status: done
- Priority: P0
- Gate: G4 / D1 runway stress push
- Owning streams: S3, S4, S5, S6, S7, S8 support; S2 consumer-awareness only
- Goal: Convert the 2026-04-28 top-level push directive into a bounded repo-backed generation pass that expands G4 and D1 runway surface area across disjoint runtime, schema, harness, tooling, and shell surfaces while preserving doctrine guardrails and emitting per-batch digests under `.tmp/push/`.
- Anchor files: `crates/ferros-runtime/src/`, `crates/ferros-node/src/`, `crates/ferros-data/src/`, `schemas/`, `harnesses/`, `xtask/src/`, `site/`, `.tmp/push/`
- Validation: `cargo test -p ferros-runtime`; `cargo test -p ferros-profile local_consent_snapshot_`; `cargo test -p ferros-data`; `cargo check -p xtask`; `cargo xtask burst`; `cargo test -p ferros-node runway_summary`; `cargo test -p ferros-node shell_listener_posts_json_rpc_`; `get_errors` clean on touched Rust, HTML, schema, queue, log, and digest surfaces.
- Constraints: Preserve frozen `schemas/profile.v0.json` and `schemas/capability-grant.v0.json`; keep consent-first and local-sovereignty invariants explicit in every generated surface; do not widen shared-truth edits beyond required bookkeeping; keep queue IDs stable and run-log entries append-only.
- Last update: 2026-04-28
- size: L
- solo: true
- track: code

### WAVE-2026-04-27-16

- Title: ADR backlog triage and _ROADMAP.md preamble update
- Status: done
- Priority: P2
- Gate: rolling
- Owning streams: S8 primary
- Goal: Catalog the topics in `docs/adr/_ROADMAP.md` and confirm which are addressed (ADR-018 through ADR-024), which remain open, and which are blocked on other waves. Additive preamble note added; ADR-024 remains Proposed.
- Anchor files: `docs/adr/_ROADMAP.md`
- Last update: 2026-04-27
- size: S
- track: code

### WAVE-2026-04-27-15

- Title: Gate narrative explainer (feeds external-facing comms readiness)
- Status: done
- Priority: P2
- Gate: pre-D1 comms readiness
- Owning streams: S8 primary
- Goal: Plain-English explainer of G1–G4 gate progression plus D1 for a non-technical audience. D1 ≠ G4 distinction explicit. No gate docs modified.
- Anchor files: `docs/explainers/gate-narrative.md`
- Last update: 2026-04-27
- size: S
- track: code

### WAVE-2026-04-27-14

- Title: Agent manifest catalog research note (feeds D1 firmware spike)
- Status: done
- Priority: P1
- Gate: pre-D1 firmware prep
- Owning streams: S3 primary; S7 consumer awareness
- Goal: Cataloged echo and timer reference agent manifests; documented HA bridge shim placeholder fields. No bridge implemented; no crate or schema modified.
- Anchor files: `docs/research/S3-agent-manifest-catalog.md`
- Last update: 2026-04-27
- size: S
- track: code

### WAVE-2026-04-27-13

- Title: Policy engine invariant catalog (feeds D1 demo operator readiness)
- Status: done
- Priority: P1
- Gate: pre-D1 operator readiness
- Owning streams: S4 primary
- Goal: Cataloged all grant/deny test invariants from capability_policy.rs and boundaries.rs in plain English. No crate or schema modified.
- Anchor files: `docs/research/S4-policy-engine-invariants.md`
- Last update: 2026-04-27
- size: S
- track: code

### WAVE-2026-04-27-12

- Title: Profile import/export round-trip spec (feeds D1 evidence scripting)
- Status: done
- Priority: P1
- Gate: pre-D1 device selection runway
- Owning streams: S2 primary
- Goal: Documented exact CLI commands and expected output for profile round-trip scripting. profile.v0.json not modified; G2 not reopened.
- Anchor files: `docs/research/S2-profile-import-export-round-trip.md`
- Last update: 2026-04-27
- size: S
- track: code

### WAVE-2026-04-27-11

- Title: Consent flow UX research note (feeds D1 demo script)
- Status: done
- Priority: P1
- Gate: pre-D1 UX planning
- Owning streams: S5 primary
- Goal: Documented deny-log slot, grant/deny display, and D1 consent-flow demonstration requirements from existing localhost shell behavior. No site/, harnesses/, or crate file modified.
- Anchor files: `docs/research/S5-consent-flow-ux.md`
- Last update: 2026-04-27
- size: S
- track: code

### WAVE-2026-04-27-10

- Title: no_std target matrix research note (feeds S7 firmware spikes)
- Status: done
- Priority: P1
- Gate: pre-D1 firmware prep
- Owning streams: S4 primary; S7 consumer awareness
- Goal: Cataloged ferros-core and ferros-runtime cross-compilation targets, feature flags, and D1 device target requirements. No crate or CI file modified.
- Anchor files: `docs/research/S4-no-std-target-matrix.md`
- Last update: 2026-04-27
- size: S
- track: code

### WAVE-2026-04-27-09

- Title: D1 bring-up checklist research note (feeds HARDWARE-QUEUE)
- Status: done
- Priority: P1
- Gate: pre-D1 device selection runway
- Owning streams: S7 primary
- Goal: Synthesized D1 evidence requirements into operator checklist with binary commands, passing/failing results, and firmware spike milestone mapping. No D1 evidence claimed; docs/gates/D1.md not modified.
- Anchor files: `docs/research/S7-d1-bring-up-checklist.md`
- Last update: 2026-04-27
- size: S
- track: code

### WAVE-2026-04-27-08

- Title: S5 consent-flow copy spec derived from legal scaffold CONSENT-LANGUAGE.md
- Status: done
- Priority: P1
- Gate: post-G3 D1 runway
- Owning streams: S5 primary
- Goal: Derive user-visible language for the S5 consent gate from CONSENT-LANGUAGE.md DRAFT; marked as draft pending counsel red-line.
- Anchor files: `streams/S5-ux/README.md`, `streams/S5-ux/BACKLOG.md`, `docs/legal/CONSENT-LANGUAGE.md`
- Last update: 2026-04-27
- size: S
- track: code

### WAVE-2026-04-27-07

- Title: S7 HA bridge consent-mapping note above ADR-023 onramp framing
- Status: done
- Priority: P1
- Gate: post-G3 G4 runway
- Owning streams: S7 primary
- Goal: Add a docs-only mapping note to the S7 owner docs describing how HA entity registration intersects ADR-023's onramp framing.
- Anchor files: `streams/S7-hub/README.md`, `streams/S7-hub/BACKLOG.md`, `docs/hub/pack-b-bring-up-worksheet.md`, `docs/adr/ADR-023-onramp-policy.md`
- Last update: 2026-04-27
- size: S
- track: code

### WAVE-2026-04-27-06

- Title: Define S5 onramp consent surface entry bar
- Status: done
- Priority: P1
- Gate: post-G3 D1 runway
- Owning streams: S5 primary
- Goal: Define the minimum honest UX surface where an external onramp item (HA entity, calendar item, contact import) becomes a proposed FERROS material item awaiting user consent, never canonical state. The surface definition covers: what the slot shows (source system name, proposed item description, consent prompt, accept/reject affordance); the governing invariant from ADR-023 (inbound data quarantined until accepted; consent explicit and auditable; external system does not define identity). Add a consumer-awareness note to ADR-023 indicating S5 is the onramp staging surface implementor. The consent mechanism and onramp calls are not wired yet; the entry bar is docs-only scope definition.
- Anchor files: `streams/S5-ux/README.md`, `streams/S5-ux/BACKLOG.md`, `docs/adr/ADR-023-onramp-policy.md`
- Validation: `get_errors` clean on all 3 anchor files.
- Constraints: Docs-only. No HA bridge protocol details. No wired onramp calls. No consent mechanism implementation. Touch to ADR-023 is consumer-awareness note only — do not reopen the ADR's decision or rationale. Do not reopen G1–G3.
- Last update: 2026-04-27
- size: S
- parallel-safe-with: [WAVE-2026-04-27-07, WAVE-2026-04-27-08]
- track: code

### WAVE-2026-04-27-05

- Title: Define an operator-facing evidence surface above the Pack B bring-up worksheet and HA bridge runway contract
- Status: done
- Priority: P1
- Gate: post-G3 G4 runway
- Owning streams: S7 primary
- Goal: Use the existing Pack B bring-up worksheet and the first HA bridge runway contract as fixed inputs to define an operator-facing evidence surface (read-only) for hub bring-up and status. Anchor the definition against the S7 README and BACKLOG so the bring-up evidence surface has a named place in the S7 plan.
- Anchor files: `streams/S7-hub/README.md`, `streams/S7-hub/BACKLOG.md`, `docs/hub/pack-b-bring-up-worksheet.md`
- Validation: `get_errors` clean on all 3 anchor files.
- Constraints: Docs-only. No bridge protocol details, no pairing handshake order, no HA fork internals. No G4 evidence. No new JSON/RPC routes.
- Last update: 2026-04-27
- size: S
- track: code

### WAVE-2026-04-27-04

- Title: Define the minimum honest first profile-surface entry bar above the frozen S2 contract
- Status: done
- Priority: P1
- Gate: post-G3 local/browser profile surface prep
- Owning streams: S5 primary, S2 consumer awareness
- Goal: Use the frozen S2 contract (`profile.v0.json`, the real `ferros profile init | show | export | import` CLI) and the prior-art `docs/legacy/personal-profile.html` as fixed inputs to define the smallest honest first browser profile surface entry bar on the localhost shell. Scope: `init`, `show`, `export`, `import` only, localhost-only, no grant mutation. Do not reopen S2. Do not wire browser-issued writes or grant/revoke actions.
- Anchor files: `streams/S5-ux/README.md`, `streams/S5-ux/BACKLOG.md`, `streams/S2-profile/README.md`
- Validation: `get_errors` clean on all 3 anchor files.
- Constraints: Docs-only. S2 consumer-awareness only — S2 contract not reopened. No browser-issued profile writes or grant mutation. No G2 re-evidence.
- Last update: 2026-04-27
- size: S
- track: code

### WAVE-2026-04-27-02

- Title: Define the minimum consent-gated browser-issued lifecycle control bar above the staged shell-intent copy
- Status: done
- Priority: P1
- Gate: post-G3 local/browser control prep
- Owning streams: S5 primary, S3 contract awareness, S4 support awareness
- Goal: Use the landed local-only `agent.run` / `agent.stop` backend slice and the newly landed selected-agent shell intent copy as fixed input to define the smallest honest browser-issued local lifecycle control bar, including where consent/audit gating begins, before the shell is allowed to send write RPC.
- Anchor files: `streams/S5-ux/README.md`, `streams/S5-ux/BACKLOG.md`, `streams/S3-agent-center/CONTRACTS.md`, `streams/S4-runtime/BACKLOG.md`
- Validation: `get_errors` clean on all 4 anchor files.
- Constraints: Docs-only. No shell code. No browser-issued writes, no grant/revoke, no remote transport, no broader S4 restart/reload semantics.
- Last update: 2026-04-27
- size: S
- track: code

### WAVE-2026-04-27-01

- Title: Stage selected-agent shell intent copy and read-only slot affordances on the live localhost shell
- Status: done
- Priority: P1
- Gate: post-G3 local/browser control prep
- Owning streams: S5 primary
- Goal: Use the landed shell-intent boundary to stage selected-agent lifecycle intent copy and read-only affordances in the focus, tools, and consent/audit slots on the current localhost shell without wiring browser-issued writes, grant/revoke actions, or broader browser control.
- Anchor files: `site/agent-center-shell.html`, `harnesses/localhost-shell-acceptance-harness.html`, `streams/S5-ux/README.md`, `streams/S5-ux/BACKLOG.md`, `streams/S5-ux/PROGRESS.md`
- Validation: `get_errors` is clean on `site/agent-center-shell.html`, `harnesses/localhost-shell-acceptance-harness.html`, `streams/S5-ux/README.md`, `streams/S5-ux/BACKLOG.md`, and `streams/S5-ux/PROGRESS.md`. Live browser validation at `http://127.0.0.1:4317/` proved selected-agent intent copy updates with agent selection and flips between `agent.run` and `agent.stop` after out-of-band local CLI `run` / `stop` plus refresh. `cargo run -p ferros-node --bin ferros -- agent run echo` passed. `cargo run -p ferros-node --bin ferros -- agent stop echo` passed. Same-origin live harness validation at `http://127.0.0.1:4317/harnesses/localhost-shell-acceptance.html` passed 19 checks, failed 0, and skipped 8 operator/optional checks.
- Constraints: Keep the shell read-only. Do not wire browser-issued writes, publish grant/revoke actions, broaden remote transport, introduce broader privileged UX, or publish broader S4 restart/reload semantics.
- Last update: 2026-04-27

### WAVE-2026-04-26-10

- Title: Define the minimum first shell-intent entry bar above the landed local-only lifecycle/write JSON-RPC slice
- Status: done
- Priority: P1
- Gate: post-G3 local/browser control prep
- Owning streams: S5 primary, S3 contract awareness, S4 support awareness, S8 truth-sync if docs move
- Goal: Use the landed local-only `agent.run` / `agent.stop` JSON-RPC slice on the current localhost shell host, the current observation-only shell, and the stable read-after-write observation path as fixed input to define the smallest honest next shell-intent surface before publishing real browser controls, grant writes, remote transport, bridge-control choreography, or broader S4 restart/reload claims.
- Anchor files: `streams/S5-ux/README.md`, `streams/S5-ux/BACKLOG.md`, `streams/S3-agent-center/CONTRACTS.md`, `streams/S4-runtime/BACKLOG.md`
- Validation: `get_errors` is clean on `streams/S5-ux/README.md`, `streams/S5-ux/BACKLOG.md`, `streams/S5-ux/PROGRESS.md`, `streams/S3-agent-center/CONTRACTS.md`, and `streams/S4-runtime/BACKLOG.md`.
- Constraints: Keep the wave docs-only and S5-owned. Do not change shell code. Do not publish real browser controls, grant/revoke actions, remote transport, bridge-control choreography, broader privileged UX, broader S4 restart/reload semantics, schemas, `crates/ferros-hub`, or G4 evidence as landed without a code-backed follow-up.
- Last update: 2026-04-27

### WAVE-2026-04-26-09

- Title: Land the minimum local-only lifecycle/write JSON/RPC slice through the current localhost shell host
- Status: done
- Priority: P1
- Gate: post-G3 local/browser write slice
- Owning streams: S3 primary, S4 support, S5 consumer awareness, S8 truth-sync if docs move
- Goal: Reuse the landed `LocalAgentApi` seam and the current localhost shell host to add only the smallest local-only JSON/RPC lifecycle/write slice for `agent.run` and `agent.stop`, keep `agent.describe`, `agent.snapshot`, and `denyLog.list` as the read-after-write observation path, and avoid publishing browser control, remote transport, grant writes, bridge-control choreography, or broader S4 restart/reload claims.
- Anchor files: `crates/ferros-node/src/lib.rs`, `crates/ferros-agents/src/rpc.rs`, `streams/S3-agent-center/CONTRACTS.md`, `streams/S5-ux/README.md`, `streams/S4-runtime/BACKLOG.md`
- Validation: `cargo test -p ferros-node agent_write_rpc_` passed. `cargo test -p ferros-node shell_listener_posts_json_rpc_` passed. `cargo test -p ferros-node agent_read_rpc_` passed. `cargo test -p ferros-agents` passed. `cargo xtask ci` passed. `get_errors` is clean on `streams/S3-agent-center/README.md`, `streams/S3-agent-center/CONTRACTS.md`, `streams/S3-agent-center/BACKLOG.md`, `streams/S3-agent-center/PROGRESS.md`, `streams/S5-ux/README.md`, `streams/S5-ux/BACKLOG.md`, `streams/S5-ux/PROGRESS.md`, `streams/S4-runtime/BACKLOG.md`, `streams/S4-runtime/PROGRESS.md`, `STATUS.md`, and `docs/contracts/CONTRACTS-OVERVIEW.md`.
- Constraints: Keep the slice local-only through the current localhost shell host and the landed `LocalAgentApi` seam. Do not publish browser control, remote transport, grant writes, bridge-control choreography, broader S4 restart/reload semantics, schemas, `crates/ferros-hub`, or G4 evidence.
- Last update: 2026-04-26

### WAVE-2026-04-26-08

- Title: Define the minimum first local write JSON/RPC entry bar above `LocalAgentApi`
- Status: done
- Priority: P1
- Gate: post-G3 local/browser write prep
- Owning streams: S3 primary, S5 consumer awareness, S4 support awareness, S8 truth-sync if docs move
- Goal: Use the landed local-only `LocalAgentApi` seam, the stable local CLI and deny-log summaries, and the current read-first localhost shell host as fixed input to define the smallest honest next write-side contract above the local path before publishing browser control, write JSON/RPC, remote transport, grant writes, bridge-control choreography, or broader S4 restart/reload claims.
- Anchor files: `streams/S3-agent-center/README.md`, `streams/S3-agent-center/CONTRACTS.md`, `streams/S5-ux/README.md`, `streams/S5-ux/BACKLOG.md`
- Validation: `get_errors` is clean on `streams/S3-agent-center/README.md`, `streams/S3-agent-center/CONTRACTS.md`, `streams/S3-agent-center/BACKLOG.md`, `streams/S3-agent-center/PROGRESS.md`, `streams/S5-ux/README.md`, `streams/S5-ux/BACKLOG.md`, and `streams/S5-ux/PROGRESS.md`.
- Constraints: Keep the wave docs-only and S3-owned. Do not change code. Do not publish write JSON/RPC, browser control, remote transport, privileged UX, grant writes, bridge-control choreography, broader S4 restart/reload semantics, schemas, `crates/ferros-hub`, or G4 evidence as landed without a code-backed local-only slice.
- Last update: 2026-04-26

### WAVE-2026-04-26-07

- Title: Expose richer local deny-reason introspection on the `LocalAgentApi` seam
- Status: done
- Priority: P1
- Gate: post-G3 local wrapper/API hardening
- Owning streams: S3 primary, S4 support, S8 truth-sync if docs move
- Goal: Reuse the landed local-only `LocalAgentApi` path to preserve and expose missing-capability deny detail on the same local lifecycle/read-after-write seam without widening into write JSON/RPC, browser control, remote transport, grant writes, bridge-control choreography, or broader S4 restart/reload claims.
- Anchor files: `crates/ferros-node/src/lib.rs`, `crates/ferros-node/src/bin/ferros.rs`, `streams/S3-agent-center/BACKLOG.md`, `streams/S3-agent-center/CONTRACTS.md`, `streams/S4-runtime/BACKLOG.md`
- Validation: `cargo test -p ferros-node local_agent_api_` passed. `cargo test -p ferros-node agent_cli_` passed. `cargo xtask ci` passed, which also covered the broader `ferros-node` read-path and shell-host suites. `get_errors` is clean on `streams/S3-agent-center/BACKLOG.md`, `streams/S3-agent-center/CONTRACTS.md`, `streams/S3-agent-center/README.md`, `streams/S3-agent-center/PROGRESS.md`, `streams/S4-runtime/BACKLOG.md`, `streams/S4-runtime/PROGRESS.md`, `STATUS.md`, and `docs/contracts/CONTRACTS-OVERVIEW.md`.
- Constraints: Keep the hot lane narrow around `crates/ferros-node/src/lib.rs`. Do not publish write JSON/RPC, browser control, remote transport, grant-write semantics, bridge-control choreography, broader S4 restart/reload semantics, schemas, `crates/ferros-hub`, or G4 evidence.
- Last update: 2026-04-26

### WAVE-2026-04-26-06

- Title: Publish the first broader lifecycle/write wrapper/API slice above the local-only seam
- Status: done
- Priority: P1
- Gate: post-G3 broader lifecycle/write slice
- Owning streams: S3 primary, S4 support, S8 truth-sync if docs move
- Goal: Reuse the newly isolated internal local host-controller seam above argv parsing to land the smallest real code-backed broader lifecycle/write wrapper/API slice S3 can honestly publish above the current local-only path while preserving the current local host/state path and read-first observation surfaces.
- Anchor files: `crates/ferros-node/src/lib.rs`, `crates/ferros-node/src/bin/ferros.rs`, `streams/S3-agent-center/BACKLOG.md`, `streams/S3-agent-center/CONTRACTS.md`, `streams/S4-runtime/BACKLOG.md`
- Validation: `cargo test -p ferros-node local_agent_api_`, `cargo test -p ferros-node agent_cli_`, `cargo test -p ferros-node agent_read_rpc_`, and `cargo test -p ferros-node shell_listener_posts_json_rpc_` all passed; `get_errors` is clean on `crates/ferros-node/src/lib.rs`, `crates/ferros-node/src/bin/ferros.rs`, `streams/S3-agent-center/BACKLOG.md`, `streams/S3-agent-center/README.md`, `streams/S3-agent-center/CONTRACTS.md`, `streams/S3-agent-center/PROGRESS.md`, `streams/S4-runtime/BACKLOG.md`, `streams/S4-runtime/PROGRESS.md`, `STATUS.md`, and `docs/contracts/CONTRACTS-OVERVIEW.md`.
- Constraints: Keep the slice narrow around the newly extracted internal controller seam in `crates/ferros-node/src/lib.rs`. Do not publish remote transport, richer remote observation/control, privileged UX, grant-write semantics, bridge-control choreography, broader S4 restart/reload semantics, schemas, `crates/ferros-hub`, or G4 evidence.
- Last update: 2026-04-26

### WAVE-2026-04-26-05

- Title: Extract the first internal local host-controller surface above the CLI/state path
- Status: done
- Priority: P1
- Gate: post-G3 host-controller extraction
- Owning streams: S4 primary, S3 coordination, S8 truth-sync if docs move
- Goal: Now that the local-only seam is documented above and the shell can prove local `run` / `stop` observation through `agent.snapshot`, extract the next smallest code-backed internal local host-controller surface above argv parsing but still below any published broader lifecycle/write wrapper/API or remote-control contract.
- Anchor files: `crates/ferros-node/src/lib.rs`, `crates/ferros-node/src/bin/ferros.rs`, `streams/S3-agent-center/CONTRACTS.md`, `streams/S4-runtime/BACKLOG.md`
- Validation: Focused `cargo test -p ferros-node agent_cli_`, `cargo test -p ferros-node agent_read_rpc_`, and `cargo test -p ferros-node shell_listener_posts_json_rpc_` all passed; `get_errors` is clean on `crates/ferros-node/src/lib.rs`, `streams/S4-runtime/BACKLOG.md`, `streams/S4-runtime/PROGRESS.md`, `streams/S3-agent-center/CONTRACTS.md`, and `streams/S3-agent-center/PROGRESS.md`.
- Constraints: Keep the slice narrow around `crates/ferros-node/src/lib.rs` as the hot root abstraction. No remote transport, no privileged UX, no grant-write semantics, no bridge-control choreography, no published broader lifecycle/write wrapper/API, no S4 restart/reload publication, no schemas, no `crates/ferros-hub`, and no G4 evidence.
- Last update: 2026-04-26

### WAVE-2026-04-26-04

- Title: Truth-sync the S2 backlog to the closed G2 boundary
- Status: done
- Priority: P1
- Gate: G2 truth-sync
- Owning streams: S2 primary, S8 truth-sync if queue or stream docs move
- Goal: Remove stale pre-freeze and unfinished-CLI wording from the S2 backlog, mark the already-closed G2 boundary items landed, and replace them with post-G2 parity and local CLI hardening follow-on wording without reopening the frozen profile or grant contracts.
- Anchor files: `streams/S2-profile/BACKLOG.md`
- Validation: `get_errors` is clean on `streams/S2-profile/BACKLOG.md`.
- Constraints: Keep the slice backlog-only and S2-owned. Do not reopen the closed G2 boundary, mutate frozen schemas, widen CLI claims beyond the landed evidence, or claim new gate movement.
- Last update: 2026-04-26

### WAVE-2026-04-26-03

- Title: Prepare the first Pack B bring-up worksheet from the runway map
- Status: done
- Priority: P1
- Gate: G4 runway
- Owning streams: S7 primary, S8 truth-sync if queue or stream docs move
- Goal: Derive the first Pack B bring-up worksheet from `docs/hub/reference-hardware.md` so the runway maps to an operator-usable worksheet without claiming hub implementation, Home Assistant bridge implementation, physical-device proof, launch truth, or G4 evidence.
- Anchor files: `docs/hub/pack-b-bring-up-worksheet.md`, `streams/S7-hub/BACKLOG.md`, `streams/S7-hub/PROGRESS.md`
- Validation: `get_errors` is clean on `docs/hub/pack-b-bring-up-worksheet.md`, `streams/S7-hub/BACKLOG.md`, and `streams/S7-hub/PROGRESS.md`.
- Constraints: Keep the slice docs-only and S7-owned. Do not widen into `crates/ferros-hub`, Home Assistant bridge implementation, pairing ratification, launch truth, or G4 evidence.
- Last update: 2026-04-26

### WAVE-2026-04-26-02

- Title: Add operator-assisted localhost shell observation proof over the local lifecycle seam
- Status: done
- Priority: P1
- Gate: post-G3 consumer reliability
- Owning streams: S5 primary, S3 contract awareness, S8 truth-sync if queue or stream docs move
- Goal: Extend the same-origin localhost harness so it can pause for out-of-band local `ferros agent run echo` / `ferros agent stop echo` commands, refresh the shell, and prove those state changes still read back through exactly one `agent.snapshot` call while keeping the shell observation-only and live deny generation outside the current shell/CLI surface.
- Anchor files: `harnesses/localhost-shell-acceptance-harness.html`, `streams/S5-ux/BACKLOG.md`, `streams/S5-ux/README.md`, `streams/S5-ux/PROGRESS.md`
- Validation: `get_errors` is clean on `harnesses/localhost-shell-acceptance-harness.html`, `streams/S5-ux/BACKLOG.md`, `streams/S5-ux/README.md`, and `streams/S5-ux/PROGRESS.md`. Harness reload with auto-skipped operator prompts produced 16 passed, 0 failed, 6 skipped. Live shell validation passed after restoring `cargo run -p ferros-node --bin ferros-node -- shell 4317`; `cargo run -p ferros-node --bin ferros -- agent run echo` followed by browser refresh showed one `agent.snapshot` call and echo observed as running; `cargo run -p ferros-node --bin ferros -- agent stop echo` followed by refresh showed one `agent.snapshot` call and echo observed as stopped.
- Constraints: Keep the slice observation-only and S5-owned. Do not add privileged writes, live deny generation, grant mutation flows, remote transport, bridge-control choreography, broader S4 restart/reload semantics, schemas, `crates/ferros-hub`, or G4 evidence.
- Last update: 2026-04-26

### WAVE-2026-04-26-01

- Title: Validate the local thumbv7em none-default-features proof and CI enforcement slice
- Status: done
- Priority: P1
- Gate: post-G3 hardening
- Owning streams: S4 primary, S1 CI awareness, S8 truth-sync if queue or stream docs move
- Goal: Validate the existing local `ferros-core` `thumbv7em-none-eabi` `no-default-features` proof and current CI enforcement slice without widening into a new hosted CI claim, broader `no_std` publication, host hardening, schemas, `crates/ferros-hub`, or G4 evidence.
- Anchor files: `.github/workflows/ci.yml`, `streams/S4-runtime/README.md`, `streams/S4-runtime/PROGRESS.md`
- Validation: `cargo check -p ferros-core --target thumbv7em-none-eabi --no-default-features` passed. `cargo check -p ferros-core --no-default-features` passed. `cargo test -p ferros-core` passed. `get_errors` is clean on `.github/workflows/ci.yml`; the matching S4 docs were already clean in the parent thread.
- Constraints: Keep the slice proof-only and S4-owned. Do not claim a new hosted CI run in this batch. Do not widen into broader `no_std` publication, remote/control work, schemas, `crates/ferros-hub`, or G4 evidence.
- Last update: 2026-04-26

### WAVE-2026-04-25-07

- Title: Define the minimum first broader lifecycle/write wrapper/API entry bar above the local-only seam
- Status: done
- Priority: P1
- Gate: post-G3 broader lifecycle/write prep
- Owning streams: S3 primary, S4 host awareness, S8 truth-sync if queue or stream docs move
- Goal: Use the landed local-only `ferros agent run` / `ferros agent stop` state-path seam, the current read-first `agent.describe`, `agent.snapshot`, `grant.list`, and `denyLog.list` observation surfaces, and the dedicated deny-by-default lifecycle/log evidence as fixed input to define the smallest honest first broader lifecycle/write wrapper/API slice S3 could publish next, naming the minimum additional code-backed surface required above the landed local-only seam before any broader wrapper/API, remote transport, richer remote observation/control, privileged UX, grant-write semantics, bridge-control sequencing, or broader S4 restart/reload claim is honest.
- Anchor files: `streams/S3-agent-center/README.md`, `streams/S3-agent-center/CONTRACTS.md`, `streams/S3-agent-center/PROGRESS.md`
- Validation: `get_errors` is clean on `streams/S3-agent-center/README.md`, `streams/S3-agent-center/CONTRACTS.md`, and `streams/S3-agent-center/PROGRESS.md`.
- Constraints: Keep the wave docs-only/prep and S3-owned. Do not change code. Do not publish a broader lifecycle/write wrapper/API or remote/control contract as landed without a code-backed surface. Do not widen into remote transport, richer remote observation/control, privileged UX, grant-write semantics, pairing choreography, bridge-control sequencing, broader S4 restart/reload semantics, schemas, `crates/ferros-hub`, or G4 evidence.
- Last update: 2026-04-26

### WAVE-2026-04-25-06

- Title: Land the first local-only lifecycle/write seam through the current CLI/state path
- Status: done
- Priority: P1
- Gate: post-G3 local-only lifecycle/write seam
- Owning streams: S3 primary, S4 host awareness, S8 truth-sync if queue or stream docs move
- Goal: Reuse `DemoRuntime::reference_host()`, `run_reference_demo_cycle()`, the current local CLI/state-path behavior, the current local CLI inspection plus read-first JSON-RPC methods, and the dedicated deny-by-default lifecycle/log harness to land the narrowest real local-only lifecycle/write seam through the existing CLI/state path, without publishing a lifecycle/write wrapper/API, richer remote observation/control, privileged UX, grant-write, bridge-control, broader S4 restart/reload semantics, schemas, `crates/ferros-hub`, or G4 evidence.
- Anchor files: `crates/ferros-node/src/lib.rs`, `streams/S3-agent-center/BACKLOG.md`, `streams/S3-agent-center/CONTRACTS.md`, `streams/S3-agent-center/README.md`
- Validation: parent-thread checks passed. `cargo test -p ferros-node agent_read_rpc_observes_cli_lifecycle_state_after_local_run_and_stop` passed. `cargo test -p ferros-node agent_cli_` passed. `cargo test -p ferros-node agent_read_rpc_` passed. `get_errors` is clean on `crates/ferros-node/src/lib.rs`, `streams/S3-agent-center/BACKLOG.md`, `streams/S3-agent-center/CONTRACTS.md`, `streams/S3-agent-center/README.md`, and `streams/S3-agent-center/PROGRESS.md`.
- Constraints: Keep the slice S3-owned, implementation-backed, and local-only through the existing CLI/state path. Reuse the documented local seams instead of publishing a wrapper/API or remote-control surface. Do not widen into richer remote observation/control, privileged UX, grant-write, pairing choreography, bridge-control sequencing, broader S4 restart/reload semantics, schemas, `crates/ferros-hub`, or G4 evidence.
- Last update: 2026-04-25

### WAVE-2026-04-25-05

- Title: Define the minimum first lifecycle/write wrapper entry bar
- Status: done
- Priority: P1
- Gate: post-G3 lifecycle/write prep
- Owning streams: S3 primary, S4 host awareness, S8 truth-sync if queue or stream docs move
- Goal: Use the landed reusable in-memory host plus dedicated deny-by-default lifecycle/log evidence as fixed input to define the smallest honest first lifecycle/write wrapper/API slice S3 could publish next, naming which current local-only seams may be reused and which write-side semantics must exist before any wrapper, richer remote observation/control, or privileged UX claim is honest.
- Anchor files: `streams/S3-agent-center/BACKLOG.md`, `streams/S3-agent-center/CONTRACTS.md`, `streams/S3-agent-center/README.md`
- Validation: parent-thread checks passed. `get_errors` is clean on `streams/S3-agent-center/BACKLOG.md`, `streams/S3-agent-center/CONTRACTS.md`, and `streams/S3-agent-center/README.md`. The landed wording was reread directly and keeps lifecycle/write wrapper/API, richer remote observation/control, privileged UX, grant-write, bridge-control, and broader S4 restart/reload semantics unpublished while naming `DemoRuntime::reference_host()`, `run_reference_demo_cycle()`, the current local CLI/state path, the current local CLI inspection plus read-first JSON-RPC methods, and the dedicated deny-by-default lifecycle/log harness as the local-only seams to reuse before any honest lifecycle/write publication.
- Constraints: Keep the wave docs-only/prep and S3-owned. Do not change code. Do not publish lifecycle/write wrapper or remote-control contract text as landed.
- Last update: 2026-04-25

### WAVE-2026-04-25-04

- Title: Expand deny-by-default evidence into a dedicated lifecycle/log harness
- Status: done
- Priority: P1
- Gate: post-G3 contract hardening
- Owning streams: S3 primary, S4 host awareness, S8 truth-sync if queue or stream docs move
- Goal: Expand deny-by-default evidence from the current manifest authorization plus `ferros-node` demo/runtime denial-log assertions into a dedicated lifecycle/log harness around the current reusable in-memory host and local `ferros` agent state path so the repo proves denied lifecycle attempts and deny-log observation together without publishing lifecycle/write wrapper APIs, richer remote observation/control transport, S5 privileged grant/revoke UX, pairing choreography, bridge-control sequencing, broader S4 restart/reload semantics, schemas, `crates/ferros-hub`, or G4 evidence.
- Anchor files: `crates/ferros-agents/src/manifest.rs`, `crates/ferros-node/src/lib.rs`, `streams/S3-agent-center/BACKLOG.md`, `streams/S3-agent-center/CONTRACTS.md`
- Validation: parent-thread checks passed. `cargo test -p ferros-agents manifest_authorization_` passed. `cargo test -p ferros-node agent_cli_` passed. `cargo test -p ferros-node agent_read_rpc_` passed. `get_errors` is clean on `crates/ferros-node/src/lib.rs`, `streams/S3-agent-center/BACKLOG.md`, `streams/S3-agent-center/CONTRACTS.md`, `streams/S3-agent-center/PROGRESS.md`, and `streams/S3-agent-center/README.md`.
- Constraints: Keep the slice S3-owned and evidence-backed. Keep the current CLI and reusable in-memory host local-only. Do not publish lifecycle/write wrapper or richer remote observation/control contracts. Do not widen into S5 privileged grant/revoke UX, pairing choreography, bridge-control sequencing, broader S4 restart/reload semantics, schemas, `crates/ferros-hub`, or G4 evidence.
- Last update: 2026-04-25

### WAVE-2026-04-25-03

- Title: Harden `ferros-node demo` into a reusable runtime-host integration layer
- Status: done
- Priority: P1
- Gate: post-G3 runtime-host hardening
- Owning streams: S3 primary, S4 host awareness, S8 truth-sync if queue or stream docs move
- Goal: Turn the current deterministic `ferros-node demo` path into a reusable runtime-host integration layer around the existing registry, reference-agent, and deny-by-default seams so later lifecycle/write wrapper work can build on a code-backed host surface without inventing pairing choreography, bridge-control sequencing, broader S4 restart/reload semantics, schemas, `crates/ferros-hub`, or G4 evidence.
- Anchor files: `crates/ferros-node/src/lib.rs`, `streams/S3-agent-center/BACKLOG.md`, `streams/S3-agent-center/CONTRACTS.md`, `streams/S3-agent-center/PROGRESS.md`, `streams/S3-agent-center/README.md`
- Validation: parent-thread checks passed. `cargo test -p ferros-node demo_` passed. `cargo test -p ferros-node reload_boundary_runtime_with_state_` passed. `cargo run -p ferros-node --bin ferros -- demo` printed the stable deterministic output (`started: echo,timer`, `echo: hello`, `timer: tick-1`, `denied: 1`). `get_errors` is clean on `crates/ferros-node/src/lib.rs`, `streams/S3-agent-center/BACKLOG.md`, `streams/S3-agent-center/CONTRACTS.md`, `streams/S3-agent-center/PROGRESS.md`, and `streams/S3-agent-center/README.md`.
- Constraints: Keep the slice S3-owned and implementation-backed. Respect the current S4 host/runtime hardening constraint beyond the in-memory demo and only take the minimum shared host-trait alignment directly required for the reusable layer. Do not publish lifecycle/write wrapper/API contracts or S5 privileged flows until those code-backed surfaces actually exist.
- Last update: 2026-04-25

### WAVE-2026-04-25-02

- Title: Consume the landed agent.snapshot observation surface in the S5 local shell
- Status: done
- Priority: P1
- Gate: post-G3 consumer reliability
- Owning streams: S5 primary, S3 consumer awareness, S8 truth-sync if docs move
- Goal: Consume the landed read-only agent.snapshot surface in `site/agent-center-shell.html` and the same-origin localhost-shell acceptance path so the user-end shell can render current agent, grant-state, and deny-log observation from one aggregated read without inventing lifecycle/write UX, pairing choreography, bridge-control sequencing, or S4 restart/reload semantics.
- Anchor files: `site/agent-center-shell.html`, `harnesses/localhost-shell-acceptance-harness.html`, `streams/S5-ux/BACKLOG.md`, `streams/S5-ux/PROGRESS.md`, `streams/S5-ux/README.md`
- Validation: parent-thread checks passed. `get_errors` is clean on `site/agent-center-shell.html`, `harnesses/localhost-shell-acceptance-harness.html`, `streams/S5-ux/BACKLOG.md`, `streams/S5-ux/PROGRESS.md`, and `streams/S5-ux/README.md`. Live browser validation at `http://127.0.0.1:4317/` showed the real shell in ready/live state with snapshot-based copy and aggregated metrics. Same-origin live harness validation at `http://127.0.0.1:4317/harnesses/localhost-shell-acceptance.html` passed 16/16 checks against the real local shell, including snapshot-only manual refresh, zero extra RPCs on loaded-agent selection, grants empty-state degradation, deny-log visibility, and read-only audit copy.
- Constraints: Keep the slice observation-only and S5-owned. Do not add lifecycle/write UX, consent or grant mutation flows, pairing choreography, bridge-control sequencing, or S4 restart/reload changes. Do not touch schemas, `crates/ferros-hub`, or G4 evidence, and only truth-sync S5 docs that actually move.
- Last update: 2026-04-25

### WAVE-2026-04-25-01

- Title: Land the first real S3 hub-facing wrapper/API slice for S7 runway
- Status: done
- Priority: P1
- Gate: G4 runway
- Owning streams: S3 primary, S7 consumer awareness, S8 truth-sync if queue or stream docs move
- Goal: Add the narrowest real S3 wrapper/API implementation surface on top of the current registry and read-first inspection seams so S3 can honestly publish the next hub-facing lifecycle-wrapper or richer observation contract once that implementation exists, without inventing pairing choreography, bridge-control sequencing, or S4 restart/reload semantics.
- Anchor files: `crates/ferros-agents/src/registry.rs`, `crates/ferros-agents/src/rpc.rs`, `crates/ferros-node/src/lib.rs`, `streams/S3-agent-center/CONTRACTS.md`, `streams/S3-agent-center/BACKLOG.md`, `streams/S7-hub/BACKLOG.md`
- Validation: focused `cargo test -p ferros-agents` and `cargo test -p ferros-node` coverage for the new wrapper/API slice; editor diagnostics on touched S3 and S7 docs if publication wording moves; verify the landed publication names only the code-backed wrapper/API surface that actually exists and keeps pairing choreography, bridge-control sequencing, S4 restart/reload semantics, schemas, and G4 evidence unpublished unless they truly land
- Constraints: Keep the slice S3-owned and implementation-backed. Do not publish a new wrapper/API contract without landing the real implementation surface it names. Do not invent pairing choreography, bridge-control sequencing, or S4 restart/reload semantics. Do not touch schemas, STATUS.md, docs/gates/G4.md, or crates/ferros-hub/, and do not claim G4 evidence.
- Last update: 2026-04-25

### WAVE-2026-04-24-18

- Title: Publish the first hub-facing wrapper boundary for S7 runway
- Status: done
- Priority: P1
- Gate: G4 runway
- Owning streams: S3 primary, S7 consumer awareness, S8 truth-sync if queue or stream docs move
- Goal: Publish an S3-owned docs-only wrapper boundary for S7 by deciding what stays on `AgentRegistry` plus local/read-first inspection surfaces and what additional lifecycle or remote-observation contract must exist before bridge control flows are honest.
- Anchor files: `streams/S3-agent-center/CONTRACTS.md`, `streams/S3-agent-center/BACKLOG.md`, `streams/S7-hub/BACKLOG.md`
- Validation: editor diagnostics on touched S3 and S7 docs; verify the published boundary names only `AgentRegistry` plus local/read-first inspection surfaces as currently sufficient and keeps hub-facing lifecycle-wrapper, remote-observation, pairing, schema, and G4 surfaces explicitly unpublished
- Constraints: Keep the slice docs-only and S3-owned. Do not change code, do not reopen S4 restart/reload semantics, do not define pairing or bridge-control choreography, do not touch schemas, `STATUS.md`, `docs/gates/G4.md`, or other shared truth surfaces, do not scaffold `crates/ferros-hub/`, and do not claim G4 evidence.
- Last update: 2026-04-25

### WAVE-2026-04-24-17

- Title: Lock the published S4 restart/reload boundary with focused tests
- Status: done
- Priority: P1
- Gate: G4 runway
- Owning streams: S4 primary, S8 truth-sync if queue or stream docs move
- Goal: Add focused `ferros-node` and `ferros-profile` tests that lock only the currently published S4 restart/reload boundary: exact-path `CliState::load(path)` reload, `runtime_with_state(state_path)` rebuilding the fixed reference runtime while replaying only persisted Registered/Running/Stopped state, and `LocalProfileStore::load_local_profile(path)` reloading profile/key/grant material only when local validation succeeds.
- Anchor files: `crates/ferros-node/src/lib.rs`, `crates/ferros-profile/src/lib.rs`, `streams/S4-runtime/BACKLOG.md`
- Validation: focused `cargo test -p ferros-node` coverage for `CliState::load(path)` and `runtime_with_state(state_path)` plus focused `cargo test -p ferros-profile` coverage for `LocalProfileStore::load_local_profile(path)` success and invalid-local-state rejection; editor diagnostics on touched S4 docs if backlog or contract wording moves
- Constraints: Keep the slice S4-owned and boundary-lock only. Do not widen into pairing, durable hub restart, re-registration choreography, `crates/ferros-hub/` scaffolding, shared schema changes, or G4 evidence claims.
- Last update: 2026-04-24

### WAVE-2026-04-24-16

- Title: Publish a narrow hub-facing restart/reload boundary for S7 runway
- Status: done
- Priority: P1
- Gate: G4 runway
- Owning streams: S4 primary, S7 consumer awareness, S8 truth-sync if queue or stream docs move
- Goal: Turn the currently named reload helpers into an explicit S4-owned docs-only boundary that states what restart-safe state, reload, and re-registration guarantees S7 may rely on now versus what remains unpublished before any authoritative pairing flow, `ferros-hub` scaffold, or Home Assistant bridge plan is honest.
- Anchor files: `streams/S4-runtime/CONTRACTS.md`, `streams/S4-runtime/BACKLOG.md`, `streams/S7-hub/BACKLOG.md`
- Validation: editor diagnostics on touched S4 and S7 docs; verify the published boundary stays consistent with the landed S7 seam brief in `streams/S7-hub/README.md` and `streams/S7-hub/CONTRACTS.md`, plus `STATUS.md` and `docs/gates/G4.md`
- Constraints: Keep the slice docs-only and S4-owned. Do not change runtime code, do not scaffold `crates/ferros-hub/`, do not define pairing, reboot, or re-registration choreography beyond the narrow published boundary, do not invent new policy semantics, and do not claim G4 evidence.
- Last update: 2026-04-24

### WAVE-2026-04-24-15

- Title: Route the landed S7 seam brief to S3 and S4
- Status: done
- Priority: P1
- Gate: G4 runway
- Owning streams: S3 primary, S4 primary, S7 consumer awareness, S8 truth-sync if queue or stream docs move
- Goal: Consume the landed S7 seam brief in S3- and S4-owned docs by recording which current registration, inspection, policy, and restart surfaces are already sufficient versus still unpublished before any authoritative pairing flow, `ferros-hub` scaffold, or Home Assistant bridge plan is honest.
- Anchor files: `streams/S3-agent-center/CONTRACTS.md`, `streams/S3-agent-center/BACKLOG.md`, `streams/S4-runtime/CONTRACTS.md`, `streams/S4-runtime/BACKLOG.md`, `streams/S7-hub/BACKLOG.md`
- Validation: editor diagnostics on touched S3, S4, and S7 docs; verify the seam-classification pass stays consistent with the landed S7 seam brief in `streams/S7-hub/README.md` and `streams/S7-hub/CONTRACTS.md`, plus `docs/hub/reference-hardware.md`, `STATUS.md`, and `docs/gates/G4.md`
- Constraints: Keep the slice docs-only and non-implementation until the concrete S3/S4 APIs exist. Do not reopen S2 answer docs, do not rewrite the landed S7 seam inventory unless it exposes a contradiction, do not scaffold `crates/ferros-hub/`, do not define Home Assistant bridge internals, do not ratify pairing or reboot choreography, and do not claim G4 evidence.
- Last update: 2026-04-24

### WAVE-2026-04-24-14

- Title: Turn the published S2 handoff into an S7 seam brief
- Status: done
- Priority: P1
- Gate: G4 runway
- Owning streams: S7 primary, S3 and S4 consumer awareness, S8 truth-sync if queue or stream docs move
- Goal: Turn the published S2 consumer-boundary handoff into an S7-owned seam brief keyed to the exact S3 registry/list/log and S4 restart/policy APIs still needed before any authoritative pairing flow, `ferros-hub` scaffold, or HA bridge plan is honest.
- Anchor files: `streams/S7-hub/README.md`, `streams/S7-hub/CONTRACTS.md`, `streams/S7-hub/BACKLOG.md`
- Validation: editor diagnostics on touched S7 docs; verify the seam brief stays consistent with `streams/S3-agent-center/CONTRACTS.md`, `streams/S4-runtime/CONTRACTS.md`, `docs/hub/reference-hardware.md`, `STATUS.md`, and `docs/gates/G4.md`
- Constraints: Keep the slice docs-only and S7-owned. Do not reopen S2 answer docs unless the seam inventory exposes a contradiction, do not scaffold `crates/ferros-hub/`, do not define Home Assistant bridge internals, do not freeze handshake order, and do not claim G4 evidence.
- Last update: 2026-04-24

### WAVE-2026-04-24-13

- Title: Draft the S7 pairing/design handoff from landed S2 answers
- Status: done
- Priority: P1
- Gate: G4 runway
- Owning streams: S7 primary, S3 and S4 consumer awareness, S8 truth-sync if queue or stream docs move
- Goal: Consume the published S2 answers for bootstrap, grant check, deny visibility, persistence, revocation, and re-registration into an S7-owned provisional pairing/design handoff that states what S7 may now assume from the stable S2 consumer boundary and what still remains open before any authoritative pairing flow, `ferros-hub` scaffold, or Home Assistant bridge plan is honest.
- Anchor files: `streams/S7-hub/README.md`, `streams/S7-hub/BACKLOG.md`
- Validation: editor diagnostics on touched S7 docs; verify the handoff stays consistent with `streams/S2-profile/README.md`, `streams/S2-profile/CONTRACTS.md`, `streams/S7-hub/CONTRACTS.md`, `docs/hub/reference-hardware.md`, `STATUS.md`, and `docs/gates/G4.md`
- Constraints: Keep the slice docs-only and S7-owned. Do not reopen S2 answer docs, do not scaffold `crates/ferros-hub/`, do not define Home Assistant bridge internals, do not ratify handshake order, and do not claim G4 evidence.
- Last update: 2026-04-24

### WAVE-2026-04-24-12

- Title: Publish S2 pairing boundary answers for S7 runway
- Status: done
- Priority: P1
- Gate: G4 runway
- Owning streams: S2 primary, S7 consumer awareness, S8 truth-sync if queue or stream docs move
- Goal: Publish S2-owned answers to the six-row S7 consumer-boundary question list by documenting how bootstrap, grant check, deny visibility, persistence, revocation, and re-registration consume the stable `ProfileId` and `CapabilityGrant` surface without widening the frozen v0 consumer contracts before S7 names an authoritative pairing flow.
- Anchor files: `streams/S2-profile/README.md`, `streams/S2-profile/CONTRACTS.md`
- Validation: editor diagnostics on touched S2 docs; verify the published answers stay consistent with the six-row S7 consumer-boundary question list in `streams/S7-hub/README.md`, with the frozen `schemas/profile.v0.json` and `schemas/capability-grant.v0.json` boundaries, and with `docs/gates/G2.md` and `STATUS.md`
- Constraints: Keep the slice docs-only and S2-owned. Do not mutate `schemas/profile.v0.json` or `schemas/capability-grant.v0.json`, do not reopen `streams/S7-hub/README.md` or `streams/S7-hub/BACKLOG.md` unless an answer exposes a contradiction, do not scaffold `crates/ferros-hub/`, do not define Home Assistant bridge internals, do not ratify handshake order, and do not claim G4 evidence.
- Last update: 2026-04-24

### WAVE-2026-04-24-11

- Title: Write the S2 consumer questions before naming an authoritative S7 pairing flow
- Status: done
- Priority: P1
- Gate: G4 runway
- Owning streams: S7 primary, S2 consumer awareness if pairing dependency wording shifts, S8 truth-sync if queue or stream docs move
- Goal: Turn the current open pairing questions plus the landed six-checkpoint pairing map into the explicit S2 consumer-question list S7 still needs answered before naming an authoritative pairing flow, without widening into `ferros-hub` scaffolding, Home Assistant bridge internals, or ratified handshake order.
- Anchor files: `streams/S7-hub/README.md`, `streams/S7-hub/BACKLOG.md`
- Validation: editor diagnostics on touched S7 docs; verify the consumer-question list stays consistent with `docs/hub/reference-hardware.md`, `streams/S7-hub/CONTRACTS.md`, `STATUS.md`, and `docs/gates/G4.md`, including reconciling any stale `streams/S7-hub/BACKLOG.md` row that still treats the landed checkpoint map as open
- Constraints: Treat the current open pairing questions and the landed checkpoint map as fixed input. Do not redefine `ProfileId` or `CapabilityGrant`, do not scaffold `crates/ferros-hub/`, do not define authoritative handshake steps, and do not claim G4 evidence. Do not reopen `docs/hub/reference-hardware.md` or shared truth surfaces unless the question list exposes a contradiction.
- Last update: 2026-04-24

### WAVE-2026-04-24-10

- Title: Map the first S7 pairing checkpoints against current seams
- Status: done
- Priority: P1
- Gate: G4 runway
- Owning streams: S7 primary, S8 truth-sync if queue or stream docs move
- Goal: Turn the newly defined bridge runway contract into a small pairing-checkpoint map across bootstrap, grant check, deny visibility, persistence, revocation, and re-registration using the current S2, S3, and S4 seams, without widening into `ferros-hub` scaffolding, HA fork work, or an authoritative pairing protocol.
- Anchor files: `docs/hub/reference-hardware.md`, `streams/S7-hub/README.md`
- Validation: editor diagnostics on touched S7 and hardware-runway docs; verify the checkpoint map stays consistent with `streams/S7-hub/CONTRACTS.md`, `streams/S7-hub/BACKLOG.md`, `STATUS.md`, and `docs/gates/G4.md`
- Constraints: Treat the landed bridge contract as fixed input. Do not reopen `streams/S7-hub/CONTRACTS.md`, `streams/S7-hub/BACKLOG.md`, or `streams/S7-hub/PROGRESS.md` unless a contradiction is found. Do not scaffold `crates/ferros-hub/`, define HA transport internals, or claim G4 evidence.
- Last update: 2026-04-24

### WAVE-2026-04-24-09

- Title: Define the first S7 Home Assistant bridge runway contract
- Status: done
- Priority: P1
- Gate: G4 runway
- Owning streams: S7 primary, S8 truth-sync if queue or stream docs move
- Goal: Define the first Home Assistant bridge runway contract at one bridge agent, one real entity minimum evidence, operator-visible deny attribution, restart-safe FERROS-side state, and the external HA fork boundary without widening into `ferros-hub` scaffolding, HA component internals, or claimed G4 evidence.
- Anchor files: `streams/S7-hub/CONTRACTS.md`, `streams/S7-hub/BACKLOG.md`, `streams/S7-hub/PROGRESS.md`
- Validation: editor diagnostics on touched S7 docs
- Constraints: Keep the slice docs-only and runway-only. Do not scaffold `crates/ferros-hub/`, do not change `Maangled/home-assistant`, do not freeze the reconnect or pairing protocol, and do not claim G4 evidence.
- Last update: 2026-04-24

### WAVE-2026-04-23-09

- Title: Execute S5 Phase A archive and link-hygiene pack
- Status: done
- Priority: P1
- Gate: post-G3 runway
- Owning streams: S5 primary, S8 truth-sync if archive surfaces move
- Goal: Verify inbound links, archive the inactive top-level HTML prototypes to `docs/legacy/`, and keep the real `site/` surface clean for the later local shell without starting localhost UI work yet.
- Anchor files: `site/index.html`, `streams/S5-ux/DOCS-HTML-PROTOTYPE-AUDIT.md`, `docs/`, `docs/legacy/`
- Validation: editor diagnostics on touched files; grep inbound references before moving any prototype files so active links are not broken
- Constraints: Keep `docs/agent-command-center.html` and `docs/forge-workbench.html` active. Do not start the S5 Phase B local web shell in this wave.
- Last update: 2026-04-24

### WAVE-2026-04-24-08

- Title: Harden read-first JSON-RPC error coverage
- Status: done
- Priority: P1
- Gate: post-G3 contract hardening
- Owning streams: S3 primary, S4 host awareness, S5 consumer awareness, S8 truth-sync if queue or stream docs move
- Goal: Strengthen the current read-first JSON-RPC boundary by locking negative-path behavior for the four existing read methods and proving one live `POST /rpc` error path through the localhost shell host without widening into new methods, transport changes, or write actions.
- Anchor files: `crates/ferros-node/src/lib.rs`, `streams/S3-agent-center/PROGRESS.md`, `streams/S3-agent-center/CONTRACTS.md`
- Validation: `cargo test -p ferros-node agent_read_rpc_`; `cargo test -p ferros-node shell_listener_posts_json_rpc_`
- Constraints: Keep the slice read-first. Do not add write actions, subscriptions, health endpoints, transport adapters, or shared contract changes unless the existing read contract semantics actually move.
- Last update: 2026-04-24

### WAVE-2026-04-24-07

- Title: Map the first x86_64 S7 bring-up contract
- Status: done
- Priority: P1
- Gate: G4 runway
- Owning streams: S7 primary, S8 truth-sync if queue or stream docs move
- Goal: Turn the active S7 runway into a concrete first-device contract by choosing the Pack B `x86_64` lane as the preferred first bring-up target, mapping unchecked G4 evidence to upstream seams and S7-owned proof points, and keeping the Home Assistant lab topology honest without widening into `ferros-hub` or HA bridge code.
- Anchor files: `streams/S7-hub/README.md`, `streams/S7-hub/BACKLOG.md`, `streams/S7-hub/CONTRACTS.md`, `docs/hub/reference-hardware.md`, `streams/S7-hub/PROGRESS.md`
- Validation: editor diagnostics on touched S7 and hardware-runway docs
- Constraints: Keep the slice in runway mode. Do not scaffold `crates/ferros-hub/`, do not freeze pairing protocol order, do not claim G4 evidence, and do not redefine `LAUNCH.md` or `docs/gates/G4.md` from this wave.
- Last update: 2026-04-24

### WAVE-2026-04-24-06

- Title: Add listener-level localhost shell smoke coverage
- Status: done
- Priority: P1
- Gate: post-G3 host hardening
- Owning streams: S4 primary, S5 consumer awareness, S8 truth-sync if queue or stream surfaces move
- Goal: Harden the current `ferros-node shell` host seam by exercising the real TCP listener path for `GET /` and `POST /rpc` without widening into new JSON/RPC methods, transport changes, or `ferros-hub` work.
- Anchor files: `crates/ferros-node/src/lib.rs`, `streams/S4-runtime/PROGRESS.md`
- Validation: `cargo test -p ferros-node shell_`
- Constraints: Keep the slice read-first and host-local. Do not add write actions, health endpoints, transport adapters, persistence changes, or hub semantics in this wave.
- Last update: 2026-04-24

### WAVE-2026-04-24-05

- Title: Add same-origin localhost shell acceptance coverage
- Status: done
- Priority: P1
- Gate: post-G3 consumer reliability
- Owning streams: S5 primary, S4 support, S8 truth-sync if queue or stream surfaces move
- Goal: Add a dedicated same-origin acceptance harness for the live `ferros-node shell` surface so the real localhost shell can be black-box tested through `GET /` and `POST /rpc` without widening into new JSON/RPC methods, privileged writes, or the remaining Phase A archive work.
- Anchor files: `harnesses/localhost-shell-acceptance-harness.html`, `crates/ferros-node/src/lib.rs`, `streams/S5-ux/BACKLOG.md`, `streams/S5-ux/PROGRESS.md`
- Validation: `cargo test -p ferros-node shell_route_`; live browser validation at `http://127.0.0.1:4317/harnesses/localhost-shell-acceptance.html` against the real localhost shell
- Constraints: Keep the slice read-first. Do not add write actions, new JSON/RPC methods, transport changes beyond serving the harness, or Phase A archive moves in this wave.
- Last update: 2026-04-24

### WAVE-2026-04-23-B01

- Title: Start S5 local web shell against JSON/RPC
- Status: done
- Priority: P3
- Gate: post-G3 S3 contract
- Owning streams: S5 primary, S3 dependency
- Goal: Begin the local agent-center web shell implementation.
- Anchor files: `site/`, `crates/ferros-agents/`, `streams/S5-ux/`
- Validation: Passed `cargo test -p ferros-node`; live browser validation against `http://127.0.0.1:4317/` confirmed real agent, grant-state, and deny-log reads through `/rpc`; editor diagnostics stayed clean on the touched S5, status, and orchestration docs
- Constraints: Start against the landed read-first S3 JSON/RPC routes. Keep the first shell slice read-heavy and do not widen into undocumented write actions.
- Last update: 2026-04-24

### WAVE-2026-04-24-04

- Title: Publish the first S3 JSON/RPC read contract
- Status: done
- Priority: P0
- Gate: post-G3 contract spine
- Owning streams: S3 primary, S5 consumer review, S4 support if host/runtime proof moves, S8 truth-sync after landing
- Goal: Define and land the first read-first S3 JSON/RPC surface for agent list, describe, grant-state, and deny-log style data without widening into privileged writes or Phase B shell rendering in the same wave.
- Anchor files: `streams/S3-agent-center/CONTRACTS.md`, `crates/ferros-agents/`, `crates/ferros-node/src/lib.rs`, `streams/S5-ux/PHASE-B-SHELL-WIREFRAME.md`, `docs/contracts/CONTRACTS-OVERVIEW.md`
- Validation: `cargo test -p ferros-agents -p ferros-node`; focused contract and route-shape coverage for the touched host and contract surfaces; editor diagnostics on touched truth-sync files
- Constraints: Keep the surface read-first. Do not start Phase B shell rendering or privileged write actions in this wave. Keep `docs/contracts/CONTRACTS-OVERVIEW.md` aligned only if the owning S3 contract surfaces move in the same wave.
- Last update: 2026-04-24

### WAVE-2026-04-24-03

- Title: Record the first green hosted CI proof and close G3
- Status: done
- Priority: P0
- Gate: G3
- Owning streams: S3 primary, S4 primary, S8 truth-sync if gate, status, or queue surfaces move
- Goal: Record the first green hosted CI run reference for the landed `cargo check -p ferros-core --no-default-features` plus `cargo run --bin ferros -- demo` workflow path, then close G3 and activate the next post-G3 queue state without widening into JSON/RPC, S5 shell implementation, or `ferros-hub` scaffolding.
- Anchor files: `docs/gates/G3.md`, `docs/gates/G4.md`, `STATUS.md`, `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`, `streams/S4-runtime/BACKLOG.md`, `streams/S5-ux/BACKLOG.md`, `streams/S7-hub/README.md`, `streams/S7-hub/PROGRESS.md`
- Validation: Confirm the current `.github/workflows/ci.yml` still contains `cargo check -p ferros-core --no-default-features` and `cargo run --bin ferros -- demo`; confirm the GitHub Actions CI workflow page records CI #20 (`run 24902870499`, commit `8383b67` on `main`) as completed successfully; verify editor diagnostics are clean on touched docs
- Constraints: Keep the slice inside hosted-evidence capture and truth-sync. Do not start S3 JSON/RPC, S5 Phase B implementation, or S7 code scaffolding in this wave.
- Last update: 2026-04-24

### WAVE-2026-04-23-08

- Title: Start S7 pairing and hardware design pack
- Status: done
- Priority: P1
- Gate: G4 runway
- Owning streams: S7 primary, S8 truth-sync if docs move, S2 consumer awareness if pairing contract wording shifts
- Goal: Keep the S7 runway moving by finishing the reference hardware recipe and documenting the current pairing constraints, open questions, and grant-aware design posture without freezing authoritative pairing semantics yet.
- Anchor files: `streams/S7-hub/README.md`, `streams/S7-hub/BACKLOG.md`, `docs/hub/reference-hardware.md`, `docs/hub/`
- Validation: editor diagnostics on touched docs; verify the pairing and hardware docs stay consistent with the current S7 README and backlog boundaries
- Constraints: Keep the slice to S7 design and documentation runway. Keep pairing notes provisional, do not scaffold `crates/ferros-hub/`, do not start the HA bridge, and do not claim G4 evidence in this wave.
- Last update: 2026-04-24

### WAVE-2026-04-24-02

- Title: Close G2 with the remaining profile CLI evidence
- Status: done
- Priority: P0
- Gate: G2
- Owning streams: S2 primary, S8 truth-sync after landing, S1 support only if a repo-backed Linux proof surface needs to move
- Goal: Finish the only remaining G2 blocker by landing a repo-backed `ferros profile export`, `import`, `grant`, and `revoke` path, including the minimum local persistence boundary for key material and signed grant state, without widening the frozen published v0 contracts or changing downstream S3, S4, or S7 consumer boundaries.
- Anchor files: `crates/ferros-profile/src/lib.rs`, `crates/ferros-node/src/lib.rs`, `crates/ferros-node/src/bin/ferros.rs`, `schemas/profile.v0.json`, `schemas/capability-grant.v0.json`, `docs/gates/G2.md`, `STATUS.md`
- Validation: `cargo test -p ferros-profile -p ferros-node`; repo-backed real-binary proof that `ferros profile init`, `grant`, `export`, `import`, `revoke`, and `show` succeed against real files and preserve the frozen `profile.v0.json` and `capability-grant.v0.json` boundaries
- Constraints: Keep `profile.v0.json` frozen as the unsigned published v0 consumer contract. Keep `SignedProfileDocument` Rust-local at v0. Do not mutate `capability-grant.v0.json`. Do not widen S3 or S4 runtime and manifest contracts, S7 pairing semantics, optional passphrase wrap, or post-G2 UX work. If a new on-disk bundle format is needed, keep it local to the CLI and store surface rather than publishing a new shared schema.
- Last update: 2026-04-24

### WAVE-2026-04-24-01

- Title: Freeze profile.v0 and settle the signed-profile v0 boundary
- Status: done
- Priority: P0
- Gate: G2
- Owning streams: S2 primary, S8 truth-sync after landing, S3 and S4 consumer awareness only if the published contract wording shifts
- Goal: Convert the current `schemas/profile.v0.json` freeze candidate into the actual frozen v0 contract by deciding whether `SignedProfileDocument` stays Rust-local at v0, landing only the minimal schema and parity changes required for freeze, and propagating that final contract through shared validation and truth surfaces without widening into the remaining profile CLI verbs.
- Anchor files: `schemas/profile.v0.json`, `crates/ferros-profile/src/lib.rs`, `schemas/fixtures/profile-valid.json`, `schemas/fixtures/signed-profile-valid.json`, `tools/generate-harness-constants.ps1`, `harnesses/_constants.js`, `harnesses/ferros-contract-validator.html`, `streams/S2-profile/CONTRACTS.md`, `docs/gates/G2.md`, `docs/contracts/CONTRACTS-OVERVIEW.md`, `STATUS.md`
- Validation: `cargo test -p ferros-profile`; if fixture or schema coverage changes, regenerate harness constants and confirm `harnesses/ferros-contract-validator.html` still accepts the frozen profile fixture set; confirm editor diagnostics are clean on touched S2 and truth-sync files
- Constraints: Keep the slice inside profile.v0 freeze semantics and freeze evidence. Do not start `ferros profile export | import | grant | revoke` in this wave. Do not mutate `schemas/capability-grant.v0.json`. Do not publish a separate signed-profile schema unless S2 can prove the unsigned profile.v0 contract cannot be frozen cleanly without it.
- Last update: 2026-04-24

### WAVE-2026-04-23-07

- Title: Tighten G3 evidence and CI demo proof
- Status: done
- Priority: P0
- Gate: G3
- Owning streams: S3 primary, S4 primary, S8 truth-sync if gate or status surfaces move
- Goal: Sync G3-facing docs to the already-landed S4 property tests and add a repo-backed CI proof for `cargo run --bin ferros -- demo` without widening into JSON/RPC or reusable host work.
- Anchor files: `.github/workflows/ci.yml`, `docs/gates/G3.md`, `STATUS.md`, `streams/S4-runtime/BACKLOG.md`, `docs/contracts/CONTRACTS-OVERVIEW.md`
- Validation: `cargo test -p ferros-core -p ferros-runtime -p ferros-agents -p ferros-node`; `cargo check -p ferros-core --no-default-features`; `cargo run --bin ferros -- demo`
- Constraints: Keep the slice inside G3 evidence, CI proof, and truth-sync. Do not start S3 JSON/RPC, reusable host work, or S5 shell implementation in this wave.
- Last update: 2026-04-23

### WAVE-2026-04-23-06

- Title: Land `KeyPair` and signed profile round-trip evidence
- Status: done
- Priority: P0
- Gate: G2
- Owning streams: S2 primary, S8 truth-sync if gate or contract docs move
- Goal: Add the first S2-owned key-material surface plus a signed profile round-trip path so `ferros-profile` can create a fresh profile, serialize it, sign it, verify it, and prove the contract with focused tests and fixtures without widening into the remaining profile CLI verbs.
- Anchor files: `crates/ferros-profile/src/lib.rs`, `crates/ferros-profile/Cargo.toml`, `schemas/profile.v0.json`, `schemas/fixtures/`, `docs/gates/G2.md`, `STATUS.md`, `streams/S2-profile/CONTRACTS.md`, `streams/S2-profile/PROGRESS.md`
- Validation: `cargo test -p ferros-profile`; update harness or truth surfaces only if the profile schema contract actually changes
- Constraints: Keep the slice inside S2 key material and signed profile evidence. Do not start `ferros profile export | import | grant | revoke` in this wave. Avoid changing downstream S3/S4 consumer boundaries unless the signed profile contract truly requires it.
- Last update: 2026-04-23

### WAVE-2026-04-23-05

- Title: Add Linux-backed `ferros profile init` to `show` proof
- Status: done
- Priority: P1
- Gate: G2
- Owning streams: S2 primary, S1 support if CI or workflow surfaces move, S8 truth-sync if gate docs change
- Goal: Land a repo-backed Linux proof for `ferros profile init` followed by `ferros profile show`, using the already-landed minimal CLI path without widening into `export | import | grant | revoke`.
- Anchor files: `.github/workflows/ci.yml`, `crates/ferros-node/src/bin/ferros.rs`, `crates/ferros-node/src/lib.rs`, `docs/gates/G2.md`, `STATUS.md`
- Validation: local `cargo test -p ferros-profile -p ferros-node`; repo-hosted Linux workflow or equivalent scripted proof for `ferros profile init` then `ferros profile show`
- Constraints: Keep the slice focused on Linux-backed evidence for the current `init | show` path. Do not start the remaining profile CLI subcommands in this wave.
- Last update: 2026-04-23

### WAVE-2026-04-23-04

- Title: Freeze profile.v0 golden fixture evidence
- Status: done
- Priority: P0
- Gate: G2
- Owning streams: S2 primary, S8 truth-sync if docs or harness surfaces move
- Goal: Add the dedicated frozen `schemas/fixtures/profile-valid.json` artifact, prove it matches `schemas/profile.v0.json`, and sync any harness or gate surfaces that still assume profile freeze evidence is missing.
- Anchor files: `schemas/profile.v0.json`, `schemas/fixtures/profile-valid.json`, `crates/ferros-profile/src/lib.rs`, `tools/generate-harness-constants.ps1`, `harnesses/_constants.js`, `harnesses/ferros-contract-validator.html`, `docs/gates/G2.md`, `STATUS.md`
- Validation: `cargo test -p ferros-profile`; regenerate harness constants if fixture coverage changes; confirm H1 contract validator still passes for the profile schema set
- Constraints: Keep the slice to profile fixture freeze evidence and truth-sync. Do not widen into new CLI subcommands or profile signing work.
- Last update: 2026-04-23

### WAVE-2026-04-23-03

- Title: Land the minimal S2 profile CLI slice
- Status: done
- Priority: P2
- Gate: G2
- Owning streams: S2 primary, S3 consumer awareness if contracts shift
- Goal: Ship the smallest useful `ferros profile init | show` path with filesystem-backed storage, using the already-landed `ProfileStore` as the persistence boundary.
- Anchor files: `crates/ferros-profile/src/lib.rs`, `crates/ferros-node/src/bin/ferros.rs`, `crates/ferros-node/src/lib.rs`, `docs/gates/G2.md`
- Validation: `cargo test -p ferros-profile -p ferros-node`
- Constraints: Keep the slice to `init` and `show` unless the implementation naturally supports one more subcommand with test coverage. Do not widen into import/export or signing in the same wave.
- Last update: 2026-04-23

### WAVE-2026-04-23-02

- Title: Add S4 policy property tests
- Status: done
- Priority: P1
- Gate: G3
- Owning streams: S4 primary
- Goal: Add property tests for `DenyByDefaultPolicy` that prove deny-by-default invariants across grant ordering and profile/capability mismatches without widening into unrelated runtime work.
- Anchor files: `crates/ferros-core/src/capability.rs`, `crates/ferros-core/tests/capability_policy.rs`, `crates/ferros-core/Cargo.toml`
- Validation: `cargo test -p ferros-core`
- Constraints: Keep the slice focused on policy properties and test dependencies. Do not claim full embedded readiness or broader `no_std` completion.
- Last update: 2026-04-23

### WAVE-2026-04-23-01

- Title: Freeze `CapabilityGrant` signing and verification evidence
- Status: done
- Priority: P0
- Gate: G2
- Owning streams: S2 primary, S8 truth-sync if docs move
- Goal: Finish the shared contract, harness, and gate-truth evidence for the now-landed signed and verifiable `CapabilityGrant` path without widening beyond the frozen stripped-payload signing rule.
- Anchor files: `schemas/capability-grant.v0.json`, `docs/contracts/CONTRACTS-OVERVIEW.md`, `tools/generate-harness-constants.ps1`, `harnesses/_constants.js`, `harnesses/ferros-contract-validator.html`, `docs/gates/G2.md`
- Validation: `cargo test -p ferros-profile`; confirm shared contract and harness surfaces match `schemas/capability-grant.v0.json`
- Constraints: Keep the slice inside G2 evidence. Do not start the full profile CLI here. Limit follow-up to contract, harness, and gate truth-sync for the signed `CapabilityGrant` boundary.
- Last update: 2026-04-23

### WAVE-2026-04-23-D01

- Title: Propagate shared revocation semantics through S2 and S3
- Status: done
- Priority: P0
- Gate: G2/G3 boundary hygiene
- Owning streams: S2, S3, S4
- Goal: Ensure revoked grants no longer authorize work through the shared grant and manifest boundary.
- Anchor files: `crates/ferros-profile/src/lib.rs`, `crates/ferros-agents/src/manifest.rs`
- Validation: `cargo test -p ferros-profile -p ferros-agents`; `cargo test -p ferros-node`
- Constraints: Keep the fix at the shared boundary, not only the node adapter.
- Last update: 2026-04-23
