# ADR-037 - Agent architect governance and routing-token chain of command

**Status:** Draft
**Date:** 2026-05-06
**Stream:** S3 / S4 / S8 / Cross-cutting
**Deciders:** Maangled
**Domain tags:** architecture / governance / policy / security / cross-cutting
**Primary evidence basis:** Implementation proof

_See [ADR-022-decision-program-governance.md](./ADR-022-decision-program-governance.md), [ADR-025-dual-root-hardware-runway.md](./ADR-025-dual-root-hardware-runway.md), [ADR-029-human-operator-session-orchestration-and-evidence-flow.md](./ADR-029-human-operator-session-orchestration-and-evidence-flow.md), and [../../DOCTRINE.md](../../DOCTRINE.md)._ 

---

## Context

FERROS now has a near-term custom-agent rollout with explicit Core and SubCore execution lanes, a dedicated Agent Architect, and safety officers. The remaining risk is chain-of-command drift: users may paste a SubCore kickoff packet into Core (or vice versa), and agents may tunnel into the most recent seam when generating next-lane seeds. The repo already favors deny-by-default boundaries, bounded lanes, and audit-first posture, so the next governance step is to formalize how architecture packets are issued, validated, and rejected when routing is wrong.

---

## Decision

**FERROS will require Agent-Architect-issued routing tokens on Core/SubCore kickoff packets and require stream agents to reject packet execution when the token target does not match the stream.**

The Agent Architect is the policy authority for near-term agent expansion packets, but remains bounded by doctrine and explicit safety roles.

---

## Rationale

A packet token is a narrow, auditable mechanism that preserves chain of command without introducing broad runtime privilege. It reduces accidental cross-stream execution and makes misroutes explicit, reversible, and reportable. This fits FERROS doctrine better than implicit trust in free-form prompt text.

### Options considered

| Option | Summary | Reason not chosen (if rejected) |
|--------|---------|--------------------------------|
| Option A (chosen) | Add routing token fields to kickoff packets and enforce target-stream validation in Core/SubCore agents | - |
| Option B | Rely on naming conventions and manual review only | Rejected because it is error-prone and does not provide deterministic refusal behavior |
| Option C | Add one universal super-agent that can rewrite packet targets on the fly | Rejected because it increases authority concentration and weakens explicit boundaries |

---

## Consequences

**Positive:**
- Core/SubCore chain-of-command errors become explicit and machine-checkable.
- Agent Architect has a concrete governance contract for bounded expansion cycles.
- Misrouted packets fail closed and return corrective guidance instead of silently executing.

**Negative / trade-offs:**
- Kickoff packet format is slightly heavier.
- Agents must enforce and report token-check failures before execution.
- Some legacy packets will need translation until all active runs emit token fields.

---

## Compliance

The following must remain true while this ADR is active:

- Core and SubCore kickoff packets include a `route_token` block.
- Core agent executes only when `route_token.target_stream == "core"`.
- SubCore agent executes only when `route_token.target_stream == "subcore"`.
- On mismatch, the receiving stream agent must refuse execution and return a correction packet request.
- The Agent Architect must emit bounded architecture packets (1-3 changes) and avoid unbounded role sprawl.

Suggested token shape for kickoff packets:

```yaml
route_token:
  token_version: "v1"
  issued_by: "FERROS Agent Architect Agent"
  target_stream: "core|subcore"
  run_profile: "core-runtime|subcore-runtime|ux-surface"
  run_id: "FRS-<stream>-<YYYYMMDD>-C<N>-W<N>"
  issued_at: "YYYY-MM-DD"
  expiry_cycle: "C<N>"
```

---

## Implementation Evidence

- Near-term rollout manifest: [../../.github/agents/ROLLOUT-MANIFEST.md](../../.github/agents/ROLLOUT-MANIFEST.md)
- Agent architect charter: [../../.github/agents/ferros-agent-architect.agent.md](../../.github/agents/ferros-agent-architect.agent.md)
- Core stream agent: [../../.github/agents/ferros-core.agent.md](../../.github/agents/ferros-core.agent.md)
- SubCore stream agent: [../../.github/agents/ferros-subcore.agent.md](../../.github/agents/ferros-subcore.agent.md)
- Coordinator: [../../.github/agents/ferros-agent.agent.md](../../.github/agents/ferros-agent.agent.md)

---

## Deferred Scope or Open Research

- Cryptographic signing or tamper-evident token verification for routing tokens.
- ACC-native packet issuance UI and routing-token minting flow.
- Expanded agent families (philosopher, advocate, democracy, discipline) as read-only or bounded overlays.
- Multi-ledger reward system design (kept outside execution authority until dedicated evidence exists).

---

## References

- [ADR-022-decision-program-governance.md](./ADR-022-decision-program-governance.md)
- [ADR-025-dual-root-hardware-runway.md](./ADR-025-dual-root-hardware-runway.md)
- [ADR-029-human-operator-session-orchestration-and-evidence-flow.md](./ADR-029-human-operator-session-orchestration-and-evidence-flow.md)
- [ADR-035-governance-and-merit-signals-are-research-only.md](./ADR-035-governance-and-merit-signals-are-research-only.md)
