# ADR-011: Routine Module System

**Status:** Accepted
**Date:** 2026-04-08
**Supersedes:** ADR-004 (template profiles demoted to alias identity pool; ADR-004 body untouched per append-only rule)
**Context:** FERROS personal profile — modular routine composition, daily-use lifestyle management, D&D character-builder metaphor

---

## Context

The personal profile system (PR 6–17) shipped an 8-template celebrity gallery (ADR-004) that seeds character creation and the alias system. While the opening sequence and onboarding flow are polished, daily use stalls after Stage 2 because:

1. **Templates are monolithic.** A user who picks "Nikola Tesla" gets Tesla's entire day structure — they cannot mix Tesla's deep-work blocks with Aurelius's morning journaling.
2. **Habits are forced.** Every template includes "Brush teeth" but many users don't care to track oral hygiene. There is no opt-in/opt-out at the task level.
3. **No composition engine.** Users cannot build a schedule from reusable parts. The schedule is either a wholesale template fork or a blank slate.
4. **Schedules don't compress.** A night-owl who wakes at 1 PM has fewer available hours, but the system has no mechanism to proportionally shrink flexible blocks.
5. **No daily reward loop.** Completing tasks in the Schedule Ledger doesn't feed back into the profile's XP, attributes, or seal chain.

The Card/Deck/Bag nomenclature (ADR-010) already provides the taxonomy. This ADR applies it to routine management: modules are Cards, composed routines are Decks, the user's active collection is their Bag.

---

## Decision

### 1. Introduce `MODULE_REGISTRY` and `STARTER_DECKS` alongside `TEMPLATE_PROFILES`

`TEMPLATE_PROFILES` is **preserved unchanged** as the alias identity pool (ADR-003 alias codes, ADR-005 session modes, `.ferros-log` claim flow all depend on it). It is no longer the primary Stage 1 selection UX — that role moves to the module system.

Two new JS constants are added to `personal-profile.html`:

- `MODULE_REGISTRY` — array of routine module Cards (atomic and compound)
- `STARTER_DECKS` — array of archetype starter Decks referencing modules by id

### 2. Three-Tier Card Taxonomy (per ADR-010)

| Kind | ADR-010 Role | Description |
|------|-------------|-------------|
| `routine-module` | Card | Atomic single-purpose block (e.g., "Cold Shower Protocol" — 1 task, 5 min) |
| `routine-compound` | Card | Bundles 2–3 atomic modules by reference (e.g., "Stoic Morning" = meditation + journaling + cold shower) |
| `routine-deck` | Deck | Archetype starter pack referencing Cards by id (e.g., "Dawn Commander"). NOT a module — a composition. |

### 3. Module Card Schema

Every module satisfies ADR-010 Card identity (`id`, `kind`, `name`, `icon`) plus module-specific fields:

```javascript
{
  // ── ADR-010 Card Identity (required) ──
  id: "dawn-athletics",           // URL-safe slug, unique
  kind: "routine-module",         // "routine-module" | "routine-compound"
  name: "Dawn Athletics",         // Human-readable label
  icon: "🏃",                     // Emoji or image ref

  // ── Module Fields ──
  category: "fitness",            // fitness | mind | hygiene | nutrition | work | social | rest | creative
  archetype: ["early-bird"],      // Archetypes this fits (array — can fit multiple)
  description: "High-intensity workout designed for early risers",
  timeConstraint: {
    anchor: "wake",               // "wake" | "sleep" | "fixed" | "relative"
    offsetMinutes: 30,            // Minutes after anchor point
    durationMinutes: 60,          // Total block duration
    flexibility: "compressible"   // "rigid" | "compressible" | "movable"
  },
  tasks: [
    { id: "warmup",   name: "Dynamic Warmup",      minutes: 10, optional: false },
    { id: "strength", name: "Strength Circuit",     minutes: 35, optional: false },
    { id: "cooldown", name: "Stretch & Cool Down",  minutes: 15, optional: true  }
  ],
  conflicts: ["late-night-deep-work"],    // Module ids known to be incompatible
  synergies: ["cold-shower-protocol"],    // Module ids that pair well
  tags: ["morning", "physical", "high-intensity"],
  origin: "curated",                      // "curated" | "community" | "custom"

  // ── Compound-only (kind: "routine-compound") ──
  modules: []                             // Array of atomic module ids this bundles
}
```

