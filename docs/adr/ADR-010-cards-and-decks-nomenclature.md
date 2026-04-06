# ADR-010: Cards and Decks Nomenclature

**Status:** Accepted
**Date:** 2026-04-06
**Context:** FERROS asset taxonomy — naming convention, catalog structure, decentralized traceability

---

## Context

FERROS manages visual assets at two levels:
1. **Individual assets** — standalone HTML renders (parts like Box Body, Edge Bracket, etc.)
2. **Compositions** — project manifests that assemble multiple assets into a scene with state
   presets (like the Loot Box Assembly)

The Forge workbench previously called these "Parts" and "Projects." While technically accurate,
these terms don't extend to the broader FERROS ecosystem where assets flow between workbenches,
game arenas, trading systems, and player inventories. A part in the Forge is a collectible in
the Arena, a tradeable in the marketplace, and a buildable in the deck builder.

Every entity a user touches — whether inspecting it, trading it, battling with it, or
assembling it — needs to be **catalogable and traceable** in a decentralized system. The
trading-card metaphor provides this naturally: cards have unique identities, provenance, and
rarity; decks are curated collections with rules and composition constraints.

---

## Decision

**Adopt "Cards" and "Decks" as the universal nomenclature for FERROS assets.**

### Definitions

| Term | Replaces | Meaning |
|------|----------|---------|
| **Card** | Part, asset, item | A single cataloged entity with a unique identity. Cards are the atomic unit of the FERROS system. Every visual asset, tool, ability, creature, or component is a Card. |
| **Deck** | Project, composition, collection | An ordered collection of Cards with composition rules, state presets, and assembly logic. Decks define how Cards combine into something greater. |
| **Bag** | Asset browser, catalog, inventory | The player's local collection of Cards and Decks. The Bag is the browsing/selection UI for any FERROS page that needs asset discovery. |

### Scope

The nomenclature applies at every layer:
- **Forge Workbench:** Bag panel shows Cards (individual renders) and Decks (assemblies).
- **Arena:** Players build Decks of Cards to compete. The Forge itself can be represented as
  a Card in the Arena battle system.
- **Trading:** Cards carry provenance metadata (creator, mint date, rarity). Trading is
  card-to-card exchange.
- **Marketplace / Store:** Items listed for acquisition are Cards. Bundles are Decks.
- **Inventory:** A player's holdings are a Bag of Cards organized into Decks.

### Card Identity

Every Card must be representable as a JSON object with at minimum:
- `id` — unique identifier
- `kind` — taxonomy classifier (part, project, creature, ability, etc.)
- `name` — human-readable label
- `icon` — emoji or image reference for compact display

The existing `part.schema.json` and `project.schema.json` already satisfy this requirement.
Future schemas (creature, ability, tool) must include these four fields.

### Why Not Just "NFT"?

The Card/Deck model is implementation-agnostic. Cards *can* be minted as on-chain tokens, but
they can also exist as plain JSON manifests in a local file system, as rows in a database, or
as entries in a P2P content-addressed store. The nomenclature describes the *concept* (unique
cataloged entity), not the *storage mechanism*.

---

## Consequences

- All FERROS UI labels use Card/Deck/Bag terminology. Legacy terms (part, project, asset
  browser) are replaced in user-facing text but may persist in code identifiers (`kind: 'part'`)
  for backward compatibility.
- New schemas must include the four Card identity fields (`id`, `kind`, `name`, `icon`).
- The Forge Bag groups assets as "Cards" and "Decks" in the catalog drawer headings.
- Game arena UIs can reference "your deck" and "play a card" with consistent meaning across
  the platform.
- The metaphor naturally accommodates rarity tiers, foil/holographic variants, and
  collectibility mechanics without inventing new vocabulary.
- Any FERROS entity — including the Forge workbench itself — can be turned into a Card,
  enabling recursive composition (a Forge Card in an Arena Deck that battles other Forge
  configurations).
