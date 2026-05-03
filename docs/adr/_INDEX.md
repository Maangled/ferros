# FERROS ADR Index

This index groups the ADR series by domain and provides the navigation surface for the decision program.

Use this file when you need to answer one of three questions quickly:

1. Which ADRs define the current doctrine or constraints for my stream?
2. Which ADRs are nearby in topic, even if they were authored by another stream?
3. Which records are decisions versus active investigation?

For program rules, see [ADR-022-decision-program-governance.md](./ADR-022-decision-program-governance.md). For the root evaluation rubric, see [../../DOCTRINE.md](../../DOCTRINE.md).

---

## Program Surfaces

- [ADR-TEMPLATE.md](./ADR-TEMPLATE.md) — template for new ADRs
- [ADR-022-decision-program-governance.md](./ADR-022-decision-program-governance.md) — taxonomy, statuses, evidence bases, and research-lane rules
- [_ROADMAP.md](./_ROADMAP.md) — anticipated future decision topics
- [_RESEARCH-NOTES/README.md](./_RESEARCH-NOTES/README.md) — research-note lane guidance
- [_EVIDENCE/README.md](./_EVIDENCE/README.md) — evidence-record guidance

---

## Foundational and Governance Records

| ADR | Topic | Why it matters |
|-----|-------|----------------|
| [ADR-0001](./ADR-0001-start-new-do-not-fork.md) | Start new, do not fork | Founding governance boundary for FERROS ownership and implementation posture |
| [ADR-002](./ADR-002-smart-contract-boundaries.md) | Smart contract boundaries | Limits where on-chain or contract logic belongs |
| [ADR-012](./ADR-012-schema-evolution-cascade.md) | Schema evolution cascade | Controls how contracts and fixtures evolve over time |
| [ADR-021](./ADR-021-dependency-admission-policy.md) | Dependency admission policy | Defines the current browser and Rust dependency posture |
| [ADR-022](./ADR-022-decision-program-governance.md) | Decision-program governance | Defines the ADR taxonomy, evidence bases, and research lane |
| [ADR-023](./ADR-023-onramp-policy.md) | Onramp policy — external systems are onramps, not identity truth | Establishes the data-direction invariant: all inbound data from HA, calendar, social-graph, and bundle pipelines is staged and requires explicit user consent before becoming canonical FERROS state |
| [ADR-024](./ADR-024-ledger-substrate.md) | Ledger/chain substrate comparison and recommendation | Evaluates Solana, EVM L2, Cosmos app-chain, and non-chain signed-ledger options; recommends non-chain signed ledger for v0.1.0–v0.2.0; holds public-chain as a future escalation path |
| [ADR-027](./ADR-027-service-parity-broker-and-assurance-tiers.md) | Service parity broker and assurance tiers | Extends ADR-023 with provider-parity, detachable association, and transaction-scoped assurance requirements |

---

## Identity, Consent, and Onboarding

| ADR | Topic | Why it matters |
|-----|-------|----------------|
| [ADR-001](./ADR-001-progression-lock-pattern.md) | Progression-lock pattern | Tamper-evident local progression and seal-chain baseline |
| [ADR-003](./ADR-003-alias-system.md) | Alias system | Pseudonymous identity on foreign devices |
| [ADR-004](./ADR-004-template-profile-specification.md) | Template profile specification | Template profile and gallery schema baseline |
| [ADR-005](./ADR-005-cross-device-identity-and-session-modes.md) | Cross-device identity and session modes | Defines the mutually exclusive session-mode model |
| [ADR-006](./ADR-006-level-zero-adaptive-onboarding.md) | Level-zero adaptive onboarding | Current onboarding doctrine and progressive reveal behavior |
| [ADR-011](./ADR-011-routine-module-system.md) | Routine module system | Applies card/deck vocabulary to routine composition and bag state |

---

## Architecture and Runtime

| ADR | Topic | Why it matters |
|-----|-------|----------------|
| [ADR-007](./ADR-007-single-file-system.md) | Single file system | Long-horizon cards-and-decks execution worldview |
| [ADR-008](./ADR-008-modular-rendering-system.md) | Modular rendering system | Shared surface primitives and rendering composition |
| [ADR-014](./ADR-014-three-layer-decomposition.md) | Three-layer decomposition | Domain, surface, and storage/runtime separation |
| [ADR-025](./ADR-025-dual-root-hardware-runway.md) | Dual-root hardware runway and ignition-lane architecture | Accepted framework for Fastest/FERROS dual-root organization, family-specific lane profiles, and a provisional S9 Ignition lane bounded by explicit non-claims |

---

## UX, Surface, and Asset Doctrine

| ADR | Topic | Why it matters |
|-----|-------|----------------|
| [ADR-009](./ADR-009-four-corner-docking-layout.md) | Four-corner docking layout | Canonical shell zoning for workbenches and surfaces |
| [ADR-010](./ADR-010-cards-and-decks-nomenclature.md) | Cards and decks nomenclature | Universal card, deck, and bag language |
| [ADR-015](./ADR-015-universal-parametric-authoring-workbench.md) | Universal parametric authoring workbench | Forge as the base pattern across domains |
| [ADR-016](./ADR-016-arena-export-target.md) | Arena export target | Arena-facing projection direction |
| [ADR-017](./ADR-017-html-surface-incubation-strategy.md) | HTML surface incubation strategy | Treats current HTML artifacts as incubators, not frozen product architecture |

---

## Ecosystem and Prior-Art Integration

| ADR | Topic | Why it matters |
|-----|-------|----------------|
| [ADR-013](./ADR-013-legacy-integration-strategy.md) | Legacy integration strategy | Governs how prior art is consumed |
| [ADR-018](./ADR-018-harvest-botgen.md) | Harvest botgen-rust | S6 prior-art boundary for agent patterns |
| [ADR-019](./ADR-019-harvest-workpace.md) | Harvest workpace-rust | S6 prior-art boundary for shell and workspace patterns |
| [ADR-020](./ADR-020-harvest-sheetgen.md) | Harvest sheetgen-rust | S6 prior-art boundary for data-layer invariants |

---

## Research Lane

Research notes capture high-value investigation before a decision is frozen.

- [_RESEARCH-NOTES/README.md](./_RESEARCH-NOTES/README.md) — process and naming rules
- [RN-2026-04-acc-card-deck-projection.md](./_RESEARCH-NOTES/RN-2026-04-acc-card-deck-projection.md) — current ACC object-to-card projection work
- [RN-2026-05-voting-decision-models.md](./_RESEARCH-NOTES/RN-2026-05-voting-decision-models.md) — preserved voting taxonomy and tally patterns from `workpace-rust` without freezing FERROS governance semantics

---

## Quick Routes by Current Need

- Profile, grants, consent, or session model work: ADR-001, ADR-002, ADR-003, ADR-005, ADR-011, ADR-012
- Shell, ACC, or user-surface work: ADR-009, ADR-010, ADR-015, ADR-017, plus the ACC research note
- Runtime and contract layering: ADR-007, ADR-014, ADR-021
- Prior-art or harvested-pattern questions: ADR-013, ADR-018, ADR-019, ADR-020
- Governance, doctrine, or decision-process work: ADR-0001, ADR-021, ADR-022, and [../../DOCTRINE.md](../../DOCTRINE.md)
