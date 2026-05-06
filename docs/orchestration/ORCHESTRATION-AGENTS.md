# FERROS Orchestration Agents

> **Canonical authority.** This document owns the role map: which agents exist, what they decide, and when to invoke them.
> See [AUTHORITY-MAP.md](AUTHORITY-MAP.md) for the full document index.
> For lane policy and stop conditions, see [ORCHESTRATION-POLICY.md](ORCHESTRATION-POLICY.md).
> For the operating loop and mode selection, see [ORCHESTRATION-EXECUTION.md](ORCHESTRATION-EXECUTION.md).

---

## Review roles inside a wave

These roles run in sequence as the default authorization chain for every wave:

| Role | Agent | Responsibility |
|------|-------|---------------|
| **Lane Architect** | `ferros-lane-architect.agent.md` | Confirms anchors, decomposition, and whether a wave can be safely split. Plans up to 8 parallel lanes. |
| **Builder lane(s)** | Owning stream agent(s) | Implements only the declared slice. Does not freelance scope or sibling lanes. |
| **Validator** | `ferros-lane-validator.agent.md` | Runs targeted executable checks before launch and after landing on consumer-facing, launch-critical, or multi-file slices. |
| **Risk/claim rationalizer** | `ferros-risk-rationalizer.agent.md` or `ferros-claim-rationalizer.agent.md` | Verifies scope, gate language, hardware claims, transport claims, and privilege posture stayed inside wave constraints. |
| **Gatekeeper** | Inline self-review (primary orchestrator model) | Converts reviews into `continue`, `stop-clean`, or `stop-escalate`. Uses the structured block format. See ORCHESTRATION-POLICY.md §Gatekeeper decision enum. |
| **Orchestrator** | `ferros-orchestrator.agent.md` | Final go/no-go call for the next segment. Authorizes lane execution and receives integration results. |

The orchestrator remains the authorizing authority but delegates safety checks to better-scoped subagents instead of treating human re-entry as the normal brake.

---

## Gatekeeper model intent

The gatekeeper role is currently performed by the primary orchestrator model as an inline self-review step. This is a known limitation: the same model that authors the wave also reviews it.

The intent is to migrate the gatekeeper role to a dedicated small-tier / fast model when that becomes mechanically available. The structured gatekeeper block format in ORCHESTRATION-POLICY.md is the stable handoff contract for that migration — no redesign of the block schema or decision enum is required when the model swap occurs.

Until that migration, the inline self-review posture is acceptable for `size: S`, docs-only, non-gate-close waves. Gate-close waves, P0 waves, and frozen-schema-touching waves remain Interactive-only and are not subject to gatekeeper review.

---

## Stream agents

Each stream has an owning agent that implements work within that stream's seams:

| Stream | Agent | Primary seam |
|--------|-------|-------------|
| S1 Foundation | `ferros-s1-foundation.agent.md` | Core crates, no-std targets, workspace manifests |
| S2 Profile & Identity | `ferros-s2-profile.agent.md` | `ferros-profile` crate, grant semantics, identity schemas |
| S3 Agent Center | `ferros-s3-agent-center.agent.md` | `ferros-agents` crate, agent-center shell surface |
| S4 Runtime / OS Core | `ferros-s4-runtime.agent.md` | `ferros-runtime`, `ferros-core`, kernel and privilege boundary |
| S5 UX | `ferros-s5-ux.agent.md` | Shell surfaces, UX module construction, SharedModules |
| S6 Ecosystem Harvest | `ferros-s6-harvest.agent.md` | External integrations, harvest pipelines |
| S7 Hub | `ferros-s7-hub.agent.md` | `ferros-hub` crate, hub service layer |
| S8 Docs & Governance | `ferros-s8-docs.agent.md` | Documentation, ADRs, orchestration docs, governance |

---

## Specialist agents

These agents handle cross-stream or safety-critical coordination that no single stream agent owns:

| Agent | When to invoke |
|-------|---------------|
| **FERROS Recursion Controller Agent** | Before asking Lane Architect for a second planning pass on a lane. Required gate for all recursive lane planning. |
| **FERROS Lane Validator Agent** | Before launch when a lane changes consumer-facing, launch-critical, or multi-file slices. After landing the same. |
| **FERROS Log Triage Agent** | When a lane fails validation or implementation. Route the earliest concrete failure here before widening the fix. |
| **FERROS Trace Analyst Agent** | When the failure boundary remains ambiguous after Log Triage. Escalate only when Log Triage cannot resolve. |
| **FERROS Integration Reviewer Agent** | After all implementation lanes finish. Checks gate truth, contract alignment, and cross-stream coherence. |
| **FERROS Status Auditor Agent** | When STATUS.md needs verification against gate docs. Do not call during an active lane run. |
| **FERROS Gate Auditor Agent** | When a gate document's evidence requirements need verification. |
| **FERROS Contract Auditor Agent** | When consumer-facing contracts need verification against implementation. |

---

## Driver agent

| Agent | Role |
|-------|------|
| **FERROS Local Driver Agent** (`ferros-driver.agent.md`) | User-facing entry point. Reads the queue, selects the next ready item, marks it in progress, delegates to the orchestrator. |

---

## Invocation decision tree

```
User request
│
├─ Is it question-driven (answer is the outcome)?
│   └─ Yes → Interactive Mode. Answer directly.
│
├─ Does it touch P0, gate-close, solo, or frozen schema?
│   └─ Yes → Interactive Mode. Single wave only.
│
├─ Is it execution-directed (implement, run, proceed)?
│   └─ Yes → Batch Mode.
│       │
│       ├─ Invoke FERROS Local Driver Agent
│       ├─ Driver → FERROS Orchestrator Agent
│       ├─ Orchestrator → FERROS Lane Architect Agent (lane plan)
│       │   └─ Recursion candidate? → FERROS Recursion Controller Agent first
│       ├─ Orchestrator → FERROS Lane Validator Agent (pre-flight)
│       ├─ Orchestrator → Stream agent(s) (implementation)
│       ├─ If failure → FERROS Log Triage Agent → FERROS Trace Analyst Agent
│       ├─ Orchestrator → FERROS Lane Validator Agent (post-flight)
│       ├─ Orchestrator → FERROS Integration Reviewer Agent
│       └─ Gatekeeper inline review → continue / stop-clean / stop-escalate
│
└─ Mixed (question + execution)?
    └─ Answer briefly if cheap, then continue as Batch Mode.
```

---

## Delegation rules

- The orchestrator does not do the main implementation work itself. It coordinates.
- Do not mix multiple owning streams into a single delegated implementation lane unless the request is truly inseparable.
- Do not implement broad code changes directly when a stream agent owns the work.
- Do not claim a gate moved unless the repo evidence actually changed.
- Keep S2 as the default serial gate owner when identity or grant semantics are involved.
- Do not end an execution-oriented coordination reply with an options list. Close with a concise executive summary at the current handoff boundary.

---

## Output format for orchestration reports

Return a short coordination report with these sections:

1. `Gate impact`
2. `Parallel lanes launched`
3. `Integration findings`
4. `Remaining blockers`
5. `Next attack`

---

*Last updated: 2026-05-03*
