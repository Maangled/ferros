# ADR-018 — Harvest botgen-rust for Agent Runtime Shapes, Not Direct Code

**Status:** Accepted  
**Date:** 2026-04-23  
**Stream:** S6  
**Deciders:** FERROS stream coordination / S6 harvest lane

---

## Context

`Maangled/botgen-rust` overlaps strongly with FERROS S3, S4, and future S6 data-work concerns: agent lifecycle, in-memory registry shape, work queue and dispatch, materialize-from-description flows, and audit or provenance surfaces. The audit captured in `.tmp/bg-r.md` shows that the repo contains several strong type and architecture ideas, but also major incompletions and invariants that FERROS cannot inherit as-is: placeholder handlers, TODO-only persistence layers, duplicated agent traits, hard-coded agent IDs, ephemeral queue state, and a split between task audit history and lifecycle events. FERROS needs the architectural shapes, but not a wholesale port of the implementation.

---

## Decision

**FERROS will harvest botgen-rust as an architecture source for S3 and S4 by adopting a small set of type and lifecycle shapes, adapting queue and registry patterns under FERROS invariants, and explicitly discarding placeholder or broken implementation layers rather than importing code wholesale.**

The output of this ADR is guidance for FERROS-owned implementations, not permission to vendor or bulk-port the botgen workspace.

---

## Rationale

The audit shows that botgen-rust gets several foundational shapes right: explicit agent state transitions, task and history types, queue scheduling concepts, and a materialize-from-description pipeline shape. Those are useful inputs for FERROS. However, the same audit also shows that the repo is not a direct implementation base for FERROS: persistence gaps are unresolved, some critical surfaces are uncompiled doc-spec prototypes, some traits are duplicated, and some operational paths are structurally unsafe or incomplete. Adopting the shape while rewriting the implementation keeps FERROS aligned with its consent-first, locally-sovereign, typed-contract model.

### Options considered

| Option | Summary | Reason not chosen (if rejected) |
|--------|---------|--------------------------------|
| Harvest architectural shapes only (chosen) | Adopt or adapt selected types and patterns, but reimplement them under FERROS contracts | — |
| Bulk-port botgen subsystems into FERROS | Reuse queue, registry, lifecycle, and generator code directly | Rejected because the audited code contains TODO-only persistence, duplicated trait layers, placeholder handlers, and non-FERROS invariants |
| Ignore botgen entirely | Treat botgen as irrelevant because it is Discord-shaped | Rejected because the audit clearly shows reusable agent, queue, and provenance shapes that would otherwise be rediscovered manually |

---

## Consequences

**Positive:**
- S4 can now implement capability and deny-by-default policy primitives without pretending the old repo is an implementation dependency.
- S3 has a concrete prior-art basis for future `Agent` trait, registry, lifecycle, and command-index design.
- S6 has an explicit boundary between architectural inspiration and prohibited direct reuse.
- FERROS can preserve good ideas from botgen-rust while rejecting its unfinished or structurally unsafe parts.

**Negative / trade-offs:**
- Some appealing subsystems, especially the queue and scheduler, must still be reimplemented rather than imported.
- S3 and S4 work may feel slower in the short term because this ADR narrows what counts as legitimate reuse.
- Later contributors must read this ADR rather than assuming any similarly named botgen type is safe to copy.

---

## Adopt / Adapt / Reference / Discard

### Adopt

- Agent lifecycle state vocabulary: adopt the explicit lifecycle-state idea from botgen-rust, including a paused or suspended state, but keep FERROS free to add a resuming transition when needed.
- Task, priority, status, and history atoms: adopt the audited `Task`, `TaskStatus`, `Priority`, and `TaskHistoryEntry` shape as the basis for FERROS task and provenance types.
- Rich lifecycle event vocabulary: promote a typed `AgentEvent` surface into FERROS as a first-class compiled type rather than leaving lifecycle events as generic strings.

### Adapt

- Base-agent concurrency shape: adapt the three-loop model (task loop, message loop, health loop) as a FERROS execution pattern, but replace placeholder task and message handlers with typed capability-aware behavior.
- Agent registry shape: adapt the in-memory registry concept, but require typed IDs, deterministic behavior, and a durable persistence strategy rather than a `String`-keyed volatile map.
- Work queue and scheduler: adapt the multi-index queue and scheduling-policy concepts, but redesign for FERROS concurrency, atomic claim behavior, and durable history without truncation.
- Materialize-from-description: adapt the `BotSpec` idea into a FERROS agent descriptor or manifest shape, but remove Discord-specific assumptions and make registration after materialization an explicit contract.
- Event bus: adapt the publish-subscribe pattern, but use typed lifecycle events and an ordered persistence sink.

### Reference

- GatewayAgent and other compiled concrete agents: use them as examples of how the lifecycle shape plays out in practice.
- AgentManager doc-spec material: treat it as architecture intent, not as executable code.
- Correlation, tracing, and command-router ideas from the botgen doc-spec surfaces: useful for later observability design, not immediate implementation.

### Discard

- `AgentHandle` implementation with a hard-coded identity.
- `PgAgentRepository` and other TODO-only persistence layers.
- Placeholder worker-pool execution logic.
- Duplicated agent-trait definitions split across shared and service layers.
- Any queue persistence design that truncates history or makes audit retention bounded by default.

---

## Downstream Implications

### S3 Agent Center

- S3 should use this ADR as the basis for future `Agent` trait, registry, manifest, and lifecycle decisions.
- S3 should not read `botgen-rust` directly for implementation work once this ADR exists.

### S4 Runtime / OS Core

- S4 may now implement the first `ferros-core` policy and capability primitives using FERROS-owned abstractions, informed by the queue, lifecycle, and event decisions here.
- S4 should treat queue durability, agent persistence, and lifecycle-event storage as future FERROS implementation work, not as code to port.

### S6 Ecosystem Harvest

- This ADR is the canonical result of the first botgen audit pass.
- Any future revisit should extend this ADR only if a later botgen subsystem is shown to add materially new value.

---

## Compliance

- If FERROS chooses to import raw botgen code into `crates/` or any S3 or S4 implementation surface, revisit this ADR.
- If a later audit shows a botgen subsystem is fully implemented, typed, and directly aligned with FERROS invariants, revisit the relevant classification here rather than bypassing the ADR process.
- If FERROS changes its queue, provenance, or lifecycle invariants, revisit this ADR and the downstream stream docs that consume it.

---

## References

- `.tmp/bg-r.md`
- [ADR-013](c:\Users\mosle\OneDrive\Desktop\GPTs\ferros\ferros\docs\adr\ADR-013-legacy-integration-strategy.md)
- [streams/S3-agent-center/README.md](c:\Users\mosle\OneDrive\Desktop\GPTs\ferros\ferros\streams\S3-agent-center\README.md)
- [streams/S4-runtime/README.md](c:\Users\mosle\OneDrive\Desktop\GPTs\ferros\ferros\streams\S4-runtime\README.md)
- [streams/S6-harvest/README.md](c:\Users\mosle\OneDrive\Desktop\GPTs\ferros\ferros\streams\S6-harvest\README.md)