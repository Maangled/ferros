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

## Current State — Phase 0 Prototype Layer

Phase 0 is an **HTML/JS/CSS prototype layer** — a living specification and gamified personal progression dashboard — running entirely in the browser with zero server requirements, accessible via the `file://` protocol.

**It is not the OS** (that is Phase 1+). It IS:
- The conformance test specification (the OS must render it natively to pass Phase 0)
- An early-adopter personal progression tool available right now

**To start:** Open `docs/personal-profile.html` in any modern browser. No installation, no server, no dependencies.

> If you are an AI agent working on this repo, read [`docs/AGENT_GUIDE.md`](./docs/AGENT_GUIDE.md) before touching any code. It contains the bug log, architecture constraints, and role-specific instructions.

### What's Working Right Now (Phase 0)

All of the following are implemented in `docs/personal-profile.html`:

- **Stage 0→1→2→3 onboarding flow** — gamified character creation, consent-first
- **MMO-style Trade Window** — consent/permission dialog controlling session mode and localStorage access
- **Genesis hype page** — locked achievement preview cards, feature pills, profile gallery entry point
- **Level 2–4 progression locks** — XP-gated assist-level unlocks with keyboard bypass fix
- **Resume banner** — returning user detection via localStorage profile, "Clear data" option
- **Export/Import profile JSON** — full profile portability panel with JSON download/upload
- **Alias Mode** — log activity under a borrowed public identity; sessionStorage only, never touches localStorage
- **Profile Gallery** — browse template profiles (Tesla, Curie, Fry, Aurelius, etc.), preview modal, "Use as Alias"
- **Template Schedules** — famous/fictional person schedule templates selectable in character creation
- **Alias Log Claim Flow** — import `.ferros-log` files from alias sessions and merge XP/seals into real profile
- **Key Recovery / Cross-Device Login** — load a profile backup on any machine, log, export a recovery log
- **Cryptographic seal chain** — SHA-256 primary with djb2 fallback (required for `file://` protocol where `crypto.subtle` is unavailable)
- **XP system, attribute tracking, achievement unlocks**

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
├── ferros-blueprint.html                  # Founding architecture board (Phase 0 spec)
├── README.md                              # This file
└── docs/
    ├── adr/                               # Architecture Decision Records
    │   ├── ADR-0001-start-new-do-not-fork.md
    │   ├── ADR-001-progression-lock-pattern.md
    │   ├── ADR-002-smart-contract-boundaries.md
    │   ├── ADR-003-alias-system.md
    │   ├── ADR-004-template-profile-specification.md
    │   └── ADR-005-cross-device-identity-and-session-modes.md
    ├── AGENT_GUIDE.md                     # Agent working guide — read before touching code
    ├── agent-command-center.html          # Agent task/command management UI
    ├── architecture-overview.md           # Prose architecture summary
    ├── deployment-roadmap.html            # Phase roadmap visualization
    ├── ferros-showcase.html               # Public showcase / landing page
    ├── home-hud-dashboard.html            # Smart home HUD prototype
    ├── personal-profile.html              # Personal progression profile (PRIMARY PROTOTYPE)
    ├── schedule-ledger.html               # Schedule/habit ledger prototype
    └── algo-trading-arena.html            # Gamified algorithmic trading platform example
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
| [`docs/algo-trading-arena.html`](./docs/algo-trading-arena.html) | AI-centric gamified algorithmic trading platform — card-based stock trading (Marvel Snap + Runeterra), lottery scratchers, idle mechanics, Genesis creatures, loot boxes, NFT ticket gallery, and AI trading agents |

---

## Documentation

- [`docs/architecture-overview.md`](./docs/architecture-overview.md) — System architecture reference (OS layers + Phase 0 prototype)
- [`docs/AGENT_GUIDE.md`](./docs/AGENT_GUIDE.md) — Agent working guide: bug log, architecture constraints, anti-patterns

### Architecture Decision Records (ADRs)

| ADR | Title |
|-----|-------|
| [ADR-0001](./docs/adr/ADR-0001-start-new-do-not-fork.md) | Start New — Do Not Fork |
| [ADR-001](./docs/adr/ADR-001-progression-lock-pattern.md) | Progression-Lock Pattern |
| [ADR-002](./docs/adr/ADR-002-smart-contract-boundaries.md) | Smart Contract Boundaries |
| [ADR-003](./docs/adr/ADR-003-alias-system.md) | Alias System |
| [ADR-004](./docs/adr/ADR-004-template-profile-specification.md) | Template Profile Specification |
| [ADR-005](./docs/adr/ADR-005-cross-device-identity-and-session-modes.md) | Cross-Device Identity & Session Modes |

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
