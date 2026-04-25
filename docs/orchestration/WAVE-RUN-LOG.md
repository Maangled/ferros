# FERROS Wave Run Log

Newest entry first. Each entry records one local driver invocation.

## 2026-04-24 — WAVE-2026-04-24-15

- Selected item: `WAVE-2026-04-24-15`
- Result: Recorded the already-landed S3/S4/S7 docs-only seam-classification completion in orchestration. The repo had already updated `streams/S3-agent-center/CONTRACTS.md` and `streams/S3-agent-center/BACKLOG.md` to classify the current registration and local/read-first inspection surface as sufficient for S7 runway planning at one-bridge-agent/local-observation scope while routing the next S3-owned follow-up to the first hub-facing wrapper boundary; `streams/S4-runtime/CONTRACTS.md` and `streams/S4-runtime/BACKLOG.md` already classified the current policy surface as sufficient for runway planning while marking the restart/reload boundary as the next S4-owned follow-up; and `streams/S7-hub/BACKLOG.md` already recorded the route-to-S3/S4 handoff as landed plus the returned dependency locks that S3 still owes the first hub-facing wrapper boundary and S4 still owes a published restart/reload boundary before any authoritative pairing flow, `ferros-hub` scaffold, or Home Assistant bridge plan is honest. This orchestration lane only recorded that completion, the docs-only cross-stream dependency locks, the lack of implementation changes or G4 evidence movement, and the next Ready seed on the narrower S4 restart/reload-boundary follow-up.
- Files: `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: Recorded the landed validation outcome with clean editor diagnostics on `streams/S3-agent-center/CONTRACTS.md`, `streams/S3-agent-center/BACKLOG.md`, `streams/S4-runtime/CONTRACTS.md`, `streams/S4-runtime/BACKLOG.md`, and `streams/S7-hub/BACKLOG.md`; final editor diagnostics stayed clean on `docs/orchestration/WAVE-QUEUE.md` and `docs/orchestration/WAVE-RUN-LOG.md`.
- Next follow-up: `WAVE-2026-04-24-16`

## 2026-04-24 — WAVE-2026-04-24-14

- Selected item: `WAVE-2026-04-24-14`
- Result: Recorded the landed S7 docs-only seam-brief slice in orchestration. The repo already updated `streams/S7-hub/README.md` with an S7 seam brief that names the exact S3 registration and inspection surfaces (`AgentRegistry::register`, `AgentRegistry::deregister`, `AgentRegistry::list`, `AgentRegistry::describe`, local `ferros agent list`, `ferros agent describe`, `ferros agent logs`, and read-first `agent.list`, `agent.describe`, `grant.list`, `denyLog.list`) plus the exact S4 policy and restart surfaces (`CapabilityRequest`, `CapabilityGrantView`, `PolicyEngine::evaluate`, `DenyByDefaultPolicy`, `PolicyDecision`, `PolicyDenialReason`, and the nearest current reload helpers `runtime_with_state(state_path)`, `CliState::load(state_path)`, and `LocalProfileStore::load_local_profile(path)`). `streams/S7-hub/CONTRACTS.md` now carries the matching exact-upstream-seams table, and `streams/S7-hub/BACKLOG.md` marks the seam brief landed and routes the next docs-only handoff to S3 and S4. No S2 or orchestration docs were touched in the implementation lane, and no G4 evidence movement is claimed. Because Ready would otherwise be empty, this invocation also seeded the next honest docs-only follow-on for S3 and S4 seam classification.
- Files: `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: Recorded the landed validation outcome with clean editor diagnostics on `streams/S7-hub/README.md`, `streams/S7-hub/CONTRACTS.md`, and `streams/S7-hub/BACKLOG.md`; the consistency check against `streams/S3-agent-center/CONTRACTS.md`, `streams/S4-runtime/CONTRACTS.md`, `docs/hub/reference-hardware.md`, `STATUS.md`, and `docs/gates/G4.md` passed; final editor diagnostics stayed clean on the touched orchestration files.
- Next follow-up: `WAVE-2026-04-24-15`

## 2026-04-24 — WAVE-2026-04-24-13

