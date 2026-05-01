# ADR-026 — AI Assistant Card/Deck IDE and HUD Workbench

**Status:** Proposed  
**Date:** 2026-05-01  
**Stream:** S3 / S5 / S6 / S8 / Cross-cutting  
**Deciders:** Maangled  
**Domain tags:** UX doctrine / architecture / runtime / governance / research / cross-cutting  
**Primary evidence basis:** Analytical proof, with implementation proof required before Accepted

_See [ADR-022-decision-program-governance.md](./ADR-022-decision-program-governance.md), [_INDEX.md](./_INDEX.md), and [../../DOCTRINE.md](../../DOCTRINE.md). Cross-reference ADR-009 for four-corner docking, ADR-010 for Card/Deck/Bag nomenclature, ADR-015 for Forge as the universal workbench pattern, ADR-017 for HTML surface incubation, and C8 (`docs/contracts/runtime-host-v1.md`) for iframe/runtime seams._

> **Note:** This ADR defines a proposed UX and architectural direction. It does not claim that a production IDE exists yet. Implementation work should start as an incubated HTML surface and graduate only after it proves code indexing, assistant proposal flow, sandbox execution, and documentation projection on real FERROS repo artifacts.

---

## Context

FERROS is entering a hardware-testing phase: an x86_64 Linux server is available, and upcoming work needs the profile, Forge, and Home Hub surfaces to become easier to inspect, explain, test, and extend. The repo already has a strong workbench direction: Forge uses the four-corner docking layout from ADR-009, the Card/Deck/Bag vocabulary from ADR-010, the runtime-host iframe seam from C8, and the universal parametric authoring doctrine from ADR-015. What remains under-specified is the broader HUD vision: how FERROS should let a human programmer, an AI assistant, and future VR/AR-style surfaces understand the system together without falling back to a traditional file explorer with scattered tool panes.

Traditional IDEs center files and folders. FERROS needs an IDE that centers meaning, consent, runtime behavior, and composable work. The desired surface is a browser-native workbench where code is represented as Cards at multiple abstraction levels, active work is shaped into Decks, relationships become an explorable system map, and testing/learning happen in a sandbox Arena. This surface should be a triple threat:

1. **IDE** — a productive place to inspect, edit, run, test, and review FERROS itself.
2. **Docs system** — a live documentation browser where every code object, ADR, schema, contract, surface, stream, and hardware target has a Card and a relationship map.
3. **Game-like creation environment** — a deck-building and card-playing atmosphere where new code is proposed, assembled, tested, challenged, taught, and played through the system.

The intent is not novelty for its own sake. The goal is to make FERROS visual, teachable, auditable, and locally sovereign while preserving the consent-first rule already present in Forge and Builder Blueprint.

---

## Decision

FERROS will adopt a **browser-native, AI-assistant-mediated Card/Deck IDE** as the canonical developer HUD pattern for working on the platform.

This IDE will treat code, docs, tests, schemas, runtime events, hardware lanes, and assistant proposals as FERROS Cards. Human work sessions become Decks. The central viewport becomes a live system map and sandbox Arena. The assistant lives inside the four-corner HUD, but it may only propose changes until the user explicitly allows them.

The working name for the surface is **FERROS Code Deck IDE**. The product description is:

> A mighty browser with a deck: a clean-code sunset on traditional explorers, where the programmer plays FERROS by building, inspecting, testing, and teaching with Cards.

---

## UX doctrine: four corners, four walls, one center Arena

### Four-corner HUD inheritance

The Code Deck IDE must inherit the ADR-009 four-corner/four-edge shell rather than inventing a new layout system.

Default Code Deck IDE corner assignments:

| Zone | Panel | Purpose |
|------|-------|---------|
| TL | **Code Bag** | Search, filter, and browse Cards representing files, symbols, schemas, ADRs, tests, surfaces, streams, hardware roots, and open issues. |
| TR | **Inspector / Docs Lens** | Show selected Card metadata, docs, schema fields, call sites, references, history, ownership, risk, tests, and related ADRs. |
| BL | **Tools / Moves** | Human-controlled actions: inspect, map, run, test, diff, refactor, stage, revert, explain, challenge, export, and open sandbox. |
| BR | **Assistant / Tutor / Referee** | AI pair assistant, teaching bot, review opponent, proposal generator, and consent gate. |