### 4. Archetype Starter Deck Schema

```javascript
{
  id: "dawn-commander",
  kind: "routine-deck",
  name: "Dawn Commander",
  icon: "🌅",
  archetype: "early-bird",
  description: "4–5 AM wake → workout → cold shower → deep work → structured day",
  cards: ["dawn-athletics", "cold-shower-protocol", "deep-focus-sprint", "morning-meditation"],
  defaultWakeTime: "04:30",
  defaultSleepTime: "21:00"
}
```

### 5. Stream ↔ Attribute Mapping (Option C: Display Layer Only)

The existing 3-stream model (A/B/C) remains the canonical storage model. D&D-style attributes are a **display-layer computation** derived from category XP totals at render time. No existing `streamAffinity` values change.

| Display Attribute | Module Categories | Storage Stream |
|-------------------|-------------------|----------------|
| STR (Strength) | fitness | C — Physical & Home Systems |
| CON (Constitution) | hygiene, nutrition, rest | C — Physical & Home Systems |
| INT (Intelligence) | mind, work | A — Governance & AI Backbone |
| WIS (Wisdom) | work (deep), creative | A — Governance & AI Backbone |
| CHA (Charisma) | social | B — Home Interaction Layer |
| DEX (Dexterity) | creative, fitness (agility) | B — Home Interaction Layer |

### 6. Composition Engine — `composeSchedule()`

Pure JavaScript, embedded inline in `personal-profile.html`. Zero external dependencies. No `crypto.subtle` usage (no hashing in composition). `file://` safe.

**Algorithm:**
1. Resolve compound Cards: expand `modules[]` references to their atomic modules
2. Anchor `fixed`-time modules to their clock time
3. Place `wake`/`sleep`-anchored modules relative to user's declared times
4. Fill remaining with `movable` modules, respecting duration
5. Detect conflicts: overlapping time slots + `conflicts[]` array pairs
6. Propose resolution: shift, compress, or prompt user to choose
7. Suggest synergies: surface unselected modules from each selected module's `synergies[]`
8. Auto-bundle: adjacent atomics sharing a category get grouped into a compound protocol

**Night Owl Compression:**
- Available hours = `sleepTime - wakeTime`
- If total module duration exceeds available hours, `flexibility: "compressible"` modules shrink proportionally
- `flexibility: "rigid"` modules never shrink
- System warns if still over-subscribed after compression

**ADR-008 Extraction Threshold:** If `composeSchedule()` and module registry logic exceed ~500 lines, extract to `docs/assets/_lib/_schedule-composer.js` per ADR-008 rules. The `@inline` marker ensures the asset remains a standalone working HTML file.

### 7. Extended Profile Schema (V2)

All new data stored inside the existing `ferros_profile` localStorage key. No new top-level keys. `saveProfile()` remains the single write point (non-negotiable per AGENT_GUIDE).

```javascript
ferros_profile: {
  metadata: {
    version: "2.0",
    schemaVersion: 2,          // ← NEW: migration detection
    created: "ISO-8601",
    lastModified: "ISO-8601"
  },
  character: {
    name, avatar, class,
    archetype,                 // ← NEW: "early-bird" | "night-owl" | "shift-worker" | "flex" | "structured"
    streamAffinity, stage, assistLevel, joined,
    wakeTime,                  // ← NEW: "HH:MM" string
    sleepTime,                 // ← NEW: "HH:MM" string
    categoryInterests: []      // ← NEW: subset of 8 categories
  },
  progression: { xp, level, seals, streak },
  achievements: [...],
  sealChain: [...],
  schedule: {                  // ← NEW: output of composeSchedule()
    archetype,
    activeDeck,                // deck id or null if custom
    wakeTime, sleepTime,
    slots: [
      { moduleId, startTime, endTime, compressed }
    ],
    unresolvedConflicts: []
  },
  completions: {               // ← NEW: moved from implicit tracking
    "2026-04-08:dawn-athletics:warmup": { done: true, ts: "ISO-8601" }
  },
  creditLog: {                 // ← NEW: batched credit events
    "2026-04-08": {
      credits: [
        { action: "complete-task", module: "dawn-athletics", task: "warmup", xp: 10, category: "fitness" }
      ],
      totalXP: 40,
      sealIndex: 42            // references sealChain[42]
    }
  },
  bag: [                       // ← NEW: Card/Deck inventory (ADR-010)
    { id: "dawn-athletics", kind: "routine-module", acquiredAt: "ISO-8601" },
    { id: "fitness-initiate", kind: "credential", earnedAt: "ISO-8601" }
  ],
  template: { ... },           // PRESERVED: alias compatibility
  journal: [...]
}
```

