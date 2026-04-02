# ADR-007: Single File System (SFS) — From Kernel to Interface in One Artifact

**Status:** Proposed
**Date:** 2026-03-31
**Context:** FERROS architecture — file system philosophy, asset model, agent ergonomics

---

## Context

FERROS is building a Rust-from-scratch operating system (ADR-0001) with a gamified trading
interface prototyped as a single HTML file (`algo-trading-arena.html`, ~2700 lines). As the
prototype matures toward production assets, a fundamental question arises:

**How should the system organize its code, assets, and state?**

The conventional answer is a hierarchical file system with directories, modules, imports,
and build pipelines. But three observations challenge this:

1. **Agent systems degrade with file sprawl.** Modern AI agents (the primary development
   workforce for FERROS) burn context navigating directory trees, resolving imports, and
   maintaining mental models across dozens of files. A 20-file project with clean separation
   of concerns is often *harder* for an agent to reason about than a single 3000-line file
   with a clear internal structure. This is not a temporary limitation — it reflects a
   fundamental tension between human organizational preferences and sequential attention
   windows.

2. **The prototype works.** The single-file HTML architecture (zero dependencies, works on
   `file://`, all CSS/JS inline) has proven remarkably productive. Every asset is visible,
   every relationship is explicit, iteration is instant. The "pitchdeck" is already
   interactive — drag-and-drop, playable games, 3D CSS animations. The architecture's
   constraint (one file) became its strength.

3. **FERROS is building the entire stack.** From bootloader to kernel to userspace to UI,
   FERROS controls every layer. There is no inherited file system to conform to. The
   "file system" is a design decision, not a given — and the conventional hierarchical
   file system (inherited from 1970s Unix) may not be the right one for a system whose
   fundamental unit is a **card**, not a **file**.

---

## Decision

**Adopt the Single File System (SFS) as the organizing principle for FERROS.**

### Core Principles

1. **One artifact, one truth.** The compiled FERROS binary contains everything: kernel,
   drivers, runtime, UI assets, game logic. There is no separate "file system" in the Unix
   sense. The binary IS the system.

2. **Ledger at the top.** The artifact begins with a content-addressed index (the "ledger")
   that maps every internal module, asset, and state block to its hash and location within
   the artifact. This ledger serves triple duty:
   - **Agent navigation:** An AI agent reads the ledger to understand the entire system
     without traversing directories
   - **Integrity verification:** Any modification to any module changes its hash, which
     propagates up to the root hash — tamper-evident by construction
   - **Fork identification:** Two versions of FERROS can be compared by diffing their
     ledgers — the hash tree makes divergences immediately visible

3. **Cards, not files.** The user-facing abstraction is not "files and folders" but
   **cards and decks**. A document is a card. A project is a deck. An application is a
   deck of executable cards. The user's desktop is their hand. This is not metaphor — it
   is the literal interface and data model.

4. **Changes are forks.** Every modification to the system produces a new version of the
   artifact with a new root hash. The previous version is preserved (content-addressed
   storage means unchanged modules are shared). This is structurally identical to blockchain
   mining: finding the next valid state of the system that the network (or the individual
   user) will adopt. As the system matures and hardens, valid state transitions become
   more constrained — "harder to mine."

5. **Assets are sovereign.** Each visual/interactive element (a card, an avatar, a loot
   box, a battle scene) is a self-contained module within the artifact. It carries its
   own rendering logic, its own state, its own identity hash. It can be referenced from
   anywhere in the system by hash. An avatar module can appear on a profile, in a battle
   scene, on a company card, in a friend list — one asset, many projections.

### The Artifact Structure

