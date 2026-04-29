# FERROS — Status Dashboard

> Dashboard, not a diary. Each section shows current state. Details live in stream PROGRESS.md files.
>
> Last updated: 2026-04-28 (WAVE-2026-04-28-45 truth-sync)

---

## Overall

| Item | State |
|------|-------|
| Active gate | **G4** — Launch |
| Demo gate | **D1** — active runway (see `docs/gates/D1.md`) |
| Launch gate | G4 (open) |
| MVP gate | G1 → G2 → G3 in sequence |
| Open streams | S1 (closeout), S3 (post-G3 contract), S4 (post-G3 hardening), S5 (localhost shell closeout and onramp follow-up), S6 (active), S7 (G4 runway), S8 (background) |

---

## Gate status

| Gate | Status | Condition |
|------|--------|-----------|
| G1 | ✅ Closed | CI run #24812246339 proved fmt, clippy, build, and test green on ubuntu-latest, macos-latest, and windows-latest |
| G2 | ✅ Closed | `profile.v0.json` remains the frozen unsigned published v0 consumer contract, `SignedProfileDocument` stays Rust-local at v0, and the real `ferros` binary now proves `profile init | grant | export | import | revoke | show` against temp-file-backed local state while `show` stays unsigned and revoked grant state stays within the frozen grant boundary |
| G3 | ✅ Closed | CI #20 (`run 24902870499`, commit `8383b67` on `main`) completed successfully on 2026-04-24 after `.github/workflows/ci.yml` wired `cargo check -p ferros-core --no-default-features` plus `cargo run --bin ferros -- demo` into the hosted Ubuntu workflow |
| D1 | 🟡 Active runway | Demo gate defined in `docs/gates/D1.md`; one device, operator-attended, profile+HA+consent+reboot-safe; not yet closed |
| G4 | 🟡 Active | G3 is closed; S7 now owns the active launch gate for `ferros-hub` on real hardware with Home Assistant integration |

---

## Stream status