Default edge assignments:

| Edge | Strip | Purpose |
|------|-------|---------|
| Top | **Orientation rail** | Repo, branch, stream, hardware target, session mode, identity/grant status, current Deck, run state, and evidence level. |
| Right | **Context rail** | Lens switching: code, docs, tests, runtime, governance, hardware, data, security, teaching. |
| Bottom | **Hand / Deck tray** | Current work deck, selected cards, queued moves, test cards, challenge cards, and staged proposals. |
| Left | **Hotkey / tool rail** | Fast access to common moves and spatial navigation modes. |

### Four-wall spatial model

ADR-009 gives FERROS a 2D browser layout. The Code Deck IDE extends that layout into the **four-wall HUD metaphor** used by VR/AR-inspired surfaces. The walls are not separate applications; they are spatial meanings that can be rendered in a browser today and translated into immersive surfaces later.

| Wall | Browser equivalent | Spatial meaning |
|------|--------------------|-----------------|
| **North wall** | Top rail + top map band | Orientation: where am I, what stream/root/branch am I in, what is true, what is running? |
| **East wall** | Right inspector/context side | Understanding: what does this Card mean, what depends on it, what docs govern it? |
| **South wall** | Bottom hand/deck tray | Play: what am I holding, what move is next, what proposal is staged? |
| **West wall** | Left tool/action side | Action: what tools or moves are available, what can I run, test, diff, or challenge? |

The center remains the **Arena**: a large, mostly unobstructed spatial viewport for the code map, runtime flow, sandbox execution, and learning/battle board. The four walls should feel like a cockpit around the center rather than a file explorer around an editor.

### Visual principles

- **No default tree prison.** A file tree may exist as one lens, but it is not the primary navigation pattern.
- **Cards before files.** The user selects meaningful FERROS entities first; file paths are metadata on Cards.
- **Decks before tabs.** A work session is a curated Deck, not a random pile of open tabs.
- **Center before chrome.** Panels collapse aggressively so the Arena can fill the screen.
- **HUD before dashboard clutter.** Status is compressed into rails, badges, and overlays.
- **Consent before mutation.** The assistant may propose, explain, test, and simulate before approval; it may not silently write.

---

## Card model for code and knowledge

A **Code Card** is the IDE projection of a FERROS entity. Every Card should carry at least the ADR-010 identity shape: `id`, `kind`, `name`, and `icon`. Code Cards may be generated from static analysis, docs, schemas, tests, runtime traces, or user curation.

### Initial Code Card kinds

| Kind | Represents | Example |
|------|------------|---------|
| `file-card` | A source or doc file | `docs/forge-workbench.html` |
| `symbol-card` | A function, type, constant, CSS class, command, or event name | `selectCard()`, `ferros:init`, `.corner-br` |
| `schema-card` | A JSON schema, field, fixture, or contract element | `schemas/card.schema.json`, `kind`, `components[]` |
| `surface-card` | A runnable HTML/UI surface | Forge, Arena, Home HUD, Code Deck IDE |
| `contract-card` | A protocol or interface boundary | C8 runtime-host v1 |
| `test-card` | A test, harness, fixture, check, or expected behavior | import/export round-trip, nonce validation |
| `adr-card` | An ADR or research note | ADR-009, ADR-015, ADR-026 |
| `stream-card` | A stream, lane, or progress surface | S3 agent center, S5 UX, S7 hub |
| `hardware-card` | A board, root, lane, run, or evidence packet | `x86_64/Fastest/S4` |
| `proposal-card` | A staged assistant recommendation | "Add Profile grant lens" |
| `issue-card` | A bug, task, blocker, or question | "Assistant edit bridge not started" |

### Abstraction tiers

The same underlying codebase must be visualized at different abstraction levels for different users and tasks.

| Tier | Scope | Use case |
|------|-------|----------|
| **Atom** | Symbol, line range, CSS class, event type, JSON field | Debugging, targeted edits, teaching syntax, local reasoning |
| **Cell** | Function, component, test case, schema object | Refactoring, docs, explainability, review |
| **Organ** | File, module, surface, fixture, contract | Navigation, ownership, dependency mapping |
| **System** | Feature, stream, product surface, hardware root, run pipeline | Planning, onboarding, cross-stream review |
| **Doctrine** | ADR, policy, invariant, evidence class | Governance, safety, claim boundaries, architecture review |

