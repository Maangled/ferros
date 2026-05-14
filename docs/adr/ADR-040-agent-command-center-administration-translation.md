# ADR-040 - Agent Command Center administration translation reservation

**Status:** Draft
**Date:** 2026-05-13
**Stream:** S3 / S5 / S8 / Cross-cutting
**Deciders:** Maangled / FERROS stream coordination
**Domain tags:** architecture / UX doctrine / governance / policy / cross-cutting
**Primary evidence basis:** Operational proof + implementation proof required before acceptance

_See [ADR-022-decision-program-governance.md](./ADR-022-decision-program-governance.md), [_INDEX.md](./_INDEX.md), and [../../DOCTRINE.md](../../DOCTRINE.md). This reservation depends on [ADR-039-operator-admin-agent-control-plane.md](./ADR-039-operator-admin-agent-control-plane.md) and will remain draft until the Agent Command Center has implementation evidence beyond the local bridge monitor._

---

## Context

ADR-039 turns the live bridge monitor into the near-term Operator Administration control plane. That monitor is intentionally local, single-operator, and bridge-scoped. The Agent Command Center needs a later decision that translates the same agent lifecycle, escalation, packet chat, directory, and status-tracking contracts into a durable ACC surface with stronger permissions, shared queues, audit trails, and service contracts.

---

## Decision

**Reserve ADR-040 for the Agent Command Center translation of the Operator Administration control plane; this record does not accept the ACC implementation, but fixes the scope and proof bar for the future decision.**

The future ACC ADR must decide how ADR-039's local monitor concepts become a multi-operator, persistent Administration surface without losing the contract boundaries proven locally.

---

## Translation Scope

The future ACC decision must cover at least these topics:

1. Multi-operator Administration queues, including ownership, assignment, handoff, and stale-attention rules.
2. Permissioned control of background agents, warrant-operated agents, stop/resume actions, and special-agent category expansion.
3. Durable packet lifecycle chat, including thread identity, query behavior, event replay, and audit retention.
4. Agent Directory projection from a hierarchy-backed source tree while preserving the generated `.github/agents/*.agent.md` discovery surface if VS Code/Copilot still requires it.
5. Running Services separation from Agent Directory, including host/process state, bridge state, shell state, and service logs.
6. Real status-tracking fields for work orders, escalations, agent cycles, category health, and stale/failure detection.
7. Four-corner ACC layout mapping for Administration, Agent Directory, Running Services, and Packet Lifecycle Chat.
8. Archive, deny, console, and quick-action placement, including whether each surface is a dock, drawer, edge rail, or command palette entry.
9. Runway mindmap projection for lane health, packet flow, and next-action visibility.
10. Migration path from bridge-local monitor endpoints to hardened ACC service endpoints.

---

## Acceptance Bar

ADR-040 must not be promoted beyond Draft until all of the following evidence exists:

- A working ACC or ACC-incubation surface that exercises the four main surfaces from ADR-039.
- A machine-readable status contract replacing provisional UI progress claims.
- A packet lifecycle chat prototype that can focus, query, or replay lifecycle threads without creating duplicate user chats.
- A permission and audit model for stop, resume, archive, deny, escalation, and special-agent actions.
- A migration note explaining which monitor endpoints remain, which are replaced, and how compatibility is preserved.

---

## Rationale

Reserving this ADR keeps the local monitor from silently becoming the final ACC architecture while also preventing the ACC work from becoming a vague rewrite. ADR-039 can continue proving the loop locally; ADR-040 owns the later translation into a durable command center.

### Options considered

| Option | Summary | Reason not chosen (if rejected) |
|--------|---------|---------------------------------|
| Reserve ADR-040 for ACC translation (chosen) | Keep a named ADR shell and acceptance bar for ACC work while ADR-039 continues local monitor implementation | - |
| Put all ACC requirements into ADR-039 | Treat local monitor and ACC as one decision | Rejected because local bridge proof and multi-operator ACC proof have different risk and evidence bars |
| Leave ACC only in the roadmap | Avoid adding an ADR until implementation starts | Rejected because the operator asked to reserve the ACC ADR and the planning scope is now specific enough to name |

---

## Consequences

**Positive:**
- ACC work has a stable ADR number and proof bar.
- The bridge monitor can keep moving without claiming to be the finished command center.
- Future contributors can find the ACC translation scope directly from the ADR index.

**Negative / trade-offs:**
- ADR-040 is intentionally incomplete until implementation evidence exists.
- Some requirements are duplicated at a high level between ADR-039 and ADR-040 to keep the handoff visible.

---

## Compliance

- Do not cite ADR-040 as accepted ACC architecture while its status remains Draft.
- Do not weaken ADR-039's local monitor contracts during ACC translation.
- Do not collapse Agent Directory, Running Services, Administration, and Packet Lifecycle Chat into one undifferentiated roster.
- Do not approve ACC shared control until permission and audit semantics exist for protected actions.

---

## Implementation Evidence

- Local monitor source proving the current bridge surface: [../../tools/acc-bridge/monitor.html](../../tools/acc-bridge/monitor.html)
- Local monitor backend proving current state/event endpoints: [../../crates/ferros-node/src/lib.rs](../../crates/ferros-node/src/lib.rs)
- ACC incubation surfaces: [../agent-command-center.html](../agent-command-center.html), [../../site/agent-center-shell.html](../../site/agent-center-shell.html)
- Current custom-agent discovery source: [../../.github/agents/ROLLOUT-MANIFEST.md](../../.github/agents/ROLLOUT-MANIFEST.md)

---

## Deferred Scope or Open Research

- Deferred: exact ACC service schema, until monitor status packets and lifecycle chat fields are implemented.
- Deferred: multi-operator conflict resolution and permission tiers.
- Deferred: final hierarchy-to-`.github/agents` mirror generator design.
- Deferred: production retention policy for packet chat, console logs, archive, and deny events.

---

## References

- [ADR-009-four-corner-docking-layout.md](./ADR-009-four-corner-docking-layout.md)
- [ADR-010-cards-and-decks-nomenclature.md](./ADR-010-cards-and-decks-nomenclature.md)
- [ADR-029-human-operator-session-orchestration-and-evidence-flow.md](./ADR-029-human-operator-session-orchestration-and-evidence-flow.md)
- [ADR-037-agent-architect-governance-and-routing-tokens.md](./ADR-037-agent-architect-governance-and-routing-tokens.md)
- [ADR-038-extended-routing-token-schema-for-architect-families.md](./ADR-038-extended-routing-token-schema-for-architect-families.md)
- [ADR-039-operator-admin-agent-control-plane.md](./ADR-039-operator-admin-agent-control-plane.md)