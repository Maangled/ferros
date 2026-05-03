# RN-2026-05 ADR-025 Family Lane Profiles

Status: Draft
Scope: Research-only
Constraint: ADR-025 remains Proposed and non-binding.

## Question
- Are `S1-S8` sufficient to describe each hardware family without hidden governance gaps?
- Which targets need compressed or federated lane shapes rather than a full lane-for-lane expansion?

## Evidence Reviewed
- `docs/adr/ADR-025-dual-root-hardware-runway.md`
- `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-guardrail-scoreboard.md`
- `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-guardrails-batch1.md`
- `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-guardrails-batch2.md`
- `docs/hardware/d1-target-inventory.md`
- `docs/research/S4-no-std-target-matrix.md`
- `docs/research/S7-d1-bring-up-checklist.md`
- `docs/streams/STREAM-E-CORE-OS.md`

## Lane Status Terms
- `required`: lane must be explicitly present for the family or root.
- `optional`: lane may exist, but its absence does not invalidate the family profile.
- `federated`: lane responsibility stays real, but the write-owning implementation or evidence surface lives on an x86_64 control plane rather than on the target itself.
- `deferred`: lane belongs to the long-term family shape but should not be treated as active runway work yet.

## Family Baseline

| Family | Current practical posture | Evidence basis for posture |
|--------|---------------------------|----------------------------|
| `x86_64` | primary Fastest and primary FERROS research family | selected first D1 target class plus x86_64-first OS doctrine |
| `Raspberry Pi 4B` | Linux-first edge migration family | secondary D1 candidate plus ARM-is-secondary doctrine |
| `Jetson Orin Nano` | Linux-first specialized edge family | edge ARM migration family with vendor-image dependency |
| `ESP32` | FERROS-compatible peripheral family, not a FERROS OS target | micro-node doctrine plus `no_std` and flash or RAM constraints |

## Per-Family Lane Profiles

### x86_64

| Lane | Fastest | FERROS | Notes |
|------|---------|--------|-------|
| `S1` | required | required | x86_64 is the first kernel or boot research family and the strongest early real-hardware target. |
| `S2` | required | optional | identity and local trust boundary are first-class on Fastest; FERROS-side identity may lag early boot or runtime work. |
| `S3` | required | optional | agent-center behavior is already active on Fastest; FERROS-side process and lifecycle research may precede full agent-center parity. |
| `S4` | required | required | runtime and process model are core on both roots. |
| `S5` | required | optional | Fastest owns the practical operator surface; FERROS-side UI research may stay behind runtime proof. |
| `S6` | required | optional | artifact storage is active on Fastest; FERROS-side storage design may remain research-first. |
| `S7` | required | optional | Fastest owns current hub runway and Home Assistant rehearsal; FERROS-side bridge behavior remains research-only. |
| `S8` | required | required | evidence, claims, and truth-sync remain mandatory on both roots. |
| `S9` | provisional | provisional | allowed only as a proposal lane subordinate to `S8` truth-sync. |

Verdict:
- `x86_64/Fastest` should keep the full practical lane set.
- `x86_64/FERROS` should keep the full conceptual lane map, but only `S1`, `S4`, and `S8` are currently hard-required for honest research progress.

### Raspberry Pi 4B

| Lane | Fastest | FERROS | Notes |
|------|---------|--------|-------|
| `S1` | required | deferred | Pi remains a Linux-first migration family now; native boot work can follow x86_64 FERROS stabilization. |
| `S2` | federated | deferred | identity can be anchored on the x86_64 control plane during early Pi bring-up. |
| `S3` | required | optional | local agents matter for kiosk or room-hub behavior; FERROS-side agent model can trail runtime research. |
| `S4` | required | required | Linux runtime today, native runtime research later. |
| `S5` | required | optional | Pi is a realistic display or kiosk surface. |
| `S6` | federated | deferred | artifact and ledger writes can be pushed upward to x86_64 during early runway work. |
| `S7` | required | optional | Pi remains a plausible Home Assistant-facing edge node. |
| `S8` | required | required | findings, evidence routing, and claim ceilings still need an explicit Pi surface even when storage is federated. |
| `S9` | provisional | provisional | allowed for reroute or appliance split proposals only. |

Verdict:
- Pi can use a compressed practical profile through federation of `S2` and `S6` to x86_64.
- Pi should not be treated as the first FERROS-native family while x86_64 kernel or runtime proof remains earlier on the roadmap.

### Jetson Orin Nano

| Lane | Fastest | FERROS | Notes |
|------|---------|--------|-------|
| `S1` | required | deferred | Jetson Fastest depends on vendor-image or boot-chain reality; FERROS-side boot remains later research. |
| `S2` | federated | deferred | early identity and enrollment can stay anchored on x86_64. |
| `S3` | required | optional | inference or coordinator agents are practical Fastest work. |
| `S4` | required | optional | runtime work is important, but GPU or vendor constraints keep FERROS-side runtime research bounded. |
| `S5` | required | optional | operator or model-observation UI is practical here. |
| `S6` | federated | deferred | artifact or model cache ownership can stay above the board during early work. |
| `S7` | required | optional | Jetson can serve hub or edge-inference bridge roles, but without proof claims beyond local runway. |
| `S8` | required | required | evidence and claim discipline stay mandatory. |
| `S9` | provisional | provisional | may propose an appliance split or inference-hub reroute only. |

