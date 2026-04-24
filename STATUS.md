# FERROS — Status Dashboard

> Dashboard, not a diary. Each section shows current state. Details live in stream PROGRESS.md files.
>
> Last updated: 2026-04-24

---

## Overall

| Item | State |
|------|-------|
| Active gate | **G4** — Launch |
| Launch gate | G4 (open) |
| MVP gate | G1 → G2 → G3 in sequence |
| Open streams | S1 (closeout), S3 (post-G3 contract), S4 (post-G3 hardening), S5 (Phase A plus Phase B localhost shell), S6 (active), S7 (G4 runway), S8 (background) |

---

## Gate status

| Gate | Status | Condition |
|------|--------|-----------|
| G1 | ✅ Closed | CI run #24812246339 proved fmt, clippy, build, and test green on ubuntu-latest, macos-latest, and windows-latest |
| G2 | ✅ Closed | `profile.v0.json` remains the frozen unsigned published v0 consumer contract, `SignedProfileDocument` stays Rust-local at v0, and the real `ferros` binary now proves `profile init | grant | export | import | revoke | show` against temp-file-backed local state while `show` stays unsigned and revoked grant state stays within the frozen grant boundary |
| G3 | ✅ Closed | CI #20 (`run 24902870499`, commit `8383b67` on `main`) completed successfully on 2026-04-24 after `.github/workflows/ci.yml` wired `cargo check -p ferros-core --no-default-features` plus `cargo run --bin ferros -- demo` into the hosted Ubuntu workflow |
| G4 | 🟡 Active | G3 is closed; S7 now owns the active launch gate for `ferros-hub` on real hardware with Home Assistant integration |

---

## Stream status

| Stream | Status | Current focus | Gate |
|--------|--------|---------------|------|
| S1 Foundation | 🟡 Closeout / hygiene | Tag `v0.0.1-foundation`, verify branch protection, keep repo hygiene rolling | G1 |
| S2 Profile & Identity | ✅ G2 closed / handoff | the frozen unsigned `profile.v0.json` consumer contract, the Rust-local `SignedProfileDocument` v0 boundary, and the real-binary `init | grant | export | import | revoke | show` lifecycle proof are landed; immediate work is to hold that boundary steady for downstream consumers | G2 |
| S3 Agent Center | 🟡 Post-G3 read contract landed | reference agents, local `ferros agent ...` CLI, the `cargo run --bin ferros -- demo` path, and the first read-first JSON/RPC contract for `agent.list`, `agent.describe`, `grant.list`, and `denyLog.list` are now in repo; broader harness coverage, transport serving, and privileged writes remain | post-G3 |
| S4 Runtime / OS Core | 🟡 Post-G3 hardening | `ferros-runtime`, in-memory executor and bus, policy property tests, the `cargo run --bin ferros -- demo` path, and the `ferros-core --no-default-features` compile slice landed; the current node host now exposes the first read-only S3 contract over the same runtime state, while broader `no_std` and host hardening remain | post-G3 |
| S5 UX | 🟨 Phase A active; Phase B localhost read slice landed | real landing page and honest status banner shipped; prototype-authority cleanup and archive mapping are underway, and the first fixed-slot localhost shell now reads live agent, grant-state, and deny-log data through `ferros-node` | post-G3 |
| S6 Ecosystem Harvest | 🟡 Active | ADR-018/019/020 landed; `ferros-data` is now a root workspace member while downstream extraction stays stream-owned | rolling |
| S7 Smart-Home Hub | 🟡 G4 runway active | hardware runway and reference-hardware prep are now explicit, while pairing semantics remain provisional until a real `ferros-hub` binary, HA bridge, and physical-device evidence begin landing | G4 |
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
| `v0.1.0` | 🟡 | First localhost shell slice is landed via `ferros-node shell`; broader UI acceptance, privileged write actions, and remaining Phase A cleanup remain |
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
| 2026-04-24 | S5 landed the first localhost shell slice: `site/agent-center-shell.html` now renders a fixed-slot agent-center shell, `crates/ferros-node/src/lib.rs` serves it at `GET /` with `POST /rpc` backed by the read-first S3 JSON/RPC contract, and live browser validation against `ferros-node shell 4317` proved real agent, grant-state, and deny-log reads without widening into privileged writes. |
| 2026-04-24 | S3 landed the first read-first JSON/RPC contract in `crates/ferros-agents/src/rpc.rs` plus a local host handler in `crates/ferros-node/src/lib.rs`, covering `agent.list`, `agent.describe`, `grant.list`, and `denyLog.list` with focused `cargo test -p ferros-agents -p ferros-node` validation and without widening into HTTP serving or privileged write actions. |
| 2026-04-24 | G3 closed: CI #20 (`run 24902870499`, commit `8383b67` on `main`) completed successfully after the hosted Ubuntu workflow began running both `cargo check -p ferros-core --no-default-features` and `cargo run --bin ferros -- demo`; G4 is now the active gate. |
| 2026-04-24 | S2 closed G2 with the final real-binary profile CLI lifecycle proof: `ferros profile init | grant | export | import | revoke | show` now runs against temp-file-backed local state, `show` stays within the unsigned `profile.v0.json` boundary, revoked persisted grant state stays inside the frozen `capability-grant.v0.json` envelope shape, and G3 is now the active gate. |
| 2026-04-24 | S2, S4, S5, S6, and S7 all advanced in a non-overlapping subagent batch: S2 froze `profile.v0.json` as the unsigned published v0 consumer contract, kept `SignedProfileDocument` Rust-local at v0, and tightened signed-profile fixture parity so the embedded profile revalidates against that frozen boundary; S4 improved the shared `MessageEnvelope` portability proof with `--no-default-features` and `thumbv7em-none-eabi` validation, S6 added an exactly-one-parent ordered-child migration guard, S5 added prototype-authority banners plus an archive map, and S7 expanded hardware runway documentation without claiming pairing semantics are frozen. |
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
| `v0.0.1-foundation` tag is not yet created; required status checks on `main` are not yet verified in branch protection | S1 | S1 |
| `ferros-agents` and the local `ferros` host path now have the first stable read-first JSON/RPC surface, but still need broader harness coverage, transport serving, and privileged write actions before the shell is feature-complete | S3, S5 | S3 |
| `ferros-runtime` closed G3, but still needs target-level `no_std` hardening beyond the current `--no-default-features` compile slice and host-path hardening beyond the in-memory demo | S4, S3, S7 | S4 |
| S5 now has a real localhost shell served by `ferros-node`, but broader browser acceptance, privileged grant/revoke actions, and the remaining Phase A archive/link-hygiene pack are still open | S5 | S5 |
| S7 runway is no longer blocked on G3, but a real `ferros-hub` binary, HA bridge, and physical-device evidence are still absent | S7 | S7 |
