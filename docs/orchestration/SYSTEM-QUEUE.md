# FERROS System Queue

This queue feeds the local driver for **system-track** work: legal/T&C, ledger/chain ADR, asset library, character/profile templates, card/deck game/trade UX, smart-contract drafts, and onramp ADR work. Consumed only by Batch Mode runs scoped to `track: system`.

## Queue item schema (same shape as WAVE-QUEUE.md)

Required fields: `Title`, `Status`, `Priority`, `Gate`, `Owning streams`, `Goal`, `Anchor files`, `Validation`, `Constraints`, `Last update`

Optional fields (additive, do not break existing item order):
- `size: S | L` — S means ≤3 anchor files, single stream, docs-only. L means multi-stream or schema-touching. Batch Mode default consumes only S.
- `parallel-safe-with: [WAVE-IDs]` — explicit non-overlap declarations.
- `serial-after: WAVE-ID` — must wait for a prior wave.
- `solo: true | false` — must run alone (truth-sync, gate close, schema freeze, shared truth surfaces).
- `track: code | system | hardware` — which queue this belongs to.

---

## Ready

None. System queue empty.

---

## In Progress

None.

---

## In Progress

None.

---

## Blocked

None.

---

## Done

### SYSTEM-2026-05-03-12

- Title: S9 disposition and ADR-025 acceptance packet
- Status: done
- Priority: P1
- Gate: pre-ADR-025 acceptance
- Owning streams: S8 primary; S1, S4, and S7 awareness
- Goal: If `SYSTEM-2026-05-03-08` through `SYSTEM-2026-05-03-11` land cleanly, update ADR-025 from Proposed to Accepted or explicitly record why it remains Proposed, set the final S9 posture, and sync the scoreboard and ADR index surfaces.
- Anchor files: `docs/adr/ADR-025-dual-root-hardware-runway.md`, `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-guardrail-scoreboard.md`, `docs/adr/_INDEX.md`
- Validation: `get_errors` clean on the touched files; ADR-025 now records framework-level acceptance while preserving all family-level non-claims.
- Constraints: Docs-only. Run solo. Do not move D1 or G4. Do not convert research evidence into hardware proof. If acceptance criteria are still incomplete, this wave must record a non-acceptance disposition instead of force-accepting ADR-025.
- Last update: 2026-05-03
- size: S
- serial-after: SYSTEM-2026-05-03-11
- solo: true
- track: system

### SYSTEM-2026-05-03-11

- Title: ADR-025 lane-packet enforcement and authority patch
- Status: done
- Priority: P1
- Gate: pre-ADR-025 acceptance
- Owning streams: S8 primary; S1, S4, and S7 awareness
- Goal: Encode the minimum read-wide or write-narrow enforcement and serial truth-sync rules needed for lane packets, and prove them with one bounded batch-plan example that keeps shared truth surfaces non-overlapping.
- Anchor files: `docs/orchestration/LOCAL-DRIVER.md`, `docs/orchestration/BATCH-MODE.md`, `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-lane-packet-enforcement.md`
- Validation: `get_errors` clean on the touched authority docs and the new enforcement note; one bounded example batch shows non-overlap plus preserved serial truth-sync authority.
- Constraints: Docs-only but live orchestration-authority touch. Run solo. Do not widen Batch Mode beyond the current claim ceiling. Do not imply runtime enforcement exists where only policy language is being added.
- Last update: 2026-05-03
- size: S
- serial-after: SYSTEM-2026-05-03-10
- solo: true
- track: system

### SYSTEM-2026-05-03-10

- Title: S9 live ignition packet from a real finding
- Status: done
- Priority: P1
- Gate: pre-S9 approval
- Owning streams: S8 primary; S3, S4, and S7 awareness
- Goal: Write one actual S9 packet triggered by a real completed finding, showing what `S8` truth-sync already recorded, what `S9` adds beyond that record, which lane or root it reloads, and why the packet is non-redundant. Prefer the current `ha-local-bridge` visibility mismatch unless a stronger completed finding already exists in repo.
- Anchor files: `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-s9-live-packet-example.md`
- Validation: `get_errors` clean on `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-s9-live-packet-example.md`; packet cites a real source finding and names explicit reload or reroute outputs distinct from normal `S8` truth-sync.
- Constraints: Docs-only. Must cite a real source finding already in repo. Do not resolve the underlying code or hardware issue in this wave. Do not imply background autonomy, queue replacement, or gate movement.
- Last update: 2026-05-03
- size: S
- serial-after: SYSTEM-2026-05-03-09
- track: system

### SYSTEM-2026-05-03-09

