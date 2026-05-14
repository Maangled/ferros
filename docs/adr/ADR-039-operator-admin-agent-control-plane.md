# ADR-039 - Operator administration agent control plane

**Status:** Draft
**Date:** 2026-05-12
**Last updated:** 2026-05-13
**Stream:** S3 / S5 / S8 / Cross-cutting
**Deciders:** Maangled / FERROS stream coordination
**Domain tags:** architecture / UX doctrine / governance / policy / cross-cutting
**Primary evidence basis:** Operational proof + formal or analytical proof

_See [ADR-022-decision-program-governance.md](./ADR-022-decision-program-governance.md), [_INDEX.md](./_INDEX.md), and [../../DOCTRINE.md](../../DOCTRINE.md). This record builds on [ADR-009-four-corner-docking-layout.md](./ADR-009-four-corner-docking-layout.md), [ADR-010-cards-and-decks-nomenclature.md](./ADR-010-cards-and-decks-nomenclature.md), [ADR-029-human-operator-session-orchestration-and-evidence-flow.md](./ADR-029-human-operator-session-orchestration-and-evidence-flow.md), [ADR-037-agent-architect-governance-and-routing-tokens.md](./ADR-037-agent-architect-governance-and-routing-tokens.md), and [ADR-038-extended-routing-token-schema-for-architect-families.md](./ADR-038-extended-routing-token-schema-for-architect-families.md)._

---

## Context

FERROS is beginning to operate agent loops as a live local control system rather than a static set of custom-agent prompts. The current bridge monitor can show open chats, running loops, a packet lifecycle log, a runtime agent roster, a deny log, and archive behavior, but the surface still blurs several different concepts: Copilot custom-agent definitions, demo/runtime service status, user-visible chat sessions, background work orders, and operator attention queues. The intended loop is recursive and escalation-aware: FERROS routes to Coding or Business, Coding routes Core/SubCore work orders, Core/SubCore return reports to Coding, Coding escalates unresolved issues to FERROS Agent, and FERROS Agent either resolves the issue back into Coding/Core/SubCore or asks the operator for clarification through an Administration surface.

---

## Decision

**FERROS will treat the local monitor as the near-term Operator Administration control plane for agent lifecycle visibility, escalation triage, packet lifecycle inspection, and guarded chat creation, while preserving `.github/agents/*.agent.md` as the current VS Code/Copilot custom-agent discovery source.**

The Administration surface replaces the older Operator Backlog label. It owns operator attention, stalled or stopped loops, escalation explanations, warrant-operated or special agents, and future category-expansion loops such as FERROS Agent <-> Agent Architect.

The monitor must separate four surfaces that are currently adjacent:

1. **Agent Directory:** catalog of custom-agent definitions, hierarchy, routing families, rollout status, and definition metadata. This is a source-of-truth directory view, not a process table.
2. **Running Services:** live local services, bridge/shell processes, runtime/demo state, health, logs, and lifecycle controls. This is a runtime view, not the agent-definition catalog.
3. **Administration:** escalations, stoppages, operator-needed chats, warrant/special-agent controls, and quick actions that require human authority.
4. **Packet Lifecycle Chat:** human-readable lifecycle thread for packets, work orders, reports, escalations, denials, and archival events. This is queryable operational memory, not a general-purpose chat launcher.

---

## Required Operating Model

### Special-agent categories

The monitor should present special-agent categories as compact operator controls, not as full text panels. The initial categories are:

| Category | Attached agents | Operator meaning |
|----------|-----------------|------------------|
| Agent Architect | FERROS Agent Architect, Coding Agent Architect, Business Agent Architect | Creates and governs agent families |
| Software Architect | Coding Agent, Core Agent, SubCore Agent, coding specialists | Routes code work orders and reports |
| Business Manager | Business Agent, Operations, Sales, Finance, HR | Routes business work orders and reports |
| Administration | FERROS Agent, escalation handlers, warrant/special agents | Clears operator attention, stoppages, and special controls |

### Monitor layout and attention model

The bridge monitor should adopt the ADR-009 four-corner contract before it is translated into ACC:

| Zone | Primary surface | Placement rule |
|------|-----------------|----------------|
| Top-left | Agent Directory | Shows the custom-agent hierarchy and definition cards; collapsed state shows directory count and attention summary |
| Top-right | Running Services and Console | Shows bridge/shell/runtime health, process controls, and a console drawer for local service logs |
| Bottom-left | Administration and Quick Actions | Shows operator attention, escalation cards, stop/resume/archive actions, and command buttons bound to selected work |
| Bottom-right | Packet Lifecycle Chat | Shows lifecycle threads, explanations, event replay, and focused packet questions |
| Center viewport | Runway mindmap | Shows lane/loop/work-order flow as a mindmap or graph, with selected node details routed to the four corners |
| Top edge | Category carousel | Shows special-agent categories as compact title-and-icon controls centered on the highest-attention category |
| Left edge | Archive drawer | Opens archived chats, closed work orders, and historical lifecycle packets without crowding active Administration |
| Right edge | Deny log drawer | Opens denied actions, protected-action failures, and policy/audit explanations near service and console context |

The category carousel is attention-ranked. The centered card is the category with the highest weighted attention score; Administration wins ties until another category has strictly higher urgency. Adjacent carousel positions should prefer categories with the next-highest percentage of stalled, waiting, stopped, or escalating agents. Compact running-loop buttons must show only the title and quick-view icons; descriptions belong on expanded cards or the lifecycle chat.

The default attention score is:

```text
attention_score = attention_count * 5 + stopped_count * 4 + escalating_count * 3 + stale_count * 2 + running_count * 0.25
```

This score is a UI ordering aid, not governance authority. Protected actions still require explicit packet, permission, or operator checks.

### Status model

Visible status must be data-backed. The minimum shared status language is:

| Status | Meaning | Default visual treatment |
|--------|---------|--------------------------|
| `running` | Agent or category is actively processing a valid work order | Green |
| `escalating` | Agent or category emitted an escalation or is waiting on upstream resolution | Yellow |
| `attention` | Escalation reached Administration or user-facing clarification is needed | Red |
| `stopped` | Agent stopped before normal work-order/report closure or failed local lifecycle | Red |
| `idle` / `unknown` | No active work or insufficient machine-readable state | Neutral |

Until packet events carry full mid-session progress, the UI may show provisional progress bars and short descriptions, but it must not claim precise progress.

Minimum machine-readable status fields are required before the UI can claim real progress:

| Field | Meaning |
|-------|---------|
| `agentId` | Stable agent identity from the directory or runtime registry |
| `cycleId` | Stable background-agent cycle identity |
| `workOrderId` | Work order identity when the cycle emits or consumes work |
| `escalationId` | Escalation identity when attention is required |
| `status` | One of the shared status values above |
| `statusReason` | Short machine-readable reason code |
| `statusDetail` | Human-readable explanation suitable for the card back or lifecycle chat |
| `startedAt` / `updatedAt` / `staleAfter` | Time fields used for stale and stopped detection |
| `progress` | Optional structured progress, omitted when unknown rather than faked |
| `sourceAgentId` / `targetAgentId` | Routing identities for handoff, report, or escalation |

### Background-agent lifecycle, work-order, and escalation contract

Background agents should not simply drift, stop silently, or create unbounded chat sessions. Every background-agent cycle must have a lifecycle record and one terminal outcome.

The minimum lifecycle states are:

| State | Meaning | Required next step |
|-------|---------|--------------------|
| `queued` | Work is known but not yet active | Start, reject, or supersede |
| `running` | Agent is actively processing a valid work order | Emit report, work order, escalation, or heartbeat |
| `waiting` | Agent is blocked on an upstream report, permission, or operator input | Link blocker or escalation |
| `reported` | Agent returned a report to its parent or owning lane | Parent acknowledges or opens follow-up work |
| `work_order_emitted` | Agent emitted valid downstream work | Target accepts or denies |
| `escalation_emitted` | Agent emitted valid escalation to FERROS Agent or Administration | FERROS/Admin resolves, reroutes, or asks user |
| `denied` | Action or packet was rejected by policy, malformed contract, or permission boundary | Deny log entry and lifecycle explanation |
| `archived` | Cycle is closed and no longer active | Archive drawer entry and retained lifecycle thread |
| `stopped` | Cycle ended without valid report, work order, escalation, denial, or archive | Administration attention |