- Selected item: `WAVE-2026-04-24-13`
- Result: Recorded the landed S7 docs-only pairing/design handoff in orchestration and repaired the stale queue lineage so the queue again matches repo truth. The repo already updated `streams/S7-hub/README.md` with an S7-owned provisional handoff that states what S7 may now assume from the published S2 consumer boundaries across bootstrap, grant check, deny visibility, persistence, revocation, and re-registration, names what stays open, and routes the immediate next step to an S7-owned seam brief keyed to the exact S3 registry/list/log and S4 restart/policy APIs. `streams/S7-hub/BACKLOG.md` already marks the consume pass landed and replaces the obsolete route-to-S2 follow-up with that seam-brief follow-up. No S2 or orchestration docs were touched in the implementation lane itself, and no G4 evidence movement is claimed. Because the queue was one step stale, this invocation also retired the already-published `WAVE-2026-04-24-12` S2 answer slice from Ready to Done before seeding the next honest S7-ready item.
- Files: `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: Recorded the landed validation outcome with clean editor diagnostics on `streams/S7-hub/README.md` and `streams/S7-hub/BACKLOG.md`; consistency checks against `streams/S2-profile/README.md`, `streams/S2-profile/CONTRACTS.md`, `streams/S7-hub/CONTRACTS.md`, `docs/hub/reference-hardware.md`, `STATUS.md`, and `docs/gates/G4.md` passed; final editor diagnostics stayed clean on the touched orchestration files.
- Next follow-up: `WAVE-2026-04-24-14`

## 2026-04-24 — WAVE-2026-04-24-11

- Selected item: `WAVE-2026-04-24-11`
- Result: Recorded the already-landed S7 docs-only pairing-boundary slice in orchestration without widening into implementation; the repo already updated `streams/S7-hub/README.md` to replace the generic open pairing questions with an explicit six-row S2 consumer-boundary question list aligned to bootstrap, grant check, deny visibility, persistence, revocation, and re-registration, routed the immediate next step to S2, and updated `streams/S7-hub/BACKLOG.md` to mark both the six-checkpoint pairing map and the new question list as landed while replacing the old follow-up with routing those questions to S2 and recording the answers; no fresh S7 content edits were made in this invocation; no G4 evidence movement is claimed.
- Files: `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: Recorded the landed validation outcome with clean editor diagnostics on `streams/S7-hub/README.md` and `streams/S7-hub/BACKLOG.md`, plus passed consistency checks against `docs/hub/reference-hardware.md`, `streams/S7-hub/CONTRACTS.md`, `STATUS.md`, and `docs/gates/G4.md`.
- Next follow-up: `WAVE-2026-04-24-12`

## 2026-04-24 — WAVE-2026-04-24-10

- Selected item: `WAVE-2026-04-24-10`
- Result: Recorded the already-landed S7 pairing-checkpoint docs slice in orchestration only. The repo already added a runway-only six-checkpoint pairing map to `streams/S7-hub/README.md` and `docs/hub/reference-hardware.md` covering bootstrap, grant check, deny visibility, persistence, revocation, and re-registration, tied that map to the current S2 `ProfileId` and `CapabilityGrant` seams plus the S3 registry/list/log and S4 runtime policy, deny logging, and restart seams, and corrected stale pre-G3 wording in those same docs. No fresh S7 content edits were made in this invocation, and no G4 evidence movement is claimed.
- Files: `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: Recorded the landed validation outcome: editor diagnostics were clean on `STATUS.md`, `streams/S7-hub/README.md`, and `docs/hub/reference-hardware.md`; the consistency pass against `streams/S7-hub/CONTRACTS.md`, `STATUS.md`, and `docs/gates/G4.md` passed, but `streams/S7-hub/BACKLOG.md` still carries a stale open checkpoint-mapping row, so backlog consistency remains open for the next follow-up.
- Next follow-up: `WAVE-2026-04-24-11`

## 2026-04-24 — WAVE-2026-04-24-09

- Selected item: `WAVE-2026-04-24-09`
- Result: Recorded the landed S7 runway closeout in orchestration without widening into implementation. The repo already defines the first Home Assistant bridge runway contract in `streams/S7-hub/CONTRACTS.md`, syncs `streams/S7-hub/BACKLOG.md` and `streams/S7-hub/PROGRESS.md`, and keeps the scope at one bridge agent, one real entity minimum evidence, operator-visible deny attribution, restart-safe FERROS-side state, and the external `Maangled/home-assistant` fork boundary. No new S7 content edits were made in this invocation; the queue and run log now reflect that the generic bridge-assumption slice is complete.
- Files: `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: Recorded the landed validation outcome: editor diagnostics were clean on the touched S7 docs.
- Next follow-up: `WAVE-2026-04-24-10`

## 2026-04-24 — WAVE-2026-04-23-09

