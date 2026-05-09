# Architect-Family Promotion Rubric

> **Authority:** docs/orchestration/AUTHORITY-MAP.md
>
> This document defines the evidence and promotion path for new architect-family roles (Coding Agent Architect, Business Agent Architect) from candidate through advisory to active status.

---

## Lifecycle states

Every architect-family role follows this strict progression:

1. **Candidate** — proposed by user, not yet authorized for execution; collects initial scope/charter
2. **Research-only** — reads codebase, analyzes patterns, produces bounded advisory recommendations; no lane execution, no state mutation
3. **Advisory** — runs bounded review lanes with lane ceiling ≤ 4; outputs are recommendations to human, not autonomous write authority
4. **Support** — runs unbounded lanes within normal ceiling (≤ 8); must defer to Core/SubCore lane architects on promotions
5. **Active** — full write authority; owns promotion logic for specialized roles in its family; may invoke other agents without pre-approval

---

## Promotion gates

### Candidate → Research-only

**Evidence required:**
- Role charter written and anchored to existing governance stack
- Non-overlapping mandate with existing roles (validated against ORCHESTRATION-AGENTS.md)
- Authority statement (what the role owns, what it does not own)

**Approval:** Operator (minimal gate; validates charter only)

**Duration:** ≤ 1 cycle or ≤ 2 weeks

---

### Research-only → Advisory

**Evidence required:**
- Three successful bounded packets delivered (each ≤ 2 anchors, ≤ 3 recommendations)
- Zero unresolved risks in any of the three packets
- No schema conflicts with frozen schemas (profile.v0.json, capability-grant.v0.json)
- No authority-mismatch incidents or route-token collisions
- Proof that output integrates cleanly with canonical authority docs
- Successful dryrun of authority-interruption contract (role respects authority_ack, pauses on frozen schema touch, reports mismatches)

**Approval:** Operator + one existing specialist role (validates evidence chain and integration)

**Duration:** ≤ 2 cycles or ≤ 4 weeks

**Constraints:** Role must remain read-mostly; outputs are recommendations and audit reports, not autonomous writes.

---

### Advisory → Active

**Evidence required:**
- Five successful advisory packets with zero unresolved risks
- Proof that role correctly invoked authority-interruption contract (did it pause when it should have?)
- No cross-family semantic collisions (route-token, target_family values, queue tracks remain distinct)
- Operator approval of promotion decision documented in authority_ack
- Role-specific promotion rubric drafted (same format as this document)
- Rollout-manifest update drafted and operator-approved

**Approval:** Operator + FERROS Prompt Architect Agent (validates governance fit and decision justification)

**Duration:** ≤ 4 cycles or ≤ 8 weeks

**Constraints:** After promotion, role owns its own evidence and promotion decisions for specialized agents in its family; must not create sibling architect families without explicit approval.

---

## Evidence standards

Every promotion gate requires submitted evidence to meet these standards:

| Standard | Requirement |
|----------|-------------|
| Anchored | Every claim cites a file path and line range (or appendix reference) from canonical docs or repo artifacts |
| Bounded | Claims are specific, not sweeping; e.g., "three packets" not "multiple packets" |
| Verifiable | Claims could be checked by a third party reading the evidence alone without running code |
| Non-overlapping | Claims do not conflict with existing specialist role mandates (validate against ORCHESTRATION-AGENTS.md each cycle) |
| Authority-aware | Role demonstrates understanding of when to pause for authority mismatch and respects hard stops |
| De-narrowed | Role demonstrates breadth in recommendations (covers ≥ 3 categories per packet's anti-narrowing rule) |

---

## Special cases

### Merging advisory roles into active roles

If two advisory roles prove redundant or overlapping, merge them:

1. Operator decides which role absorbs the other
2. Evidence from both roles carries forward
3. Merged role immediately enters Advisory state (reset to Advisory even if one role was closer to Active)
4. Merged role must collect new evidence before promotion to Active

### Retiring advisory roles

If an advisory role is no longer needed:

1. Operator issues retirement decision anchored to explicit reason
2. Role transitions to `retired` (same as `merged`)
3. All outstanding packets from retired role are either closed-out (if Safe to close) or reassigned to another role
4. Retired role does not generate new packets

### Research-only role stuck (no promotable evidence after 2 cycles)

If a research-only role has not collected sufficient evidence after 2 cycles:

1. Role is demoted back to `candidate`
2. Charter is re-reviewed
3. Role may be re-proposed with narrowed scope or retired if no clear path forward

---

## FERROS Coding Agent Architect promotion status

**Current state:** Candidate (proposed by kickoff packet FRS-coding-20260509-C1-W1)

**Charter:** Route-rule gap review, lifecycle promotion logic audit, registry integrity sweep, evidence standard delta proposal

**Next gate:** Collect evidence for Candidate → Research-only approval

---

## FERROS Business Agent Architect promotion status

**Current state:** Candidate (proposed for future cycles)

**Charter:** TBD (blocked until Coding Agent Architect reaches Advisory state)

**Next gate:** Held pending Coding Agent Architect evidence chain

---

*Last updated: 2026-05-09*