```
┌─────────────────────────────────────────────────┐
│ LEDGER (root hash, module index, version)       │
├─────────────────────────────────────────────────┤
│ KERNEL                                          │
│   ├─ bootloader    [hash: a3f8...]              │
│   ├─ scheduler     [hash: 7b42...]              │
│   ├─ memory_mgr    [hash: c91d...]              │
│   ├─ cap_model     [hash: 2e58...]              │
│   └─ ipc           [hash: d4a1...]              │
├─────────────────────────────────────────────────┤
│ RUNTIME                                         │
│   ├─ wasm_host     [hash: 8f3c...]              │
│   ├─ consent_eng   [hash: 1a7e...]              │
│   └─ seal_chain    [hash: 5c29...]  (ADR-001)   │
├─────────────────────────────────────────────────┤
│ ASSETS                                          │
│   ├─ card_system   [hash: e6b4...]  (~1500 LOE) │
│   │   ├─ trading_card_renderer                  │
│   │   ├─ rarity_system                          │
│   │   ├─ ability_engine                         │
│   │   └─ card_state_machine                     │
│   ├─ avatar_system [hash: 3d9f...]  (~1000 LOE) │
│   │   ├─ ghost_renderer (neon outlines)         │
│   │   ├─ 30_avatar_variants                     │
│   │   └─ customization_engine                   │
│   ├─ loot_system   [hash: 72c1...]  (~800 LOE)  │
│   │   ├─ 3d_box_renderer (neopunk holo)         │
│   │   ├─ wooden_chest_renderer (AAA texture)     │
│   │   └─ opening_ceremony_animation             │
│   ├─ battle_arena  [hash: b5e7...]  (~2000 LOE) │
│   │   ├─ lane_system                            │
│   │   ├─ pvp_match_renderer                     │
│   │   └─ miniature_board (for hero scene)       │
│   ├─ chess_match   [hash: 9a43...]  (~1200 LOE) │
│   │   ├─ stock_chess_engine                     │
│   │   ├─ king_queen_bank_pieces                 │
│   │   └─ spectator_view                         │
│   └─ deck_system   [hash: f18a...]              │
│       ├─ work_deck / school_deck / trade_deck   │
│       └─ windows_file_importer                  │
├─────────────────────────────────────────────────┤
│ UI SHELL                                        │
│   ├─ compositor    [hash: 4c6d...]              │
│   ├─ card_renderer [hash: 8e2a...]              │
│   └─ deck_browser  [hash: a1f3...]              │
├─────────────────────────────────────────────────┤
│ USER STATE (hash-chain, per ADR-001)            │
│   ├─ profile_seal_chain                         │
│   ├─ deck_configurations                        │
│   └─ progression_state                          │
└─────────────────────────────────────────────────┘
```

---

## Prior Art & Supporting Knowledge

### Direct Precedents

| System | Relevance | Lesson |
|--------|-----------|--------|
| **Plan 9 (Bell Labs)** | Everything is a file, single namespace across network | Unifying abstraction works — but "file" was the wrong unit for modern use |
| **Nix/NixOS** | Content-addressed store, every package identified by hash, atomic upgrades | Proves hash-addressed modules work at OS scale; FERROS extends this to UI assets |
| **IPFS** | Content-addressed distributed storage, files identified by hash | The "ledger + hash" model works for decentralized systems |
| **Unikernels (MirageOS, IncludeOS)** | Compile entire app + OS into single bootable binary | Proves "one artifact = one system" is viable and performant |
| **SQLite** | Single file replaces entire database system | Most deployed database in history; single-file wins for embedded systems |
| **ROM cartridges (NES/SNES/GB)** | Entire game = one ROM file; no file system | The original SFS; games included code, assets, audio, state save locations |
| **Git** | Content-addressed DAG, forks are first-class, diffing via hash comparison | FERROS artifact versioning mirrors git's object model |

### Technical Infrastructure That Enables SFS

- **Rust's `include_bytes!` / `include_str!`**: Embeds arbitrary data into the compiled
  binary at compile time. Assets become part of the executable.
- **`#[link_section]`**: Places data in named ELF sections. The ledger can be a dedicated
  section at offset 0, readable without parsing the entire binary.
- **WASM modules**: Self-contained, sandboxed, content-addressable. UI assets compile to
  WASM and embed in the artifact.
