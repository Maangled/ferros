# 2026-05-06 Cycle 4 H9 Truth-Sync Delta (L7)

## 1) Implemented markers/selectors by lane (L1-L7)

- L1: no-op-by-rule (utility markers conditional not triggered).
- L2: added `data-module` / `data-surface-state` harmonization for runway/home-hub/forge/arena/deny containers and state flips.
- L3: no-op-by-rule (optional home-hub inspector consistency selectors not required).
- L4: no-op-by-rule (optional `PostureStatusBadge` granularity not required).
- L5: added arena rehearsal detail selector groups in surface/inspector.
- L6: migrated brittle harness checks to selector-first where stable selectors exist.
- L7: this truth-sync note.

## 2) Disputes raised and resolution

- Dispute: whether this lane should continue to publish bounded skipped placeholders after Cycle 4 rerun evidence was captured.
- Resolution: replace stale skipped placeholders with the actual captured Cycle 4 outcomes, including exact pass/fail/skip/total counts and named failures for both ports.

## 3) Validation evidence summary

### 3.1 Focused Rust test lines required by this note

- `cargo test -p ferros-node shell_route`
  - Result: passed.
  - Totals: 11 passed, 0 failed.
- `cargo test -p ferros-node shell_route_serves_localhost_acceptance_harness`
  - Result: passed.
  - Totals: 1 passed, 0 failed.

### 3.2 Live H9 rerun evidence (Cycle 4 context)

- Port 4324 (`http://127.0.0.1:4324/harnesses/localhost-shell-acceptance.html`)
  - Status: completed.
  - pass/fail/skip/total: 62 / 4 / 2 / 68
  - Named failures:
    1) Five route buttons are visible (found 8 route buttons)
    2) Runway surface exposes checklist rows through the shared operator-step module
    3) Local echo grant is seeded for the allowed lifecycle proof (operator cancelled assisted grant-seeding step)
    4) Lifecycle deny path revokes the active echo grant without a pre-refresh shell reload (operator cancelled assisted revoke step)
- Port 4326 (`http://127.0.0.1:4326/harnesses/localhost-shell-acceptance.html`)
  - Status: completed.
  - pass/fail/skip/total: 66 / 4 / 2 / 72
  - Named failures:
    1) Five route buttons are visible (found 8 route buttons)
    2) Runway surface exposes checklist rows through the shared operator-step module
    3) Local echo grant is seeded for the allowed lifecycle proof (operator cancelled assisted grant-seeding step)
    4) Lifecycle deny path revokes the active echo grant without a pre-refresh shell reload (operator cancelled assisted revoke step)

## 4) Residual risks

- Known failures remain open on both ports and must be resolved in code/harness behavior, especially route-button cardinality and operator-assisted lifecycle grant/revoke flow handling.
- Port-to-port count drift (4324 vs 4326) indicates scenario/config divergence risk that should be normalized or explicitly documented.
- This note records evidence; it does not claim those four failing assertions are fixed.

## 5) Next queue seeds

- Seed A: fix route button cardinality so the expected five route buttons render consistently.
- Seed B: restore runway checklist row exposure through the shared operator-step module.
- Seed C: harden assisted grant-seeding and revoke steps so operator cancellation paths satisfy lifecycle proof assertions.
- Seed D: rerun H9 on 4324 and 4326 after fixes and confirm totals converge with the same named-failure set cleared.

## 6) Explicit non-claim boundaries (preserved)

- This note does not claim hardware proof.
- This note does not claim Home Assistant proof.
- This note does not claim gate closure.
- This note does not claim launch readiness or launch execution.
