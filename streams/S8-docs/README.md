# S8 — Docs / Governance / Contributor Onboarding

**Stream:** S8  
**Status:** 🟡 Active (background cadence)  
**Gate:** Rolling — no single gate; contributes to all gates indirectly

---

## Mission

A new contributor should be able to pick a stream, read one document, and open a meaningful PR within an hour. This stream keeps the documentation honest, concise, and synchronized with actual project state. It operates on a weekly cadence in the background of all other streams.

---

## Scope

- Stream planning docs: this directory and equivalents for S1–S7.
- Top-level docs: `ROADMAP.md`, `LAUNCH.md`, `STATUS.md`, `CONTRIBUTING.md`.
- Architecture docs: `docs/architecture-overview.md` (update), `ARCHITECTURE.md` (if needed).
- Governance: `GLOSSARY.md`, `THREAT-MODEL.md`, `SECURITY.md`, `GOVERNANCE.md`, `CODE_OF_CONDUCT.md`.
- ADR templates and process: `docs/adr/ADR-TEMPLATE.md`.
- Contracts overview: `docs/contracts/CONTRACTS-OVERVIEW.md`.
- Issue and PR templates: `.github/ISSUE_TEMPLATE/`, `.github/PULL_REQUEST_TEMPLATE.md`.
- Per-stream `good-first-issue` seeding: 5 per stream before inviting contributors.
- Weekly `STATUS.md` updates: keep the dashboard current.

---

## Out of scope

- Writing application code (all other streams).
- Threat modeling that requires completed system design — `THREAT-MODEL.md` is a living doc; initial skeleton is in scope, but it cannot be complete until S2/S4 are stable.

---

## Dependencies

- **S1 (G1):** CONTRIBUTING.md routing depends on the stream model being live in the repo.

---

## What this stream blocks

- **Contributor scale-up:** No external contributors until S8 has per-stream good-first-issue seeding and CONTRIBUTING.md routing done.
- **Governance credibility:** THREAT-MODEL.md and SECURITY.md are gate requirements for S7 (launch).

---

## Definition of done (rolling)

- [x] `CONTRIBUTING.md` routes contributors to streams and gates (not a generic workflow).
- [ ] All 8 stream READMEs accurate and linked from `ROADMAP.md`.
- [ ] `STATUS.md` updated at least weekly while any stream is active.
- [x] `docs/adr/ADR-TEMPLATE.md` exists.
- [ ] `THREAT-MODEL.md` skeleton exists (can be incomplete until S2/S4 stable).
- [x] `SECURITY.md` skeleton exists.
- [ ] 5 `good-first-issue`-labelled issues seeded per stream before public contributor invite.

---

## Likely files

| Path | Role |
|------|------|
| `ROADMAP.md` | Stream model + gate-based coordination |
| `LAUNCH.md` | Precise launch definition |
| `STATUS.md` | Dashboard |
| `CONTRIBUTING.md` | Contributor routing to streams/gates |
| `GLOSSARY.md` | Shared vocabulary |
| `THREAT-MODEL.md` | Security threat model skeleton |
| `SECURITY.md` | Vulnerability reporting policy |
| `GOVERNANCE.md` | Project governance |
| `CODE_OF_CONDUCT.md` | Contributor code of conduct |
| `docs/adr/ADR-TEMPLATE.md` | ADR template |
| `docs/contracts/CONTRACTS-OVERVIEW.md` | Contracts overview |
| `streams/SN-*/` | Per-stream planning dirs |
| `docs/gates/G*.md` | Gate documents |

---

## Immediate next steps

1. Land `THREAT-MODEL.md` as the minimum launch-facing skeleton and keep it explicitly incomplete where S2/S4 are still moving.
2. Add `GOVERNANCE.md` and `CODE_OF_CONDUCT.md` so governance docs match the current stream charter.
3. Add `.github/ISSUE_TEMPLATE/` and `.github/PULL_REQUEST_TEMPLATE.md` for contributor intake.
4. Seed 5 `good-first-issue` candidates per stream before any broader contributor invite.
5. Keep `docs/contracts/CONTRACTS-OVERVIEW.md` synchronized with already-landed contract surfaces as S2-S4 docs evolve.
