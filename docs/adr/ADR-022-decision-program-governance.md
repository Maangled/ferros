# ADR-022 — Decision Program Governance, Taxonomy, and Evidence Tiers

**Status:** Accepted  
**Date:** 2026-04-24  
**Stream:** Cross-cutting  
**Deciders:** FERROS stream coordination / S8 docs
**Domain tags:** Governance, process, research-program  
**Primary evidence basis:** Analytical proof + current repo practice

---

## Context

The FERROS ADR set now spans multiple kinds of decisions: foundational architecture, UX doctrine, policy, ecosystem harvest, and governance posture. The series already has useful structure through status fields, stream ownership, and a shared template, but it does not yet declare what kind of decision each record represents, what evidence is expected, when a topic belongs in a research note instead of an ADR, or how later contributors should navigate the growing set without reconstructing the logic from chat history or oral context.

As FERROS expands into stronger governance, contract, shell, launch, and research questions, the ADR program needs an explicit operating model so the repo can preserve context without freezing premature decisions.

---

## Decision

**FERROS will treat its decision system as a structured program with three linked artifact types: ADRs for accepted or proposed decisions, research notes for pre-decision investigation, and evidence records for implementation or proof links that outgrow a single ADR body.**

Additional decisions:

- ADRs remain the canonical record of decisions that materially constrain streams, gates, contracts, doctrine, or governance.
- Research notes are the canonical place for high-value but not-yet-settled investigations, especially when evidence is still accumulating or when forcing a decision would create premature commitment.
- ADRs and research notes must carry domain tags and identify their primary evidence basis.
- Existing ADRs remain in place and are not renumbered or rewritten to fit the new model. The new governance layer is additive.
- `docs/adr/_INDEX.md` is the navigation surface for the ADR program. `docs/adr/_ROADMAP.md` records anticipated future topics without assigning authority before a record exists.

### Artifact types

| Artifact | Purpose | Typical status values |
|----------|---------|-----------------------|
| ADR | Record a decision, its rationale, and compliance rules | Draft, Proposed, Accepted, Superseded, Deprecated |
| Research note | Capture investigation before a decision is frozen | Draft, Exploring, Ready for ADR, Archived, Rejected |
| Evidence record | Link a decision or note to tests, CI, PRs, harnesses, or other proof surfaces | additive metadata; no decision status |

### Domain tags

Use one or more of the following domain tags when creating a new ADR or research note:

- architecture
- runtime
- UX doctrine
- governance
- policy
- security
- research
- ecosystem
- real-world application
- economics
- cross-cutting

### Evidence bases

Each ADR or research note should declare a primary evidence basis and may cite secondary ones.

| Evidence basis | Use when the record depends on | Typical artifacts |
|---------------|---------------------------------|-------------------|
| Implementation proof | Running behavior or contract conformance | tests, harnesses, CI, demos, fixtures |
| Formal or analytical proof | Invariants, schemas, cryptographic logic, structural reasoning | mathematical or protocol argument, benchmark, contract analysis |
| Research or precedent proof | External knowledge domains | papers, standards, legal or ethical precedent, prior art |
| Operational proof | Deployment or governance reality | field tests, real-device runs, sign-off trails, operational records |

---

## Rationale

This ADR formalizes patterns the repo is already using informally.

The newer ADRs already use status fields, stream ownership, compliance sections, and cross-cutting policy language. The workbench and shell ADRs already separate accepted decisions from unresolved research questions. The missing piece is a shared description of that program so future contributors can tell which records are binding, which are exploratory, and what kind of proof is being offered.

Without this layer, the program risks three failure modes:

- ADR inflation without navigation or taxonomy
- premature commitment on research-heavy topics such as governance, economics, or future rendering layers
- proof dilution, where records from very different domains look equally settled despite carrying different evidence quality

### Options considered

| Option | Summary | Reason not chosen (if rejected) |
|--------|---------|--------------------------------|
| Add a governing meta-ADR, research lane, and index surfaces (chosen) | Extend current practice into a structured decision program without rewriting history | — |
| Keep the current ADR set informal and rely on contributor judgment | Avoid new process documents and let future ADRs self-organize | Rejected because the repo is already large enough that missing taxonomy and evidence rules create context loss and inconsistent authority |
| Split technical, UX, and governance records into completely separate systems immediately | Create separate series for every domain | Rejected because FERROS still benefits from one shared decision archive; the immediate need is taxonomy and navigation, not fragmentation |

---

## Consequences

**Positive:**
- FERROS gains a durable decision-program structure before more domains and streams add records.
- Research can accumulate in-repo without forcing premature architectural or governance commitments.
- Future contributors get a clearer path: doctrine, index, relevant ADRs, then research notes where the decision is not yet frozen.
- The repo can preserve reasoning that would otherwise live only in conversation, memory, or review prose.

**Negative / trade-offs:**
- The docs and governance stream gains more stewardship work.
- Not every older ADR will immediately carry the new fields, so the program will remain partially mixed until records are gradually backfilled.
- Adding a research lane increases documentation volume and requires discipline to avoid leaving stale exploratory notes uncurated.

---

## Compliance

- New ADRs should use the updated template fields for domain tags and evidence basis.
- Research-heavy questions should default to a research note when the evidence is not yet strong enough for a real decision.
- Existing ADRs do not need to be rewritten, but new indexes or roadmaps may classify them.
- If a future change makes the ADR, research, or evidence split actively harmful, revisit this ADR explicitly rather than drifting around it.

---

## References

- [../../DOCTRINE.md](../../DOCTRINE.md)
- [ADR-0001-start-new-do-not-fork.md](./ADR-0001-start-new-do-not-fork.md)
- [ADR-015-universal-parametric-authoring-workbench.md](./ADR-015-universal-parametric-authoring-workbench.md)
- [ADR-021-dependency-admission-policy.md](./ADR-021-dependency-admission-policy.md)
- [../../streams/S8-docs/README.md](../../streams/S8-docs/README.md)
