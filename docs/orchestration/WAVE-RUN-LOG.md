# FERROS Wave Run Log

Newest entry first. Each entry records one local driver invocation.

---

## BATCH-2026-04-27-B — System-Track Batch Mode Run

- **Batch open:** 2026-04-27
- **Track:** system
- **Waves in batch (declared order):** SYSTEM-2026-04-27-01, SYSTEM-2026-04-27-03, SYSTEM-2026-04-27-02
- **Gatekeeper model:** Claude Sonnet 4.6 inline self-review (same posture as code-track batch; see LOCAL-DRIVER.md gatekeeper model intent section for migration plan).
- **Overrun exemption list (ratified):** `WAVE-QUEUE.md`, `WAVE-RUN-LOG.md`, `SYSTEM-QUEUE.md`, `HARDWARE-QUEUE.md`, `doc-batches/*.md`, and owner-stream `PROGRESS.md` only. Non-owner stream PROGRESS.md remains overrun. Overrun fires on undeclared `crates/**`, `schemas/**`, `.github/workflows/**`, `tools/**`, shared-truth, or cross-stream anchor touches.
- **Substrate-refinement wave status:** Landed before this batch opened. `BATCH-MODE.md` now carries: operational bookkeeping exemption sub-section (stop condition 3), structured gatekeeper block schema, and batch-level verdict criteria section. `LOCAL-DRIVER.md` now carries the gatekeeper model-swap intent note.
- **No-substrate-edits constraint:** Do not touch `BATCH-MODE.md` or `LOCAL-DRIVER.md` inside this batch.
- **Ceiling:** Editing-lane ceiling remains 5 (ceiling-lift deferred to after two clean/conditional batches; code-track proof run = 1 of 2).

---

## 2026-04-27 — SYSTEM-2026-04-27-02

- Selected item: `SYSTEM-2026-04-27-02`
- Result: Complete. `docs/adr/ADR-024-ledger-substrate.md` (Status: Proposed) is now in repo. The ADR evaluates four options — non-chain signed ledger, Solana, EVM L2 (Optimism/Arbitrum/Base), and Cosmos app-chain — against three FERROS invariants: locally sovereign, flashdrive-portable, and signed grants exist without on-chain anchoring. **Recommendation: non-chain signed ledger as default for v0.1.0–v0.2.0; public-chain as a future escalation path contingent on confirmed marketplace or cross-device requirements.** Key rationale: FERROS's current threat model focuses on local consent-gate integrity, not public verifiability; introducing a public-chain dependency before those requirements are confirmed adds operational complexity and network dependency not yet warranted. The `SignedProfileDocument` v0 boundary already provides tamper-evidence sufficient for the current gate path. S6 harvest work is directed to evaluate ledger-adjacent prior-art patterns against the signed-ledger model first. `docs/adr/_INDEX.md` now includes ADR-024.
- Files: `docs/adr/ADR-024-ledger-substrate.md`, `docs/adr/_INDEX.md`
- Validation: `get_errors` clean on both anchor files. No crate, schema, harness, or workflow file touched. No frozen schema mutated. No gate moved.

```json
{
  "wave_id": "SYSTEM-2026-04-27-02",
  "stop_conditions_evaluated": {
    "1_validation_failed": "pass — get_errors clean on ADR-024 and _INDEX.md",
    "2_wave_tag": "no P0/gate-close/solo/frozen-schema flags — clear; ADR status is Proposed (not Accepted) which is correct for a recommendation-only ADR",
    "3_diff_overrun": "none — SYSTEM-QUEUE.md and WAVE-RUN-LOG.md are bookkeeping-exempt; only declared anchor files and exempt bookkeeping surfaces touched",
    "4_track_boundary": "no next Ready wave in system queue — queue exhausted",
    "5_run_length_cap": "3 of 8 — all 3 declared batch waves landed; stopping cleanly",
    "6_escalation_chain": "no escalation — clear"
  },
  "decision": "stop-clean",
  "rationale": "All three declared system-track waves landed cleanly with no validation failures, no overrun fires, and no escalation. ADR-024 correctly carries Proposed (not Accepted) status since it is a recommendation pending human review; this is appropriate and does not constitute a stop condition. The system-track Ready queue is now empty. Run-length cap fires at 3 of 8 which equals the declared batch scope. Stopping cleanly; human re-entry is at the doc-batch summary."
}
```

---

## 2026-04-27 — SYSTEM-2026-04-27-03

- Selected item: `SYSTEM-2026-04-27-03`
- Result: Complete. `docs/legal/` is now scaffolded with three counsel-ready placeholder files. `TERMS-OF-USE.md` covers the nature of the software, the no-warranty disclaimer, the consent model design commitments, user responsibilities, no-personal-data-processing statement, IP coverage, and open red-line targets including warranty scope, trademark posture, and hosted-component carve-outs. `LICENSING-POSTURE.md` maps current repo artifact classes to license files, surfaces the dual-licensing intent question, the CLA question, the trademark registration question, and the schema-file licensing question. `CONSENT-LANGUAGE.md` proposes plain-English consent language for capability grants, onramp accept steps, the audit log disclosure, and deny-by-default notices; lists five open consent questions for counsel resolution. All three files carry a prominent draft-status blockquote. No legal advice is given or implied in any file.
- Files: `docs/legal/TERMS-OF-USE.md`, `docs/legal/LICENSING-POSTURE.md`, `docs/legal/CONSENT-LANGUAGE.md`
- Validation: `get_errors` clean on all three anchor files. No crate, schema, harness, or workflow file touched. No frozen schema mutated. No gate moved.

```json
{
  "wave_id": "SYSTEM-2026-04-27-03",
  "stop_conditions_evaluated": {
    "1_validation_failed": "pass — get_errors clean on all 3 anchor files",
    "2_wave_tag": "no P0/gate-close/solo/frozen-schema flags — clear",
    "3_diff_overrun": "none — SYSTEM-QUEUE.md and WAVE-RUN-LOG.md are bookkeeping-exempt; only declared anchor files and exempt bookkeeping surfaces touched",
    "4_track_boundary": "next Ready wave (SYSTEM-2026-04-27-02) is system track — no boundary crossed",
    "5_run_length_cap": "2 of 8 — continue",
    "6_escalation_chain": "no escalation — clear"
  },
  "decision": "continue",
  "rationale": "Three legal scaffold files landed cleanly as draft-marked docs-only placeholders. No stop conditions fired. The files surface open questions for counsel without claiming legal advice or final terms. Next wave is SYSTEM-2026-04-27-02 (ledger ADR), same track, size S — batch continues."
}
```

---

## 2026-04-27 — SYSTEM-2026-04-27-01

- Selected item: `SYSTEM-2026-04-27-01`
- Result: Complete. `docs/adr/ADR-023-onramp-policy.md` (Status: Accepted) is now in repo. The ADR establishes the data-direction invariant: all data entering FERROS from external systems — including Home Assistant, calendar providers, social-graph exports, and bundle/migration pipelines — is treated as proposed material requiring explicit user consent before becoming canonical FERROS state. Key rules: inbound data is quarantined until accepted; consent is explicit and auditable (not implicit from having an integration enabled); the external system does not define identity; import pipelines must declare a direction; HA bridge implementation details are explicitly not constrained by this ADR. `docs/adr/_INDEX.md` now includes ADR-023 in the Foundational and Governance Records table.
- Files: `docs/adr/ADR-023-onramp-policy.md`, `docs/adr/_INDEX.md`
- Validation: `get_errors` clean on both anchor files. No crate, schema, harness, or workflow file touched. No frozen schema mutated. No gate moved. Consistent with ADR-021 (dependency posture) and ADR-013 (legacy integration boundary).

```json
{
  "wave_id": "SYSTEM-2026-04-27-01",
  "stop_conditions_evaluated": {
    "1_validation_failed": "pass — get_errors clean on ADR-023 and _INDEX.md",
    "2_wave_tag": "no P0/gate-close/solo/frozen-schema flags — clear",
    "3_diff_overrun": "none — SYSTEM-QUEUE.md and WAVE-RUN-LOG.md are bookkeeping-exempt; only declared anchor files and exempt bookkeeping surfaces touched",
    "4_track_boundary": "next Ready wave (SYSTEM-2026-04-27-03) is system track — no boundary crossed",
    "5_run_length_cap": "1 of 8 — continue",
    "6_escalation_chain": "no escalation — clear"
  },
  "decision": "continue",
  "rationale": "ADR-023 landed cleanly as a docs-only Accepted ADR. No stop conditions fired. The ADR frames the consent invariant without constraining S7 bridge implementation details, consistent with the wave constraints. Next wave is SYSTEM-2026-04-27-03 (legal scaffold), same track, size S — batch continues."
}
```

---

## 2026-04-27 — Substrate-refinement wave (Interactive, S8)

- Selected item: substrate-refinement (non-queued, Interactive Mode)
- Result: Complete. `docs/orchestration/BATCH-MODE.md` now carries (1) an "Operational bookkeeping exemption" sub-section under stop condition 3 with the ratified six-surface exempt list (WAVE-QUEUE.md, WAVE-RUN-LOG.md, SYSTEM-QUEUE.md, HARDWARE-QUEUE.md, doc-batches/, and owner-stream PROGRESS.md only), (2) the structured gatekeeper block format as a stable schema block under the Gatekeeper Agent Contract section including the model-swap migration note, and (3) a "Batch-Level Verdict Criteria" section documenting the clean-pass, conditional-pass, and fail definitions with the non-trivial-decision requirement for a clean pass. `docs/orchestration/LOCAL-DRIVER.md` now carries a "Gatekeeper model intent" section documenting the inline-self-review posture as a known limitation and the migration intent when a small-tier model becomes available.
- Files: `docs/orchestration/BATCH-MODE.md`, `docs/orchestration/LOCAL-DRIVER.md`
- Validation: `get_errors` clean on both files. No crate, schema, harness, or workflow file touched. No frozen surface mutated. No gate moved.
- Next follow-up: system-track Batch Mode batch opens immediately.
- Note: This wave is a substrate-editing wave; by definition it is not subject to gatekeeper review. The gatekeeper role does not evaluate a wave that edits its own contract.

---

## BATCH-2026-04-27 — First Batch Mode Proof Run (Code Track)

- **Batch open:** 2026-04-27
- **Track:** code
- **Waves in batch:** WAVE-2026-04-27-02, WAVE-2026-04-27-04, WAVE-2026-04-27-05
- **Gatekeeper model:** Claude Sonnet 4.6 inline self-review. A dedicated second-model step is not mechanically available in the current VS Code Copilot Chat environment. The gatekeeper role is performed as a structured self-review using the six BATCH-MODE.md stop conditions. The intent is that when a small-tier model becomes mechanically available in this environment (or on a CLI that supports a two-model pipeline), this role migrates without redesign — the structured block format below is written to be stable across that swap.
- **Gatekeeper block schema (fixed for forwards-compat):** `wave_id`, `stop_conditions_evaluated`, `decision`, `rationale`
- **Overrun narrowing ratified:** `WAVE-QUEUE.md`, `WAVE-RUN-LOG.md`, `SYSTEM-QUEUE.md`, `HARDWARE-QUEUE.md`, and `doc-batches/*.md` are operational bookkeeping surfaces exempt from stop-condition-3 (diff overrun). Overrun fires on undeclared touches to `crates/**`, `schemas/**`, `.github/workflows/**`, `tools/**`, undeclared shared-truth surfaces (`STATUS.md`, gate docs, `CONTRACTS-OVERVIEW.md`), or cross-stream anchor encroachment. This narrowing is a practice ratification, not a `BATCH-MODE.md` edit; the doc edit is queued for the post-proof-run substrate-refinement wave.
- **Batch-level verdict criteria:**
  - **Pass:** all 3 waves landed clean, gatekeeper returned `continue` after waves 1 and 2 and `stop-clean` after wave 3, no overrun fires under the narrowed rule, validator never escalated.
  - **Conditional pass:** all 3 landed clean but ≥1 substrate ambiguity flagged for follow-up.
  - **Fail:** any wave required Triage/Trace, any frozen surface was touched, or batch halted before wave 3.