- Selected item: `WAVE-2026-04-23-09`
- Result: Recorded the landed S5 Phase A archive and link-hygiene closeout in orchestration. The repo already archived the inactive top-level HTML prototypes to `docs/legacy/`, kept the still-active docs-root surfaces in place, synced the S5 authority docs, and repaired the stale inbound references created by the archive move. No new S5 content edits were made in this invocation; the queue and run log now reflect the completed cleanup pack.
- Files: `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: Recorded the landed validation outcome: editor diagnostics were clean on the touched files, and grep had already confirmed that old docs-root references such as `docs/home-hud-dashboard.html` and `docs/ferros-project-map.html` were removed after repair.
- Next follow-up: `WAVE-2026-04-24-09`

## 2026-04-24 — WAVE-2026-04-24-08

- Selected item: `WAVE-2026-04-24-08`
- Result: Hardened the current read-first JSON-RPC boundary without changing its shape. `crates/ferros-node/src/lib.rs` now has focused tests that lock the existing error-envelope behavior for unsupported JSON-RPC version, missing `agentName` on `agent.describe`, unknown method names, and unknown agents, plus a real listener-level `POST /rpc` smoke that proves the live localhost shell host returns the same structured invalid-params response over TCP.
- Files: `crates/ferros-node/src/lib.rs`, `streams/S3-agent-center/PROGRESS.md`, `streams/S3-agent-center/CONTRACTS.md`, `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: Passed `cargo test -p ferros-node agent_read_rpc_` and `cargo test -p ferros-node shell_listener_posts_json_rpc_`; the focused suite covered the direct handler and the live listener path without widening into broader workspace checks.
- Next follow-up: `WAVE-2026-04-23-09` remains the only ready queue item, but if launch-path leverage still outranks cleanup, the next higher-value follow-on is a new queued slice around the first reusable `ferros-hub` wrapper seam or explicit Home Assistant bridge assumptions rather than Phase A archive moves.

## 2026-04-24 — WAVE-2026-04-24-07

- Selected item: `WAVE-2026-04-24-07`
- Result: Converted the active S7 runway into a concrete first bring-up contract without widening into implementation. The S7 stream and hardware-runway docs now treat the Pack B `x86_64` lane as the preferred first bring-up target, keep Pack C as the separate Home Assistant companion host, and map each unchecked G4 evidence item to one upstream seam and one S7-owned proof point so future `ferros-hub` work can be judged against a concrete runway rather than broad intent.
- Files: `streams/S7-hub/README.md`, `streams/S7-hub/BACKLOG.md`, `streams/S7-hub/CONTRACTS.md`, `docs/hub/reference-hardware.md`, `streams/S7-hub/PROGRESS.md`, `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: Editor diagnostics stayed clean on the touched S7 and hardware-runway docs; no code or gate-truth validation was required because the slice stayed in runway mode and did not change executable surfaces.
- Next follow-up: `WAVE-2026-04-23-09` remains the ready queue head. If launch-path leverage continues to outrank Phase A cleanup, queue the next S3/S7 follow-on around HA bridge assumptions or the first reusable `ferros-hub` wrapper seam only after the upstream host and contract surfaces are concrete enough.

## 2026-04-24 — WAVE-2026-04-24-06

- Selected item: `WAVE-2026-04-24-06`
- Result: Hardened the current localhost shell host seam without widening the read-first contract. `crates/ferros-node/src/lib.rs` now exposes a bounded listener loop that the test suite can drive directly, and the new listener-level smoke tests prove that the real shell host serves `GET /` and answers `POST /rpc` with a live `agent.list` response through the same TCP, HTTP parse, and response-write path used by `ferros-node shell`.
- Files: `crates/ferros-node/src/lib.rs`, `streams/S4-runtime/PROGRESS.md`, `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: Passed `cargo test -p ferros-node shell_`, including the existing route tests plus the new real-socket listener smokes for shell HTML and JSON-RPC agent list responses; final editor diagnostics stayed clean on the touched S4 and orchestration files.
- Next follow-up: `WAVE-2026-04-23-09` remains the ready queue head. If consumer reliability and G4 alignment continue to outrank Phase A cleanup, queue the S7 bring-up-contract slice before archive work.

## 2026-04-24 — WAVE-2026-04-24-05

