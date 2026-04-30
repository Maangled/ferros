# S7 Smart-Home Hub — Contracts

---

## Contracts owned or staged by S7

S7 owns deployment-facing hub surfaces. It does not get to redefine S2 profile or capability-grant semantics from stream-local planning docs.

| Contract | Type | Location | Status |
|----------|------|----------|--------|
| `HubConfig` type (device profile, port, HA endpoint) | Rust type | `crates/ferros-hub/` | ⬜ Not yet created |
| HA bridge agent manifest | `AgentManifest` JSON | `crates/ferros-hub/src/ha_bridge.rs` | 🟨 Landed as a manifest-backed local bridge registration seam for the simulated bridge path; real Home Assistant packaging remains deferred |
| `LocalHubRuntimeSummary` + runway observation | Rust types + local proof output | `crates/ferros-hub/src/ha_bridge.rs`, `crates/ferros-hub/src/lib.rs`, `crates/ferros-node/src/lib.rs` | 🟨 Landed as a local-only runway seam; `summary`, `prove-bridge`, and `/runway-summary(.json)` stay non-evidentiary while carrying bounded restart, proposal, and decision rehearsal context |
| Local hub state snapshot contract | Local-only persisted snapshot + JSON schema | `.tmp/hub/local-hub-state-snapshot.json`, `schemas/hub-local-state-snapshot.schema.json` | 🟨 Landed with bounded guardrails and H1 validator coverage |
| Local onramp proposal contract | Rust types + local JSON artifact + local-only schema | `crates/ferros-data/src/lib.rs`, `.tmp/hub/local-onramp-proposal.json`, `schemas/onramp-proposal.schema.json` | 🟨 Landed as local-only proposed material; quarantined pending consent |
| Local onramp decision receipt contract | Rust types + local JSON artifact + local-only schema | `crates/ferros-data/src/lib.rs`, `.tmp/hub/local-onramp-decision-receipt.json`, `schemas/onramp-decision-rehearsal.schema.json` | 🟨 Landed as a local-only recorded decision rehearsal linked to proposed material; non-authoritative and non-canonical |
| Pairing constraints and open questions | Stream-local planning notes | `streams/S7-hub/*.md` | 🟨 Runway only; authoritative protocol intentionally deferred |
| Reference hardware runway and evidence template | Doc | `docs/hub/reference-hardware.md` | 🟨 In progress |

---

## Contracts consumed by S7

| Contract | Source | Purpose |
|----------|--------|---------|
| `ProfileId`, `CapabilityGrant` | S2 | Pairing flow issues grants; profile binds to device |
| `Agent` trait, `AgentRegistry` | S3 | Hub registers HA-bridge agents through S3 |
| `ferros-runtime` executor + bus | S4 | Hub wraps the runtime as its execution environment |
| `ferros-data` primitives | S6 | Optional: data generation for hub agents |

---

## Landed local restart-aware and onramp runway seams

This section is local-only and non-gate-closing. It records the landed restart-aware and local onramp rehearsal packets without promoting them to broader published hub contracts.

| Surface | What is now landed | What stays open |
|---------|--------------------|-----------------|
| persisted restart state | `LocalHubStateSnapshot` writes under `.tmp/hub/`, and the bounded schema-backed snapshot contract now has H1 validator coverage, including parity checks against hardware/proof/launch wording and remote-looking summary text. | Not a partner-facing schema, not a public restart API, and not device or power-cycle evidence. |
| local proposed-material emission | `LocalOnrampProposal` now writes one bounded `.tmp/hub/local-onramp-proposal.json` artifact from the allowed bridge proof path, and the schema-backed proposal contract now has H1 validator coverage against remote-looking text, hardware/proof/launch wording, and canonical/grant wording. | Not a public ingress contract, not an accept/reject flow, not canonical profile or grant mutation, and not Home Assistant proof. |
| local decision rehearsal emission | `LocalOnrampDecisionReceipt` now writes one bounded `.tmp/hub/local-onramp-decision-receipt.json` artifact from the allowed bridge proof path, and the schema-backed decision contract now has H1 validator coverage against remote-looking text, whitespace-only and leading-`//` text, hardware/proof/launch wording, and canonical/grant wording. | Not an accept/reject transport, not an executed consent event, not canonical profile or grant mutation, and not Home Assistant proof. |
| local hub proof outputs | `LocalHubRuntimeSummary` plus `summary` and `prove-bridge` now report bounded `fresh-start | reloaded | unavailable` restart context on the existing local hub seam. | Not durable runtime evidence and not a G4 or D1 claim. |
| downstream runway observation | The existing read-only `/runway-summary(.json)` seam now additively carries optional `hubRestart`, `hubOnrampProposal`, and `hubOnrampDecisionReceipt` runway context, and the current localhost shell plus same-origin acceptance harness observe all three on the existing shell route as display-only context. | Not a new route, not browser-issued hub control, not accept/reject transport, not remote transport, and not Home Assistant integration proof. |
| local helper alignment | `cargo xtask hub-runway` now proves snapshot write/reload through the hub-owned summary seam, validates the emitted proposal and decision artifacts, and prints compact proposal plus decision report lines after the exact `ferros-hub summary` output. | Not a second contract surface and not evidence that the cross-stream contracts overview is fully synchronized. |

