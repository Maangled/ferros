# S7 Smart-Home Hub — Progress

Reverse-chronological. Append a dated entry at the top per session.

---

## 2026-04-29 — Local onramp rehearsal packet landed on the existing runway route

- WAVE-2026-04-29-55 published `LocalOnrampProposal` in `ferros-data` with bounded validation and local JSON write helpers for quarantined pending-consent proposed material.
- WAVE-2026-04-29-56 emitted one `.tmp/hub/local-onramp-proposal.json` artifact from the allowed simulated bridge proof path and threaded it into the hub-owned runtime summary seam.
- WAVE-2026-04-29-57 added bounded schema and H1 validator coverage around the proposal artifact without reopening frozen S2 schemas.
- WAVE-2026-04-29-58, WAVE-2026-04-29-59, and WAVE-2026-04-29-60 kept observation on the existing read-only `/runway-summary(.json)` route: optional `hubOnrampProposal` payload in node, display-only shell/inspector rendering, and same-origin acceptance-harness proof on that same route.
- WAVE-2026-04-29-61 extended `cargo xtask hub-runway` so it validates and reports the emitted proposal artifact while reusing the same hub-owned summary seam.
- Kept the entire packet local-only and non-gate-closing: no accept/reject flow, no canonical profile/grant mutation, no remote transport, no Home Assistant proof, no physical-device evidence, and no G4 closure.

## 2026-04-29 — Restart-aware local hub runway packet landed

- WAVE-2026-04-29-46 added a bounded typed `LocalHubStateSnapshot` seam under `.tmp/hub/` and kept its guardrails local-only and non-evidentiary.
- WAVE-2026-04-29-47 threaded that seam into the typed hub runtime summary plus the existing `summary | prove-bridge` outputs, reporting bounded `fresh-start | reloaded | unavailable` restart context without widening into a public restart API.
- WAVE-2026-04-29-48 extended the existing read-only `/runway-summary(.json)` seam additively with optional `hubRestart` runway context sourced from the hub summary seam.
- WAVE-2026-04-29-49 and WAVE-2026-04-29-50 kept that context display-only on the existing localhost shell route and proved the same route through the same-origin acceptance harness.
- WAVE-2026-04-29-51 added the bounded local restart snapshot schema plus H1 validator parity coverage for banned summary wording and remote-looking text.
- WAVE-2026-04-29-52 aligned `cargo xtask hub-runway` to prove snapshot write/reload through the hub-owned summary seam and print the exact `ferros-hub summary` output.
- Kept the whole packet local-only and non-gate-closing: no G4 closure, no physical-device evidence, no Home Assistant integration proof, no remote transport, and no daemon/server claim.

## 2026-04-28 — Local hub library, policy, CLI, schema, and xtask runway landed

- WAVE-2026-04-28-38 promoted `ferros-hub` from a binary-first scaffold to a library-backed local runway crate and kept the binary thin.
- WAVE-2026-04-28-39 moved local bridge registration onto `ferros-agents` manifest/registry primitives while preserving the local-only bridge row.
- WAVE-2026-04-28-40 moved local allow/deny evaluation onto `ferros-core` policy primitives over real `ferros_profile::CapabilityGrant` input, including revoked-grant semantics.
- WAVE-2026-04-28-41 added a typed `LocalHubRuntimeSummary` over the landed registry and policy seam.
- WAVE-2026-04-28-42 added thin `ferros-hub summary | prove-bridge | deny-demo` proof commands over the landed library surface.
- WAVE-2026-04-28-43 admitted bounded local artifact/report schemas into `harnesses/_constants.js` and the H1 contract validator without widening production hub code.
- WAVE-2026-04-28-44 added `cargo xtask hub-runway` as a helper over the same local hub proof seam and confirmed the existing `.tmp/hub/simulated-local-bridge-artifact.json` path.
- Kept the entire packet local-only and non-gate-closing: no D1 or G4 movement, no Home Assistant dashboard proof, no hardware evidence, and no remote-transport claim.

## 2026-04-28 — Local-only ferros-hub scaffold, bridge seam, and proof loop landed

- WAVE-2026-04-28-34 added the first local-only `ferros-hub` workspace member and binary scaffold and codified subagent review as the default safety posture for this size-L execution.
- WAVE-2026-04-28-35 added the first crate-local bridge seam with one default local bridge agent and one simulated local artifact summary without adding Home Assistant, hardware, remote-transport, or privileged-write claims.
- WAVE-2026-04-28-36 extended that seam with simulated allow, deny, and invalid-path error/reporting behavior under `cargo test -p ferros-hub bridge_` and emitted `.tmp/hub/simulated-local-bridge-artifact.json` with local-only, non-evidentiary fields.
- Kept the slice non-gate-closing: no D1 or G4 movement, no Home Assistant proof, and no physical-device evidence.

## 2026-04-26 — Pack B operator rehearsal notes added to the runway docs

- Extended `docs/hub/reference-hardware.md` and `docs/hub/pack-b-bring-up-worksheet.md` with prep-only operator rehearsal prompts for clean reboot, DUT-only cold boot, consent-deny observation, and artifact capture discipline.
- Kept the slice runway-only and evidence-prep only: no `crates/ferros-hub/` scaffold, no Home Assistant bridge implementation, no `docs/gates/G4.md` edits, and no launch or hardware evidence claims.

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
