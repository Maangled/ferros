# RN-2026-05 — Voting and Decision Models

**Status:** Exploring  
**Date:** 2026-05-02  
**Stream sponsor:** S6 / Cross-cutting  
**Domain tags:** governance / research / ecosystem / cross-cutting  
**Primary evidence basis:** Implementation proof

## Question

Which decision and voting patterns from `workpace-rust` are worth preserving for future
FERROS consent, governance, or card-system actions without freezing governance semantics too
early?

## Why this matters now

The final archive sweep of `workpace-rust` found a real subsystem that ADR-019 intentionally
left out: seven implemented vote modes, a type-dispatched tally pipeline, and persisted vote
totals. FERROS is not ready to ratify a voting mechanism, but allowing the repo to disappear
without preserving the shapes would force a later stream to rediscover them from scratch.

This note preserves the usable taxonomy while keeping voting in the research lane, consistent
with ADR-022 and the roadmap rule that governance-mechanism choice should not enter the ADR
lane before FERROS has a concrete decision surface.

## Inputs and Related Records

- [ADR-013](../ADR-013-legacy-integration-strategy.md)
- [ADR-019](../ADR-019-harvest-workpace.md)
- [ADR-022](../ADR-022-decision-program-governance.md)
- [../_ROADMAP.md](../_ROADMAP.md)
- [../../../../workpace-rust/modules/voting/src/voting_types/mod.rs](../../../../workpace-rust/modules/voting/src/voting_types/mod.rs)
- [../../../../workpace-rust/modules/voting/src/tally_votes.rs](../../../../workpace-rust/modules/voting/src/tally_votes.rs)
- [../../../../workpace-rust/modules/voting/src/voting_types/binary.rs](../../../../workpace-rust/modules/voting/src/voting_types/binary.rs)
- [../../../../workpace-rust/modules/database/src/voting.rs](../../../../workpace-rust/modules/database/src/voting.rs)

## Findings

### Voting modes already implemented in `workpace-rust`

`workpace-rust` implements seven vote modes behind a match-dispatched tally pipeline:

| Mode | Shape | Likely FERROS use |
|------|-------|-------------------|
| `binary` | yes/no majority | narrow consent gates, publish-or-not gates, feature toggles |
| `binary_with_justification` | yes/no plus collected reasons | decisions that need rationale capture |
| `approval_voting` | independent accept/reject per option | shortlist or admissions-style filtering |
| `multiple_choice` | select one option from many | route or mode selection |
| `weighted_voting` | distribute points across options | allocation or prioritization questions |
| `consensus_scale` | averaged score across a bounded scale | sentiment or readiness measurement |
| `open_ended` | collect text only | rationale gathering, comments, or proposal refinement |

The important preservation point is not the exact Rust code. It is the fact that the old repo
already separated several distinct decision shapes rather than forcing everything into one
binary vote primitive.

### Tally orchestration pattern

`modules/voting/src/tally_votes.rs` routes tally work by `vote_type` and writes the results to
`vote_totals` using an `INSERT ... ON CONFLICT ... DO UPDATE` upsert. That combination preserves
three useful ideas:

- vote collection can stay append-oriented while the latest aggregate result is cached
  separately;
- tally logic can vary by vote mode without changing the caller contract; and
- a typed or enumerated vote mode should remain exhaustively handled rather than being a loose
  plugin string.

### Voting power model found in the old repo

The current `binary` path is not one-person-one-vote. It derives voting power from claimed
tokens attached to a card identifier and compares `yes` votes against that total.

This is useful prior art because it exposes a real FERROS design fork:

- identity-based consent or governance;
- token- or stake-weighted decisions;
- delegated or grant-scoped decisions; or
- some split between hard consent gates and softer recommendation tallies.

FERROS should not inherit the token model by default, but it should remember that the old repo
already explored weighted decisions anchored to card state rather than user roles.

### What should carry forward

- Preserve the taxonomy of decision shapes.
- Preserve the separation between collected ballots and latest aggregate tally.
- Preserve rationale-bearing vote forms as a first-class option rather than an afterthought.
- Preserve exhaustive dispatch over a known vote-mode set.

### What should not carry forward automatically

- Direct adoption of the old database tables or JSON structures.
- Implicit token-weighted governance as FERROS default policy.
- Any assumption that a tally should directly mutate canonical FERROS state without an explicit
  consent or policy boundary.

## Open Questions

1. Which FERROS surfaces actually need voting versus simpler consent prompts or operator review?
2. Should future FERROS decision models distinguish hard consent, advisory scoring, and
   governance votes as separate classes?
3. If weighted voting exists, what is the authority source: identity, asset ownership, grants,
   delegated authority, or something else?
4. Should tally results live in a dedicated decision ledger, a local profile surface, or a
   stream-specific subsystem?
5. What minimum audit trail is required before a decision result can trigger state changes?

## Promotion Criteria

Promote this note into an ADR only when all of the following are true:

- FERROS has a concrete decision surface that genuinely needs more than a simple allow/deny
  consent gate.
- The authority model for voting power is explicit and reviewable.
- The audit and state-change boundary for decision outcomes is defined.
- A chosen decision model is intended to constrain streams, contracts, or governance behavior.

## References

- [ADR-013](../ADR-013-legacy-integration-strategy.md)
- [ADR-019](../ADR-019-harvest-workpace.md)
- [ADR-022](../ADR-022-decision-program-governance.md)
- [../_ROADMAP.md](../_ROADMAP.md)