Verdict:
- Jetson should use the same compressed Fastest posture as Pi: local runtime and operator behavior, federated identity and storage, no native-runtime overclaim.
- Jetson FERROS work should stay explicitly deferred while x86_64 FERROS remains the first serious OS target.

### ESP32

| Lane | Fastest | FERROS | Notes |
|------|---------|--------|-------|
| `S1` | required | required | boot and firmware boundary are unavoidable on a micro-node. |
| `S2` | federated | federated | persistent identity should be anchored in a parent FERROS node or companion control plane. |
| `S3` | optional | optional | a single-purpose firmware may not need a general agent registry. |
| `S4` | required | required | the runtime or event-loop model is the core technical question on ESP32. |
| `S5` | optional | optional | physical I/O may exist, but not every ESP32 needs a distinct UX lane beyond the device contract. |
| `S6` | federated | federated | storage and artifact retention should not be assumed locally on a constrained target. |
| `S7` | required | optional | the board's realistic role is as a peripheral or bridge-facing device. |
| `S8` | required | required | even a peripheral target needs a bounded evidence and claim lane. |
| `S9` | provisional | provisional | allowed only when a board-specific ignition or handoff packet is actually needed. |

Verdict:
- ESP32 needs the strongest compression rule in the set.
- The practical minimum honest profile is `S1`, `S4`, `S7`, and `S8`, with `S2` and `S6` federated upward and `S3` or `S5` present only when the board actually exposes them.
- This matches current doctrine that ESP32 is a FERROS-compatible ecosystem device, not the first FERROS OS computer.

## Compressed-Lane Rules
- Constrained targets may compress lanes only by marking them `federated`, `optional`, or `deferred`; they may not silently disappear.
- `S8` may not be removed from any family. Evidence and claim ceilings always need an explicit home even when artifact storage is federated.
- `S2` and `S6` are the first lanes eligible for federation to x86_64 because early edge or peripheral work does not need independent canonical identity or durable artifact ownership on every board.
- `S3` may be optional on a single-purpose peripheral target, but only when the board is explicitly not serving as a general FERROS agent host.
- `S5` may be optional on a headless or purely sensor-actuator target, but a Pi or Jetson kiosk-style surface should keep it active.
- `S1` and `S4` remain the non-negotiable technical lanes for any family that still claims a real runtime or firmware posture.

## Recommendation
- Resolve guardrail check 1 as `adjust`: `S1-S8` remain sufficient as the baseline vocabulary, but the family profile must declare which lanes are required, optional, federated, or deferred.
- Resolve guardrail check 4 as `adjust`: ESP32 and similar constrained targets should use a compressed profile instead of pretending every family has the same practical lane count.
- Keep x86_64 as the only family where both Fastest and FERROS roots can honestly present the full lane map today.
- Treat Pi and Jetson as migration families with federated identity and storage during early runway work.
- Treat ESP32 as a peripheral family whose compressed profile protects the model from over-scaffolding and false OS claims.

## ADR Text Impact
- Clarify that ADR-025 uses a family-specific lane profile rather than a literal identical file tree for every board class on day one.
- Clarify that compressed families must name federated and deferred lanes explicitly rather than omitting them silently.
- Clarify that `ESP32` is a compressed peripheral family in the current model and does not prove FERROS-native OS viability by itself.

## D1/G4 Claim Impact
- No D1 movement.
- No G4 movement.
- No Home Assistant proof claim.
- No physical-device evidence claim.
- No FERROS-native OS claim for Pi, Jetson, or ESP32.

## Research Disclaimer
This note resolves the lane-sufficiency and embedded-compression questions at the research level only. It does not instantiate the proposed hardware-root directory standard, does not promote ADR-025, and does not create new hardware evidence.

## HANDOFF CARD
- Lane ID: O4
- Status: complete
- Files read: `docs/adr/ADR-025-dual-root-hardware-runway.md`; `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-guardrail-scoreboard.md`; `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-guardrails-batch1.md`; `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-guardrails-batch2.md`; `docs/hardware/d1-target-inventory.md`; `docs/research/S4-no-std-target-matrix.md`; `docs/research/S7-d1-bring-up-checklist.md`; `docs/streams/STREAM-E-CORE-OS.md`
- Files changed: `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-family-lane-profiles.md`
- Evidence produced: family-by-family lane profile and compressed-lane rule note
- Claims added: a concrete lane vocabulary now exists for `x86_64`, `Raspberry Pi 4B`, `Jetson Orin Nano`, and `ESP32`
- Claims explicitly not added: ADR promotion, hardware-root directory creation, D1 closure, G4 closure, Home Assistant proof, FERROS-native OS proof
- Validation: source-reference consistency review against ADR-025 and current hardware or orchestration notes
- Residual risks: evidence-routing and S9 non-redundancy still need their own completed notes before ADR disposition
- Next safe follow-up, if any: land the evidence-routing and live S9 packet notes, then return to authority enforcement