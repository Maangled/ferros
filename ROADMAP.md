# FERROS — Roadmap

> Stream-first. Gate-driven. Launch = hub on real hardware.

---

## Model

FERROS is developed as **eight parallel streams**, each independently assignable to an agent or contributor. Streams share contracts (traits + JSON Schemas in `schemas/`) but do not block each other on timelines. Progress gates (**G1–G4**) mark the points where stream outputs must converge before the next phase of work can proceed.

This is not a Gantt chart. There are no fixed calendar dates. A gate opens when its entry criteria are satisfied — not when a sprint ends.

---

## The Eight Streams

| # | Stream | Depends on | Blocks |
|---|--------|-----------|--------|
| S1 | **Foundation** — workspace, CI, licensing, tooling | — | all |
| S2 | **Profile & Identity** — Ed25519 identity, capability grants, consent manifests | S1 | S3, S4, S7, S8 |
| S3 | **Agent Center** — `Agent` trait, registry, spawn/stop, IPC bus | S2 | S4 UX hooks, contributor onboarding |
| S4 | **Runtime / OS Core** — consent bus, executor, policy engine, `ferros-node` | S1 | S3 runtime hooks, S7 |
| S5 | **UX** — site → agent center shell → WASM demos | S1; contract stubs from S3/S4 | launch-readiness |
| S6 | **Ecosystem Harvest** — extract/rewrite from `sheetgen-rust`, `botgen-rust`, `workpace-rust` | S1, S4 traits | S3, S7 |
| S7 | **Smart-Home Hub** — `ferros-hub`, Home Assistant integration, edge targets | S4, S6 | launch |
| S8 | **Docs / Governance / Onboarding** — living docs, ADRs, contributor guide | S1 | contributor scale-up |

See individual stream directories under `streams/` for mission, scope, backlog, and contracts.

---

## Gates

| Gate | Condition | Unlocks |
|------|-----------|---------|
| **G1** | S1 green — `cargo build && cargo test` pass on Linux/macOS/Windows; CI running | S2–S8 in earnest |
| **G2** | S2 profile v0 frozen — types, schema, round-trip proven | S3 active development; S7 pairing design |
| **G3** | S3 + S4 minimal demo — agent-center-on-runtime with two reference agents | S5 Phase B (local web shell); S7 implementation |
| **G4** | S7 hub on Pi — single-binary `ferros-hub` runs on aarch64, pairs with profile, registers one HA entity, survives reboot, enforces consent | **Launch** |

Gate documents live in `docs/gates/`.

---

## MVP vs. Launch-target vs. Post-launch

### MVP — FERROS is real (not public)

The project is real when the following work:

- S1 foundation done.
- S2 profile v0: create, sign, verify, revoke.
- S4 runtime: capability bus with deny-by-default consent enforcement.
- S3 agent center CLI: register and run two reference agents (`echo`, `timer`).
- Conformance harness validates one reference implementation.

Tagged: `v0.1.0-rc`. Nothing public; link shared privately.

### Launch-target — `v0.2.0`

Launch is **not** a website going live, a crates.io publish, or a PR merge. See `LAUNCH.md` for the precise definition.

The launch-target scope:

- `ferros-hub` running on a Pi or home server.
- Home Assistant fork integration: FERROS-managed agents appear as HA entities with consent gates.
- Reboot-safe persistent profile and grant storage.
- Reference hardware doc + install script.
- Private beta: ≤5 home installs before any wider mention.

Tagged: `v0.2.0`.

### Post-launch — `v0.3.0+`

- Multi-node / distributed case.
- Embedded target (`thumbv7em-none-eabi`) for AI-edge scenarios.
- Agent marketplace and community manifests — only if there is a community; do not pre-build.
- `crates.io` publication of stable primitives.

---

## Milestones (stream-pull model)

Milestones are delivery tags pulled from completed stream work, not deadlines pushed from a calendar.

| Tag | Condition |
|-----|-----------|
| `v0.0.1-foundation` | S1 done |
| `v0.0.2-profile` | S2 profile v0 frozen |
| `v0.0.3-runtime` | S4 consent bus runnable |
| `v0.0.4-agents` | S3 agent center CLI with two reference agents |
| `v0.0.5-harvest` | S6 harvest ADRs merged for all three source repos |
| `v0.1.0-rc` | MVP: S1+S2+S3+S4 functional; conformance harness green |
| `v0.1.0` | Agent center local web shell (S5 Phase B) |
| `v0.2.0-rc` | `ferros-hub` pairing demo on x86_64 |
| `v0.2.0` | Hub on aarch64/Pi with HA integration, reboot-safe, consent enforced |

---

## Coordination rules

- **Contract-first:** every cross-stream interaction is a trait in `ferros-core` or a schema in `schemas/`. Streams edit implementations, not contracts, unless they own the contract stream.
- **One stream = one directory subtree.** See `CODEOWNERS` for ownership mapping.
- **Progress files:** each stream maintains `streams/SN-name/PROGRESS.md`. Append a dated entry per session.
- **Schema versioning is sacred:** bump `schemas/*.vN.json`; never mutate a frozen version in place.
- **Integration branch weekly:** `integration/week-N` merges all stream tips, runs full CI, resolves drift before it accumulates.
