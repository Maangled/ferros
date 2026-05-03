# RN-2026-05 ADR-025 Guardrail Scoreboard

Status: Ready for ADR
Scope: Research-only
Authority: ADR-022
Constraint: ADR-025 acceptance is framework-level only; family-level proof remains separate.

## Disposition Summary
- Date resolved: 2026-05-03
- ADR-025 disposition: Accepted as a framework-level architecture and governance record
- S9 disposition: accepted as a provisional service lane inside the framework, not as a permanent always-running service
- Remaining non-claims: no D1 movement, no G4 movement, no Home Assistant proof, no physical-device evidence, no FERROS-native OS proof

## Check 1. Lane Sufficiency
Question:
- Are S1-S8 sufficient to describe per-hardware work without hidden gaps?

Evidence reviewed:
- docs/adr/ADR-025-dual-root-hardware-runway.md
- docs/orchestration/HARDWARE-QUEUE.md
- streams/S1-foundation/README.md
- streams/S7-hub/PROGRESS.md

Decision pressure:
- Need clear lane ownership before accepting dual-root policy.

Risks:
- Lane overlap can blur gate claims.

Recommendation:
- Adjust.

ADR text impact:
- Add family-specific lane profiles with required, optional, federated, and deferred lane states.

D1/G4 claim impact:
- No direct gate movement; structure-only.

Unresolved evidence:
- No framework-level blocker remains. Family-level operational proof is still required before any board-level claim upgrade.

## Check 2. S9 Necessity
Question:
- Is S9 needed as a separate ignition lane versus S8 governance extension?

Evidence reviewed:
- docs/adr/ADR-025-dual-root-hardware-runway.md
- docs/adr/ADR-022-decision-program-governance.md
- docs/orchestration/LOCAL-DRIVER.md

Decision pressure:
- Must avoid adding a lane with ambiguous authority.

Risks:
- Duplicate governance loops and unclear stop conditions.

Recommendation:
- Adjust.

ADR text impact:
- Keep `S9` provisional and require lane-aware reload or reroute outputs tied to completed findings.

D1/G4 claim impact:
- No direct gate movement.

Unresolved evidence:
- No framework-level blocker remains. Additional operational repetition is future implementation proof, not an ADR blocker.

## Check 3. Fastest/FERROS Separation
Question:
- Does dual-root separation reduce overclaim risk while preserving delivery velocity?

Evidence reviewed:
- docs/adr/ADR-025-dual-root-hardware-runway.md
- docs/hardware/d1-target-inventory.md
- docs/orchestration/HARDWARE-QUEUE.md

Decision pressure:
- Must keep Track A evidence and Track B architecture disposition decoupled.

Risks:
- Premature binding of FERROS-root authority.

Recommendation:
- Accept with enforcement language.

ADR text impact:
- Strengthen non-claim language so Fastest findings do not imply FERROS-root proof or gate movement.

D1/G4 claim impact:
- No direct gate movement.

Unresolved evidence:
- No framework-level blocker remains. Family-level evidence and handoff practice still require later operational proof.

## Check 4. Embedded-Device Compression
Question:
- Should constrained targets like ESP32 collapse lane count or lane content?

Evidence reviewed:
- docs/adr/ADR-025-dual-root-hardware-runway.md
- docs/research/S4-no-std-target-matrix.md
- docs/research/S1-boot-sequence-d1-target.md

Decision pressure:
- Lane model must remain practical on constrained targets.

Risks:
- Over-scaffolding and stalled execution.

Recommendation:
- Adjust.

ADR text impact:
- Add compressed-lane rules for constrained targets and treat ESP32 as a compressed peripheral family.

D1/G4 claim impact:
- No direct gate movement.

Unresolved evidence:
- No framework-level blocker remains. Per-family implementation proof remains future work.

## Check 5. Server-Control-Plane
Question:
- Can x86_64 Fastest safely serve as control-plane for Pi/Jetson/ESP32 findings?

Evidence reviewed:
- docs/adr/ADR-025-dual-root-hardware-runway.md
- docs/hardware/d1-target-inventory.md
- docs/research/S7-d1-bring-up-checklist.md

Decision pressure:
- Multi-board coordination depends on this assumption.

Risks:
- Control-plane overreach can misstate hardware evidence.

Recommendation:
- Accept with caveats.

ADR text impact:
- Permit `x86_64/Fastest` control-plane summaries only with named source attribution and copied claim ceilings.

D1/G4 claim impact:
- No direct gate movement.

Unresolved evidence:
- No framework-level blocker remains. Multi-board operational routing proof remains future implementation evidence.

## Check 6. Claim-Boundary
Question:
- Does the model prevent accidental claims beyond current evidence?

Evidence reviewed:
- docs/adr/ADR-025-dual-root-hardware-runway.md
- STATUS.md
- docs/orchestration/HARDWARE-QUEUE.md
- docs/gates/D1.md

Decision pressure:
- Claim safety is mandatory before any status promotion.

Risks:
- Language drift can imply D1/G4 closure without evidence.

Recommendation:
- Adjust.

ADR text impact:
- Add findings, doc-batches, and run-log claim-boundary checklists so aggregation cannot silently widen claims.

D1/G4 claim impact:
- Must remain no movement until evidence exists.

Unresolved evidence:
- Wording discipline remains manual until tooling exists, but that is not a blocker to framework acceptance.

## Check 7. Agent-Permission
Question:
- Is read-wide/write-narrow operationally enforceable across roots and lanes?

Evidence reviewed:
- docs/adr/ADR-025-dual-root-hardware-runway.md
- docs/orchestration/LOCAL-DRIVER.md
- docs/adr/ADR-022-decision-program-governance.md

Decision pressure:
- Permission model is core to safe parallel execution.

Risks:
- Unenforced write boundaries can create conflicting truth surfaces.

Recommendation:
- Adjust.

ADR text impact:
- Add governance-level read-wide/write-narrow and serial truth-sync rules for lane packets.

D1/G4 claim impact:
- No direct gate movement.

Unresolved evidence:
- Policy definition now exists. Tooling or CI enforcement remains future hardening work.

## Research Disclaimer
This scoreboard remains a research summary and does not itself move any gate or create hardware evidence. It records the completed guardrail dispositions that supported ADR-025 acceptance.

## HANDOFF CARD
- Lane ID: B1
- Status: complete
- Files read: docs/adr/ADR-025-dual-root-hardware-runway.md; docs/adr/ADR-022-decision-program-governance.md; docs/adr/_INDEX.md; docs/adr/_ROADMAP.md
- Files changed: docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-guardrail-scoreboard.md
- Evidence produced: seven-check scoreboard scaffold with required slots
- Claims added: research scaffold exists
- Claims explicitly not added: ADR-025 promotion, gate movement, binding architecture
- Validation: template completeness review for checks 1-7
- Residual risks: recommendations remain pending until batch notes are completed
- Next safe follow-up, if any: use the accepted framework to guide real hardware-family execution without widening the existing non-claims