The Inspector must expose a **lens switcher** so the same Card can be viewed as code, docs, tests, runtime behavior, governance, teaching material, or dependency node.

### Card faces

A Code Card should support multiple faces, similar to a trading card with front/back/foil variants:

| Face | Purpose |
|------|---------|
| **Front** | Name, icon, kind, status, risk, owner/stream, summary |
| **Back** | Full docs, path, line ranges, schema fields, related ADRs, examples |
| **Edges** | Incoming/outgoing dependencies, tests, runtime events, hardware relevance |
| **Foil / live face** | Runtime pulse, recent changes, failing tests, assistant warning, evidence status |

Cards are therefore not just visual tiles. They are compact, inspectable, navigable handles into the whole system.

---

## Deck model for work

A **Code Deck** is a curated set of Cards with a purpose, order, constraints, and allowed moves. Decks replace the traditional IDE habit of accumulating arbitrary tabs and panes.

Initial Deck kinds:

| Deck kind | Purpose |
|-----------|---------|
| **Work Deck** | Current implementation task: the Cards needed to make one change. |
| **Review Deck** | Pull request or patch review: changed Cards, impacted tests, related ADRs, risk notes. |
| **Docs Deck** | Learning/explanation path through a subsystem. |
| **Test Deck** | Selected tests, fixtures, harnesses, runtime traces, and expected outcomes. |
| **Forge Deck** | Cards that compose a new asset, surface, or UI prototype. |
| **Hardware Deck** | Server/board/lane Cards needed for a hardware run or evidence packet. |
| **Battle Deck** | Teaching/testing session where the user, assistant, or bot plays moves against code challenges. |
| **Release Deck** | Gate checklist: ADRs, tests, evidence records, docs, changelog, deployment artifacts. |

Decks can be saved as local JSON manifests so sessions are reproducible and teachable. A Deck is both a workspace and a story: what the user was trying to do, which Cards mattered, which moves were played, and what evidence was produced.

---

## Center Arena: map, sandbox, and teaching board

The center viewport is the **Sandbox Arena**. It merges three functions:

1. **Mind map** — show how Cards interconnect.
2. **Sandbox** — stage edits, run tests, simulate runtime events, inspect traces.
3. **Arena** — teach, challenge, and battle code understanding through game-like moves.

### System map

The default Arena view is a live code map. It should support multiple graph lenses:

| Lens | Edges shown |
|------|-------------|
| **Source graph** | imports, includes, script references, CSS token usage |
| **Symbol graph** | function calls, type references, event emit/listen pairs |
| **Contract graph** | schema fields, runtime messages, host/asset boundaries |
| **Surface graph** | Forge → Arena → Home HUD → Profile → Hub relationships |
| **Test graph** | which tests/harnesses/fixtures cover each Card |
| **Governance graph** | ADRs, policies, progress specs, claim boundaries |
| **Hardware graph** | roots, lanes, run targets, service dependencies, evidence packets |
| **Agent graph** | assistant proposals, command-center jobs, consent gates, audit entries |

The graph must support zooming between abstraction tiers. A user can start at "Home Hub" and zoom down to a message type, test fixture, CSS class, or JSON field.

### Live code-flow visualization

When the IDE has runtime information available, Cards should animate flow through the system:

- C8 messages moving from host to iframe asset (`ferros:init`, `ferros:update`) and back (`ferros:event`, `ferros:resize`).
- User actions moving from HUD panel to state mutation to render update.
- Test runs lighting up the Cards they execute.
- Assistant proposals moving from prompt → proposal Card → diff Card → test Card → consent gate → applied change.
- Hardware runs moving from server command → service log → local artifact → evidence Card.

The first implementation may use simulated or static traces. Real trace ingestion is deferred until the Linux server tooling exists.

### Sandbox Arena

The sandbox must let a user test ideas without mutating canonical repo state.

Minimum sandbox moves:

| Move | Meaning |
|------|---------|
| **Play Card** | Place a code/doc/test Card into the Arena. |
| **Link Cards** | Declare or inspect a dependency. |
| **Fork Card** | Create a scratch variant of a Card. |
| **Propose Move** | Ask the assistant to suggest a change. |
| **Challenge Move** | Ask the assistant or bot to attack the proposal with tests, invariants, or ADR constraints. |
| **Run Test Card** | Execute or simulate a relevant test/harness. |
| **Promote** | Turn a sandbox proposal into a staged diff after user approval. |
| **Revert** | Drop staged sandbox state without touching canonical files. |

### Teaching and battle mode

The Arena doubles as a teaching/testing tool. The core analogy is:

> Like playing a bot in chess, but the chess board is Turing complete.

The board is "chess-like" because turns, pieces, legal moves, challenges, and strategy are visible. It is "Turing complete" because the Cards represent real executable code, real contracts, real tests, and real runtime paths. The bot can teach by playing simple moves, review by challenging risky moves, or test by forcing the user to explain why a dependency, schema, or runtime path behaves a certain way.

Battle mode should support:

- **Tutor bot:** teaches a subsystem through guided Card sequences.
- **Reviewer bot:** challenges proposed changes with tests, ADRs, and invariants.
- **Refactor bot:** suggests alternate moves but waits for consent before applying.
- **Regression bot:** tries to find what breaks when a Card changes.
- **Onboarding bot:** gives new contributors a Deck that introduces FERROS by playing through Profile, Forge, Home Hub, and hardware runway Cards.

This mode must remain useful without gamification pressure. Game mechanics are an interface for understanding and agency, not a substitute for correctness.

---

## Assistant role and consent boundary

The assistant is a panel, not the owner of the workspace.

Assistant roles:

| Role | What it may do |
|------|----------------|
| **Cartographer** | Build and explain maps of Cards, Decks, dependencies, docs, and runtime flows. |
| **Pair programmer** | Draft changes, refactors, tests, schemas, and docs as proposals. |
| **Tutor** | Generate teaching Decks and explain Cards at the user's abstraction level. |
| **Referee** | Check proposals against ADRs, contracts, schemas, tests, dependency policy, and claim boundaries. |
| **Opponent** | In battle mode, challenge the user's move and search for bugs or missing evidence. |
| **Historian** | Summarize prior Decks, decisions, runs, and evidence without rewriting them. |

Assistant invariants:

1. The assistant may not silently mutate canonical files, manifests, schemas, or hardware state.
2. Every assistant-generated change must become a `proposal-card` first.
3. A proposal must show at least summary, affected Cards, risk, tests/evidence, and rollback path before promotion.
4. The user must explicitly choose **Allow** before a proposal becomes a staged diff or command.
5. The user must explicitly choose a second action before a staged diff is committed, pushed, or dispatched to hardware.
6. Denied proposals remain useful as teaching artifacts but do not change state.
7. Assistant confidence is never treated as evidence; tests, traces, contracts, and human approval are evidence.

This preserves the consent-first doctrine from existing FERROS workbench surfaces while making AI assistance central to the IDE experience.

---

## Relationship to existing FERROS surfaces

### Forge

Forge remains the authoring surface for visual/parametric assets. The Code Deck IDE is the authoring surface for FERROS code, docs, tests, contracts, and hardware runbooks. They share the same shell pattern and Card/Deck/Bag language.

The IDE may inspect Forge Cards and generate Forge-related docs or tests, but it does not replace Forge.

### Profile

Profile work should appear as identity, grants, consent, session, and template-profile Cards. The IDE should make it easy to see how a profile rule affects assistant permissions, Home Hub behavior, and hardware evidence claims.

### Home Hub

Home Hub work should appear as surface Cards, service Cards, event Cards, hub/device Cards, and hardware lane Cards. The IDE should show how Home Hub connects to S7 evidence, onramp policy, and the future local server runtime.

### Linux server / hardware runway

The Linux server is the first natural backend for the Code Deck IDE. The browser shell can remain local and dependency-light while the server provides optional services:

- source indexing
- symbol graph extraction
- test execution
- sandbox filesystem worktrees
- log/trace ingestion
- local assistant bridge
- hardware run orchestration
- evidence packet generation

These services must be projected into the IDE as Cards and Decks, not hidden as opaque backend magic.

---