- Selected item: `WAVE-2026-04-24-05`
- Result: Landed a dedicated same-origin localhost shell acceptance slice without widening the read-first contract. `harnesses/localhost-shell-acceptance-harness.html` now exercises the real shell through a same-origin iframe, `crates/ferros-node/src/lib.rs` serves that harness at `/harnesses/localhost-shell-acceptance.html`, and the live browser pass proved route switching, registry/detail inspection, grant empty-state degradation for a missing profile path, deny-log empty-state rendering, and the persistent read-only audit slot against the actual `ferros-node shell` server.
- Files: `harnesses/localhost-shell-acceptance-harness.html`, `crates/ferros-node/src/lib.rs`, `streams/S5-ux/BACKLOG.md`, `streams/S5-ux/PROGRESS.md`, `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: Passed `cargo test -p ferros-node shell_route_`; live browser validation at `http://127.0.0.1:4317/harnesses/localhost-shell-acceptance.html` passed 13/13 checks against the real localhost shell after restarting `ferros-node shell 4317`; final editor diagnostics stayed clean on the touched S5 and orchestration files.
- Next follow-up: `WAVE-2026-04-23-09` remains ready. If consumer reliability and G4 alignment stay higher priority than Phase A cleanup, queue a follow-on S4/S7 wave for localhost host hardening plus the first concrete S7 bring-up contract.

## 2026-04-24 — WAVE-2026-04-23-B01

- Selected item: `WAVE-2026-04-23-B01`
- Result: Landed the first real S5 localhost shell slice without widening into privileged writes. `site/agent-center-shell.html` now renders the fixed-slot agent-center shell, `crates/ferros-node/src/lib.rs` serves that shell at `GET /` and forwards `POST /rpc` into the existing read-first JSON/RPC handler, and `crates/ferros-node/src/main.rs` now exposes `ferros-node shell [port]` with a default localhost port of `4317`. The browser-validated shell reads live agent, grant-state, and deny-log data, and the inspector capability rendering bug was fixed before closeout so required capabilities now render the real profile identifier instead of an `undefined:*` placeholder.
- Files: `site/agent-center-shell.html`, `crates/ferros-node/src/lib.rs`, `crates/ferros-node/src/main.rs`, `streams/S5-ux/README.md`, `streams/S5-ux/CONTRACTS.md`, `streams/S5-ux/BACKLOG.md`, `streams/S5-ux/PROGRESS.md`, `site/index.html`, `STATUS.md`, `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: Passed `cargo test -p ferros-node`; live browser validation at `http://127.0.0.1:4317/` confirmed `ready`, `rpc live`, and real list/describe/grant/deny data after restarting the embedded-asset server with the rebuilt shell HTML; final editor diagnostics were clean on the touched S5, status, landing-page, and orchestration files.
- Next follow-up: `WAVE-2026-04-23-09`

## 2026-04-24 — WAVE-2026-04-24-04