- `docs/contracts/CONTRACTS-OVERVIEW.md` now reflects the shared proposal-plus-decision runway split at cross-stream level, while this stream-local contract record remains the authoritative S7 truth surface for the bounded restart, proposal, and decision rehearsal details that sit beneath that shared overview.

---

## Exact upstream seams S7 still needs named

This section records the concrete S3 and S4 surfaces S7 can honestly point at today. It does not turn them into a finished hub contract, and it does not authorize `crates/ferros-hub/` scaffolding.

| Upstream seam | Exact current surface | Why S7 cares | Why it is not enough yet |
|---------------|-----------------------|--------------|--------------------------|
| S3 registration seam | `AgentRegistry::register`, `AgentRegistry::deregister`, `AgentRegistry::list`, `AgentRegistry::describe` | The first bridge slice must register as a visible FERROS agent, not as hidden stream-local state. | S3 has not yet published a hub-owned lifecycle wrapper or bridge-specific registration contract. |
| S3 inspection seam | local `ferros agent list`, `ferros agent describe`, `ferros agent logs`; read-first `agent.list`, `agent.describe`, `grant.list`, `denyLog.list` | S7 needs an honest local observation path for bridge presence, grant reads, and deny visibility. | S3 still treats transport serving, remote observation, and privileged writes as open work. |
| S4 policy seam | `CapabilityRequest`, `CapabilityGrantView`, `PolicyEngine::evaluate`, `DenyByDefaultPolicy`, `PolicyDecision`, `PolicyDenialReason` | S7 needs a named policy boundary for grant checks and deny reasons instead of inventing a hub-local grant model. | S4 has not yet published the final hub-facing wrapper around those policy types or their operator-visible propagation. |
| S4 restart seam | nearest current reload helpers are `runtime_with_state(state_path)`, `CliState::load(state_path)`, and `LocalProfileStore::load_local_profile(path)` | S7 needs a real restart and re-registration contract before it can name a reboot-safe pairing flow honestly. | These are still node-local reload helpers, not a stable S4-owned hub restart API. |

---

## Pairing boundary note

The future hub pairing flow must end in signed, reboot-survivable, revocable capability grants because launch requires those properties in practice. That does not mean the exact handshake should be frozen in these docs yet. Until `ferros-hub` exists and the S2/S3/S4 implementation seams are concrete, S7 should track constraints, storage needs, and validation expectations rather than pretend the protocol is already settled.

---

## First bring-up contract (x86_64-first)

S7's first honest bring-up target is the Pack B `x86_64` lane from `docs/hub/reference-hardware.md`. This is a bring-up preference, not a launch redefinition: `aarch64` remains a launch-valid target, but the first integration pass should optimize for observability and rollback.