### 8. `saveProfile()` Compliance

All writes — from both `personal-profile.html` and `schedule-ledger.html` — flow through `saveProfile()` or a wrapper that terminates by calling `saveProfile()`.

**`saveLedgerCompletion()` in schedule-ledger.html:**
1. Mutates the in-memory `profile` object (`completions` and `creditLog` fields ONLY)
2. Calls `saveProfile()` as its final action — NEVER calls `localStorage.setItem()` directly
3. The existing `saveProfile()` guard catches all non-write conditions: `if (sessionMode || aliasMode || recoveryMode) return;`

### 9. Migration: `migrateProfileV1toV2()`

**Trigger:** Runs at page load, after `ferros_profile` is read from localStorage but BEFORE `showStage()` is called. If `profile.metadata.schemaVersion` is absent or < 2, migration runs.

**Steps:**
1. Assign default archetype based on existing `class` + `template.id`'s `scheduleArchetype`
2. Map existing `template.scheduleBlocks` into `schedule.slots`
3. Initialize empty `completions`, `creditLog`, `bag`
4. Seed `bag` with starter modules matching their template's schedule blocks
5. Set `metadata.schemaVersion = 2`
6. Save via `saveProfile()`

**UX:** No modal, no re-consent, no re-onboarding. User sees their existing dashboard with V2 fields populated from defaults. Stage is preserved.

### 10. Cross-Page Data Contract

| Page | Read | Write |
|------|------|-------|
| `personal-profile.html` | Full `ferros_profile` | Full `ferros_profile` via `saveProfile()` |
| `schedule-ledger.html` | Full `ferros_profile` (read-only for display) | `completions` + `creditLog` fields ONLY, via `saveLedgerCompletion()` → `saveProfile()` |
| Other pages | Full `ferros_profile` (read-only) | Never |

### 11. Credit Batching Rule

Credits batch into one `creditLog` entry per day (or per session). One seal per batch, not per micro-action. Estimated ~365 seals/year — well within localStorage budget.

### 12. Celebrity Name Abstraction

Module names are fully abstracted. No real-person references in `MODULE_REGISTRY`:
- "Obama's Workout" → "Executive Athletics"
- "Buffett's Reading" → "Market Intelligence Hour"
- "Aurelius Journaling" → "Stoic Reflection"

The celebrity **identities** persist only in `TEMPLATE_PROFILES` as the alias system requires.

---

## Five Archetypes

| Archetype | Slug | Description | Wake Pattern |
|-----------|------|-------------|-------------|
| Early Bird | `early-bird` | 4–5 AM wake, front-loaded productivity, early wind-down | Fixed early |
| Night Owl | `night-owl` | No set wake, day compresses around available hours, evening creative peak | Anchor: wake (late) |
| Shift Worker | `shift-worker` | Rotating schedules, sleep anchors shift dynamically | Variable |
| Flex | `flex` | Minimal time anchors, modules float to available slots | Movable |
| Structured | `structured` | Every hour accounted for, rigid blocks, military precision | Fixed, full day |

---

## D&D Metaphor Mapping

| D&D Concept | FERROS Equivalent | Storage Location |
|-------------|-------------------|-----------------|
| Race | Archetype | `character.archetype` |
| Class | Class (Architect, Engineer, Guardian, Scholar, Artisan, Healer, Guided, Community) | `character.class` |
| Abilities | 6 attributes (STR/INT/WIS/CHA/CON/DEX) | Computed from category XP at render time |
| Skills | Tracked habits within modules | `schedule.slots[].tasks` |
| Feats | Compound modules / achievements | Bag entries with `kind: "routine-compound"` |
| Hit Points | Streak / consistency score | `progression.streak` |
| Experience | XP from completed tasks | `progression.xp` |
| Level | Progression through seal chain | `progression.level` |
| Quest Log | Active scheduled modules | `schedule.slots` |
| Character Sheet | Stage 3 dashboard | Personal Profile |
| Inventory / Bag | Modules, achievements, credentials | `bag[]` |

