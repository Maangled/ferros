# ADR-004: Template Profile Specification

## Status
Accepted

## Context
The Profile Gallery (PR 7) and Template Schedules (PR 8) both need a shared data schema for template/famous profiles. These need to be:
- Tiny enough to hardcode directly into `personal-profile.html` without bloating the file.
- Structured enough to be forward-compatible with ledger storage (when the FERROS distributed ledger exists).
- Expressive enough to seed a meaningful alias session or character creation template.

The file is a single self-contained HTML document (zero external dependencies, works on `file://` protocol). All template data lives as a JS constant in the embedded `<script>` block.

## Decision

### Template Stub Schema

Each template profile is a JSON object with the following fields:

```json
{
  "id": "nikola-tesla",
  "aliasCode": "tesla-a1b2",
  "name": "Nikola Tesla",
  "tagline": "The mind is sharper and keener in seclusion.",
  "avatar": "⚡",
  "class": "Engineer",
  "streamAffinity": "A",
  "scheduleArchetype": "deep-work-nocturnal",
  "fictionalOrReal": "real",
  "hardcodedVersion": "1.0",
  "ledgerPointer": null,
  "templateSchedule": {
    "blocks": [
      { "time": "10:00", "name": "Correspondence & Reading", "stream": "A", "durationMin": 60 },
      { "time": "11:00", "name": "Deep Research", "stream": "A", "durationMin": 240 },
      { "time": "15:00", "name": "Laboratory Work", "stream": "C", "durationMin": 180 },
      { "time": "18:00", "name": "Walk & Observation", "stream": "B", "durationMin": 60 },
      { "time": "21:00", "name": "Writing & Invention Notes", "stream": "A", "durationMin": 120 }
    ]
  },
  "description": "Inventor and electrical engineer known for obsessive deep work, nocturnal habits, and radical creative solitude."
}
```

### Field Definitions

| Field | Type | Required | Notes |
|---|---|---|---|
| `id` | string | ✅ | URL-safe slug, unique per template |
| `aliasCode` | string | ✅ | `{first-word}-{4-char hex stub}` — used as alias session identifier |
| `name` | string | ✅ | Display name |
| `tagline` | string | ✅ | Short quote or descriptor shown in gallery card |
| `avatar` | string | ✅ | Single emoji — used as the avatar in alias session |
| `class` | string | ✅ | One of: Architect, Engineer, Guardian, Scholar, Artisan, Healer, Guided, Community |
| `streamAffinity` | string | ✅ | One of: A, B, C |
| `scheduleArchetype` | string | ✅ | Slug describing the schedule pattern (e.g. `deep-work-nocturnal`, `9-5-structured`, `flexible-shift`) |
| `fictionalOrReal` | string | ✅ | `"real"` or `"fictional"` |
| `hardcodedVersion` | string | ✅ | Semver string — bumped when the stub is updated |
| `ledgerPointer` | string\|null | ✅ | `null` until the FERROS ledger exists; will hold a ledger transaction hash or IPFS CID |
| `templateSchedule` | object | ✅ | Contains `blocks` array with `time`, `name`, `stream`, `durationMin` |
| `description` | string | ✅ | 1-2 sentence bio shown in gallery preview modal |

### The 8 Hardcoded Templates

| id | Name | Class | Stream | Schedule Archetype | Real/Fictional |
|---|---|---|---|---|---|
| `nikola-tesla` | Nikola Tesla | Engineer | A | `deep-work-nocturnal` | real |
| `frida-kahlo` | Frida Kahlo | Artisan | B | `pain-driven-creative` | real |
| `marie-curie` | Marie Curie | Scholar | A | `structured-research` | real |
| `marcus-aurelius` | Marcus Aurelius | Guardian | A | `stoic-morning-ruler` | real |
| `philip-fry` | Philip J. Fry | Guided | B | `flexible-chaotic` | fictional |
| `florence-nightingale` | Florence Nightingale | Healer | C | `systems-care-rotational` | real |
| `ada-lovelace` | Ada Lovelace | Architect | A | `analytical-visionary` | real |
| `sam-malone` | Sam Malone | Community | B | `service-flexible-shift` | fictional |

### `aliasCode` Generation Rule

The `aliasCode` is deterministic from the template `id`:
- Take the first word of the `id` slug (e.g. `nikola` from `nikola-tesla`).
- Append a 4-character hex string derived from a simple djb2 hash of the full `id`.
- Format: `{first-word}-{4hex}` (e.g. `nikola-50a9`).
- This is hardcoded in the stub — not computed at runtime.

