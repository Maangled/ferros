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
| Open streams | S1 (closeout), S2 (active), S3 (pre-G3 scaffold), S4 (policy slice), S5 (planning), S6 (active), S8 (background) |

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
| S3 Agent Center | 🟨 Pre-G3 scaffold | `ferros-agents` boundary and in-memory registry landed; CLI, reference agents, and runtime integration still pending | G3 |
| S4 Runtime / OS Core | 🟡 Active (policy slice) | deny-by-default capability/policy primitives are in `ferros-core`; `ferros-runtime` and `ferros-node` are not started yet | G3 |
| S5 UX | 🟨 Planning active; implementation blocked on G3 | shell composition note landed; no local web shell or site truth-banner slice is shipped yet | post-G3 |
| S6 Ecosystem Harvest | 🟡 Active | ADR-018/019/020 landed; downstream streams should consume ADR conclusions, not raw legacy repos | rolling |
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
| 2026-04-23 | S6 harvest ADRs landed: ADR-018 (`botgen-rust`), ADR-019 (`workpace-rust`), and ADR-020 (`sheetgen-rust`). |
| 2026-04-23 | S4 landed the first `ferros-core` capability/policy slice with focused deny-by-default tests; `ferros-runtime` and `ferros-node` remain unstarted. |
| 2026-04-23 | S3 landed a pre-G3 `ferros-agents` scaffold with manifest authorization helpers and an in-memory registry. |
| 2026-04-23 | S5 recorded shell direction in `SURFACE-FIRST-SHELL.md`; no HTML shell implementation has landed yet. |
| 2026-04-23 | G1 closed: CI run #24812246339 proved fmt, clippy, build, and test green across ubuntu-latest, macos-latest, and windows-latest. |
| 2026-04-21 | Wave 0 closed (contracts C1–C10 verified). Stream docs scaffolded. Stream-first planning model adopted. |

---

## Known blockers

| Blocker | Affects | Owner |
|---------|---------|-------|
| `v0.0.1-foundation` tag is not yet created; required status checks on `main` are not yet verified in branch protection | S1 | S1 |
| `ferros-profile` still needs key material, signature verification, frozen schemas, CLI flows, and schema parity tests for G2 | S2, S3, S7 | S2 |
| `ferros-agents` is still a pre-G3 scaffold only; lifecycle CLI, reference agents, and runtime hooks are not implemented | S3, S5 | S3 |
| `ferros-runtime` and `ferros-node` do not exist yet; current S4 progress stops at `ferros-core` policy primitives | S4, S3, S7 | S4 |
| S5 has planning artifacts only; the local web shell and site truth-banner slice are not implemented | S5 | S5 |
