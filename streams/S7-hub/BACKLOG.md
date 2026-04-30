# S7 Smart-Home Hub — Backlog

---

## Now

- [ ] Keep `docs/hub/reference-hardware.md` current as the hardware recipe authority for candidate hardware, storage/network assumptions, and G4 evidence fields
- [ ] Select one `aarch64` candidate and one `x86_64` candidate for the first on-device sessions; treat the Pack B `x86_64` lane as the first bring-up target unless hardware reality forces Pi-first
- [x] Define the first Home Assistant lab topology for hardware bring-up and consent-deny observation around one device under test plus one separate HA host
- [x] Define the first Home Assistant bridge runway contract at one bridge agent, one real entity, runtime-enforced capability checks, operator-visible deny traces, and restart-safe FERROS state
- [x] Land the runway-only six-checkpoint pairing map bounded by S2 consumer surfaces (`ProfileId`, `CapabilityGrant`) and the current S3/S4 seams
- [x] Land the queue-ready S2 consumer-boundary question list for bootstrap, grant check, deny visibility, persistence, revocation, and re-registration without freezing protocol steps
- [x] Consume the published S2 consumer-boundary answers into an S7-owned handoff/assumptions slice without freezing handshake order, storage layout, or re-registration choreography
- [x] Land the docs-only S7 seam brief keyed to the current S3 registration and inspection surfaces plus the current S4 policy and restart seams
- [x] Define the operator-facing evidence surface (read-only) for hub bring-up and status, sourced from `docs/hub/pack-b-bring-up-worksheet.md`, with explicit field definitions, read-only constraint, D1 non-evidence note, and no new JSON/RPC routes
- [x] Map HA entity registration to ADR-023 onramp framing: HA entities arrive as proposed FERROS material and must route through the S5 onramp consent surface before becoming canonical state; add consumer-awareness note to ADR-023 for S7; update bring-up worksheet with onramp-event note

## Next runway

