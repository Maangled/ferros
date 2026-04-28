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

- [ ] Turn the bridge runway contract into a DUT-side evidence checklist once a real hub/bridge path exists; keep the first proof at one registered bridge agent and one real entity
- [x] Route the landed seam brief to S3 and S4 and record which registration, inspection, policy, and restart surfaces are already sufficient versus still unpublished before any authoritative pairing flow, `ferros-hub` scaffold, or HA bridge plan is honest
- [ ] Keep the returned seam classifications as upstream dependency locks: S3 now publishes only `AgentRegistry` plus local/read-first inspection surfaces, including the aggregated read-only `agent.snapshot` wrapper, as the honest runway boundary, while hub-facing lifecycle-wrapper and richer remote observation/control remain unpublished; S4 now publishes only validated local profile/grant reload plus fixed reference-runtime state replay while durable hub restart/re-registration semantics remain unpublished before any authoritative pairing flow, `ferros-hub` scaffold, or HA bridge plan is honest
- [ ] Keep that seam handoff docs-only and non-implementation until the concrete S3/S4 lifecycle or write APIs exist
- [ ] Recheck runway pairing notes against `docs/hub/reference-hardware.md`, `streams/S7-hub/CONTRACTS.md`, `STATUS.md`, and `docs/gates/G4.md` whenever S7 wording moves, S2 consumer-boundary rules change, or new S3/S4 seam details publish

## Later runway prep

- [x] Prepare the first evidence-capture worksheet for on-device sessions without claiming G4 evidence yet (`docs/hub/pack-b-bring-up-worksheet.md`)
- [x] Outline the operator notes needed for power-cycle validation and consent-deny observation
- [ ] Prepare the eventual `docs/hub/install.md` doc scope as a placeholder only, not runnable install instructions
- [ ] Queue the G4 evidence handoff points that must be satisfied before any launch claim

## Blocked / deferred in current G4 runway

- Runtime and hub implementation work remain out of scope in the active G4 runway: no `crates/ferros-hub/` scaffold, no pairing implementation, no HA bridge, no runtime wiring, and no G4 evidence claims in this pass.
- The first honest G4 evidence is blocked on a real `ferros-hub` binary and HA bridge existing on physical hardware.
- HA custom component work depends on the `Maangled/home-assistant` fork being available.

