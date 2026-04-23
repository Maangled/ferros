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
| Open streams | S1 (closeout), S2 (active), S4 (prep), S6 (active), S8 (background) |

---

## Gate status

| Gate | Status | Condition |
|------|--------|-----------|
| G1 | ✅ Closed | CI run #24812246339 proved fmt, clippy, build, and test green on ubuntu-latest, macos-latest, and windows-latest |
| G2 | 🟡 Active | G1 is closed; profile v0 implementation and freeze evidence are now the critical path |
| G3 | ⬜ Blocked | G2 must close first; S3+S4 minimal agent-center-on-runtime demo |
| G4 | ⬜ Blocked | G3 must close first; `ferros-hub` on real hardware with HA integration |

---

## Stream status

| Stream | Status | Current focus | Gate |
|--------|--------|---------------|------|
| S1 Foundation | 🟡 Closeout / hygiene | Tag `v0.0.1-foundation`, verify branch protection, keep repo hygiene rolling | G1 |
| S2 Profile & Identity | 🟡 Active | Ed25519, grants, schema freeze, CLI, and schema parity tests | G2 |
| S3 Agent Center | ⬜ Blocked on G2 | `Agent` trait, registry, IPC bus | G3 |
| S4 Runtime / OS Core | 🟡 Prep active | capability and policy interfaces, runtime design, await real grant type | G3 |
| S5 UX | ⬜ Blocked on G3 | Site cleanup, then agent shell | post-G3 |
| S6 Ecosystem Harvest | 🟡 Active | audit botgen/workpace/sheetgen and publish harvest ADRs | rolling |
| S7 Smart-Home Hub | ⬜ Blocked on G2/G3 | pairing needs stable profile types; implementation needs runtime | G4 |
| S8 Docs / Governance | 🟡 Active (background) | Stream scaffolding, CONTRIBUTING, ADR templates | rolling |

---

## Milestone tags

| Tag | Status | Condition |
|-----|--------|-----------|
| `v0.0.1-foundation` | 🟡 | G1 closed; tag pending |
| `v0.0.2-profile` | ⬜ | S2 profile v0 frozen |
| `v0.0.3-runtime` | ⬜ | S4 consent bus runnable |
| `v0.0.4-agents` | ⬜ | S3 agent center CLI with two reference agents |
| `v0.0.5-harvest` | ⬜ | S6 harvest ADRs merged |
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
| 2026-04-23 | G1 closed: CI run #24812246339 proved fmt, clippy, build, and test green across ubuntu-latest, macos-latest, and windows-latest. |
| 2026-04-22 | S1 workspace artifacts landed: Cargo workspace, CI workflows, site move, CODEOWNERS, `cargo xtask ci`, and initial `ferros-core` / `ferros-profile` crates. |
| 2026-04-21 | Wave 0 closed (contracts C1–C10 verified). Stream docs scaffolded. Stream-first planning model adopted. |

---

## Known blockers

| Blocker | Affects | Owner |
|---------|---------|-------|
| `v0.0.1-foundation` tag is not yet created; required status checks on `main` are not yet verified in branch protection | S1 | S1 |
| `ferros-profile` still needs key material, signature verification, frozen schemas, CLI flows, and schema parity tests for G2 | S2, S3, S7 | S2 |
| `ferros-core` is still a foundation skeleton only; runtime and policy engine work have not started | S4, S3, S7 | S4 |
