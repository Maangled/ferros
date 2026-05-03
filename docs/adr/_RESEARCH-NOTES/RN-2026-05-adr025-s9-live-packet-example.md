# RN-2026-05 ADR-025 S9 Live Packet Example

Status: Draft
Scope: Research-only
Constraint: ADR-025 remains Proposed and non-binding.

## Question
- What does one real `S9` packet look like when triggered by an already-completed finding?
- What does `S9` add beyond the `S8` truth-sync surfaces that already captured the finding?

## Evidence Reviewed
- `docs/adr/ADR-025-dual-root-hardware-runway.md`
- `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-s9-provisional-service-packet.md`
- `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-x86-overlay-lane-map.md`
- `docs/hardware/findings/FINDINGS-homelab001-local-bringup.md`
- `docs/orchestration/WAVE-RUN-LOG.md`
- `docs/orchestration/doc-batches/DOC-BATCH-2026-05-03-HOMEHUB-LOCAL-FINDINGS-01.md`

## Triggering Finding
Use the `ha-local-bridge` visibility mismatch from the completed `homelab001` local bring-up findings packet.

Observed facts already recorded by `S8`:
- `ferros-hub summary` reports `bridgeAgent: ha-local-bridge@0.1.0`
- `ferros agent list` shows only `echo, timer`
- `ferros agent describe ha-local-bridge` returns `unknown agent`

This is a real completed finding because the outputs, artifacts, exit codes, and residual risks are already preserved in the findings packet and its doc-batch or run-log surfaces.

## What S8 Already Did
`S8` truth-sync already:
- recorded the exact commands and outputs in `docs/hardware/findings/FINDINGS-homelab001-local-bringup.md`
- preserved the mismatch as a failure rather than rewriting it away
- recorded the claim ceiling and non-claims in the findings packet, doc-batch, and run-log surfaces
- named the next direct investigative segment: `REENTRY-HOMEHUB-LOCAL-AGENT-VISIBILITY-01`

That is correct `S8` behavior. It says what became true.

## What S9 Adds
`S9` does not restate the finding. It routes the implication.

This packet adds:
- a cross-root classification: the mismatch belongs to `x86_64/Fastest` first, not to the abstract FERROS root alone
- a reload proposal: the current x86_64 Fastest model should treat bridge-agent visibility as an explicit success criterion for the local agent-center and runtime seams
- a reroute proposal: the finding should feed both an immediate code-track investigation and a narrower FERROS-side process-model question if the visibility split proves intentional

## Reload Or Reroute Targets

### Primary reload target
- `x86_64/Fastest/F3` — local agent-center visibility and registry behavior

Reason:
- the current CLI and agent-center surfaces are where the mismatch is directly visible to the operator

### Secondary reload target
- `x86_64/Fastest/F4` — runtime or lifecycle assumptions behind hub-registered versus CLI-visible agents

Reason:
- if the hub summary and CLI path are reading different lifecycles or registries, the runtime boundary needs clarification

### Optional future research reroute
- `x86_64/FERROS/R3` — process and lifecycle model research

Reason:
- if the split is intentional, the FERROS-side process model should eventually explain why bridge-facing agents and general local agents do not share the same visibility rules

## Live S9 Packet

```text
S9 packet id: S9-LIVE-2026-05-03-01
trigger: bridge-agent visibility mismatch from homelab001 local bring-up
source finding: docs/hardware/findings/FINDINGS-homelab001-local-bringup.md
what S8 already answered: the mismatch is real, locally captured, and still unresolved
what S9 adds: reload x86_64/Fastest F3 and F4 with bridge-agent visibility as an explicit success check; keep optional FERROS/R3 follow-up if the split is architectural rather than accidental
immediate handoff: REENTRY-HOMEHUB-LOCAL-AGENT-VISIBILITY-01
status: proposal-only
```

## Why This Is Non-Redundant
Without `S9`, the repo has an honest finding and an honest next investigation, but it does not yet have a lane-aware routing statement that says which overlay surfaces should reload because of that finding.

This packet is therefore non-redundant because it:
- ties the finding to the overlay lane map
- names the first reload targets
- preserves a narrower immediate follow-up rather than expanding into a vague governance loop

## Non-Claims
- no claim that the mismatch is understood yet
- no claim that Home Assistant integration is proven
- no claim that the bridge is correctly registered everywhere
- no claim that D1 or G4 moved
- no claim that ADR-025 is Accepted
- no claim that `S9` is now a permanent always-running service lane

## Recommendation
- Resolve guardrail check 2 as `adjust`: `S9` is useful and non-redundant when it emits a concrete lane-aware reload or reroute packet tied to a completed finding.
- Keep `S9` provisional until at least one live packet example and the lane-packet enforcement rules both exist.
- Use this packet as the minimum proof that `S9` is more than a renamed `S8` bookkeeping surface.

## ADR Text Impact
- Clarify that `S9` becomes justified only when it emits a real reload or reroute packet tied to a named finding already captured by `S8`.
- Clarify that `S9` outputs are lane-aware handoffs, not duplicate findings packets.
- Clarify that `S9` remains proposal-only until the enforcement model is documented.

## D1/G4 Claim Impact
- No D1 movement.
- No G4 movement.
- No Home Assistant proof claim.
- No physical-device evidence claim.

## Research Disclaimer
This note is a live routing example only. It does not execute the reload, does not replace code-track investigation, and does not convert the finding into a broader hardware or launch claim.

## HANDOFF CARD
- Lane ID: O6
- Status: complete
- Files read: `docs/adr/ADR-025-dual-root-hardware-runway.md`; `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-s9-provisional-service-packet.md`; `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-x86-overlay-lane-map.md`; `docs/hardware/findings/FINDINGS-homelab001-local-bringup.md`; `docs/orchestration/WAVE-RUN-LOG.md`; `docs/orchestration/doc-batches/DOC-BATCH-2026-05-03-HOMEHUB-LOCAL-FINDINGS-01.md`
- Files changed: `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-s9-live-packet-example.md`
- Evidence produced: live S9 packet example tied to a completed local finding
- Claims added: one concrete non-redundant S9 routing example now exists in repo
- Claims explicitly not added: ADR promotion, S9 permanence, Home Assistant proof, D1 closure, G4 closure
- Validation: source-reference consistency review against the completed findings packet and overlay notes
- Residual risks: lane-packet enforcement still needs to be documented before S9 can move beyond proposal-only status
- Next safe follow-up, if any: patch lane-packet enforcement and then record ADR-025 disposition honestly