# FERROS Orchestration Policy

> **Canonical authority.** This document owns all stable orchestration rules.
> Supersedes the policy sections of `LOCAL-DRIVER.md` and `BATCH-MODE.md`.
> See [AUTHORITY-MAP.md](AUTHORITY-MAP.md) for the full document index.

---

## Lane policy

### Ceilings

- Default safe ceiling: **8 parallel repo-editing lanes** per wave.
  - Raised from 5 after two consecutive conditional-pass Batch Mode runs (BATCH-2026-04-27 + BATCH-2026-04-27-B), both with named-and-resolvable ambiguities and non-trivial gatekeeper decisions.
  - **Revert clause:** if any subsequent batch fails — defined as a Triage/Trace escalation, a frozen-surface touch, or a halt before the final declared wave — the ceiling reverts to 5 in the next substrate-refinement wave.
- Total lane count across one wave (including approved recursive sub-lanes): **≤ 12**.
- Do not force 8 lanes when fewer safe non-overlapping lanes actually exist.

### Lane classes

Every planned lane belongs to one of four classes:

| Class | What it touches |
|-------|----------------|
| **Implementation** | Code or crate work on a stream-owned seam |
| **Harness** | Validators, acceptance harnesses, or targeted proof surfaces |
| **Docs-owner** | Stream-owned README, BACKLOG, CONTRACTS, or runway-doc updates |
| **Truth-sync** | Shared-truth surfaces: `STATUS.md`, gate docs, queue files, cross-stream progress reconciliation |

Preferred ordering: parallelize implementation and non-overlapping harness lanes first; allow docs-owner lanes only when their anchors stay stream-local; serialize truth-sync lanes after implementation lands.

### Critical-path reservation

Prefer to reserve **1 or 2 lanes** for the active critical path or gate-owner work when such work exists. Use the remaining safe lanes for non-overlapping support work such as runway docs, backlog reduction, ADR or research-note capture, HTML resurfacing or archive hygiene, and targeted review or truth-sync slices.

### Shared truth surfaces

Shared truth surfaces — `STATUS.md`, gate docs, contracts overview, queue files, CI files, and root workspace manifests — should be reconciled **after** implementation lanes land, not edited concurrently by multiple lanes. When a wave fans out into multiple internal lane packets, shared-truth surfaces should have **one declared write-owning lane** inside that wave. Other lanes may read the same surfaces for context but must not mutate them unless the mutation is the declared slice.

---

## Two-speed execution posture

| Posture | Use for |
|---------|---------|
| **Fast** | Reversible local work: implementation slices, harness repair, local artifact generation, queue-safe docs-owner updates, narrow truth sync |
| **Slow** | Credibility-sensitive work: gate movement, hardware or Home Assistant proof claims, privilege-boundary expansion, remote transport, security posture changes, branch-protection verification, release-tag assertions |

Slow-posture work should either run as Interactive-only by queue metadata (`gate-close` or `solo: true`) or stay outside an active batch segment until a human deliberately names it.

---

## Recursive lane policy

Recursive lane planning is allowed as a bounded refinement step only.

- The orchestrator may ask **FERROS Lane Architect Agent** for one more planning pass on a generated lane only after **FERROS Recursion Controller Agent** approves it.
- Maximum recursion depth: **2**. There is no third planning layer.
- Do not recurse lanes that touch **≤ 2 anchor files**.
- Do not recurse single-stream lanes with no contract or ownership seam.
- Do not recurse shared truth surfaces (`STATUS.md`, gate docs, contracts overview, queue files, CI files, root manifests).
- If a recursive child plan does not add a new seam, owner boundary, or anchor set, collapse it back into the parent lane.

---

## Failure handling chain

1. Use **FERROS Lane Validator Agent** before launch and after landing when the lane changes consumer-facing, launch-critical, or multi-file slices.
2. If a lane fails validation or implementation, route the earliest concrete failure through **FERROS Log Triage Agent** before widening the fix.
3. Escalate to **FERROS Trace Analyst Agent** only when the failure boundary remains ambiguous after triage.
4. If a lane discovers a new owning stream or contract seam mid-flight, escalate back to the parent orchestrator rather than freelancing a sibling lane.

---

## Stop conditions and segment boundaries

Any one of the following fires. Conditions 1, 2, 3, 4, and 6 are **hard stops**. Condition 5 is a **segment boundary**: it closes the current batch segment, emits the normal doc-batch artifact, and in Queue-Clear Mode immediately opens the next segment when no hard stop fired.

1. **Validation failed** — validation failed on the just-landed wave.
2. **Wave tag** — the just-finished wave or the next Ready wave is tagged `priority: P0`, `gate-close`, `solo: true`, or touches a frozen schema (`schemas/profile.v0.json`, `schemas/capability-grant.v0.json`).
3. **Diff overrun** — the landing diff touched files outside the wave's declared `anchor files` set, subject to the operational bookkeeping exemption below.
   - Shared-truth overlap is a special case of diff overrun: if more than one internal lane writes the same shared-truth surface, or if a non-truth-sync lane mutates queue, run-log, doc-batch, gate, or other cross-stream truth surfaces outside its declared slice, stop condition 3 fires unless the touch is covered by the bookkeeping exemption.
