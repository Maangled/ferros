# RN-2026-04 — ACC Card / Deck Projection and Handoff Model

**Status:** Exploring  
**Date:** 2026-04-24  
**Stream sponsor:** Cross-cutting (S3 / S5 / S8)  
**Domain tags:** UX doctrine, runtime projection, research  
**Primary evidence basis:** Analytical proof + current repo doctrine

---

## Question

How should FERROS project the current agent-center backend objects into the card, deck, bag, table, and arena language without granting architectural authority to the Discord-like ACC prototype?

---

## Why This Matters Now

The repo currently holds two different kinds of ACC artifact:

- `docs/agent-command-center.html` as subject-matter prior art for agent-center flows
- `streams/S5-ux/PHASE-B-SHELL-WIREFRAME.md` as the canonical shell topology and intent model

At the same time, the accepted ADR set already commits FERROS to cards, decks, bags, the Forge shell pattern, and eventual arena projection. If the first live shell is built without an explicit object-to-card mapping, the prototype shape can silently become product authority even though the doctrine points elsewhere.

---

## Inputs and Related Records

- [../ADR-007-single-file-system.md](../ADR-007-single-file-system.md)
- [../ADR-009-four-corner-docking-layout.md](../ADR-009-four-corner-docking-layout.md)
- [../ADR-010-cards-and-decks-nomenclature.md](../ADR-010-cards-and-decks-nomenclature.md)
- [../ADR-015-universal-parametric-authoring-workbench.md](../ADR-015-universal-parametric-authoring-workbench.md)
- [../ADR-017-html-surface-incubation-strategy.md](../ADR-017-html-surface-incubation-strategy.md)
- [../../streams/S5-ux/PHASE-B-SHELL-WIREFRAME.md](../../streams/S5-ux/PHASE-B-SHELL-WIREFRAME.md)
- [../../docs/agent-command-center.html](../../agent-command-center.html)
- [../../docs/forge-workbench.html](../../forge-workbench.html)

---

## Current Canonical Objects

The current backend and shell work implies the following canonical object families:

| Object family | Current owner | Current meaning |
|---------------|---------------|-----------------|
| Agent | S3 | Registry entry with manifest, status, capabilities, and lifecycle state |
| Orchestration run | Cross-cutting / local-driver pattern | Ordered execution state across waves, agents, or delegated tasks |
| Capability grant | S2 | Signed permission envelope and consent artifact |
| Deny event | S4 + S3 runtime surfaces | A blocked capability request with context and audit value |
| Profile action | S2 + shell | User-owned action that changes profile, consent, or local state |

These objects should remain canonical even if the shell changes visual language.

---

## Working Projection Model

### Visual projection

| Canonical object | Proposed projection |
|------------------|---------------------|
| Agent | Card |
| Available agent set or user-curated operational set | Deck |
| User-owned collection of agent, grant, and routine surfaces | Bag |
| Active orchestration or task sequence | Table or spread of played cards |
| Deny event | Interrupt or audit card |
| Profile action history | Audit trail or journal-backed card history |

### Mechanical projection

The shell should remain shell-owned and backend-backed.

1. Registry or route selection chooses a canonical object.
2. The shell projects that object into a card-aware surface without changing the underlying type boundary.
3. Privileged action flows through explicit shell intents, consent resolution, and audit handoff.
4. Card and deck metaphors remain projection and composition language, not a replacement for the canonical backend records.

This keeps the backend contract stable while allowing the live surfaces to migrate toward the arena and table model later.

---

## Prototype Authority Split

The following authority split should remain explicit:

- `docs/agent-command-center.html` is subject-matter prior art for agent-center flows and vocabulary.
- `streams/S5-ux/PHASE-B-SHELL-WIREFRAME.md` is the canonical shell topology.
- ADR-009, ADR-010, ADR-015, and ADR-017 define the live direction for shell layout, card language, and incubation strategy.

The first live shell should therefore borrow subject matter from the ACC prototype, but not its Discord-like layout as architecture precedent.

---

## Open Questions

- Should grants project as standalone cards, modifiers attached to agent cards, or both depending on route?
- Should orchestration runs be modeled as decks in preparation, cards in play on the table, or as both states at different phases?
- Which profile actions deserve durable card identity versus simple audit-log projection?
- How much of the arena metaphor should appear in the first localhost shell slice before there is a richer visual renderer?

---

## Promotion Criteria

Promote this note into a formal ADR when:

- the S3 remote contract is written clearly enough that the first live shell has stable canonical objects
- S5 has a real first-shell slice or implementation plan that needs a binding object-to-card rule
- the projection model is strong enough to constrain future shell or arena work instead of merely informing it