| Stream | Status | Current focus | Gate |
|--------|--------|---------------|------|
| S1 Foundation | 🟡 Closeout / hygiene | a manual release-candidate bundle workflow is now landed alongside CI and integration concurrency guards; tag `v0.0.1-foundation` and branch-protection verification still remain | G1 |
| S2 Profile & Identity | ✅ G2 closed / handoff | the frozen unsigned `profile.v0.json` consumer contract, the Rust-local `SignedProfileDocument` v0 boundary, the real-binary `init | grant | export | import | revoke | show` lifecycle proof, rollback-safe local bundle import on invalid grant state, and an additive local consent snapshot readiness surface for downstream local consumers are landed; immediate work is to hold that boundary steady without reopening the frozen schemas | G2 |
| S3 Agent Center | 🟡 Post-G3 localhost contract hardening | reference agents, local `ferros agent ...` CLI, the `cargo run --bin ferros -- demo` path, the read-first JSON/RPC contract for `agent.list`, `agent.describe`, `agent.snapshot`, `grant.list`, and `denyLog.list`, the first code-backed local-only lifecycle/read-after-write seam, the first broader local-only `LocalAgentApi` wrapper/API slice above that seam, and the first local-only `agent.run` / `agent.stop` JSON-RPC slice on the current localhost shell host are now in repo; denied local writes keep the same local summaries while broader transport and privileged writes remain open | post-G3 |
| S4 Runtime / OS Core | 🟡 Post-G3 hardening | `ferros-runtime`, in-memory executor and bus, policy property tests, the `cargo run --bin ferros -- demo` path, the `ferros-core --no-default-features` compile slice, a local `thumbv7em-none-eabi --no-default-features` proof, the local-only `LocalRunwayState` checkpoint helper surface for downstream runway-summary consumers, and the narrow host/controller support for both the local-only `LocalAgentApi` seam and the first local-only lifecycle/write JSON-RPC slice are now in repo; CI is configured to enforce the same thumb-target check while broader `no_std` and host hardening remain | post-G3 |
| S5 UX | 🟨 Phase A archive/link-hygiene landed; Phase B localhost shell, lifecycle bar, and profile slice landed | real landing page and honest status banner shipped; the Phase A archive/link-hygiene pass and docs-root reference repairs are landed, the fixed-slot localhost shell now reads live agent, grant-state, and deny-log data through `ferros-node`, exposes a narrow grant-aware selected-agent lifecycle bar, and renders the first localhost `/profile` slice for `init | show | export | import`, while the same-origin acceptance harness closes the local lifecycle, deny, and profile proof loop on one embedded path; grant/revoke, remote transport, onramp wiring, and broader browser control remain open | post-G3 |
| S6 Ecosystem Harvest | 🟡 Active | ADR-018/019/020 landed; `ferros-data` now owns the typed local-push audit envelope boundary, the schema is admitted into harness constants and contract validation, and `cargo xtask burst` emits `.tmp/push/burst-local-push-envelope.json` as local-only non-partner-facing output while downstream extraction stays stream-owned | rolling |
| S7 Smart-Home Hub | 🟡 G4 runway active | the hardware runway, `x86_64`-first bring-up plan, first Home Assistant bridge runway contract, first Pack B bring-up worksheet, operator rehearsal notes, a library-backed local-only `ferros-hub` crate, manifest-backed local bridge registration, `ferros-core` policy-backed allow/deny evaluation, a typed local runtime summary, thin `summary | prove-bridge | deny-demo` CLI proofs, bounded local artifact/report schemas with H1 validator coverage, and `cargo xtask hub-runway` are now in repo; pairing semantics stay provisional while real Home Assistant proof, physical-device evidence, remote transport, and durable hub runtime evidence remain open | G4 |
| S8 Docs / Governance | 🟡 Active (background) | status/gate/contracts truth-sync baseline is in repo; doctrine plus the ADR index/roadmap/research baseline are now landed; `SECURITY.md`, `THREAT-MODEL.md`, `GOVERNANCE.md`, `CODE_OF_CONDUCT.md`, and contributor intake templates now exist, while issue seeding remains open | rolling |

---

## Milestone tags

| Tag | Status | Condition |
|-----|--------|-----------|
| `v0.0.1-foundation` | 🟡 | G1 closed; tag pending |
| `v0.0.2-profile` | 🟡 | G2 closed; tag pending |
| `v0.0.3-runtime` | 🟡 | G3 is closed; tag pending while broader `no_std` and host hardening remain |
| `v0.0.4-agents` | 🟡 | G3 is closed and the first read-first JSON/RPC contract is landed; tag pending while broader harness hardening, transport serving, and privileged writes remain |
| `v0.0.5-harvest` | 🟡 | harvest ADRs landed; downstream extraction continues |
| `v0.1.0-rc` | 🟡 | MVP gate path G1 → G2 → G3 is now closed; tag pending |
| `v0.1.0` | 🟡 | First localhost shell slice plus the narrow local lifecycle/profile proof loop are landed via `ferros-node shell`; privileged grant/revoke, onramp consent wiring, remote transport, and broader browser control remain |
| `v0.2.0-rc` | ⬜ | `ferros-hub` pairing demo on x86_64 |
| `v0.2.0` | ⬜ | **Launch** — hub on Pi with HA, consent enforced, reboot-safe |

---

## Critical path

```
S1 Foundation → G1 → S2 Profile → G2 → S3 Agent Center → G3 → S7 Hub → G4 → Launch
                  └→ S4 Runtime ─────────────────────────┘
```

The **agent center + runtime convergence** path is now closed at G3. The active gate is G4, while the S3 remote contract, S5 local shell, and additional S4 hardening proceed in parallel behind that launch-facing gate.

---

## Recent activity

