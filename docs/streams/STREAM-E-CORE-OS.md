# Stream E — Core OS & Native Rendering

> **Stream status:** Research track, Wave R0 (architecture planning). No Rust source tree yet.
> **Philosophy:** Independence is a feature. FERROS is not a website. It's an operating system that happens to start on the web.

---

## What This Stream Is

Stream E is the long game. While Streams A–D build a fully functional web platform, Stream E proves that the platform can eventually run without a browser — natively, on bare metal, from boot.

The mission: FERROS should boot on an x86_64 machine, load `ferros-blueprint.html` from local storage, and render it natively — no browser, no OS, just FERROS.

This is not a distraction from the web platform. It is the reason the web platform is designed the way it is. The `file://` constraint, the localStorage model, the local-first philosophy — all of these are choices that make sense on the web *and* on bare metal. The two tracks reinforce each other.

---

## Why Native Matters for FERROS's Mission

### Independence from the Browser

A FERROS profile that only works in Chrome is fragile. Chrome can change its `file://` security model. Chrome can add or remove APIs. Chrome can stop existing.

A FERROS kernel that renders `ferros-blueprint.html` directly is immune to all of that. The rendering contract is between FERROS and the HTML spec, not between FERROS and Google.

### The "No Infrastructure" Guarantee

FERROS's identity system, creative pipeline, and consumer surfaces are all designed to work without a server. This is the local-first philosophy. But "local-first" is incomplete if "local" means "local to a browser on someone else's OS."

The full local-first guarantee is: your FERROS profile, your card collection, your schedule, your agent directives — all of it runs on hardware you own, in software you control, from the moment the machine powers on.

Stream E makes that guarantee real.

### The Fixture Corpus as the Conformance Target

Stream A produces the golden fixture corpus — 19 JSON files that describe what a valid Profile, Card, Deck, Session, and Schedule Event look like. These fixtures are the web platform's truth.

For native rendering to be correct, it must render these same fixtures the same way the web platform renders them. Stream E's R3 capability (native rendering vs web fixture corpus) is the proof that native conformance is real.

---

## Hardware Targets

Reference: `docs/core-hardware-targets.md` (authoritative). Summary here.

| Class | Example Hardware | Phase 0 Role | FERROS Status |
|-------|-----------------|-------------|---------------|
| Personal x86 | Laptops, mini PCs | Daily-driver machine, profile root, local-first personal system | **Primary OS target** |
| Command x86 | Desktops, servers, GPU towers | Cluster coordinator, agents, heavy inference | **Strongest early real-hardware target** |
| Edge ARM | Raspberry Pi 4/5, Jetson | Screens, room hubs, edge inference | Linux-first today, FERROS migration path |
| Micro node | ESP32-class | Sensors, switches, tiny room controllers | Ecosystem device — not a FERROS OS target |

### Targeting Principles

1. **x86_64-first.** The first kernel bring-up is x86_64. QEMU is the development target before real hardware.
2. **UEFI-first.** FERROS uses the UEFI boot path — modern firmware, not legacy BIOS shims.
3. **ARM is real but secondary.** ARM edge nodes (Raspberry Pi 4, Jetson) are documented as migration targets after x86_64 is proven.
4. **ESP32 is ecosystem, not OS.** Micro nodes are FERROS-compatible peripherals that communicate with FERROS nodes. They are not the smallest FERROS computer.

---

## QEMU Bring-Up Goals

QEMU is the path from architecture to reality. Before we run on real hardware, we need a stable emulated environment where the boot flow is reproducible and debuggable.

**R2 target:** QEMU boots to a FERROS surface.

### What the QEMU bring-up proves:

1. The bootloader (Rust-native UEFI) loads correctly in emulation
2. The kernel initializes memory and enters userspace (or equivalent)
3. The filesystem driver reads from a virtual disk image
4. The HTML/CSS renderer loads `ferros-blueprint.html` from disk
5. The framebuffer outputs a rendered frame matching the web rendering of the same file

### QEMU bring-up sequence:

```
Power on (QEMU -bios OVMF)
  → UEFI firmware initializes
  → FERROS bootloader (UEFI application) loads
  → Bootloader hands off to kernel entry point
  → Kernel initializes: memory manager, framebuffer, minimal filesystem
  → Kernel loads ferros-blueprint.html from virtual disk
  → HTML/CSS renderer parses and lays out the document
  → Renderer outputs pixel buffer to UEFI framebuffer
  → Screen shows: ferros-blueprint.html rendered natively
```

This is the Phase 0 OS success condition as defined in `docs/progress/ferros-core-os.md`.

---

## Renderer Conformance Against the Web Fixture Corpus

R1 capability: A renderer conformance suite exists with golden fixtures.

### What the conformance suite tests:

The golden fixture corpus from Stream A defines what every FERROS data object looks like. For native rendering to be correct, the renderer must display these objects the same way the web platform displays them.

The conformance suite works by comparing:
1. **Web reference:** A screenshot of the web platform rendering a golden fixture (Chrome, `file://`)
2. **Native output:** A pixel buffer from the native renderer rendering the same fixture

If the outputs match (within a tolerance threshold for font rendering differences), the renderer is conformant.

### R3 — Native/web parity:

R3 is the end state: the native renderer produces output that is semantically equivalent to the web platform for all golden fixtures.

