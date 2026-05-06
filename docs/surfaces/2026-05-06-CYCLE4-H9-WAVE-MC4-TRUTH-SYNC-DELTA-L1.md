# 2026-05-06 Cycle 4 H9 Wave MC4 Truth-Sync Delta (L1)

## 1) Scope and authority outcome

- Run ID: CYCLE4-H9-WAVE-MC4
- Lane: L1 only
- Mode: resumed Core stream session with one permitted direct owning-dependency hop
- Anchor surfaces touched:
  - site/agent-center-shell.html
  - harnesses/localhost-shell-acceptance-harness.html
   - crates/ferros-node/src/lib.rs

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
5. Legacy ferros-node compatibility shim:
   - The shell host now serves the same H9 harness content at both `/harnesses/localhost-shell-acceptance.html` and `/harnesses/localhost-shell-acceptance-harness.html`.
   - This one-hop ferros-node alias was required because the requested live evidence URL was a real 404.
6. Live harness timing repair:
   - Profile adapter checks now wait for the completed structured `/profile` result instead of treating the intermediate busy state as settled.
   - This removed a live-only race that previously surfaced five false Profile Adapter failures on both ports.

## 3) Required validation execution and resumed outcome

Focused required commands executed:
- cargo test -p ferros-node shell_route -> passed (11 passed, 0 failed)
- cargo test -p ferros-node shell_route_serves_localhost_acceptance_harness -> passed (1 passed, 0 failed)
- cargo test -p ferros-node shell_route_serves_localhost_acceptance_harness after harness timing repair -> passed (1 passed, 0 failed)

Observed hard-stop history retained for trace truth:
- Initial MC4 attempt hit Condition 2 after legacy ferros-node string assertions failed.
- Resumed lane cleared that stop by preserving the new UX contract and adding a bounded compatibility shim.

Earliest concrete stop-causes from the initial halted attempt:
- crates/ferros-node/src/lib.rs legacy `html.contains(...)` assertions for removed route ids and legacy runway checklist strings.

Log triage routing performed:
- FERROS Log Triage Agent classification: validation/harness drift (unambiguous).
- Trace analyst escalation: not required (boundary is clear).
- Owning remediation surface: S5 UX (HTML/harness contract drift) with one direct S4 ferros-node route alias for requested live evidence compatibility.

## 4) Port evidence status (pre-fix baseline vs post-fix)

Pre-fix baseline from prior truth-sync (L7):
- Port 4324: 62 / 4 / 2 / 68
- Port 4326: 66 / 4 / 2 / 72

Post-fix live dual-port reruns:
- Port 4324 (`http://127.0.0.1:4324/harnesses/localhost-shell-acceptance-harness.html?unattended=1`): 74 / 0 / 10 / 84
- Port 4326 (`http://127.0.0.1:4326/harnesses/localhost-shell-acceptance-harness.html?unattended=1`): 74 / 0 / 10 / 84
- Unexpected live failures after the final rerun: 0 on both ports.
- Remaining skips are expected and rule-backed:
  - legacy eight-route assertion
  - legacy runway checklist-row assertions
  - Home-Hub / Forge / Arena route activation checks
  - operator-assisted grant-seeding and revoke checks in unattended mode
  - optional hub restart fallback checks when restart context is present

## 5) Unexpected failure posture and settlement state

- Zero-unexpected-failure confirmation: claimable.
- Settlement status: ready for settlement on the MC4 slice.
- Residual H9 status:
  - The targeted four H9 failure classes are cleared in the final cargo and live-harness evidence captured here.
  - The live harness now converges across both requested ports with identical clean counts.

## 6) Next attack items

1. Decide whether the legacy harness-path alias in ferros-node should remain permanent or be retired after downstream callers move fully to one canonical H9 route.
2. If interactive operator evidence is needed beyond unattended proof, rerun H9 with manual grant and revoke steps to exercise the allowed lifecycle and deny-observation branches.
3. Remove any temporary `.tmp/h9-*` profile artifacts created during live validation if a follow-on lane requires a fully cleaned local workspace.

## 7) Explicit non-claims preserved

- No gate closure claim.
- No hardware proof claim.
- No Home Assistant proof claim.
- No schema or runtime seam contract change claimed beyond anchor-surface UI/harness behavior edits plus the minimal ferros-node harness-path alias needed to serve the requested live evidence URL.
