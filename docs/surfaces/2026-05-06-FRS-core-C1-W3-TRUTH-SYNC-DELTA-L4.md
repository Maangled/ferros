# 2026-05-06 FRS-core-C1-W3 Truth-Sync Delta (L4)

Authority: docs/orchestration/AUTHORITY-MAP.md
Date: 2026-05-06
Run ID: FRS-core-20260506-C1-W3
Run profile: core-runtime
Track: code

## Preflight and continuity

- Active-session preflight remained non-blocking and authority anchors stayed version-locked at `Last updated: 2026-05-03`.
- Mission continuity rule preserved: C1 advanced from W2 to W3 with no new mission boundary declared.

## Recursion-cycle lane outcomes

- Continuity lane (retirement guardrails): completed with retirement execution.
  - Executed retirement path for the temporary alias route `/harnesses/localhost-shell-acceptance-harness.html`.
  - Canonical path remains `/harnesses/localhost-shell-acceptance.html`.
  - Added explicit enforcement check in tests so alias retirement cannot silently revert.

- Adjacent seam lane (node-runtime metadata normalization): no-op by rationale in this bounded cycle.
  - No active header drift was evidenced after retirement; deferred until a concrete consumer mismatch appears.

- Breadth lane (runtime/core stress): no-op by rationale in this bounded cycle.
  - Kept scope bounded to retirement closure plus regression verification.

- Breadth verification lane: completed.
  - Verified retired alias path returns 404.
  - Verified canonical dual-port unattended matrix stayed stable on both ports.

- Truth-sync lane: completed.
  - This delta was written and run log was prepended newest-first.

## Retirement readiness criteria and outcome

Readiness criteria:
1. Focused route tests remain green after alias removal.
2. Canonical harness route remains live and green on dual-port unattended runs.
3. Retired alias path returns deterministic `404 Not Found`.
4. Enforcement test exists to prevent silent permanence or rollback.

Outcome: all readiness criteria met; retirement executed in this cycle.

## Changes landed

- `crates/ferros-node/src/lib.rs`
  - Removed alias serving for `/harnesses/localhost-shell-acceptance-harness.html`.
  - Replaced prior dual-path parity test with explicit retirement guardrail test:
    - `shell_route_rejects_retired_harness_alias_path`.

## Validation evidence

- Focused required checks:
  - `cargo test -p ferros-node shell_route` -> pass (12 passed, 0 failed)
  - `cargo test -p ferros-node shell_route_serves_localhost_acceptance_harness` -> pass (1 passed, 0 failed)
- Live retirement verification:
  - `http://127.0.0.1:4324/harnesses/localhost-shell-acceptance-harness.html?unattended=1` -> 404 (`FERROS local shell route not found`)
- Canonical dual-port unattended matrix:
  - `http://127.0.0.1:4324/harnesses/localhost-shell-acceptance.html?unattended=1` -> 74 passed, 0 failed, 10 skipped, 84 total
  - `http://127.0.0.1:4326/harnesses/localhost-shell-acceptance.html?unattended=1` -> 74 passed, 0 failed, 10 skipped, 84 total
- Unexpected regressions: none.

## Claims added

- Compatibility bridge retirement was executed with explicit enforcement checks.
- Canonical harness route remained stable after retirement across focused and live evidence.

## Claims explicitly not added

- No gate closure claim.
- No hardware proof claim.
- No Home Assistant proof claim.
- No widened canonical mutation claim beyond evidenced local shell-route retirement and canonical-route stability.

## Residual risks

- Operator-assisted lifecycle branches remain intentionally unclaimed in unattended evidence and still require separate interactive evidence for those paths.
- Historical docs and run logs still reference the retired alias as past evidence context; those references are historical, not current serving policy.