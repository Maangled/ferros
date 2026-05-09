# ADR-038 - Extended routing-token schema for architect-family roles

**Status:** Active  
**Date:** 2026-05-09  
**Stream:** S3 / S4 / S8 / Cross-cutting  
**Deciders:** FERROS Prompt Architect Agent  
**Domain tags:** architecture / governance / routing / security / cross-cutting  
**Supersedes:** ADR-037 §Suggested token shape (extends, does not replace)  
**Related:** ADR-037, AUTHORITY-INTERRUPTION.md, ARCHITECT-FAMILY-PROMOTION.md

---

## Context

ADR-037 established routing tokens for Core and SubCore streams. The current token schema assumes execution always targets one of two streams: `core` or `subcore`.

With the introduction of architect-family roles (Coding Agent Architect, Business Agent Architect), the routing semantics must distinguish between:

- **execution stream** (where a lane is executed: Core or SubCore, or null for architect-only work),
- **domain family** (which specialist family a packet targets: coding, business, or architect).

The current token schema allows only one semantic identity per packet, which creates ambiguity when a packet targets an architect role rather than a stream. For example, a Coding Agent Architect kickoff packet cannot cleanly express that it targets the architect family while not executing in either stream.

This ADR extends the token schema to support architect-family targeting without overloading or collapsing existing stream semantics.

---

## Decision

**FERROS will extend routing-token schema with a new `target_family` field, introduce mutual-exclusivity rules between `target_stream` and `target_family`, and preserve the existing stream semantics for Core/SubCore packets.**

The extended schema will be:

```yaml
route_token:
  token_version: "v2"
  issued_by: "FERROS Agent Architect Agent | FERROS Prompt Architect Agent"
  
  # Execution target (exactly one must be set)
  target_stream: "core | subcore | null"           # execution stream, mutually exclusive with target_family
  target_family: "coding | business | architect | null"  # domain family, mutually exclusive with target_stream
  
  # Work and identity
  run_profile: "core-runtime | subcore-runtime | architect-review | architect-hardening | ux-surface | | business-domain"
  run_id: "FRS-<stream-or-family>-<YYYYMMDD>-C<N>-W<N>"
  
  # Metadata
  issued_at: "YYYY-MM-DD"
  issued_at_utc: "YYYY-MM-DDTHH:MM:SSZ" (optional)
  expiry_cycle: "C<N>"
  posture: "recursive-lane-system | batch | interactive | queue-clear"
  track: "code | system | hardware"  # queue ownership, separate from target
```

**Constraints:**

1. Either `target_stream` OR `target_family` must be set; both cannot be non-null simultaneously.
2. If `target_stream` is non-null (e.g., `core`), then `target_family` must be null.
3. If `target_family` is non-null (e.g., `coding`), then `target_stream` must be null.
4. If neither is set, the token is malformed and execution must be refused.
5. `track` remains queue-track ownership and is never a stream or family identifier.
6. `run_profile` is a hint about the work type but is not an authorization field; routing decisions use target_stream/target_family only.

---

## Rationale

### Why this schema?

**Clarity:** Separates execution stream from domain family, preventing overload of `target_stream`.

**Extensibility:** New architect families can be added to the enum without redefining stream routing.

**Mutual exclusivity:** Prevents confusion about whether a packet targets an execution stream or a review family by making the choice explicit and enforcing it at validation time.

**Backward compatibility:** Core and SubCore packets continue to use the existing `target_stream` semantics; `target_family` is null for them.

**Chain of command:** FERROS Agent or FERROS Prompt Architect can validate token format before handing off to receiving agent, reducing validation burden on each agent.

### Alternative schemas considered

| Option | Summary | Reason not chosen (if rejected) |
|--------|---------|--------------------------------|
| Option A (chosen) | Separate `target_stream` and `target_family` with mutual exclusivity | - |
| Option B | Create a unified `target` field that can be `"core"`, `"coding-architect"`, etc. | Rejected because it conflates stream and family semantics and makes future extensions ambiguous |
| Option C | Nest architect-family routing in a separate `architect_packet` wrapper | Rejected because it complicates packet structure and requires multiple routing checks |
| Option D | Use `run_profile` as the routing source | Rejected because `run_profile` is a hint, not a policy; it can be mismatched without detection |

