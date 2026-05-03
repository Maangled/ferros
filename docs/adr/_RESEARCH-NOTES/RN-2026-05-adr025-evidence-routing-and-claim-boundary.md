# RN-2026-05 ADR-025 Evidence Routing And Claim Boundary

Status: Draft
Scope: Research-only
Constraint: ADR-025 remains Proposed and non-binding.

## Question
- Can `x86_64/Fastest` safely serve as the control-plane witness for multi-board findings without blurring source attribution?
- What claim-boundary checks must every findings, doc-batch, and run-log surface carry before an aggregated statement is honest?

## Evidence Reviewed
- `docs/adr/ADR-025-dual-root-hardware-runway.md`
- `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-guardrail-scoreboard.md`
- `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-guardrails-batch1.md`
- `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-guardrails-batch2.md`
- `docs/hardware/findings/FINDINGS-homelab001-local-bringup.md`
- `docs/orchestration/WAVE-RUN-LOG.md`
- `docs/orchestration/doc-batches/DOC-BATCH-2026-05-03-HOMEHUB-LOCAL-FINDINGS-01.md`
- `docs/gates/D1.md`
- `docs/research/S7-d1-bring-up-checklist.md`

## Attribution Chain
The control-plane role is a witness and ledger surface, not a claim amplifier.

Recommended chain:

```text
originating board or host
  -> exact command or operator action
  -> raw artifact path
  -> findings packet with local claim ceiling
  -> doc-batch summary that preserves source attribution
  -> run-log entry with stop-condition review
  -> x86_64/Fastest control-plane summary that repeats only supported claims
```

Rules:
- Every layer must name the predecessor surface explicitly.
- Higher layers may summarize, but may not reinterpret raw outputs into broader claims.
- Aggregation may state where a finding came from and what follow-up it implies, but it may not silently upgrade a finding into proof of Home Assistant, hardware durability, or gate movement.

## Concrete Routing Example
The current `homelab001` bridge-agent visibility mismatch is the minimum honest control-plane example already in repo.

### Source finding
- `docs/hardware/findings/FINDINGS-homelab001-local-bringup.md`
- machine: `homelab001`
- commands recorded:
  - `cargo run -p ferros-hub -- summary`
  - `cargo run -p ferros-node --bin ferros -- agent list`
  - `cargo run -p ferros-node --bin ferros -- agent describe ha-local-bridge`
- observed mismatch:
  - `ferros-hub summary` reports `bridgeAgent: ha-local-bridge@0.1.0`
  - `ferros agent list` shows only `echo, timer`
  - `ferros agent describe ha-local-bridge` returns `unknown agent`

### Staged aggregation path
- findings packet preserves raw commands, exit codes, and artifact paths under `.local-artifacts/reentry-homehub-local-01/`
- `docs/orchestration/doc-batches/DOC-BATCH-2026-05-03-HOMEHUB-LOCAL-FINDINGS-01.md` classifies the mismatch as a real local finding and names `REENTRY-HOMEHUB-LOCAL-AGENT-VISIBILITY-01` as the next segment
- `docs/orchestration/WAVE-RUN-LOG.md` records that honest local capture was the success condition and that the non-zero `agent describe` exit was preserved as a finding rather than hidden
- an `x86_64/Fastest` control-plane summary may therefore say only:
  - this finding was captured on `homelab001`
  - the bridge-agent visibility mismatch exists in the current local rehearsal surfaces
  - a narrower agent-visibility follow-up is warranted

### Prohibited upgrade from the same routing chain
- no claim that the bridge is fully registered in the CLI path
- no claim that Home Assistant integration works on a real host
- no claim that D1 or G4 evidence moved

## Surface Checklists

### Findings packet checklist
- capture date, operator, machine, and execution mode
- list exact commands run
- record exit results, including failures that remain part of the finding
- list artifact paths or copied outputs
- include a local claim ceiling and explicit non-claims
- classify remaining gaps rather than smoothing them away

### Doc-batch checklist
- state the source finding packet by path
- preserve which track or lane produced the observation
- restate only claims that were already supported in the findings packet
- list claims explicitly not added
- name the next queued segment when a narrower follow-up is known

### Run-log checklist
- record the success condition for the segment honestly
- preserve any non-zero exit that is part of the finding rather than treating it as invisible
- state why the segment can continue or must stop
- keep the stop-condition block aligned with the actual scope and evidence class

## Recommended Control-Plane Language
Allowed pattern:

> The x86_64 Fastest control-plane surface witnessed a source-attributed finding from the named board or host, preserved the existing claim ceiling from the findings packet, and routed the issue into the next queued follow-up without upgrading it into hardware, Home Assistant, or gate proof.

Forbidden pattern:

> Because the finding is visible in x86_64 control-plane summaries, the project has therefore proven bridge registration, Home Assistant integration, or launch readiness.

## Recommendation
- Resolve guardrail check 5 as `accept with caveats`: `x86_64/Fastest` can serve as the control-plane witness for findings only when source attribution is explicit and the claim ceiling is copied upward verbatim.
- Resolve guardrail check 6 as `adjust`: every aggregated control-plane statement must carry an explicit non-claim ledger so run-log or doc-batch wording cannot outrun the underlying findings packet.
- Keep the control-plane role descriptive and review-oriented; do not let it become evidence laundering.

## ADR Text Impact
- Clarify that `x86_64/Fastest` may aggregate findings as a control-plane witness, but only with named source attribution and no upward claim escalation.
- Clarify that findings, doc-batches, and run-log surfaces must repeat the active claim ceiling when a hardware-family result is summarized outside its origin surface.
- Clarify that the control-plane role does not convert local rehearsal findings into D1, G4, Home Assistant, or FERROS-native proof.

## D1/G4 Claim Impact
- No D1 movement.
- No G4 movement.
- No Home Assistant proof claim.
- No physical-device evidence claim.

## Research Disclaimer
This note defines an attribution and claim-boundary pattern only. It does not prove multi-board aggregation on real hardware and does not authorize any automatic control-plane claim upgrade.

## HANDOFF CARD
- Lane ID: O5
- Status: complete
- Files read: `docs/adr/ADR-025-dual-root-hardware-runway.md`; `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-guardrail-scoreboard.md`; `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-guardrails-batch1.md`; `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-guardrails-batch2.md`; `docs/hardware/findings/FINDINGS-homelab001-local-bringup.md`; `docs/orchestration/WAVE-RUN-LOG.md`; `docs/orchestration/doc-batches/DOC-BATCH-2026-05-03-HOMEHUB-LOCAL-FINDINGS-01.md`; `docs/gates/D1.md`; `docs/research/S7-d1-bring-up-checklist.md`
- Files changed: `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-evidence-routing-and-claim-boundary.md`
- Evidence produced: control-plane attribution and claim-boundary note for ADR-025
- Claims added: a source-attributed aggregation pattern now exists for x86_64 control-plane summaries
- Claims explicitly not added: ADR promotion, Home Assistant proof, device-control proof, D1 closure, G4 closure
- Validation: source-reference consistency review against current findings, doc-batch, and run-log surfaces
- Residual risks: lane-packet enforcement still needs explicit authority language before this becomes more than a review pattern
- Next safe follow-up, if any: land a live S9 packet example and then patch lane-packet enforcement