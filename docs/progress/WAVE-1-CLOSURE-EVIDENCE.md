# Wave 1 Closure Evidence

**Wave 1 declared CLOSED — 2026-04-22**

This document is the permanent record that Wave 1 (V1–V8, Vertical Slice) was verified and closed. It supplements the Wave 0 closure record at [`CLOSURE-EVIDENCE.md`](../../CLOSURE-EVIDENCE.md) and is the final delivery artifact for Phase A Foundation Finalization.

Tracking umbrella: [Phase A — Foundation Finalization #62](https://github.com/Maangled/ferros/issues/62)

---

## Wave 1 Exit Criteria — Final Status

| # | Capability | Status | Evidence |
|---|-----------|--------|---------|
| V1 | Template → Profile creation completes without soft-lock | ✅ | Journey 1; H5 PASS — 30/30 (2026-04-22) |
| V2 | Export → Import produces identical profile in fresh browser | ✅ | Journey 1; H2 PASS — 25/25 (2026-04-22) |
| V3a | All 4 valid session modes complete expected flows (H2 round-trip) | ✅ | H2 PASS — 25/25 (2026-04-22) |
| V3b | Invalid session mode combinations rejected with correct error codes | ✅ | H4 PASS — 26/26 (2026-04-22) |
| V4 | Alias session → export → claim → XP merge works | ✅ | Commit `8d7c123`; `ferros-core.js` line 886+; `acceptance-harness.html` line 442+ |
| V5 | Card loads in Forge → editable → renders in Runtime | ✅ | Journey 3; H5 PASS — 30/30 (2026-04-22) |
| V6 | Runtime init/update/event loop completes | ✅ | Journey 3; H3 PASS — 18/18 (2026-04-18) |
| V7 | Card round-trip export/import preserves all parameters | ✅ | `schemas/fixtures/card-deck-roundtrip.json`; H1 PASS — 30/30 (2026-04-22) |
| V8 | Phase 0 acceptance harness (H5) proves Journey 1 end-to-end | ✅ | H5 PASS — 30/30 (2026-04-22) |

All V1–V8 capabilities verified. Wave 1 exit criteria met.

---

## Harness Matrix — At Wave 1 Closure

Verified in Chrome via `file://`. Reference commit: `1a93ece` (merge of PR #66, base of PR 11). V4 implementation present via `8d7c123`.

| Harness | File | Role | Result | Date |
|---------|------|------|--------|------|
| H1 | `harnesses/ferros-contract-validator.html` | C1–C7 gate | ✅ PASS — 30/30 | 2026-04-22 |
| H2 | `harnesses/round-trip-harness.html` | C9 gate | ✅ PASS — 25/25 | 2026-04-22 |
| H3 | `harnesses/runtime-harness.html` | C8 gate | ✅ PASS — 18/18 | 2026-04-18 |
| H4 | `harnesses/negative-harness.html` | C10 gate | ✅ PASS — 26/26 | 2026-04-22 |
| H5 | `harnesses/acceptance-harness.html` | V1/V8 supporting | ✅ PASS — 30/30 | 2026-04-22 |
| H6 | `harnesses/write-path-harness.html` | C9 write supporting | ✅ PASS — 25/25 | 2026-04-19 |
| H7 | `harnesses/semantic-fixture-linter.html` | C2/C4/C5/C6 supporting | ✅ PASS — 107/107 | 2026-04-19 |
| H8 | `harnesses/ui-acceptance-harness.html` | C10/UI supporting | ✅ PASS — 17/17 | 2026-04-21 |
| H9 | `harnesses/consumer-helper-harness.html` | C7/C9/C10 helpers | ✅ PASS — 28/28 | 2026-04-22 |
| Preflight | `harnesses/preflight-check.html` | file:// readiness | ✅ PASS — 6/6 | 2026-04-19 |

H8 environmental context: see [`docs/progress/H8-RERUN-ENV-NOTES.md`](./H8-RERUN-ENV-NOTES.md).

---

## V4 — Alias → Claim → XP Merge Evidence

**Capability:** V4 — Alias session → export → claim → XP merge  
**Status:** ✅ Resolved  
**Implementation commit:** `8d7c123` — `Add portable alias/recovery log claim support`  
**Primary artifact:** `docs/assets/_core/ferros-core.js` — alias-claim logic and XP merge at line 886+  
**Acceptance coverage:** `harnesses/acceptance-harness.html` — alias-claim end-to-end assertions at line 442+  
**Schema fixtures:**

- `schemas/fixtures/alias-session-log.json` — C1 golden fixture (alias session)
- `schemas/fixtures/recovery-session-log.json` — C1 golden fixture (recovery session)
- `schemas/fixtures/invalid-duplicate-alias-claim.json` — C1 negative fixture (duplicate claim guard)

V4 closes the last ⬜ Tier 2 capability. All Wave 1 vertical-slice capabilities (V1–V8) are now ✅.

---

## V7 — Card Round-Trip Evidence

**Capability:** V7 — Card round-trip export/import preserves all parameters  
**Status:** ✅ Resolved  
**Fixture:** [`schemas/fixtures/card-deck-roundtrip.json`](../../schemas/fixtures/card-deck-roundtrip.json) — golden fixture exercising C4/C5 round-trip  
**Harness coverage:** H1 validates `card-deck-roundtrip.json` against `card.schema.json` + `deck.schema.json`; H2 round-trip verifies storage preservation  
**Surface:** `docs/forge-workbench.html` — Forge → export → import → re-render loop, verified in Journey 3 (H5 group)

---

## Audit Remediation — Wave 1 Final Dispositions

These are the two audit findings from the Phase 0 exit audit that carried "Deferred → Wave 1" status at Wave 0 closure. Both receive final dispositions here as part of Phase A Wave 1 closure.

### Finding #9 — Card/deck not included in export

| Field | Value |
|-------|-------|
| Finding | Card/deck objects not included in export envelope |
| Prior status | ⬜ Deferred → Wave 1 (V5-V7) |
| **Disposition** | **✅ Resolved** |
| Evidence | V5/V6/V7 complete (Journey 3 end-to-end). Card round-trip verified by `schemas/fixtures/card-deck-roundtrip.json` (H1 PASS 30/30) and Journey 3 group in H5 (PASS 30/30). The Forge export → Arena Runtime import flow exercises the card/deck inclusion path. |
| Closed by | PR 11 (this PR) — 2026-04-22 |

### Finding #19 — Contract/fixture co-location fragmented

| Field | Value |
|-------|-------|
| Finding | Contract schemas and fixture files are in separate directories (`schemas/` vs `schemas/fixtures/`) rather than co-located per contract |
| Prior status | ⬜ Deferred → Wave 1 |
| **Disposition** | **Deferred — Wave 3+ housekeeping** |
| Rationale | `docs/contracts/manifest.json` provides full cross-referenced navigation by contract (schema file, fixture files, gate harness, contract doc). Physical co-location would be a mechanical reorg with no correctness or enforcement impact. Low value relative to coordination cost of reshuffling all fixture paths in harnesses and generators. Deferred to a future housekeeping PR tagged `housekeeping`. |
| Closed by | PR 11 (this PR) — final disposition recorded 2026-04-22 |

---

## Phase A PR Summary

All four Phase A Foundation Finalization PRs are accounted for at Wave 1 closure.

| PR | Issue item | Commit / PR # | Status |
|---|---|---|---|
| PR 8 | A2 — H8 clean rerun + environmental notes | Merged PR [#60](https://github.com/Maangled/ferros/pull/60) | ✅ Merged |
| PR 9 | A1 — H9 consumer-helper harness | Merged PR [#64](https://github.com/Maangled/ferros/pull/64) | ✅ Merged |
| PR 10 | A4 — V4 alias/claim/XP merge | Landed on `main` @ `8d7c123` | ✅ Landed |
| PR 11 | A3 + A5 — Wave 1 closure evidence | This document | ✅ Active |

Umbrella: [Phase A — Foundation Finalization #62](https://github.com/Maangled/ferros/issues/62)

---

## Reconciliation Gate — Reference

The Wave 1 Reconciliation Gate was a prerequisite for all Wave 1 execution PRs. It is CLOSED and does not need re-opening for Wave 1 closure.

- **Status:** ✅ CLOSED — reviewer sign-off recorded 2026-04-19
- **Tracking issue:** [Reconciliation Gate — Wave 1 #53](https://github.com/Maangled/ferros/issues/53) — closed as completed in [#55](https://github.com/Maangled/ferros/pull/55)
- **Gate document:** [`docs/progress/reconciliation-gate-wave1.md`](./reconciliation-gate-wave1.md)
- **Items verified:** 20 / 20 (B↔A: 6, A↔C: 5, B,C↔D: 5, E↔A–D: 4)
- **Governance:** [`docs/ORCHESTRATION.md` §4](../ORCHESTRATION.md#4-cross-stream-reconciliation-phase)

---

## Wave 1 Declaration

**Wave 1 — Vertical Slice — is CLOSED as of 2026-04-22.**

- All V1–V8 capabilities verified ✅
- All gate harnesses pass: H1 30/30, H2 25/25, H3 18/18, H4 26/26
- All supporting harnesses pass: H5 30/30, H6 25/25, H7 107/107, H8 17/17, H9 28/28, Preflight 6/6
- Phase A Foundation Finalization PRs (PR 8–PR 11) are landed
- Audit findings #9 and #19 have final dispositions recorded above
- Reconciliation Gate [#53](https://github.com/Maangled/ferros/issues/53) remains CLOSED

**Phase B entry begins with PR 12 — [Arena Export Target ADR](./PR-PLAN-PR8-PR13.md#pr-12--arena-export-target-adr-b1)**

---

## Traceability

- **Plan:** [`docs/streams/STREAM-A-CONTRACTS.md`](../streams/STREAM-A-CONTRACTS.md) §Wave 1 Hardening Backlog; [`docs/progress/PR-PLAN-PR8-PR13.md`](./PR-PLAN-PR8-PR13.md) §PR 11
- **Plan PR:** #52 (ORCHESTRATION v2), #65 (stream-first normalization)
- **Exit criteria closed:** V4, A3 (V7 evidence), A5 (audit findings #9, #19)
- **Stream A contract dependency:** none (doc-only PR)