A background-agent cycle is considered complete only after one of these terminal outcomes:

1. It emits a **work order** to another agent/category.
2. It emits an **escalation** to FERROS Agent or Administration.
3. It returns a **report** to the parent cycle that accepts the closure.
4. It is explicitly **denied**, **archived**, or **stopped** with an Administration-visible reason.

Lane guidance, architecture uncertainty, permission uncertainty, or category-proliferation uncertainty is treated as an escalation until a later ADR narrows that language.

A valid work order must include `workOrderId`, `sourceAgentId`, `targetAgentId` or `targetCategory`, `intent`, `scope`, `constraints`, `expectedReturn`, `createdAt`, and `routingToken` when the target family requires one. A work order cannot rely on visible chat text as its only state.

A valid escalation must include `escalationId`, `sourceAgentId`, `ownerAgentId`, `reasonCode`, `operatorQuestion` or `resolutionRequest`, `blockedWorkOrderId` when applicable, `severity`, `createdAt`, and an intended return route. Escalations to the user pass through FERROS Agent or Administration, not direct Core/SubCore user chat creation.

### Chat creation rule

The monitor should not provide freeform "new user chat" creation. Operator chats should open from specific loop cards, Administration cards, lifecycle chat, or explicit FERROS Agent entrypoints. Repeated Open, Explain, or Chat actions for the same escalation should focus or update the latest related chat where possible instead of creating visible duplicates.

### Packet lifecycle chat design

The Packet Lifecycle Chat is the operator-readable conversation around packet state. It should behave like a focused thread over lifecycle events, not as a new freeform chat surface.

The first implementation should support:

| Capability | Requirement |
|------------|-------------|
| Thread identity | One lifecycle thread per packet, work order, escalation, or loop cycle; related threads may be grouped but remain individually addressable |
| Event timeline | Append-only rendering of queued, accepted, routed, reported, denied, escalated, archived, and stopped events |
| Focus behavior | Open, Explain, and Chat actions focus an existing lifecycle/user chat when one already exists for the same escalation or packet |
| Query behavior | Operator can ask what happened, why a packet is blocked, who owns it, what the next action is, and what evidence supports the status |
| Action binding | Quick actions launched from the chat must be bound to the selected packet, not global freeform commands |
| Audit visibility | Denials, protected-action failures, and stop reasons are visible in the lifecycle thread and mirrored into the deny log when applicable |
| ACC translation | Later ACC implementation can replay the same lifecycle thread for multiple operators without treating it as a personal chat transcript |

The lifecycle chat may summarize noisy service logs, but raw console output belongs in the Running Services console drawer.

### Agent Directory vs Running Services model

Agent Directory and Running Services must stay separate even when a directory agent has an active runtime process.

| Surface | Owns | Must not own |
|---------|------|--------------|
| Agent Directory | Agent definitions, category hierarchy, display labels, intended authority, source file path, rollout status, generated mirror status | Process liveness, port ownership, service logs, runtime stop/start authority |
| Running Services | Bridge, shell, coordinator, monitor backend, local runtime/demo services, process state, ports, logs, health checks | Custom-agent definition truth, category governance, prompt source ownership |
| Administration | Human attention, escalations, stops, protected actions, warrant/special-agent controls, quick actions | Raw service logs as the primary view, agent source file editing |
| Packet Lifecycle Chat | Lifecycle explanation, packet questions, event replay, focused action context | Agent discovery, service management, unbounded new chat creation |

This split avoids the current roster ambiguity where demo/runtime agents, `.agent.md` definitions, and open chat sessions can look like one thing.

### Agent definitions and hierarchy

Workspace custom agents remain in `.github/agents/*.agent.md` for now because that is the current VS Code/Copilot discovery surface. FERROS may add hierarchy metadata, generated manifests, or a future mirrored `agents/` tree, but the canonical discoverable definitions must not be moved until subfolder discovery or mirror generation has implementation proof.

The proposed source hierarchy outside `.github` is:

```text
agents/
  README.md
  manifest.json
  source/
    ferros/
      ferros-agent.agent.md
      architects/
        ferros-agent-architect.agent.md
        ferros-prompt-architect.agent.md
        ferros-orchestration-architect.agent.md
        ferros-documentation-architect.agent.md
      officers/
        ferros-backup-officer.agent.md
        ferros-audit-recovery-officer.agent.md
    coding/
      ferros-coding-agent.agent.md
      architects/
        ferros-coding-agent-architect.agent.md
        ferros-core-lane-architect.agent.md
        ferros-subcore-lane-architect.agent.md
      execution/
        ferros-core.agent.md
        ferros-subcore.agent.md
      incubation/
        ferros-coding-continuity.agent.md
        ferros-coding-packet-validator.agent.md
        ferros-coding-malformed-response.agent.md
    business/
      ferros-business-agent.agent.md
      architects/
        ferros-business-agent-architect.agent.md
    archive/
      retired-or-superseded-agent-specs/
  generated/
    github-agents/
      ferros-agent.agent.md
      ...
```

The future generator should read `agents/manifest.json`, validate category metadata, and write the flat `.github/agents/*.agent.md` mirror for VS Code/Copilot discovery. Until that generator exists and is proven, `.github/agents` remains canonical and the hierarchy remains a proposal.

---

## Expansion Math

Agent-family generation can grow exponentially if agent architects create categories that can create further categories.

Let:

- `b` = average number of new child categories created by one architect category
- `d` = permitted category-generation depth
- `a` = average number of execution/admin/tool agents attached per category
- `w` = average number of active work orders each agent may emit before a report or escalation is required

Then category count grows approximately as:

```text
categories(d) = (b^(d + 1) - 1) / (b - 1), when b > 1
agents(d) ~= categories(d) * a
max_open_work_orders(d) ~= agents(d) * w
```

Generation `i` contributes `b^i` categories. The danger is not only total agent count; it is also live work-order fan-out if each generated agent can open more work before reporting. FERROS therefore needs category-depth caps, work-order fan-out caps, stale detection, and visible escalation ratios before recursive agent-creation authority can run unattended.

Examples:

| Branch factor `b` | Depth `d` | Categories | Agents at `a = 6` | Open work orders at `w = 2` |
|-------------------|-----------|------------|-------------------|-----------------------------|
| 2 | 2 | 7 | 42 | 84 |
| 2 | 3 | 15 | 90 | 180 |
| 3 | 3 | 40 | 240 | 480 |
| 4 | 3 | 85 | 510 | 1,020 |

This justifies strict category-depth caps, promotion gates, work-order fan-out limits, visible stall percentages, and Administration-centered controls before FERROS allows recursive agent-creation authority to operate unattended.

---

## Rationale

The chosen model makes the current bridge monitor useful immediately while keeping the more ambitious Agent Command Center migration honest.

### Options considered

| Option | Summary | Reason not chosen (if rejected) |
|--------|---------|--------------------------------|
| Operator Administration control plane (chosen) | Evolve the current monitor into a local admin surface with explicit agent directory, services, escalation, and lifecycle chat boundaries | - |
| Move all agent definitions into a new hierarchy immediately | Align file paths with conceptual hierarchy now | Rejected for now because VS Code/Copilot custom-agent discovery currently expects `.github/agents/*.agent.md`; a move needs proof or generated mirroring first |
| Build the full ACC first | Skip monitor hardening and go directly to the multi-operator surface | Rejected because the monitor is already live, local, and proving the interaction model under BRIDGE-WORKAROUND scope |
| Keep using freeform chat creation and tables | Minimal UI churn | Rejected because it creates duplicate sessions, hides escalation ownership, and does not scale to first-principles agent expansion |

---

## Consequences

**Positive:**
- Operator attention becomes a first-class Administration lane rather than an overloaded chat dropdown.
- The monitor can scale from a handful of loops to generated agent families without hiding the difference between definitions, services, and chats.
- Agent proliferation gets a mathematical brake before it becomes operationally opaque.
- The ACC migration gains a concrete local prototype instead of starting from abstract UX goals.
- Packet lifecycle information becomes queryable by a human-facing chat instead of remaining only a passive log.