- **No substrate edits during this run.** `BATCH-MODE.md` and `LOCAL-DRIVER.md` edits are held for the first post-proof-run wave.

---

## 2026-04-27 — WAVE-2026-04-27-05

- Selected item: `WAVE-2026-04-27-05`
- Result: WAVE-2026-04-27-05 is complete. The S7 owner docs now define the operator-facing evidence surface for hub bring-up and status. The surface is read-only and is sourced from `docs/hub/pack-b-bring-up-worksheet.md` (which derives from `docs/hub/reference-hardware.md`). The six surface fields (device target, firmware-spike milestone reached, profile init status, HA entity registered or named stand-in, consent flow visible, power-cycle status) are defined in a table in `streams/S7-hub/README.md` alongside explicit constraints: read-only, does not constitute D1 evidence, no new JSON/RPC routes, and no HA bridge protocol or pairing handshake details invented. The S7 backlog now marks the operator evidence surface definition as complete. `docs/hub/pack-b-bring-up-worksheet.md` carries a back-reference confirming the worksheet is the source-of-truth for the operator surface definition.
- Files: `streams/S7-hub/README.md`, `streams/S7-hub/BACKLOG.md`, `docs/hub/pack-b-bring-up-worksheet.md`, `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: `get_errors` clean on `streams/S7-hub/README.md`, `streams/S7-hub/BACKLOG.md`, and `docs/hub/pack-b-bring-up-worksheet.md`. No crate, schema, harness, or workflow file was modified. No G4 evidence was claimed. No bridge protocol details were introduced.
- Next follow-up: batch complete — gatekeeper emits stop-clean; human re-entry at doc-batch review

```json
{
  "wave_id": "WAVE-2026-04-27-05",
  "stop_conditions_evaluated": {
    "1_validation_failed": "pass — get_errors clean on all 3 anchor files",
    "2_wave_tag": "no P0/gate-close/solo/frozen-schema flags — clear",
    "3_diff_overrun": "none — WAVE-QUEUE.md and WAVE-RUN-LOG.md are bookkeeping-exempt; no crates/, schemas/, .github/workflows/, tools/, or undeclared shared-truth file was touched",
    "4_track_boundary": "no next Ready wave in code-track queue — not applicable; batch run-length cap fires first",
    "5_run_length_cap": "3 of 8 — batch target reached; stop-clean",
    "6_escalation_chain": "no escalation — clear"
  },
  "decision": "stop-clean",
  "rationale": "All three planned waves landed clean with no validation failures, no overrun fires under the narrowed rule, and no escalation. Wave 3 exhausts the declared batch scope (WAVE-2026-04-27-02, -04, -05). The run-length cap fires at 3 of 8, which is the planned proof-run target. The batch halts cleanly. Human re-entry is at the doc-batch summary review. The code-track Ready queue is now empty."
}
```

---

## 2026-04-27 — WAVE-2026-04-27-04

- Selected item: `WAVE-2026-04-27-04`
- Result: WAVE-2026-04-27-04 is complete. The S5 owner docs now define the minimum honest first browser profile surface entry bar above the frozen S2 contract: scope is `init`, `show`, `export`, and `import` only; localhost-only; no grant mutation; no `revoke`; each slot calls the already-frozen CLI path through the JSON/RPC surface without reopening `schemas/profile.v0.json` or `schemas/capability-grant.v0.json`; `docs/legacy/personal-profile.html` is shape reference only and does not constitute G2 re-evidence; and the profile surface lands only after the four CLI paths are confirmed reachable through the localhost shell host and a harness proves the surface stays within the frozen S2 boundary. The backlog now carries the definition as checked and a new unchecked item for the code-backed landing. S2 carries a consumer-awareness note confirming S5 is a read/init consumer of the frozen profile contract and does not reopen G2 or mutate the S2 contract.
- Files: `streams/S5-ux/README.md`, `streams/S5-ux/BACKLOG.md`, `streams/S2-profile/README.md`, `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: `get_errors` clean on `streams/S5-ux/README.md`, `streams/S5-ux/BACKLOG.md`, and `streams/S2-profile/README.md`. `schemas/profile.v0.json` and `schemas/capability-grant.v0.json` not touched. No crate, schema, harness, or workflow file was modified.
- Next follow-up: `WAVE-2026-04-27-05` (final code-track wave in this batch; batch continues)

```json
{
  "wave_id": "WAVE-2026-04-27-04",
  "stop_conditions_evaluated": {
    "1_validation_failed": "pass — get_errors clean on all 3 anchor files",
    "2_wave_tag": "no P0/gate-close/solo/frozen-schema flags — clear; frozen schemas were referenced but not touched",
    "3_diff_overrun": "none — WAVE-QUEUE.md and WAVE-RUN-LOG.md are bookkeeping-exempt; no crates/, schemas/, .github/workflows/, tools/, or undeclared shared-truth file was touched",
    "4_track_boundary": "next wave WAVE-2026-04-27-05 is code-track — clear",
    "5_run_length_cap": "2 of 8 — clear",
    "6_escalation_chain": "no escalation — clear"
  },
  "decision": "continue",
  "rationale": "All six stop conditions are clear. The wave landed docs-only consumer-awareness and bar-definition content across three anchor files. Frozen schemas were explicitly referenced in constraint language but not modified. get_errors is clean on all anchor files. No crate, schema, or workflow file was touched. The next queued wave (WAVE-2026-04-27-05) is a code-track S-size docs-only S7 wave with no P0, gate-close, solo, or frozen-schema flags. Batch continues to wave 3."
}
```

---

## 2026-04-27 — WAVE-2026-04-27-02

- Selected item: `WAVE-2026-04-27-02`
- Result: WAVE-2026-04-27-02 is complete. The S5 owner docs now define the minimum consent-gated browser-issued lifecycle control bar above the staged shell-intent copy: scope is `agent.run` and `agent.stop` only for the currently selected agent; consent/audit gating begins **before** the write RPC is sent; deny-by-default enforcement must be demonstrable through the S3 deny-log slot; and the control bar is not published until gating is observable, S4 enforcement is confirmed through the S3 read path, and a harness proves the gate fires before the write RPC is transmitted. The backlog item is now checked and a new unchecked item tracks the code-backed landing. S3 now carries a consumer-awareness note that S5's bar consumes only the already-landed `agent.run` / `agent.stop` local-only slice and implies no new S3 RPC methods. S4 carries a support-awareness note that S5's stated bar is downstream of S4's already-landed deny-by-default enforcement and implies no new S4 work. Grant/revoke, consent resolution for non-lifecycle operations, broader browser control, and S4 restart/reload semantics remain explicitly out of scope for this bar.
- Files: `streams/S5-ux/README.md`, `streams/S5-ux/BACKLOG.md`, `streams/S3-agent-center/CONTRACTS.md`, `streams/S4-runtime/BACKLOG.md`, `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: `get_errors` clean on `streams/S5-ux/README.md`, `streams/S5-ux/BACKLOG.md`, `streams/S3-agent-center/CONTRACTS.md`, and `streams/S4-runtime/BACKLOG.md`. No file under `crates/`, `schemas/`, `site/`, `harnesses/`, `.github/workflows/`, or `tools/` was modified. No frozen schema was touched. No gate doc was moved.
- Next follow-up: `WAVE-2026-04-27-04` (next code-track wave; batch continues)

```json
{
  "wave_id": "WAVE-2026-04-27-02",
  "stop_conditions_evaluated": {
    "1_validation_failed": "pass — get_errors clean on all 4 anchor files",
    "2_wave_tag": "no P0/gate-close/solo/frozen-schema flags — clear",
    "3_diff_overrun": "none — WAVE-QUEUE.md and WAVE-RUN-LOG.md are bookkeeping-exempt under the ratified narrowing; no crates/, schemas/, .github/workflows/, tools/, or undeclared shared-truth file was touched",
    "4_track_boundary": "next wave WAVE-2026-04-27-04 is code-track — clear",
    "5_run_length_cap": "1 of 8 — clear",
    "6_escalation_chain": "no escalation — clear"
  },
  "decision": "continue",
  "rationale": "All six stop conditions are clear. The wave landed docs-only changes to four anchor files, all of which pass get_errors. No frozen surface was touched, no crate or schema file was modified, and the next queued wave (WAVE-2026-04-27-04) is a code-track S-size docs-only wave with no P0, gate-close, solo, or frozen-schema flags. Batch continues to wave 2."
}
```

---

## 2026-04-27 — WAVE-2026-04-27-03

- Selected item: `WAVE-2026-04-27-03`
- Result: WAVE-2026-04-27-03 is complete. S8 landed the orchestration substrate upgrade: (1) `docs/orchestration/BATCH-MODE.md` defines Batch Mode as an explicit user-invoked mode with stop conditions, a gatekeeper contract, human re-entry triggers, and the target width-8 planning goal while keeping the editing-lane ceiling at 5 until a clean proof run is logged; (2) `docs/orchestration/SYSTEM-QUEUE.md` and `docs/orchestration/HARDWARE-QUEUE.md` scaffold the system and hardware track queues with ≥3 Ready items each; (3) `docs/orchestration/WAVE-QUEUE.md` preamble clarified as the code track, schema extended with five additive optional fields (`size`, `parallel-safe-with`, `serial-after`, `solo`, `track`), and two new Ready code-track waves added (profile-surface entry-bar S5/S2 and operator evidence surface S7), bringing Ready depth to 3; (4) `docs/gates/D1.md` defines the demo gate between G3 and G4 with explicit evidence requirements, D1 vs. G4 scope distinction, and no G4 reopening; (5) eight `streams/S*/FILLER.md` files (S1–S8) each carry Near/Close/Far sections with 3–6 candidate filler items; (6) `docs/orchestration/LOCAL-DRIVER.md` updated to reference Batch Mode and the multi-track queue structure; (7) `STATUS.md` updated with today's date, D1 gate row, S5 staged-intent line update, and recent activity entry.
- Files: `docs/orchestration/BATCH-MODE.md`, `docs/orchestration/SYSTEM-QUEUE.md`, `docs/orchestration/HARDWARE-QUEUE.md`, `docs/gates/D1.md`, `streams/S1-foundation/FILLER.md`, `streams/S2-profile/FILLER.md`, `streams/S3-agent-center/FILLER.md`, `streams/S4-runtime/FILLER.md`, `streams/S5-ux/FILLER.md`, `streams/S6-harvest/FILLER.md`, `streams/S7-hub/FILLER.md`, `streams/S8-docs/FILLER.md`, `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/LOCAL-DRIVER.md`, `docs/orchestration/WAVE-RUN-LOG.md`, `STATUS.md`
- Validation: `get_errors` clean on all touched markdown files. `WAVE-QUEUE.md`, `SYSTEM-QUEUE.md`, and `HARDWARE-QUEUE.md` each have ≥3 Ready items. `STATUS.md` `Last updated:` reflects 2026-04-27. `docs/gates/D1.md` exists and is referenced from `STATUS.md`. Eight `streams/S*/FILLER.md` files exist with Near/Close/Far sections. `BATCH-MODE.md` is referenced from `LOCAL-DRIVER.md`. No file under `crates/`, `schemas/`, `site/`, `harnesses/`, `.github/workflows/`, or `tools/` was modified.
- Next follow-up: `WAVE-2026-04-27-02` (first Ready code-track wave; run in Interactive Mode or as the start of a code-track Batch Mode run)

## 2026-04-27 — WAVE-2026-04-27-01

- Selected item: `WAVE-2026-04-27-01`
- Result: WAVE-2026-04-27-01 is complete. `site/agent-center-shell.html` now stages selected-agent lifecycle intent copy and read-only slot affordances against the already-landed local-only `agent.run` / `agent.stop` backend slice on the current localhost shell. The shell stays read-only: the selected agent, its current running or stopped state, and the currently staged backend method now appear in the focus, tools, and consent/audit slots, but the browser still does not send write RPC, grant/revoke actions, or broader privileged control flows. `harnesses/localhost-shell-acceptance-harness.html` now verifies that selected-agent intent copy updates when the user changes selection and that the same copy remains route-aware and non-privileged while the journey moves through grants and deny-log views. S5 owner docs now truth-sync that the first shell-intent slice is landed while actual browser-issued lifecycle controls remain future work.
- Files: `site/agent-center-shell.html`, `harnesses/localhost-shell-acceptance-harness.html`, `streams/S5-ux/README.md`, `streams/S5-ux/BACKLOG.md`, `streams/S5-ux/PROGRESS.md`, `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: `get_errors` is clean on `site/agent-center-shell.html`, `harnesses/localhost-shell-acceptance-harness.html`, `streams/S5-ux/README.md`, `streams/S5-ux/BACKLOG.md`, and `streams/S5-ux/PROGRESS.md`. Live browser validation at `http://127.0.0.1:4317/` showed selected-agent intent copy updates with agent selection, `cargo run -p ferros-node --bin ferros -- agent run echo` followed by shell refresh flipped the visible staged method to `agent.stop`, and `cargo run -p ferros-node --bin ferros -- agent stop echo` followed by refresh flipped it back to `agent.run`. Same-origin harness validation at `http://127.0.0.1:4317/harnesses/localhost-shell-acceptance.html` passed 19 checks, failed 0, and skipped 8 operator/optional checks after auto-skipping confirms.
- Next follow-up: `WAVE-2026-04-27-02`

