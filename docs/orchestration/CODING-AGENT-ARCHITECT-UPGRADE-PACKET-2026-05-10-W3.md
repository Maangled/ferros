# Coding Agent Architect Upgrade Packet 2026-05-10 W3

Status: Active
Date: 2026-05-10
Authority: docs/orchestration/AUTHORITY-MAP.md

## Packet identity

- packet_id: FRS-coding-20260510-C1-W3
- packet_type: architect-family continuation
- target_agent: FERROS Coding Agent Architect
- parent_packet_id: FRS-coding-20260510-C1-W2
- objective: Candidate -> Research-only evidence packet 2 of 3

## Route token (v2)

```yaml
route_token:
  token_version: "v2"
  issued_by: "FERROS Prompt Architect Agent"
  target_stream: null
  target_family: "coding"
  run_profile: "architect-review"
  run_id: "FRS-coding-20260510-C1-W3"
  parent_run_id: "FRS-coding-20260510-C1-W2"
  recursion_depth: 1
  issued_at: "2026-05-10"
  expiry_cycle: "C1"
  posture: "recursive-lane-system"
  track: "system"
  ttl_ms: 300000
```

## Continuity baton

```yaml
continuity_baton:
  previous_owner: "FERROS Agent"
  next_owner: "FERROS Coding Agent Architect"
  next_action: "execute-review"
  evidence_goal: "Deliver bounded recommendation packet proving candidate-to-research-only readiness (2/3)"
```

## Scope and non-ownership

In scope:
- coding-family registry updates and architect routing policy deltas
- packet schema and field strictness for continuation compatibility
- queue-track alignment checks, escalation gates, and evidence contract hardening

Out of scope:
- direct Core/SubCore feature implementation
- business-family routing changes
- runtime or hardware stream execution
- undocumented authority overrides

## Lane plan (execution-ready)

L1 - Coding registry and packet-template delta
- Define update/upgrade lifecycle field strictness and promotion-boundary compatibility.

L2 - Routing recursion and traceability enforcement
- Enforce parent/child traceability (`parent_run_id`, `recursion_depth`, `ttl_ms`) and recursion-limit guards.

L3 - Validation, evidence contract, and queue continuity
- Tighten validation/evidence requirements and verify queue-surface continuity with system-track ownership.

## Anti-narrowing lane seeds

- Seed A: Registry promotion and demotion rules for coding specialists and architect paths
- Seed B: Packet validator interoperability and malformed-response recovery coupling
- Seed C: Queue-track boundary checks across code/system to prevent routing drift
- Seed D: Continuation packet minimal-delta policy for stable run lineage
- Seed E: Evidence taxonomy upgrades for audit-ready packet closure

## First-wave work order

1. Instantiate run context and bind `parent_run_id`, `recursion_depth`, `ttl_ms`, and `expiry_cycle`.
2. Generate update/upgrade packet draft under `architect-review` profile.
3. Run strict field validation and route-token normalization checks.
4. Execute anti-narrowing seed sanity check across L1/L2/L3.
5. Emit evidence bundle and queue-surface impact note.
6. If all gates pass, start execution under FERROS Coding Agent Architect; else escalate blocker plus remediation delta.

## Validation and escalation gates

- Require v2 route-token mutual exclusivity (`target_family` non-null, `target_stream` null).
- Require route-token echo before work starts; mismatch blocks execution.
- Require authority-ack path on authority mismatch before new lane expansion.
- Require stop/escalation criteria to name blocker, owning role, and remediation delta.

## Evidence bundle index

- E1: Route-token validation result and normalization report
- E2: Scope and non-claim conformance check
- E3: Lane deltas for L1/L2/L3 with anti-narrowing seed coverage map
- E4: Queue-surface continuity and track-boundary impact note
- E5: First-wave gate decision (`continue`, `stop-clean`, or `stop-escalate`)

## Non-claims

- No gate closure claim
- No hardware proof claim
- No Home Assistant proof claim
- No runtime execution claim
- No schema-freeze mutation claim

## Delivery note

This packet is queued as `SYSTEM-2026-05-10-02` in docs/orchestration/SYSTEM-QUEUE.md. Execution start is recorded in docs/orchestration/WAVE-RUN-LOG.md.