"Semantically equivalent" does not mean pixel-perfect. It means:
- All required fields are visible
- Layout structure matches (headings, values, order)
- Colors and typography match the defined scheme
- No fields are dropped, truncated, or garbled

---

## Scoped OS Components

From `docs/progress/ferros-core-os.md`:

| Component | Current | Notes |
|-----------|---------|-------|
| UEFI Bootloader | 1% | Rust-native boot path required |
| Kernel (x86_64) | 1% | Primary Phase 0 target |
| Kernel (ARM/AArch64) | 0% | Not started — Wave 2+ |
| Kernel (RISC-V) | 0% | Not started |
| Memory Manager | 0% | Required for virtual memory init |
| Filesystem Driver | 0% | Minimal read-only for Phase 0 |
| HTML/CSS Renderer | 1% | Requirements specified in blueprint |
| Framebuffer / Graphics | 0% | Required for output |
| Phase 0 Conformance | 1% | Success = renders blueprint natively |

---

## Wave Structure

Stream E does not use the same wave model as Streams A–D. It uses a **research track** model — capabilities can be worked on in any order, and no Stream E capability blocks any Stream A–D capability.

| Capability | Goal | Entry | Exit |
|-----------|------|-------|------|
| R1 | Renderer conformance suite | Anytime | Suite exists; golden web renders captured |
| R2 | QEMU bring-up proven | R1 useful but not required | QEMU boots to FERROS surface |
| R3 | Native/web parity | R1 + R2 | Native output matches web corpus |

### Recommended Sequence

Although these capabilities are independent, the logical progression is:

1. **R1 first:** Capture the web rendering baseline. Screenshot every golden fixture in Chrome. These screenshots are the conformance targets. This work can be done right now — it requires only Chrome and the existing harnesses.

2. **R2 second:** Get QEMU booting. This is the hard part — it requires Rust code, a working kernel, and a functional HTML/CSS renderer. This is the primary research work of Stream E.

3. **R3 third:** Run R1's web screenshots against R2's native output. Document parity. This closes the research track.

---

## Entry / Exit Criteria

### R1 Entry
- Stream A fixtures exist (they do ✅)
- Web harnesses run in Chrome (they do ✅)

### R1 Exit
- Conformance suite exists as a harness or automated tool
- Web reference screenshots captured for all 13 golden fixtures
- Comparison methodology documented

### R2 Entry
- No hard entry requirement — can start anytime
- R1 useful for knowing what the target looks like

### R2 Exit
- QEMU launches and reaches the FERROS surface
- A human-readable output (even text-only) confirms the boot path
- The boot image is committed to the repository or documented

### R3 Entry
- R1 exit ✅ (have the target)
- R2 exit ✅ (have the renderer)

### R3 Exit
- For each golden fixture: native output is documented alongside web reference
- Parity gaps (if any) are listed as known issues
- A responsible human has verified the outputs are "semantically equivalent"

---

## Key Artifacts to be Produced by Stream E

| Artifact | Description | Wave |
|---------|-------------|------|
| Web reference screenshots | Chrome-rendered golden fixtures | R1 |
| Conformance test suite | Automated comparison tool or harness | R1 |
| QEMU boot image | First working boot to FERROS surface | R2 |
| Boot documentation | Step-by-step QEMU bring-up guide | R2 |
| Parity report | Native vs web output comparison | R3 |
| ARM migration docs | Documented path for first ARM target | R2/R3 |

---

## Legacy Integration Items for Stream E

Per ADR-013, one legacy item is relevant to Stream E:

| ID | Pattern | Source | Status |
|----|---------|--------|--------|
| L10 | WASM contract validators | `workpace-rust build-wasm.sh` | Research track — not activated yet |

WASM contract validators would allow Stream A's schema validation logic to run natively (inside the OS) rather than relying on JavaScript. This is a Stream A/E boundary item — deferred to the research track.

---

## Philosophy: Why the Web and the OS Are the Same Project

It might seem like Stream E is a separate project from the web platform. It's not.

**The web platform is the specification.** When `ferros-blueprint.html` renders in Chrome, it defines exactly what the native renderer must produce. The HTML file is both the user-facing artifact and the engineering spec for the OS renderer.

**The contracts are the same.** The same `schemas/profile.schema.json` that the web platform validates against is the schema the OS must understand. The same `FerrosCore.serializeExport()` logic that produces portable profile tokens is the logic the OS must implement natively.

**The local-first model is shared.** The web platform uses localStorage. The OS uses a local filesystem. Both are local-first, offline-capable, user-controlled storage. The data model is identical; only the storage layer changes.

This is why the platform is called FERROS — it's one system expressed at different layers of the stack. The web layer is the accessible entry point. The OS layer is the sovereign endpoint. Both are FERROS.

---

## Architecture References

Key documents for Stream E work:

| Document | Content |
|---------|---------|
| `docs/architecture-overview.md` | Boot-to-render path, subsystem targets |
| `docs/core-hardware-targets.md` | Device classes, hardware roles |
| `ferros-blueprint.html` | Phase 0 conformance target — the document the OS must render |
| `docs/deployment-roadmap.html` | Separates Linux deployment track from kernel track |
| `docs/adr/ADR-0001-start-new-do-not-fork.md` | Greenfield implementation — not a forked kernel |
| `docs/progress/ferros-core-os.md` | Detailed milestone gates and component status |
| `docs/progress/blueprint.md` | Blueprint as conformance target documentation |
