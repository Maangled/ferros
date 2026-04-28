# DOC-BATCH-2026-04-27-C — Code-Track Batch Mode Run C

**Date:** 2026-04-27  
**Track:** code  
**Batch ID:** BATCH-2026-04-27-C  
**Waves:** WAVE-2026-04-27-06, WAVE-2026-04-27-07, WAVE-2026-04-27-08 (3 of 8 declared; queue exhausted)  
**Gatekeeper verdict:** **clean pass**

---

## §1 — Preamble

This batch landed the three ADR-023-informed code-track waves that were declared in the session following BATCH-2026-04-27-B (system-track) and the ceiling-lift substrate wave. The ceiling-lift wave (Interactive, S8) preceded this batch and raised the editing-lane ceiling from 5 to 8 with a revert clause; that change is not part of this batch's verdict but is recorded in the run log.

---

## §2 — Waves landed

| Wave | Title | Anchor files | Gatekeeper |
|------|-------|--------------|-----------|
| WAVE-2026-04-27-06 | Define S5 onramp consent surface entry bar | `streams/S5-ux/README.md`, `streams/S5-ux/BACKLOG.md`, `docs/adr/ADR-023-onramp-policy.md` | `continue` |
| WAVE-2026-04-27-07 | S7 HA bridge consent-mapping note above ADR-023 onramp framing | `streams/S7-hub/README.md`, `streams/S7-hub/BACKLOG.md`, `docs/hub/pack-b-bring-up-worksheet.md`, `docs/adr/ADR-023-onramp-policy.md` | `continue` |
| WAVE-2026-04-27-08 | S5 consent-flow copy spec derived from legal scaffold CONSENT-LANGUAGE.md | `streams/S5-ux/README.md`, `streams/S5-ux/BACKLOG.md`, `docs/legal/CONSENT-LANGUAGE.md` | `stop-clean` |

All `get_errors` checks clean. No crate, schema, harness, or CI workflow file touched. No frozen schema mutated. No gate moved or claimed.

---

## §3 — Contracts stable

- **ADR-023** (Accepted): Referenced by all three waves. Consumer-awareness notes added for S5 (wave -06) and S7 (wave -07). ADR-023's decision, rationale, and five supporting rules were not modified in any wave.
- **ADR-024** (Proposed — pending ratification): Not touched. Held at Proposed as directed.
- **Frozen schemas** (`schemas/profile.v0.json`, `schemas/capability-grant.v0.json`): Not touched.
- **S3/S4 JSON/RPC read-first contract surface**: Not touched.
- **G1/G2/G3 closed evidence wording**: Not touched.

---

## §4 — Stream phases advanced

**S5 (UX stream):**
- Phase B now has three additional scope-definition sections: the lifecycle control bar (from batch-A), the profile surface entry bar (from batch-A), the onramp consent surface entry bar (wave -06), and the consent-flow copy spec (wave -08).
- Backlog has two new unchecked items: land the onramp consent surface (wired), clear draft status on consent-flow copy spec once counsel review completes.
- S5 is the named consumer of ADR-023's quarantine-until-accepted invariant (for the onramp surface) and of `docs/legal/CONSENT-LANGUAGE.md` draft sections (for the copy spec).

**S7 (Smart-Home Hub stream):**
- README now carries the "HA bridge onramp mapping (ADR-023)" section: HA entities arrive as proposed FERROS material, must route through S5 onramp surface before becoming canonical state. Bridge protocol details remain S7-owned decisions, not constrained by this mapping note.
- Backlog has the ADR-023 mapping item checked.
- Pack B bring-up worksheet now carries the "ADR-023 onramp mapping note" section: HA entity registration steps in the worksheet are onramp events, not direct canonical state changes; the "HA entity registered" evidence field in the operator surface records staged material, not accepted-and-canonical material.

---

## §5 — Gatekeeper verdict

**Verdict: clean pass.**

All three waves landed with clean `get_errors` validation, no overrun, no frozen surface touched, no gate moved, and non-trivial gatekeeper decisions (continue / continue / stop-clean). There are no named ambiguities in this batch.

The batch closed at 3 of the declared 8-wave capacity. Queue exhaustion is not an anomaly — the batch was scoped to the three ADR-023-informed waves and no further Ready waves were queued.

---

## §6 — What waits

| Item | Status | Next action |
|------|--------|-------------|
| ADR-024 ratification | Proposed — pending (a) counsel review of `docs/legal/` scaffold and (b) explicit ratification turn | Human ratification turn when counsel review of `docs/legal/TERMS-OF-USE.md`, `LICENSING-POSTURE.md`, `CONSENT-LANGUAGE.md` is complete |
| CONSENT-LANGUAGE.md draft clearing | Draft — awaiting counsel red-line | Coordinated update with `streams/S5-ux/README.md` consent-flow copy spec when counsel clears the draft |
| S5 onramp consent surface (wired) | Unchecked in backlog | Code-backed follow-up after S3/S4 audit-log seam can capture an explicit accept event for a staged onramp item |
| S5 lifecycle control bar (wired) | Unchecked in backlog | Code-backed follow-up, harness gate required before landing |
| S5 minimum profile surface (wired) | Unchecked in backlog | Code-backed follow-up, harness gate required before landing |
| Hardware track | Parked | User names hardware session window before hardware-track waves begin |
| Ceiling revert monitoring | Revert clause active | If any subsequent batch fails (Triage/Trace escalation, frozen-surface touch, or halt before final wave), ceiling reverts to 5 in next substrate-refinement wave |

---

## §7 — Next human decision

No immediate decision required. The batch is closed, all ambiguities are resolved or named and parked.

Suggested next decision points:
1. **ADR-024 ratification:** when `docs/legal/` counsel review is complete, issue an explicit ratification turn to move ADR-024 from Proposed to Accepted.
2. **Hardware track window:** name the hardware session window when ready; hardware-track waves are staged in `docs/orchestration/HARDWARE-QUEUE.md`.
3. **Next code-track batch:** if further docs-only ADR-023-informed or D1 runway work is desired, queue the next batch of S waves against the current ceiling-8 capacity.
