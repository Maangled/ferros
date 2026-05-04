# ADR-029 - Human-operator session orchestration and evidence flow

**Status:** Accepted  
**Date:** 2026-05-04  
**Stream:** Cross-cutting / S5 / S7 / S8  
**Deciders:** Maangled  
**Domain tags:** governance / policy / UX doctrine / operational proof / cross-cutting  
**Primary evidence basis:** Operational proof

_See [ADR-022-decision-program-governance.md](./ADR-022-decision-program-governance.md), [_INDEX.md](./_INDEX.md), and [../../DOCTRINE.md](../../DOCTRINE.md). Cross-reference [ADR-023-onramp-policy.md](./ADR-023-onramp-policy.md) for the current proposed-material invariant and [../orchestration/OPERATOR-SESSION-PATTERN.md](../orchestration/OPERATOR-SESSION-PATTERN.md) for the operating loop this ADR formalizes._

---

## Context

FERROS already uses operator-attended sessions for real-hardware work, launch evidence, and local shell validation, but the repo does not yet treat operator sessions as a first-class orchestration lane. The current behavior is spread across `HARDWARE-QUEUE.md`, `WAVE-RUN-LOG.md`, gate findings packets, and chat-driven instructions. The next UX lane needs a repeatable loop where human tests can be queued, issued, observed, triaged, and re-queued without mixing implementation work, hotfixes, and meta comments into one undifferentiated stream.

---

## Decision

**FERROS will treat human-operator sessions as a first-class orchestration pattern with a dedicated Human Test Backlog, explicit agent-issued instructions, structured result capture, coordinator triage, and named evidence requirements before an item can be closed.**

Human comments that are not directly about the active task will be routed to the coordinator instead of being allowed to silently rewrite the active slice.

---

## Rationale

This option turns existing practice into a reproducible operating model instead of relying on ad hoc chat memory.

### Options considered

| Option | Summary | Reason not chosen (if rejected) |
|--------|---------|--------------------------------|
| Option A (chosen) | Add a dedicated operator-session pattern with a Human Test Backlog, coordinator triage, and evidence rules | - |
| Option B | Keep operator sessions as informal chat-driven steps attached to hardware or UX work as needed | Rejected because it makes prioritization, retesting, and evidence closure too dependent on session memory and makes hotfix routing inconsistent |
| Option C | Fold operator sessions entirely into `HARDWARE-QUEUE.md` | Rejected because many upcoming operator loops are UX-facing rather than hardware-only and need their own queue discipline |

---

## Consequences

**Positive:**
- Operator work becomes queueable and auditable.
- Hotfixes can be promoted explicitly instead of interrupting the active queue informally.
- Human comments can be separated into immediate-task feedback and broader architectural direction.
- S5, S7, and later module lanes can share one human-testing pattern.

**Negative / trade-offs:**
- The repo gains more orchestration surfaces to maintain.
- Some sessions that felt lightweight in chat will now require named instruction and evidence packets.
- Coordinator review becomes a real role that must stay disciplined to avoid over-bureaucratizing small tests.

---

## Compliance

- Do not mark a human-test item closed without a named session or findings reference.
- Do not route meta comments directly into the active implementation slice; send them through the coordinator.
- Do not treat operator-visible labels such as `verified`, `security`, or `ready` as valid unless backed by named evidence.
- Revisit this ADR if FERROS later adds remote, multi-operator, or scheduled unattended operator flows that exceed the current local-session model.

---

## Implementation Evidence

- Operational precedent already exists in [../orchestration/HARDWARE-QUEUE.md](../orchestration/HARDWARE-QUEUE.md), [../orchestration/WAVE-RUN-LOG.md](../orchestration/WAVE-RUN-LOG.md), and [../gates/D1.md](../gates/D1.md).
- This ADR is operationalized by [../orchestration/OPERATOR-SESSION-PATTERN.md](../orchestration/OPERATOR-SESSION-PATTERN.md), [../backlogs/HUMAN-TEST-BACKLOG.md](../backlogs/HUMAN-TEST-BACKLOG.md), [../operator-sessions/SESSION-LOG.md](../operator-sessions/SESSION-LOG.md), and [../operator-sessions/INSTRUCTION-TEMPLATE.md](../operator-sessions/INSTRUCTION-TEMPLATE.md).
- The immediate driver is the interconnected UX program documented in [../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md](../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md).

---

## Deferred Scope or Open Research

- Deferred: remote operator sessions and multi-operator coordination
- Deferred: automated parsing of operator findings into backlog entries
- Deferred: whether coordinator review becomes a dedicated agent mode or stays inside the current orchestration stack
- Related future topics: proposed material and receipts, evidence badges, tool-lane disclosures, home-topology lineage, and Arena non-evidentiary boundaries

---

## References

- [ADR-022-decision-program-governance.md](./ADR-022-decision-program-governance.md)
- [ADR-023-onramp-policy.md](./ADR-023-onramp-policy.md)
- [../orchestration/LOCAL-DRIVER.md](../orchestration/LOCAL-DRIVER.md)
- [../orchestration/HARDWARE-QUEUE.md](../orchestration/HARDWARE-QUEUE.md)
- [../orchestration/WAVE-RUN-LOG.md](../orchestration/WAVE-RUN-LOG.md)
- [../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md](../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md)
- [../operator-sessions/INSTRUCTION-TEMPLATE.md](../operator-sessions/INSTRUCTION-TEMPLATE.md)