# FERROS Core Hardware Targets

> Hardware ambition needs to stay aligned with the current repo reality: Phase 0 is still a local-first, `file://`-compatible, browser-prototype phase, while the future FERROS OS bring-up remains x86_64-first, QEMU-first, and UEFI-first.

---

## Purpose

This document defines the hardware target model for FERROS so the project can plan against real machines without confusing:

- **full FERROS OS targets**
- **Linux-first deployment targets**
- **FERROS-compatible peripherals and edge devices**

The goal is not to split the core into many disconnected architecture tracks. The goal is to define **device classes**, their **runtime role**, and their relationship to the current core roadmap.

---

## Core Rule

Every FERROS surface must run locally on the weakest supported personal node, and optionally upscale onto nearby trusted household compute through explicit local-network delegation.

Implications:

- No household command node should be a hard dependency for daily use.
- Edge nodes should continue operating in local-only mode if heavier compute is unavailable.
- Household acceleration is opt-in, local-first, and consent-driven.
- FERROS should scale from lightweight personal hardware up to multi-node home clusters without changing the user-facing mental model.

---

## Two Hardware Tracks

### 1. Full FERROS OS Targets

These are systems that should eventually boot FERROS itself.

Near-term order of operations:

1. **x86_64 QEMU/KVM** — primary Phase 0 validation target
2. **Real x86_64 desktop/server hardware** — first practical reference machine
3. **x86_64 laptops / portable daily-driver systems** — personal-node expansion
4. **ARM/AArch64 real hardware** — migration path after the x86_64 boot and render path is stable

### 2. FERROS-Compatible Devices

These participate in the FERROS ecosystem without being Phase 0 full-OS bring-up targets.

This includes low-power controllers, Linux-first edge nodes, and future room hardware that can host FERROS surfaces, sensors, automation, and delegated compute.

---

## Device Class Matrix

| Class | Example Hardware | Near-Term Role | FERROS Status |
|------|-------------------|----------------|---------------|
| **Micro node** | ESP32-class boards | Sensors, switches, relays, tiny room controllers, battery-backed appliance endpoints | Ecosystem device, not a Phase 0 full-OS target |
| **Edge ARM node** | Raspberry Pi 4/5, Jetson Orin Nano | Wall screens, room hubs, camera inference, local automation runtime, kiosk surfaces | Linux-first today, FERROS-targetable later |
| **Personal x86 node** | Laptops, handhelds, mini PCs, integrated-GPU Intel/AMD systems | Daily-driver machine, profile root, calendar/ledger/authoring host | Major future FERROS OS target |
| **Command x86 node** | Desktop PCs, home servers, NAS-adjacent boxes, consumer GPU towers | Cluster coordinator, agent runtime, heavy inference, replication, media, shared household compute | Strongest early real-hardware FERROS target |

---

## Device Class Details

### Micro Node

**Examples:** ESP32, future low-power control boards, wall-switch controllers, sensor appliances

**Role:**

- sensing
- switching and relay control
- low-power offline-safe automation
- battery-aware appliance behavior
- room-level transport or protocol endpoint

**Architecture stance:**

- not a good Phase 0 FERROS OS target
- does not match the current Phase 0 assumptions around UEFI, virtual memory, filesystem reads, and HTML/CSS rendering
- should be modeled as a **FERROS-managed peripheral tier**, not as the smallest full FERROS computer

### Edge ARM Node

**Examples:** Raspberry Pi 4/5, Jetson Orin Nano, later room screens or smart display nodes

**Role:**

- room display and kiosk host
- local automation runtime
- camera and sensor aggregation
- voice, mic, speaker, and touchscreen panel host
- local inference bridge for room-scale AI behavior

**Architecture stance:**

- matches the practical deployment path in `docs/deployment-roadmap.html`
- should run Linux first in the near term
- should be considered a **future FERROS migration class**, not the first kernel bring-up target

### Personal x86 Node

**Examples:** Intel/AMD laptops, handheld gaming PCs, integrated-GPU desktops, mini PCs

**Role:**

- primary user machine
- portable identity and profile root
- calendar, ledger, and authoring surface
- local-first daily-driver system

