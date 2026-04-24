# FERROS Orchestration Control Document v2

> **Status:** Historical governance document for the earlier Streams A-E planning model.
>
> Active day-to-day execution in the current repo is driven by the S1-S8 stream docs, the gate documents, and the local-driver queue under `docs/orchestration/`. That active local-driver model now includes bounded recursive lane planning, lane validation, and failure triage through hidden helper agents. Keep this file as governance background and precedent unless a later ADR explicitly reactivates or replaces its control model.

---

## §1 Dependency Graph

Stream A is the **hidden upstream dependency** for Streams B and C. "Schemas frozen" is not sufficient — the dependency is explicit and SHA-pinned:

> **B and C may draft in parallel, but no execution PR merges until the specific Stream A contract artifact it depends on is approved and referenced by SHA.**

### Blocking relationships

| Stream | Blocked By | Condition to Unblock |
|--------|------------|----------------------|
| A | Nothing — Wave 0 closed (PR #47) | N/A |
| B | Stream A | The exact `schemas/*.schema.json` file(s) B depends on must be approved and their merge SHA referenced in B's execution PR |
| C | Stream A | Same requirement as B — schema approval + SHA reference |
| D | Streams B **and** C | Artifacts from both B (profile data, agent directives, schedule events) and C (card fixtures, deck manifests) must be merged and referenced by SHA |
| E | Nothing (research track) | E is non-blocking — see §6 |

### Stream E isolation rule

E is parallel research only. It **may not** define requirements that active Streams A–D must follow. If E discovers a constraint that affects A–D, it files a cross-stream issue (see §5) rather than imposing it unilaterally.

### Dependency diagram

```
Stream A  ──────────────────────────────────┐
  │ (frozen schemas, contracts, fixture     │
  │  corpus — approved + SHA-pinned)        │
  ▼                                         ▼
Stream B                               Stream C
  │ (profile data, agent directives,    │ (card fixtures, deck manifests,
  │  schedule events)                   │  rendered surfaces)
  └──────────────┬──────────────────────┘
                 ▼
            Stream D
         (consumer surfaces)

Stream E  ── research only ──► cross-stream issue if constraint found
```

---

## §2 Approval Authority

### Plan PRs

- **Approver:** @Maangled (repo owner).
- ChatGPT and GitHub Copilot reviews are **advisory only** — they are not approval.
- A plan PR is not merged until @Maangled explicitly approves it.

### Execution PRs

- **Approver:** @Maangled.
- Every execution PR **must** reference:
  1. The approved plan file path and section (e.g., `docs/streams/STREAM-B-IDENTITY-COCKPIT.md §Wave 1`)
  2. The plan PR number (e.g., `Plan PR: #NN`)
  3. The specific exit criteria IDs from `docs/streams/STREAMS-OVERVIEW.md` that this PR closes (e.g., `V1, V2, S1`)
- An execution PR that does not meet all three requirements is **not reviewable**.

### Cross-stream conflicts

- **Arbitrator:** @Maangled.
- Escalation path:
  1. File a GitHub issue labeled `cross-stream-conflict`.
  2. Tag both affected stream plans in the issue body.
  3. Both associated execution PRs are blocked until the issue is closed by @Maangled.

---

## §3 Execution PR Traceability Rule

Every execution PR description **must** include the following traceability block verbatim (filling in the bracketed values). If this block is absent, the PR is **not reviewable**.

```markdown
## Traceability
- **Plan:** docs/streams/STREAM-X-*.md
- **Plan PR:** #NN
- **Exit criteria closed:** V1, V2 (or whichever IDs apply)
- **Stream A contract dependency:** schemas/profile.schema.json @ SHA (or "none")
```

**Field guidance:**

| Field | What to write |
|-------|---------------|
| `Plan:` | The exact relative path to the stream plan file and section heading that authorizes this work |
| `Plan PR:` | The PR number in which that plan was approved by @Maangled |
| `Exit criteria closed:` | One or more exit-criteria IDs as defined in `docs/streams/STREAMS-OVERVIEW.md` (e.g., `V1`, `V2`, `S1`) |
| `Stream A contract dependency:` | The filename and full commit SHA of the Stream A schema or contract this PR depends on, or the literal string `none` if there is no dependency |

---

## §4 Cross-Stream Reconciliation Phase

After **each** stream's plan PR has been individually reviewed and approved, a single reconciliation review must be conducted before any Wave 1 execution PR merges.

### What the reconciliation checks

| Check | Parties |
|-------|---------|
| B's optional field assumptions vs A's schema freeze | A ↔ B |
| C's card/deck export needs vs A's schema guarantees | A ↔ C |
| D's assumptions about artifacts from B and C | B, C ↔ D |
| E's research findings vs active stream assumptions | E ↔ A–D |

### Reconciliation process

1. Open a **GitHub issue** (not a PR) titled `Reconciliation Gate — Wave 1` labeled `reconciliation-gate`.
2. The issue body lists each check above with a checkbox.
3. @Maangled or a designated reviewer works through every checkbox.
4. **The reconciliation issue must be closed before any Wave 1 execution PR merges.**

> A PR that merges before the `reconciliation-gate` issue is closed violates this policy and must be reverted.

### Current reconciliation gate status

**Wave 1 Reconciliation Gate: CLOSED** — reviewer closure recorded 2026-04-19.

- **Tracking issue:** [Reconciliation Gate — Wave 1 #53](https://github.com/Maangled/ferros/issues/53) — closed as completed in [#55](https://github.com/Maangled/ferros/pull/55)
- **Gate document:** [`docs/progress/reconciliation-gate-wave1.md`](./progress/reconciliation-gate-wave1.md)
- **Total reconciliation items:** 20 (6 for A↔B, 5 for A↔C, 5 for B,C↔D, 4 for E↔A–D)
- **Items verified:** 20 / 20

The gate document contains the specific checkboxes for each cross-stream check, with concrete references to schemas, contracts, and stream plan assumptions. All items are verified and the reconciliation gate no longer blocks Wave 1 execution PR merges.

---

## §5 Anti-Drift Controls

A stream is **paused** if any one of the following conditions is discovered:

| Trigger | Description |
|---------|-------------|
| Schema ambiguity | Field semantics are unclear or in dispute between a producing stream and a consuming stream |
| Missing or stale fixtures/generators | Running the generators in `tools/` produces a `git diff` that is non-empty (per the drift rule in `tools/README.md`) |
| Conflicting exit criteria | Two or more streams claim ownership of the same exit-criteria ID with incompatible definitions |
| Required upstream artifact not merged | An execution PR depends on a Stream A (or B/C) artifact that has not yet been merged to its target branch |

### Pause procedure

1. Open a GitHub issue labeled `stream-paused:{stream-letter}` (e.g., `stream-paused:B`).
2. Reference the specific trigger and affected artifacts in the issue body.
3. All execution work on that stream stops.
4. Work on the stream resumes **only** when the pause issue is closed by @Maangled.

---

## §6 Stream E Boundary

Stream E is a **parallel research track**. Its scope and constraints are:

- **Consumes:** The Stream A fixture corpus as a conformance target (per `docs/streams/STREAMS-OVERVIEW.md`).
- **Does not redefine:** Audit rules, permission model, orchestration procedure, or conformance expectations for Streams A–D.
- **Does not block:** Stream E findings do not block or gate any execution PR in Streams A–D.
- **Does not impose:** Stream E may not add requirements that A–D must implement.

If Stream E discovers something that **should** affect A–D:

1. File a GitHub issue describing the finding.
2. Tag the affected stream plan(s) in the issue.
3. @Maangled decides whether to incorporate the finding into an active stream's plan.

Stream E **files requests — it does not impose**.

---

## §7 Tool Allocation

| Stream(s) | Assigned Tool | Rationale |
|-----------|---------------|-----------|
| A, D, E | **GitHub Copilot** (coding agent) | Contract/backend/research work; requires file-level reasoning across schemas and docs |
| B, C | **VS Code Copilot** | UI-heavy prototype work in single-file HTML; benefits from inline edit loop |
| Cross-stream reconciliation | **ChatGPT** | Advisory review of cross-cutting assumptions — **not approval authority** |

> Note: ChatGPT's role is advisory. Reconciliation findings from ChatGPT must still be signed off by @Maangled before any blocking issue is closed.

---

## §8 Future State

> ⚠️ **FUTURE STATE — not current execution scope.**
>
> The items below describe intended architecture that has **not** been scheduled, planned, or approved for any current wave. They are recorded here to preserve intent without polluting active operating procedure.

### Agent Command Center handoff

The Agent Command Center (`docs/agent-command-center.html`, now aligned with S5 Phase B intent rather than the old Stream B label) is envisioned as a future runtime control plane for cross-stream orchestration. When sufficiently mature, it may assume parts of the coordination role currently expressed through the gate docs, the local-driver queue, and related governance surfaces.

**This handoff is not scheduled.** It requires:
- Stream B Wave 2+ completion
- A formal ADR approving the governance transfer
- Explicit approval by @Maangled

Until those conditions are met, this document remains historical governance context. The active execution references are the current stream docs, gate docs, `STATUS.md`, and the local-driver files in `docs/orchestration/`.

## Current local-driver addendum

The active S1-S8 local-driver model is intentionally narrower than the historical A-E control model above.

- Top-level orchestration still begins with lane planning and keeps the safe ceiling at **5 parallel repo-editing lanes**.
- Hidden helper agents now allow one extra bounded planning pass per generated lane, capped at depth **2** and **12 total lanes** across a wave.
- Failed lanes are expected to route through log triage first and trace analysis only when the failing boundary remains ambiguous.
- Shared truth surfaces remain reconciliation targets, not concurrent implementation lanes.

For the active operating procedure, queue discipline, and lane rules, defer to `docs/orchestration/LOCAL-DRIVER.md`.

---

*Document owner: @Maangled. Last substantive revision: this file's merge commit SHA.*