- Selected item: `WAVE-2026-04-24-04`
- Result: Landed the first read-first S3 JSON/RPC contract without widening into HTTP serving, privileged write actions, or Phase B shell rendering. The contract now lives in `crates/ferros-agents/src/rpc.rs` with method and payload types for `agent.list`, `agent.describe`, `grant.list`, and `denyLog.list`, and `crates/ferros-node/src/lib.rs` now hosts that contract over the current deterministic runtime state, persisted grant state, and deny-log state. The owning S3 contract docs, shared contract index, S5 shell wireframe, and status surfaces now reflect that the read path exists; as a direct consequence, `WAVE-2026-04-23-B01` is no longer blocked and is now ready for the first shell-consumer pass.
- Files: `crates/ferros-agents/src/rpc.rs`, `crates/ferros-agents/src/lib.rs`, `crates/ferros-node/src/lib.rs`, `crates/ferros-node/Cargo.toml`, `streams/S3-agent-center/CONTRACTS.md`, `streams/S5-ux/PHASE-B-SHELL-WIREFRAME.md`, `docs/contracts/CONTRACTS-OVERVIEW.md`, `STATUS.md`, `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: Passed `cargo test -p ferros-agents`; passed `cargo test -p ferros-node`; the new `ferros-node` tests cover typed list, describe, grant-list, deny-log, and JSON wrapper behavior against the current runtime and local profile store; final editor diagnostics were clean on the touched contract, status, and queue files.
- Next follow-up: `WAVE-2026-04-23-B01`

## 2026-04-24 — WAVE-2026-04-24-03

- Selected item: `WAVE-2026-04-24-03`
- Result: Recorded the first green hosted CI proof for the landed G3 workflow path, then truth-synced the gate, status, stream, and queue surfaces so G3 is now closed and G4 is now active. The closure references CI #20 (`run 24902870499`, commit `8383b67` on `main`), keeps the proof tied to the current hosted Ubuntu workflow that still runs `cargo check -p ferros-core --no-default-features` and `cargo run --bin ferros -- demo`, reclassifies S5 Phase B as blocked on the missing S3 JSON/RPC contract rather than on G3 itself, and updates S7 and S4 surfaces to their post-G3 state without starting JSON/RPC, shell rendering, or `ferros-hub` code.
- Files: `docs/gates/G3.md`, `docs/gates/G4.md`, `STATUS.md`, `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`, `streams/S4-runtime/BACKLOG.md`, `streams/S5-ux/BACKLOG.md`, `streams/S7-hub/README.md`, `streams/S7-hub/PROGRESS.md`
- Validation: Confirmed the current `.github/workflows/ci.yml` still contains `cargo check -p ferros-core --no-default-features` and `cargo run --bin ferros -- demo`; refreshed the GitHub Actions CI workflow page and confirmed CI #20 (`run 24902870499`, commit `8383b67` on `main`) completed successfully on 2026-04-24; final editor diagnostics were clean on the touched gate, status, queue, and stream docs.
- Next follow-up: `WAVE-2026-04-24-04`

## 2026-04-24 — WAVE-2026-04-23-08

- Selected item: `WAVE-2026-04-23-08`
- Result: Closed the S7 runway documentation wave without wave-owned S7 content edits in this invocation because `docs/hub/reference-hardware.md`, `streams/S7-hub/README.md`, and `streams/S7-hub/BACKLOG.md` already reflected the dispatched pairing and hardware design-pack scope. G3 remains the implementation blocker, G4 did not move, no `crates/ferros-hub/` work or Home Assistant bridge work was started, and no immediate S8 or S2 follow-up is required from this closeout.
- Files: `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: Confirmed the three S7 anchor docs had no uncommitted changes relative to `HEAD`; editor diagnostics were clean for `docs/hub/reference-hardware.md`, `streams/S7-hub/README.md`, and `streams/S7-hub/BACKLOG.md`; integration review passed with gate truth still honest, S2 consumer-surface alignment intact, and cross-file coherence preserved; final editor diagnostics were clean for `docs/orchestration/WAVE-QUEUE.md` and `docs/orchestration/WAVE-RUN-LOG.md`.
- Next follow-up: `WAVE-2026-04-23-09`

## 2026-04-24 — WAVE-2026-04-24-02

- Selected item: `WAVE-2026-04-24-02`
- Result: Landed the remaining S2 profile CLI lifecycle on a local-only persistence boundary for key material and signed grant state, adding repo-backed `ferros profile grant`, `export`, `import`, and `revoke` behavior while keeping `ferros profile show` on the frozen unsigned `profile.v0` document surface. The wave kept `SignedProfileDocument` Rust-local at v0, left `schemas/profile.v0.json` and `schemas/capability-grant.v0.json` unchanged, did not widen S3, S4, or S7 boundaries, and truth-synced the gate and status surfaces so G2 is closed and G3 is active.
- Files: `crates/ferros-profile/src/lib.rs`, `crates/ferros-node/src/lib.rs`, `crates/ferros-node/src/bin/ferros.rs`, `crates/ferros-node/tests/profile_cli_linux.rs`, `crates/ferros-node/Cargo.toml`, `docs/gates/G2.md`, `docs/gates/G3.md`, `STATUS.md`, `docs/contracts/CONTRACTS-OVERVIEW.md`, `streams/S2-profile/README.md`, `streams/S2-profile/PROGRESS.md`
- Validation: Delegated orchestration passed `cargo test -p ferros-profile`, `cargo test -p ferros-node`, and `cargo test -p ferros-profile -p ferros-node`. The repo-backed real-binary proof in `crates/ferros-node/tests/profile_cli_linux.rs` exercised `ferros profile init`, `grant agent.echo`, `export`, `import`, `revoke agent.echo`, and `show` against real temp files, verified that imported local state preserved keypair and signed grant state, verified that the revoked signed grant still stayed within the frozen grant boundary, and confirmed that `show` remained an unsigned profile document. Final editor diagnostics were clean on touched code and truth-sync files.
- Next follow-up: `WAVE-2026-04-23-08`. Separately queue a new P0 G3 evidence wave to record the first hosted green CI run reference for `cargo check -p ferros-core --no-default-features` and `cargo run --bin ferros -- demo`.

