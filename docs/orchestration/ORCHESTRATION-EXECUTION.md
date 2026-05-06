# FERROS Orchestration Execution

> **Canonical authority.** This document owns all orchestration workflow: how waves are selected, how lanes run, and how batches progress.
> Supersedes the workflow sections of `LOCAL-DRIVER.md` and `BATCH-MODE.md`.
> See [AUTHORITY-MAP.md](AUTHORITY-MAP.md) for the full document index.
> For all stable rules (ceilings, stop conditions, exemption lists, decision enums), see [ORCHESTRATION-POLICY.md](ORCHESTRATION-POLICY.md).

---

## Operating loop

1. Invoke **FERROS Local Driver Agent** and ask it to run the next wave, or give it a specific wave ID.
2. The driver reads the relevant queue, selects the next ready item, marks it in progress, and delegates to **FERROS Orchestrator Agent**.
3. The orchestrator runs the default authorization chain: Lane Architect review → builder lane execution → validator review → risk or claim rationalization → Gatekeeper decision → top-level orchestrator authorization.
4. The orchestrator uses bounded recursion only when a lane earns one more planning pass, validates lane scope before launch, routes failed lanes through log triage, and returns the integration result.
5. Continue automatically when Batch Mode or Queue-Clear Mode is active; stop after one wave when Interactive Mode is required.

---

## Mode selection

### Interactive Mode

One wave per invocation. The driver runs one Ready item and stops. The human re-invokes for the next wave.

Interactive Mode is **required** when any of the following is true:

- Wave is tagged `priority: P0`.
- Wave is tagged `gate-close`.
- Wave touches a frozen schema (`schemas/profile.v0.json`, `schemas/capability-grant.v0.json`).
- Wave is tagged `solo: true`.
- The current invocation is question-driven rather than execution-driven — a planning question, code question, product-direction question, requested message, or specific choice that only the user can authorise — and the question itself is the requested outcome.
- The agent is looping, stuck, or needs external or user-grounded input that the repo and tool surface cannot supply.
- The wave requires a physical-world action the agent cannot perform directly.

Interactive Mode is not deprecated by Batch Mode. It is the required posture for question-driven or authority-sensitive work.

### Batch Mode (default execution posture)

The driver may continue processing Ready waves sequentially — without waiting for a human re-invocation between them — until any stop condition fires.

Batch Mode applies by default to execution-oriented invocations unless an Interactive-only condition is present.

Batch Mode processes `size: S` waves by default. `size: L` waves are eligible when the orchestrator runs the full review stack: Lane Architect → builder lane(s) → validator → scope or claim rationalization → Gatekeeper → top-level orchestrator authorization.

`size: L` does **not** by itself require human approval. Human re-entry is reserved for Interactive-only conditions.

### Queue-Clear Mode (drain posture)

Queue-Clear Mode is an explicit Batch Mode objective: the user tells the driver to clear or drain a queue, and the driver keeps opening successive batch segments on that scoped queue until the queue is empty or a hard stop fires.

Queue-Clear Mode keeps the same lane ceilings, validation rules, and gatekeeper contract as Batch Mode. The difference is scheduling intent:

- `stop-clean` closes the current batch segment, not the overall queue-clear run.
- The run-length cap (stop condition 5) emits a doc-batch artifact and opens the next segment automatically when no hard stop fired.
- Human doc-batch review remains the re-entry surface, but it is asynchronous and non-blocking while queue clear is active.

### Mixed invocations: question plus execution

Not every user question forces Interactive Mode. The controlling distinction is whether the question is the requested outcome or a light preface to continued execution.

Treat as **Batch Mode** when the user is still clearly directing execution and the question is lightweight, bounded, or deferrable:
- "Give me a progress update and then proceed."
- "Proceed and lay out the next batches leading up to opening another writer lane."
- "Keep going, and tell me where we are first."

In those mixed invocations: give the brief answer first if it is cheap and local; then continue the batch in the same response.

Treat as **Interactive Mode** when the question is itself the requested deliverable, or when answering it requires a user-authority decision, product-direction choice, or a deeper investigative pass that would materially interrupt execution.

---

## Default authorization chain

Human re-entry is not the default safety mechanism. Subagent review is.

1. Top-level orchestrator
2. Lane Architect review
3. Builder lane execution
4. Validator review
5. Risk or claim rationalization review
6. Gatekeeper decision
7. Top-level orchestrator authorization for the next segment

The human becomes the next authority only when an Interactive-only condition is present.

---

## Standard batch rhythm

When Batch Mode or Queue-Clear Mode is active, reuse this shape instead of improvising a new flow:

1. Read the scoped queue.
2. Select the next non-overlapping lanes.
3. Execute the lanes.
4. Run the targeted validations for the landed waves.
5. Run one broader safety harness when the touched surface has one.
6. Append the run log.
7. Perform serial truth sync only after the owner lanes land.
8. Emit the claim ledger (see §Claim ledger below).

The target planning width is **8 waves per batch segment**, matching the editing-lane ceiling.

---

## Claim ledger

Every completed batch segment must emit a short claim ledger alongside the doc-batch summary. Required fields:

- `Claims added` — `None` or an exact list of newly supported claims.
- `Claims explicitly not added` — list important non-claims (gate closure, hardware proof, Home Assistant proof, remote transport, or privilege expansion when relevant).
- `Evidence produced` — tests, harness runs, artifacts, logs, or other concrete outputs.
- `Truth surfaces touched` — shared-truth files updated in the segment.

The claim ledger prevents the batch narrative from getting ahead of the evidence.

---

## Human re-entry surfaces

Batch Mode replaces the per-wave "proceed?" prompt with three defined re-entry surfaces. None require the human to be watching in real time.

### 1. Doc-batch ready

Every ~10 landed waves, **or** at a stream `PROGRESS.md` phase boundary, the orchestrator emits a doc-batch summary file at:

```
docs/orchestration/doc-batches/DOC-BATCH-YYYY-MM-DD.md
```

The file summarises: which waves landed, which contracts moved or were confirmed stable, which stream PROGRESS.md phases advanced, and what is queued next and why.

In Queue-Clear Mode, doc-batch emission is not a blocking checkpoint — the driver keeps draining until the queue empties or a hard stop fires.

### 2. Hardware demo ready

When a Track-C (`track: hardware`) deliverable is flashable or runnable on real hardware, the orchestrator stops the batch, emits a verification list naming the exact checks the human should run, and waits for human findings filed under `docs/hardware/findings/`.

### 3. Gatekeeper escalation

When the Gatekeeper returns `stop-escalate`, the driver emits a brief escalation note in the run log and halts.

---

## Operator session integration

When human work leaves agent-only execution, route it through the operator session pattern instead of treating it as an ad hoc chat interruption.

### Operator session loop

1. A Human Test Backlog item becomes `Ready` after preflight gates pass.
2. The agent creates or selects an instruction packet for the operator.
3. The operator performs the task and records results and comments.
4. The Backlog Coordinator triages the result.
5. The item is either closed, routed to Agent Backlog, routed to a hotfix lane, routed to Hardware Queue, or escalated.
6. Completed agent work returns to Human Test Backlog when human retest is required.
7. The next ready human item is issued.

### Required artifacts before an operator session begins

1. A named Human Test Backlog item
2. A session or instruction file
3. Expected success criteria
4. Stop criteria and rollback path
5. Findings or evidence capture location
6. Named coordinator review path

### Instruction packet fields

```
Session ID:
Backlog item:
Operator:
Goal:
Commands or routes to use:
Expected observation:
Evidence capture path:
Rollback path:
Stop criteria:
```

### Operator session states

| State | Meaning | Owner |
|-------|---------|-------|
| Planned | Listed, not yet ready | Coordinator |
| Ready | Preflight gates passed; operator can be instructed | Agent + Coordinator |
| In Operator Session | Human execution in progress | Operator |
| Awaiting Coordinator Review | Result captured and waiting for triage | Coordinator |
| Hotfix | A front-of-queue local repair is required | Agent |
| Waiting On Agent Backlog | Implementation or docs work required before retest | Agent |
| Waiting On Hardware Queue | Environment or hardware work required | Coordinator + hardware lane |
| Closed | Item completed with acceptable evidence | Coordinator |
| Escalated | Human decision or architectural redirection required | Coordinator + user |

### Coordinator outcomes

After each session item the Backlog Coordinator must produce one of:

1. **Close** — result met expectations and evidence is sufficient.
2. **Hotfix** — a narrow local defect should be repaired before broader progress continues.
3. **Agent Backlog** — implementation, harness, or docs work required, but not urgent enough to preempt the active queue.
4. **Hardware Queue** — physical, topology, device, or environment work required.
5. **Escalate** — operator feedback implies a product-direction, policy, or architecture decision that should not be resolved inside the current slice.

### Hotfix lane rule

If an operator session finds a defect that blocks the next meaningful human step, the coordinator may push a hotfix to the front of Agent Backlog. Hotfixes must be small, bounded to the observed defect, and validated before the same operator item is retried.

### Human comments handling

Split human comments into two classes:

1. **Immediate-task comments** — comments directly about the current item, its pass/fail status, or a local defect. These stay with the session item.
2. **Meta comments** — broader architecture, product-direction, or planning feedback. These go to the Backlog Coordinator and may create an Agent Backlog item, ADR candidate, research note candidate, or queue reprioritization decision.

---

## Response shape guidance

Batch Mode replies should optimize for continued execution rather than repeated checkpoint prompts.

- If the user gives a direct implementation goal with no real question, keep running sequential batches until a hard stop, a meaningful handoff boundary, or the stated goal is reached.
- During a continuing run, progress updates should stay brief unless the user explicitly asks for depth.
- Close an execution-oriented run with an executive summary when the batch reaches the requested goal, a human-test boundary, or another real handoff surface.
- Do not end execution-oriented replies with option menus or branching choice lists.
- If a sanity pause is required before proceeding, end with a short summary, the concrete next plan, and one direct proceed question.

---

## Human re-entry rule

Human re-entry is not the default safety mechanism. Subagent review is. Human re-entry is reserved for:

- Question-driven work where the question itself is the requested outcome.
- Repeated failure or missing-input loops.
- Physical-world actions.
- True user-authority or product-direction questions.

When human re-entry becomes a named operator step, route it through the operator session pattern above and the Human Test Backlog instead of treating it as an ad hoc chat interruption.

---

*Last updated: 2026-05-03*
