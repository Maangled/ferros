# Ferros Hub Local LAN Device Onboarding Plan

Status: Draft
Date: 2026-05-03
Scope: planning only

## Purpose
Define the first local LAN-device onboarding plan for `homelab001` without requiring Home Assistant.

## Planning Posture
- Local observation first.
- Device records are proposed material until accepted.
- Home Assistant is optional and lower priority than profile or hub bring-up.
- Matter remains a future path unless separately implemented and proven.

## Claim Ceiling
- Do not claim Matter support or Matter onboarding.
- Do not claim device control.
- Do not claim packet inspection, flow telemetry, or network accounting beyond explicit local observation paths.
- Treat semi-smart mirrors as observed LAN devices, not integrated FERROS devices, unless a real integration path exists.

## Phase 0 Onboarding Shape

### 1. Local Inventory
Capture devices with operator-owned local observations only:
- manual device label
- local hostname if visible
- IP or MAC if visible from operator-owned network surfaces
- observed service or behavior note

### 2. Proposed-Material Staging
Treat each discovered device as proposed material, not canonical FERROS state, until the operator accepts the record.

### 3. Ownership And Consent Note
Record who owns the device or whether observation is authorized on the local LAN before claiming any meaningful tracking or onboarding.

### 4. Hub-Visible Observation Plan
The near-term target is a hub-visible inventory or observation surface, not device control. For Phase 0, acceptable local evidence is:
- a device note captured under `.local-artifacts`
- a local shell observation path
- a router or host-side observation note owned by the operator

## Suggested Device Classes

### Generic LAN Device
- Record label, address information if visible, and observed service note.

### Matter-Capable Device
- Record only as a candidate device class unless real Matter commissioning or inspection exists.

### Semi-Smart Mirror
- Treat as a network-observed display device.
- Record weather-fetch or network behavior only as an operator observation.
- Do not imply direct FERROS control or official smart-home integration.

## Minimal Device Evidence Template

| Field | Value to capture |
|------|-------------------|
| Device label | `___` |
| IP or MAC if available | `___` |
| Discovery method | `manual note` / `operator-owned LAN surface` / `future hub observation` |
| Observed service or behavior | `___` |
| Network usage observation path | `___` |
| Consent or ownership note | `___` |
| What Ferros can honestly claim | `local observation only` / `local proposed-material record only` |
| What Ferros cannot honestly claim | `device control` / `Matter support` / `deep telemetry` unless separately proven |

## Local Observation Paths Allowed In This Plan
- operator-entered notes under `.local-artifacts`
- host or LAN observations the operator already controls
- future read-only hub inventory surfaces once implemented

## Local Observation Paths Not Yet Claimed
- no packet capture
- no flow collector
- no automatic device fingerprinting
- no Home Assistant discovery dependency
- no remote inventory service

## ADR-023 Boundary
Discovered device records are onramp-style proposed material. Observation does not equal acceptance, ownership, or canonical FERROS state.

## HANDOFF CARD
- Lane ID: A2
- Status: complete
- Files read: docs/hardware/; docs/hub/; docs/research/S7-d1-bring-up-checklist.md; docs/adr/ADR-023-onramp-policy.md; docs/orchestration/REENTRY-PHASE0-HARDWARE-READINESS-AUDIT.md; streams/S7-hub/README.md
- Files changed: docs/hub/local-lan-device-onboarding.md
- Evidence produced: planning note for local LAN-device onboarding without Home Assistant dependency
- Claims added: local observation-first onboarding plan and minimal device evidence template
- Claims explicitly not added: Matter support, device control, packet inspection, network telemetry, Home Assistant proof
- Validation: repo-grounded planning review against current docs and claim ceilings
- Residual risks: no implementation or live device inventory surface exists yet
- Next safe follow-up, if any: queue a narrow code-track wave for local device inventory only if it can be done without new dependencies