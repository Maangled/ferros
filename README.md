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
│   ├── ferros-showcase.html       # Public showcase / landing page prototype
│   ├── agent-command-center.html  # Agent governance and ops console prototype
│   ├── home-hud-dashboard.html    # Home kiosk HUD prototype
│   ├── schedule-ledger.html       # Personal schedule ledger prototype
│   ├── personal-profile.html      # RPG-style personal progression dashboard
│   ├── deployment-roadmap.html    # 7-home cluster deployment roadmap
│   ├── architecture-overview.md   # System architecture reference
│   └── adr/                       # Architecture Decision Records
└── LICENSE
```

Source code for the kernel, bootloader, and services will be organized by subsystem as development progresses.

---

## Interactive Prototypes

These self-contained HTML documents serve as both design targets and milestone visualizations for the FERROS platform. Open them locally in any browser — no server or external dependencies required.

| File | Description |
|------|-------------|
| [`docs/ferros-showcase.html`](./docs/ferros-showcase.html) | Public showcase website — a "ferros.dev" landing page with interactive architecture diagram, phase roadmap, boot sequence animation, and contributor guide |
| [`docs/agent-command-center.html`](./docs/agent-command-center.html) | Web-based agent coordination platform — replaces Discord integration from botgen-rust; combines chat, agent governance, live operations console, Kanban project board, and IoT home integration |
| [`docs/home-hud-dashboard.html`](./docs/home-hud-dashboard.html) | FERROS Home HUD Dashboard — the primary kiosk/screen interface for a FERROS-powered home; real-time clock, schedule panel, device controls, agent activity feed, and consent queue |
| [`docs/schedule-ledger.html`](./docs/schedule-ledger.html) | Personal Schedule Ledger — newspaper-themed interactive schedule editor with My Feed viewer, 5-step editor wizard, visual map (year/week/day views), and localStorage persistence |
| [`docs/personal-profile.html`](./docs/personal-profile.html) | RPG-style personal progression dashboard with skill trees, achievements, XP tracking, and ADA-conscious assistance levels (Guided → Director) |
| [`docs/deployment-roadmap.html`](./docs/deployment-roadmap.html) | Technical deployment roadmap with concrete timelines, hardware plans, Gantt chart, interactive cost calculator, and risk register for the 7-home cluster |

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