## Technical architecture direction

### Surface layer

The initial surface should be an incubated HTML document, likely:

```text
docs/code-deck-ide.html
```

It should follow the same offline constraints as other FERROS HTML surfaces:

- no external dependencies
- no CDN links
- no module imports for the incubated surface
- no mandatory network access for static browsing
- inline CSS/JS or repo-local assets only
- works from `file://` for documentation and mock graph browsing

### Data layer

The first implementation can use generated or hand-authored fixtures:

```text
schemas/code-card.schema.json
schemas/code-deck.schema.json
schemas/fixtures/code-deck-ide/*.json
```

Candidate manifest shapes:

```json
{
  "id": "adr-009",
  "kind": "adr-card",
  "name": "Four-Corner Docking Layout",
  "icon": "🧭",
  "path": "docs/adr/ADR-009-four-corner-docking-layout.md",
  "tier": "doctrine",
  "tags": ["UX doctrine", "layout"],
  "edges": [
    { "type": "governs", "to": "forge-workbench" },
    { "type": "governs", "to": "code-deck-ide" }
  ]
}
```

```json
{
  "id": "hardware-server-runway",
  "kind": "hardware-deck",
  "name": "x86_64 Server Hardware Testing Deck",
  "cards": ["x86_64-fastest-s1", "profile-grants", "forge-workbench", "home-hub", "evidence-packet"],
  "allowedMoves": ["inspect", "run", "test", "record-evidence", "propose"],
  "status": "draft"
}
```

These schemas are deferred implementation details; this ADR freezes the direction, not exact field names.

### Indexing layer

The IDE needs a source-to-Card projection pipeline. It may begin as static fixtures, then move to a local server indexer.

Indexing stages:

1. **Docs index** — ADRs, progress specs, contracts, stream docs, schemas.
2. **Surface index** — HTML surfaces, CSS tokens/classes, inline scripts, runtime targets.
3. **Schema index** — JSON schemas, fixtures, manifest examples, field-level relationships.
4. **Symbol index** — functions, constants, events, commands, Rust modules, tests.
5. **Runtime index** — traces, logs, test results, C8 messages, server events.
6. **Hardware index** — board roots, runbooks, findings, evidence packets.

### Runtime seam

Where the IDE previews visual surfaces, it should use the existing C8 iframe/runtime-host seam. The central Arena may host:

- Forge previews
- Card/deck render targets
- Home Hub prototype panes
- code-flow visualizations
- sandbox result viewers
- teaching boards

The Code Deck IDE should treat these viewers as Cards hosted in the Arena, not as privileged one-off panes.

### Server bridge

A server bridge is allowed for real development tasks. It must be explicit and inspectable.

Minimum bridge boundaries:

| Capability | Constraint |
|------------|------------|
| Read repo | Allowed after user grants local path/worktree access. |
| Index code | Allowed; produces Card/Deck fixtures and graph edges. |
| Run tests | Allowed through explicit user action or approved Deck move. |
| Create scratch worktree | Allowed for sandboxing; canonical branch untouched until user promotes. |
| Write files | Requires proposal + Allow + staged diff confirmation. |
| Dispatch hardware command | Requires proposal + Allow + target confirmation + evidence logging. |
| Persist history | Must be local-first and auditable. |

This bridge belongs naturally to the x86_64 Fastest runway at first, but the UI must not claim FERROS-native OS behavior merely because the bridge runs on Linux.

---

## Rationale

### Why a card/deck IDE?

FERROS already speaks Card/Deck/Bag across assets, Forge, Arena, inventory, and routines. Extending that model to code makes the system self-similar: the tools used to build FERROS follow the same metaphors as the product surfaces they produce.

This has practical advantages:

- Human programmers can see code as objects with identity, state, relationships, and evidence.
- The assistant has stable handles for explanation and proposals instead of vague file blobs.
- Documentation stays connected to the exact Cards it governs.
- Tests and runtime traces become visible parts of the workspace.
- Hardware evidence can be linked to the code, docs, and claims it supports.
- New contributors can learn by playing through curated Decks instead of reading the repo linearly.

### Why a browser-native workbench?

