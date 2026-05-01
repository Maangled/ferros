# ADR-025 — Dual-root hardware runway and ignition-lane architecture

**Status:** Proposed  
**Date:** 2026-04-29  
**Stream:** Cross-cutting / S1 / S4 / S7 / S8  
**Deciders:** Maangled  
**Domain tags:** architecture / hardware / runtime / governance / orchestration / research / launch  
**Primary evidence basis:** Research or precedent proof, with operational proof required before Accepted

_See [ADR-022-decision-program-governance.md](./ADR-022-decision-program-governance.md), [_INDEX.md](./_INDEX.md), and [../../DOCTRINE.md](../../DOCTRINE.md). Cross-reference ADR-023 (onramp policy) for the consent-invariant that any hardware-bridge path must satisfy, and ADR-024 (ledger substrate) for the non-chain signed-ledger posture that hardware evidence records must respect._

> **Note:** This ADR documents a proposed architecture, not a binding commitment. S9 (Ignition lane) is provisional. Implementation work on hardware roots should be scoped to research notes and planning documents until the research guardrail checks listed below are satisfied and this ADR reaches Accepted status through the process defined in ADR-022.

---

## Context

FERROS is moving from a local-only software runway toward real hardware evidence, but the project must avoid collapsing two different goals into one path. The fastest product path needs to run FERROS on available hardware using practical host operating systems, vendor tooling, Linux services, and normal deployment mechanics. The FERROS-native path needs to work backward toward the long-term operating-system vision: Rust-native boot, kernel, memory, process supervision, storage, networking, display/UI, identity, permissions, and hardware-specific runtime primitives.

The project now has multiple hardware families available or expected: an `x86_64` Linux server, Raspberry Pi 4B boards, Jetson Orin Nano, and ESP32-class embedded devices. Each hardware family needs a way to advance quickly without blocking the deeper FERROS-native research path. At the same time, every target must stay aligned with the existing stream model and avoid overclaiming D1, G4, Home Assistant integration, physical-device evidence, or FERROS-native OS status before those facts are proven.

The existing S1–S8 stream system is already useful as a hardware runway map:

- **S1** — foundation, boot, toolchain, host baseline, bootloader/kernel research
- **S2** — identity, board profile, grants, local trust boundary
- **S3** — agent center, local agents, registry, lifecycle, logs
- **S4** — runtime / OS core, process model, bus, storage path, networking assumptions
- **S5** — UX, dashboard, display, kiosk/operator surface
- **S6** — storage/data, cards/decks, artifacts, snapshots, findings, backup/restore
- **S7** — hub, Home Assistant, physical-device bridge, real-world evidence
- **S8** — docs, governance, claims, findings, truth-sync

A ninth lane is now proposed as a recursive "ignition" lane. S9 is not a replacement for S1–S8. It is the lane that reloads, rehydrates, and retargets the other lanes after a product path or FERROS-native path learns something important.

---

## Decision

FERROS will adopt a **dual-root hardware runway architecture** for each hardware family: every product target gets both a **Fastest** root and a **FERROS** root, each organized by S1–S8 hardware lanes, with a provisional S9 **Ignition** lane used to reload, retarget, and recursively translate discoveries between the two roots.

Each hardware family is represented as an orchestrated median with two directional runway sides:

```text
987654321 | hardware-family | 123456789
```

The right side represents the forward product runway, usually the **Fastest** path:

```text
S1 → S2 → S3 → S4 → S5 → S6 → S7 → S8 → S9
```

The left side represents the reverse-engineering / FERROS-native runway, usually the **FERROS** path:

```text
S9 → S8 → S7 → S6 → S5 → S4 → S3 → S2 → S1
```

The primary hardware-root hierarchy is:

```text
Ferros Core Agent
  ├─ x86_64
  │   ├─ Fastest
  │   │   ├─ S1
  │   │   ├─ S2
  │   │   ├─ ...
  │   │   └─ S9
  │   └─ FERROS
  │       ├─ S1
  │       ├─ S2
  │       ├─ ...
  │       └─ S9
  ├─ Raspberry Pi 4B
  │   ├─ Fastest
  │   └─ FERROS
  ├─ Jetson Orin Nano
  │   ├─ Fastest
  │   └─ FERROS
  └─ ESP32
      ├─ Fastest
      └─ FERROS
```

The **Fastest** root is allowed to use practical tools to get product behavior running quickly.

The **FERROS** root is allowed to research, design, and eventually implement the native FERROS path for that hardware family.

Both roots may read each other. Writes are restricted by root and lane so agents can safely coordinate without overwriting another target's plan.

---

## Rationale

FERROS needs a hardware architecture that lets the project move quickly without abandoning the deeper OS vision.

A single hardware plan per board is too ambiguous. It forces every decision to answer two incompatible questions at once:

1. What is the fastest way to make FERROS useful on this device?
2. What is the FERROS-native way to own the boot/runtime/storage/display/networking stack?

The dual-root structure separates those questions while keeping them synchronized.

The **Fastest** root proves product reality:

* Can it run?
* Can it serve the dashboard?
* Can it emit local artifacts?
* Can it coordinate agents?
* Can it observe Home Assistant?
* Can it survive reboot?
* Can a non-programmer eventually use it?

The **FERROS** root translates product reality into OS architecture:

* What should FERROS eventually boot?
* What runtime primitives are actually needed?
* Which Linux/vendor services must FERROS replace?
* Which hardware drivers matter first?
* Which storage and identity primitives must exist below the app layer?
* Which display and UI assumptions must become native?

The S1–S8 lane structure keeps the model aligned with the current stream system. The S9 Ignition lane adds the missing recursive layer: after any root learns something, S9 decides whether to reload the other lanes, spawn a board-specific plan, or produce a target artifact such as:

* `x86_64` — server runway reload after local-agent integration
* `raspi-4b` — Pi appliance or Pi native boot experiment plan
* `jetson-orin-nano` — Orion home-hub readiness plan
* `esp32` — FERROS hub lightswitch plan

---

## Highway model

The hardware-family node acts as the orchestrating median:

```text
FERROS-native reverse runway      median       Fastest product runway

S9 S8 S7 S6 S5 S4 S3 S2 S1    | x86_64 |    S1 S2 S3 S4 S5 S6 S7 S8 S9
S9 S8 S7 S6 S5 S4 S3 S2 S1    | Pi 4B  |    S1 S2 S3 S4 S5 S6 S7 S8 S9
S9 S8 S7 S6 S5 S4 S3 S2 S1    | Jetson |    S1 S2 S3 S4 S5 S6 S7 S8 S9
S9 S8 S7 S6 S5 S4 S3 S2 S1    | ESP32  |    S1 S2 S3 S4 S5 S6 S7 S8 S9
```

The product runway usually moves from S1 toward S9:

```text
foundation → identity → agents → runtime → UX → storage → hub/device → evidence → ignition
```

The FERROS-native runway usually moves from S9 back toward S1:

```text
ignition → evidence → hub/device → storage → UX → runtime → agents → identity → boot/kernel
```

This lets FERROS work backward from observed product behavior into operating-system architecture.

---

## S9 Ignition lane

S9 is a proposed service lane, not a normal feature lane.

S9 exists to answer:

1. What changed?
2. Which lanes need to reload?
3. Which root should consume the discovery?
4. Is this a product-path discovery, a FERROS-native discovery, or both?
5. Does this produce a new board-specific plan?
6. Does this alter the allowed claims for the hardware family?

S9 may create or update:

* lane reload notes
* handoff documents
* board-specific ignition plans
* findings routing
* target-specific product concepts
* target-specific FERROS-native research prompts

Examples:

### ESP32

S9 may turn product/hub discoveries into:

```text
ESP32 FERROS hub lightswitch plan
```

This might reload:

* S1 — no_std target/toolchain
* S2 — tiny device identity
* S3 — peripheral agent registration
* S4 — embedded event/runtime primitive
* S5 — physical button/light UX
* S6 — event card/deck artifact
* S7 — Home Assistant switch bridge
* S8 — findings and claim boundary

### Jetson Orin Nano

S9 may turn server/Pi learnings into:

```text
Orion home hub first-user appliance plan
```

This might reload:

* S1 — vendor image baseline
* S3 — local agent workload classes
* S4 — GPU/AI runtime boundary
* S5 — first-user dashboard
* S7 — home hub evidence
* S8 — first-user safety claims

### x86_64

S9 may turn local-agent server progress into:

```text
x86_64 integration runway reload
```

This might reload:

* S3 — agent execution model
* S4 — process supervisor requirements
* S6 — local artifact storage
* S7 — bridge/coordinator role
* S8 — evidence and logs

### Raspberry Pi 4B

S9 may split the two available boards:

```text
Pi 4B Fastest = Linux appliance path
Pi 4B FERROS = immediate native boot/runtime tinkering path
```

---

## Research guardrail

The S9 lane and the bidirectional runway model are **Proposed**, not fully frozen.

Before this ADR is promoted to Accepted, the project must produce research notes or equivalent plan documents that check whether the model should use:

* exactly S1–S8 plus S9,
* fewer lanes for constrained devices,
* more lanes for complex hardware,
* target-specific collapsed lanes,
* or a different service-lane model.

Research must verify that the runway structure does not drift from the FERROS blueprint. Agents may explore whether the model should do more or less, but must not silently mutate the blueprint.

Required research checks:

1. **Lane sufficiency check** — Are S1–S8 enough to describe each hardware family?
2. **S9 necessity check** — Does Ignition produce useful reload/handoff behavior, or can S8 governance cover it?
3. **Fastest/FERROS separation check** — Does the dual-root split prevent confusion, or does it duplicate too much work?
4. **Embedded-device compression check** — Does ESP32 need all lanes, or should some lanes be collapsed?
5. **Server-control-plane check** — Does `x86_64/Fastest` become a valid control plane for Pi/ESP/Jetson findings?
6. **Claim-boundary check** — Does the model prevent accidental G4, Home Assistant, physical-device, or FERROS-native OS overclaims?
7. **Agent-permission check** — Can agents safely read across roots while writing only to assigned roots/lanes?

Until those checks are complete, S9 is a proposed service lane and must be marked as provisional in hardware-root docs.

---

## Agent permission model

Agents follow read-wide, write-narrow rules.

### Ferros Core Agent

```text
read: all hardware roots
write: root standards, matrix, cross-root truth sync, ADR/research surfaces only
```

### Hardware-family agent

Example: `x86_64 Agent`

```text
read: all hardware roots
write: docs/hardware/roots/x86_64/** only
```

### Root agent

Example: `x86_64 Fastest Agent`

```text
read: all hardware roots
write: docs/hardware/roots/x86_64/fastest/** only
```

### Lane agent

Example: `x86_64 Fastest S4 Agent`

```text
read: all hardware roots
write: docs/hardware/roots/x86_64/fastest/S4-runtime.md only
```

Agents may synchronize construction by reading across roots, but may not write outside their assigned root/lane.

---

## Directory standard

The proposed hardware-root directory shape is:

```text
docs/hardware/roots/
  README.md
  BOARD-ROOT-STANDARD.md
  PERMISSIONS.md
  MATRIX.md
  RESEARCH-GUARDRAILS.md

  x86_64/
    README.md
    fastest/
      README.md
      S1-foundation.md
      S2-identity.md
      S3-agent-center.md
      S4-runtime.md
      S5-ux.md
      S6-storage-data.md
      S7-hub-ha.md
      S8-evidence.md
      S9-ignition.md
      HANDOFFS.md
    ferros/
      README.md
      S1-foundation.md
      S2-identity.md
      S3-agent-center.md
      S4-runtime.md
      S5-ux.md
      S6-storage-data.md
      S7-hub-ha.md
      S8-evidence.md
      S9-ignition.md
      HANDOFFS.md

  raspi-4b/
    fastest/
      ...
    ferros/
      ...

  jetson-orin-nano/
    fastest/
      ...
    ferros/
      HOLD.md
      ...

  esp32/
    fastest/
      ...
    ferros/
      ...
```

