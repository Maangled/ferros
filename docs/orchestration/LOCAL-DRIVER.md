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
3. The orchestrator runs the default authorization chain: Lane Architect review, builder lane execution, validator review, risk or claim rationalization, Gatekeeper decision, then top-level orchestrator authorization.
4. The orchestrator uses bounded recursion only when a lane earns one more planning pass, validates lane scope before launch, routes failed lanes through log triage, and returns the integration result.
5. Reinvoke the driver for the next wave unless Batch Mode or Queue-Clear Mode is active.

Interactive Mode is the single-wave posture. For Batch Mode (multiple Ready waves per invocation without per-wave human re-invocation), see `docs/orchestration/BATCH-MODE.md`. When the explicit objective is `clear the queue`, Batch Mode runs in **Queue-Clear** posture: it keeps draining the selected queue, emits doc-batch summaries as non-blocking review artifacts, and stops only when a hard Batch Mode stop condition fires or the scoped queue is empty. All lane policy rules below apply inside every wave regardless of mode.

Human re-entry is not the default safety mechanism. Subagent review is. Human re-entry is reserved for user-directed work, repeated failure or missing-input loops, physical-world actions, and true user-authority or product-direction questions.

## Default lane policy

The default execution posture is not single-lane unless the work truly demands it.

- Every substantial orchestration pass should start with lane planning.
- The default safe ceiling is up to **8 parallel repo-editing lanes**. This ceiling was raised from 5 to 8 after two consecutive conditional-pass Batch Mode runs were recorded in `WAVE-RUN-LOG.md` (BATCH-2026-04-27 code-track proof run and BATCH-2026-04-27-B system-track run), both with named-and-resolvable ambiguities and non-trivial gatekeeper decisions. **Revert clause:** if any subsequent batch fails — defined as a Triage/Trace escalation, a frozen-surface touch, or a halt before the final declared wave — the ceiling reverts to 5 in the next substrate-refinement wave, without requiring partner-facing renegotiation.
- The total lane count across one wave should stay at or below **12**, including any approved recursive sub-lanes. This cap is unchanged.
- Prefer to reserve **1 or 2 lanes** for the active critical path or gate-owner work when such work exists.
- Use the remaining safe lanes for non-overlapping support work such as runway docs, backlog reduction, ADR or research-note capture, HTML resurfacing or archive hygiene, and targeted review or truth-sync slices.
- Do not force 8 lanes when fewer safe non-overlapping lanes actually exist.
- Treat planned lanes as belonging to one of four classes:
	- **Implementation lane** — code or crate work on a stream-owned seam.
	- **Harness lane** — validators, acceptance harnesses, or targeted proof surfaces.
	- **Docs-owner lane** — stream-owned README, BACKLOG, CONTRACTS, or runway-doc updates.
	- **Truth-sync lane** — shared-truth surfaces such as `STATUS.md`, gate docs, queue files, and cross-stream progress reconciliation.
- Prefer to parallelize implementation lanes and non-overlapping harness lanes first, allow docs-owner lanes only when their anchors stay stream-local, and serialize truth-sync lanes after implementation lands.
- Shared truth surfaces such as `STATUS.md`, gate docs, contracts overview, queue files, CI files, and root workspace manifests should usually be reconciled **after** the implementation lanes land, not edited concurrently by multiple lanes.
- When a gate closes or an achievement is verified, the next foundational push should repack the full lane budget against the next highest-leverage safe slice.

## Two-speed execution posture

Execution tempo is separate from Interactive vs Batch Mode.

- **Fast posture** is for reversible local work: implementation slices, harness repair, local artifact generation, queue-safe docs-owner updates, and narrow truth sync.
- **Slow posture** is for credibility-sensitive work: gate movement, hardware or Home Assistant proof claims, privilege-boundary expansion, remote transport, security posture changes, branch-protection verification, and release-tag assertions.
- Slow-posture work should either run as Interactive-only by queue metadata (`gate-close` or `solo: true`) or stay outside an active batch segment until a human deliberately names it.

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

## Review roles inside a wave

The orchestrator remains the authorizing authority, but it should delegate the safety checks to better-scoped subagents instead of treating human re-entry as the normal brake.

- **Lane Architect** confirms anchors, decomposition, and whether a wave can be safely split.
- **Builder lane(s)** implement only the declared slice.
- **Validator** runs the targeted executable checks.
- **Risk or claim rationalizer** verifies that scope, gate language, hardware claims, transport claims, and privilege posture stayed inside the wave constraints.
- **Gatekeeper** converts the reviews into `continue`, `stop-clean`, or `stop-escalate`.
- **Top-level orchestrator** makes the final go/no-go call for the next segment.

## Standard batch rhythm

When Batch Mode or Queue-Clear Mode is active, reuse the same batch shape instead of improvising a new flow each time:

1. Read the scoped queue.
2. Select the next non-overlapping lanes.
3. Execute the lanes.
4. Run the targeted validations for the landed waves.
5. Run one broader safety harness when the touched surface has one.
6. Append the run log.
7. Perform serial truth sync only after the owner lanes land.
8. Emit the claim ledger described in `docs/orchestration/BATCH-MODE.md`.

## Gatekeeper model intent

The gatekeeper role (used inside Batch Mode between waves) is currently performed by the primary orchestrator model as an inline self-review step. This is a known limitation: the same model that authors the wave is also reviewing it.

The intent is to migrate the gatekeeper role to a dedicated small-tier / fast model (e.g., a mini-tier model in the same tooling surface) when that becomes mechanically available. The structured gatekeeper block format documented in `docs/orchestration/BATCH-MODE.md` is the stable handoff contract for that migration — no redesign of the block schema or decision enum is required when the model swap occurs.

Until that migration, the inline self-review posture is acceptable for `size: S`, docs-only, non-gate-close waves. Gate-close waves, P0 waves, and frozen-schema-touching waves remain Interactive-only and are not subject to gatekeeper review.

## Non-goals

- No background autonomy after the chat turn ends.
- No GitHub-hosted execution of the `.agent.md` stack.
- No silent batching of multiple queue items unless the user explicitly invokes Batch Mode or explicitly asks to clear or drain a queue (see `docs/orchestration/BATCH-MODE.md`).

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