- Title: ADR-025 evidence-routing and claim-boundary packet
- Status: done
- Priority: P1
- Gate: pre-ADR-025 acceptance
- Owning streams: S8 primary; S7 awareness
- Goal: Define the control-plane attribution chain and claim red-team checklist that orchestration outputs must carry so `x86_64/Fastest` can aggregate multi-board findings without source-of-evidence drift or overclaim.
- Anchor files: `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-evidence-routing-and-claim-boundary.md`
- Validation: `get_errors` clean on `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-evidence-routing-and-claim-boundary.md`; note includes one Pack B or Pack C attribution example plus checklist integration points for findings, doc-batches, and run-log surfaces.
- Constraints: Docs-only and research-only. Do not claim remote orchestration proof, Home Assistant proof, or physical-device evidence. Preserve the current claim ceiling language.
- Last update: 2026-05-03
- size: S
- serial-after: SYSTEM-2026-05-03-08
- track: system

### SYSTEM-2026-05-03-08

- Title: ADR-025 per-family lane profile and compression table
- Status: done
- Priority: P1
- Gate: pre-ADR-025 acceptance
- Owning streams: S8 primary; S1, S4, and S7 awareness
- Goal: Produce one research-backed note that turns guardrail checks 1 and 4 into concrete per-family tables for `x86_64`, `Raspberry Pi 4B`, `Jetson Orin Nano`, and `ESP32`, stating required versus optional lanes and the compressed-lane rule for constrained targets.
- Anchor files: `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-family-lane-profiles.md`
- Validation: `get_errors` clean on `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-family-lane-profiles.md`; note includes required-versus-optional lane tables and an explicit compressed profile for constrained targets.
- Constraints: Docs-only and research-only. Do not create hardware-root directories. Do not imply any board has runnable FERROS-native implementation, D1 evidence, or G4 evidence.
- Last update: 2026-05-03
- size: S
- track: system

### SYSTEM-2026-05-03-07

- Title: ADR-025 lane-packet metadata translation note
- Status: done
- Priority: P1
- Gate: pre-ADR-025 overlay pilot
- Owning streams: S8 primary; S1, S4, and S7 awareness
- Goal: Define how current queue metadata fields `size`, `parallel-safe-with`, `serial-after`, `solo`, and `track` would translate into ADR-025 lane packets so the overlay pilot can preserve the current scheduling and truth-sync discipline before any migration is attempted.
- Anchor files: `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-lane-packet-metadata-translation.md`
- Validation: `get_errors` clean on `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-lane-packet-metadata-translation.md`; note includes one-to-one mappings, preserved current semantics, and explicit enforcement gaps.
- Constraints: Docs-only and research-only. Run only after `SYSTEM-2026-05-03-06`. Do not change Batch Mode behavior in this wave. Do not claim that ADR-025 packets are active runtime authority yet.
- Last update: 2026-05-03
- size: S
- serial-after: SYSTEM-2026-05-03-06
- track: system

### SYSTEM-2026-05-03-06

- Title: Provisional S9 ignition non-redundancy packet
- Status: done
- Priority: P1
- Gate: pre-ADR-025 overlay pilot
- Owning streams: S8 primary; S4 and S7 awareness
- Goal: Write one provisional S9 service-packet note that proves a concrete reload or reroute function distinct from normal S8 truth-sync, defines the allowed triggers, inputs, outputs, and non-claims, and states how the packet remains subordinate to the current queue and gate system.
- Anchor files: `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-s9-provisional-service-packet.md`
- Validation: `get_errors` clean on `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-s9-provisional-service-packet.md`; note names a non-redundant S9 function, explicit authority boundary, and explicit non-claims.
- Constraints: Docs-only and research-only. Run only after `SYSTEM-2026-05-03-05`. Keep S9 provisional. Do not let S9 replace S8 truth-sync, queue authority, or gatekeeper logic. Do not imply always-running autonomous lanes.
- Last update: 2026-05-03
- size: S
- serial-after: SYSTEM-2026-05-03-05
- track: system

### SYSTEM-2026-05-03-05

- Title: x86_64 Fastest and FERROS overlay lane map
- Status: done
- Priority: P1
- Gate: pre-ADR-025 overlay pilot
- Owning streams: S8 primary; S1, S3, S4, S5, S6, and S7 awareness
- Goal: Produce one research-backed lane-map note that crosswalks the current S1-S8 stream system into provisional `x86_64/Fastest` and `x86_64/FERROS` lanes, tests the proposed Fastest grouping and FERROS-side architecture grouping, and states exactly which current authorities remain in force.
- Anchor files: `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-x86-overlay-lane-map.md`
- Validation: `get_errors` clean on `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-x86-overlay-lane-map.md`; note includes current-stream crosswalk, claim ceilings, preserved authorities, and explicit unresolved edges.
- Constraints: Docs-only and research-only. Run only after `SYSTEM-2026-05-03-04`. Do not create the hardware-root directory standard yet. Do not imply stream retirement, gate movement, hardware evidence, or FERROS-native runtime proof.
- Last update: 2026-05-03
- size: S
- serial-after: SYSTEM-2026-05-03-04
- track: system

