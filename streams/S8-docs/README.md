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

- [ ] `CONTRIBUTING.md` routes contributors to streams and gates (not a generic workflow).
- [ ] All 8 stream READMEs accurate and linked from `ROADMAP.md`.
- [ ] `STATUS.md` updated at least weekly while any stream is active.
- [ ] `docs/adr/ADR-TEMPLATE.md` exists.
- [ ] `THREAT-MODEL.md` skeleton exists (can be incomplete until S2/S4 stable).
- [ ] `SECURITY.md` skeleton exists.
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

1. Review and update `ROADMAP.md`, `LAUNCH.md`, `STATUS.md` (done in this PR).
2. Write/update `CONTRIBUTING.md` with stream routing.
3. Write `docs/adr/ADR-TEMPLATE.md`.
4. Write `docs/contracts/CONTRACTS-OVERVIEW.md`.
5. Write gate docs `G1.md`–`G4.md`.
6. Skeleton `THREAT-MODEL.md` and `SECURITY.md`.
