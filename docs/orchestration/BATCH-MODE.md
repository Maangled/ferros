# FERROS Batch Mode

Batch Mode is an explicit, user-invoked operating mode that coexists with the existing Interactive Mode defined in `LOCAL-DRIVER.md`. Neither mode alters the lane policy (≤5 repo-editing lanes, ≤12 total). Batch Mode changes **how many queue items the driver processes per invocation**, not how each individual wave executes.

---

## Modes

### Interactive Mode (default today)

One wave per invocation. The driver runs one Ready item and stops. The human re-invokes for the next wave.

Interactive Mode is **required** when any of the following is true:
- Wave is tagged `priority: P0`.
- Wave is tagged `gate-close`.
- Wave touches a frozen schema (`schemas/profile.v0.json`, `schemas/capability-grant.v0.json`).
- Wave is tagged `solo: true`.
- The user has not explicitly requested Batch Mode.

Interactive Mode is the safe default. It is not deprecated by Batch Mode.

### Batch Mode (new)

The driver may continue processing Ready waves sequentially — without waiting for a human re-invocation between them — until any stop condition fires (see below).

**To invoke Batch Mode:** include explicit instruction in the driver invocation, e.g. "Run the next batch of code-track waves." or "Run Batch Mode on the system queue."

Batch Mode processes only `size: S` waves by default. `size: L` waves require explicit human approval before entering a batch run.

The **target planning width** for Batch Mode runs is 8 waves per batch, matching the queue backfill depth this document is designed to support. The editing-lane ceiling is now 8, matching this planning target, after two consecutive conditional-pass Batch Mode runs were recorded (BATCH-2026-04-27 + BATCH-2026-04-27-B). See `LOCAL-DRIVER.md` for the revert clause.

---

## Stop Conditions

Any one of the following fires → stop the batch and wait for human re-entry:

1. **Validation failed** on the just-landed wave.
2. **Wave tag:** the just-finished wave or the next Ready wave is tagged `priority: P0`, `gate-close`, `solo: true`, or touches a frozen schema.
3. **Diff overrun:** the landing diff touched files outside the wave's declared `anchor files` set, subject to the operational bookkeeping exemption below.

   #### Operational bookkeeping exemption

   The following surfaces are exempt from stop condition 3. Every wave touches them as part of the wave-completion contract, not as uncontrolled scope expansion. Overrun does not fire for changes to:

   - `docs/orchestration/WAVE-QUEUE.md`
   - `docs/orchestration/WAVE-RUN-LOG.md`
   - `docs/orchestration/SYSTEM-QUEUE.md`
   - `docs/orchestration/HARDWARE-QUEUE.md`
   - `docs/orchestration/doc-batches/DOC-BATCH-*.md`
   - The **owner stream's** `streams/S*/PROGRESS.md` only. If a wave's declared owning stream is S5, then `streams/S5-ux/PROGRESS.md` is exempt; touching a different stream's PROGRESS.md is still an overrun.

   Overrun fires on undeclared touches to: `crates/**`, `schemas/**`, `.github/workflows/**`, `tools/**`, undeclared shared-truth surfaces (`STATUS.md`, gate docs, `CONTRACTS-OVERVIEW.md`), another stream's anchor files, or a non-owner stream's `PROGRESS.md`.
4. **Track boundary:** the next Ready wave's `track` value differs from the current batch's track.
5. **Run-length cap:** 8 waves have landed in the current batch.
6. **Escalation chain exhausted:** the validator escalated to Log Triage, and Log Triage escalated to Trace Analyst without resolving the failure.

---

## Gatekeeper Agent Contract

Each wave landing in Batch Mode is reviewed by a lightweight **Gatekeeper** step before the driver moves on.

**Inputs:**
- Wave ID and goal text
- Anchor files declared in the queue item
- Validation result (pass / fail / escalated)
- Diff summary (files touched, line delta)
- Current tail of `WAVE-RUN-LOG.md` (last 5 entries)

**Decision:** one of:
- `continue` — batch proceeds to the next Ready wave
- `stop-clean` — batch halts gracefully; human re-entry at the next normal doc-batch checkpoint
- `stop-escalate` — batch halts; human re-entry required before any further automation

**Rationale:** one paragraph appended to `WAVE-RUN-LOG.md` as a sub-entry of the wave record.

**Policy scope only.** The Gatekeeper makes **policy decisions** (should the batch continue given what just happened?), not code or architecture review. Code and architecture review happens at doc-batch boundaries by the human.

