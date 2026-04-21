# FERROS — Status Dashboard

> Dashboard, not a diary. Each section shows current state. Details live in stream PROGRESS.md files.
>
> Last updated: 2026-04-21

---

## Overall

| Item | State |
|------|-------|
| Active gate | **G1** — Foundation |
| Launch gate | G4 (open) |
| MVP gate | G1 → G2 → G3 in sequence |
| Open streams | S1 (active), S8 (background) |

---

## Gate status

| Gate | Status | Condition |
|------|--------|-----------|
| G1 | 🔴 Open | `cargo build && cargo test` green on Linux/macOS/Windows; CI running |
| G2 | ⬜ Blocked | G1 must close first; S2 profile v0 frozen |
| G3 | ⬜ Blocked | G2 must close first; S3+S4 minimal agent-center-on-runtime demo |
| G4 | ⬜ Blocked | G3 must close first; `ferros-hub` on real hardware with HA integration |

---

## Stream status

| Stream | Status | Current focus | Gate |
|--------|--------|---------------|------|
| S1 Foundation | 🔴 Not started | Cargo workspace, CI, site move, tooling | G1 |
| S2 Profile & Identity | ⬜ Blocked on G1 | `ferros-profile` crate, Ed25519, schemas | G2 |
| S3 Agent Center | ⬜ Blocked on G2 | `Agent` trait, registry, IPC bus | G3 |
| S4 Runtime / OS Core | ⬜ Blocked on G1 | `ferros-core`, consent bus, executor | G3 |
| S5 UX | ⬜ Blocked on G1 | Site cleanup, then agent shell | post-G3 |
| S6 Ecosystem Harvest | ⬜ Blocked on G1 | Audit sheetgen/botgen/workpace, ADRs | rolling |
| S7 Smart-Home Hub | ⬜ Blocked on G3 | `ferros-hub`, HA integration | G4 |
| S8 Docs / Governance | 🟡 Active (background) | Stream scaffolding, CONTRIBUTING, ADR templates | rolling |

---

## Milestone tags

| Tag | Status | Condition |
|-----|--------|-----------|
| `v0.0.1-foundation` | ⬜ | S1 done |
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
| 2026-04-21 | Wave 0 closed (contracts C1–C10 verified). Stream docs scaffolded. Stream-first planning model adopted. |

---

## Known blockers

| Blocker | Affects | Owner |
|---------|---------|-------|
| Cargo workspace not yet set up | S1, all | S1 |
| No CI workflow | S1, all | S1 |
| `ferros-profile` crate does not exist | S2 | S2 |
| `ferros-core` crate does not exist | S4 | S4 |
