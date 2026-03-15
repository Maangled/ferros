# FERROS — Foundational Embedded Rust Runtime Operating System

> Memory safe. Consent first. Locally sovereign. Distributed by design. Agent-ready.

**Internal Codename:** Project Nomos (constitutional layer / long-range doctrine)

FERROS is a ground-up Rust-based operating system designed to replace Linux in environments where memory safety, verifiable component ownership, auditable governance, privacy-preserving distributed coordination, and human-readable consent are non-negotiable architectural requirements.

It is not a Linux distribution, not a container runtime, and not a research prototype. It is a systems engineering program with a concrete first deliverable: boot a Rust-native kernel on real or virtualized hardware and render a self-describing HTML architecture board as the system's first usable interface.

---

## Founding Blueprint

📄 **[`ferros-blueprint.html`](./ferros-blueprint.html)** — The founding architecture board, styled as a dark-theme command-center dashboard. Open this file locally in any browser. It is self-contained with no external dependencies.

This document is simultaneously the **specification** and the **Phase 0 conformance test**. When FERROS renders it natively, Phase 0 is complete.

---

## Primary Deployment Targets

- Secure smart-home HUD kiosks
- Peer-to-peer coordination devices
- Federated infrastructure nodes
- Identity-sovereign personal computing platforms
- Trusted AI agent host systems

---

## Core Philosophy

| # | Principle | Summary |
|---|-----------|---------|
| 1 | **Memory Safety as Infrastructure** | Every component in Rust; unsafe code quarantined, documented, and tracked |
| 2 | **Reduced Kernel Attack Surface** | Microkernel-hybrid; drivers and network stacks run in isolated user-space |
| 3 | **Reproducible Builds** | Every build reproducible from source; artifacts hashed and signed |
| 4 | **Observable, Accountable, Reviewable** | Structured health telemetry, tamper-evident configuration logs |
| 5 | **Secure by Default** | Services start with no permissions; all grants are explicit |
| 6 | **Privacy by Design** | Data encrypted at rest; no silent data flows |
| 7 | **User-Readable Consent** | Every permission request is one sentence in plain language |
| 8 | **Local-First Sovereignty** | User data lives on the local device first; cloud is always optional |
| 9 | **Distributed by Design** | CRDTs, E2E-encrypted channels, capability-scoped trust delegation |
| 10 | **Agent-Ready Architecture** | Autonomous AI agents under explicit human governance |

---

## Roadmap

| Phase | Name | Status |
|-------|------|--------|
| **0** | Render the Board | 🟢 Active |
| **1** | Local UI Shell & Secure Storage | 🔵 Planned |
| **2** | User Identity & Consent Engine | 🔵 Planned |
| **3** | Smart-Home HUD & Device Control | ⚪ Future |
| **4** | Contract & Wallet Integration | ⚪ Future |
| **5** | Distributed Sync & Federated Processing | ⚪ Future |
| **6** | Agent Hosting & Migration | ⚪ Future |
| **7** | Hardened Release Candidate & Governance Review | ⚪ Future |

### Phase 0 — Render the Board

**Success criteria:**
- System boots from a Rust-native UEFI bootloader
- Kernel initializes virtual memory, framebuffer, and a minimal filesystem driver
- HTML parser reads the blueprint document from local storage
- CSS engine computes layout for sections, cards, grids, tables, and typography
- Text is rendered with a readable embedded font at native resolution
- Colors, borders, gradients, and panel layout match the specification
- No external network access is required
- The system runs on QEMU/KVM and at least one reference hardware target

---

## Repository Structure

```
ferros/
├── ferros-blueprint.html   # Founding architecture board (Phase 0 test case)
├── README.md               # This file
├── docs/                   # Architecture specs, ADRs, threat models, governance
└── LICENSE
```

Source code for the kernel, bootloader, and services will be organized by subsystem as development progresses.

---

## Documentation

- [`docs/architecture-overview.md`](./docs/architecture-overview.md) — System architecture reference

Architecture Decision Records (ADRs) will be added to `docs/adr/` as architectural decisions are made.

---

## Related Projects

- **botgen-rust** — Current AI agent hosting system; FERROS Phase 6 provides a migration path from Discord/cloud to self-hosted infrastructure
- **HeroProtagonist.dao** — Identity and workspace system that FERROS will eventually support natively
- **Athena OS** — Design inspiration for declarative state management and atomic upgrades
- **AEGIS-OS** — Inspiration for owner-controlled verified boot and hardware-rooted isolation

---

## License

See [LICENSE](./LICENSE).

---

*FERROS — Founding Specification v0.1 — 2026*
