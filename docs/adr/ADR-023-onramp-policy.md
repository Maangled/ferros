# ADR-023 — External Systems Are Onramps, Not Identity Truth

**Status:** Accepted  
**Date:** 2026-04-27  
**Stream:** S8 primary; S7 consumer awareness; S2 consumer awareness  
**Deciders:** FERROS stream coordination / S8 docs / S7 hub / S2 profile  
**Domain tags:** policy / governance / real-world application / cross-cutting  
**Primary evidence basis:** Research or precedent proof

_See [ADR-022-decision-program-governance.md](./ADR-022-decision-program-governance.md), [_INDEX.md](./_INDEX.md), and [../../DOCTRINE.md](../../DOCTRINE.md). Cross-check against [ADR-021-dependency-admission-policy.md](./ADR-021-dependency-admission-policy.md) for consistency with the dependency and data-provenance posture._

---

## Context

FERROS is designed to be a locally sovereign, consent-first identity and capability layer. As S7 hub work advances toward a Home Assistant bridge, and as future streams might import calendar data, social-graph data (e.g., LinkedIn or Facebook export), or prior-art bundles from sheetgen-rust/botgen-rust, a recurring question arises: when data flows _into_ FERROS from an external system, who owns that data and when does it become canonical FERROS state?

Without an explicit policy ADR, individual streams risk making inconsistent assumptions. S7 might treat HA-imported entity state as authoritative; S2 might absorb a social-graph import as a profile field without a consent gate; S6 harvest work might promote prior-art data into the working profile without an explicit accept step. Each of these would violate the FERROS sovereignty principle in a different way.

The dependency admission policy (ADR-021) covers what code and packages FERROS pulls in. This ADR covers what _data_ FERROS pulls in from connected real-world systems — and what the invariant is before that data can become part of the canonical FERROS state.

---

## Decision

**All data entering FERROS from external systems — including Home Assistant, calendar providers, social-graph exports, and bundle or migration pipelines — is treated as proposed material requiring explicit user consent before it becomes canonical FERROS state. External systems are onramps, not identity truth.**

Supporting rules:

- **Inbound data is quarantined until accepted.** Data imported from an external system arrives in a staging area (which may be as minimal as a displayed diff or a preview surface). It does not automatically populate the profile, capability grants, or any sealed progression record.
- **Consent is explicit, not implicit.** A FERROS user must take an affirmative action to accept imported data. Passively having an integration enabled is not consent. The consent action must be auditable: it must be loggable as a consent event that can be reviewed later.
- **The external system does not define identity.** HA entity names, calendar UIDs, LinkedIn profile fields, and bundle manifest IDs may become _inputs_ to FERROS, but they do not become FERROS identity attributes without a FERROS-native accept step. The FERROS profile (governed by `schemas/profile.v0.json` and ADR-001/ADR-004) remains the canonical identity surface.
- **Import pipelines must declare a direction.** Every onramp integration must document the data-direction invariant: data flows from the external system into a FERROS staging area; FERROS-internal state does not flow back to the external system unless a separate export step is explicitly authorized.
- **HA bridge implementation details are not constrained by this ADR.** The pairing handshake order, HA entity schema, bridge protocol, and physical-device specifics remain S7-owned decisions. This ADR establishes the consent invariant that any bridge implementation must satisfy, not how the bridge works internally.
- **Bundle and migration pipelines are onramps.** Data arriving via `ferros profile import`, sheetgen-rust data extraction, botgen-rust pattern harvest, or workpace-rust workspace snapshots enters as proposed material. A preview-and-accept step is required before that data affects the sealed profile or progression state. See ADR-013 for the legacy integration boundary.

---

## Rationale

FERROS's sovereignty model requires that the user is always the decision-maker about what is canonical in their identity and capability state. Allowing an external system to silently push data into that state would be the canonical failure mode of federated identity systems: the identity provider becomes the authority instead of the user.

The onramp framing is not a technical constraint — it is a UX and trust invariant. It means that even a fully working HA bridge that reads real entity state must still route that state through a user-visible accept step before it becomes part of the operator's FERROS profile or progress evidence.

This constraint also applies to the S6 harvest work. Prior-art data from sheetgen-rust, botgen-rust, and workpace-rust is valuable, but it is historical material from a different system, not FERROS-native state. The harvest ADRs (ADR-018, ADR-019, ADR-020) established the lift/rewrite/discard boundary for _code_; this ADR establishes the equivalent boundary for _data_.

### Options considered

| Option | Summary | Reason not chosen (if rejected) |
|--------|---------|--------------------------------|
| External systems as onramps with explicit consent gate (chosen) | All inbound data is staged; user consent is required before canonical state is updated | — |
| Trusted source model | Certain external systems (e.g., HA entities, Google Calendar) are treated as authoritative inputs that automatically update FERROS state | Rejected: violates the sovereignty principle; the user becomes a downstream consumer of external authority rather than the root decision-maker |
| No policy; stream-by-stream decision | Each stream decides independently how to handle inbound data | Rejected: inconsistent treatment creates implicit federated-identity assumptions; S7, S5, and S2 would make different decisions about the same invariant |

---

## Consequences

**Positive:**
- The sovereignty principle is preserved end-to-end. No external system can silently become an identity authority.
- Stream implementations (S7 bridge, S5 profile surface, S6 harvest) have a clear invariant to design against rather than re-litigating the consent question mid-flight.
- The consent-event audit log (already established in the S3/S4 layer) becomes the natural capture point for onramp accept events, reusing infrastructure that already exists.
- Future onramp integrations (calendar, social graph, marketplace) have a repeatable pattern.

**Negative / trade-offs:**
- Every onramp integration must include a staging/preview and an accept step, which adds UX surface area even for simple integrations.
- Automatic sync — the pattern many connected-home and calendar systems assume — is explicitly ruled out without a consent step, which may surprise users coming from those systems.

---

## Compliance

- If FERROS adopts a fully automated sync model for any category of data, this ADR must be revisited to determine whether the sync constitutes implicit consent or requires a new consent model.
- If the `schemas/profile.v0.json` schema is extended to include HA-native or social-graph-native fields, this ADR must be reviewed to ensure the import boundary remains intact.
- If S7 implements a bridge that pushes data from FERROS back to HA without an explicit export step, that is a compliance violation against this ADR.
- Cross-check against ADR-021 whenever a new integration adds a new code or data dependency.

---

## S5 surface consumer-awareness

S5 (UX stream) is the onramp staging surface implementor under this ADR. The minimum honest onramp consent surface entry bar is defined in `streams/S5-ux/README.md` (Phase B: onramp consent surface entry bar section). That surface is the UX materialisation of this ADR's quarantine-until-accepted invariant. S5 does not modify this ADR's decision or rationale; it is a consumer that must satisfy the invariant, not an owner of it.

---

## S7 bridge consumer-awareness

S7 (Smart-Home Hub stream) is an onramp source under this ADR. HA entities discovered by the FERROS bridge arrive as proposed FERROS material and must route through the S5 onramp consent surface before becoming canonical FERROS profile or capability-grant state. The S7 onramp mapping is documented in `streams/S7-hub/README.md` (HA bridge onramp mapping section) and `docs/hub/pack-b-bring-up-worksheet.md` (ADR-023 onramp mapping note). S7 does not modify this ADR's decision or rationale; the bridge is a source of proposed material, not an authority over consent outcomes.