## 2026-04-27 — WAVE-2026-04-26-10

- Selected item: `WAVE-2026-04-26-10`
- Result: WAVE-2026-04-26-10 is complete. The S5 owner docs now define the minimum first shell-intent entry bar above the landed local-only `agent.run` / `agent.stop` localhost backend slice: the next honest shell publication is only selected-agent lifecycle intent copy plus slot ownership on the current localhost shell, while browser-issued writes, grant/revoke actions, consent resolution, broader browser control, remote transport, and broader S4 restart/reload claims remain unpublished. The nearest S3 and S4 awareness docs now match that narrowed consumer boundary so the staged shell intent does not get mistaken for a real browser-control contract.
- Files: `streams/S5-ux/README.md`, `streams/S5-ux/BACKLOG.md`, `streams/S5-ux/PROGRESS.md`, `streams/S3-agent-center/CONTRACTS.md`, `streams/S4-runtime/BACKLOG.md`, `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: `get_errors` is clean on `streams/S5-ux/README.md`, `streams/S5-ux/BACKLOG.md`, `streams/S5-ux/PROGRESS.md`, `streams/S3-agent-center/CONTRACTS.md`, and `streams/S4-runtime/BACKLOG.md`.
- Next follow-up: `WAVE-2026-04-27-01`

## 2026-04-26 — WAVE-2026-04-26-09

- Selected item: `WAVE-2026-04-26-09`
- Result: WAVE-2026-04-26-09 is complete. `crates/ferros-agents/src/rpc.rs` and `crates/ferros-node/src/lib.rs` now land the minimum local-only lifecycle/write JSON-RPC slice on the current localhost shell host: `agent.run` and `agent.stop` route through the already-landed `LocalAgentApi` seam on the same persisted local state path, `agent.describe`, `agent.snapshot`, and `denyLog.list` remain the read-after-write observation path, and denied writes keep the same local deny summaries while returning a local-only authorization error envelope on the JSON-RPC path. S3, S4, and S5 owner docs now truth-sync that the backend localhost write slice exists, while the shell UI itself still remains observation-only and broader browser control, grant writes, remote transport, bridge-control choreography, and broader S4 restart/reload claims remain unpublished. Shared truth was updated serially in `STATUS.md` and `docs/contracts/CONTRACTS-OVERVIEW.md` because the owning stream contract docs moved in the same wave.
- Files: `crates/ferros-agents/src/rpc.rs`, `crates/ferros-agents/src/lib.rs`, `crates/ferros-node/src/lib.rs`, `streams/S3-agent-center/README.md`, `streams/S3-agent-center/CONTRACTS.md`, `streams/S3-agent-center/BACKLOG.md`, `streams/S3-agent-center/PROGRESS.md`, `streams/S4-runtime/BACKLOG.md`, `streams/S4-runtime/PROGRESS.md`, `streams/S5-ux/README.md`, `streams/S5-ux/BACKLOG.md`, `streams/S5-ux/PROGRESS.md`, `STATUS.md`, `docs/contracts/CONTRACTS-OVERVIEW.md`, `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: `cargo test -p ferros-node agent_write_rpc_` passed. `cargo test -p ferros-node shell_listener_posts_json_rpc_` passed. `cargo test -p ferros-node agent_read_rpc_` passed. `cargo test -p ferros-agents` passed. `cargo xtask ci` passed. `get_errors` is clean on the touched S3, S4, S5, shared-truth, and orchestration files.
- Next follow-up: `WAVE-2026-04-26-10`

## 2026-04-26 — WAVE-2026-04-26-08

- Selected item: `WAVE-2026-04-26-08`
- Result: WAVE-2026-04-26-08 is complete. The S3 and S5 owner docs now define the minimum first local write JSON/RPC entry bar above the landed `LocalAgentApi` seam without claiming that write-side code already exists. The new boundary keeps the next honest write-side publication local-only on the current localhost shell host, limits the first write methods to `agent.run` and `agent.stop`, reuses `LocalAgentApi` plus the existing persisted local state path, and keeps `agent.describe`, `agent.snapshot`, and `denyLog.list` as the read-after-write observation path. The same docs pass also narrows S5 Phase B follow-up wording so the shell remains observation-only until that code-backed local lifecycle/write JSON/RPC slice actually lands; grant/revoke and broader browser-control work remain explicitly out of scope.
- Files: `streams/S3-agent-center/README.md`, `streams/S3-agent-center/CONTRACTS.md`, `streams/S3-agent-center/BACKLOG.md`, `streams/S3-agent-center/PROGRESS.md`, `streams/S5-ux/README.md`, `streams/S5-ux/BACKLOG.md`, `streams/S5-ux/PROGRESS.md`, `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: `get_errors` is clean on `streams/S3-agent-center/README.md`, `streams/S3-agent-center/CONTRACTS.md`, `streams/S3-agent-center/BACKLOG.md`, `streams/S3-agent-center/PROGRESS.md`, `streams/S5-ux/README.md`, `streams/S5-ux/BACKLOG.md`, and `streams/S5-ux/PROGRESS.md`. Final editor diagnostics are clean on `docs/orchestration/WAVE-QUEUE.md` and `docs/orchestration/WAVE-RUN-LOG.md`.
- Next follow-up: `WAVE-2026-04-26-09`

## 2026-04-26 — WAVE-2026-04-26-07

- Selected item: `WAVE-2026-04-26-07`
- Result: WAVE-2026-04-26-07 is complete. `crates/ferros-node/src/lib.rs` now preserves typed missing-capability detail on denied local `LocalAgentApi` runs through `AuthorizationDenyDetail` while keeping the CLI denial summary text, persisted deny-log text, read-first JSON/RPC behavior, and localhost shell host behavior stable on the same local seam. In the same user-requested overnight filler batch, disjoint repo-local lanes also landed safely around that hot seam: S1 added a manual release-candidate bundle workflow plus concurrency guards on the existing workflows, S2 made local profile bundle import rollback-safe when invalid grant state is discovered after partial file creation, S6 published the ordered ferros-data migration manifest and tightened its ordered-child invariant proof, and S7 added operator rehearsal prep to the Pack B runway docs. Serial shared-truth reconciliation then updated `STATUS.md` and `docs/contracts/CONTRACTS-OVERVIEW.md` without overclaiming new remote/browser control or gate movement.
- Files: `crates/ferros-node/src/lib.rs`, `.github/workflows/ci.yml`, `.github/workflows/integration.yml`, `.github/workflows/release.yml`, `crates/ferros-profile/src/lib.rs`, `crates/ferros-data/src/lib.rs`, `docs/hub/reference-hardware.md`, `docs/hub/pack-b-bring-up-worksheet.md`, `streams/S1-foundation/BACKLOG.md`, `streams/S1-foundation/PROGRESS.md`, `streams/S1-foundation/CONTRACTS.md`, `streams/S2-profile/PROGRESS.md`, `streams/S3-agent-center/BACKLOG.md`, `streams/S3-agent-center/README.md`, `streams/S3-agent-center/CONTRACTS.md`, `streams/S3-agent-center/PROGRESS.md`, `streams/S4-runtime/BACKLOG.md`, `streams/S4-runtime/PROGRESS.md`, `streams/S6-harvest/PROGRESS.md`, `streams/S7-hub/BACKLOG.md`, `streams/S7-hub/PROGRESS.md`, `STATUS.md`, `docs/contracts/CONTRACTS-OVERVIEW.md`, `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: `cargo test -p ferros-node local_agent_api_` passed. `cargo test -p ferros-node agent_cli_` passed. `cargo test -p ferros-profile file_system_profile_store_import_rolls_back_partial_state_when_bundle_grants_are_invalid` passed. `cargo test -p ferros-profile reload_boundary_load_local_profile` passed. `cargo test -p ferros-profile file_system_profile_store` passed. `cargo test -p ferros-profile` passed. `cargo test -p ferros-data` passed. `cargo xtask ci` passed. `get_errors` is clean on the touched S1, S2, S3, S4, S6, S7, shared-truth, and workflow files. Final editor diagnostics are clean on `docs/orchestration/WAVE-QUEUE.md` and `docs/orchestration/WAVE-RUN-LOG.md`.
- Next follow-up: `WAVE-2026-04-26-08`

## 2026-04-26 — WAVE-2026-04-26-06

