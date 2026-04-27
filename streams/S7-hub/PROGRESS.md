# S7 Smart-Home Hub — Progress

Reverse-chronological. Append a dated entry at the top per session.

---

## 2026-04-26 — Pack B bring-up worksheet prepared from the runway map

- Added `docs/hub/pack-b-bring-up-worksheet.md` as the first operator worksheet for Pack B x86_64 bring-up sessions plus a separate Pack C Home Assistant host.
- Kept `docs/hub/reference-hardware.md` as the authority and mirrored its topology, pre-run checks, and G4 evidence rows into capture placeholders only.
- Kept the slice prep-only: no `crates/ferros-hub/` scaffold, no Home Assistant bridge implementation, no `docs/gates/G4.md` edits, and no launch or hardware evidence claims.

## 2026-04-24 — First Home Assistant bridge runway contract defined

- Added a narrow bridge contract in `streams/S7-hub/CONTRACTS.md` that fixes the first FERROS-side assumptions at one bridge agent through the S3 registry/list path, manifest-declared `CapabilityRequirement` values bounded by S2 `CapabilityGrant` state, one real-entity minimum evidence slice, operator-visible deny attribution, restart-safe FERROS state, and the external HA-fork boundary.
- Marked the first Home Assistant lab topology and bridge-assumption slice as defined in `streams/S7-hub/BACKLOG.md` while keeping the remaining next steps limited to evidence prep and upstream pairing questions.
- No `crates/ferros-hub/` scaffold, no `Maangled/home-assistant` fork changes, and no G4 truth changes were made in this pass.

## 2026-04-24 — x86_64-first bring-up contract mapped

- Converted the S7 runway from broad hardware-prep prose into a concrete first bring-up contract centered on the Pack B `x86_64` lane plus a separate Pack C Home Assistant host.
- Mapped each unchecked G4 evidence item to one upstream seam and one S7-owned proof point in `streams/S7-hub/CONTRACTS.md` and `docs/hub/reference-hardware.md` without claiming launch evidence or freezing pairing semantics.
- Kept the slice in runway mode only: no `crates/ferros-hub/` scaffold, no HA bridge code, no protocol ratification, and no G4 truth changes.

## 2026-04-24 — G3 closed; G4 runway active

- Recorded the first hosted green CI proof for the landed G3 workflow path and truth-synced the gate and status surfaces so G4 is now the active gate.
- S7 is no longer blocked on G3; runway work can continue while the real `ferros-hub` binary, HA bridge, and physical-device evidence remain open.

---

## 2026-04-24 — Hardware runway tightened

- Expanded `docs/hub/reference-hardware.md` from a placeholder into a runway and evidence-prep doc aligned to `LAUNCH.md` and `docs/gates/G4.md`.
- Reframed S7 planning docs around runway mode so they do not overclaim a running hub or a finalized pairing protocol before implementation exists.
- Stream remains blocked on G3 and on the absence of a real `ferros-hub` binary, HA bridge, and physical-device evidence.

---

## 2026-04-21 — Stream scaffolded

- Stream directory and planning documents created.
- Blocked on G3 (S3 + S4 minimal agent-center-on-runtime demo).
- Reference hardware doc and pairing flow design can begin before G3.
- This stream owns the launch gate (G4).