---

## Root meanings

### Fastest root

The Fastest root is allowed to use existing platforms and vendor support to prove product behavior quickly.

Allowed:

* Linux host OS
* vendor images
* systemd or equivalent service managers
* normal networking
* normal filesystems
* standard Rust binaries
* practical setup scripts
* local dashboards
* local agents
* logging and findings collection

Forbidden unless proven:

* FERROS-native boot claim
* kernel claim
* physical-device claim for another board
* Home Assistant proof without real HA evidence
* G4 closure
* launch claim

### FERROS root

The FERROS root works backward toward the native OS path.

Allowed:

* bootloader research
* kernel and no_std research
* board memory map notes
* display driver exploration
* storage primitive design
* network primitive design
* process/runtime primitive design
* identity and permission integration below the app layer

Forbidden unless proven:

* product readiness claim
* user-facing appliance claim
* D1 closure
* G4 closure
* "runs FERROS OS" claim before actual boot evidence exists

---

## Hardware-family initial posture

### x86_64

The `x86_64/Fastest` root is the primary integration runway and likely control plane.

It may run:

* `ferros-node`
* `ferros-hub`
* local agents
* local onramp proposal rehearsal
* local dashboard
* findings collection for downstream board tests

The `x86_64/FERROS` root owns UEFI, bootloader, and kernel-path research for normal PC/server hardware.

### Raspberry Pi 4B

Two Pi boards should be split:

* one Pi for `raspi-4b/Fastest`
* one Pi for `raspi-4b/FERROS`

The Fastest Pi may use Linux to prove appliance behavior.

The FERROS Pi may be used immediately for native boot/runtime tinkering, with no requirement to preserve product stability.

### Jetson Orin Nano

The Jetson is protected as a later first-user appliance candidate.

Its Fastest root may be planned but should avoid mutation until server and Pi roots are stable.

Its FERROS root remains research-heavy until the project has stronger evidence about GPU/AI/runtime needs.

### ESP32

ESP32 is the embedded edge/peripheral proving ground.

The Fastest root may use the simplest practical embedded path to prove a lightswitch/sensor/control-node concept.

The FERROS root researches a no_std, capability-limited embedded runtime and tiny device identity model.

---

## Handoff rules

Each root must maintain `HANDOFFS.md`.

### Fastest → FERROS handoff

Every Fastest discovery should answer:

* What worked?
* Which host/vendor/Linux feature did it depend on?
* Which FERROS-native subsystem would eventually need to replace that dependency?
* What is the smallest Rust-native primitive implied by the discovery?

Example:

```text
Fastest discovery:
systemd restarts ferros-hub after reboot.

FERROS implication:
S4 needs a native supervisor primitive:
- start process
- observe state
- restart on failure
- persist bounded local snapshot
- report state to dashboard
```

### FERROS → Fastest handoff

Every FERROS discovery should answer:

* What constraint did the native path discover?
* Should the Fastest path avoid depending too strongly on a host feature?
* Does a boundary need to be added now to prevent future lock-in?

Example:

```text
FERROS discovery:
Native boot path cannot assume a writable root filesystem early.

Fastest implication:
Move proposal/snapshot path assumptions behind a storage abstraction before appliance image work.
```

---

## Options considered