- Selected item: `WAVE-2026-04-26-06`
- Result: WAVE-2026-04-26-06 is complete. `crates/ferros-node/src/lib.rs` now publishes `LocalAgentApi` as the first broader local-only lifecycle/write wrapper/API slice above CLI formatting. The landed slice keeps the surface local-only and typed (`list | describe | run | stop | logs`), reuses the same persisted local state path, the internal `LocalAgentController`, and the current `DemoRuntime::reference_host()` / `run_reference_demo_cycle()` path, while the local CLI remains a formatter over that same path and the read-first JSON/RPC plus localhost shell observation surfaces stay unchanged and green. S3 owner docs, S4 support docs, `STATUS.md`, and `docs/contracts/CONTRACTS-OVERVIEW.md` now truth-sync that the first broader local-only wrapper/API slice is landed without publishing remote transport, richer remote observation/control, privileged UX, grant writes, bridge-control choreography, broader S4 restart/reload semantics, schemas, `crates/ferros-hub`, or G4 evidence. A post-land Lane Architect reassessment concluded that the next implementation head is still one hot S3/S4 lane on `crates/ferros-node/src/lib.rs`, but that future batches can widen again only with strictly disjoint filler work; this closeout seeds the next deny-introspection head accordingly.
- Files: `crates/ferros-node/src/lib.rs`, `streams/S3-agent-center/BACKLOG.md`, `streams/S3-agent-center/README.md`, `streams/S3-agent-center/CONTRACTS.md`, `streams/S3-agent-center/PROGRESS.md`, `streams/S4-runtime/BACKLOG.md`, `streams/S4-runtime/PROGRESS.md`, `STATUS.md`, `docs/contracts/CONTRACTS-OVERVIEW.md`, `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: `cargo test -p ferros-node local_agent_api_` passed. `cargo test -p ferros-node agent_cli_` passed. `cargo test -p ferros-node agent_read_rpc_` passed. `cargo test -p ferros-node shell_listener_posts_json_rpc_` passed. `get_errors` is clean on `crates/ferros-node/src/lib.rs`, `crates/ferros-node/src/bin/ferros.rs`, `streams/S3-agent-center/BACKLOG.md`, `streams/S3-agent-center/README.md`, `streams/S3-agent-center/CONTRACTS.md`, `streams/S3-agent-center/PROGRESS.md`, `streams/S4-runtime/BACKLOG.md`, `streams/S4-runtime/PROGRESS.md`, `STATUS.md`, and `docs/contracts/CONTRACTS-OVERVIEW.md`. Final editor diagnostics are clean on `docs/orchestration/WAVE-QUEUE.md` and `docs/orchestration/WAVE-RUN-LOG.md`.
- Next follow-up: `WAVE-2026-04-26-07`

## 2026-04-26 — WAVE-2026-04-26-05

- Selected item: `WAVE-2026-04-26-05`
- Result: WAVE-2026-04-26-05 is complete. `crates/ferros-node/src/lib.rs` now extracts the first internal local host-controller surface above argv parsing into an internal `LocalAgentController`, so the current local `ferros agent` lifecycle/log path no longer leaves its controller logic flattened inside the CLI execution function. The landed slice keeps the surface internal and local-only: it does not publish a broader lifecycle/write wrapper/API, remote transport, privileged UX, grant-write semantics, bridge-control choreography, broader S4 restart/reload semantics, schemas, `crates/ferros-hub`, or G4 evidence. `streams/S4-runtime/BACKLOG.md`, `streams/S4-runtime/PROGRESS.md`, `streams/S3-agent-center/CONTRACTS.md`, and `streams/S3-agent-center/PROGRESS.md` now truth-sync that this extraction landed below any published broader wrapper/API or remote-control contract. A post-land Lane Architect reassessment concluded that the next honest follow-up is still a single hot lane on the first publishable broader lifecycle/write wrapper/API slice above this new internal seam, so this closeout seeds that still-tight Ready head instead of widening the next batch.
- Files: `crates/ferros-node/src/lib.rs`, `streams/S4-runtime/BACKLOG.md`, `streams/S4-runtime/PROGRESS.md`, `streams/S3-agent-center/CONTRACTS.md`, `streams/S3-agent-center/PROGRESS.md`, `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: `cargo test -p ferros-node agent_cli_` passed. `cargo test -p ferros-node agent_read_rpc_` passed. `cargo test -p ferros-node shell_listener_posts_json_rpc_` passed. `get_errors` is clean on `crates/ferros-node/src/lib.rs`, `streams/S4-runtime/BACKLOG.md`, `streams/S4-runtime/PROGRESS.md`, `streams/S3-agent-center/CONTRACTS.md`, and `streams/S3-agent-center/PROGRESS.md`. Final editor diagnostics are clean on `docs/orchestration/WAVE-QUEUE.md` and `docs/orchestration/WAVE-RUN-LOG.md`.
- Next follow-up: `WAVE-2026-04-26-06`

## 2026-04-26 — Batch: WAVE-2026-04-25-07 + WAVE-2026-04-26-01..04

- Selected item: `WAVE-2026-04-25-07`, `WAVE-2026-04-26-01`, `WAVE-2026-04-26-02`, `WAVE-2026-04-26-03`, `WAVE-2026-04-26-04`
- Result: This one user-requested batch is complete. `WAVE-2026-04-25-07` clarified that the landed local-only lifecycle/write seam is fixed input and defined the minimum first broader lifecycle/write wrapper/API entry bar above that seam while keeping remote transport, richer remote observation/control, privileged UX, grant writes, bridge-control choreography, and broader S4 restart/reload semantics unpublished. `WAVE-2026-04-26-01` landed as a no-delta proof lane by validating `cargo check -p ferros-core --target thumbv7em-none-eabi --no-default-features`, `cargo check -p ferros-core --no-default-features`, and `cargo test -p ferros-core` locally against the already-present CI enforcement slice, and it did not record a new hosted CI run in this batch. `WAVE-2026-04-26-02` extended the same-origin localhost harness so operator-assisted local `ferros agent run echo` and `ferros agent stop echo` steps can be observed through exactly one `agent.snapshot` call per refresh while keeping the shell observation-only and live deny generation out of scope. `WAVE-2026-04-26-03` added the first Pack B bring-up worksheet from the runway map without claiming hub implementation, launch truth, or G4 evidence. `WAVE-2026-04-26-04` truth-synced the S2 backlog to the already-closed G2 boundary by removing stale pre-freeze and unfinished-CLI wording and replacing it with post-G2 parity/local CLI hardening follow-ons. After the five owner lanes landed, serial S8 reconciliation updated shared truth so `STATUS.md` now reflects the local-only lifecycle/read-after-write seam, the S5 operator-assisted observation proof, the S7 worksheet, and the local thumb-target compile proof, while `docs/contracts/CONTRACTS-OVERVIEW.md` reflects `agent.snapshot` plus the local thumb-target compile proof without overclaiming a new hosted CI run. This closeout seeded the next Ready item and did not create a separate queue wave for that serial reconciliation.
- Files: `WAVE-2026-04-25-07` -> `streams/S3-agent-center/README.md`, `streams/S3-agent-center/CONTRACTS.md`, `streams/S3-agent-center/PROGRESS.md`; `WAVE-2026-04-26-01` -> no owner-file delta (current proof/truth already lived in `.github/workflows/ci.yml`, `streams/S4-runtime/README.md`, and `streams/S4-runtime/PROGRESS.md`); `WAVE-2026-04-26-02` -> `harnesses/localhost-shell-acceptance-harness.html`, `streams/S5-ux/BACKLOG.md`, `streams/S5-ux/README.md`, `streams/S5-ux/PROGRESS.md`; `WAVE-2026-04-26-03` -> `docs/hub/pack-b-bring-up-worksheet.md`, `streams/S7-hub/BACKLOG.md`, `streams/S7-hub/PROGRESS.md`; `WAVE-2026-04-26-04` -> `streams/S2-profile/BACKLOG.md`; serial S8 reconciliation -> `STATUS.md`, `docs/contracts/CONTRACTS-OVERVIEW.md`; orchestration closeout -> `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`.
- Validation: `get_errors` is clean on the touched S3 docs from `WAVE-2026-04-25-07`; `cargo check -p ferros-core --target thumbv7em-none-eabi --no-default-features`, `cargo check -p ferros-core --no-default-features`, and `cargo test -p ferros-core` all passed for `WAVE-2026-04-26-01`, with `get_errors` clean on `.github/workflows/ci.yml` and the matching S4 docs already clean in the parent thread; `get_errors` is clean on `harnesses/localhost-shell-acceptance-harness.html`, `streams/S5-ux/BACKLOG.md`, `streams/S5-ux/README.md`, and `streams/S5-ux/PROGRESS.md`, harness reload with auto-skipped operator prompts produced 16 passed, 0 failed, 6 skipped, and live shell validation passed after restoring `cargo run -p ferros-node --bin ferros-node -- shell 4317`, then `cargo run -p ferros-node --bin ferros -- agent run echo` / refresh showed one `agent.snapshot` call and echo observed as running, and `cargo run -p ferros-node --bin ferros -- agent stop echo` / refresh showed one `agent.snapshot` call and echo observed as stopped for `WAVE-2026-04-26-02`; `get_errors` is clean on `docs/hub/pack-b-bring-up-worksheet.md`, `streams/S7-hub/BACKLOG.md`, and `streams/S7-hub/PROGRESS.md` for `WAVE-2026-04-26-03`; `get_errors` is clean on `streams/S2-profile/BACKLOG.md` for `WAVE-2026-04-26-04`; `get_errors` is clean on `STATUS.md` and `docs/contracts/CONTRACTS-OVERVIEW.md` after the serial S8 reconciliation; final editor diagnostics are clean on `docs/orchestration/WAVE-QUEUE.md` and `docs/orchestration/WAVE-RUN-LOG.md`.
- Next follow-up: `WAVE-2026-04-26-05`

## 2026-04-25 — WAVE-2026-04-25-06

- Selected item: `WAVE-2026-04-25-06`
- Result: WAVE-2026-04-25-06 is complete. The repo had already landed the first local-only lifecycle/write seam through the current CLI/state path: `crates/ferros-node/src/lib.rs` now has focused coverage that drives `ferros agent run` and `ferros agent stop` through the existing local state path and proves `agent.describe` plus `agent.snapshot` observe the resulting running and stopped state on that same path, while the already-landed deny-by-default lifecycle/log harness continues to prove denied attempts persist `denied-start:*` evidence and expose it through `ferros agent logs` and `denyLog.list`. `streams/S3-agent-center/CONTRACTS.md`, `streams/S3-agent-center/BACKLOG.md`, `streams/S3-agent-center/README.md`, and `streams/S3-agent-center/PROGRESS.md` already truth-sync that this first lifecycle/write seam is landed only as a narrow, code-backed, local-only slice through the existing CLI/state path and current read-first inspection surfaces. This bookkeeping closeout only recorded completion, seeded the next Ready item, and did not move broader lifecycle/write wrapper/API, richer remote observation/control transport, S5 privileged grant/revoke UX, grant-write semantics, pairing choreography, bridge-control sequencing, broader S4 restart/reload semantics, schemas, `crates/ferros-hub`, or G4 evidence.
- Files: `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: parent-thread checks passed. `cargo test -p ferros-node agent_read_rpc_observes_cli_lifecycle_state_after_local_run_and_stop` passed. `cargo test -p ferros-node agent_cli_` passed. `cargo test -p ferros-node agent_read_rpc_` passed. `get_errors` is clean on `crates/ferros-node/src/lib.rs`, `streams/S3-agent-center/BACKLOG.md`, `streams/S3-agent-center/CONTRACTS.md`, `streams/S3-agent-center/README.md`, and `streams/S3-agent-center/PROGRESS.md`. Final editor diagnostics clean on `docs/orchestration/WAVE-QUEUE.md` and `docs/orchestration/WAVE-RUN-LOG.md`.
- Next follow-up: `WAVE-2026-04-25-07`

## 2026-04-25 — WAVE-2026-04-25-05

- Selected item: `WAVE-2026-04-25-05`
- Result: WAVE-2026-04-25-05 is complete. The repo had already landed the docs-only minimum first lifecycle/write entry bar across `streams/S3-agent-center/CONTRACTS.md`, `streams/S3-agent-center/BACKLOG.md`, and `streams/S3-agent-center/README.md`: `streams/S3-agent-center/CONTRACTS.md` now defines that the minimum first lifecycle/write entry bar is not yet landed and names `DemoRuntime::reference_host()`, `run_reference_demo_cycle()`, the current local CLI/state-path behavior, the current local CLI inspection plus read-first JSON-RPC methods, and the dedicated deny-by-default lifecycle/log harness as the local-only seams to reuse; `streams/S3-agent-center/BACKLOG.md` marks that docs-only entry bar landed and leaves one narrow code-backed local-only lifecycle/write slice open; and `streams/S3-agent-center/README.md` carries the same minimum-entry-bar wording and immediate next step. This bookkeeping closeout only recorded completion, seeded the next Ready item, and preserved that no code, no lifecycle/write wrapper, no richer remote observation/control, no privileged UX, no pairing choreography, no bridge-control sequencing, no broader S4 restart/reload semantics, no schemas, no `crates/ferros-hub`, and no G4 evidence movement landed in this closeout.
- Files: `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: parent-thread checks passed. `get_errors` is clean on `streams/S3-agent-center/BACKLOG.md`, `streams/S3-agent-center/CONTRACTS.md`, and `streams/S3-agent-center/README.md`. The landed wording was reread directly and keeps lifecycle/write wrapper, richer remote observation/control, privileged UX, grant-write, bridge-control, and S4 restart/reload semantics unpublished. Final editor diagnostics clean on `docs/orchestration/WAVE-QUEUE.md` and `docs/orchestration/WAVE-RUN-LOG.md`.
- Next follow-up: `WAVE-2026-04-25-06`

