# 2026-05-06 Cycle 4 H9 Wave MC4 Truth-Sync Delta (L1)

## 1) Scope and authority outcome

- Run ID: CYCLE4-H9-WAVE-MC4
- Lane: L1 only
- Mode: Interactive Mode, single focused lane
- Anchor surfaces touched:
  - site/agent-center-shell.html
  - harnesses/localhost-shell-acceptance-harness.html

Preflight authority marker lock check passed:
- AUTHORITY-MAP.md -> Last updated: 2026-05-03
- ORCHESTRATION-POLICY.md -> Last updated: 2026-05-03
- ORCHESTRATION-EXECUTION.md -> Last updated: 2026-05-03
- ORCHESTRATION-AGENTS.md -> Last updated: 2026-05-03
- QUEUE-SURFACES.md -> Last updated: 2026-05-03

## 2) Targeted fixes applied in anchor scope

1. Route cardinality fix:
   - Shell route descriptors reduced to five active route buttons: agents, grants, profile, runway, denyLog.
   - Harness route-count assertion updated to "Five route buttons are visible".
2. Runway checklist exposure boundary fix:
   - Runway surface no longer renders legacy RunwayChecklistRowCard/data-runway-index selectors.
   - Harness moved to selector-first boundary assertions for checklist contract checks.
3. Operator-assisted lifecycle split:
   - Assisted grant-seeding and revoke checks are now interactive-only.
   - Unattended mode skips those steps without failing the run.
4. Dual-port stability prep:
   - Harness includes unattended query-mode support (unattended=1 | mode=unattended | interactive=0).

## 3) Required validation execution and hard-stop outcome

Focused required commands executed:
- cargo test -p ferros-node shell_route -> FAILED
- cargo test -p ferros-node shell_route_serves_localhost_acceptance_harness -> FAILED

Hard stop condition fired:
- Condition 2 (Cargo test failure).

Earliest concrete failures:
- crates/ferros-node/src/lib.rs:4666
  - assertion failed: html.contains("id: 'homeHub'")
- crates/ferros-node/src/lib.rs:5087
  - assertion failed: html.contains("Runway checklist rows expose RunwayChecklistRowCard markers with stable data-runway-index mapping")

Log triage routing performed:
- FERROS Log Triage Agent classification: validation/harness drift (unambiguous).
- Trace analyst escalation: not required (boundary is clear).
- Owning remediation surface: S5 UX (HTML/harness contract drift) with S4 ferros-node test alignment.

## 4) Port evidence status (pre-fix baseline vs post-fix)

Pre-fix baseline from prior truth-sync (L7):
- Port 4324: 62 / 4 / 2 / 68
- Port 4326: 66 / 4 / 2 / 72

Post-fix live dual-port reruns:
- Not executed due hard stop after cargo failures.
- No post-fix pass/fail/skip/total counts can be truthfully claimed for 4324 or 4326 in this run.

## 5) Unexpected failure posture and settlement state

- Zero-unexpected-failure confirmation: Not claimable in MC4 due hard stop before live reruns.
- Settlement status: stop-escalate.
- Residual H9 status at stop:
  - Prior four named H9 failures were targeted by code/harness changes, but rerun evidence is blocked pending test-contract reconciliation.

## 6) Next attack items

1. Reconcile ferros-node shell-route assertions with the new five-route and selector-first checklist boundary contract, or preserve expected legacy strings as explicit compatibility shims.
2. Re-run:
   - cargo test -p ferros-node shell_route
   - cargo test -p ferros-node shell_route_serves_localhost_acceptance_harness
3. Only after cargo passes, execute live H9 harness reruns on:
   - http://127.0.0.1:4324/harnesses/localhost-shell-acceptance-harness.html
   - http://127.0.0.1:4326/harnesses/localhost-shell-acceptance-harness.html
4. Record exact post-fix per-port pass/fail/skip/total and unexpected-failure count for closure decision.

## 7) Explicit non-claims preserved

- No gate closure claim.
- No hardware proof claim.
- No Home Assistant proof claim.
- No schema or runtime seam contract change claimed beyond anchor-surface UI/harness behavior edits.