| Date | Event |
|------|-------|
| 2026-04-28 | WAVE-2026-04-28-38 through WAVE-2026-04-28-44 landed in sequence: `ferros-hub` is now library-backed, the local bridge registers through `ferros-agents` primitives, authorization composes through `ferros-core` policy over real `CapabilityGrant` inputs, a typed local runtime summary and thin `summary | prove-bridge | deny-demo` CLI proofs are landed, bounded local artifact/report schemas are admitted into the H1 validator, and `cargo xtask hub-runway` now confirms the same `.tmp/hub/simulated-local-bridge-artifact.json` proof path. No gate moved. No HA, hardware, or remote-transport claim. |
| 2026-04-28 | WAVE-2026-04-28-34 through WAVE-2026-04-28-36 landed in sequence: Batch Mode and the Local Driver now default this size-L posture to subagent review, `ferros-hub` is a real local workspace member and binary scaffold, the first crate-local bridge seam is in repo, and `cargo test -p ferros-hub bridge_` now proves simulated allow/deny/error behavior while emitting `.tmp/hub/simulated-local-bridge-artifact.json` with only local-only, non-evidentiary fields. No gate moved. No HA or hardware claim. |
| 2026-04-28 | Owner-wave drain and final truth-sync landed: S4 published local-only runway checkpoint helpers for downstream runway summaries, S2 added additive local consent snapshot readiness for downstream local consumers, S5 closed the localhost lifecycle/deny/profile proof loop on one same-origin harness path and codified the landed `/profile` slice as a narrow closeout surface, and S6 admitted the local-push schema into harness consumers while `cargo xtask burst` now emits `.tmp/push/burst-local-push-envelope.json`. No gate moved. No hardware or HA claim. |
| 2026-04-27 | BATCH-2026-04-27-D (code track, 8 waves) landed: first batch to plan ≥6 parallel-safe-with waves and first full use of the width-8 editing-lane ceiling. All 8 waves are docs-only D1 bring-up runway and comms readiness preparation. New directories: `docs/research/` (6 research notes covering S7 D1 checklist, S4 no_std target matrix, S5 consent flow UX, S2 profile round-trip scripting, S4 policy engine invariants, S3 agent manifest catalog) and `docs/explainers/` (gate narrative explainer for non-technical audience). Additive preamble note added to `docs/adr/_ROADMAP.md` recording post-BATCH-C ADR state (ADR-018 through ADR-024; ADR-024 remains Proposed — not promoted). No crate, schema, harness, or CI workflow files modified. No D1 evidence claimed. Gatekeeper verdict: clean pass (8/8 waves, no overrun, no escalation, revert clause not triggered). |
| 2026-04-27 | Ceiling-lift wave (Interactive, S8) raised the editing-lane ceiling from 5 to 8 in `LOCAL-DRIVER.md`, citing BATCH-2026-04-27 and BATCH-2026-04-27-B (two consecutive conditional-pass Batch Mode runs) as evidence. Revert clause added: if any subsequent batch fails (Triage/Trace escalation, frozen-surface touch, or halt before final wave), ceiling reverts to 5 in the next substrate-refinement wave. `BATCH-MODE.md` planning-target sentence updated to reflect 8-wave planning target and 8-lane ceiling now matching. Batch-C (code track, 3 waves) followed: WAVE-2026-04-27-06 defined the S5 onramp consent surface entry bar under ADR-023 (minimum honest UX slot for proposed onramp items, quarantine-until-accepted invariant, channel-agnostic); WAVE-2026-04-27-07 added the S7 HA bridge consent-mapping note (HA entities arrive as proposed material, must route through S5 onramp surface, no bridge protocol constraints added); WAVE-2026-04-27-08 derived the S5 consent-flow copy spec from `docs/legal/CONSENT-LANGUAGE.md` DRAFT (capability grant, onramp accept, deny-visibility copy slots, marked draft pending counsel red-line with coordinated-clearing requirement). ADR-023 now carries S5 and S7 consumer-awareness notes. Gatekeeper verdict: clean pass (3/3 waves, no named ambiguities, non-trivial decisions, no overrun). |
| 2026-04-27 | Substrate-refinement wave and system-track Batch Mode run (BATCH-2026-04-27-B) landed: `BATCH-MODE.md` now carries the ratified operational-bookkeeping exemption list (including owner-stream PROGRESS.md), the structured gatekeeper block schema, and the batch-level verdict criteria section; `LOCAL-DRIVER.md` carries the gatekeeper model-swap intent note. System-track batch (3 waves): ADR-023 (onramp policy — external systems are onramps, not identity truth; Status: Accepted) and ADR-024 (ledger/chain substrate comparison; Status: Proposed; recommendation: non-chain signed ledger for v0.1.0–v0.2.0) are now in repo; `docs/legal/` is scaffolded with three counsel-ready draft files (TERMS-OF-USE.md, LICENSING-POSTURE.md, CONSENT-LANGUAGE.md). Gatekeeper verdict: conditional pass pending human doc-batch review of ADR-024 Proposed status. |
| 2026-04-27 | First Batch Mode proof run (code track) landed clean: WAVE-2026-04-27-02 defined the minimum consent-gated browser-issued lifecycle control bar above the staged shell-intent copy (S5/S3/S4 docs); WAVE-2026-04-27-04 defined the minimum honest first browser profile surface entry bar above the frozen S2 contract (S5/S2 docs); WAVE-2026-04-27-05 defined the operator-facing evidence surface for hub bring-up and status (S7 docs, sourced from Pack B worksheet). All three waves landed docs-only; gatekeeper verdict is conditional pass with one substrate ambiguity (overrun-narrowing policy) queued for ratification. |
| 2026-04-27 | S8 landed WAVE-2026-04-27-03: upgraded the orchestration substrate with Batch Mode (`docs/orchestration/BATCH-MODE.md`), three-track queues (system + hardware queues added), D1 demo gate (`docs/gates/D1.md`), eight per-stream FILLER.md files, code-track queue backfilled to depth 3 (two new Ready waves added), and STATUS.md updated to reflect the 2026-04-27 staged-intent landing and the new D1 gate row. |
| 2026-04-27 | S5 staged selected-agent lifecycle intent copy and read-only slot affordances on the live localhost shell (WAVE-2026-04-27-01): the focus, tools, and consent/audit slots now show the selected agent's state and the appropriate `agent.run` / `agent.stop` intent copy while the shell remains observation-only. |
| 2026-04-26 | S3/S4 landed the first local-only lifecycle/write JSON-RPC slice on the current localhost shell host: `crates/ferros-node/src/lib.rs` now routes `agent.run` and `agent.stop` through the landed `LocalAgentApi` seam, keeps `agent.describe`, `agent.snapshot`, and `denyLog.list` as the read-after-write observation path, and leaves browser control plus broader remote/write claims unpublished. |
| 2026-04-26 | An overnight filler batch landed around the hot S3/S4 seam: `crates/ferros-node/src/lib.rs` now preserves typed missing-capability detail on denied local `LocalAgentApi` runs while CLI/log summaries stay stable, S1 added a manual release-candidate bundle workflow plus workflow concurrency guards, S2 made local bundle import rollback-safe on invalid grant state, S6 published ordered ferros-data migration manifest coverage, and S7 added operator rehearsal prep to the current Pack B runway docs. |
| 2026-04-26 | S3/S4 landed the first broader local-only wrapper/API slice above the current CLI/state path: `crates/ferros-node/src/lib.rs` now publishes `LocalAgentApi` as a typed local `list | describe | run | stop | logs` surface above CLI formatting while still reusing the same local state path, deny-by-default behavior, read-first JSON/RPC observation, and localhost shell host. |
| 2026-04-26 | A five-lane owner batch advanced in parallel: S2 truth-synced its backlog to the already-closed G2 boundary, S3 fixed the first broader lifecycle/write wrapper/API entry bar above the landed local-only seam, S5 extended the same-origin localhost harness to prove operator-assisted local run/stop observation through one `agent.snapshot` refresh, S7 prepared the first Pack B bring-up worksheet from the existing hardware runway map, and S4 reran the local `thumbv7em-none-eabi --no-default-features` proof green while CI remains configured to enforce that same check. |
| 2026-04-24 | S5 landed the first localhost shell slice: `site/agent-center-shell.html` now renders a fixed-slot agent-center shell, `crates/ferros-node/src/lib.rs` serves it at `GET /` with `POST /rpc` backed by the read-first S3 JSON/RPC contract, and live browser validation against `ferros-node shell 4317` proved real agent, grant-state, and deny-log reads without widening into privileged writes. |
| 2026-04-24 | S3 landed the first read-first JSON/RPC contract in `crates/ferros-agents/src/rpc.rs` plus a local host handler in `crates/ferros-node/src/lib.rs`, covering `agent.list`, `agent.describe`, `grant.list`, and `denyLog.list` with focused `cargo test -p ferros-agents -p ferros-node` validation and without widening into HTTP serving or privileged write actions. |
| 2026-04-24 | G3 closed: CI #20 (`run 24902870499`, commit `8383b67` on `main`) completed successfully after the hosted Ubuntu workflow began running both `cargo check -p ferros-core --no-default-features` and `cargo run --bin ferros -- demo`; G4 is now the active gate. |
| 2026-04-24 | S2 closed G2 with the final real-binary profile CLI lifecycle proof: `ferros profile init | grant | export | import | revoke | show` now runs against temp-file-backed local state, `show` stays within the unsigned `profile.v0.json` boundary, revoked persisted grant state stays inside the frozen `capability-grant.v0.json` envelope shape, and G3 is now the active gate. |
| 2026-04-24 | S2, S4, S5, S6, and S7 all advanced in a non-overlapping subagent batch: S2 froze `profile.v0.json` as the unsigned published v0 consumer contract, kept `SignedProfileDocument` Rust-local at v0, and tightened signed-profile fixture parity so the embedded profile revalidates against that frozen boundary; S4 improved the shared `MessageEnvelope` portability proof with `--no-default-features` and `thumbv7em-none-eabi` validation, S6 added an exactly-one-parent ordered-child migration guard, S5 finished the Phase A archive/link-hygiene pass with prototype-authority banners, archive mapping, and repaired docs-root references, and S7 defined the first Home Assistant bridge runway contract without claiming pairing semantics are frozen. |
| 2026-04-24 | S8 truth-sync corrected the contributor intake state: `.github/ISSUE_TEMPLATE/stream-task.md` and `.github/PULL_REQUEST_TEMPLATE.md` were already present and are now treated as the current intake baseline rather than open work. |
| 2026-04-24 | S8 landed the missing governance skeleton files: `THREAT-MODEL.md`, `GOVERNANCE.md`, and `CODE_OF_CONDUCT.md`. The threat model is intentionally partial and keyed to the current G2/G3/G4 posture rather than claiming production hardening. |
| 2026-04-24 | S8 landed the ADR context-lock baseline: `DOCTRINE.md`, ADR-022, the ADR index and roadmap, the research-note and evidence lanes, and the first ACC card/deck projection research note; `docs/ORCHESTRATION.md` was also downgraded to historical governance context rather than the active execution authority. |
| 2026-04-23 | G3 truth surfaces synced to repo evidence: `.github/workflows/ci.yml` now explicitly wires `cargo check -p ferros-core --no-default-features` and `cargo run --bin ferros -- demo` into CI, while G3 remains blocked on G2 and a recorded green run reference is still pending. |
| 2026-04-23 | S2 landed `KeyPair` plus an additive signed profile envelope in `ferros-profile`: Ed25519 key generation, a `KeyPair` path that derives a `ProfileId` from its verifying key, and create → serialize → sign → verify → revoke evidence now pass in `cargo test -p ferros-profile` without mutating `schemas/profile.v0.json`. |
| 2026-04-23 | S2 landed the first signed and verifiable `CapabilityGrant` path in `ferros-profile`: the stripped JSON payload contract is now explicitly frozen in `schemas/capability-grant.v0.json`, `grant-valid.json` and `grant-invalid-sig.json` are in repo, and `cargo test -p ferros-profile` covers verify plus revoke without claiming G2 closed. |
| 2026-04-23 | S3 and S4 converged on the first runnable demo path: `cargo run --bin ferros -- demo` now registers `echo` and `timer`, echoes a message, emits a timer tick, verifies the current deny-by-default path, and ships a local `ferros agent list | describe | run | stop | logs` CLI validated by `cargo test -p ferros-node`. |
| 2026-04-23 | S6 harvest ADRs landed: ADR-018 (`botgen-rust`), ADR-019 (`workpace-rust`), and ADR-020 (`sheetgen-rust`). |
| 2026-04-23 | S4 landed the first `ferros-core` capability/policy slice, published `ferros-runtime`, wired an in-memory host path through `ferros-node`, and now compiles `ferros-core` with `--no-default-features` without claiming full embedded readiness yet. |
| 2026-04-23 | S3 landed a pre-G3 `ferros-agents` scaffold, then extended it with a transport boundary and two reference agents inside the convergence demo path. |
| 2026-04-23 | S5 landed Phase A landing-page cleanup on `site/index.html`, including the honest repository-status banner; the local agent-center web shell remains Phase B work pending G3. |
| 2026-04-23 | S6 admitted `crates/ferros-data/` to the root workspace and validated the narrow slice with root-level Cargo commands. |
| 2026-04-23 | G1 closed: CI run #24812246339 proved fmt, clippy, build, and test green across ubuntu-latest, macos-latest, and windows-latest. |
| 2026-04-21 | Wave 0 closed (contracts C1–C10 verified). Stream docs scaffolded. Stream-first planning model adopted. |

