# Doc-Batch Summary — 2026-04-27 (System Track)

**Batch ID:** BATCH-2026-04-27-B  
**Track:** system  
**Batch type:** System-track Batch Mode run (second batch; first on system track)  
**Predecessor:** Substrate-refinement wave (Interactive, S8) — landed before this batch opened  
**Waves landed:** 3 (SYSTEM-2026-04-27-01, SYSTEM-2026-04-27-03, SYSTEM-2026-04-27-02)  
**Run order:** -01 → -03 → -02 (user-specified)  
**Gatekeeper verdict:** Conditional pass (see §4)

---

## 1. Substrate-refinement wave (pre-batch, Interactive Mode)

Landed before the system-track batch opened. This wave is the human-approved action item from BATCH-2026-04-27 (code-track conditional pass).

**What landed in `docs/orchestration/BATCH-MODE.md`:**
1. Stop condition 3 now has an "Operational bookkeeping exemption" sub-section listing the six exempt surfaces: `WAVE-QUEUE.md`, `WAVE-RUN-LOG.md`, `SYSTEM-QUEUE.md`, `HARDWARE-QUEUE.md`, `doc-batches/*.md`, and **owner-stream `PROGRESS.md` only** (non-owner stream PROGRESS.md remains overrun). The overrun-trigger list is explicit.
2. The Gatekeeper Agent Contract section now carries the structured block format schema as a stable JSON block with model-swap migration note.
3. A new "Batch-Level Verdict Criteria" section documents clean-pass, conditional-pass, and fail definitions — including the requirement that a clean pass must show at least one non-trivial gatekeeper decision.

**What landed in `docs/orchestration/LOCAL-DRIVER.md`:**
- A new "Gatekeeper model intent" section documents the inline-self-review posture as a known limitation and states the intent to migrate to a dedicated small-tier model when mechanically available, with the structured block as the stable handoff contract.

**Validation:** `get_errors` clean on both substrate files. No crate, schema, frozen surface, or gate file touched.

---

## 2. Waves landed

### SYSTEM-2026-04-27-01 — ADR-023: Onramp policy

**Status:** Accepted  
**File:** `docs/adr/ADR-023-onramp-policy.md`  
**ADR index:** updated — ADR-023 added to Foundational and Governance Records table

**What the ADR establishes:**
- All data entering FERROS from external systems (Home Assistant, calendar, social-graph exports, bundle/migration pipelines) is treated as proposed material requiring explicit user consent before becoming canonical FERROS state.
- Five supporting rules: inbound data quarantined until accepted; consent is explicit and auditable (not implicit from having an integration enabled); external system does not define identity; import pipelines must declare a direction; HA bridge implementation details are not constrained by this ADR.
- The consent-event audit log (already in the S3/S4 layer) is the natural capture point for onramp accept events.
- Cross-referenced: ADR-021 (dependency posture), ADR-013 (legacy integration boundary).

**Consumer awareness:**
- S7: HA bridge implementation is not constrained by ADR-023, but any bridge that imports HA entity state must route it through a consent gate before that state becomes canonical FERROS state.
- S2: social-graph and calendar import paths (if ever built) must follow the same onramp accept invariant.

---

### SYSTEM-2026-04-27-03 — Legal scaffold

**Files:** `docs/legal/TERMS-OF-USE.md`, `docs/legal/LICENSING-POSTURE.md`, `docs/legal/CONSENT-LANGUAGE.md`  
**All files:** marked DRAFT, marked as not constituting legal advice.

**TERMS-OF-USE.md** covers: nature of software (locally sovereign, not a hosted service), no-warranty disclaimer, consent-model design commitments (not contractual representations), user responsibilities, no-personal-data-processing statement, IP/license coverage, governing-law placeholder. Red-line targets marked throughout.

**LICENSING-POSTURE.md** covers: artifact-class → license mapping table (Rust code, docs, schemas, HTML harnesses, stream planning docs), dual-licensing intent question, CLA question, trademark registration question, schema-file licensing question.

**CONSENT-LANGUAGE.md** covers: three core consent principles (explicit, auditable, revocable), proposed plain-English language for capability grants / onramp accept / audit-log disclosure / deny-by-default notices, five open consent questions for counsel (GDPR fit, audit-log metadata, revocation scope, minors, jurisdiction-specific onramp language).

---

### SYSTEM-2026-04-27-02 — ADR-024: Ledger/chain substrate

**Status:** Proposed (not Accepted — requires human review before implementation begins)  
**File:** `docs/adr/ADR-024-ledger-substrate.md`  
**ADR index:** updated — ADR-024 added to Foundational and Governance Records table

**Evaluation:** Four options scored against three FERROS invariants (locally sovereign, flashdrive-portable, signed grants exist without on-chain anchoring):