**Recommended model class:** small / fast (e.g., a mini-tier model). The orchestrator stays on the larger model.

#### Structured block format

Each gatekeeper decision is recorded as a JSON block in the wave's run-log entry using this schema:

```json
{
  "wave_id": "WAVE-YYYY-MM-DD-NN",
  "stop_conditions_evaluated": {
    "1_validation_failed": "...",
    "2_wave_tag": "...",
    "3_diff_overrun": "...",
    "4_track_boundary": "...",
    "5_run_length_cap": "...",
    "6_escalation_chain": "..."
  },
  "decision": "continue | stop-clean | stop-escalate",
  "rationale": "..."
}
```

The block format is stable. When a dedicated small-tier gatekeeper model becomes mechanically available in the tooling, the gatekeeper role will swap to that model without redesigning the block schema. The structured block is the handoff contract for that migration.

---

## Human Re-entry Triggers

Batch Mode replaces the per-wave "proceed?" prompt with three well-defined re-entry surfaces. None of these require the human to be watching in real time:

### 1. Doc-batch ready

Every ~10 landed waves, **or** at a stream `PROGRESS.md` phase boundary, the orchestrator emits a doc-batch summary file at:

```
docs/orchestration/doc-batches/DOC-BATCH-YYYY-MM-DD.md
```

The file summarises:
- Which waves landed in the batch
- Which contracts moved or were confirmed stable
- Which stream PROGRESS.md phases advanced
- What is queued next and why

The human reviews the doc-batch file and decides whether to run another batch, redirect the queue, or switch to Interactive Mode.

### 2. Hardware demo ready

When a Track-C (`track: hardware`) deliverable is flashable or runnable on real hardware, the orchestrator:
1. Stops the batch.
2. Emits a verification list naming the exact checks the human should run.
3. Waits for human findings, which are filed under `docs/hardware/findings/`.

### 3. Gatekeeper escalation

When the Gatekeeper returns `stop-escalate`, the driver emits a brief escalation note in the run log and halts. Escalations are expected to be rare — the goal is one escalation per tens of waves at most.

---

## Interaction with Track Queues

Batch Mode scopes each run to one track. The three track queues are:

| Queue file | Track value | Ownership |
|---|---|---|
| `docs/orchestration/WAVE-QUEUE.md` | `code` | S1–S8 code and docs |
| `docs/orchestration/SYSTEM-QUEUE.md` | `system` | Legal, ledger, asset, onramp |
| `docs/orchestration/HARDWARE-QUEUE.md` | `hardware` | Firmware, bring-up, UX sessions |

A Batch Mode run that reaches a track-boundary stop condition (condition 4 above) simply halts at that boundary. The human re-invokes with the target track explicitly named if they want to continue in a different track.

---

## Batch-Level Verdict Criteria

The human reviewer applies one of three verdicts to each completed batch during doc-batch review:

- **Clean pass** — all waves landed clean, gatekeeper returned `continue` throughout and `stop-clean` on the last wave, no overrun fires under the narrowed rule, no escalation, and the run log shows at least one non-trivial gatekeeper decision (e.g., a near-miss on a stop condition that was correctly held back from firing). A run where the gatekeeper never encountered a candidate stop condition is not sufficient for a clean pass — it suggests the batch was not probing hard enough.
- **Conditional pass** — all waves landed clean and the gatekeeper behaved correctly, but ≥1 substrate ambiguity was surfaced and flagged. This is the normal, healthy outcome of a first proof run and triggers a substrate-refinement wave as the next queued item. A conditional pass does not downgrade to fail unless the named ambiguity is a blocking condition.
- **Fail** — triage or trace analysis is required; a frozen surface was touched; the batch halted before its declared scope completed; or the gatekeeper escalated to `stop-escalate`.

The verdict is set by the human during doc-batch review. The gatekeeper's `decision` field informs but does not automatically determine the verdict.

---

## Relationship to LOCAL-DRIVER.md

Batch Mode layers on top of the current default Interactive Mode. All existing lane policy rules, recursion limits, failure-handling routes, and queue discipline rules from `LOCAL-DRIVER.md` apply unchanged inside every wave that runs within a batch.

Do not delete or override any rule in `LOCAL-DRIVER.md` to accommodate Batch Mode. Batch Mode is an additive scheduling layer, not a replacement execution model.
