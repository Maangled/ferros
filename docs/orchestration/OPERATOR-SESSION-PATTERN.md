# Operator Session Pattern

This document defines the human-in-the-loop execution pattern for FERROS operator sessions.

The goal is to make operator work explicit, reviewable, and queueable instead of handling it ad hoc in chat.

---

## Terminology

- **Human Test Backlog**: the ordered list of operator-visible tests waiting for human execution
- **Agent Backlog**: the ordered list of implementation, docs, and harness work owned by agents
- **Backlog Coordinator**: the role that triages operator results, routes human comments, and decides whether an item closes, becomes a hotfix, or re-enters the agent backlog
- **Operator session**: one named human execution window with one or more explicitly scoped Human Test Backlog items
- **Hotfix lane**: a front-of-queue implementation lane created when the operator finds a local defect that should be repaired before broader progress continues

Use **operator** for the human participant in this loop. Reserve **user** for product-facing language.

---

## Core loop

1. A Human Test Backlog item becomes `Ready` after preflight gates pass.
2. The agent creates or selects an instruction packet for the operator.
3. The operator performs the task and records results and comments.
4. The Backlog Coordinator triages the result.
5. The item is either closed, routed to Agent Backlog, routed to a hotfix lane, routed to Hardware Queue, or escalated.
6. Completed agent work returns to Human Test Backlog when human retest is required.
7. The next ready human item is issued.

Canonical loop:

`Human Test Backlog -> Agent instruction -> Operator result + comment -> Coordinator triage -> close / hotfix / agent backlog / escalation -> retest if needed`

---

## Required artifacts

Every operator session should have these artifacts before execution begins:

1. A named Human Test Backlog item
2. A session or instruction file
3. Expected success criteria
4. Stop criteria and rollback path
5. Findings or evidence capture location
6. Named coordinator review path

Recommended fields in an instruction packet:

```text
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

---

## Session states

| State | Meaning | Owner |
|-------|---------|-------|
| Planned | Listed, not yet ready | Coordinator |
| Ready | Preflight gates passed; operator can be instructed | Agent + Coordinator |
| In Operator Session | Human execution in progress | Operator |
| Awaiting Coordinator Review | Result captured and waiting for triage | Coordinator |
| Hotfix | A front-of-queue local repair is required | Agent |
| Waiting On Agent Backlog | Implementation or docs work is required before retest | Agent |
| Waiting On Hardware Queue | Environment or hardware work is required | Coordinator + hardware lane |
| Closed | Item completed with acceptable evidence | Coordinator |
| Escalated | Human decision or architectural redirection required | Coordinator + user |

---

## Coordinator decisions

The Backlog Coordinator must make one of five outcomes after each session item:

1. **Close**: result met expectations and evidence is sufficient.
2. **Hotfix**: a narrow local defect should be repaired before broader progress continues.
3. **Agent Backlog**: implementation, harness, or docs work is required, but not urgent enough to preempt the active queue.
4. **Hardware Queue**: physical, topology, device, or environment work is required.
5. **Escalate**: operator feedback implies a product-direction, policy, or architecture decision that should not be resolved inside the current slice.

---

## Human comments handling

Human comments should be split into two classes:

1. **Immediate-task comments**: comments directly about the current item, its pass/fail status, or a local defect. These stay with the session item.
2. **Meta comments**: broader architecture, product-direction, or planning feedback. These go to the Backlog Coordinator, not directly into the active slice.

Meta comments may create:

- an Agent Backlog item
- an ADR candidate
- a research note candidate
- a queue reprioritization decision

---

## Hotfix lane rule

If an operator session finds a defect that blocks the next meaningful human step, the coordinator may push a hotfix to the front of Agent Backlog.

Hotfixes should be:

1. small
2. bounded to the observed defect
3. validated before the same operator item is retried

---

## Evidence rules

An operator item is not complete unless the result includes enough evidence to support the decision.

Examples:

- screenshot or shell output
- named receipt or findings packet
- route or command used
- observed result
- comment on whether the result is canonical, proposed, rehearsal, or blocked

The coordinator should reject unsupported labels such as `verified`, `security`, or `ready` unless backed by named evidence.

---

## Relationship to other docs

- `docs/backlogs/HUMAN-TEST-BACKLOG.md` owns the ordered human queue.
- `docs/operator-sessions/SESSION-LOG.md` records completed or blocked sessions.
- `docs/operator-sessions/INSTRUCTION-TEMPLATE.md` is the default instruction-packet scaffold.
- `docs/adr/ADR-029-human-operator-session-orchestration-and-evidence-flow.md` is the governing decision record.
- `docs/orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md` is the current UX-planning packet that seeds the first operator-session backlog.