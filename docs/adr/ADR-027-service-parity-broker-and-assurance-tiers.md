# ADR-027 — Service Parity Broker and Assurance-Tiered Onramps

**Status:** Proposed  
**Date:** 2026-05-02  
**Stream:** Cross-cutting (S2/S5/S7/S8)  
**Deciders:** FERROS stream coordination / S8 docs / S2 identity / S7 hub / S5 UX  
**Domain tags:** policy / governance / security / real-world application / cross-cutting  
**Primary evidence basis:** Research or precedent proof

_See [ADR-022-decision-program-governance.md](./ADR-022-decision-program-governance.md), [ADR-023-onramp-policy.md](./ADR-023-onramp-policy.md), and [../../DOCTRINE.md](../../DOCTRINE.md)._

---

## Context

ADR-023 established that external systems are onramps, not identity truth. That policy prevents
silent authority transfer from FERROS to external providers, but it does not yet define two
critical implementation questions:

1. How FERROS should treat duplicated capability across competing external services (for example,
   music links, calendar providers, identity providers, cloud-drive providers) without trapping
   users in any single vendor.
2. How FERROS should classify trust posture for imported identities and accounts when some
   integrations are proprietary, some are partially auditable, and some are fully open,
   reproducible, and inspectable.

The practical requirement is to support real-world bridges while preserving user sovereignty and
clear exit paths.

---

## Decision

**FERROS adopts a Service Parity Broker pattern for external integrations and an assurance-tier
model for identity-sensitive actions, extending ADR-023 without weakening its consent gate.**

Supporting rules:

- **Parity before lock-in.** FERROS should normalize equivalent external-service capabilities into
  one user-facing action surface whenever feasible.
- **Association is reversible.** Any external account association that influences profile or
  capability state must provide a documented detach path and must not prevent migration to a
  FERROS-native credential path.
- **Tier is action-scoped.** Assurance requirements apply to the transaction being executed, not
  only to the baseline account.
- **High-trust actions require high-assurance context.** Actions designated as trust-critical
  (for example governance voting, irreversible grants, or high-impact state changes) require an
  S-tier transaction context regardless of the user's default login provider.
- **No single-provider truth.** A proprietary provider may be acceptable for convenience-tier
  entry, but cannot become the sole trust root for critical operations.

---

## Rationale

External bridges are necessary for adoption. Users often arrive with existing providers and need
continuity. The platform still needs to preserve sovereign control, provider optionality, and a
clear path toward stronger assurance.

Service parity and assurance tiers solve different problems that frequently appear together:

- Parity prevents experience-level lock-in by ensuring "what you can do" is abstracted from
  "which provider you used".
- Assurance tiers prevent trust-level confusion by making high-impact operations require a
  stronger, auditable transaction context.

### Options considered

| Option | Summary | Reason not chosen (if rejected) |
|--------|---------|--------------------------------|
| Service parity + assurance tiers (chosen) | Normalize capabilities across providers and gate critical actions by transaction assurance | — |
| Single-provider convenience model | Prefer one mainstream provider and optimize onboarding around it | Rejected: creates lock-in and weakens sovereign detach/migration posture |
| Uniform trust model for all actions | Treat all operations as equivalent regardless of impact | Rejected: high-risk operations need stronger guarantees than basic onboarding |

---

## Assurance Tier Model

Initial policy labels:

- **A-tier context:** accepted convenience integrations with constrained trust assumptions;
  suitable for low-risk, reversible operations.
- **S-tier context:** strong assurance posture for trust-critical operations, favoring open,
  inspectable, and auditable paths with explicit local evidence where possible.

Notes:

- These labels are policy shorthand and do not accuse or endorse any specific provider.
- A user may hold an A-tier baseline account and still execute an S-tier transaction by meeting
  additional step-up requirements for that operation.

---

## Consequences

**Positive:**
- Users retain practical integration options without surrendering sovereignty.
- Provider detachment and migration become explicit requirements rather than best effort.
- Trust-critical actions gain a clear assurance gate aligned with FERROS doctrine.
- S5 can present one consistent action surface even when multiple providers are connected.

**Negative / trade-offs:**
- Integration work increases because capability normalization and detach paths must be designed.
- Tiered UX adds complexity, especially where step-up assurance is required mid-flow.
- Some providers may not support all parity or export requirements cleanly.

---

## Compliance

- If an integration cannot be detached without profile loss, it is non-compliant with this ADR.
- If a trust-critical action can execute without the required assurance tier, it is non-compliant.
- If provider-specific surfaces bypass the broker and become de facto truth sources, revisit this
  ADR and ADR-023.
- Tier classification criteria and exact step-up mechanics may evolve, but any change must remain
  explicit and auditable.

---

## Deferred Scope

- Concrete cryptographic step-up protocol and hardware requirements.
- Full rubric for classifying actions into assurance tiers.
- Export/import profile portability contracts across all provider classes.

---

## References

- [ADR-022](./ADR-022-decision-program-governance.md)
- [ADR-023](./ADR-023-onramp-policy.md)
- [RN-2026-05-voting-decision-models.md](./_RESEARCH-NOTES/RN-2026-05-voting-decision-models.md)