**Architecture stance:**

- strong eventual FERROS target after x86_64 QEMU and reference-hardware bring-up
- should be treated as the baseline personal system that still works when no command node is reachable

### Command x86 Node

**Examples:** desktop towers, home servers, consumer GPU machines, future dedicated server boxes

**Role:**

- shared household compute coordinator
- agent host and orchestration runtime
- heavy inference and export jobs
- replication, storage, backup, NAS-adjacent duties
- household acceleration surface for lighter clients

**Architecture stance:**

- best early real-machine FERROS destination after QEMU
- strongest bridge between the current prototype phase and a real home cluster model
- should be framed as a **user-owned local compute mesh node**, not as involuntary or opaque distributed compute

---

## Current Grounded Hardware Stack

The cleanest hardware sequence from the current repo state is:

1. **Phase 0 OS bring-up:** QEMU/KVM on x86_64
2. **First real FERROS machine:** x86_64 desktop or server hardware
3. **First household Linux deployment:** Raspberry Pi 4/5 kiosk or HUD node
4. **First AI edge node:** Jetson Orin Nano
5. **First ecosystem peripheral class:** ESP32-class micro nodes
6. **First household cluster model:** command-node x86 + Pi/Jetson edge nodes + opt-in client laptops

This keeps the repo aligned with both:

- `README.md` and `docs/architecture-overview.md` for the future OS path
- `docs/deployment-roadmap.html` for the practical Linux-first home rollout path

---

## Household Compute Model

FERROS should not assume all intelligence or automation runs on one powerful machine.

Instead, the household model should be:

- **local-first baseline** on personal and edge devices
- **explicit delegation** to nearby trusted command hardware when available
- **graceful degradation** when heavier compute is offline or restarting
- **replication of critical routines and automations** so a single machine failure does not collapse the home system

Good phrasing for this direction:

- user-owned peer compute pool
- household edge fabric
- opt-in home compute mesh
- trusted local compute delegation

Avoid framing it as a "botnet" even informally. The repo’s identity, permission, and consent model points in the opposite direction: explicit grants, explicit trust, explicit visibility.

---

## Core Planning Guidance

The core should stay as **one FERROS Core OS bar** rather than splitting immediately into many separate architecture bars.

What should change is the planning detail behind that bar:

- document a hardware target matrix
- separate **Phase 0 OS targets** from **Linux-first deployment targets**
- separate **ecosystem peripherals** from **full OS hardware**
- define the household interconnect model the same way the frontend is now being modeled as one interconnected system

---

## Recommended Core Ladder Update

The current core progress ladder should be interpreted as:

| % | Gate | Meaning |
|---|------|---------|
| 10% | Boot target defined | Boot flow and Phase 0 component boundaries are documented |
| 20% | x86_64/QEMU contract fixed | UEFI, framebuffer, and x86_64-first assumptions are explicit |
| 30% | Hardware target matrix documented | Device classes and hardware roles are defined |
| 40% | Device-class assumptions defined | Minimal storage/display/runtime assumptions exist by class |
| 50% | Emulated Phase 0 render contract | The full boot-to-render path is testable in emulation |
| 60% | First real x86_64 reference machine selected | One concrete desktop/server-class machine is chosen for bring-up |
| 70% | Command-node bring-up plan | Real-machine command-node assumptions are documented |
| 80% | ARM migration candidate documented | One ARM edge target is documented alongside the x86_64 path |
| 90% | Device-class conformance checklist | Each device class has explicit validation expectations |
| 100% | QEMU + real hardware milestone | FERROS boots in QEMU and one real x86_64 reference machine, with a documented ARM migration path |

---

## Source Alignment

- `README.md` — Phase 0 success criteria remain the canonical OS target definition.
- `docs/architecture-overview.md` — x86_64-first, QEMU-first, UEFI-first grounding for the OS path.
- `docs/deployment-roadmap.html` — Pi and Jetson are already framed as Linux-first practical deployment hardware.
- `docs/progress/ferros-core-os.md` — core progress ladder should reference this device-class model.
