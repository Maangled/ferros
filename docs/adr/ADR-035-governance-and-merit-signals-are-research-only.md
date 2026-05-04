# ADR-035 - Governance and merit signals are research-only

**Status:** Draft  
**Date:** 2026-05-04  
**Stream:** Cross-cutting / S8  
**Deciders:** Maangled  
**Domain tags:** governance / research / policy / cross-cutting  
**Primary evidence basis:** Research or precedent proof

_See [ADR-022-decision-program-governance.md](./ADR-022-decision-program-governance.md), [../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md](../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md), and the discovery note at `/home/homelab001/apps/FERROS-PDF-DISCOVERY-NOTE.md`._

---

## Context

The discovery packet surfaced concepts such as Hero cards, merit signals, councils, AI-judged authority, and contribution-weighted legitimacy. These may be useful research prompts, but FERROS does not yet have the evidence, anti-capture rules, or product need to elevate them into active authority surfaces or product claims.

---

## Decision

**If accepted, FERROS will treat governance, merit, ranking, and contribution-weighted authority concepts from the discovery packet as research-only until separate evidence and governance ADRs justify promotion.**

This draft exists to stop premature authority features from leaking into UX scaffolding.

---

## Rationale

Research signals are useful, but authority claims are costly and easy to overstate.

### Options considered

| Option | Summary | Reason not chosen (if rejected) |
|--------|---------|--------------------------------|
| Option A (draft target) | Keep governance and merit ideas in research-only posture | - |
| Option B | Let merit and governance signals enter UX as soft guidance | Rejected because even soft authority language can become de facto product policy |
| Option C | Ban all governance-oriented research until much later | Rejected because the ideas are still worth preserving as research prompts |

---

## Consequences

**Positive:**
- Prevents accidental authority creep in the operator UX.
- Preserves research value without pretending product legitimacy.
- Gives future governance work a clear quarantine boundary.

**Negative / trade-offs:**
- Some visually compelling progression or rank motifs remain unavailable.
- Future governance work will need dedicated ADRs instead of piggybacking on current scaffolds.

---

## Compliance

- Do not surface governance or merit signals as active product authority without separate ADR review.
- Do not treat contribution-weighted labels as evidence, security, or identity truth.
- Revisit this ADR if a future governance lane gains concrete implementation evidence and explicit anti-capture design.

---

## Implementation Evidence

- Immediate pressure comes from the discovery packet and the operator-UX plan that must quarantine these ideas while still mining useful motifs.

---

## Deferred Scope or Open Research

- Deferred: future governance research notes and decision models.
- Deferred: whether any progression signals can be reused safely as local-only, non-authoritative motifs.
- Deferred: anti-capture and legitimacy requirements for any future governance lane.

---

## References

- [ADR-022-decision-program-governance.md](./ADR-022-decision-program-governance.md)
- [../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md](../orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md)
