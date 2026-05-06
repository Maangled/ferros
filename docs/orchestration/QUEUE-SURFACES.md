# FERROS Queue Surfaces

> **Canonical authority.** This document describes the three queue tracks, their ownership, and the append-only run history.
> See [AUTHORITY-MAP.md](AUTHORITY-MAP.md) for the full document index.
> For queue item schema and optional scheduling fields, see [ORCHESTRATION-POLICY.md](ORCHESTRATION-POLICY.md) §Queue item schema.

---

## Three queue tracks

Batch Mode scopes each run to one track. The three track queues are:

| Queue file | `track` value | Ownership | Contents |
|------------|---------------|-----------|----------|
| [`WAVE-QUEUE.md`](WAVE-QUEUE.md) | `code` | S1–S8 code and docs | Code, crate, shell, docs-adjacent implementation waves |
| [`SYSTEM-QUEUE.md`](SYSTEM-QUEUE.md) | `system` | Legal, ledger, asset, onramp | Legal docs, ledger entries, asset hygiene, onramp operations |
| [`HARDWARE-QUEUE.md`](HARDWARE-QUEUE.md) | `hardware` | Firmware, bring-up, UX sessions | Hardware bring-up, firmware, device, Home Assistant, UX sessions |

A Batch Mode run that reaches a track-boundary stop condition (condition 4) halts at that boundary. Queue-Clear Mode drains the scoped track queue by default — it does not silently hop into a different track unless the original invocation named that broader scope.

---

## Queue item lifecycle

Items move through these statuses in each queue file:

| Status | Meaning |
|--------|---------|
| `Ready` | Preflight gates passed; eligible for selection |
| `In Progress` | Currently active in a wave |
| `Blocked` | Cannot proceed; blocker named in item |
| `Done` | Completed; moved to Done section |

---

## WAVE-RUN-LOG.md — append-only run history

`docs/orchestration/WAVE-RUN-LOG.md` is the append-only record of all completed or blocked waves across all tracks.

**Do not edit past entries.** The run log is historical evidence. Past entries are cited by REENTRY packets, ADR research notes, gatekeeper model notes, and batch-level verdict records. Editing them invalidates those citations.

New entries are appended by the orchestrator at wave completion as part of the standard batch rhythm (step 6: "Append the run log").

---

## Bookkeeping exemption (summary)

All five queue files and stream PROGRESS.md files are exempt from the diff-overrun stop condition (stop condition 3). Updating them is part of the wave-completion contract. See [ORCHESTRATION-POLICY.md](ORCHESTRATION-POLICY.md) §Operational bookkeeping exemption for the full list.

---

## Doc-batch summary files

Every ~10 landed waves the orchestrator emits a batch summary at:

```
docs/orchestration/doc-batches/DOC-BATCH-YYYY-MM-DD.md
```

These files are human review artifacts. They are also exempt from the diff-overrun check. In Queue-Clear Mode they are non-blocking checkpoints — the driver keeps draining until a hard stop fires or the queue empties.

---

*Last updated: 2026-05-03*