- [x] Packet A: scaffolded a local-only `ferros-hub` binary skeleton and got `cargo check -p ferros-hub` green without claiming hardware evidence
- [x] Packet B: landed one local-only bridge seam plus one simulated local bridge artifact summary without touching the Home Assistant fork
- [x] Packet C: proved the bridge seam through targeted local allow, deny, and error/reporting checks under `cargo test -p ferros-hub bridge_` and emitted `.tmp/hub/simulated-local-bridge-artifact.json` as local-only, non-evidentiary output
- [x] Extended the local runway into a library-backed `ferros-hub` crate with `ferros-agents` registry composition, `ferros-core` policy composition over real `CapabilityGrant` input, and a typed local runtime summary without reopening G4 truth
- [x] Landed thin `ferros-hub summary | prove-bridge | deny-demo` proof commands over the reusable library seam while keeping the binary wrapper-only
- [x] Admitted bounded local artifact/report schemas into `harnesses/_constants.js` and the H1 contract validator without widening into partner-facing or frozen S2 contract work
- [x] Added a bounded typed local hub state snapshot seam under `.tmp/hub/` and kept the guardrails local-only and non-evidentiary
- [x] Threaded that snapshot seam through the typed hub runtime summary plus the existing `summary | prove-bridge` outputs so restart observation stays on the hub-owned local seam
- [x] Extended the existing read-only `/runway-summary(.json)` seam additively with optional `hubRestart` runway context and kept the localhost shell runway panel and inspector display-only on the same route
- [x] Proved the same restart-aware runway path through the same-origin localhost shell acceptance harness without opening a new route
- [x] Added the bounded local restart snapshot schema plus H1 validator parity coverage for banned summary wording and remote-looking text
- [x] Added `cargo xtask hub-runway` as a helper over the existing local hub proof seam; it now proves snapshot write/reload through the published hub summary seam and prints the exact `ferros-hub summary` output without reopening `ferros-hub` source files
- [x] Published the bounded local onramp proposal primitive in `ferros-data`, emitted `.tmp/hub/local-onramp-proposal.json` from the allowed bridge proof path, and kept it quarantined pending consent, local-only, and non-evidentiary
- [x] Extended the existing read-only `/runway-summary(.json)` seam additively with optional `hubOnrampProposal` context and kept the current localhost shell plus same-origin acceptance harness display-only on the same route
- [x] Admitted the bounded local onramp proposal schema into H1 validator coverage and extended `cargo xtask hub-runway` to validate/report the emitted proposal artifact on the same local rehearsal seam
- [x] Emitted the bounded `.tmp/hub/local-onramp-decision-receipt.json` artifact from the allowed bridge proof path, kept it local-only and non-evidentiary, and exposed it on the existing hub-owned `summary | prove-bridge` seam
- [x] Extended the existing read-only `/runway-summary(.json)` seam additively with optional `hubOnrampDecisionReceipt` context and kept node, shell, and same-origin acceptance-harness observation display-only on the same route
- [x] Admitted the bounded local decision rehearsal schema into H1 validator coverage and extended `cargo xtask hub-runway` to validate/report both the proposal and decision artifacts on the same local rehearsal seam
- [ ] Packet D: run the physical-device evidence checkpoint only after a human names the session window; keep it on the hardware track
- [x] All landed local executable runway packets to date stayed explicitly non-gate-closing: no G4 closure, no hardware proof, no Home Assistant dashboard proof, and no remote transport claim
- [ ] Turn the bridge runway contract into a DUT-side evidence checklist once a real hub/bridge path exists; keep the first proof at one registered bridge agent and one real entity
- [x] Route the landed seam brief to S3 and S4 and record which registration, inspection, policy, and restart surfaces are already sufficient versus still unpublished before any authoritative pairing flow, `ferros-hub` scaffold, or HA bridge plan is honest
- [ ] Keep the returned seam classifications as upstream dependency locks: S3 now publishes `AgentRegistry`, the local/read-first inspection surfaces, the aggregated read-only `agent.snapshot` wrapper, and the current localhost-only `agent.run` / `agent.stop` lifecycle slice as the honest runway boundary, while hub-facing lifecycle-wrapper and richer remote observation/control remain unpublished; S4 now publishes validated local profile/grant reload plus fixed reference-runtime state replay while durable hub restart/re-registration semantics remain unpublished before any authoritative pairing flow, durable hub runtime path, or real Home Assistant bridge proof is honest
- [ ] Keep the first executable packets local-only until the concrete broader S3/S4 remote or hub-facing lifecycle/write APIs exist; do not widen into remote control or launch-facing claims
- [ ] Recheck runway pairing notes against `docs/hub/reference-hardware.md`, `streams/S7-hub/CONTRACTS.md`, `STATUS.md`, and `docs/gates/G4.md` whenever S7 wording moves, S2 consumer-boundary rules change, or new S3/S4 seam details publish

## Later runway prep

- [x] Prepare the first evidence-capture worksheet for on-device sessions without claiming G4 evidence yet (`docs/hub/pack-b-bring-up-worksheet.md`)
- [x] Outline the operator notes needed for power-cycle validation and consent-deny observation
- [ ] Prepare the eventual `docs/hub/install.md` doc scope as a placeholder only, not runnable install instructions
- [ ] Queue the G4 evidence handoff points that must be satisfied before any launch claim

## Blocked / deferred in current G4 runway

- Physical-device and launch-facing proof remain out of scope in the active local-only G4 runway: the restart-aware `.tmp/hub` snapshot, the `.tmp/hub/local-onramp-proposal.json` proposal artifact, the `.tmp/hub/local-onramp-decision-receipt.json` decision artifact, `summary | prove-bridge`, `/runway-summary(.json)` `hubRestart` plus `hubOnrampProposal` plus `hubOnrampDecisionReceipt` context, shell/harness coverage, schema validation, and `cargo xtask hub-runway` remain non-evidentiary local proof only; no accept/reject consent flow, no canonical mutation, no G4 evidence claims, no Home Assistant dashboard proof, and no hardware-track execution without a named human session window.
- The first honest device-side G4 evidence is still blocked on moving the landed local-only `ferros-hub` scaffold and simulated bridge proof onto physical hardware and proving real Home Assistant behavior there.
- HA custom component work depends on the `Maangled/home-assistant` fork being available.