4. **Track boundary** — the next Ready wave's `track` value differs from the current batch's track.
5. **Run-length cap** — 8 waves have landed in the current batch segment. Emit the doc-batch summary and either continue automatically in Queue-Clear Mode or halt cleanly in bounded Batch Mode. *(Segment boundary, not a hard stop.)*
6. **Escalation chain exhausted** — the validator escalated to Log Triage, and Log Triage escalated to Trace Analyst without resolving the failure.

### Operational bookkeeping exemption

The following surfaces are exempt from stop condition 3. Every wave touches them as part of the wave-completion contract, not as uncontrolled scope expansion. Overrun does not fire for changes to:

- `docs/orchestration/WAVE-QUEUE.md`
- `docs/orchestration/WAVE-RUN-LOG.md`
- `docs/orchestration/SYSTEM-QUEUE.md`
- `docs/orchestration/HARDWARE-QUEUE.md`
- `docs/orchestration/doc-batches/DOC-BATCH-*.md`
- The **owner stream's** `streams/S*/PROGRESS.md` only. If a wave's declared owning stream is S5, then `streams/S5-ux/PROGRESS.md` is exempt; touching a different stream's PROGRESS.md is still an overrun.

Overrun fires on undeclared touches to: `crates/**`, `schemas/**`, `.github/workflows/**`, `tools/**`, undeclared shared-truth surfaces (`STATUS.md`, gate docs, `CONTRACTS-OVERVIEW.md`), another stream's anchor files, or a non-owner stream's `PROGRESS.md`.

---

## Gatekeeper decision enum

The gatekeeper produces exactly one of three decisions per wave:

| Decision | Meaning |
|----------|---------|
| `continue` | Wave clean; proceed to next wave in batch |
| `stop-clean` | Stop current batch segment cleanly; emit doc-batch artifact; no escalation |
| `stop-escalate` | Hard failure; emit escalation note in run log; halt batch |

`stop-clean` closes the current batch segment, not the overall queue-clear run when Queue-Clear Mode is active.

### Structured gatekeeper block format

Every gatekeeper evaluation must produce a block in this schema. The format is stable — it is the handoff contract for the planned migration of the gatekeeper role to a dedicated small-tier model:

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

---

## Queue item schema

Required fields for all three track queues:

```
Title
Status
Priority
Gate
Owning streams
Goal
Anchor files
Validation
Constraints
Last update
```

Optional scheduling fields (additive, do not break existing field order):

| Field | Values | Purpose |
|-------|--------|---------|
| `size` | `S` or `L` | S = ≤3 anchor files, single stream, single-crate or docs-only. L = multi-crate, multi-stream, or schema-touching. Batch Mode default consumes only S. |
| `parallel-safe-with` | `[WAVE-IDs]` | Explicit non-overlap declarations |
| `serial-after` | `WAVE-ID` | Must wait for prior wave |
| `solo` | `true` or `false` | Must run alone (truth-sync, gate close, schema freeze, shared truth surfaces) |
| `track` | `code`, `system`, or `hardware` | Which queue this belongs to |

---

## Batch-level verdict criteria

Applied by the human reviewer during doc-batch review:

| Verdict | Conditions |
|---------|-----------|
| **Clean pass** | All waves landed clean; gatekeeper returned `continue` throughout and `stop-clean` on the last wave; no overrun; no escalation; at least one non-trivial gatekeeper decision (a near-miss on a stop condition correctly held back). A run where the gatekeeper never encountered a candidate stop condition is not sufficient for a clean pass. |
| **Conditional pass** | All waves landed clean and gatekeeper behaved correctly; ≥1 substrate ambiguity surfaced and flagged. Normal, healthy outcome of a first proof run; triggers a substrate-refinement wave as the next queued item. A conditional pass does not downgrade to fail unless the named ambiguity is a blocking condition. |
| **Fail** | Triage or trace analysis required; a frozen surface was touched; batch halted before its declared scope completed; or gatekeeper escalated to `stop-escalate`. |

The verdict is set by the human during doc-batch review. The gatekeeper's `decision` field informs but does not automatically determine the verdict.

---

## Operational packet authority-header standard

Every operational packet (REENTRY doc, instruction packet, doc-batch) must declare its authority source in its header. Use this format:

```
Authority: docs/orchestration/AUTHORITY-MAP.md
```

Do not copy live gate-state snapshots (from `STATUS.md` or gate docs) into operational packets. Link to `STATUS.md` instead. Copied snapshots go stale and mislead future readers.

---

*Last updated: 2026-05-03*
