# FERROS Orchestration Authority Map

This is the front door for the orchestration documentation. Read this first.

---

## Canonical documents (active authority)

| Document | Owns |
|----------|------|
| [ORCHESTRATION-POLICY.md](ORCHESTRATION-POLICY.md) | All stable rules: lane ceilings, lane classes, two-speed posture, recursive lane policy, failure handling chain, stop conditions, bookkeeping exemption list, gatekeeper decision enum, queue item schema, overrun rules |
| [ORCHESTRATION-EXECUTION.md](ORCHESTRATION-EXECUTION.md) | All workflow: operating loop, mode selection (Interactive / Batch / Queue-Clear / Mixed), standard batch rhythm, steering checkpoint, human re-entry triggers, operator session integration, mixed invocation handling, response shape guidance |
| [ORCHESTRATION-AGENTS.md](ORCHESTRATION-AGENTS.md) | Role map: all named agent roles, invocation decision tree, delegation rules, stream agent list, specialist agent list |
| [QUEUE-SURFACES.md](QUEUE-SURFACES.md) | Three queue tracks (code / system / hardware), purpose and ownership of each, queue item schema pointer, WAVE-RUN-LOG as append-only history |
| [AUTHORITY-MAP.md](AUTHORITY-MAP.md) | This file — the index and shim registry |

---

## Shimmed documents (legacy, preserved for link stability)

These files still exist and may still be referenced by WAVE-RUN-LOG history entries, ADR research notes, and REENTRY packets. They are **not** the active policy source. Each carries a shim header pointing here.

| Document | Shimmed by | Legacy content now in |
|----------|------------|----------------------|
| [LOCAL-DRIVER.md](LOCAL-DRIVER.md) | Shim header at top | ORCHESTRATION-POLICY.md + ORCHESTRATION-EXECUTION.md |
| [BATCH-MODE.md](BATCH-MODE.md) | Shim header at top | ORCHESTRATION-POLICY.md + ORCHESTRATION-EXECUTION.md |
| [OPERATOR-SESSION-PATTERN.md](OPERATOR-SESSION-PATTERN.md) | Shim header at top | ORCHESTRATION-EXECUTION.md §Operator Session |

Shim lifespan: each shimmed file declares its removal target in its shim header. Do not remove a shim file until all live operational packets (REENTRY docs, active WAVE-QUEUE items) that reference it have been updated or closed.

---

## Out-of-scope (historical)

`docs/ORCHESTRATION.md` is a historical narrative document, not a policy authority. It may reference `LOCAL-DRIVER.md` for the active procedure — that sentence will be updated to point here after the next migration cycle.

---

## Quick-reference: which file answers my question?

| Question | Go to |
|----------|-------|
| How many lanes can run in parallel? | ORCHESTRATION-POLICY.md §Lane policy |
| What triggers a hard stop during a batch? | ORCHESTRATION-POLICY.md §Stop conditions |
| What files are exempt from the diff-overrun check? | ORCHESTRATION-POLICY.md §Bookkeeping exemption |
| How does Interactive Mode differ from Batch Mode? | ORCHESTRATION-EXECUTION.md §Mode selection |
| What does the standard batch rhythm look like? | ORCHESTRATION-EXECUTION.md §Standard batch rhythm |
| When does a human need to re-enter? | ORCHESTRATION-EXECUTION.md §Human re-entry |
| Who is the Gatekeeper and what does it decide? | ORCHESTRATION-AGENTS.md §Gatekeeper |
| What is the structured gatekeeper block format? | ORCHESTRATION-EXECUTION.md §Gatekeeper block |
| Where is the code-track queue? | QUEUE-SURFACES.md |
| Where is the run history? | WAVE-RUN-LOG.md (append-only, do not edit past entries) |

---

*Last updated: 2026-05-03*
