# FERROS Progress Specs

**Overall FERROS Ecosystem:** 1%

This index tracks the current documented status of the FERROS ecosystem using `README.md` as the authoritative source for percentages and top-level status notes.

## How to read these files

- `README.md` percentages and status notes are authoritative.
- Prototype HTML files may contain illustrative or demo percentages; those are not authoritative for these specs.
- Milestone gates are planning targets for repo-verifiable progress, not delivery commitments.
- Phase vocabulary in this subtree is limited to: `Prototype`, `Architecture`, `Active`, `Planned`, `Blocked`, `Future`.
- `50%` means a complete local-first standalone workflow that is understandable and useful without backend services.
- `60%` through `80%` is where service, agent, API, contract, or multi-surface integration begins.
- `90%` through `100%` is reserved for hardened, portable, auditable, and reusable cross-surface behavior.

## Interconnected System Model

- **Card** — the atomic FERROS object.
- **Deck** — a composed collection of Cards.
- **Bag** — the local catalog or inventory of Cards and Decks.
- **Portal Runtime instance** — a rendered or experienced Deck/Card surface inside the Arena Runtime.
- **Profile-linked object** — any Card, Deck, or runtime state attributed to identity, permissions, rewards, or verification.

## Summary Table

| Name | Current % | Phase | Status | Spec Link |
|------|-----------|-------|--------|-----------|
| FERROS Core OS | 1% | Architecture | Phase 0 rendering path is defined; no implementation tree exists yet | [ferros-core-os.md](./ferros-core-os.md) |
| Founding Blueprint | 1% | Active | Read-only Phase 0 conformance target | [blueprint.md](./blueprint.md) |
| The Forge | 2% | Prototype | Early architecture; workbench exists but no creation tools exist yet | [forge.md](./forge.md) |
| Arena Runtime | 1% | Prototype | Reusable portal/runtime layer is emerging but not separated yet | [arena-runtime.md](./arena-runtime.md) |
| Battle Arena | 1% | Prototype | Game-specific arena prototype currently shares one overloaded surface | [trading-arena.md](./trading-arena.md) |
| Personal Profile | 1% | Active | Prototype exists in `docs/personal-profile.html` and serves as the portable identity root | [personal-profile.md](./personal-profile.md) |
| Schedule Ledger | 1% | Prototype | Prototype exists in `docs/schedule-ledger.html` | [schedule-ledger.md](./schedule-ledger.md) |
| Showcase / Landing Page | 1% | Prototype | Prototype exists in `docs/ferros-showcase.html` | [showcase.md](./showcase.md) |
| Agent Command Center | 1% | Prototype | Prototype exists in `docs/agent-command-center.html` | [agent-command-center.md](./agent-command-center.md) |
| Home HUD Dashboard | 1% | Prototype | Prototype exists in `docs/home-hud-dashboard.html` | [home-hud.md](./home-hud.md) |
| User / Identity System | 1% | Architecture | Primary bottleneck; consent-first, cross-device, alias modes | [user-identity-system.md](./user-identity-system.md) |
| Templates & Profiles | 1% | Prototype | Template profiles exist but need major expansion | [templates-and-profiles.md](./templates-and-profiles.md) |
| Assets, Cards & Decks | 1% | Planned | Card systems are referenced but barely started | [assets-cards-decks.md](./assets-cards-decks.md) |
| Agent Integration | 1% | Blocked | Depends on the user system before agent-driven content updates can work | [agent-integration.md](./agent-integration.md) |

## 🖥️ FERROS Core OS

| Name | Current % | Phase | Status | Spec Link |
|------|-----------|-------|--------|-----------|
| FERROS Core OS | 1% | Architecture | Phase 0 rendering path is defined; no implementation tree exists yet | [ferros-core-os.md](./ferros-core-os.md) |
| Founding Blueprint | 1% | Active | Read-only Phase 0 conformance target | [blueprint.md](./blueprint.md) |

## 🌐 Online Platform / Web Projects

| Name | Current % | Phase | Status | Spec Link |
|------|-----------|-------|--------|-----------|
| The Forge | 2% | Prototype | Early architecture; workbench exists but no creation tools exist yet | [forge.md](./forge.md) |
| Arena Runtime | 1% | Prototype | Reusable portal/runtime layer is emerging but not separated yet | [arena-runtime.md](./arena-runtime.md) |
| Battle Arena | 1% | Prototype | Game-specific arena prototype currently shares one overloaded surface | [trading-arena.md](./trading-arena.md) |
| Personal Profile | 1% | Active | Prototype exists in `docs/personal-profile.html` and serves as the portable identity root | [personal-profile.md](./personal-profile.md) |
| Schedule Ledger | 1% | Prototype | Prototype exists in `docs/schedule-ledger.html` | [schedule-ledger.md](./schedule-ledger.md) |
| Showcase / Landing Page | 1% | Prototype | Prototype exists in `docs/ferros-showcase.html` | [showcase.md](./showcase.md) |
| Agent Command Center | 1% | Prototype | Prototype exists in `docs/agent-command-center.html` | [agent-command-center.md](./agent-command-center.md) |
| Home HUD Dashboard | 1% | Prototype | Prototype exists in `docs/home-hud-dashboard.html` | [home-hud.md](./home-hud.md) |

## 🔧 Shared Infrastructure

| Name | Current % | Phase | Status | Spec Link |
|------|-----------|-------|--------|-----------|
| User / Identity System | 1% | Architecture | Primary bottleneck; consent-first, cross-device, alias modes | [user-identity-system.md](./user-identity-system.md) |
| Templates & Profiles | 1% | Prototype | Template profiles exist but need major expansion | [templates-and-profiles.md](./templates-and-profiles.md) |
| Assets, Cards & Decks | 1% | Planned | Card systems are referenced but barely started | [assets-cards-decks.md](./assets-cards-decks.md) |
| Agent Integration | 1% | Blocked | Depends on the user system before agent-driven content updates can work | [agent-integration.md](./agent-integration.md) |