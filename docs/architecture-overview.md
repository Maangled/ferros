# FERROS — Architecture Overview

> This document is a prose summary of the system architecture described in
> [`ferros-blueprint.html`](../ferros-blueprint.html). The blueprint is the
> authoritative reference and Phase 0 conformance test.

---

## System Layers

```
┌─────────────────────────────────────────────────────┐
│              AI Agent Hosting Layer                  │
├─────────────────────────────────────────────────────┤
│           Smart Contract Bridge / Consent Engine     │
├─────────────────────────────────────────────────────┤
│   Identity &     │  Distributed  │  Package &        │
│   Permission     │  Sync (CRDTs) │  Update Model     │
├─────────────────────────────────────────────────────┤
│  HTML Renderer   │  Display / UI │  Networking       │
│  (CSS + Layout)  │  Compositor   │  (TLS 1.3)        │
├─────────────────────────────────────────────────────┤
│           Storage Subsystem (CoW, Encrypted)         │
├─────────────────────────────────────────────────────┤
│           Driver Model (User-space, MMIO caps)       │
├─────────────────────────────────────────────────────┤
│        Kernel: VMM · Scheduler · Capability IPC      │
├─────────────────────────────────────────────────────┤
│              UEFI Bootloader (Rust-native)           │
└─────────────────────────────────────────────────────┘
```

---

## Bootloader & Early Boot

The bootloader is a standalone Rust binary targeting the UEFI environment — not a
wrapper around GRUB or any C-based loader. It:

- Verifies the kernel image signature against a local trust anchor
- Initializes the framebuffer for early visual feedback
- Sets up initial page tables
- Transfers control to the kernel entry point
- Where hardware permits, stores boot measurements in a local attestation log (measured boot)

## Kernel

The kernel follows a **microkernel-hybrid** design. Responsibilities:

- Virtual memory management with hardware-enforced process isolation
- Pre-emptive scheduling (priority-based and fair-share policies)
- Synchronous/asynchronous IPC through typed capability channels

All higher-level services — filesystems, networking, device drivers, HTML renderer —
run as isolated **user-space servers**. The blast radius of any single component
failure is limited to that component's address space.

## Process & Memory Model

- Each process runs in its own address space
- **Slab allocator** for kernel objects; **buddy allocator** for user-space pages
- Shared memory regions require explicit capability grants from both parties
- Memory ownership tracked through Rust's type system at compile time and through
  capability tables at runtime
- No global shared state outside of explicitly negotiated IPC channels

## Driver Model

Drivers are user-space processes that receive hardware access through MMIO capability
grants from the kernel. A **driver registry** manages discovery, loading, versioning,
and hot-replacement. Each driver declares its hardware requirements and permission scope
in a manifest. Faulty or unresponsive drivers are restarted without kernel involvement.

**Initial driver targets:**
- AHCI / NVMe storage
- virtio-blk / virtio-net / virtio-gpu (virtualized environments)
- USB HID (input devices)
- Basic linear framebuffer (display output)

## Storage Subsystem

A log-structured, copy-on-write (CoW) filesystem with:

- Cryptographic checksums at every block level (integrity verification)
- File-level encryption with keys derived from user authentication credentials
- Atomic snapshots for upgrades and rollback

Phase 0 requires only **read-only** access to a simple partition. Full read-write with
encryption and snapshots is a Phase 1 deliverable.

## Networking Subsystem

User-space TCP/IP stack in Rust, inspired by smoltcp, extended with TLS 1.3 as a
first-class primitive. Features:

- All connections encrypted by default; plain-text requires explicit logged opt-in
- DNS-over-HTTPS and DNS-over-TLS support
- Both client-server and peer-to-peer topologies

Networking is **not required for Phase 0** but is foundational for Phase 3 onward.

## Display & UI Subsystem

The display pipeline:

1. Framebuffer obtained from UEFI or a basic GPU driver
2. Rust-native 2D compositor (alpha blending, damage tracking, surface layering)
3. Font rendering via **fontdue** or **ab_glyph** with bundled font files (no external
   font services required)

For Phase 0, a single full-screen surface is sufficient.

## HTML Rendering Path (Critical Path for Phase 0)

A purpose-built Rust-native HTML5 parser + CSS layout engine. **Not** an embedded
browser engine like Chromium or WebKit. Supported features for Phase 0:

| Feature | Status |
|---------|--------|
| Semantic HTML5 elements | Required |
| Embedded CSS (class, element, pseudo-class selectors) | Required |
| Full box model (padding, margin, border) | Required |
| Flexbox layout | Required |
| CSS grid layout | Required |
| Color values (hex, rgba, named, CSS variables) | Required |
| Font sizing, line height, letter spacing | Required |
| border-radius | Required |
| Background colors | Required |
| Linear gradients | Required |
| Basic keyframe animations (opacity/color) | Required |
| JavaScript | Not required (Phase 0) |
| External resource loading | Not required (Phase 0) |
| Dynamic DOM manipulation | Not required (Phase 0) |

## Package & Update Model

- Content-addressed, signed archives
- Each package declares capabilities, dependencies, and required permissions in a
  machine-readable manifest
- Updates applied atomically: new version staged, integrity-verified, swapped in a
  single filesystem operation
- Rollback to the previous version is always available
- No auto-update without user consent

## Identity & Permission Layer

- **Local-first** identity: cryptographic keypair per user
- Stored in hardware-backed secure storage (TPM, secure enclave) where available;
  encrypted keystore otherwise
- Optional linkage to decentralized identifiers (DIDs) for cross-device recognition
- Permissions are **capability tokens**: unforgeable references encoding resource,
  permitted operations, time window, and requesting identity
- Capabilities are scopeable, delegatable, and revocable at any time

## Smart Contract Integration Layer

A sandboxed contract runtime bridge for interacting with on-chain and off-chain smart
contracts. Design constraints:

- No direct kernel access
- Communicates exclusively through the consent engine
- Abstracts chain-specific details behind a common API
- Supports multiple ledger backends
- Multi-chain support via unified bridge interface

## Local-First & Distributed Synchronization

- All user data lives on the local device first
- Synchronization uses **CRDTs** (Conflict-free Replicated Data Types) for eventual
  consistency without central coordination
- Sync channels are end-to-end encrypted
- Direct P2P sync between devices on the same network
- Relay-assisted sync through untrusted intermediaries (relays cannot read the data
  they relay)
- Conflict resolution is deterministic and user-inspectable

## AI Agent Hosting Layer

FERROS provides a sandboxed execution environment for autonomous AI agents:

- Each agent runs in an isolated process with capability-scoped resource access
- All agent actions logged in the tamper-evident audit log
- Agents cannot access user data, device controls, network resources, or other agents
  without explicit capability grants through the consent engine
- Designed to support migration of agent workloads from external platforms (e.g.,
  Discord bots managed by botgen-rust) onto self-hosted FERROS infrastructure

---

## Phase 0 — Required Pipeline

```
Boot → Storage → Parse → Layout → Paint
```

| Step | Component | Requirement |
|------|-----------|-------------|
| Boot | Rust-native UEFI bootloader | Verified boot, framebuffer init |
| Storage | Minimal filesystem driver | Read-only partition access |
| Parse | HTML5 parser | Load ferros-blueprint.html |
| Layout | CSS engine | Block, flex, grid, full box model |
| Paint | Framebuffer compositor | Font raster, colors, gradients |

**Validation targets:** QEMU/KVM (x86_64) and at least one reference hardware board.

---

## Security Principles

| Property | Mechanism |
|----------|-----------|
| Asset traceability | Content-addressed components, local SBOM |
| Integrity verification | Signed manifests verified at every boot |
| Signed modules | Trust anchor set; attestation reports |
| Encrypted data | User-derived keys, transparent to authorized apps |
| Permission-scoped access | Capability tokens at every IPC boundary |
| Tamper evidence | Append-only, hash-chained audit log |
| Human-readable authorization | Single-sentence permission prompts, character-limited |
| Compartmentalization | Isolated address spaces, minimal IPC surface |
| Auditing & rollback | Every update snapshottable; single-operation rollback |
| Secure messaging | E2E encryption with forward secrecy |

---

## Consent Engine Properties

Every data access must pass through the consent engine. Grants are:

- **Revocable** — immediate effect, propagates to downstream services
- **Traceable** — tamper-evident log entry for every grant and revocation
- **Minimal** — narrowest possible access scope
- **Time-bounded** — no perpetual grants
- **Purpose-bound** — a different purpose requires a separate grant

---

*See also: [`ferros-blueprint.html`](../ferros-blueprint.html) for the full founding
specification and Phase 0 conformance test document.*
