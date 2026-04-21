# Contributing to FERROS

FERROS uses a **stream-first** development model. Before opening a PR, read this document to find the right stream for your work.

---

## Quick start

1. **Find your stream.** Every task in FERROS belongs to one of eight streams. See the table below.
2. **Read the stream README.** Each stream has a `streams/SN-name/README.md` with mission, scope, backlog, and immediate next steps.
3. **Check the gate.** If the stream is blocked on a gate, see `docs/gates/GN.md` to understand what must be true before work can land.
4. **Open a PR** tagged `[SN]` in the title (e.g., `[S1] Add Cargo workspace root`).

---

## Stream map

| Stream | What it covers | Directory | Gate |
|--------|---------------|-----------|------|
| S1 Foundation | Cargo workspace, CI, tooling, site structure | `streams/S1-foundation/` | G1 |
| S2 Profile & Identity | `ferros-profile` crate, Ed25519 identity, schemas, profile CLI | `streams/S2-profile/` | G2 |
| S3 Agent Center | `ferros-agents` crate, agent registry, spawn/stop, IPC bus, agent CLI | `streams/S3-agent-center/` | G3 |
| S4 Runtime / OS Core | `ferros-core`, `ferros-runtime`, `ferros-node`, policy engine | `streams/S4-runtime/` | G3 |
| S5 UX | Site, agent center web shell, WASM demo | `streams/S5-ux/` | rolling |
| S6 Ecosystem Harvest | Audit + extract from `sheetgen-rust`, `botgen-rust`, `workpace-rust` | `streams/S6-harvest/` | rolling |
| S7 Smart-Home Hub | `ferros-hub`, Home Assistant integration, edge targets | `streams/S7-hub/` | G4 |
| S8 Docs / Governance | Docs, ADRs, contributor onboarding, governance | `streams/S8-docs/` | rolling |

---

## Gates

Gates are convergence points where stream outputs must be verified before the next phase opens.

| Gate | What it requires | Details |
|------|-----------------|---------|
| G1 | S1 green — CI passing on Linux/macOS/Windows | `docs/gates/G1.md` |
| G2 | S2 profile v0 frozen — types, schema, round-trip proven | `docs/gates/G2.md` |
| G3 | S3+S4 minimal agent-center-on-runtime demo | `docs/gates/G3.md` |
| G4 | S7 hub on real hardware — **this is launch** | `docs/gates/G4.md` |

---

## Contracts

Cross-stream interfaces (Rust traits, JSON Schemas, CLI APIs) are defined in contracts. Before adding a new cross-stream dependency, read `docs/contracts/CONTRACTS-OVERVIEW.md`.

Rules:
- Do not modify a contract you do not own. File an issue or PR on the owning stream instead.
- Do not mutate a frozen schema in place. New fields go into a new version.

---

## PR conventions

- Title format: `[SN] Brief description` — e.g., `[S2] Add ferros-profile crate scaffold`.
- PRs that touch contracts must update the owning stream's `CONTRACTS.md`.
- PRs that close a gate must update the corresponding `docs/gates/GN.md`.
- Keep PRs scoped to one stream where possible. Cross-stream changes require a short explanation of why the scope is necessary.

---

## Progress tracking

Each stream has a `streams/SN-name/PROGRESS.md`. If you work on a stream, append a dated entry. This keeps the project history honest and makes it easy for other contributors to pick up where you left off.

---

## Good first issues

Look for issues labelled `good-first-issue` and `stream/SN-*`. Each stream will have at least 5 seeded good-first-issues before the project invites external contributors publicly. If you are reading this before that point, you are an early contributor — welcome, and please reach out directly before starting work.

---

## Code of conduct

Be direct and honest. Respect others' time. Do not overpromise in documentation or PRs. If something is not working, say so.

A formal `CODE_OF_CONDUCT.md` will be added as part of S8 governance work.
