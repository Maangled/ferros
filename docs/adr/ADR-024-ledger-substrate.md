# ADR-024 — Ledger/Chain Substrate Comparison and Recommendation

**Status:** Proposed — pending (a) counsel review of `docs/legal/` scaffold and (b) explicit ratification turn.  
**Date:** 2026-04-27  
**Stream:** S8 primary; S6 consumer awareness  
**Deciders:** FERROS stream coordination / S8 docs / S6 harvest  
**Domain tags:** architecture / economics / governance / cross-cutting  
**Primary evidence basis:** Research or precedent proof

_See [ADR-022-decision-program-governance.md](./ADR-022-decision-program-governance.md), [_INDEX.md](./_INDEX.md), and [../../DOCTRINE.md](../../DOCTRINE.md). Cross-reference ADR-023 (onramp policy) for the consent-invariant that any ledger integration must satisfy, and ADR-021 (dependency admission) for the dependency posture any new substrate dependency must pass._

> **Note:** This ADR produces a recommendation, not a binding commitment. FERROS's stance on ledger/chain substrate is an open policy question as of 2026-04-27. Implementation work should not begin until this ADR reaches Accepted status through the process defined in ADR-022. No frozen schemas are touched by this ADR.

---

## Context

FERROS issues capability grants, maintains audit logs, and tracks progression seals. All of these artifacts are currently stored locally on the user's device as signed JSON documents with no external verification layer. This works for the current locally-sovereign, single-device use case.

However, FERROS's longer-horizon goals — a marketplace for agents and decks, cross-device profile portability, and verifiable grant histories — raise questions about whether local-only signed records are sufficient. A publicly verifiable ledger or a shared distributed log could provide:

- **Revocation finality:** a grant revocation that is globally visible, not just locally asserted.
- **Marketplace settlement:** verifiable transfer of ownership or license for purchased agent bundles or deck artifacts.
- **Cross-device consistency:** a profile or grant state that can be recovered after device loss without a trusted central server.

The current S6 harvest work (ADR-018, ADR-019, ADR-020) surfaces prior-art patterns from botgen-rust and sheetgen-rust that include ledger references. The ledger question must be resolved at the policy level before S6 or S7 can proceed with any ledger-touching implementation.

---

## Decision

**FERROS will adopt a non-chain signed-ledger model as its default for the v0.1.0–v0.2.0 window, and will treat public-chain options as a future escalation path contingent on explicit marketplace or cross-device requirements being confirmed.**

The recommendation in detail:

- **For now:** all grants, seals, audit events, and capability records remain locally signed JSON documents using the already-landed `SignedProfileDocument` v0 boundary. No public-chain write is required.
- **Near-term:** when cross-device portability or marketplace settlement is actively designed (post-D1), evaluate a signed-ledger anchoring model — an append-only log of signed events that can be replicated to any peer or hub device — before committing to a public chain.
- **If public-chain is ultimately needed:** prefer a chain option that satisfies the three FERROS invariants below (locally sovereign, deliverable on a flashdrive, and does not require continuous internet connectivity to verify local state).

---

## Rationale

### FERROS invariants a ledger option must satisfy

Any ledger or chain substrate considered for FERROS must satisfy all three:

1. **Locally sovereign.** A FERROS user must be able to operate with full capability (issue grants, verify seals, read audit logs) without connecting to a public network. The ledger must support a fully offline local replica or a local-only mode.
2. **Deliverable on a flashdrive.** The FERROS profile, grants, and audit log must be portable on offline storage. A ledger model that requires a live network node to verify local state violates this constraint.
3. **Signed grants exist without on-chain anchoring.** The current `SignedProfileDocument` v0 model already provides tamper-evident signed grants. A ledger layer is additive (it provides revocation finality and marketplace settlement), not a prerequisite for the basic model.

### Options considered

| Option | Summary | FERROS invariant fit | Reason not chosen as default |
|--------|---------|----------------------|------------------------------|
| Non-chain signed ledger (chosen) | Local append-only signed event log, replicable to peers | Full fit — works offline, flashdrive-portable, additive to current model | — |
| Solana | High-throughput L1; small on-chain state cost | Partial fit — requires network for finality; offline operation requires local snapshot tooling not yet evaluated | High operational complexity for v0.1.0–v0.2.0; network dependency not yet justified |
| EVM L2 (Optimism, Arbitrum, Base, etc.) | Low-cost EVM-compatible rollup | Partial fit — same network-dependency issue as Solana; EVM tooling is browser-friendly but adds a dependency stack (ADR-021 concern) | Rollup withdrawal windows and bridge assumptions conflict with flashdrive-portability invariant at current scope |
| Cosmos app-chain | Sovereign app-specific chain; IBC-compatible | Partial fit — strong sovereignty story, but operating a Cosmos validator set is a significant infrastructure commitment | Premature for v0.1.0–v0.2.0; revisit if FERROS Hub grows to a multi-operator network |
| No ledger (status quo held indefinitely) | Local signed JSON only; no cross-device or marketplace ledger path | Full fit for current scope | Does not support marketplace settlement or cross-device recovery without a trusted server |

### Why the signed-ledger model is recommended over public-chain for now

The current threat model (`THREAT-MODEL.md`) focuses on local device security and consent-gate integrity, not on public verifiability or trustless marketplace settlement. Introducing a public-chain dependency before those requirements are confirmed would add operational complexity, network dependency, and a new external trust relationship that FERROS's sovereignty model has not yet evaluated.

The signed-ledger model preserves optionality: if FERROS adopts Solana or an EVM L2 for marketplace settlement in the v0.3.0+ window, the signed-event format can be anchored to a public chain without redesigning the local model.

### S6 harvest context

ADR-018 (botgen-rust), ADR-019 (workpace-rust), and ADR-020 (sheetgen-rust) each identified ledger-adjacent patterns in the prior art. This ADR's recommendation means those patterns should be evaluated against the non-chain signed-ledger model first; any harvest that implies a public-chain write should be flagged as a future-escalation item, not a v0.1.0–v0.2.0 implementation item.

---

## Consequences

**Positive:**
- FERROS remains fully locally sovereign and flashdrive-portable through the v0.2.0 window.
- No new external dependencies (chains, RPCs, wallets) are added to the dependency manifest.
- The decision is made at the policy level, so S6 and S7 have a reference answer for ledger-touching harvest and bridge work.
- The `SignedProfileDocument` v0 boundary already provides tamper-evidence; this ADR confirms it is sufficient for the current gate path.
- Public-chain options remain available for future escalation; this ADR does not foreclose them.

**Negative / trade-offs:**
- Marketplace settlement and cross-device recovery without a trusted server require the non-chain signed-ledger model to be designed and implemented before those features can ship — that work is not trivial.
- Revisiting this ADR to accept a public-chain model in the v0.3.0+ window will require re-evaluating the dependency manifest, operational posture, and threat model at that time.
- "Proposed" status means implementation of the signed-ledger layer should not begin until this ADR moves to Accepted via human review.

---

## Compliance

- If FERROS introduces a marketplace or cross-device recovery feature that requires public verifiability, this ADR must be revisited before implementation begins.
- If any stream adds a public-chain write (Solana, EVM, Cosmos) as a dependency, that addition must reference this ADR as either an accepted escalation decision or evidence that this ADR has been superseded.
- If the signed-ledger model design begins, it must satisfy all three FERROS invariants listed in the Rationale section.
- Cross-check against ADR-021 before adding any new chain SDK or wallet dependency.