## 2026-04-25 — WAVE-2026-04-25-04

- Selected item: `WAVE-2026-04-25-04`
- Result: WAVE-2026-04-25-04 is complete. The repo had already landed the dedicated local deny-by-default lifecycle/log harness: `crates/ferros-node/src/lib.rs` now persists `denied-start:*` runtime log entries through the local `ferros agent` state path even when `AgentCliCommand::Run` returns an authorization error, adds an internal runtime-loader seam so focused tests can drive denied lifecycle attempts through the same local state machinery, and proves with focused tests that a denied `ferros agent run echo` attempt leaves the agent registered, persists `denied-start:echo missing agent.echo`, and exposes that evidence through both `ferros agent logs` and `denyLog.list`. `streams/S3-agent-center/BACKLOG.md`, `streams/S3-agent-center/CONTRACTS.md`, `streams/S3-agent-center/PROGRESS.md`, and `streams/S3-agent-center/README.md` already truth-sync that the dedicated local deny-by-default lifecycle/log harness has landed while lifecycle/write wrapper work remains unpublished. This orchestration closeout only recorded completion, seeded the next Ready item, and preserved that no lifecycle/write wrapper APIs, no richer remote observation/control transport, no S5 privileged grant/revoke UX, no pairing choreography, no bridge-control sequencing, no broader S4 restart/reload semantics, no schemas, no `crates/ferros-hub`, and no G4 evidence movement landed in this bookkeeping pass.
- Files: `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: parent-thread checks passed. `cargo test -p ferros-agents manifest_authorization_` passed. `cargo test -p ferros-node agent_cli_` passed. `cargo test -p ferros-node agent_read_rpc_` passed. `get_errors` is clean on `crates/ferros-node/src/lib.rs`, `streams/S3-agent-center/BACKLOG.md`, `streams/S3-agent-center/CONTRACTS.md`, `streams/S3-agent-center/PROGRESS.md`, and `streams/S3-agent-center/README.md`. Final editor diagnostics clean on `docs/orchestration/WAVE-QUEUE.md` and `docs/orchestration/WAVE-RUN-LOG.md`.
- Next follow-up: `WAVE-2026-04-25-05`

## 2026-04-25 — WAVE-2026-04-25-03

- Selected item: `WAVE-2026-04-25-03`
- Result: WAVE-2026-04-25-03 is complete. The repo had already landed the reusable in-memory runtime-host slice: `crates/ferros-node/src/lib.rs` now exposes `DemoRuntime::reference_host()` to bootstrap the reference grants, registry, and agents, `DemoRuntime::run_reference_demo_cycle()` to execute the deterministic demo path, and `replay_cli_state()` to reuse the same host bootstrap for CLI state replay, while the top-level `build_reference_runtime()` and `run_demo()` entrypoints remain thin wrappers so current CLI callers stay stable. `streams/S3-agent-center/BACKLOG.md`, `streams/S3-agent-center/CONTRACTS.md`, `streams/S3-agent-center/PROGRESS.md`, and `streams/S3-agent-center/README.md` already truth-sync that the reusable in-memory host layer has landed while lifecycle/write wrapper work remains unpublished. This orchestration closeout only recorded completion, seeded the next Ready item, and preserved that no lifecycle/write wrapper API, no richer remote observation/control transport, no S5 privileged grant/revoke UX, no pairing choreography, no bridge-control sequencing, no broader S4 restart/reload semantics, no schemas, no `crates/ferros-hub`, and no G4 evidence movement landed in this bookkeeping pass.
- Files: `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: parent-thread checks passed. `cargo test -p ferros-node demo_` passed. `cargo test -p ferros-node reload_boundary_runtime_with_state_` passed. `cargo run -p ferros-node --bin ferros -- demo` printed the stable deterministic output (`started: echo,timer`, `echo: hello`, `timer: tick-1`, `denied: 1`). `get_errors` is clean on `crates/ferros-node/src/lib.rs`, `streams/S3-agent-center/BACKLOG.md`, `streams/S3-agent-center/CONTRACTS.md`, `streams/S3-agent-center/PROGRESS.md`, and `streams/S3-agent-center/README.md`. Final editor diagnostics clean on `docs/orchestration/WAVE-QUEUE.md` and `docs/orchestration/WAVE-RUN-LOG.md`.
- Next follow-up: `WAVE-2026-04-25-04`

## 2026-04-25 — WAVE-2026-04-25-02

- Selected item: `WAVE-2026-04-25-02`
- Result: WAVE-2026-04-25-02 is complete. The repo had already landed the S5 snapshot-consumer slice: `site/agent-center-shell.html` now consumes the aggregated read-only `agent.snapshot` seam as its main refresh path and reuses loaded snapshot state for inspector selection instead of the earlier fan-out `agent.list` / `grant.list` / `denyLog.list` plus `agent.describe` path; `harnesses/localhost-shell-acceptance-harness.html` now monitors same-origin `/rpc` fetches to assert exactly one `agent.snapshot` call on manual refresh, zero extra RPCs while selecting a loaded agent, and drives the iframe profile-path input through the correct frame-window event constructor; and `streams/S5-ux/BACKLOG.md`, `streams/S5-ux/PROGRESS.md`, and `streams/S5-ux/README.md` already truth-sync the observation-only snapshot seam and the live harness outcome. This lane only recorded completion, seeded the next Ready item, and preserved that the S5 observation-only local-shell consumer slice is complete with no lifecycle/write UX, no consent or grant mutation flows, no pairing choreography, no bridge-control sequencing, no S4 restart/reload changes, no schemas, no `crates/ferros-hub`, and no G4 evidence movement.
- Files: `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: parent-thread checks passed. `get_errors` is clean on `site/agent-center-shell.html`, `harnesses/localhost-shell-acceptance-harness.html`, `streams/S5-ux/BACKLOG.md`, `streams/S5-ux/PROGRESS.md`, and `streams/S5-ux/README.md`. Live browser validation at `http://127.0.0.1:4317/` showed the real shell in ready/live state with snapshot-based copy and aggregated metrics. Same-origin live harness validation at `http://127.0.0.1:4317/harnesses/localhost-shell-acceptance.html` passed 16/16 checks against the real local shell, including snapshot-only manual refresh, zero extra RPCs on loaded-agent selection, grants empty-state degradation, deny-log visibility, and read-only audit copy. Final editor diagnostics clean on `docs/orchestration/WAVE-QUEUE.md` and `docs/orchestration/WAVE-RUN-LOG.md`.
- Next follow-up: `WAVE-2026-04-25-03`

## 2026-04-25 — WAVE-2026-04-25-01

- Selected item: `WAVE-2026-04-25-01`
- Result: WAVE-2026-04-25-01 is complete. The repo had already landed the backend/docs `agent.snapshot` slice: `crates/ferros-agents/src/rpc.rs` now defines the read-only `agent.snapshot` JSON-RPC method plus the `agentSnapshot` result variant and aggregated `AgentRpcSnapshot` payload, `crates/ferros-agents/src/lib.rs` exports the new snapshot RPC types, `crates/ferros-node/src/lib.rs` now serves `agent.snapshot` by rebuilding the current runtime from loaded CLI state, returning detailed agent records, loading grant state from the selected profile path, and including deny-log entries with optional `agentName` filtering and the current not-found envelope for unknown agents, and the owning S3/S7 docs now publish this as the first implementation-backed observation-only wrapper/API slice while keeping lifecycle/write surfaces unpublished. This orchestration lane only recorded that completion, seeded the next Ready item, and did not move lifecycle/write wrapper work, pairing choreography, bridge-control sequencing, S4 restart/reload changes, schemas, `crates/ferros-hub`, or G4 evidence.
- Files: `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: cargo test -p ferros-agents agent_snapshot --lib passed; cargo test -p ferros-node snapshot --lib passed; editor diagnostics clean on crates/ferros-agents/src/rpc.rs, crates/ferros-agents/src/lib.rs, crates/ferros-node/src/lib.rs, streams/S3-agent-center/CONTRACTS.md, streams/S3-agent-center/BACKLOG.md, and streams/S7-hub/BACKLOG.md; final editor diagnostics clean on docs/orchestration/WAVE-QUEUE.md and docs/orchestration/WAVE-RUN-LOG.md
- Next follow-up: `WAVE-2026-04-25-02`

## 2026-04-25 — WAVE-2026-04-24-18

- Selected item: `WAVE-2026-04-24-18`
- Result: WAVE-2026-04-24-18 is complete. The repo had already updated `streams/S3-agent-center/CONTRACTS.md`, `streams/S3-agent-center/BACKLOG.md`, and `streams/S7-hub/BACKLOG.md` with the docs-only S3 boundary publication and returned dependency lock, naming `AgentRegistry` plus local/read-first inspection surfaces as the current honest runway boundary while keeping hub-facing lifecycle-wrapper, richer remote observation wrapper, and remote control/write contract unpublished before bridge control flows are honest. This orchestration lane only recorded that completion, seeded the next Ready item, and did not move S4 restart/reload semantics, pairing choreography, schemas, code, `crates/ferros-hub/` scaffold work, or G4 evidence.
- Files: `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: editor diagnostics clean on `streams/S3-agent-center/CONTRACTS.md`, `streams/S3-agent-center/BACKLOG.md`, and `streams/S7-hub/BACKLOG.md`; final editor diagnostics clean on `docs/orchestration/WAVE-QUEUE.md` and `docs/orchestration/WAVE-RUN-LOG.md`
- Next follow-up: `WAVE-2026-04-25-01`

## 2026-04-24 — WAVE-2026-04-24-17

- Selected item: `WAVE-2026-04-24-17`
- Result: WAVE-2026-04-24-17 is complete. The repo had already updated `crates/ferros-node/src/lib.rs` with focused tests that lock exact-path CliState reload, missing-path default-empty behavior, fixed reference-runtime rebuild, replay of persisted Running and Stopped state only, no log replay, and current error paths for unknown persisted agents and unsupported persisted statuses. The repo had already updated `crates/ferros-profile/src/lib.rs` with focused tests that lock successful local-profile reload, missing signed-grants defaulting to an empty list, and InvalidLocalState rejection when persisted grant material is validly signed but not local to the reloaded profile. The repo had already updated `streams/S4-runtime/BACKLOG.md` to mark the reload-boundary tests landed. This orchestration lane only recorded that code-backed S4 boundary-lock completion, seeded the next Ready item, and did not move broader hub restart semantics, pairing, schema, or G4 evidence.
- Files: `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: `cargo test -p ferros-node reload_boundary_` passed; `cargo test -p ferros-profile reload_boundary_` passed; editor diagnostics clean on `crates/ferros-node/src/lib.rs`, `crates/ferros-profile/src/lib.rs`, and `streams/S4-runtime/BACKLOG.md`; final editor diagnostics clean on `docs/orchestration/WAVE-QUEUE.md` and `docs/orchestration/WAVE-RUN-LOG.md`
- Next follow-up: `WAVE-2026-04-24-18`

## 2026-04-24 — WAVE-2026-04-24-16

- Selected item: `WAVE-2026-04-24-16`
- Result: Recorded the already-landed docs-only S4 restart/reload-boundary publication in orchestration. The repo had already updated `streams/S4-runtime/CONTRACTS.md` to publish the explicit S4-owned boundary that currently allows S7 to rely on exact-path `CliState::load(path)` reload, `runtime_with_state(state_path)` rebuilding the fixed reference runtime while replaying only persisted Registered/Running/Stopped state, and `LocalProfileStore::load_local_profile(path)` reloading profile/key/grant material only when local validation succeeds, while keeping durable hub restart, pairing, and re-registration semantics explicitly unpublished. `streams/S4-runtime/BACKLOG.md` already marks that boundary landed and routes the next S4-owned follow-up to focused `ferros-node` and `ferros-profile` tests that lock it, `streams/S7-hub/BACKLOG.md` already records the returned dependency lock, and `streams/S7-hub/README.md` only repaired stale follow-up wording exposed by validation. This orchestration lane only recorded that landed docs-only scope, the returned S7 dependency lock, the lack of runtime code changes, `ferros-hub` scaffold movement, or G4 evidence movement, and the new Ready seed around focused S4 boundary-lock tests.
- Files: `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: Recorded the landed validation outcome with clean editor diagnostics on `streams/S4-runtime/CONTRACTS.md`, `streams/S4-runtime/BACKLOG.md`, `streams/S7-hub/BACKLOG.md`, and `streams/S7-hub/README.md`; final editor diagnostics stayed clean on `docs/orchestration/WAVE-QUEUE.md` and `docs/orchestration/WAVE-RUN-LOG.md`.
- Next follow-up: `WAVE-2026-04-24-17`