| G4 evidence item | Upstream seam that must exist | S7-owned proof point for first bring-up |
|------------------|-------------------------------|-----------------------------------------|
| `ferros-hub` cross-compiles for a launch-valid Linux target | S4 host/runtime packaging seam plus the eventual `crates/ferros-hub/` wrapper | Produce a successful `x86_64-unknown-linux-gnu` build on the chosen Pack B device class before asking for Pi-specific proof |
| Physical device run | S4 host seam must be stable enough to wrap; S7 owns the deployment wrapper and session notes | Run the first hub session on a real Pack B home-server-class device reachable by SSH, not on a laptop or VM |
| Device profile persists after restart | S2 CLI surface plus the eventual S4/S7 storage boundary | Create the device profile on the DUT, record the persistent storage path, and prove the same profile loads after restart |
| `ferros agent list` shows the HA bridge agent | S3 registry/list surface plus the future HA bridge manifest | Record the first run where the HA bridge agent is registered on the DUT and visible through the hub-owned agent list path |
| Real HA entity appears in the dashboard | External HA fork plus S3 registration and S4 runtime message flow | Use a separate HA host and record one real entity synchronized through the bridge, not a mocked entity |
| Consent deny is logged and operator-visible | S4 deny logging plus S3 log access plus HA-facing visibility | Capture one ungranted request that is denied, logged, and surfaced either in HA UI or `ferros agent logs` |
| Hub survives full power cycle | S2 persisted profile/grant state plus S4 runtime re-registration seam | Perform a DUT-only power cut on the chosen Pack B lane and prove profile reload, agent re-registration, and HA-side recovery |
| Independent install confirmed | All seams above plus S7 operator notes and install guidance | Repeat the validated bring-up contract on a second non-primary home setup before claiming launch readiness |

### Guardrails for this contract

- This contract does not authorize `crates/ferros-hub/` scaffolding by itself.
- This contract does not freeze pairing order or grant-issuance ceremony.
- If any G4 row above depends on a missing S2 field, a missing S3 contract, or a missing S4 host/runtime seam, S7 must escalate that upstream dependency instead of inventing a stream-local workaround.

---

## First Home Assistant bridge runway contract

This section is runway-only. It makes the first bridge slice concrete enough to plan and queue against, but it does not authorize `crates/ferros-hub/` scaffolding, publish a transport, or freeze the reconnect mechanism.

| Minimum assumption | Runway contract |
|--------------------|-----------------|
| FERROS-side registration unit | The first Home Assistant bridge slice is one bridge agent registered through the S3 `AgentRegistry` and visible through the local `ferros agent list` and `ferros agent describe` path on the device under test. S7 may plan around one `AgentManifest`, not a multi-agent topology. |
| Capability boundary | The bridge agent manifest declares `CapabilityRequirement` values, while execution stays gated by S2 `CapabilityGrant` state and S4 runtime policy at runtime. S7 can document that boundary, but it must not redefine grant semantics or invent a stream-local bypass around deny logging. |
| Minimum first evidence scope | One real Home Assistant entity synchronized through the bridge is enough for the first honest evidence slice. Mocked entities, replay fixtures, and UI-only demos do not satisfy this runway contract. |
| Deny visibility expectation | A denied bridge action must be attributable from the device-under-test side through runtime deny logging and remain operator-visible through the local FERROS inspection path, including `ferros agent logs`. A silent missing entity or a dashboard-only inference is not sufficient. |
| Restart and recovery posture | Home Assistant restarts or temporary sync interruptions may pause bridge traffic, but they must not wipe hub-owned `ProfileId`, `CapabilityGrant`, or bridge-registration state on the device under test. The exact reconnect choreography stays open in this repo until the real bridge exists. |
| External fork boundary | `Maangled/home-assistant` remains a separate repository. This repo tracks only the FERROS-side integration contract and evidence boundary; it does not freeze HA component internals, release cadence, or a final pairing protocol. |

- The provisional operator path remains local to FERROS: `ferros agent run`, `ferros agent stop`, and `ferros agent logs` are the control and inspection verbs S7 can assume for first bridge evidence once implementation exists.
- If the first bridge slice needs more than one registered bridge agent, more than one real entity, or a repo-local transport contract to sound plausible, the scope is too wide for this runway pass.

---

## Notes on the HA fork

The `Maangled/home-assistant` fork is a separate repository. S7 adds a FERROS custom component to that fork that bridges the HA device/entity model to the FERROS agent center. Changes to the HA fork are tracked in that repo; S7 documents the integration interface here.
