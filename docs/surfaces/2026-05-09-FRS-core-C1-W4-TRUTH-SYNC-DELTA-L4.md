# Truth-Sync Delta - FRS-core-20260506-C1-W4

- run_id: FRS-core-20260506-C1-W4
- issued_at: 2026-05-09
- stop_condition: No stop condition fired.
- authority_ack: none

## Claims

- Retired alias `/harnesses/localhost-shell-acceptance-harness.html` remains enforced as a negative contract (404) and is covered by durable test evidence.
- Canonical `/harnesses/localhost-shell-acceptance.html` path remains served.
- Shell boundaries for local runway summary and profile adapter routes remain stable under focused boundary tests.
- Profile adapter continues to reject grant-mutation actions.
- Runtime runway checkpoint progression and consent-detail invariants remain intact.
- Core capability and message-envelope invariants remain intact.
- All packet-named validation commands passed with zero failures.

## Non-claims

- No gate closure claim.
- No hardware proof claim.
- No Home Assistant proof claim.
- No native-runtime or OS-level execution claim.
- No claims beyond explicitly executed tests.

## Validation Outputs

Lane 1:
- cargo test -p ferros-node shell_route_rejects_retired_harness_alias_path -> pass (1 passed, 0 failed, 0 ignored, 58 filtered out)
- cargo test -p ferros-node shell_route_serves_localhost_acceptance_harness -> pass (1 passed, 0 failed, 0 ignored, 58 filtered out)
- cargo test -p ferros-node shell_route_returns_not_found_for_unknown_paths -> pass (1 passed, 0 failed, 0 ignored, 58 filtered out)

Lane 2:
- cargo test -p ferros-node shell_route_gets_local_runway_summary_json -> pass (3 passed, 0 failed, 0 ignored, 56 filtered out)
- cargo test -p ferros-node shell_route_posts_profile_init_and_show_through_local_adapter -> pass (1 passed, 0 failed, 0 ignored, 58 filtered out)
- cargo test -p ferros-node shell_route_profile_adapter_rejects_grant_mutation_actions -> pass (1 passed, 0 failed, 0 ignored, 58 filtered out)

Lane 3:
- cargo test -p ferros-runtime start_path_advances_through_the_local_runway_checkpoints -> pass (1 passed, 0 failed, 0 ignored, 4 filtered out)
- cargo test -p ferros-runtime shell_detail_matches_terminal_and_consent_boundaries -> pass (1 passed, 0 failed, 0 ignored, 4 filtered out)
- cargo test -p ferros-runtime adapter_composes_transition_executor_and_bus_through_runtime_seams -> pass (1 passed, 0 failed, 0 ignored, 4 filtered out)

Lane 4:
- cargo test -p ferros-core -> pass (26 passed across ferros-core tests; 0 failed)
- cargo test -p ferros-node shell_route_posts_json_rpc_agent_list -> pass (1 passed, 0 failed, 0 ignored, 58 filtered out)
- cargo test -p ferros-node shell_route_profile_adapter_rejects_grant_mutation_actions -> pass (1 passed, 0 failed, 0 ignored, 58 filtered out)

## Next Seed Candidates

- Continuity: Extend retired-alias negative-contract coverage to include query-string and trailing-slash variants without widening route surface.
- Breadth: Add a runtime-focused assertion that local runway summary JSON remains schema-stable under checkpoint progression changes.
- Breadth: Add a cross-crate boundary test proving node shell allow/deny behavior remains aligned with ferros-core capability request validation invariants.