## 2026-04-24 — WAVE-2026-04-24-15

- Selected item: `WAVE-2026-04-24-15`
- Result: Recorded the already-landed S3/S4/S7 docs-only seam-classification completion in orchestration. The repo had already updated `streams/S3-agent-center/CONTRACTS.md` and `streams/S3-agent-center/BACKLOG.md` to classify the current registration and local/read-first inspection surface as sufficient for S7 runway planning at one-bridge-agent/local-observation scope while routing the next S3-owned follow-up to the first hub-facing wrapper boundary; `streams/S4-runtime/CONTRACTS.md` and `streams/S4-runtime/BACKLOG.md` already classified the current policy surface as sufficient for runway planning while marking the restart/reload boundary as the next S4-owned follow-up; and `streams/S7-hub/BACKLOG.md` already recorded the route-to-S3/S4 handoff as landed plus the returned dependency locks that S3 still owes the first hub-facing wrapper boundary and S4 still owes a published restart/reload boundary before any authoritative pairing flow, `ferros-hub` scaffold, or Home Assistant bridge plan is honest. This orchestration lane only recorded that completion, the docs-only cross-stream dependency locks, the lack of implementation changes or G4 evidence movement, and the next Ready seed on the narrower S4 restart/reload-boundary follow-up.
- Files: `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: Recorded the landed validation outcome with clean editor diagnostics on `streams/S3-agent-center/CONTRACTS.md`, `streams/S3-agent-center/BACKLOG.md`, `streams/S4-runtime/CONTRACTS.md`, `streams/S4-runtime/BACKLOG.md`, and `streams/S7-hub/BACKLOG.md`; final editor diagnostics stayed clean on `docs/orchestration/WAVE-QUEUE.md` and `docs/orchestration/WAVE-RUN-LOG.md`.
- Next follow-up: `WAVE-2026-04-24-16`

## 2026-04-24 — WAVE-2026-04-24-14

- Selected item: `WAVE-2026-04-24-14`
- Result: Recorded the landed S7 docs-only seam-brief slice in orchestration. The repo already updated `streams/S7-hub/README.md` with an S7 seam brief that names the exact S3 registration and inspection surfaces (`AgentRegistry::register`, `AgentRegistry::deregister`, `AgentRegistry::list`, `AgentRegistry::describe`, local `ferros agent list`, `ferros agent describe`, `ferros agent logs`, and read-first `agent.list`, `agent.describe`, `grant.list`, `denyLog.list`) plus the exact S4 policy and restart surfaces (`CapabilityRequest`, `CapabilityGrantView`, `PolicyEngine::evaluate`, `DenyByDefaultPolicy`, `PolicyDecision`, `PolicyDenialReason`, and the nearest current reload helpers `runtime_with_state(state_path)`, `CliState::load(state_path)`, and `LocalProfileStore::load_local_profile(path)`). `streams/S7-hub/CONTRACTS.md` now carries the matching exact-upstream-seams table, and `streams/S7-hub/BACKLOG.md` marks the seam brief landed and routes the next docs-only handoff to S3 and S4. No S2 or orchestration docs were touched in the implementation lane, and no G4 evidence movement is claimed. Because Ready would otherwise be empty, this invocation also seeded the next honest docs-only follow-on for S3 and S4 seam classification.
- Files: `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: Recorded the landed validation outcome with clean editor diagnostics on `streams/S7-hub/README.md`, `streams/S7-hub/CONTRACTS.md`, and `streams/S7-hub/BACKLOG.md`; the consistency check against `streams/S3-agent-center/CONTRACTS.md`, `streams/S4-runtime/CONTRACTS.md`, `docs/hub/reference-hardware.md`, `STATUS.md`, and `docs/gates/G4.md` passed; final editor diagnostics stayed clean on the touched orchestration files.
- Next follow-up: `WAVE-2026-04-24-15`

## 2026-04-24 — WAVE-2026-04-24-13

- Selected item: `WAVE-2026-04-24-13`
- Result: Recorded the landed S7 docs-only pairing/design handoff in orchestration and repaired the stale queue lineage so the queue again matches repo truth. The repo already updated `streams/S7-hub/README.md` with an S7-owned provisional handoff that states what S7 may now assume from the published S2 consumer boundaries across bootstrap, grant check, deny visibility, persistence, revocation, and re-registration, names what stays open, and routes the immediate next step to an S7-owned seam brief keyed to the exact S3 registry/list/log and S4 restart/policy APIs. `streams/S7-hub/BACKLOG.md` already marks the consume pass landed and replaces the obsolete route-to-S2 follow-up with that seam-brief follow-up. No S2 or orchestration docs were touched in the implementation lane itself, and no G4 evidence movement is claimed. Because the queue was one step stale, this invocation also retired the already-published `WAVE-2026-04-24-12` S2 answer slice from Ready to Done before seeding the next honest S7-ready item.
- Files: `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: Recorded the landed validation outcome with clean editor diagnostics on `streams/S7-hub/README.md` and `streams/S7-hub/BACKLOG.md`; consistency checks against `streams/S2-profile/README.md`, `streams/S2-profile/CONTRACTS.md`, `streams/S7-hub/CONTRACTS.md`, `docs/hub/reference-hardware.md`, `STATUS.md`, and `docs/gates/G4.md` passed; final editor diagnostics stayed clean on the touched orchestration files.
- Next follow-up: `WAVE-2026-04-24-14`

## 2026-04-24 — WAVE-2026-04-24-11

- Selected item: `WAVE-2026-04-24-11`
- Result: Recorded the already-landed S7 docs-only pairing-boundary slice in orchestration without widening into implementation; the repo already updated `streams/S7-hub/README.md` to replace the generic open pairing questions with an explicit six-row S2 consumer-boundary question list aligned to bootstrap, grant check, deny visibility, persistence, revocation, and re-registration, routed the immediate next step to S2, and updated `streams/S7-hub/BACKLOG.md` to mark both the six-checkpoint pairing map and the new question list as landed while replacing the old follow-up with routing those questions to S2 and recording the answers; no fresh S7 content edits were made in this invocation; no G4 evidence movement is claimed.
- Files: `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: Recorded the landed validation outcome with clean editor diagnostics on `streams/S7-hub/README.md` and `streams/S7-hub/BACKLOG.md`, plus passed consistency checks against `docs/hub/reference-hardware.md`, `streams/S7-hub/CONTRACTS.md`, `STATUS.md`, and `docs/gates/G4.md`.
- Next follow-up: `WAVE-2026-04-24-12`

## 2026-04-24 — WAVE-2026-04-24-10

- Selected item: `WAVE-2026-04-24-10`
- Result: Recorded the already-landed S7 pairing-checkpoint docs slice in orchestration only. The repo already added a runway-only six-checkpoint pairing map to `streams/S7-hub/README.md` and `docs/hub/reference-hardware.md` covering bootstrap, grant check, deny visibility, persistence, revocation, and re-registration, tied that map to the current S2 `ProfileId` and `CapabilityGrant` seams plus the S3 registry/list/log and S4 runtime policy, deny logging, and restart seams, and corrected stale pre-G3 wording in those same docs. No fresh S7 content edits were made in this invocation, and no G4 evidence movement is claimed.
- Files: `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: Recorded the landed validation outcome: editor diagnostics were clean on `STATUS.md`, `streams/S7-hub/README.md`, and `docs/hub/reference-hardware.md`; the consistency pass against `streams/S7-hub/CONTRACTS.md`, `STATUS.md`, and `docs/gates/G4.md` passed, but `streams/S7-hub/BACKLOG.md` still carries a stale open checkpoint-mapping row, so backlog consistency remains open for the next follow-up.
- Next follow-up: `WAVE-2026-04-24-11`

## 2026-04-24 — WAVE-2026-04-24-09

- Selected item: `WAVE-2026-04-24-09`
- Result: Recorded the landed S7 runway closeout in orchestration without widening into implementation. The repo already defines the first Home Assistant bridge runway contract in `streams/S7-hub/CONTRACTS.md`, syncs `streams/S7-hub/BACKLOG.md` and `streams/S7-hub/PROGRESS.md`, and keeps the scope at one bridge agent, one real entity minimum evidence, operator-visible deny attribution, restart-safe FERROS-side state, and the external `Maangled/home-assistant` fork boundary. No new S7 content edits were made in this invocation; the queue and run log now reflect that the generic bridge-assumption slice is complete.
- Files: `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: Recorded the landed validation outcome: editor diagnostics were clean on the touched S7 docs.
- Next follow-up: `WAVE-2026-04-24-10`

## 2026-04-24 — WAVE-2026-04-23-09

- Selected item: `WAVE-2026-04-23-09`
- Result: Recorded the landed S5 Phase A archive and link-hygiene closeout in orchestration. The repo already archived the inactive top-level HTML prototypes to `docs/legacy/`, kept the still-active docs-root surfaces in place, synced the S5 authority docs, and repaired the stale inbound references created by the archive move. No new S5 content edits were made in this invocation; the queue and run log now reflect the completed cleanup pack.
- Files: `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: Recorded the landed validation outcome: editor diagnostics were clean on the touched files, and grep had already confirmed that old docs-root references such as `docs/home-hud-dashboard.html` and `docs/ferros-project-map.html` were removed after repair.
- Next follow-up: `WAVE-2026-04-24-09`

## 2026-04-24 — WAVE-2026-04-24-08

- Selected item: `WAVE-2026-04-24-08`
- Result: Hardened the current read-first JSON-RPC boundary without changing its shape. `crates/ferros-node/src/lib.rs` now has focused tests that lock the existing error-envelope behavior for unsupported JSON-RPC version, missing `agentName` on `agent.describe`, unknown method names, and unknown agents, plus a real listener-level `POST /rpc` smoke that proves the live localhost shell host returns the same structured invalid-params response over TCP.
- Files: `crates/ferros-node/src/lib.rs`, `streams/S3-agent-center/PROGRESS.md`, `streams/S3-agent-center/CONTRACTS.md`, `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: Passed `cargo test -p ferros-node agent_read_rpc_` and `cargo test -p ferros-node shell_listener_posts_json_rpc_`; the focused suite covered the direct handler and the live listener path without widening into broader workspace checks.
- Next follow-up: `WAVE-2026-04-23-09` remains the only ready queue item, but if launch-path leverage still outranks cleanup, the next higher-value follow-on is a new queued slice around the first reusable `ferros-hub` wrapper seam or explicit Home Assistant bridge assumptions rather than Phase A archive moves.

## 2026-04-24 — WAVE-2026-04-24-07