| Option | Summary | Reason not chosen |
|--------|---------|-------------------|
| Single hardware root per target | Each board has one plan | Rejected because it mixes fastest product work with FERROS-native research and causes overclaim risk |
| Separate product and OS projects | Fastest work and FERROS-native work live in unrelated plans | Rejected because discoveries would drift apart and agents would lose the shared blueprint |
| Dual-root model with S1–S8 only | Each hardware family has Fastest and FERROS roots using existing lanes | Viable, but misses the recursive reload/handoff behavior now emerging from planning |
| Dual-root model with provisional S9 Ignition | Fastest and FERROS roots use S1–S8 plus S9 service lane | Chosen as Proposed; requires research validation before Accepted |
| Board-specific lane counts only | Each hardware target invents its own structure | Rejected for now because it weakens orchestration and agent parallelism |

---

## Consequences

### Positive

* Fastest product work can move immediately without blocking FERROS-native research.
* FERROS-native research can begin immediately without risking product runway stability.
* Agents can parallelize safely by hardware family, root, and lane.
* The project gains a standard way to compare x86_64, Pi, Jetson, and ESP32 work.
* Server-hosted integration and board-target evidence become separate claim classes.
* S9 creates an explicit place for recursive reloads, handoffs, and target-specific ignition plans.
* Hardware planning becomes compatible with the existing S1–S8 stream vocabulary.

### Negative / trade-offs

* The model adds structure before all hardware facts are known.
* S9 may duplicate S8 governance unless research proves its separate value.
* ESP32 and other constrained targets may not need all lanes.
* Agents may over-scaffold unless write boundaries are enforced.
* The dual-root model can create duplicated docs if handoff discipline is weak.

---

## Compliance

This ADR remains valid only if the following stay true:

* Fastest and FERROS roots remain clearly separated.
* Agents can read across roots but write only to assigned root/lane surfaces.
* Hardware-root documents distinguish planning, rehearsal, physical evidence, and gate evidence.
* S9 remains provisional until research validates it.
* No root may claim D1, G4, Home Assistant proof, physical-device evidence, or FERROS-native OS status without explicit evidence.
* Fastest work must feed FERROS-native research through handoffs.
* FERROS-native research must feed Fastest boundary decisions through handoffs.

Revisit this ADR if:

* S9 proves redundant with S8.
* Hardware roots become too heavy for constrained targets.
* The board-root model blocks rather than accelerates evidence collection.
* A different lane count becomes clearly superior.
* A real physical-device evidence packet reveals a missing lane or wrong boundary.

---

## Implementation Evidence

Current evidence is mostly architectural and operational, not final hardware proof.

* Existing hardware queue already separates hardware-track planning and findings from normal code-track work.
* Local onramp rehearsal now produces proposed material without canonical mutation.
* `ferros-hub`, `ferros-node`, and local runway summaries provide the first software shape for board-root rehearsal.
* Real physical-device evidence remains absent and must not be claimed through this ADR.

---

## Deferred Scope or Open Research

Deferred until research notes or hardware-root plans land:

* Final acceptance of S9 as a permanent lane.
* Whether S1–S8 plus S9 is too much for ESP32-class targets.
* Whether some hardware families need collapsed or expanded lanes.
* The exact x86_64 server control-plane runbook.
* The exact Raspberry Pi split between Linux appliance path and FERROS-native tinkering path.
* Jetson first-user hold/release criteria.
* ESP32 lightswitch / peripheral-agent proof plan.
* UEFI bootloader strategy for x86_64.
* Raspberry Pi native boot strategy.
* How board-root findings become D1 or G4 evidence.

---

## References

* `docs/orchestration/HARDWARE-QUEUE.md`
* `docs/orchestration/WAVE-QUEUE.md`
* `docs/hardware/`
* `streams/S1-foundation/`
* `streams/S2-profile/`
* `streams/S3-agent-center/`
* `streams/S4-runtime/`
* `streams/S5-ux/`
* `streams/S6-harvest/`
* `streams/S7-hub/`
* `streams/S8-docs/`
* `docs/adr/ADR-022-decision-program-governance.md`
* `docs/adr/ADR-023-onramp-policy.md`
* `docs/adr/ADR-024-ledger-substrate.md`
