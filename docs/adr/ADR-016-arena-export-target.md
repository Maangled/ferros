# ADR-016: Arena Export Target

## Status
Accepted

## Date
2026-04-22

## Context

The Forge workbench (`docs/forge-workbench.html`) can assemble Cards into Decks and export
both locally. The Arena Runtime (`docs/algo-trading-arena.html`) is the rendering host for
FERROS Cards and Decks. Until now, no documented contract exists for what a Forge-exported
Deck payload looks like when handed to the Arena Runtime for consumption.

Two surfaces are converging on the same data flow:
- **Forge (80% gate):** "Arena export target — Forge exports reusable Deck manifests or payloads
  that target the Arena Runtime cleanly."
- **Arena Runtime (70% gate):** "Forge export compatibility — Forge can export Cards or Decks
  that target the Arena Runtime cleanly."

Without a decided schema, both surfaces will invent their own conventions and produce an
integration seam that requires a translation layer. This ADR locks the shape of the export
target before either surface implements it.

### Existing contracts in play

| Contract | ID | File |
|----------|----|------|
| Card schema | C4 | `schemas/card.schema.json` |
| Deck schema | C5 | `schemas/deck.schema.json` |
| Runtime Host Contract | C8 | `docs/contracts/runtime-host-v1.md` |
| Storage Rules (export envelope) | C9 | `docs/contracts/storage-rules.md` |

The C8 `ferros:init` message already carries a `config` block. The Arena export payload
extends this with a resolved Deck manifest so the runtime can load Deck contents without a
separate network or file-system fetch.

---

## Decision

### 1. Export envelope shape

A Forge-to-Arena export is a **standalone JSON blob** with the following top-level structure:

```json
{
  "ferrosVersion": "1.0",
  "exportedAt": "<ISO 8601>",
  "exportType": "arena-deck",
  "deck": { /* conforms to schemas/deck.schema.json */ },
  "cards": [ /* each item conforms to schemas/card.schema.json */ ],
  "runtimeHint": {
    "control": "demo | interactive | static",
    "initialState": "<state name or null>"
  }
}
```

| Field | Required | Description |
|-------|----------|-------------|
| `ferrosVersion` | Yes | Schema version string (e.g. `"1.0"`) |
| `exportedAt` | Yes | ISO 8601 timestamp of export |
| `exportType` | Yes | Always `"arena-deck"` for this contract |
| `deck` | Yes | Full Deck object conforming to C5 (`deck.schema.json`) |
| `cards` | Yes | Array of Card objects, each conforming to C4 (`card.schema.json`). Must include every card referenced in `deck.cardSlots[].cardId`. |
| `runtimeHint` | No | Optional hints for Arena Runtime initialization |
| `runtimeHint.control` | No | Initial C8 control mode. Defaults to `"demo"` if absent. |
| `runtimeHint.initialState` | No | Named state preset to pass in `ferros:init`. Null if absent. |

### 2. Card resolution rule

`cards` must be a **self-contained** snapshot of all cards referenced by the deck at export
time. The Arena Runtime must not need to resolve card references from any secondary store.
Cards not referenced by `deck.cardSlots` may be omitted.

### 3. Relationship to C8 (Runtime Host Contract)

When the Arena Runtime receives an arena-deck payload it:
1. Parses and validates the envelope (ferrosVersion check, deck/cards shape).
2. Resolves cards into an in-memory lookup keyed by `card.id`.
3. Sends `ferros:init` to each card iframe with `config.control` from `runtimeHint.control`
   (or `"demo"` if absent) and `config.state` from `runtimeHint.initialState`.
4. Proceeds with the normal C8 lifecycle (`ferros:init` → `ferros:update` → events).

The arena-deck payload is consumed **before** C8 messaging starts. C8 remains the sole
in-flight protocol once the runtime is initialized.

### 4. Relationship to C5 ↔ C8

C5 defines what a Deck **is** (schema). C8 defines how a Deck **runs** (runtime host
lifecycle). The arena-deck export envelope is the **hand-off layer** between authoring (Forge,
C5) and runtime (Arena, C8). No existing contract is modified.

### 5. Version negotiation

The `ferrosVersion` field in the export envelope uses the same semantics as the profile export
(C9): exact MAJOR match required. A runtime receiving a `ferrosVersion` with a different MAJOR
must reject with `ARENA_EXPORT_VERSION_MISMATCH` and render a human-readable error rather than
silently loading invalid data.

Minor version differences within the same MAJOR are accepted silently (additive fields
tolerated).

### 6. File name convention

Forge-generated arena-deck export files must follow:

```
ferros-deck-[sanitized-deck-name]-[YYYY-MM-DD].arena.json
```

- `sanitized-deck-name`: deck name lowercased, spaces replaced with `-`, non-alphanumeric
  characters removed.
- `.arena.json` extension distinguishes arena-deck payloads from plain profile exports
  (`.json`) and audit logs (`.ferros-log`).

---

## Consequences

### Positive
- Both Forge (80% gate) and Arena Runtime (70% gate) now have a single target to implement
  against. No translation layer required.
- The hand-off is self-contained: the runtime receives everything it needs to render a Deck
  in one file, matching the offline-first `file://` constraint.
- The `runtimeHint` block lets Forge authors set the initial control mode without modifying
  C8 or C5.
- Versioning rules are consistent with the existing C9 profile export pattern.

### Negative / trade-offs
- Cards are duplicated between the Forge Bag and the arena-deck export. Acceptable at Phase 1:
  the Bag is a local catalog, not a distribution network.
- No delta-export: re-exporting after editing a card requires a full re-export of the deck.
  Addressed when Arena Runtime reaches the "external embedding" gate (60%).

### Neutral
- C4, C5, C8, C9 are unchanged. This ADR adds a new envelope; it does not modify any existing
  contract.

---

## Related

- `docs/progress/forge.md` — Forge 80% gate: "Arena export target"
- `docs/progress/arena-runtime.md` — Arena Runtime 70% gate: "Forge export compatibility"
- `docs/contracts/runtime-host-v1.md` — C8 lifecycle the runtime follows after loading the export
- `schemas/card.schema.json` — C4 card shape embedded in the export
- `schemas/deck.schema.json` — C5 deck shape embedded in the export
- `docs/contracts/storage-rules.md` — C9 export envelope pattern this follows
- ADR-008 (Modular Rendering System) — rendering conventions
- ADR-010 (Cards and Decks Nomenclature) — Card/Deck/Bag vocabulary
