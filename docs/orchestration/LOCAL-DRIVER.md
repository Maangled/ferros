# FERROS Local Driver Pattern

This is the local operating pattern for running repeatable FERROS orchestration waves without pretending custom agents are background daemons.

## What runs where

- `.github/agents/ferros-driver.agent.md` is the user-facing entry point for local queue processing.
- `.github/agents/ferros-orchestrator.agent.md` remains the cross-stream coordinator that actually launches stream-owned lanes.
- Hidden helper agents now support the orchestrator: bounded recursive lane planning, lane validation, log triage, and trace analysis.
- `docs/orchestration/WAVE-QUEUE.md` is the local source of truth for actionable waves.
- `docs/orchestration/WAVE-RUN-LOG.md` is the append-only history of completed or blocked waves.

The runtime is still the local Copilot chat surface. The repo stores the queue, the run history, and the orchestration instructions.

## Operating loop

1. Invoke **FERROS Local Driver Agent** and ask it to run the next wave, or give it a specific wave ID.
2. The driver reads `WAVE-QUEUE.md`, selects the next ready item, marks it in progress, and delegates to **FERROS Orchestrator Agent**.
3. The orchestrator lane-plans the wave, uses bounded recursion only when a lane earns one more planning pass, validates lane scope before launch, routes failed lanes through log triage, then returns the integration result.
4. Reinvoke the driver for the next wave.

This is **Interactive Mode** — the default. For Batch Mode (multiple Ready waves per invocation without per-wave human re-invocation), see `docs/orchestration/BATCH-MODE.md`. All lane policy rules below apply inside every wave regardless of mode.

## Default lane policy

The default execution posture is not single-lane unless the work truly demands it.

- Every substantial orchestration pass should start with lane planning.
- The default safe ceiling is up to **5 parallel repo-editing lanes**.
- The total lane count across one wave should stay at or below **12**, including any approved recursive sub-lanes.
- Prefer to reserve **1 or 2 lanes** for the active critical path or gate-owner work when such work exists.
- Use the remaining safe lanes for non-overlapping support work such as runway docs, backlog reduction, ADR or research-note capture, HTML resurfacing or archive hygiene, and targeted review or truth-sync slices.
- Do not force 5 lanes when fewer safe non-overlapping lanes actually exist.
- Shared truth surfaces such as `STATUS.md`, gate docs, contracts overview, queue files, CI files, and root workspace manifests should usually be reconciled **after** the implementation lanes land, not edited concurrently by multiple lanes.
- When a gate closes or an achievement is verified, the next foundational push should repack the full lane budget against the next highest-leverage safe slice.

## Recursive lane policy

Recursive lane planning is allowed, but only as a bounded refinement step.

- The orchestrator may ask **FERROS Lane Architect Agent** for one more planning pass on a generated lane only after **FERROS Recursion Controller Agent** approves it.
- Recursion stops at depth **2**. There is no third planning layer.
- Do not recurse lanes that touch **2 or fewer anchor files**.
- Do not recurse single-stream lanes with no contract or ownership seam.
- Do not recurse shared truth surfaces such as `STATUS.md`, gate docs, contracts overview, queue files, CI files, or root manifests.
- If a recursive child plan does not add a new seam, owner boundary, or anchor set, collapse it back into the parent lane.

## Failure handling

- Use **FERROS Lane Validator Agent** before launch and after landing when the lane changes consumer-facing, launch-critical, or multi-file slices.
- If a lane fails validation or implementation, route the earliest concrete failure through **FERROS Log Triage Agent** before widening the fix.
- Escalate to **FERROS Trace Analyst Agent** only when the failure boundary remains ambiguous after triage.
- If a lane discovers a new owning stream or contract seam mid-flight, escalate back to the parent orchestrator rather than freelancing a sibling lane.

## Gatekeeper model intent

The gatekeeper role (used inside Batch Mode between waves) is currently performed by the primary orchestrator model as an inline self-review step. This is a known limitation: the same model that authors the wave is also reviewing it.

The intent is to migrate the gatekeeper role to a dedicated small-tier / fast model (e.g., a mini-tier model in the same tooling surface) when that becomes mechanically available. The structured gatekeeper block format documented in `docs/orchestration/BATCH-MODE.md` is the stable handoff contract for that migration — no redesign of the block schema or decision enum is required when the model swap occurs.

Until that migration, the inline self-review posture is acceptable for `size: S`, docs-only, non-gate-close waves. Gate-close waves, P0 waves, and frozen-schema-touching waves remain Interactive-only and are not subject to gatekeeper review.

## Non-goals

- No background autonomy after the chat turn ends.
- No GitHub-hosted execution of the `.agent.md` stack.
- No silent batching of multiple queue items unless the user explicitly invokes Batch Mode (see `docs/orchestration/BATCH-MODE.md`).

## Queue discipline

- Keep wave items small enough that one orchestration pass can reasonably finish or sharpen them.
- Use the queue to encode priority, gate impact, anchor files, and validation before launching work.
- Optional queue metadata fields (`size`, `parallel-safe-with`, `serial-after`, `solo`, `track`) are additive. Use them to enable Batch Mode scheduling and to express ordering dependencies. See `docs/orchestration/BATCH-MODE.md` for how Batch Mode uses these fields.
- If a wave is likely to need recursive planning, express that in the queued goal, constraints, or validation rather than adding new queue-only metadata.
- If a wave is partially complete, move it back to `Ready` with a narrower next step instead of leaving vague notes in chat history.
- A single queued wave may still fan out into multiple stream-owned lanes internally when the orchestrator can prove the lanes are non-overlapping.
- The three track queues are: `docs/orchestration/WAVE-QUEUE.md` (`track: code`), `docs/orchestration/SYSTEM-QUEUE.md` (`track: system`), and `docs/orchestration/HARDWARE-QUEUE.md` (`track: hardware`). Each track queue is consumed independently.

## Suggested invocations

- Run the next FERROS wave.
- Process `WAVE-2026-04-23-01`.
- Run the next queue item and update the log.

## Relationship to legacy orchestration docs

`docs/ORCHESTRATION.md` captures older governance history. This local driver pattern is the current repo-backed execution loop for the active S1-S8 stream model.