Reference implementation (for verification only, not used at runtime):

```javascript
function aliasCodeFor(id) {
  let hash = 5381;
  for (let i = 0; i < id.length; i++) {
    hash = ((hash << 5) + hash) + id.charCodeAt(i);
    hash = hash & 0xFFFFFFFF;
  }
  const hex4 = (hash >>> 0).toString(16).padStart(8, '0').slice(-4);
  return id.split('-')[0] + '-' + hex4;
}
```

### `TEMPLATE_PROFILES` Constant

Copy this directly into the `<script>` block of `personal-profile.html`:

```javascript
const TEMPLATE_PROFILES = [
  {
    "id": "nikola-tesla",
    "aliasCode": "nikola-50a9",
    "name": "Nikola Tesla",
    "tagline": "The mind is sharper and keener in seclusion.",
    "avatar": "⚡",
    "class": "Engineer",
    "streamAffinity": "A",
    "scheduleArchetype": "deep-work-nocturnal",
    "fictionalOrReal": "real",
    "hardcodedVersion": "1.0",
    "ledgerPointer": null,
    "templateSchedule": {
      "blocks": [
        { "time": "10:00", "name": "Correspondence & Reading", "stream": "A", "durationMin": 60 },
        { "time": "11:00", "name": "Deep Research", "stream": "A", "durationMin": 240 },
        { "time": "15:00", "name": "Laboratory Work", "stream": "C", "durationMin": 180 },
        { "time": "18:00", "name": "Walk & Observation", "stream": "B", "durationMin": 60 },
        { "time": "21:00", "name": "Writing & Invention Notes", "stream": "A", "durationMin": 120 }
      ]
    },
    "description": "Inventor and electrical engineer known for obsessive deep work, nocturnal habits, and radical creative solitude."
  },
  {
    "id": "frida-kahlo",
    "aliasCode": "frida-82a7",
    "name": "Frida Kahlo",
    "tagline": "I paint my own reality.",
    "avatar": "🌺",
    "class": "Artisan",
    "streamAffinity": "B",
    "scheduleArchetype": "pain-driven-creative",
    "fictionalOrReal": "real",
    "hardcodedVersion": "1.0",
    "ledgerPointer": null,
    "templateSchedule": {
      "blocks": [
        { "time": "09:00", "name": "Morning Journaling", "stream": "B", "durationMin": 60 },
        { "time": "10:00", "name": "Studio: Painting Session", "stream": "B", "durationMin": 180 },
        { "time": "13:00", "name": "Rest & Recovery", "stream": "C", "durationMin": 90 },
        { "time": "15:00", "name": "Study & Inspiration", "stream": "A", "durationMin": 60 },
        { "time": "16:00", "name": "Studio: Detail Work", "stream": "B", "durationMin": 120 },
        { "time": "19:00", "name": "Writing & Letters", "stream": "A", "durationMin": 60 }
      ]
    },
    "description": "Mexican painter who channeled chronic pain into raw, surrealist self-portraiture — work sessions structured around her body's limits, not a clock."
  },
  {
    "id": "marie-curie",
    "aliasCode": "marie-4658",
    "name": "Marie Curie",
    "tagline": "Nothing in life is to be feared, only to be understood.",
    "avatar": "⚗️",
    "class": "Scholar",
    "streamAffinity": "A",
    "scheduleArchetype": "structured-research",
    "fictionalOrReal": "real",
    "hardcodedVersion": "1.0",
    "ledgerPointer": null,
    "templateSchedule": {
      "blocks": [
        { "time": "07:00", "name": "Morning Walk & Reflection", "stream": "B", "durationMin": 30 },
        { "time": "08:00", "name": "Laboratory Work", "stream": "A", "durationMin": 240 },
        { "time": "12:00", "name": "Lunch & Rest", "stream": "C", "durationMin": 60 },
        { "time": "13:00", "name": "Data Analysis & Writing", "stream": "A", "durationMin": 180 },
        { "time": "16:00", "name": "Correspondence & Meetings", "stream": "A", "durationMin": 60 },
        { "time": "17:00", "name": "Preparation for Next Day", "stream": "A", "durationMin": 60 }
      ]
    },
    "description": "Two-time Nobel laureate who ran rigorous lab schedules around radioactive research — structured, relentless, and driven by disciplined curiosity."
  },
  {
    "id": "marcus-aurelius",
    "aliasCode": "marcus-3867",
    "name": "Marcus Aurelius",
    "tagline": "You have power over your mind, not outside events.",
    "avatar": "🏛️",
    "class": "Guardian",
    "streamAffinity": "A",
    "scheduleArchetype": "stoic-morning-ruler",
    "fictionalOrReal": "real",
    "hardcodedVersion": "1.0",
    "ledgerPointer": null,
    "templateSchedule": {
      "blocks": [
        { "time": "05:00", "name": "Dawn Reflection & Journaling", "stream": "A", "durationMin": 60 },
        { "time": "06:00", "name": "Physical Training", "stream": "C", "durationMin": 60 },
        { "time": "07:00", "name": "Morning Briefings", "stream": "A", "durationMin": 90 },
        { "time": "09:00", "name": "Administrative Duties", "stream": "A", "durationMin": 180 },
        { "time": "13:00", "name": "Midday Contemplation", "stream": "B", "durationMin": 60 },
        { "time": "14:00", "name": "Military & Civic Affairs", "stream": "A", "durationMin": 180 }
      ]
    },
    "description": "Roman emperor and Stoic philosopher who began every day with written self-examination before turning to the demands of empire."
  },
  {
    "id": "philip-fry",
    "aliasCode": "philip-0869",
    "name": "Philip J. Fry",
    "tagline": "I don't know what I'm doing, but I'm doing it.",
    "avatar": "🛸",
    "class": "Guided",
    "streamAffinity": "B",
    "scheduleArchetype": "flexible-chaotic",
    "fictionalOrReal": "fictional",
    "hardcodedVersion": "1.0",
    "ledgerPointer": null,
    "templateSchedule": {
      "blocks": [
        { "time": "11:00", "name": "Wake Up & Coffee", "stream": "C", "durationMin": 30 },
        { "time": "12:00", "name": "Delivery Route", "stream": "B", "durationMin": 120 },
        { "time": "14:00", "name": "Lunch & Loafing", "stream": "C", "durationMin": 90 },
        { "time": "16:00", "name": "Planet Express Errands", "stream": "B", "durationMin": 60 },
        { "time": "18:00", "name": "Hanging Out", "stream": "B", "durationMin": 60 },
        { "time": "20:00", "name": "TV & Snacks", "stream": "C", "durationMin": 120 }
      ]
    },
    "description": "A 20th-century pizza delivery boy frozen for a thousand years — accidental hero, maximum chaos, zero plan, somehow gets it done."
  },
  {
    "id": "florence-nightingale",
    "aliasCode": "florence-506a",
    "name": "Florence Nightingale",
    "tagline": "I attribute my success to this: I never gave or took any excuse.",
    "avatar": "🕯️",
    "class": "Healer",
    "streamAffinity": "C",
    "scheduleArchetype": "systems-care-rotational",
    "fictionalOrReal": "real",
    "hardcodedVersion": "1.0",
    "ledgerPointer": null,
    "templateSchedule": {
      "blocks": [
        { "time": "06:00", "name": "Ward Rounds & Patient Review", "stream": "C", "durationMin": 90 },
        { "time": "08:00", "name": "Statistical Analysis", "stream": "A", "durationMin": 120 },
        { "time": "10:00", "name": "Staff Briefing & Training", "stream": "A", "durationMin": 60 },
        { "time": "11:00", "name": "Patient Care & Documentation", "stream": "C", "durationMin": 180 },
        { "time": "14:00", "name": "Correspondence & Advocacy", "stream": "A", "durationMin": 90 },
        { "time": "16:00", "name": "Evening Ward Inspection", "stream": "C", "durationMin": 60 }
      ]
    },
    "description": "Pioneer of modern nursing and hospital data visualization who ran wards as tightly as a systems engineer and lobbied governments with statistical charts."
  },
  {
    "id": "ada-lovelace",
    "aliasCode": "ada-3cc3",
    "name": "Ada Lovelace",
    "tagline": "That brain of mine is something more than merely mortal.",
    "avatar": "🔢",
    "class": "Architect",
    "streamAffinity": "A",
    "scheduleArchetype": "analytical-visionary",
    "fictionalOrReal": "real",
    "hardcodedVersion": "1.0",
    "ledgerPointer": null,
    "templateSchedule": {
      "blocks": [
        { "time": "08:00", "name": "Mathematical Study", "stream": "A", "durationMin": 120 },
        { "time": "10:00", "name": "Correspondence & Collaboration", "stream": "A", "durationMin": 60 },
        { "time": "11:00", "name": "Engine Design & Notation", "stream": "A", "durationMin": 180 },
        { "time": "14:00", "name": "Reading & Research", "stream": "A", "durationMin": 60 },
        { "time": "15:00", "name": "Translation & Writing", "stream": "A", "durationMin": 120 },
        { "time": "17:00", "name": "Social Engagements", "stream": "B", "durationMin": 60 }
      ]
    },
    "description": "Mathematician and the world's first programmer — wrote the first algorithm intended for a machine a century before computers existed."
  },
  {
    "id": "sam-malone",
    "aliasCode": "sam-e94f",
    "name": "Sam Malone",
    "tagline": "I'm too cute to be this dumb.",
    "avatar": "🍺",
    "class": "Community",
    "streamAffinity": "B",
    "scheduleArchetype": "service-flexible-shift",
    "fictionalOrReal": "fictional",
    "hardcodedVersion": "1.0",
    "ledgerPointer": null,
    "templateSchedule": {
      "blocks": [
        { "time": "10:00", "name": "Opening the Bar", "stream": "C", "durationMin": 60 },
        { "time": "11:00", "name": "Inventory & Ordering", "stream": "A", "durationMin": 60 },
        { "time": "13:00", "name": "Lunch Rush", "stream": "B", "durationMin": 120 },
        { "time": "16:00", "name": "Downtime & Team Chat", "stream": "B", "durationMin": 60 },
        { "time": "18:00", "name": "Evening Service", "stream": "B", "durationMin": 180 },
        { "time": "21:00", "name": "Wind Down & Close", "stream": "C", "durationMin": 60 }
      ]
    },
    "description": "Former Red Sox pitcher turned Boston bar owner — community anchor, natural connector, and proof that showing up every day is its own kind of discipline."
  }
];
```

