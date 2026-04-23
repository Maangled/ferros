# FERROS — Status Dashboard

> Dashboard, not a diary. Each section shows current state. Details live in stream PROGRESS.md files.
>
> Last updated: 2026-04-23

---

## Overall

| Item | State |
|------|-------|
| Active gate | **G2** — Profile v0 Frozen |
| Launch gate | G4 (open) |
| MVP gate | G1 → G2 → G3 in sequence |
| Open streams | S1 (closeout), S2 (active), S3 (convergence), S4 (convergence), S5 (Phase A), S6 (active), S8 (background) |

---

## Gate status

| Gate | Status | Condition |
|------|--------|-----------|
| G1 | ✅ Closed | CI run #24812246339 proved fmt, clippy, build, and test green on ubuntu-latest, macos-latest, and windows-latest |
| G2 | 🟡 Active | Signed `CapabilityGrant` evidence landed; profile v0 freeze and CLI proof remain the critical path |
| G3 | ⬜ Blocked | G2 must close first; S3+S4 minimal agent-center-on-runtime demo |
| G4 | ⬜ Blocked | G3 must close first; `ferros-hub` on real hardware with HA integration |

---

## Stream status

| Stream | Status | Current focus | Gate |
|--------|--------|---------------|------|
| S1 Foundation | 🟡 Closeout / hygiene | Tag `v0.0.1-foundation`, verify branch protection, keep repo hygiene rolling | G1 |
| S2 Profile & Identity | 🟡 Active | signed `CapabilityGrant` envelope landed; remaining work is key material, profile v0 freeze, and CLI flows | G2 |
| S3 Agent Center | 🟡 Convergence active | reference agents, local `ferros agent ...` CLI, and `ferros-node demo` landed; JSON/RPC and post-G2 contract hardening remain | G3 |
| S4 Runtime / OS Core | 🟡 Convergence active | `ferros-runtime`, in-memory executor and bus, `ferros-node demo`, and the `ferros-core --no-default-features` compile slice landed; property tests and broader `no_std` hardening remain | G3 |
| S5 UX | 🟨 Phase A active; Phase B blocked on G3 | real landing page and honest status banner shipped; local agent-center web shell remains post-G3 work | post-G3 |
| S6 Ecosystem Harvest | 🟡 Active | ADR-018/019/020 landed; `ferros-data` is now a root workspace member while downstream extraction stays stream-owned | rolling |
| S7 Smart-Home Hub | ⬜ Blocked on G2/G3 | pairing needs stable profile types; implementation needs runtime | G4 |
| S8 Docs / Governance | 🟡 Active (background) | status/gate/contracts truth-sync baseline is in repo; governance skeleton remains partial (`SECURITY.md` yes, `THREAT-MODEL.md`/`GOVERNANCE.md`/`CODE_OF_CONDUCT.md` not yet landed) | rolling |

---

## Milestone tags

| Tag | Status | Condition |
|-----|--------|-----------|
| `v0.0.1-foundation` | 🟡 | G1 closed; tag pending |
| `v0.0.2-profile` | ⬜ | S2 profile v0 frozen |
| `v0.0.3-runtime` | 🟡 | `ferros-runtime`, `ferros-node demo`, and the `ferros-core --no-default-features` compile slice landed; property tests and broader `no_std` hardening remain |
| `v0.0.4-agents` | 🟡 | reference agents, local CLI, and demo path landed; JSON/RPC and post-G2 contract hardening remain |
| `v0.0.5-harvest` | 🟡 | harvest ADRs landed; downstream extraction continues |
| `v0.1.0-rc` | ⬜ | MVP: S1+S2+S3+S4 functional |
| `v0.1.0` | ⬜ | Agent center local web shell (S5 Phase B) |
| `v0.2.0-rc` | ⬜ | `ferros-hub` pairing demo on x86_64 |
| `v0.2.0` | ⬜ | **Launch** — hub on Pi with HA, consent enforced, reboot-safe |

---

## Critical path

```
S1 Foundation → G1 → S2 Profile → G2 → S3 Agent Center → G3 → S7 Hub → G4 → Launch
                  └→ S4 Runtime ─────────────────────────┘
```

The **profile → agent center** path is the gating path. Everything else is parallel.

---

## Recent activity

| Date | Event |
|------|-------|
| 2026-04-23 | S2 landed the first signed and verifiable `CapabilityGrant` path in `ferros-profile`: the stripped JSON payload contract is now explicitly frozen in `schemas/capability-grant.v0.json`, `grant-valid.json` and `grant-invalid-sig.json` are in repo, and `cargo test -p ferros-profile` covers verify plus revoke without claiming G2 closed. |
| 2026-04-23 | S3 and S4 converged on the first runnable demo path: `ferros-node demo` now registers `echo` and `timer`, echoes a message, emits a timer tick, proves deny-by-default with the current real `CapabilityGrant` type, and ships a local `ferros agent list | describe | run | stop | logs` CLI validated by `cargo test -p ferros-node`. |
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
| `ferros-profile` still needs key material, full profile-level round-trip evidence, `schemas/profile.v0.json` freeze, and CLI `init | show | export | import | grant | revoke` for G2; the signed `CapabilityGrant` verify path and grant fixtures are now landed | S2, S3, S7 | S2 |
| `ferros-agents` and the local `ferros` CLI still need a stable post-G2 grant contract, JSON/RPC surface, and broader harness/CI hardening | S3, S5 | S3 |
| `ferros-runtime` still needs property tests, target-level `no_std` hardening beyond the current `--no-default-features` compile slice, and host-path hardening beyond the in-memory demo | S4, S3, S7 | S4 |
| S5 Phase A is live on the landing page, but the local web shell remains blocked behind G3 and the S3 JSON/RPC surface | S5 | S5 |