FERROS already uses browser surfaces as incubators. A browser-native IDE can prototype the HUD vision quickly, open locally, reuse the Forge shell, and eventually bridge to a Linux server or Rust-native renderer without making the early UX wait for the final runtime stack.

### Why keep game mechanics?

The game metaphor is not decoration. Deck building, hands, moves, arenas, battles, and bots give the system a vocabulary for controlled action:

- A **Card** is a thing.
- A **Deck** is an intention.
- A **Move** is an operation.
- An **Arena** is a safe place to test consequences.
- A **Bot** is a tutor/reviewer/opponent.
- A **Win condition** is passing tests, satisfying ADR constraints, preserving consent, and producing evidence.

This makes complex software work visible and teachable without reducing it to a file tree.

---

## Options considered

| Option | Summary | Reason not chosen |
|--------|---------|-------------------|
| Traditional IDE clone | File explorer, editor tabs, terminal, assistant sidebar | Rejected because it does not express FERROS Card/Deck doctrine, HUD vision, or teaching/sandbox needs. |
| Docs-only code browser | Static documentation graph without edit/test/sandbox moves | Rejected because it cannot serve as the IDE or game-like creation surface. |
| Game-only coding arena | Card battles and teaching mode without serious code editing and evidence flow | Rejected because FERROS needs a real programming tool, not only a metaphor. |
| Assistant-first chat IDE | Chat controls the repo and surfaces are secondary | Rejected because it violates the desired spatial HUD model and risks weakening consent boundaries. |
| Browser-native Card/Deck IDE (chosen) | Four-corner HUD, Code Cards, Deck tray, system map, sandbox Arena, assistant as proposal/tutor/referee | Chosen because it unifies IDE, docs, and game-like teaching while extending existing FERROS workbench doctrine. |

---

## Consequences

### Positive

- The UX vision becomes concrete: four corners, four walls, central Arena, Cards, Decks, moves, and consent gates.
- FERROS gains a self-hosting story: the system can be worked on through the same Card/Deck metaphors that it exposes to users.
- Documentation, tests, ADRs, hardware lanes, and code can become one navigable graph instead of separate silos.
- The assistant gets a constrained role with visible proposals and reviewable move history.
- New contributors can learn FERROS by playing curated Decks and fighting review bots in a safe sandbox.
- Hardware testing gains a visual planning/evidence surface rather than becoming detached shell scripts and logs.

### Negative / trade-offs

- A good Code Card projection pipeline is non-trivial. Poor indexing would make the UX feel decorative instead of useful.
- The card/deck metaphor may become confusing if every object is over-cardified without careful abstraction tiers.
- Graph visualization can become noisy; lens discipline is mandatory.
- The sandbox/server bridge introduces real permissions and safety concerns that the static HTML incubator does not have.
- Battle/teaching mode must avoid becoming a gimmick; it must stay tied to tests, contracts, and evidence.
- The IDE could tempt premature claims about AI automation. The assistant must remain proposal-bound until evidence and consent requirements are met.

---

## Compliance

This ADR remains valid only if the following stay true:

- The Code Deck IDE follows ADR-009's four-corner/four-edge shell or explicitly justifies any divergence in a later ADR.
- Code, docs, tests, schemas, surfaces, hardware lanes, and proposals are represented through Card/Deck vocabulary consistent with ADR-010.
- Domain-specific IDE behavior remains a specialization of the universal workbench direction in ADR-015, not a separate UX architecture.
- Any visual surface incubator follows the dependency and offline constraints governing current FERROS HTML surfaces.
- Assistant actions remain proposal-first and consent-gated before canonical mutation.
- The sandbox boundary remains explicit: scratch state is not canonical state.
- Server or hardware commands require a visible target, explicit approval, and evidence/audit output.
- Graph edges and runtime animations must distinguish observed evidence from inferred or simulated relationships.
- Claims about hardware, Home Hub, Profile, or FERROS-native OS status must follow the relevant ADR and evidence boundaries.

Revisit this ADR if:

- The Card/Deck metaphor proves insufficient for code navigation after implementation proof.
- Graph/lens complexity overwhelms users and a simpler spatial model emerges.
- A future native FERROS surface replaces browser incubation as the primary HUD layer.
- The assistant permission model changes materially.
- A real hardware/testing pipeline reveals missing safety boundaries for sandbox, server, or command execution.

