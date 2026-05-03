# RN-2026-05 ADR-025 x86_64 Overlay Lane Map

Status: Draft
Scope: Research-only
Constraint: ADR-025 remains Proposed and non-binding.

## Question
- How can ADR-025 be piloted on `x86_64` without retiring the current S1-S8 stream, gate, and queue substrate?
- What is the smallest honest mapping from the current stream model into provisional `x86_64/Fastest` and `x86_64/FERROS` lanes?

## Evidence Reviewed
- `STATUS.md`
- `docs/adr/ADR-025-dual-root-hardware-runway.md`
- `docs/orchestration/ADR-025-X86-OVERLAY-PILOT-01.md`
- `streams/S1-foundation/README.md`
- `streams/S2-profile/README.md`
- `streams/S3-agent-center/README.md`
- `streams/S4-runtime/README.md`
- `streams/S5-ux/README.md`
- `streams/S6-harvest/README.md`
- `streams/S7-hub/README.md`
- `streams/S8-docs/README.md`

## Current-Stream To x86_64 Overlay Crosswalk

### x86_64/Fastest

| Lane | Current stream anchor | Overlay role | Current authority that remains live |
|------|-----------------------|--------------|------------------------------------|
| `F1` | `S1 Foundation` | host baseline, toolchain, CI, release hygiene | S1 still owns foundation closeout and workflow truth |
| `F2` | `S2 Profile` | identity, profile, grant, local trust boundary | S2 still owns frozen profile and grant boundaries |
| `F3` | `S3 Agent Center` | local agents, registry, lifecycle, logs | S3 still owns CLI and localhost agent control seams |
| `F4` | `S4 Runtime` | executor, bus, policy, runtime checkpoint surfaces | S4 still owns runtime primitives and `no_std` posture |
| `F5` | `S5 UX` | localhost shell and operator-facing read path | S5 still owns shell rendering and browser-facing wording |
| `F6` | `S6 Harvest` | typed local models and prior-art-fed data seams | S6 still owns `ferros-data` and harvest boundaries |
| `F7` | `S7 Hub` | `ferros-hub`, local bridge rehearsal, x86_64 integration runway | S7 still owns hub proof-chain and hardware runway claims |
| `F8` | `S8 Docs` | governance, queue truth, run-log, claim ceilings | S8 still owns truth-sync and ADR bookkeeping |
| `F9` | `S9` provisional | ignition, reload, reroute, handoff proposals | remains subordinate to S8 truth-sync and queue authority |

### x86_64/FERROS

| Lane | Research area | Near-term posture |
|------|---------------|-------------------|
| `R1` | bootloader and boot path | architecture-first research only |
| `R2` | kernel and privilege model | architecture-first research only |
| `R3` | process and lifecycle model | architecture-first research only |
| `R4` | memory and storage primitives | architecture-first research only |
| `R5` | driver and hardware-abstraction model | architecture-first research only |
| `R6` | networking primitives | architecture-first research only |
| `R7` | display and UI subsystem | architecture-first research only |
| `R8` | package, update, and onramp behavior | architecture-first research only |
| `R9` | ignition and synthesis back into Fastest constraints | provisional and proposal-only |

## Candidate Fastest Grouping Assessment

Candidate grouping from the pilot note:
- `S1-S4` backend
- `S5` mediator
- `S6-S9` operator-facing surfaces

Assessment:
- `S1-S4` as a backend cluster is directionally useful for x86_64 pilot framing because the current repo already converges foundation, identity, agent, and runtime work into the practical local stack.
- `S5` is plausibly a mediator lane because it is the main read-first operator surface, but it should stay display or control-shell scoped rather than becoming a new source of truth.
- `S6-S9` should not be treated as one operator-facing bundle. `S6` is a typed-model and research seam, `S7` is operational hub work, `S8` is governance and truth-sync, and `S9` is provisional coordination. Keeping them separate is safer than flattening them into one front-end cluster.