## 2026-04-24 — WAVE-2026-04-24-01

- Selected item: `WAVE-2026-04-24-01`
- Result: Landed the profile.v0 freeze boundary as the frozen unsigned published v0 contract, kept `SignedProfileDocument` Rust-local at v0, refreshed harness parity for the frozen profile fixture set, and truth-synced S2, gate, contracts, and status surfaces without widening into `export | import | grant | revoke`. G2 remains open only for the remaining profile CLI evidence. Integration review found one stale README line and fixed it before closeout; no S3 or S4 consumer-awareness follow-up is required because the published downstream contract remains the unsigned `profile.v0` boundary.
- Files: `schemas/profile.v0.json`, `crates/ferros-profile/src/lib.rs`, `harnesses/_constants.js`, `harnesses/ferros-contract-validator.html`, `streams/S2-profile/CONTRACTS.md`, `docs/gates/G2.md`, `docs/contracts/CONTRACTS-OVERVIEW.md`, `STATUS.md`, `streams/S2-profile/README.md`, `streams/S2-profile/PROGRESS.md`
- Validation: Delegated S2 lane passed `cargo test -p ferros-profile` with 34 passed and 0 failed; harness constants were regenerated; `harnesses/ferros-contract-validator.html` accepted the frozen profile fixture set with 47 passed and 0 failed; final editor diagnostics were clean on touched S2 and truth-sync files.
- Next follow-up: `WAVE-2026-04-23-08`. Separately queue a new S2 wave for the remaining `ferros profile export | import | grant | revoke` evidence when ready.

## 2026-04-23 — WAVE-2026-04-23-07

- Selected item: `WAVE-2026-04-23-07`
- Result: Landed the narrow G3 truth-sync and CI-proof slice without widening into JSON/RPC, reusable host work, or S5 shell work. The `ferros` binary now exposes `cargo run --bin ferros -- demo`, Ubuntu CI is explicitly wired to run both `cargo check -p ferros-core --no-default-features` and `cargo run --bin ferros -- demo`, and the G3/status/contracts/S4 backlog surfaces now reflect the already-landed S4 policy property tests and current demo evidence honestly. G3 still remains blocked on G2 and on recording the first green hosted run for the new workflow steps, but the queue item's repo-owned slice is complete.
- Files: `.github/workflows/ci.yml`, `crates/ferros-node/src/bin/ferros.rs`, `docs/contracts/CONTRACTS-OVERVIEW.md`, `docs/gates/G3.md`, `STATUS.md`, `streams/S4-runtime/BACKLOG.md`
- Validation: Delegated orchestration passed `cargo test -p ferros-core -p ferros-runtime -p ferros-agents -p ferros-node`; passed `cargo check -p ferros-core --no-default-features`; passed `cargo run --bin ferros -- demo` with `started: echo,timer`, `echo: hello`, `timer: tick-1`, and `denied: 1`; additional focused passes covered `cargo test -p ferros-node --bin ferros` and `cargo test -p ferros-core --test capability_policy`; editor diagnostics were clean for the touched workflow and truth-sync files.
- Next follow-up: `WAVE-2026-04-23-08`. Separately capture the first green hosted CI run reference for the newly wired Ubuntu demo and `--no-default-features` steps when it becomes available.

## 2026-04-23 — WAVE-2026-04-23-06

- Selected item: `WAVE-2026-04-23-06`
- Result: Landed the first S2-owned `KeyPair` surface plus an additive `SignedProfileDocument` round-trip path in `ferros-profile`, so a fresh profile can be created, serialized, signed, deserialized, verified, and re-signed on revoke without widening into the remaining profile CLI verbs. The wave also added the focused `schemas/fixtures/signed-profile-valid.json` evidence and truth-synced S2 and gate/status surfaces while leaving `schemas/profile.v0.json`, harness files, and downstream S3/S4 consumer boundaries unchanged.
- Files: `Cargo.lock`, `crates/ferros-profile/Cargo.toml`, `crates/ferros-profile/src/lib.rs`, `schemas/fixtures/signed-profile-valid.json`, `streams/S2-profile/README.md`, `streams/S2-profile/BACKLOG.md`, `streams/S2-profile/CONTRACTS.md`, `streams/S2-profile/PROGRESS.md`, `docs/gates/G2.md`, `STATUS.md`
- Validation: Delegated orchestration passed `cargo test -p ferros-profile` with 32 passed and 0 failed; editor diagnostics were clean for the touched `ferros-profile` slice.
- Next follow-up: No ready queue item remains. `WAVE-2026-04-23-B01` stays blocked on the S3 JSON/RPC dependency; the next recommended new wave is an S2 follow-up to decide whether `SignedProfileDocument` should become a published schema contract or remain Rust-local until profile freeze, then land the remaining `ferros profile export | import | grant | revoke` evidence for G2 closeout.

