# Surfaces L1-L6 UX Delta Round 2 MC2

Date: 2026-05-05

## Scope

This delta captures additive marker work across the local shell surfaces with no runtime behavior change.

Implemented markers:
- L1: `MetricCard` markers on center metrics.
- L2: `GrantsSurfaceCard` and `ProfileSurfaceCard` container markers plus `data-surface-state` defaults.
- L3: `HomeHubBridgeProposalGroup` wrapper marker.
- L4: `ForgeAuthoringStripCard` and `ExportReadinessCard` markers.
- L5: `ArenaLifecycleRehearsalStageCard` marker on Arena only.

Verification surfaces touched:
- `harnesses/localhost-shell-acceptance-harness.html`
- `crates/ferros-node/src/lib.rs` (`shell_route_serves_local_shell_html`)

## Claim Ceiling

- Additive HTML markers only.
- No route id changes.
- No new backend API behavior.
- No canonical, evidentiary, or transport claim expansion.

## Disputes And Rulings

1. L5 ownership dispute
Decision: Lifecycle rehearsal marker is Arena-scoped only. Home-Hub remains observation-focused.

2. Selector strategy dispute
Decision: H9 keeps legacy text checks while adding deterministic marker checks during transition.

3. Shared seam collision risk
Decision: Wrapper marker additions are accepted with serial settlement in this order: L2, L3, L4, L5.

## Unresolved Seams

- Full retirement of brittle text-content harness checks is deferred.
- Additional selector harmonization for all surfaces is deferred.

## Next Queue Seeds

- L1: Add marker coverage for additional utility card classes if introduced.
- L2: Expand `data-surface-state` to more route surfaces only when needed.
- L3: Add optional bridge/proposal consistency selector checks in inspector.
- L4: Evaluate optional `PostureStatusBadge` marker split if harness needs finer granularity.
- L5: Consider richer Arena rehearsal detail rows while preserving non-evidentiary ceiling.
- L6: Migrate remaining harness text checks to selector checks.
- L7: Publish round summary with proof logs and residual risk notes.
