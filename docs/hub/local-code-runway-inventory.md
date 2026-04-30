# FERROS Hub — Local Code Runway Inventory

> This document is the local-only code-runway handoff for the current S7 packet. It inventories the landed rehearsal surfaces already in repo, the artifact or read path each one uses, the quickest command that revalidates it, and the claim ceiling that must remain intact before any DUT or Home Assistant session begins.

---

## Current mode

- Everything listed here is local rehearsal inventory, not hardware evidence.
- The goal is deterministic handoff into future DUT work without reopening what the current local runway does and does not prove.
- `docs/hub/reference-hardware.md`, `LAUNCH.md`, and `docs/gates/G4.md` remain the launch-facing truth surfaces.

---

## Landed local runway inventory

| Surface | Owner and anchor | Artifact or read path | Primary validation | Non-claim boundary |
|---------|------------------|-----------------------|--------------------|--------------------|
| Simulated local bridge proof | S7: `crates/ferros-hub/src/ha_bridge.rs` | `.tmp/hub/simulated-local-bridge-artifact.json` | `cargo test -p ferros-hub bridge_`; `cargo run -p ferros-hub -- prove-bridge` | Simulated local bridge proof only. Not Home Assistant proof, not hardware evidence, not remote transport, and not canonical profile or grant mutation. |
| Restart snapshot contract | S7: `crates/ferros-hub/src/ha_bridge.rs`; `schemas/hub-local-state-snapshot.schema.json` | `.tmp/hub/local-hub-state-snapshot.json` | `cargo xtask hub-runway`; direct H1 run of `harnesses/ferros-contract-validator.html` | Local restart observation only. Not a published restart API, not power-cycle evidence, and not target-hardware durability proof. |
| Local onramp proposal artifact | S6 contract plus S7 emission: `crates/ferros-data/src/lib.rs`; `crates/ferros-hub/src/ha_bridge.rs` | `.tmp/hub/local-onramp-proposal.json` | `cargo test -p ferros-data onramp_proposal_`; `cargo test -p ferros-hub onramp_proposal_`; `cargo xtask hub-runway` | Pending-consent proposed material only. Not an accept or reject flow, not canonical state, not grant issuance, and not Home Assistant proof. |
| Local decision rehearsal receipt | S6 contract plus S7 emission: `crates/ferros-data/src/lib.rs`; `crates/ferros-hub/src/ha_bridge.rs` | `.tmp/hub/local-onramp-decision-receipt.json` | `cargo test -p ferros-data onramp_decision_`; `cargo test -p ferros-hub onramp_decision_`; `cargo xtask hub-runway` | Recorded decision rehearsal only. Not an executed consent event, not accept or reject transport, not canonical mutation, and not Home Assistant proof. |
| Hub-owned summary and CLI proof output | S7: `crates/ferros-hub/src/lib.rs`; `crates/ferros-hub/src/ha_bridge.rs` | `ferros-hub summary` and `ferros-hub prove-bridge` stdout | `cargo run -p ferros-hub -- summary`; `cargo run -p ferros-hub -- prove-bridge` | Local readout only. Not durable runtime evidence, not device proof, and not a gate-closing surface. |
| Read-only runway summary projection | S4 consuming S7: `crates/ferros-node/src/lib.rs` | `/runway-summary.json` with optional `hubRestart`, `hubOnrampProposal`, and `hubOnrampDecisionReceipt` | `cargo test -p ferros-node runway_summary`; `cargo test -p ferros-node onramp_`; `cargo test -p ferros-node onramp_decision_` | Additive read-only projection only. Not a new route family, not direct `.tmp/hub` file reads, not write control, and not remote transport. |
| Localhost shell runway display | S5 consuming S4: `site/agent-center-shell.html` | `http://127.0.0.1:4319/` runway route | `cargo test -p ferros-node shell_route_`; live local shell load at `http://127.0.0.1:4319/` | Display-only shell context only. No accept or reject control, no grant or profile mutation, no Home Assistant proof, and no hardware evidence. |
| H1 contract-validator coverage | S1 harness with S6 and S7 contract inputs: `harnesses/ferros-contract-validator.html`; `harnesses/_constants.js` | Local validator page plus embedded schemas for bridge artifact, restart snapshot, proposal artifact, and decision receipt | `powershell -NoProfile -ExecutionPolicy Bypass -File tools/generate-harness-constants.ps1`; direct file-based run of `harnesses/ferros-contract-validator.html` | Schema and payload-shape validation only. Not runtime evidence, not a partner contract, and not a gate claim. |
| H9 same-origin acceptance coverage | S5 harness with S4 host: `harnesses/localhost-shell-acceptance-harness.html`; `crates/ferros-node/src/lib.rs` | `http://127.0.0.1:4319/harnesses/localhost-shell-acceptance.html` | `cargo test -p ferros-node shell_route_serves_localhost_acceptance_harness`; live same-origin harness run at `http://127.0.0.1:4319/harnesses/localhost-shell-acceptance.html` | Black-box localhost proof only. Not browser-issued write proof, not remote browsing, and not physical-device evidence. |
| `xtask` proof-chain helper | S1 helper over S7-owned seams: `xtask/src/main.rs` | `cargo xtask hub-runway` output plus the `.tmp/hub/` artifacts above | `cargo check -p xtask`; `cargo xtask hub-runway` | Helper only. Not a second contract surface, not a second emission path, and not evidence that G4 is closed. |

---

## Minimum rerun order before any DUT session

1. Run `cargo xtask hub-runway` to refresh the local proof chain, snapshot reload, and proposal plus decision artifact checks through the published hub-owned seam.
2. Run the file-based H1 validator to confirm the embedded schema bundle still accepts the bounded local artifacts and rejects overclaiming or remote-looking payloads.
3. Run `cargo test -p ferros-node shell_route_serves_localhost_acceptance_harness` to keep the served H9 path alive before any manual localhost session.
4. If the local shell host is up on `http://127.0.0.1:4319/`, load the runway route and the served H9 harness once to confirm display-only shell observation still matches the read-only `/runway-summary.json` seam.

---

## Still not true

- No physical-device evidence exists yet.
- No real Home Assistant entity registration or dashboard proof exists yet.
- No real accept or reject transport exists yet.
- No canonical profile or capability-grant mutation is performed by these local surfaces.
- No remote transport or broader daemon/server deployment claim is established here.
- No target-hardware reboot or durable runtime evidence exists yet.
- No independent install evidence exists yet.
- G4 remains open.