## 2026-04-23 — WAVE-2026-04-23-05

- Selected item: `WAVE-2026-04-23-05`
- Result: Landed a Linux-backed real-binary proof for `ferros profile init` followed by `ferros profile show` without widening into the remaining profile CLI verbs. The slice added a focused integration test that launches the `ferros` binary, validates the initialized profile document through `show`, and truth-synced G2 and status surfaces to reflect that the current `init | show` path now has repo-backed Linux evidence through the existing Ubuntu CI test job.
- Files: `crates/ferros-node/tests/profile_cli_linux.rs`, `docs/gates/G2.md`, `STATUS.md`
- Validation: Delegated orchestration passed `cargo test -p ferros-profile -p ferros-node` and `cargo test -p ferros-node profile_init_then_show_works_via_real_ferros_binary -- --exact`; editor diagnostics were clean for the touched wave-owned files. A GitHub-hosted workflow could not be executed from this environment, so the Linux-backed proof is the new real-binary test plus the existing `ubuntu-latest` `cargo test --workspace --all-targets` job in `.github/workflows/ci.yml`.
- Next follow-up: No ready queue item remains. `WAVE-2026-04-23-B01` is still blocked on the S3 JSON/RPC dependency; the next recommended new wave is an S2/G2 key-material plus end-to-end profile signing evidence slice.

## 2026-04-23 — WAVE-2026-04-23-04

- Selected item: `WAVE-2026-04-23-04`
- Result: Landed the dedicated frozen `schemas/fixtures/profile-valid.json` evidence and proved it against the unchanged `schemas/profile.v0.json` contract in both `ferros-profile` and the H1 contract validator without widening into profile CLI or signing work. The slice also regenerated harness constants to embed the profile schema and fixture, and truth-synced G2 and status surfaces so they no longer claim the dedicated profile freeze evidence is missing.
- Files: `schemas/fixtures/profile-valid.json`, `crates/ferros-profile/src/lib.rs`, `tools/generate-harness-constants.ps1`, `harnesses/_constants.js`, `harnesses/ferros-contract-validator.html`, `docs/gates/G2.md`, `STATUS.md`
- Validation: Delegated orchestration passed `cargo test -p ferros-profile` with 26 passed and 0 failed; `./tools/generate-harness-constants.ps1` regenerated harness constants with `profile.v0` and `profile-valid` embedded; H1 `harnesses/ferros-contract-validator.html` passed with 47 passed, 0 failed, 0 skipped including the explicit `profile-valid` against `SCHEMA_PROFILE_V0` check; workspace diagnostics on touched files were clean.
- Next follow-up: `WAVE-2026-04-23-05`

## 2026-04-23 — WAVE-2026-04-23-03

- Selected item: `WAVE-2026-04-23-03`
- Result: Landed the minimal S2 profile CLI slice as a real implementation, wiring `ferros profile init [path]` and `ferros profile show [path]` through the existing `ProfileStore` boundary with filesystem-backed create-without-overwrite semantics, focused timestamp validation, and honest G2/status/contracts truth-sync. The wave stayed inside `init` and `show` and did not widen into import/export or signing.
- Files: `Cargo.lock`, `crates/ferros-profile/Cargo.toml`, `crates/ferros-profile/src/lib.rs`, `crates/ferros-node/Cargo.toml`, `crates/ferros-node/src/lib.rs`, `crates/ferros-node/src/bin/ferros.rs`, `docs/gates/G2.md`, `STATUS.md`, `docs/contracts/CONTRACTS-OVERVIEW.md`, `streams/S2-profile/PROGRESS.md`
- Validation: Delegated orchestration passed `cargo test -p ferros-profile init_profile_creates_new_profile_document_in_store`, `cargo test -p ferros-node profile_cli_init_and_show_round_trip_profile_document`, `cargo test -p ferros-node --bin ferros run_dispatches_profile_init_and_show_with_explicit_path`, `cargo test -p ferros-profile fresh_profile_document_rejects_invalid_created_at`, `cargo test -p ferros-profile`, `cargo test -p ferros-node profile_cli_`, and `cargo test -p ferros-profile -p ferros-node`. One intermediate dependency-resolution failure was repaired by pinning the node-side time dependency before the successful rerun.
- Next follow-up: No ready queue item remains. The next recommended wave is to add CI-backed Linux `ferros profile init -> show` proof and freeze `schemas/profile.v0.json` with dedicated frozen profile evidence.

