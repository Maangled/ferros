# S8 Docs / Governance / Contributor Onboarding — Progress

Reverse-chronological. Append a dated entry at the top per session.

---

## 2026-04-23 — ADR-021 dependency policy formalized

- Added `docs/adr/ADR-021-dependency-admission-policy.md` to codify the current dependency posture: browser surfaces stay framework-free and package-free, the Rust substrate stays minimal, and third-party crates require narrow audited justification.
- Left broader docs untouched because `docs/AGENT_GUIDE.md`, ADR-015, and the current workspace manifests already reflect this rule; the new ADR is the governing consolidation.

## 2026-04-21 — Stream-first planning docs scaffolded

- Created `ROADMAP.md`, `LAUNCH.md`, `STATUS.md` at repo root.
- Scaffolded all 8 stream directories (`streams/S1-foundation/` through `streams/S8-docs/`) with README, PROGRESS, BACKLOG, and CONTRACTS files.
- Created gate docs `docs/gates/G1.md` through `G4.md`.
- Created `docs/adr/ADR-TEMPLATE.md`.
- Created `docs/contracts/CONTRACTS-OVERVIEW.md`.
- Updated `CONTRIBUTING.md` to route contributors to streams and gates.
- Stream model adopted; previous wave-based model preserved in `docs/streams/` and `docs/progress/` for historical reference.