---

## Stage Flow Changes

### Stage 1 (Character Creation) — Refocused

- **Primary UX:** Archetype selector (5 cards) → Class picker (8 options, preserved) → Avatar & name (preserved) → Wake/sleep time (new) → Category interests (new multi-select from 8 categories)
- **Secondary:** "Browse Aliases" panel (celebrity gallery, accessible but not the main flow). `TEMPLATE_PROFILES` gallery moves here.

### Stage 2 (First Protocol) → Module Composition

Replaces the current checklist/form with a module composition workflow:
1. System recommends starter Deck or ~5 modules based on archetype + category interests
2. User browses full module library (filtered), adds/removes
3. `composeSchedule()` arranges into day-view timeline, highlights conflicts
4. User resolves conflicts
5. "Forge Your Protocol" saves to `ferros_profile.schedule`

**Assist Level Interaction (ADR-006):**
- Level 1 (Guided): Sees starter Decks only. Single "Pick a Deck" step.
- Level 2: Compounds + individual atomics within a deck.
- Level 3–4: Full atomic composition, manual scheduling, conflict resolution.

### Stage 3 (Dashboard) — Character Sheet + Daily Play

- **Left:** Character sheet, attribute radar (computed), level, streak
- **Center:** Quest log (today's modules as checkbox quests), next 3 uncompleted
- **Right:** Achievements, Bag (module/credential Cards), journal
- **Bottom dock (ADR-009):** Quick-complete bar

---

## Consequences

### Positive
- Users compose lifestyle algorithms from modular blocks instead of forking monolithic celebrity templates
- Night-owl compression handles variable wake times naturally
- D&D metaphor provides familiar, gamified framing for habit tracking
- 8 module categories map cleanly onto existing 3-stream model via display-layer attributes
- Credit batching keeps seal chain growth at ~1/day, well within localStorage limits
- `TEMPLATE_PROFILES` alias codes preserved — zero breakage for ADR-003/005

### Negative
- `MODULE_REGISTRY` + `STARTER_DECKS` add ~5–8 KB to `personal-profile.html` (acceptable given current file is ~3291 lines)
- Composition engine adds algorithmic complexity inline — monitor against ADR-008 extraction threshold
- Two distinct data structures (`TEMPLATE_PROFILES` for aliases, `MODULE_REGISTRY` for routines) may cause confusion — agent guide must document clearly

### Out of Scope (Future)
- User-created custom modules (`origin: "custom"`) — requires persistence + sharing infrastructure
- Module marketplace — requires cross-network sync (Phase 2+)
- AI-powered DM / NPC simulation — requires agent integration
- On-chain credential anchoring — deferred per ADR-002 boundaries
- `schedule-ledger.html` Editor integration ("Import from Profile") — separate PR

---

## Related

- [ADR-004: Template Profile Specification](./ADR-004-template-profile-specification.md) — Superseded for routine selection; preserved as alias identity pool
- [ADR-010: Cards and Decks Nomenclature](./ADR-010-cards-and-decks-nomenclature.md) — Module = Card, Routine = Deck, Bag = inventory
- [ADR-008: Modular Rendering System](./ADR-008-modular-rendering-system.md) — Extraction threshold for composition engine
- [ADR-006: Level Zero Adaptive Onboarding](./ADR-006-level-zero-adaptive-onboarding.md) — Assist-level interaction with module complexity
- [ADR-003: The Alias System](./ADR-003-alias-system.md) — Celebrity templates persist for alias codes
- [ADR-005: Cross-Device Identity and Session Modes](./ADR-005-cross-device-identity-and-session-modes.md) — `saveProfile()` guard compliance
- [ADR-001: Progression-Lock Pattern](./ADR-001-progression-lock-pattern.md) — Credit batching into seal chain
- [ADR-009: Four-Corner Docking Layout](./ADR-009-four-corner-docking-layout.md) — Stage 3 bottom dock