---

## Consequences

**Positive:**
- Architect-family kickoff packets can be routed cleanly without semantic collision.
- Core/SubCore streams continue unchanged.
- Multi-agent routing becomes auditable and machine-checkable.
- Future architect families (e.g., philosopher, advocate) can reuse the same schema.
- Authority-interruption contract can validate token schema as part of preflight checks.

**Negative / trade-offs:**
- Kickoff packet format is slightly heavier (two new optional fields per packet).
- Agents must enforce mutual-exclusivity rule and refuse malformed tokens.
- Existing live packets that do not include `token_version: v2` will be treated as legacy (v1) and warned but not rejected.

---

## Implementation

### Receiving agent responsibility

Every stream agent (Core, SubCore) and architect-family agent (Coding Agent Architect, Business Agent Architect) must:

1. Check for presence of `route_token` block.
2. If missing, refuse execution and return corrective guidance.
3. Validate mutual-exclusivity:
   - If `target_family` is non-null, refuse execution if agent is a stream (Core/SubCore).
   - If `target_stream` is non-null, refuse execution if agent is an architect-family role.
4. On mismatch, return explanation and corrective packet request.

### FERROS Agent / FERROS Prompt Architect responsibility

- Generate kickoff packets with valid token schema.
- Respect mutual-exclusivity rule when creating new packets.
- Validate incoming user requests for token field presence and correctness.
- On validation failure, generate corrective kickoff packet.

### Validator / Gatekeeper responsibility

- Check route-token schema during wave validation.
- Report schema errors as Unresolved Risks if detected mid-wave.
- Trigger authority-interruption contract if token version mismatches occur during execution.

---

## Compliance

The following must remain true while this ADR is active:

- Every kickoff packet includes a `route_token` block with `token_version: v1 | v2`.
- Either `target_stream` or `target_family` is set (but not both).
- `track` is never used as a routing identifier; it remains queue-ownership only.
- Core agent executes only if `target_stream == "core"` and `target_family == null`.
- SubCore agent executes only if `target_stream == "subcore"` and `target_family == null`.
- Coding Agent Architect executes only if `target_family == "coding"` and `target_stream == null`.
- Business Agent Architect executes only if `target_family == "business"` and `target_stream == null`.
- Malformed tokens (both fields set, or both null) are refused with corrective guidance.

---

## Examples

### Existing Core lane packet (v1 → v2 compatible)

```yaml
route_token:
  token_version: "v2"
  issued_by: "FERROS Agent Architect Agent"
  target_stream: "core"
  target_family: null
  run_profile: "core-runtime"
  run_id: "FRS-core-20260509-C1-W1"
  issued_at: "2026-05-09"
  expiry_cycle: "C1"
  posture: "recursive-lane-system"
  track: "code"
```

### Coding Agent Architect review packet (new family)

```yaml
route_token:
  token_version: "v2"
  issued_by: "FERROS Prompt Architect Agent"
  target_stream: null
  target_family: "coding"
  run_profile: "architect-hardening"
  run_id: "FRS-coding-20260509-C1-W1"
  issued_at: "2026-05-09"
  issued_at_utc: "2026-05-09T14:32:01Z"
  expiry_cycle: "C1"
  posture: "recursive-lane-system"
  track: "system"
```

### Malformed packet (rejected)

```yaml
# INVALID: both fields set
route_token:
  target_stream: "core"
  target_family: "coding"
  # Will be rejected by validation
```

---

## Deferred scope

- Token signing / cryptographic verification (future hardening).
- Runtime registry checks (token signature validation against stored key material).
- Per-packet expiry timestamps (currently using expiry_cycle only; finer-grained TTL deferred).

---

## References

- ADR-037: Agent Architect Governance and Routing Tokens
- docs/orchestration/AUTHORITY-INTERRUPTION.md
- docs/orchestration/ARCHITECT-FAMILY-PROMOTION.md
- .github/agents/ferros-core.agent.md
- .github/agents/ferros-subcore.agent.md
- .github/agents/ferros-coding-agent-architect.agent.md

---

*Last updated: 2026-05-09*