| Option | Invariant fit | Outcome |
|--------|--------------|---------|
| Non-chain signed ledger | Full fit | **Recommended for v0.1.0–v0.2.0** |
| Solana | Partial | Held as future escalation path |
| EVM L2 | Partial | Held as future escalation path |
| Cosmos app-chain | Partial | Held as future escalation path |
| Status quo (local JSON only) | Full fit | Not sufficient for marketplace/cross-device goals |

**Recommendation:** non-chain signed ledger as default; public-chain as future escalation path contingent on confirmed marketplace or cross-device requirements. The `SignedProfileDocument` v0 boundary is sufficient for the v0.1.0–v0.2.0 gate path.

**S6 directive:** harvest work should evaluate ledger-adjacent prior-art patterns against the non-chain signed-ledger model; any pattern that implies a public-chain write is a future-escalation item.

---

## 3. Contracts stable

No contracts moved in this batch. The following frozen surfaces were referenced but not modified:

- `schemas/profile.v0.json` — unchanged
- `schemas/capability-grant.v0.json` — unchanged
- G1/G2/G3 closed evidence wording — unchanged
- `crates/ferros-agents/src/rpc.rs` read-first JSON/RPC contract — unchanged

`docs/contracts/CONTRACTS-OVERVIEW.md` was not touched.

---

## 4. Gatekeeper verdict: Conditional pass

**All three system-track waves landed clean.** Gatekeeper returned `continue` after waves 1 and 2, `stop-clean` after wave 3 (queue exhausted). No validation failures, no frozen surface touched, no escalation, no overrun fires under the ratified narrowed rule.

**Gatekeeper non-trivial decision noted:** ADR-024 carries Status: Proposed (not Accepted), which is the correct choice for a recommendation-only ADR pending human review. The gatekeeper explicitly evaluated whether this constituted a stop condition (it does not — the ADR constraints say "Recommendation only, not a binding commitment"), and correctly returned `continue`. This is the first non-trivial gatekeeper decision in the system-track batch.

**Verdict: Conditional pass.**

**Named ambiguity (for audit trail):** ADR-024 (`docs/adr/ADR-024-ledger-substrate.md`) is held at Proposed pending (a) counsel review of the `docs/legal/` scaffold and (b) an explicit ratification turn. The recommendation (non-chain signed ledger for v0.1.0–v0.2.0) is directionally accepted but is not a vetted commitment until those two conditions are met. This is the sole named ambiguity for this batch. Future readers auditing whether substrate ambiguities are being resolved over time should check whether ADR-024's Proposed status has been cleared before the v0.2.0 milestone.

---

## 5. Open items for human review

### ADR-024 ratification

ADR-024 is Proposed. To move it to Accepted, the human reviewer must confirm:
1. The non-chain signed-ledger recommendation is acceptable for the v0.1.0–v0.2.0 window.
2. The three FERROS invariants (locally sovereign, flashdrive-portable, signed grants exist) are correctly stated.
3. The public-chain options (Solana, EVM L2, Cosmos) are correctly held as future escalation paths.

If the reviewer has a different recommendation (e.g., Solana should be the default), update ADR-024 before the next batch that touches ledger-adjacent work.

### Legal scaffold next steps

The three `docs/legal/` files are structured placeholders. They require counsel review before v0.1.0 launch. No action is required from the human at this re-entry point unless a specific legal question has been resolved externally and should be incorporated.

---

## 6. Batch-level run summary

This is the second completed batch (code-track = 1, system-track = 2). Both batches produced conditional passes.

Per the approved ceiling-lift criteria: two clean/conditional batches → ceiling-lift is eligible after this batch.

**Ceiling-lift wave:** per the approved plan, queue the ceiling-lift wave now (raise editing-lane ceiling from 5 to 8 in `LOCAL-DRIVER.md`, citing BATCH-2026-04-27 and BATCH-2026-04-27-B as evidence). This is the human's decision — signal below.

---

## 7. Next human decision

Review this doc-batch and answer the following:

1. **Ratify ADR-024 recommendation.** Confirm non-chain signed ledger as the v0.1.0–v0.2.0 default, or provide a different recommendation. If confirmed, I will update ADR-024 status to Accepted.

2. **Confirm system-track verdict.** Conditional pass is the gatekeeper's read. If the ADR-024 Proposed status or the legal scaffolding surfaces an issue you want addressed before the verdict is confirmed, flag it.

3. **Ceiling-lift.** Two conditional-pass batches are now in the run log (code-track + system-track). Per the approved plan: signal yes/no on queuing the ceiling-lift wave now. If yes, it lands as a single Interactive-mode wave on `LOCAL-DRIVER.md`.

4. **Next batch or redirect.** Options:
   - Hardware track (3 Ready items: D1 device inventory, firmware spike plan, UX session script — HARDWARE-2026-04-27-02 is serial-after -01; you name the session window).
   - Refill the code-track queue for a third batch (code-track waves should now be informed by ADR-023 and ADR-024 outputs).
   - Interactive-mode wave for a specific item.
   - ADR-024 Accepted status update (if ratified above, this is a one-file Interactive wave).
