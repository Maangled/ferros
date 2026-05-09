# Truth-Sync Delta - FRS-core-20260506-C1-W6

- run_id: FRS-core-20260506-C1-W6
- issued_at: 2026-05-09
- stop_condition: No stop condition fired.
- authority_ack: none

## Facts

- Preflight version-lock confirmed: AUTHORITY-MAP.md (2026-05-09), ORCHESTRATION-POLICY.md (2026-05-03), ORCHESTRATION-AGENTS.md (2026-05-09), QUEUE-SURFACES.md (2026-05-03), ORCHESTRATION-EXECUTION.md current. No mismatch detected.
- Lane 1 primary test `local_agent_api_runway_summary_serializes_and_tracks_profile_and_deny_observation` failed on first run due to a stale assertion: `agent_count` expected 2 but actual was 3 after `LocalBridgeStandInAgent` (ha-local-bridge) was added to the reference host.
- Stale assertion fixed: `assert_eq!(missing_summary.agent_count, 2)` → `assert_eq!(missing_summary.agent_count, 3)` in `crates/ferros-node/src/lib.rs`.
- Lane 4 reconciliation revealed three additional stale agent-list assertions in the same file, all missing `ha-local-bridge`:
  - `demo_runs_deterministically_and_denies_unauthorized_work` at line 2918
  - `agent_cli_lists_reference_agents_with_status` at line 3057
  - `reload_boundary_runtime_with_state_rebuilds_reference_runtime_without_replaying_logs` at line 3903
- All four stale assertions corrected; full ferros-node suite (59 lib tests) confirmed passing after corrections.
- All other named lane commands passed without modification.

## Claims

- Named W6 continuity test `local_agent_api_runway_summary_serializes_and_tracks_profile_and_deny_observation` passes: runway summary correctly serializes agent count, profile init observation, deny log tracking, and hub restart observation with the current 3-agent reference host.
- Runtime local_runway module invariants (5 tests) remain stable.
- Runtime boundaries integration surface (16 tests) remains stable.
- Runtime x86_64 subcore smoke (5 tests) remains stable.
- Core capability policy surface (15 tests) remains stable.
- Core message envelope surface (4 tests) remains stable.
- Core foundation surface (4 + 2 + 1 = 7 tests) remains stable.
- Four stale agent-count/list assertions corrected to reflect the current 3-agent reference host roster (echo, ha-local-bridge, timer); no behavioral change introduced.

## Non-claims

- No gate closure claim.
- No hardware proof claim.
- No Home Assistant proof claim.
- No native-runtime or OS-level execution claim.
- No scope claims beyond named validations.
- No claim that ha-local-bridge agent behavioral coverage is complete; only roster-count assertions were corrected.

## Validation Outputs

Lane 1 (continuity):
- cargo test -p ferros-node local_agent_api_runway_summary_serializes_and_tracks_profile_and_deny_observation -> pass after assertion fix (1 passed, 0 failed, 58 filtered out)
- cargo test -p ferros-runtime --lib local_runway -> pass (5 passed, 0 failed, 0 filtered out)

Lane 2 (breadth):
- cargo test -p ferros-runtime --test boundaries -> pass (16 passed, 0 failed, 0 filtered out)
- cargo test -p ferros-runtime -> pass (5 lib + 16 boundaries + 5 x86_64_subcore_smoke = 26 passed, 0 failed)

Lane 3 (breadth):
- cargo test -p ferros-core --test message_envelope -> pass (4 passed, 0 failed, 0 filtered out)
- cargo test -p ferros-core --test capability_policy -> pass (15 passed, 0 failed, 0 filtered out)
- cargo test -p ferros-core -> pass (26 passed, 0 failed)

Lane 4 (reconciliation):
- cargo test -p ferros-node -> pass (59 lib passed, 0 failed; 2 bin passed; 1 integration passed)

## Residual risk notes

- ha-local-bridge agent behavioral coverage is minimal; agent-count assertions are now current but the agent's functional contract has no dedicated test.
- Any future addition of agents to the reference host roster will require updating all hard-coded agent-list assertions across the test suite.
- Authority drift remains a live operational risk; strict preflight required each wave.
- Claim leakage risk remains unless non-claims are explicitly restated in closeout.
