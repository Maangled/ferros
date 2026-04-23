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
- `docs/orchestration/WAVE-RUN-LOG.md` is append-only with newest entries first.
- Each run-log entry must record the selected item ID, result, validations, and next follow-up.