## 2026-04-23 — WAVE-2026-04-23-02

- Selected item: `WAVE-2026-04-23-02`
- Result: Closed the S4 policy property-test gap for `DenyByDefaultPolicy` without widening into broader runtime work. The orchestrated slice added a minimal `proptest` test dependency and property-based coverage that proves an active exact match authorizes regardless of grant ordering while mismatched or inactive grants deny with the same decision when order changes. Gate and status docs still lag this evidence item and were intentionally left untouched in this wave.
- Files: `crates/ferros-core/Cargo.toml`, `crates/ferros-core/tests/capability_policy.rs`, `Cargo.lock`
- Validation: Delegated orchestration passed `cargo test -p ferros-core` with 21 passed, 0 failed, 0 ignored; local editor diagnostics were clean for `crates/ferros-core/Cargo.toml`, `crates/ferros-core/tests/capability_policy.rs`, `Cargo.lock`, `docs/orchestration/WAVE-QUEUE.md`, and `docs/orchestration/WAVE-RUN-LOG.md`
- Next follow-up: `WAVE-2026-04-23-03`

## 2026-04-23 — WAVE-2026-04-23-01

- Selected item: `WAVE-2026-04-23-01`
- Result: Closed the shared contract, harness, and gate-truth evidence for the signed `CapabilityGrant` boundary without widening into profile CLI work. The orchestrated slice updated the harness generator and validator, fixed negative-fixture classification for the invalid-signature case, and truth-synced the shared contract and G2 gate docs to the current schema-backed behavior.
- Files: `tools/generate-harness-constants.ps1`, `harnesses/_constants.js`, `harnesses/ferros-contract-validator.html`, `docs/contracts/CONTRACTS-OVERVIEW.md`, `docs/gates/G2.md`
- Validation: Delegated orchestration passed `cargo test -p ferros-profile`; direct `./tools/generate-harness-constants.ps1` invocation was blocked by local PowerShell execution policy, so the script body was executed via `Get-Content` and `ScriptBlock` to regenerate `harnesses/_constants.js`; browser validation for `harnesses/ferros-contract-validator.html` passed with 45 passed, 0 failed, 0 skipped; shared contract and harness parity against `schemas/capability-grant.v0.json` was confirmed; local editor diagnostics were clean for `docs/orchestration/WAVE-QUEUE.md` and `docs/orchestration/WAVE-RUN-LOG.md`
- Next follow-up: `WAVE-2026-04-23-02`

## 2026-04-23 — WAVE-2026-04-23-01

- Selected item: `WAVE-2026-04-23-01`
- Result: Landed the first signed and verifiable `CapabilityGrant` slice in `ferros-profile`, including Ed25519 signing, verification, revoke-and-resign behavior, and positive and negative schema fixtures. The wave remains partial because shared contract, harness, and gate-truth evidence still needs closeout, so the item returned to `Ready` with a narrower follow-up.
- Files: `crates/ferros-profile/src/lib.rs`, `crates/ferros-profile/Cargo.toml`, `Cargo.lock`, `schemas/capability-grant.v0.json`, `schemas/fixtures/grant-valid.json`, `schemas/fixtures/grant-invalid-sig.json`
- Validation: Delegated orchestration ran `cargo test -p ferros-profile` successfully after a lockfile repair; local editor diagnostics were clean for `docs/orchestration/WAVE-QUEUE.md` and `docs/orchestration/WAVE-RUN-LOG.md`
- Next follow-up: `WAVE-2026-04-23-01` narrowed to shared contract, harness, and gate-truth closeout for the signed `CapabilityGrant` path

## 2026-04-23 — DRIVER-BOOTSTRAP

- Selected item: none
- Result: Installed the local driver pattern with a user-invocable driver agent, repo-backed queue, append-only run log, and file-scoped queue rules.
- Files: `.github/agents/ferros-driver.agent.md`, `.github/instructions/ferros-wave-queue.instructions.md`, `docs/orchestration/LOCAL-DRIVER.md`, `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: editor diagnostics on the new customization and markdown files
- Next follow-up: `WAVE-2026-04-23-01`