### `ledgerPointer` Migration Path

When the FERROS ledger is operational:
1. Each template stub is published to the ledger as a provenance-anchored document (ADR-002 boundary: provenance anchoring use case).
2. The ledger returns a transaction hash or CID.
3. The `ledgerPointer` field is updated in the HTML from `null` to the hash.
4. The gallery can then optionally fetch extended profile data from the ledger pointer, while still falling back to the hardcoded stub if offline.

## Consequences

### Positive
- Stubs are tiny (~300 bytes each × 8 = ~2.4KB total) — negligible file size impact.
- `ledgerPointer: null` is the clean migration hook — no schema changes needed when the ledger arrives.
- The `aliasCode` is stable and human-readable.
- Hardcoded template schedules provide real value as starting templates during character creation (PR 8).

### Negative
- Template schedule content is opinionated/curated — some users may disagree with the archetype mapping.
- `aliasCode` hash is djb2-based (same as the `file://` fallback hash) — not cryptographically strong. Sufficient for identification, not for security.
- `fictionalOrReal` field has no enforcement — purely informational for now.

### Out of Scope
- User-created custom templates (future).
- Template versioning / update notifications (future).
- Fetching extended profile data from `ledgerPointer` (future — requires ledger infrastructure).
- Template "cost" / bounty for alias use (future — see ADR-003 design notes).

