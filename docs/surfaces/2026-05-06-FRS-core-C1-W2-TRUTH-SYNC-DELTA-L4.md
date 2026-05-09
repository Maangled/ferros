# 2026-05-06 FRS-core-C1-W2 Truth-Sync Delta (L4)

Authority: docs/orchestration/AUTHORITY-MAP.md
Date: 2026-05-06
Run ID: FRS-core-20260506-C1-W2
Run profile: core-runtime
Track: code

## Preflight and route token

- Route token validated for core execution:
  - token_version: v1
  - target_stream: core
  - run_id: FRS-core-20260506-C1-W2
- Non-blocking preflight continuity applied (active stream session in C1).
- Authority lock markers remained aligned at `Last updated: 2026-05-03` for:
  - AUTHORITY-MAP
  - ORCHESTRATION-POLICY
  - ORCHESTRATION-EXECUTION
  - ORCHESTRATION-AGENTS
  - QUEUE-SURFACES

## Recursion-cycle lane outcomes

- Continuity lane: completed.
  - Landed one focused dual-path continuity guard test in ferros-node to lock parity between the canonical and alias harness paths.

- Adjacent seam lane (metadata/header normalization): no-op by rationale.
  - No serving-header divergence was observed in this bounded cycle; lane deferred to a follow-on only if real consumer drift appears.

- Breadth lane (runtime/core stress): no-op by rationale.
  - Kept scope bounded to shell-route continuity and verification in this recursion cycle to avoid widening beyond the declared objective.

- Adjacent seam lane (profile bootstrap fallback wording): no-op by rationale.
  - Existing profile fallback wording remains aligned with the selector-first and compatibility-boundary posture validated in live evidence.

- Breadth verification lane: completed.
  - Reran unattended dual-port regression matrix and confirmed invariant outcomes.

- Truth-sync lane: completed.
  - This delta was written and run-log append was performed newest-first.

## Changes landed

- `crates/ferros-node/src/lib.rs`
  - Added `shell_route_serves_both_harness_paths_with_identical_payload` test to assert both harness paths return identical payload and content type.

## Validation evidence

- Focused route checks:
  - `cargo test -p ferros-node shell_route` -> pass (12 passed, 0 failed)
  - `cargo test -p ferros-node shell_route_serves_localhost_acceptance_harness` -> pass (1 passed, 0 failed)
- Dual-port unattended live evidence:
  - Port 4324: 74 passed, 0 failed, 10 skipped, 84 total
  - Port 4326: 74 passed, 0 failed, 10 skipped, 84 total
- Unexpected dual-port regressions: none.

## Stop-condition evaluation

- 1 (validation failure): not triggered.
- 2 (hard-stop authority/tag): not triggered.
- 3 (scope overrun): not triggered.
- 4 (policy/chain break): not triggered.
- 5 (segment boundary): not triggered.
- 6 (malformed output): not triggered.

Gatekeeper decision: stop-clean.

## Claims added

- Dual-path harness serving now has an explicit continuity contract test in ferros-node.
- Bounded verification reconfirmed stable dual-port unattended behavior after the continuity hardening change.

## Claims explicitly not added

- No gate closure claim.
- No hardware proof claim.
- No Home Assistant proof claim.
- No broadened canonical mutation claim outside evidenced local shell/harness compatibility behavior.

## Residual risks

- Dual-path bridge can still drift into permanence unless retirement criteria are executed in a bounded follow-on recursion cycle.
- Operator-assisted lifecycle branches remain intentionally unclaimed in unattended proof and require separate interactive evidence if needed.