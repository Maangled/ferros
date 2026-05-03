# RN-2026-05 ADR-025 Home Hub Local Critical-Path Impact

Status: Draft
Scope: Research-only
Constraint: ADR-025 remains Proposed and non-binding.

## Input Change
- Home Assistant is now optional and onramp-oriented in the near term.
- Same-machine Home Assistant on `homelab001` is allowed for local rehearsal if useful.
- Separate-host Home Assistant validation is deferred.
- The immediate Track A critical path is local `ferros` profile bring-up, local `ferros-hub` bring-up, and LAN-device onboarding planning on `homelab001`.

## Impact On Guardrail Checks

### Check 3. Fastest/FERROS Separation
Assessment:
- Strengthened.

Why:
- The new policy makes the separation more explicit: local Fastest-root rehearsal may use co-located Home Assistant when helpful, while FERROS-root claims remain deferred until later evidence exists.

Implication:
- Local homelab bring-up can proceed without implying that Home Assistant is the core product or that ADR-025 is accepted.

### Check 5. Server-Control-Plane
Assessment:
- Narrowed and clarified.

Why:
- The immediate control-plane target is now `homelab001` local evidence routing for profile, hub, and future LAN-device work rather than early multi-machine Home Assistant coordination.

Implication:
- The x86_64 control-plane assumption still looks plausible, but the next evidence should focus on local command and artifact routing before remote or multi-board claims.

### Check 6. Claim-Boundary
Assessment:
- Strengthened.

Why:
- The new policy explicitly says co-located Home Assistant is not separate-host proof, not G4 proof, and not a blocker for profile or hub work.

Implication:
- The claim ceiling is easier to enforce because the repo now has explicit near-term non-claims for Home Assistant.

### Check 1. Lane Sufficiency
Assessment:
- Slightly adjusted.

Why:
- The current Phase 0 critical path favors a compact local bring-up lane set before broader separate-host or multi-machine work.

Implication:
- ADR-025 should keep room for phased lane compression in early x86_64 local bring-up without forcing Home Assistant-first sequencing.

### Check 2. S9 Necessity
Assessment:
- Still unresolved.

Why:
- The immediate local bring-up packet does not yet prove whether S9 needs to be a distinct ignition lane or can stay provisional.

Implication:
- Keep S9 provisional until a real local bring-up and truth-sync packet show a concrete need.

## Recommendation
- Keep ADR-025 as Proposed.
- Do not promote it from this policy change alone.
- Treat the new local critical path as research evidence about sequencing and claim boundaries, not as a binding architectural decision.

## Missing Evidence Before Any Promotion Discussion
- One real homelab001 local bring-up findings packet covering profile init or show and hub readout.
- One honest local device-observation packet if LAN-device onboarding is attempted.
- One later comparison of co-located versus separate-host Home Assistant value after the local bring-up path is better understood.

## Research Disclaimer
This note does not promote ADR-025, close any gate, or claim local or physical evidence.

## HANDOFF CARD
- Lane ID: B1
- Status: complete
- Files read: docs/adr/ADR-025-dual-root-hardware-runway.md; docs/adr/ADR-022-decision-program-governance.md; docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-guardrail-scoreboard.md; docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-guardrails-batch1.md; docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-guardrails-batch2.md; docs/orchestration/REENTRY-PHASE0-HARDWARE-READINESS-AUDIT.md
- Files changed: docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-homehub-local-critical-path.md
- Evidence produced: research note on ADR-025 impact from Home Assistant de-emphasis and homelab local priority
- Claims added: research framing for the new local critical path
- Claims explicitly not added: ADR promotion, gate movement, Home Assistant proof, hardware evidence
- Validation: research-note consistency review against current ADR-025 and phase-0 policy docs
- Residual risks: real local bring-up findings are still missing, so this note remains sequencing guidance only
- Next safe follow-up, if any: feed homelab001 local bring-up findings back into the ADR-025 scoreboard before any disposition change