---

## Addendum (2026-04-08): Superseded by ADR-011

**ADR-011 (Routine Module System)** supersedes this ADR for routine/schedule selection. The `TEMPLATE_PROFILES` constant and its schema defined above are **preserved as the alias identity pool** — they are NOT removed. Alias codes (`nikola-50a9`, `frida-82a7`, etc.), `.ferros-log` claim flow (ADR-003), and session mode verification (ADR-005) all depend on `TEMPLATE_PROFILES` existing at runtime.

The celebrity gallery is demoted from primary Stage 1 selection UX to a secondary "Browse Aliases" panel. New routine composition uses `MODULE_REGISTRY` and `STARTER_DECKS` (defined in ADR-011), which are separate data structures with no dependency on `TEMPLATE_PROFILES`.

---

## Related
- [ADR-001: Progression-Lock Pattern](./ADR-001-progression-lock-pattern.md)
- [ADR-002: Smart Contract Boundaries](./ADR-002-smart-contract-boundaries.md)
- [ADR-003: The Alias System](./ADR-003-alias-system.md)
- [ADR-011: Routine Module System](./ADR-011-routine-module-system.md) — Supersedes this ADR for routine selection
- FERROS Blueprint: Section 07 — Ledger & Smart Contract Coordination
- PR 7: Profile Gallery
- PR 8: Template Schedules (future)
