# ADR-017: HTML Surface Incubation Strategy

- Status: Accepted
- Date: 2026-04-22
- Stream: Cross-cutting (S5 UX, Stream docs, repository structure)

## Context

FERROS already carries a large set of HTML surface prototypes. They are not accidental scope sprawl.
They serve two real purposes:

1. They are aspirational pathways that make future surfaces concrete enough to plan against.
2. They act as the fastest editable drivers for the eventual front end.

In another large Rust workspace, the front end lived inside its own Cargo package, generated in Rust and connected through WASM. That approach provided strong separation, but it significantly bottlenecked UI iteration and made direct editing slower. FERROS intentionally started with standalone HTML surfaces first and is working backward from those surfaces into contracts, runtime seams, and eventually Rust-owned implementations.

Now that FERROS has a stream model, work should proceed from both directions:

- Contract/core work moves forward from the Rust and governance side.
- Surface work moves forward from the HTML prototype side.
- The two directions converge as streams become operational.

## Decision

1. HTML surfaces are treated as incubation assets and front-end drivers, not as proof that backend/runtime scope is complete.
2. Backup and obsolete HTML variants should not live in the main `docs/` top layer. They move into an archive path.
3. Active HTML surfaces may remain temporarily reachable while streams are being stood up, but the long-term direction is to move them out of the top layer of `docs/` and eventually out of `docs/` entirely.
4. The likely destination for mature surfaces is a dedicated front-end package once the team can regain fast editing without repeating the earlier WASM workflow bottleneck.
5. Reviewers should evaluate HTML surfaces as part of a bidirectional delivery strategy: they pressure-test UX and contracts now, while Rust crates and runtime contracts catch up from the systems side.

## Immediate repository policy

- Keep contract docs, ADRs, gates, and progress records prominent in `docs/`.
- Move backup HTML variants into `docs/surfaces/archive/`.
- Keep future reorganization incremental so links and evidence trails do not break.

## Consequences

- The repo becomes easier to review because archival HTML noise is removed from the primary docs layer.
- Surface work remains legitimate and strategically useful rather than being treated as random prototype drift.
- Stream execution can advance in parallel from front-end and backend directions without forcing premature unification.
- A future surface-package migration remains open, but it is not required before current stream work can continue.