---

## Known blockers

| Blocker | Affects | Owner |
|---------|---------|-------|
| `v0.0.1-foundation` tag is not yet created and required status checks on `main` are not yet verified in branch protection, even though the repo now has a manual release-candidate bundle workflow for local closeout hygiene | S1 | S1 |
| `ferros-agents` and the local `ferros` host path now have the first stable read-first JSON/RPC surface, a code-backed local-only lifecycle/read-after-write seam, the first broader local-only `LocalAgentApi` wrapper/API slice, typed local deny detail on denied runs, and a local-only `agent.run` / `agent.stop` JSON-RPC slice on the current localhost shell host, but still need consumer-side shell controls, transport serving beyond localhost, and broader privileged write actions before the shell is feature-complete | S3, S5 | S3 |
| `ferros-runtime` closed G3, but still needs target-level `no_std` hardening beyond the current local `--no-default-features` plus `thumbv7em-none-eabi` compile proof and host-path hardening beyond the in-memory demo | S4, S3, S7 | S4 |
| S5 now has a real localhost shell, a narrow selected-agent lifecycle control bar, the first localhost `/profile` slice, and a same-origin lifecycle/deny/profile proof loop, but focused `/profile` route-test closeout, onramp consent wiring, broader browser control, and privileged grant/revoke actions are still open | S5 | S5 |
| S7 runway is no longer blocked on G3, and the local `ferros-hub` library seam, manifest/policy-backed simulated bridge proof path, typed runtime summary, CLI proofs, schema/harness validation, xtask helper, and emitted local artifact are now in repo, but real Home Assistant proof, durable runtime behavior on target hardware, and physical-device evidence are still absent | S7 | S7 |
