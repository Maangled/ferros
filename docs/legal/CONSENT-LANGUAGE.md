# FERROS Consent Language

> **Status: DRAFT — Awaiting counsel red-line. This document does not constitute legal advice or final consent language. It is a structured placeholder for legal review.**

**Last updated:** 2026-04-27  
**Version:** 0.1-draft

---

## Purpose

This document describes the consent model FERROS uses and proposes plain-English language for consent surfaces — capability grants, onramp accept steps, and audit log disclosures. It is organized so that counsel can red-line individual sections without needing to understand the full codebase.

---

## Core Consent Principles

FERROS is built on three consent invariants. These are design commitments, not contractual representations, until this document is reviewed by counsel.

1. **Consent is explicit, not ambient.** Connecting an integration (Home Assistant, calendar, social-graph import) does not itself constitute consent to use the imported data as canonical FERROS state. An explicit accept step is required.

2. **Consent is auditable.** Every consent event — capability grant issuance, onramp data acceptance, revocation — is logged locally. The log is readable by the user at any time.

3. **Consent is revocable.** A capability grant can be revoked. Revocation is not a deletion of history; it is an explicit event in the audit log that downstream consumers can observe. Whether revocation affects already-processed data is a product and legal question.

> _Red-line target: confirm whether these three principles are correctly stated as design commitments vs. enforceable terms. Identify any jurisdiction where "consent is revocable" requires specific additional language (e.g., GDPR right to withdrawal)._

---

## Capability Grant Language

When a FERROS user issues a capability grant, the following language (or a counsel-approved variant) should appear on the consent surface:

> **You are granting [agent name or capability description] the ability to [capability description]. This grant is stored locally and can be revoked at any time from the consent log. Granting this capability does not share your data with any third party unless you explicitly configure an external integration.**

> _Red-line target: confirm this language is sufficient. Advise on whether "capability" requires a legal definition in terms of use. Advise on whether the phrase "does not share your data with any third party" can be warranted._

---

## Onramp Accept Language

When a user accepts incoming data from an external system (Home Assistant, calendar, social-graph export), the following language (or a counsel-approved variant) should appear:

> **You are accepting [data description] from [external system name] into your FERROS profile. This data will become part of your local FERROS state. You can review or remove it from your profile at any time. Accepting this data does not affect [external system name] — it remains in your local FERROS installation only.**

> _Red-line target: confirm "accepting this data does not affect [external system name]" is accurate for all integration types. Flag if this needs a carve-out for bidirectional integrations._

---

## Audit Log Disclosure

Users should understand what the audit log contains before relying on it. Proposed disclosure:

> **Your FERROS audit log records consent events: grants, revocations, onramp accepts, and deny events. The log is stored locally on your device. FERROS does not transmit log contents to any remote server. The log is append-only: events are not deleted, though you may export or migrate it.**

> _Red-line target: confirm "append-only" and "not deleted" are accurate given the current implementation. Flag if GDPR or similar regulations require the ability to delete audit log entries._

---

## Denial and Deny-by-Default

FERROS enforces deny-by-default: an agent that does not have an explicit capability grant is denied, and the denial is logged. Users should be informed that:

> **FERROS denies capability requests by default. If an agent you expect to work is not functioning, check the deny log. A denial is not permanent — you can issue a capability grant at any time.**

> _Red-line target: confirm this framing is accurate. Flag if deny-by-default needs additional disclosure in jurisdictions that require affirmative consent for data processing._

---

## Open Consent Questions for Counsel

The following questions are explicitly unresolved and require counsel input before the v0.1.0 launch tag:

1. Does the FERROS consent model constitute "consent" in the legal sense under GDPR, CCPA, or other applicable regulations?
2. Is the audit log sufficient as a "record of consent" or does it require additional metadata (timestamp, user identifier, purpose)?
3. Does "revocable consent" require that FERROS guarantee rollback of already-processed data, or is a prospective-only revocation sufficient?
4. What consent language is required for minors using FERROS on shared or family devices?
5. Does the onramp accept step (importing HA entity state, calendar data, etc.) require jurisdiction-specific language?

---

_This document is a structured placeholder. It proposes consent language for FERROS surfaces so that counsel has a starting surface for red-line. It does not constitute a legal document, legal advice, or binding consent terms._

---

## S5 surface consumer-awareness

S5 (UX stream) is the first consumer of the draft consent language sections in this document. The consent-flow copy spec is defined in `streams/S5-ux/README.md` (Phase B: consent-flow copy spec section) and derives copy from the following sections of this document:

- **Capability Grant Language** — used as the capability grant consent slot copy.
- **Onramp Accept Language** — used as the onramp accept consent slot copy.
- **Denial and Deny-by-Default** — used as the deny-visibility disclosure copy.

The S5 copy spec carries the same DRAFT status as this document. When counsel clears the draft status on this document, the copy spec in `streams/S5-ux/README.md` must be updated in the same coordinated pass. S5 does not modify the consent language in this document; it is a consumer that must use counsel-approved variants once they are available.
