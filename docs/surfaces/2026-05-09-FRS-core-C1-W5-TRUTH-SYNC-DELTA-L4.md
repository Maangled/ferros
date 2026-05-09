# Truth-Sync Delta - FRS-core-20260506-C1-W5

- run_id: FRS-core-20260506-C1-W5
- issued_at: 2026-05-09
- stop_condition: No stop condition fired.
- authority_ack: none

## Facts

- Preflight version-lock remained aligned with canonical authority markers for this packet baseline.
- All named W5 Lane 1-4 validation commands passed with zero failures.
- No source-code mutations were required to satisfy W5 validation objectives.

## Claims

- Alias-retirement deny durability remains enforced.
- Canonical localhost harness continuity remains intact.
- Adjacent shell boundaries remain stable, including grant-mutation rejection behavior.
- Runtime checkpoint and consent-detail invariants remain stable.
- Core capability and envelope invariants remain stable.

## Non-claims

- No gate closure claim.
- No hardware proof claim.
- No Home Assistant proof claim.
- No native-runtime or OS-level execution claim.
- No scope claims beyond named validations.

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
- cargo test -p ferros-core -> pass (26 passed, 0 failed)
- cargo test -p ferros-node shell_route_posts_json_rpc_agent_list -> pass (1 passed, 0 failed, 0 ignored, 58 filtered out)
- cargo test -p ferros-node shell_route_profile_adapter_rejects_grant_mutation_actions -> pass (1 passed, 0 failed, 0 ignored, 58 filtered out)

## Residual risk notes

- Regression risk remains if only named tests run and edge-path variants are not expanded.
- Authority drift remains a live operational risk; strict preflight is required each wave.
- Claim leakage risk remains unless non-claims are explicitly restated in closeout.