Recommended x86_64 pilot grouping:
- Tier 1: `F1-F4` backend and platform substrate
- Tier 2: `F5` observation and operator mediation
- Tier 3a: `F7` operational hub lane
- Tier 3b: `F6` harvest and typed-model lane
- Tier 3c: `F8` governance and truth-sync lane
- Tier 3d: `F9` ignition and reroute lane

## Preserved Authorities
- `STATUS.md` remains the live gate and stream authority.
- `docs/orchestration/LOCAL-DRIVER.md` and `docs/orchestration/BATCH-MODE.md` remain the live orchestration authority.
- `docs/orchestration/WAVE-QUEUE.md`, `docs/orchestration/SYSTEM-QUEUE.md`, and `docs/orchestration/HARDWARE-QUEUE.md` remain the live scheduling authority.
- S1-S8 stream ownership remains intact; the overlay re-labels the x86_64 pilot surface but does not retire stream boundaries.
- Gate order remains `G1 -> G2 -> G3 -> D1/G4 runway`; the overlay does not replace gate sequencing.

## Unresolved Edges
- Whether `F1-F4` can be queued as a practical bundle without blurring S2, S3, and S4 ownership boundaries.
- Whether `F5` needs explicit read-only authority language in the overlay map.
- Whether `F6`, `F7`, `F8`, and `F9` need separate packet classes instead of one shared "front-end" grouping.
- How `R1-R9` research lanes should be scheduled relative to current `track: system` and `track: code` work.
- How `F9` and `R9` relate to the future Agent Center traffic view without implying background autonomy.

## Recommendation
- Accept the x86_64 overlay pilot as a mapping exercise only.
- Keep the current S1-S8 stream, gate, and queue stack as live authority while the overlay is tested.
- Use this note as the basis for the provisional S9 packet and the lane-packet metadata translation note.
- Do not create the hardware-root directory standard yet.

## ADR Text Impact
- Add an explicit x86_64 overlay pilot note to ADR-025 so the first pilot is described as a mapping over current stream truth rather than a replacement governance model.
- Clarify that the proposed Fastest grouping is a candidate taxonomy and not a current stream-ownership rewrite.
- Clarify that `S9` remains subordinate to `S8` truth-sync until non-redundancy is proven.

## D1/G4 Claim Impact
- No D1 movement.
- No G4 movement.
- No Home Assistant proof claim.
- No physical-device evidence claim.
- No FERROS-native OS claim.

## Research Disclaimer
This note maps a candidate x86_64 overlay shape. It does not promote ADR-025, retire S1-S8, change queue authority, move any gate, or claim hardware or native-runtime evidence.

## HANDOFF CARD
- Lane ID: O1
- Status: complete
- Files read: `STATUS.md`; `docs/adr/ADR-025-dual-root-hardware-runway.md`; `docs/orchestration/ADR-025-X86-OVERLAY-PILOT-01.md`; `streams/S1-foundation/README.md`; `streams/S2-profile/README.md`; `streams/S3-agent-center/README.md`; `streams/S4-runtime/README.md`; `streams/S5-ux/README.md`; `streams/S6-harvest/README.md`; `streams/S7-hub/README.md`; `streams/S8-docs/README.md`
- Files changed: `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-x86-overlay-lane-map.md`
- Evidence produced: x86_64 overlay lane-map note with current-stream crosswalk and preserved-authority constraints
- Claims added: candidate x86_64 Fastest or FERROS mapping over current stream truth
- Claims explicitly not added: ADR promotion, stream retirement, gate movement, hardware evidence, native-runtime proof
- Validation: source-reference consistency review against current stream READMEs and overlay pilot note
- Residual risks: S9 non-redundancy, metadata translation, and research-lane scheduling remain unresolved
- Next safe follow-up, if any: land the provisional S9 service-packet note and the metadata translation note