**Negative / trade-offs:**
- The monitor state contract must expand before the UI can honestly show mid-session progress.
- Agent definitions remain physically flat in `.github/agents` even though the user-facing directory is hierarchical.
- Session dedupe and focus behavior must compensate for current SDK session mechanics.
- Administration can become too powerful if warrant/special-agent controls are added before permission language and audit trails mature.

---

## Compliance

- Do not move workspace custom-agent definitions out of `.github/agents/*.agent.md` until discovery or generated mirroring is proven.
- Do not expose freeform monitor chat creation as the normal operator path; chats must come from a loop, escalation, lifecycle, or FERROS entrypoint.
- Do not show precise progress percentages unless packet or runtime state carries machine-readable progress fields.
- Do not collapse custom-agent definitions, running services, and open chats into one roster.
- Do not show background-agent cycles as complete unless they have a report, work order, escalation, denial, archive, or stopped reason in the lifecycle record.
- Do not create hierarchy-backed source-of-truth agent folders until the generated `.github/agents` mirror or VS Code/Copilot discovery behavior is proven.
- Revisit this ADR if the Copilot SDK supports shared multi-agent sessions, durable cross-agent conversations, or reliable custom-agent hierarchy discovery.
- Revisit this ADR before enabling unattended recursive category creation beyond the approved generation depth.

---

## Implementation Evidence

- Current bridge monitor: [../../tools/acc-bridge/monitor.html](../../tools/acc-bridge/monitor.html)
- Current monitor backend and runtime/demo agent roster: [../../crates/ferros-node/src/lib.rs](../../crates/ferros-node/src/lib.rs)
- Current custom-agent discovery source: [../../.github/agents/ROLLOUT-MANIFEST.md](../../.github/agents/ROLLOUT-MANIFEST.md)
- Current coordinator SDK wrapper: [../../coordinator/src/session-manager.ts](../../coordinator/src/session-manager.ts)
- Current ACC incubation surfaces: [../agent-command-center.html](../agent-command-center.html), [../../site/agent-center-shell.html](../../site/agent-center-shell.html)

---

## Deferred Scope or Open Research

- Deferred: full ACC multi-operator collaboration, permissioned shared administration, and stronger audit trails.
- Deferred: moving custom-agent definitions into a hierarchy-backed source tree with a generated flat `.github/agents` mirror.
- Deferred: final warrant-operated special-agent permission language.
- Deferred: live mid-session progress tracking until packets carry the required fields.
- Deferred: replacing local BRIDGE-WORKAROUND monitor endpoints with a hardened ACC service contract.
- Deferred: implementing the hierarchy-to-`.github/agents` mirror generator.
- Deferred: ACC translation, now reserved by [ADR-040-agent-command-center-administration-translation.md](./ADR-040-agent-command-center-administration-translation.md).
- Related research note: [./_RESEARCH-NOTES/RN-2026-04-acc-card-deck-projection.md](./_RESEARCH-NOTES/RN-2026-04-acc-card-deck-projection.md)

---

## References

- [ADR-009-four-corner-docking-layout.md](./ADR-009-four-corner-docking-layout.md)
- [ADR-010-cards-and-decks-nomenclature.md](./ADR-010-cards-and-decks-nomenclature.md)
- [ADR-029-human-operator-session-orchestration-and-evidence-flow.md](./ADR-029-human-operator-session-orchestration-and-evidence-flow.md)
- [ADR-037-agent-architect-governance-and-routing-tokens.md](./ADR-037-agent-architect-governance-and-routing-tokens.md)
- [ADR-038-extended-routing-token-schema-for-architect-families.md](./ADR-038-extended-routing-token-schema-for-architect-families.md)
- [ADR-040-agent-command-center-administration-translation.md](./ADR-040-agent-command-center-administration-translation.md)
- [../orchestration/ORCHESTRATION-AGENTS.md](../orchestration/ORCHESTRATION-AGENTS.md)
- [../orchestration/ORCHESTRATOR-COORDINATOR-ARCHITECTURE.md](../orchestration/ORCHESTRATOR-COORDINATOR-ARCHITECTURE.md)