### SYSTEM-2026-05-03-04

- Title: ADR-025 x86_64 overlay pilot coordination lock
- Status: done
- Priority: P1
- Gate: pre-ADR-025 overlay pilot
- Owning streams: S8 primary; S1, S4, and S7 awareness
- Goal: Create one explicit coordination note that starts the ADR-025 x86_64 overlay pilot as a bounded non-binding packet, preserves the current S1-S8 plus gate plus queue authority stack, records the claim ceiling, and names the next queued overlay waves.
- Anchor files: `docs/orchestration/ADR-025-X86-OVERLAY-PILOT-01.md`
- Validation: `get_errors` clean on `docs/orchestration/ADR-025-X86-OVERLAY-PILOT-01.md`; note records current authority, claim ceiling, provisional S9 posture, and queued next waves.
- Constraints: Docs-only. Do not promote ADR-025. Do not retire or replace the current stream stack. Do not create binding hardware-root authority. Do not move D1 or G4. Do not claim background autonomy.
- Last update: 2026-05-03
- size: S
- track: system

### SYSTEM-2026-04-27-02

- Title: Draft ADR — Ledger/chain substrate comparison and recommendation
- Status: done
- Priority: P1
- Gate: pre-G4 policy runway
- Owning streams: S8 primary; S6 consumer awareness
- Goal: Author a comparison ADR evaluating Solana, EVM L2, Cosmos app-chain, and non-chain signed-ledger options against FERROS invariants (locally sovereign, consent-first, deliverable on flashdrive, signed grants exist). The ADR produces a recommendation but not a commitment. It cross-references existing prior-art mentions in the repo ADR set.
- Anchor files: `docs/adr/ADR-024-ledger-substrate.md`
- Validation: `get_errors` clean on `docs/adr/ADR-024-ledger-substrate.md` and `docs/adr/_INDEX.md`.
- Constraints: Docs-only. Recommendation only, not a binding commitment. Do not reopen G1–G3. Do not mutate frozen schemas.
- Last update: 2026-04-27
- size: S
- track: system

### SYSTEM-2026-04-27-03

- Title: Legal/T&C scaffold — Terms, licensing posture, and consent language
- Status: done
- Priority: P2
- Gate: pre-G4 policy runway
- Owning streams: S8 primary
- Goal: Scaffold `docs/legal/` with three placeholder files: `TERMS-OF-USE.md`, `LICENSING-POSTURE.md`, and `CONSENT-LANGUAGE.md`. Each file states the FERROS posture in plain English and explicitly marks itself as draft awaiting counsel red-line. No legal advice is given or implied; these are structured placeholders so counsel has a clear surface to red-line rather than starting from scratch.
- Anchor files: `docs/legal/TERMS-OF-USE.md`, `docs/legal/LICENSING-POSTURE.md`, `docs/legal/CONSENT-LANGUAGE.md`
- Validation: `get_errors` clean on the three new files.
- Constraints: Docs-only. Mark every file as draft. Do not claim these constitute legal advice or final terms.
- Last update: 2026-04-27
- size: S
- track: system

### SYSTEM-2026-04-27-01

- Title: Draft ADR — External systems are onramps, not identity truth
- Status: done
- Priority: P1
- Gate: pre-G4 policy runway
- Owning streams: S8 primary; S7 consumer awareness; S2 consumer awareness
- Goal: Author the onramp policy ADR covering Home Assistant, calendar import, social-graph import (LinkedIn/Facebook style), and bundle/migration pipelines. The ADR frames all imported data as proposed FERROS material requiring explicit consent before becoming canonical state. It does not constrain the HA bridge implementation details (those remain S7-owned) but does establish the data-direction invariant so future S7 implementation work can reference a decided policy rather than re-litigating it mid-flight. Cross-check against `docs/adr/ADR-021-dependency-admission-policy.md` for consistency.
- Anchor files: `docs/adr/ADR-023-onramp-policy.md`
- Validation: `get_errors` clean on `docs/adr/ADR-023-onramp-policy.md` and `docs/adr/_INDEX.md`.
- Constraints: Docs-only. Do not claim HA bridge implementation details, pairing handshake order, or consent UI internals. Do not reopen G1–G3. Do not mutate frozen schemas.
- Last update: 2026-04-27
- size: S
- track: system

None yet. System queue established in WAVE-2026-04-27-03.