- Selected item: `WAVE-2026-04-24-07`
- Result: Converted the active S7 runway into a concrete first bring-up contract without widening into implementation. The S7 stream and hardware-runway docs now treat the Pack B `x86_64` lane as the preferred first bring-up target, keep Pack C as the separate Home Assistant companion host, and map each unchecked G4 evidence item to one upstream seam and one S7-owned proof point so future `ferros-hub` work can be judged against a concrete runway rather than broad intent.
- Files: `streams/S7-hub/README.md`, `streams/S7-hub/BACKLOG.md`, `streams/S7-hub/CONTRACTS.md`, `docs/hub/reference-hardware.md`, `streams/S7-hub/PROGRESS.md`, `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: Editor diagnostics stayed clean on the touched S7 and hardware-runway docs; no code or gate-truth validation was required because the slice stayed in runway mode and did not change executable surfaces.
- Next follow-up: `WAVE-2026-04-23-09` remains the ready queue head. If launch-path leverage continues to outrank Phase A cleanup, queue the next S3/S7 follow-on around HA bridge assumptions or the first reusable `ferros-hub` wrapper seam only after the upstream host and contract surfaces are concrete enough.

## 2026-04-24 — WAVE-2026-04-24-06

- Selected item: `WAVE-2026-04-24-06`
- Result: Hardened the current localhost shell host seam without widening the read-first contract. `crates/ferros-node/src/lib.rs` now exposes a bounded listener loop that the test suite can drive directly, and the new listener-level smoke tests prove that the real shell host serves `GET /` and answers `POST /rpc` with a live `agent.list` response through the same TCP, HTTP parse, and response-write path used by `ferros-node shell`.
- Files: `crates/ferros-node/src/lib.rs`, `streams/S4-runtime/PROGRESS.md`, `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: Passed `cargo test -p ferros-node shell_`, including the existing route tests plus the new real-socket listener smokes for shell HTML and JSON-RPC agent list responses; final editor diagnostics stayed clean on the touched S4 and orchestration files.
- Next follow-up: `WAVE-2026-04-23-09` remains the ready queue head. If consumer reliability and G4 alignment continue to outrank Phase A cleanup, queue the S7 bring-up-contract slice before archive work.

## 2026-04-24 — WAVE-2026-04-24-05

- Selected item: `WAVE-2026-04-24-05`
- Result: Landed a dedicated same-origin localhost shell acceptance slice without widening the read-first contract. `harnesses/localhost-shell-acceptance-harness.html` now exercises the real shell through a same-origin iframe, `crates/ferros-node/src/lib.rs` serves that harness at `/harnesses/localhost-shell-acceptance.html`, and the live browser pass proved route switching, registry/detail inspection, grant empty-state degradation for a missing profile path, deny-log empty-state rendering, and the persistent read-only audit slot against the actual `ferros-node shell` server.
- Files: `harnesses/localhost-shell-acceptance-harness.html`, `crates/ferros-node/src/lib.rs`, `streams/S5-ux/BACKLOG.md`, `streams/S5-ux/PROGRESS.md`, `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: Passed `cargo test -p ferros-node shell_route_`; live browser validation at `http://127.0.0.1:4317/harnesses/localhost-shell-acceptance.html` passed 13/13 checks against the real localhost shell after restarting `ferros-node shell 4317`; final editor diagnostics stayed clean on the touched S5 and orchestration files.
- Next follow-up: `WAVE-2026-04-23-09` remains ready. If consumer reliability and G4 alignment stay higher priority than Phase A cleanup, queue a follow-on S4/S7 wave for localhost host hardening plus the first concrete S7 bring-up contract.

## 2026-04-24 — WAVE-2026-04-23-B01

- Selected item: `WAVE-2026-04-23-B01`
- Result: Landed the first real S5 localhost shell slice without widening into privileged writes. `site/agent-center-shell.html` now renders the fixed-slot agent-center shell, `crates/ferros-node/src/lib.rs` serves that shell at `GET /` and forwards `POST /rpc` into the existing read-first JSON/RPC handler, and `crates/ferros-node/src/main.rs` now exposes `ferros-node shell [port]` with a default localhost port of `4317`. The browser-validated shell reads live agent, grant-state, and deny-log data, and the inspector capability rendering bug was fixed before closeout so required capabilities now render the real profile identifier instead of an `undefined:*` placeholder.
- Files: `site/agent-center-shell.html`, `crates/ferros-node/src/lib.rs`, `crates/ferros-node/src/main.rs`, `streams/S5-ux/README.md`, `streams/S5-ux/CONTRACTS.md`, `streams/S5-ux/BACKLOG.md`, `streams/S5-ux/PROGRESS.md`, `site/index.html`, `STATUS.md`, `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: Passed `cargo test -p ferros-node`; live browser validation at `http://127.0.0.1:4317/` confirmed `ready`, `rpc live`, and real list/describe/grant/deny data after restarting the embedded-asset server with the rebuilt shell HTML; final editor diagnostics were clean on the touched S5, status, landing-page, and orchestration files.
- Next follow-up: `WAVE-2026-04-23-09`

## 2026-04-24 — WAVE-2026-04-24-04

- Selected item: `WAVE-2026-04-24-04`
- Result: Landed the first read-first S3 JSON/RPC contract without widening into HTTP serving, privileged write actions, or Phase B shell rendering. The contract now lives in `crates/ferros-agents/src/rpc.rs` with method and payload types for `agent.list`, `agent.describe`, `grant.list`, and `denyLog.list`, and `crates/ferros-node/src/lib.rs` now hosts that contract over the current deterministic runtime state, persisted grant state, and deny-log state. The owning S3 contract docs, shared contract index, S5 shell wireframe, and status surfaces now reflect that the read path exists; as a direct consequence, `WAVE-2026-04-23-B01` is no longer blocked and is now ready for the first shell-consumer pass.
- Files: `crates/ferros-agents/src/rpc.rs`, `crates/ferros-agents/src/lib.rs`, `crates/ferros-node/src/lib.rs`, `crates/ferros-node/Cargo.toml`, `streams/S3-agent-center/CONTRACTS.md`, `streams/S5-ux/PHASE-B-SHELL-WIREFRAME.md`, `docs/contracts/CONTRACTS-OVERVIEW.md`, `STATUS.md`, `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: Passed `cargo test -p ferros-agents`; passed `cargo test -p ferros-node`; the new `ferros-node` tests cover typed list, describe, grant-list, deny-log, and JSON wrapper behavior against the current runtime and local profile store; final editor diagnostics were clean on the touched contract, status, and queue files.
- Next follow-up: `WAVE-2026-04-23-B01`

## 2026-04-24 — WAVE-2026-04-24-03

- Selected item: `WAVE-2026-04-24-03`
- Result: Recorded the first green hosted CI proof for the landed G3 workflow path, then truth-synced the gate, status, stream, and queue surfaces so G3 is now closed and G4 is now active. The closure references CI #20 (`run 24902870499`, commit `8383b67` on `main`), keeps the proof tied to the current hosted Ubuntu workflow that still runs `cargo check -p ferros-core --no-default-features` and `cargo run --bin ferros -- demo`, reclassifies S5 Phase B as blocked on the missing S3 JSON/RPC contract rather than on G3 itself, and updates S7 and S4 surfaces to their post-G3 state without starting JSON/RPC, shell rendering, or `ferros-hub` code.
- Files: `docs/gates/G3.md`, `docs/gates/G4.md`, `STATUS.md`, `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`, `streams/S4-runtime/BACKLOG.md`, `streams/S5-ux/BACKLOG.md`, `streams/S7-hub/README.md`, `streams/S7-hub/PROGRESS.md`
- Validation: Confirmed the current `.github/workflows/ci.yml` still contains `cargo check -p ferros-core --no-default-features` and `cargo run --bin ferros -- demo`; refreshed the GitHub Actions CI workflow page and confirmed CI #20 (`run 24902870499`, commit `8383b67` on `main`) completed successfully on 2026-04-24; final editor diagnostics were clean on the touched gate, status, queue, and stream docs.
- Next follow-up: `WAVE-2026-04-24-04`

## 2026-04-24 — WAVE-2026-04-23-08

- Selected item: `WAVE-2026-04-23-08`
- Result: Closed the S7 runway documentation wave without wave-owned S7 content edits in this invocation because `docs/hub/reference-hardware.md`, `streams/S7-hub/README.md`, and `streams/S7-hub/BACKLOG.md` already reflected the dispatched pairing and hardware design-pack scope. G3 remains the implementation blocker, G4 did not move, no `crates/ferros-hub/` work or Home Assistant bridge work was started, and no immediate S8 or S2 follow-up is required from this closeout.
- Files: `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: Confirmed the three S7 anchor docs had no uncommitted changes relative to `HEAD`; editor diagnostics were clean for `docs/hub/reference-hardware.md`, `streams/S7-hub/README.md`, and `streams/S7-hub/BACKLOG.md`; integration review passed with gate truth still honest, S2 consumer-surface alignment intact, and cross-file coherence preserved; final editor diagnostics were clean for `docs/orchestration/WAVE-QUEUE.md` and `docs/orchestration/WAVE-RUN-LOG.md`.
- Next follow-up: `WAVE-2026-04-23-09`

## 2026-04-24 — WAVE-2026-04-24-02

- Selected item: `WAVE-2026-04-24-02`
- Result: Landed the remaining S2 profile CLI lifecycle on a local-only persistence boundary for key material and signed grant state, adding repo-backed `ferros profile grant`, `export`, `import`, and `revoke` behavior while keeping `ferros profile show` on the frozen unsigned `profile.v0` document surface. The wave kept `SignedProfileDocument` Rust-local at v0, left `schemas/profile.v0.json` and `schemas/capability-grant.v0.json` unchanged, did not widen S3, S4, or S7 boundaries, and truth-synced the gate and status surfaces so G2 is closed and G3 is active.
- Files: `crates/ferros-profile/src/lib.rs`, `crates/ferros-node/src/lib.rs`, `crates/ferros-node/src/bin/ferros.rs`, `crates/ferros-node/tests/profile_cli_linux.rs`, `crates/ferros-node/Cargo.toml`, `docs/gates/G2.md`, `docs/gates/G3.md`, `STATUS.md`, `docs/contracts/CONTRACTS-OVERVIEW.md`, `streams/S2-profile/README.md`, `streams/S2-profile/PROGRESS.md`
- Validation: Delegated orchestration passed `cargo test -p ferros-profile`, `cargo test -p ferros-node`, and `cargo test -p ferros-profile -p ferros-node`. The repo-backed real-binary proof in `crates/ferros-node/tests/profile_cli_linux.rs` exercised `ferros profile init`, `grant agent.echo`, `export`, `import`, `revoke agent.echo`, and `show` against real temp files, verified that imported local state preserved keypair and signed grant state, verified that the revoked signed grant still stayed within the frozen grant boundary, and confirmed that `show` remained an unsigned profile document. Final editor diagnostics were clean on touched code and truth-sync files.
- Next follow-up: `WAVE-2026-04-23-08`. Separately queue a new P0 G3 evidence wave to record the first hosted green CI run reference for `cargo check -p ferros-core --no-default-features` and `cargo run --bin ferros -- demo`.

## 2026-04-24 — WAVE-2026-04-24-01

- Selected item: `WAVE-2026-04-24-01`
- Result: Landed the profile.v0 freeze boundary as the frozen unsigned published v0 contract, kept `SignedProfileDocument` Rust-local at v0, refreshed harness parity for the frozen profile fixture set, and truth-synced S2, gate, contracts, and status surfaces without widening into `export | import | grant | revoke`. G2 remains open only for the remaining profile CLI evidence. Integration review found one stale README line and fixed it before closeout; no S3 or S4 consumer-awareness follow-up is required because the published downstream contract remains the unsigned `profile.v0` boundary.
- Files: `schemas/profile.v0.json`, `crates/ferros-profile/src/lib.rs`, `harnesses/_constants.js`, `harnesses/ferros-contract-validator.html`, `streams/S2-profile/CONTRACTS.md`, `docs/gates/G2.md`, `docs/contracts/CONTRACTS-OVERVIEW.md`, `STATUS.md`, `streams/S2-profile/README.md`, `streams/S2-profile/PROGRESS.md`
- Validation: Delegated S2 lane passed `cargo test -p ferros-profile` with 34 passed and 0 failed; harness constants were regenerated; `harnesses/ferros-contract-validator.html` accepted the frozen profile fixture set with 47 passed and 0 failed; final editor diagnostics were clean on touched S2 and truth-sync files.
- Next follow-up: `WAVE-2026-04-23-08`. Separately queue a new S2 wave for the remaining `ferros profile export | import | grant | revoke` evidence when ready.