- **Tar/CPIO-in-binary**: Linux's initramfs is literally a compressed CPIO archive embedded
  in the kernel binary. FERROS can do the same for its asset modules.

### The "Mining" Analogy Is Structurally Accurate

In blockchain mining, a valid block must:
1. Reference the previous block's hash (chain integrity)
2. Satisfy a difficulty constraint (proof of work)
3. Be accepted by the network (consensus)

In FERROS SFS, a valid system update must:
1. Reference the previous artifact's root hash (chain integrity)
2. Pass all verification checks — type safety, capability constraints, seal chain
   validity (proof of correctness)
3. Be accepted by the user or DAO governance (consent/consensus)

As the system matures, the verification checks become stricter (more tests, more
invariants, more governance rules) — it becomes "harder to mine" a valid update. This
is a feature: it means the system converges toward stability.

---

## Why We Cannot Begin Pure SFS Today (Browser Phase)

| Limitation | Why It Blocks | Mitigation |
|------------|--------------|------------|
| **Browser sandbox** | HTML/JS cannot access local filesystem, cannot compile Rust, cannot write bootable artifacts | Current phase uses browser as the rendering target; SFS principles applied within the HTML artifact (single file, ledger-like structure at top) |
| **No Rust kernel yet** | The SFS artifact requires the FERROS kernel to boot from; kernel is in early development | Prototype the asset modules as standalone HTML files that simulate the SFS structure |
| **`file://` crypto limitation** | `crypto.subtle` unavailable on `file://` (ADR-001 documents this) | djb2 fallback for local integrity; SHA-256 when served |
| **Asset complexity** | AAA-quality loot boxes, photorealistic holograms, 30 avatar variants — these require WebGL/Canvas/SVG work that pushes single-file HTML past practical limits | Allow asset modules to be separate HTML files during prototyping, with a concatenation/embedding step that simulates the final SFS artifact |
| **Agent context windows** | Even with SFS, a 50,000-line file exceeds agent context | The ledger solves this: agents read the ledger (100-200 lines) to understand the system, then read only the relevant module. SFS doesn't mean "one unstructured blob" — it means "one artifact with internal structure and a readable index" |

### The Migration Path

```
PHASE 1 (NOW)     → Single HTML files, each an asset prototype
                     algo-trading-arena.html = pitchdeck/shell
                     card-system.html = full card engine prototype
                     avatar-system.html = full avatar engine prototype
                     etc.

PHASE 2 (BUILD)   → Concatenation script assembles HTML modules into
                     single deliverable (simulates SFS)
                     Ledger/index generated automatically at top

PHASE 3 (WASM)    → Asset modules compiled to WASM
                     Embedded in Rust binary via include_bytes!
                     Served by FERROS HTTP server OR rendered natively

PHASE 4 (KERNEL)  → Full SFS: Rust binary = kernel + runtime + assets
                     Boots from bare metal or QEMU
                     "File system" is the artifact's internal structure
                     User state is hash-chain in dedicated storage region
```

---

## Why the Ground-Up Rust System Solves the Remaining Problems

### No inherited file system assumptions
Linux assumes `/usr/bin`, `/etc`, `/home`. FERROS has no such assumptions. The "file
system" is whatever we design. We design cards and decks.

### Content-addressed everything
When every module is identified by hash, there is no "file path" to break. Renaming,
moving, reorganizing — these are ledger updates, not data mutations. The content never
moves; only the index changes.

### HTML → Rust-on-metal pipeline
The current HTML prototypes are not throwaway work. They ARE the specification:
- The CSS becomes the design token system compiled into the compositor
- The JS becomes the game logic compiled to WASM or native Rust
- The HTML structure becomes the UI tree rendered by the card renderer
- The emoji become the placeholder coordinates for actual asset rendering

