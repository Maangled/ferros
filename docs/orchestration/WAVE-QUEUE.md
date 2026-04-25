# FERROS Wave Queue

This queue feeds the local driver pattern. Process one wave per invocation unless the user explicitly requests a batch.

## Ready

### WAVE-2026-04-24-16

- Title: Publish a narrow hub-facing restart/reload boundary for S7 runway
- Status: ready
- Priority: P1
- Gate: G4 runway
- Owning streams: S4 primary, S7 consumer awareness, S8 truth-sync if queue or stream docs move
- Goal: Turn the currently named reload helpers into an explicit S4-owned docs-only boundary that states what restart-safe state, reload, and re-registration guarantees S7 may rely on now versus what remains unpublished before any authoritative pairing flow, `ferros-hub` scaffold, or Home Assistant bridge plan is honest.
- Anchor files: `streams/S4-runtime/CONTRACTS.md`, `streams/S4-runtime/BACKLOG.md`, `streams/S7-hub/BACKLOG.md`
- Validation: editor diagnostics on touched S4 and S7 docs; verify the published boundary stays consistent with the landed S7 seam brief in `streams/S7-hub/README.md` and `streams/S7-hub/CONTRACTS.md`, plus `STATUS.md` and `docs/gates/G4.md`
- Constraints: Keep the slice docs-only and S4-owned. Do not change runtime code, do not scaffold `crates/ferros-hub/`, do not define pairing, reboot, or re-registration choreography beyond the narrow published boundary, do not invent new policy semantics, and do not claim G4 evidence.
- Last update: 2026-04-24

## In Progress

None.

## Blocked

None.

## Done

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
