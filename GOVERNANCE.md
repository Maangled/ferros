# FERROS Governance

FERROS is currently governed as a small-team, pre-1.0 project. The purpose of governance at this stage is not bureaucracy; it is to keep authority explicit, keep claims honest, and make it clear which documents actually control decisions.

---

## Governance Goals

Governance in FERROS should protect five things:

1. User sovereignty over identity, automation, and local state.
2. Explicit ownership of contracts, streams, and approval surfaces.
3. Honest gate and launch claims backed by evidence.
4. Minimal surprise for contributors about where decisions live.
5. The ability to grow later without rewriting the project around hidden assumptions.

The root decision rubric lives in [DOCTRINE.md](./DOCTRINE.md).

---

## Current Governance Model

FERROS uses a stream-first, gate-driven model.

- Streams define ownership boundaries for implementation and planning.
- Gates define convergence points where evidence must exist before the next stage opens.
- ADRs define decisions that materially constrain architecture, doctrine, policy, or governance.
- Contracts define cross-stream interfaces and are owned by the stream that publishes them.
- Status and launch surfaces must stay honest; they summarize truth, they do not invent it.

This is intentionally lighter than a foundation, steering committee, or formal RFC process. The repo is still pre-1.0 and small enough that clarity matters more than ceremony.

---

## Active Authority Surfaces

Use the following order when deciding which document controls a question:

1. [DOCTRINE.md](./DOCTRINE.md) for root evaluation principles.
2. [docs/adr/_INDEX.md](docs/adr/_INDEX.md) and the referenced ADRs for accepted decisions.
3. `streams/SN-*/README.md`, `BACKLOG.md`, `CONTRACTS.md`, and `PROGRESS.md` for stream ownership and current work.
4. `docs/gates/GN.md` for gate requirements and evidence.
5. [docs/contracts/CONTRACTS-OVERVIEW.md](docs/contracts/CONTRACTS-OVERVIEW.md) plus owning stream contract docs for interface boundaries.
6. [STATUS.md](STATUS.md) and [LAUNCH.md](LAUNCH.md) for dashboard and launch truth.
7. [SECURITY.md](SECURITY.md) and [THREAT-MODEL.md](THREAT-MODEL.md) for vulnerability reporting and security posture.

Historical or incubator surfaces may still be valuable, but they should not override the sources above unless a newer ADR or explicit update says so.

---

## Roles

### Maintainers

Maintainers keep the repository coherent and decide whether a change is ready to land when ownership or evidence is disputed.

Current responsibilities:

- preserve the doctrine and ADR program
- keep stream and gate truth surfaces aligned with reality
- reject changes that overclaim readiness or bypass owning contracts
- coordinate pre-1.0 release and launch criteria

### Stream owners

Each stream owns the implementation, planning docs, and contract surfaces assigned to that stream.

Stream owners are expected to:

- keep their stream docs truthful
- update contracts they own when behavior changes
- record meaningful progress in `PROGRESS.md`
- identify when a gate or cross-stream claim has become stale

### Contributors

Contributors work inside the stream model, follow owning contracts, and avoid broadening scope without explanation.

Contributors are expected to:

- read [CONTRIBUTING.md](./CONTRIBUTING.md) first
- route work to the correct stream
- avoid mutating frozen contracts in place
- state uncertainties rather than hiding them in docs or code

---

## Decision Types

| Decision type | Primary surface |
|--------------|-----------------|
| Root doctrine or governance posture | `DOCTRINE.md` or an ADR that explicitly amends it |
| Architecture, policy, shell, or contract doctrine | ADR |
| Exploratory question not ready for a decision | Research note under `docs/adr/_RESEARCH-NOTES/` |
| Stream planning and sequencing | stream docs under `streams/` |
| Gate closure | gate doc plus evidence-backed truth-sync in related status surfaces |
| Vulnerability handling | `SECURITY.md` |
| Conduct expectations | `CODE_OF_CONDUCT.md` |

---

## How Decisions Land

1. Start from the owning stream or contract surface.
2. If the change alters doctrine, a boundary, or a cross-stream rule, write or update an ADR.
3. If the question is still exploratory, keep it in a research note until the evidence is strong enough for a decision.
4. When code or docs land, update the nearby truth surfaces in the same change when practical.
5. Do not declare a gate or launch condition satisfied without the required evidence recorded in repo.

---

## Cross-Stream Rules

- Do not silently edit another stream's contract boundary.
- Do not treat a prototype, draft HTML surface, or convenience CLI as the canonical contract unless the owning docs say it is.
- Do not mutate a frozen schema version in place.
- Do not upgrade status or gate language beyond what evidence supports.
- When cross-stream changes are necessary, explain why the scope cannot stay narrower.

---

## Conflict Resolution

When documents disagree, prefer the most authoritative current surface from the list above. If the disagreement is substantive rather than stale wording:

1. Stop treating the conflict as implicit consensus.
2. Identify the owning stream or decision record.
3. Write the smallest correcting change that restores one source of truth.
4. If the conflict reflects a real unresolved trade-off, open or update an ADR or research note instead of forcing a quiet merge.

---

## Release and Launch Integrity

Before claiming a milestone, gate closure, or launch milestone:

- check the gate document
- check the relevant stream docs
- check the dashboard and launch definition
- verify the evidence exists in repo and is still true

In FERROS, a merged PR is not equivalent to shipped capability, and a runnable prototype is not equivalent to launch.

---

## Future Governance Expansion

This file is intentionally minimal. Future versions may add:

- more explicit maintainer or reviewer roles
- issue and PR templates for contributor intake
- release-signoff checklists
- formal dependency review prompts
- community governance once the project is post-launch and has real external contributors

Any major governance expansion should be recorded through the ADR program rather than growing only in prose here.