---

## Implementation evidence required before Accepted

This ADR should remain Proposed until at least one incubated surface proves the pattern with real repo artifacts.

Minimum acceptance evidence:

1. `docs/code-deck-ide.html` or successor loads locally and renders the four-corner/four-wall HUD.
2. The Code Bag shows at least ADR, schema, surface, test/fixture, and stream Cards.
3. The Inspector shows at least two Card faces: summary and related docs/edges.
4. The Arena renders a graph or map with real relationships from the repo.
5. The Deck tray can save/load a local Work Deck fixture.
6. The assistant panel can create a proposal Card without applying changes.
7. The sandbox can simulate or run at least one test/check and show evidence on related Cards.
8. The implementation documents which graph edges are static, inferred, simulated, or observed.

Optional stronger evidence:

- Linux server bridge produces Cards from a real source index.
- Assistant edit bridge produces staged diffs only after approval.
- Battle mode teaches a real FERROS subsystem through a curated Deck.
- Hardware runway Deck records a server run or evidence packet without overclaiming physical-device proof.

---

## Deferred scope or open research

- Exact `code-card.schema.json` and `code-deck.schema.json` field definitions.
- Which parser/indexer stack should produce symbol-level Cards for Rust, HTML, CSS, and JS.
- Whether graph layout is hand-authored, static-generated, force-directed, hierarchical, or renderer-assisted.
- How to preserve line-range accuracy across edits.
- How sandbox worktrees, diffs, and rollback are represented in manifests.
- How battle mode should score moves without creating misleading incentives.
- How the IDE should interoperate with existing editors during the transition.
- Whether future VR/AR surfaces should render the four walls literally or keep them as semantic zones.
- How much of the server bridge belongs in `ferros-core`, `ferros-surfaces`, or hardware roots.

---

## Follow-up work

| Item | Owner track | Priority |
|------|-------------|----------|
| Create `docs/code-deck-ide.html` as the first incubated HUD prototype | S5 UX / S8 docs | Near-term |
| Define seed fixtures for ADR, schema, surface, stream, and hardware Cards | S6 data / S8 docs | Near-term |
| Draft `schemas/code-card.schema.json` and `schemas/code-deck.schema.json` | S6 data | Near-term |
| Add a Code Deck IDE progress spec under `docs/progress/` | S8 docs | Near-term |
| Prototype graph lenses from static repo metadata | S5 UX / S6 data | Near-term |
| Connect proposal Cards to assistant edit bridge work | S3 agent center / S5 UX | Mid-term |
| Add server-backed indexing and test execution on the x86_64 Fastest root | S1 / S3 / S4 / S7 | Mid-term |
| Build teaching/battle Decks for Profile, Forge, Home Hub, and hardware runway onboarding | S5 UX / S8 docs | Mid-term |

---

## References

- [ADR-009](./ADR-009-four-corner-docking-layout.md) — four-corner docking layout and edge/corner semantics.
- [ADR-010](./ADR-010-cards-and-decks-nomenclature.md) — Card/Deck/Bag vocabulary.
- [ADR-015](./ADR-015-universal-parametric-authoring-workbench.md) — Forge as the universal parametric authoring workbench pattern.
- [ADR-017](./ADR-017-html-surface-incubation-strategy.md) — HTML surfaces as incubators.
- [ADR-021](./ADR-021-dependency-admission-policy.md) — dependency posture for browser and Rust surfaces.
- [ADR-022](./ADR-022-decision-program-governance.md) — ADR taxonomy, statuses, and evidence basis rules.
- [ADR-025](./ADR-025-dual-root-hardware-runway.md) — hardware runway and server/hardware evidence posture.
- [Runtime Host Contract v1](../contracts/runtime-host-v1.md) — iframe/postMessage seam used by Forge, Arena, Home HUD, and future embedder surfaces.
- [The Forge progress spec](../progress/forge.md) — current Forge status and milestone gates.
- [Builder Blueprint](../builder-blueprint.md) — domain-specialized workbench direction and Twin Architecture framing.
- [_RESEARCH-NOTES/RN-2026-04-acc-card-deck-projection.md](./_RESEARCH-NOTES/RN-2026-04-acc-card-deck-projection.md) — related card/deck projection research lane.
