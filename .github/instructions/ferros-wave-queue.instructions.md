---
name: FERROS Wave Queue Rules
description: "Use when updating FERROS local driver files, the orchestration wave queue, or the wave run log. Keeps queue items stable and run logs append-only."
applyTo: "docs/orchestration/*.md"
---

# FERROS Wave Queue Rules

- `docs/orchestration/WAVE-QUEUE.md` is the source of truth for queued local orchestration waves.
- Keep queue sections in this order: `Ready`, `In Progress`, `Blocked`, `Done`.
- Each queue item should keep this field order: `Title`, `Status`, `Priority`, `Gate`, `Owning streams`, `Goal`, `Anchor files`, `Validation`, `Constraints`, `Last update`.
- Preserve the queue item ID and core wording when moving an item between sections; only change fields that actually moved.
- `docs/orchestration/WAVE-RUN-LOG.md` is append-only with newest entries first. Do not edit past entries.
- Each run-log entry must record the selected item ID, result, validations, and next follow-up.

## Authority and policy references

- The canonical policy authority for all lane rules, stop conditions, and gatekeeper logic is `docs/orchestration/ORCHESTRATION-POLICY.md`.
- The canonical workflow authority for operating loop, mode selection, and batch rhythm is `docs/orchestration/ORCHESTRATION-EXECUTION.md`.
- The full index of canonical and shimmed docs is `docs/orchestration/AUTHORITY-MAP.md`.
- `LOCAL-DRIVER.md` and `BATCH-MODE.md` are **shims** — preserved for link stability, not active policy sources.

## Operational packet rules

- Every new operational packet (REENTRY doc, instruction packet, coordination note) must declare its authority source as: `Authority: docs/orchestration/AUTHORITY-MAP.md`
- Do **not** copy gate-state snapshots from `STATUS.md` into operational packets. Link to `STATUS.md` instead. Copied snapshots go stale and mislead future readers.
- When a packet references shim files, note that the canonical location has moved.