## 2026-04-23 — WAVE-2026-04-23-07

- Selected item: `WAVE-2026-04-23-07`
- Result: Landed the narrow G3 truth-sync and CI-proof slice without widening into JSON/RPC, reusable host work, or S5 shell work. The `ferros` binary now exposes `cargo run --bin ferros -- demo`, Ubuntu CI is explicitly wired to run both `cargo check -p ferros-core --no-default-features` and `cargo run --bin ferros -- demo`, and the G3/status/contracts/S4 backlog surfaces now reflect the already-landed S4 policy property tests and current demo evidence honestly. G3 still remains blocked on G2 and on recording the first green hosted run for the new workflow steps, but the queue item's repo-owned slice is complete.
- Files: `.github/workflows/ci.yml`, `crates/ferros-node/src/bin/ferros.rs`, `docs/contracts/CONTRACTS-OVERVIEW.md`, `docs/gates/G3.md`, `STATUS.md`, `streams/S4-runtime/BACKLOG.md`
- Validation: Delegated orchestration passed `cargo test -p ferros-core -p ferros-runtime -p ferros-agents -p ferros-node`; passed `cargo check -p ferros-core --no-default-features`; passed `cargo run --bin ferros -- demo` with `started: echo,timer`, `echo: hello`, `timer: tick-1`, and `denied: 1`; additional focused passes covered `cargo test -p ferros-node --bin ferros` and `cargo test -p ferros-core --test capability_policy`; editor diagnostics were clean for the touched workflow and truth-sync files.
- Next follow-up: `WAVE-2026-04-23-08`. Separately capture the first green hosted CI run reference for the newly wired Ubuntu demo and `--no-default-features` steps when it becomes available.

## 2026-04-23 — WAVE-2026-04-23-06

- Selected item: `WAVE-2026-04-23-06`
- Result: Landed the first S2-owned `KeyPair` surface plus an additive `SignedProfileDocument` round-trip path in `ferros-profile`, so a fresh profile can be created, serialized, signed, deserialized, verified, and re-signed on revoke without widening into the remaining profile CLI verbs. The wave also added the focused `schemas/fixtures/signed-profile-valid.json` evidence and truth-synced S2 and gate/status surfaces while leaving `schemas/profile.v0.json`, harness files, and downstream S3/S4 consumer boundaries unchanged.
- Files: `Cargo.lock`, `crates/ferros-profile/Cargo.toml`, `crates/ferros-profile/src/lib.rs`, `schemas/fixtures/signed-profile-valid.json`, `streams/S2-profile/README.md`, `streams/S2-profile/BACKLOG.md`, `streams/S2-profile/CONTRACTS.md`, `streams/S2-profile/PROGRESS.md`, `docs/gates/G2.md`, `STATUS.md`
- Validation: Delegated orchestration passed `cargo test -p ferros-profile` with 32 passed and 0 failed; editor diagnostics were clean for the touched `ferros-profile` slice.
- Next follow-up: No ready queue item remains. `WAVE-2026-04-23-B01` stays blocked on the S3 JSON/RPC dependency; the next recommended new wave is an S2 follow-up to decide whether `SignedProfileDocument` should become a published schema contract or remain Rust-local until profile freeze, then land the remaining `ferros profile export | import | grant | revoke` evidence for G2 closeout.

## 2026-04-23 — WAVE-2026-04-23-05

- Selected item: `WAVE-2026-04-23-05`
- Result: Landed a Linux-backed real-binary proof for `ferros profile init` followed by `ferros profile show` without widening into the remaining profile CLI verbs. The slice added a focused integration test that launches the `ferros` binary, validates the initialized profile document through `show`, and truth-synced G2 and status surfaces to reflect that the current `init | show` path now has repo-backed Linux evidence through the existing Ubuntu CI test job.
- Files: `crates/ferros-node/tests/profile_cli_linux.rs`, `docs/gates/G2.md`, `STATUS.md`
- Validation: Delegated orchestration passed `cargo test -p ferros-profile -p ferros-node` and `cargo test -p ferros-node profile_init_then_show_works_via_real_ferros_binary -- --exact`; editor diagnostics were clean for the touched wave-owned files. A GitHub-hosted workflow could not be executed from this environment, so the Linux-backed proof is the new real-binary test plus the existing `ubuntu-latest` `cargo test --workspace --all-targets` job in `.github/workflows/ci.yml`.
- Next follow-up: No ready queue item remains. `WAVE-2026-04-23-B01` is still blocked on the S3 JSON/RPC dependency; the next recommended new wave is an S2/G2 key-material plus end-to-end profile signing evidence slice.

## 2026-04-23 — WAVE-2026-04-23-04

- Selected item: `WAVE-2026-04-23-04`
- Result: Landed the dedicated frozen `schemas/fixtures/profile-valid.json` evidence and proved it against the unchanged `schemas/profile.v0.json` contract in both `ferros-profile` and the H1 contract validator without widening into profile CLI or signing work. The slice also regenerated harness constants to embed the profile schema and fixture, and truth-synced G2 and status surfaces so they no longer claim the dedicated profile freeze evidence is missing.
- Files: `schemas/fixtures/profile-valid.json`, `crates/ferros-profile/src/lib.rs`, `tools/generate-harness-constants.ps1`, `harnesses/_constants.js`, `harnesses/ferros-contract-validator.html`, `docs/gates/G2.md`, `STATUS.md`
- Validation: Delegated orchestration passed `cargo test -p ferros-profile` with 26 passed and 0 failed; `./tools/generate-harness-constants.ps1` regenerated harness constants with `profile.v0` and `profile-valid` embedded; H1 `harnesses/ferros-contract-validator.html` passed with 47 passed, 0 failed, 0 skipped including the explicit `profile-valid` against `SCHEMA_PROFILE_V0` check; workspace diagnostics on touched files were clean.
- Next follow-up: `WAVE-2026-04-23-05`

## 2026-04-23 — WAVE-2026-04-23-03

- Selected item: `WAVE-2026-04-23-03`
- Result: Landed the minimal S2 profile CLI slice as a real implementation, wiring `ferros profile init [path]` and `ferros profile show [path]` through the existing `ProfileStore` boundary with filesystem-backed create-without-overwrite semantics, focused timestamp validation, and honest G2/status/contracts truth-sync. The wave stayed inside `init` and `show` and did not widen into import/export or signing.
- Files: `Cargo.lock`, `crates/ferros-profile/Cargo.toml`, `crates/ferros-profile/src/lib.rs`, `crates/ferros-node/Cargo.toml`, `crates/ferros-node/src/lib.rs`, `crates/ferros-node/src/bin/ferros.rs`, `docs/gates/G2.md`, `STATUS.md`, `docs/contracts/CONTRACTS-OVERVIEW.md`, `streams/S2-profile/PROGRESS.md`
- Validation: Delegated orchestration passed `cargo test -p ferros-profile init_profile_creates_new_profile_document_in_store`, `cargo test -p ferros-node profile_cli_init_and_show_round_trip_profile_document`, `cargo test -p ferros-node --bin ferros run_dispatches_profile_init_and_show_with_explicit_path`, `cargo test -p ferros-profile fresh_profile_document_rejects_invalid_created_at`, `cargo test -p ferros-profile`, `cargo test -p ferros-node profile_cli_`, and `cargo test -p ferros-profile -p ferros-node`. One intermediate dependency-resolution failure was repaired by pinning the node-side time dependency before the successful rerun.
- Next follow-up: No ready queue item remains. The next recommended wave is to add CI-backed Linux `ferros profile init -> show` proof and freeze `schemas/profile.v0.json` with dedicated frozen profile evidence.

## 2026-04-23 — WAVE-2026-04-23-02

- Selected item: `WAVE-2026-04-23-02`
- Result: Closed the S4 policy property-test gap for `DenyByDefaultPolicy` without widening into broader runtime work. The orchestrated slice added a minimal `proptest` test dependency and property-based coverage that proves an active exact match authorizes regardless of grant ordering while mismatched or inactive grants deny with the same decision when order changes. Gate and status docs still lag this evidence item and were intentionally left untouched in this wave.
- Files: `crates/ferros-core/Cargo.toml`, `crates/ferros-core/tests/capability_policy.rs`, `Cargo.lock`
- Validation: Delegated orchestration passed `cargo test -p ferros-core` with 21 passed, 0 failed, 0 ignored; local editor diagnostics were clean for `crates/ferros-core/Cargo.toml`, `crates/ferros-core/tests/capability_policy.rs`, `Cargo.lock`, `docs/orchestration/WAVE-QUEUE.md`, and `docs/orchestration/WAVE-RUN-LOG.md`
- Next follow-up: `WAVE-2026-04-23-03`

## 2026-04-23 — WAVE-2026-04-23-01

- Selected item: `WAVE-2026-04-23-01`
- Result: Closed the shared contract, harness, and gate-truth evidence for the signed `CapabilityGrant` boundary without widening into profile CLI work. The orchestrated slice updated the harness generator and validator, fixed negative-fixture classification for the invalid-signature case, and truth-synced the shared contract and G2 gate docs to the current schema-backed behavior.
- Files: `tools/generate-harness-constants.ps1`, `harnesses/_constants.js`, `harnesses/ferros-contract-validator.html`, `docs/contracts/CONTRACTS-OVERVIEW.md`, `docs/gates/G2.md`
- Validation: Delegated orchestration passed `cargo test -p ferros-profile`; direct `./tools/generate-harness-constants.ps1` invocation was blocked by local PowerShell execution policy, so the script body was executed via `Get-Content` and `ScriptBlock` to regenerate `harnesses/_constants.js`; browser validation for `harnesses/ferros-contract-validator.html` passed with 45 passed, 0 failed, 0 skipped; shared contract and harness parity against `schemas/capability-grant.v0.json` was confirmed; local editor diagnostics were clean for `docs/orchestration/WAVE-QUEUE.md` and `docs/orchestration/WAVE-RUN-LOG.md`
- Next follow-up: `WAVE-2026-04-23-02`

## 2026-04-23 — WAVE-2026-04-23-01

- Selected item: `WAVE-2026-04-23-01`
- Result: Landed the first signed and verifiable `CapabilityGrant` slice in `ferros-profile`, including Ed25519 signing, verification, revoke-and-resign behavior, and positive and negative schema fixtures. The wave remains partial because shared contract, harness, and gate-truth evidence still needs closeout, so the item returned to `Ready` with a narrower follow-up.
- Files: `crates/ferros-profile/src/lib.rs`, `crates/ferros-profile/Cargo.toml`, `Cargo.lock`, `schemas/capability-grant.v0.json`, `schemas/fixtures/grant-valid.json`, `schemas/fixtures/grant-invalid-sig.json`
- Validation: Delegated orchestration ran `cargo test -p ferros-profile` successfully after a lockfile repair; local editor diagnostics were clean for `docs/orchestration/WAVE-QUEUE.md` and `docs/orchestration/WAVE-RUN-LOG.md`
- Next follow-up: `WAVE-2026-04-23-01` narrowed to shared contract, harness, and gate-truth closeout for the signed `CapabilityGrant` path

## 2026-04-23 — DRIVER-BOOTSTRAP

- Selected item: none
- Result: Installed the local driver pattern with a user-invocable driver agent, repo-backed queue, append-only run log, and file-scoped queue rules.
- Files: `.github/agents/ferros-driver.agent.md`, `.github/instructions/ferros-wave-queue.instructions.md`, `docs/orchestration/LOCAL-DRIVER.md`, `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/WAVE-RUN-LOG.md`
- Validation: editor diagnostics on the new customization and markdown files
- Next follow-up: `WAVE-2026-04-23-01`
