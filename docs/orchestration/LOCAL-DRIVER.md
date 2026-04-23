# FERROS Local Driver Pattern

This is the local operating pattern for running repeatable FERROS orchestration waves without pretending custom agents are background daemons.

## What runs where

- `.github/agents/ferros-driver.agent.md` is the user-facing entry point for local queue processing.
- `.github/agents/ferros-orchestrator.agent.md` remains the cross-stream coordinator that actually launches stream-owned lanes.
- `docs/orchestration/WAVE-QUEUE.md` is the local source of truth for actionable waves.
- `docs/orchestration/WAVE-RUN-LOG.md` is the append-only history of completed or blocked waves.

The runtime is still the local Copilot chat surface. The repo stores the queue, the run history, and the orchestration instructions.

## Operating loop

1. Invoke **FERROS Local Driver Agent** and ask it to run the next wave, or give it a specific wave ID.
2. The driver reads `WAVE-QUEUE.md`, selects the next ready item, marks it in progress, and delegates to **FERROS Orchestrator Agent**.
3. The orchestrator runs the owning stream lanes, returns the integration result, and the driver updates both queue and run log.
4. Reinvoke the driver for the next wave.

## Non-goals

- No background autonomy after the chat turn ends.
- No GitHub-hosted execution of the `.agent.md` stack.
- No silent batching of multiple queue items unless the user explicitly asks for it.

## Queue discipline

- Keep wave items small enough that one orchestration pass can reasonably finish or sharpen them.
- Use the queue to encode priority, gate impact, anchor files, and validation before launching work.
- If a wave is partially complete, move it back to `Ready` with a narrower next step instead of leaving vague notes in chat history.

## Suggested invocations

- Run the next FERROS wave.
- Process `WAVE-2026-04-23-01`.
- Run the next queue item and update the log.

## Relationship to legacy orchestration docs

`docs/ORCHESTRATION.md` captures older governance history. This local driver pattern is the current repo-backed execution loop for the active S1-S8 stream model.