### Streamlined forking of internet traffic
A FERROS workstation intercepts HTTP traffic at the network layer (Rust-native TCP/IP
stack), renders HTML content through its own compositor (not a borrowed browser engine),
and stores visited content as cards in the user's collection. "Browsing the web" becomes
"collecting cards from the internet." Each card is content-addressed, locally stored,
and owned by the user — not cached temporarily and controlled by the server.

### Agent-optimized development
The SFS with ledger is designed for agents:
- Agent reads ledger → understands entire system in ~200 lines
- Agent targets a specific module by hash → reads only that module
- Agent proposes a change → new hash computed → new ledger generated
- Change review = diff the ledger (which modules changed, what are the new hashes)
- No directory traversal, no import resolution, no build system surprises

---

## The Card-as-Asset-as-Contract Model

Each asset in the FERROS system is simultaneously:

1. **A visual card** — renderable in any context (profile, battle, store, hero scene)
2. **A self-contained module** — carries its own code, state, and rendering logic
3. **A content-addressed artifact** — identified by hash, immutable once sealed
4. **A governance token** — the module's NFT controls who can modify the rules it encodes
5. **A DAO-governed contract** — the rules of "card-based trading" are encoded in the
   card-based-trading module, governed by a DAO that owns its NFT, maintained by a
   botnet that processes user feedback through consent-driven pipelines

Real-world market constraints control card abilities:
- No dividend → no Lifesteal ability (can't reinvest what doesn't exist)
- High volatility → eligible for Rally ability (dip triggers exist)
- Low correlation → eligible for Elusive ability (stealth trades plausible)
- The market IS the game designer; the cards capture market identity for use in the arena

Companies don't design their own cards — their stock behavior does. Apple's card emerges
from Apple's market data. The card system captures corporate identity through market
dynamics, not through corporate branding partnerships.

---

## Consequences

### Positive
- Eliminates file system as a source of complexity, fragility, and agent confusion
- All system state is content-addressed and tamper-evident by construction
- User experience is cards and decks, not files and folders — reducing cognitive load
- Development is agent-optimized from architecture level, not bolted on
- Versioning, forking, and merging are built into the data model, not layered on via git
- Natural path from browser prototype to bare-metal OS without architectural discontinuity

### Negative
- Breaks every convention in operating system design (files, directories, paths, mounts)
- Tooling must be built from scratch (no `ls`, no `cd`, no `grep` — replaced by
  ledger queries, card searches, deck browses)
- Developer onboarding requires learning a new mental model
- The "one artifact" constraint may have performance implications for very large systems
  (mitigated by lazy loading of modules and copy-on-write pages)

### Risks
- The abstraction may leak — some operations genuinely need file-like semantics (logs,
  pipes, device I/O). These must be modeled as card streams or deck channels.
- Existing software expects POSIX file APIs. Compatibility layer needed for running
  non-native applications. (FERROS accepts this cost; see ADR-0001.)
- If the ledger grows very large, the "agent reads ledger" advantage diminishes.
  (Mitigated by hierarchical ledger with summary + detail levels.)

---

## Alternatives Considered

| Alternative | Reason Rejected |
|-------------|-----------------|
| Conventional hierarchical file system | Inherited from 1970s Unix; optimized for human terminal use, not agent development or card-based UX |
| Microservice architecture (many small files + API) | Maximizes agent context burn; every service boundary is a context switch |
| Database-only (everything in SQLite) | Close to SFS in spirit, but SQLite is not bootable and adds an abstraction layer between the system and its data |
| Keep current single HTML forever | Works for prototype; doesn't scale to AAA assets, kernel integration, or multi-module systems |

---

## Related ADRs
- [ADR-0001: Start New, Do Not Fork](./ADR-0001-start-new-do-not-fork.md) — Rust from scratch
- [ADR-001: Progression-Lock Pattern](./ADR-001-progression-lock-pattern.md) — Hash-chain user state (the SFS user state model)
- [ADR-002: Smart Contract Boundaries](./ADR-002-smart-contract-boundaries.md) — Asset governance
