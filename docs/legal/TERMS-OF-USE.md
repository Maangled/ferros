# FERROS Terms of Use

> **Status: DRAFT — Awaiting counsel red-line. This document does not constitute legal advice or final terms. It is a structured placeholder for legal review.**

**Effective date:** Pending  
**Last updated:** 2026-04-27  
**Version:** 0.1-draft

---

## Purpose

This document describes the terms under which FERROS software and associated services may be used. It is provided as a structured draft for counsel review. No rights are waived, granted, or disclaimed beyond what FERROS's open-source license already establishes until counsel review and signature processes are complete.

---

## 1. Nature of the Software

FERROS is a locally sovereign, consent-first identity and capability platform. It runs on devices you control. It does not operate as a hosted service unless you explicitly configure and operate such a deployment yourself. The software is provided under the terms of the LICENSE file at the root of the repository.

---

## 2. No Warranty

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND. THIS PLACEHOLDER REPEATS THE WARRANTY DISCLAIMER IN THE OPERATIVE LICENSE. COUNSEL SHOULD CONFIRM WHETHER ADDITIONAL LANGUAGE IS REQUIRED FOR NON-SOFTWARE ELEMENTS (DOCS, SCHEMAS, HARNESSES).

> _Red-line target: confirm warranty scope extends correctly to documentation, schema files, and HTML harnesses._

---

## 3. Consent Model

FERROS is designed around explicit consent. When you use FERROS:

- You control what data is stored locally.
- You control what capability grants are issued.
- External system data (Home Assistant entities, calendar imports, social-graph exports) does not become canonical FERROS state without your explicit accept action. See ADR-023 for the governing policy.
- Consent events are logged locally and are auditable by you.

These are design commitments, not contractual guarantees, until this document is reviewed and finalized by counsel.

> _Red-line target: distinguish design commitments from contractual representations. Determine which commitments can be warranted._

---

## 4. User Responsibilities

You are responsible for:

- The security of devices on which FERROS runs.
- The accuracy of capability grants you issue.
- Any use of FERROS to integrate external systems (HA bridges, calendar feeds, social-graph imports).
- Compliance with any applicable law in your jurisdiction regarding identity storage, data processing, and system integration.

---

## 5. No Personal Data Processing by FERROS Project

FERROS software processes data locally on your device. The FERROS project does not receive, store, or process your personal data. If you operate a networked FERROS deployment (e.g., a localhost shell accessible beyond your device), you are responsible for that network boundary.

> _Red-line target: confirm this framing is accurate for any future hosted components (marketplace, update feed, etc.) and add appropriate carve-outs._

---

## 6. Intellectual Property

The FERROS software and documentation are licensed under the terms in the LICENSE and LICENSE-DOCS files. Schemas and harnesses follow the same license unless individually marked otherwise. No trademark rights are granted.

> _Red-line target: confirm trademark posture and whether "FERROS" requires registration before the v0.1.0 launch milestone._

---

## 7. Governing Law

> _Red-line target: Insert applicable governing law and jurisdiction clause after counsel review._

---

## 8. Changes to These Terms

These terms may be updated. The authoritative version is in the repository at `docs/legal/TERMS-OF-USE.md`. Until a release version is tagged and signed, this document is a draft.

---

_This document is a structured placeholder. It repeats the FERROS design posture in plain English so that counsel has a starting surface for red-line. It does not constitute a legal document, legal advice, or final terms of use._
