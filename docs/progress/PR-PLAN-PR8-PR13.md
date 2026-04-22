# PR Plan — PR 8 → PR 13 (Phase A → Phase B entry)

Companion to [`AGENDA-2026-04-21.md`](./AGENDA-2026-04-21.md) and [`PROGRESS.md`](./PROGRESS.md). Execution tracking lives in individual issues per PR, linked to the [Phase A umbrella issue](https://github.com/Maangled/ferros/issues). This doc is the durable plan.

Ordered by dependency. Each PR is scoped to be reviewable in a single sitting, mirrors the Wave 0 PR1→PR7 cadence (one narrow scope per PR), and has a binary merge gate.

## PR 8 — H8 clean rerun + environmental notes *(A2)*
- **Branch:** `pr8-h8-rerun-signoff`
- **Scope:**
  - Fresh browser session verification of `harnesses/ui-acceptance-harness.html`.
  - Short note in `docs/progress/` documenting the stable rerun environment.
  - `PROGRESS.md` Wave 0 Harness Status table: H8 row flips to `PASS — N/N`.
- **Merge gate:** H8 green in two independent sessions; PROGRESS.md updated; no code changes (evidence-only, like PR 7).
- **Size:** XS. **Risk:** none. **Blocks:** nothing.

## PR 9 — Consumer-helper harness (H9) *(A1)*
- **Branch:** `pr9-consumer-helper-harness`
- **Scope:**
  - New `harnesses/consumer-helper-harness.html` exercising `FerrosCore.loadProfile()`, `FerrosCore.saveProfile()`, `FerrosCore.pushAuditEntry()`.
  - Coverage matrix: all profile fixtures, corrupted JSON, quota-exceeded, audit ring-buffer cap (1000), session-mode write-path denial.
  - Register H9 in `docs/contracts/manifest.json`.
  - `PROGRESS.md`: add H9 row; remove helper enforcement known-gap note; remove reconciliation non-blocking hardening note.
- **Merge gate:** H9 green (pinned count); all existing harnesses still green; manifest validates.
- **Size:** M. **Risk:** medium. **Blocks:** PR 12.

## PR 10 — V4 alias → claim → XP merge *(A4)*
- **Branch:** `pr10-v4-alias-claim-xp-merge`
- **Status note:** Code scope is already present on `main` via commit `8d7c123` (`Add portable alias/recovery log claim support`). Treat PR 10 as code-complete; PR 11 remains the closure/evidence follow-through.
- **Scope:**
  - Implementation in monolith + `ferros-core.js` (FerrosCore is canonical path per PR 5).
  - New alias-session + claim-log fixture pair with golden XP merge result.
  - Extend H5 acceptance harness with alias-claim end-to-end assertion.
  - `PROGRESS.md` Tier 2: V4 flips to ✅.
- **Merge gate:** H5 green with new assertion; H1/H2/H4 still green; new fixtures validate against C1 + C7.
- **Size:** L. **Risk:** medium-high. **Blocks:** PR 11.

## PR 11 — Wave 1 closure evidence + audit reconciliation *(A3 + A5)*
- **Branch:** `pr11-wave1-closure-evidence`
- **Scope:**
  - New `docs/progress/WAVE-1-CLOSURE-EVIDENCE.md` (or append to `CLOSURE-EVIDENCE.md`): V7 round-trip artifact, V4 alias-claim transcript, full harness matrix at PR 10 HEAD.
  - Audit remediation: finding #9 (card/deck in export) — resolve with V5–V7 evidence or re-ticket; finding #19 (fixture co-location) — resolve or re-defer with rationale.
  - Wave status: "Wave 1 CLOSED — date" with evidence link.
- **Merge gate:** Evidence doc present; both audit rows have final dispositions; no code changes.
- **Size:** S. **Risk:** none. **Blocks:** PR 12 entry.

## PR 12 — Arena Export Target ADR *(B1)*
- **Branch:** `pr12-arena-export-adr`
- **Scope:**
  - New `docs/adr/ADR-016-arena-export-target.md`: deck export schema, runtime consumption contract, version negotiation, C5 ↔ C8 relationship for the export flow.
  - Cross-references from `docs/progress/forge.md` and `docs/progress/arena-runtime.md`.
  - `PROGRESS.md` Module Spec Files table updated.
  - No code changes — decide-before-building PR.
- **Merge gate:** ADR in Accepted status; both module specs cross-link it; contract manifest notes upcoming consumer.
- **Size:** M. **Risk:** low (docs only, high leverage).

## PR 13 — C6 runtime consumption spec *(B2)*
- **Branch:** `pr13-c6-runtime-consumption`
- **Scope:**
  - Promote `FerrosCore.templateToEvents()` to a consumed contract.
  - New/extended contract doc (append to `storage-rules.md` + runtime-host-v1.md addendum).
  - At least one schedule-event golden consumed by a template → events flow.
  - Extend H1 with C6 consumption assertions.
  - `PROGRESS.md`: remove C6 deferral gap; add contract coverage row.
- **Merge gate:** H1 green with new C6 assertions; fixture validates; gap note removed.
- **Size:** M. **Risk:** low-medium. **Blocks:** S1.

## Summary

| PR | Issue item | Status | Size | Scope type | Risk | Blocks |
|---|---|---|---|---|---|---|
| PR 8  | A2      | Merged #60 | XS | Evidence | None | — |
| PR 9  | A1      | Merged #64 | M | Harness (new) | Medium | PR 12 |
| PR 10 | A4      | Landed on `main` @ `8d7c123` | L | Product code | Medium-high | PR 11 |
| PR 11 | A3 + A5 | Planned | S | Evidence + doc | None | PR 12 |
| PR 12 | B1      | Planned | M | ADR (decision) | Low | PR 13, S1/S2 |
| PR 13 | B2      | Planned | M | Contract spec + harness | Low-medium | S1 |

**Phase A exit after PR 11. Phase B entry begins with PR 12.** The original PR 10 code scope is already present on `main`; the remaining Phase A work is closure evidence plus the PR 12/PR 13 on-ramp items.

## Conventions (carried over from Wave 0)

- One narrow scope per PR (no mixed harness + product code).
- Evidence PRs (PR 8, PR 11) are code-change-free — mirrors PR 7 pattern.
- Every PR runs the full harness regression check, not just the new thing.
- `PROGRESS.md` is updated in the same PR that earns the status change.
- Merge commits cite the capability ID (e.g., `V4`, `H9`, `C6`) in the subject.