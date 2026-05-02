# S6 Legacy Archive Sweep — 2026-05-02

**Scope:** `home-assistant`, `workpace-rust`, `workspace-old`, `sheetgen-rust`, `botgen-rust`, `palworld-server`, `tunes-bot-js`, `home-browser`  
**Purpose:** one final evidence pass before archive or retirement work  
**Outcome:** preserve deltas that FERROS still wants, then archive the rest with explicit confidence

---

## Summary

The final sweep confirmed that FERROS had already captured most of the durable architectural
value from `botgen-rust`, `workpace-rust`, and `sheetgen-rust` in ADR-018, ADR-019, and
ADR-020. The remaining deltas were smaller and more specific:

- `home-assistant` contributed shell and spatial-surface patterns that extend ADR-009.
- `workpace-rust` still contained a useful voting taxonomy and tally shape that FERROS had not
  preserved yet.
- `sheetgen-rust` contained more precise hierarchy and cropping rules than ADR-020 had
  previously recorded.

Everything else in this pass is now low-risk archive material.

Owner framing to preserve for later end-user language: in a late-game FERROS posture, the
"home browser" is the user-facing shell for a sovereign multi-device system where devices act
as a checks-and-balances committee and local data remains user-owned.

## Folder Findings

### `home-assistant`

**Keep:**
- sticky top-edge control chrome and context switching (`input_select.room_view`)
- independently scrollable side rail for dense controls
- spatial dashboard posture that keeps shell chrome fixed while the main viewport swaps states

**Current FERROS landing point:** ADR-009 now records context switching and responsive
variants as an extension of the four-corner docking model.

**Do not over-harvest:**
- Home Assistant entity plumbing, Lovelace specifics, and deployment mechanics

**Further research if time is needed later:**
- layer-toggle authoring for floorplans and drafting surfaces
- Assistant-mediated SVG mutation with explicit consent gating
- mobile dock reflow and sticky context bars

### `workpace-rust`

**Already captured well:**
- typed IPC
- shell layering
- signed delivery envelopes
- capability-scoped UI messaging

**New durable delta:**
- seven voting modes
- match-dispatched tally orchestration
- `vote_totals` upsert pattern
- token-weighted vote-power experiment anchored to card state

**Current FERROS landing point:** preserved in
[RN-2026-05-voting-decision-models.md](../adr/_RESEARCH-NOTES/RN-2026-05-voting-decision-models.md)
instead of a premature ADR, per ADR-022 and the roadmap research-lane rule.

**Archive note:** the `modules/` topology remains a useful readability signal, but FERROS does
not need to adopt the old crate explosion or hard-coded runtime coupling to preserve that
clarity.

### `workspace-old`

`workspace-old/workspace-old` is effectively a separate `moonlightsrv` smart-home server rather
than just an older `workpace-rust` snapshot. It contains domain modules for home automation,
calendar, messaging, and device control, but nothing that now blocks archive.

**Archive posture:** safe to archive with minimal further review.

### `sheetgen-rust`

**Already captured well:**
- migration-first schema authority
- database-enforced invariants
- JSONB snapshot posture
- contract-generation caution

**New durable delta:**
- explicit authored hierarchy: `workspace -> project -> sheet -> titleblock -> viewport -> view -> drawing -> annotation`
- child-owned cropping and overlay semantics
- exact-one-parent SQL pattern with `CHECK` plus partial indexes
- stronger detail on per-parent z-order constraints and rendering snapshots

**Current FERROS landing point:** ADR-020 now records the hierarchy and parentage patterns more
explicitly.

**Further research if time is needed later:**
- blob upload lifecycle details
- event or undo groundwork that never reached full implementation

### `botgen-rust`

The final pass did not uncover any architectural blocker beyond ADR-018. The old repo still has
useful runtime hygiene patterns, but they enrich ADR-018 rather than changing it.

**Archive posture:** safe to archive under ADR-018.

### `palworld-server`

`palworld-server` is not part of the FERROS harvest surface.

Operational check during this sweep found:

- no running Palworld container
- no stopped Palworld-named container with a restart policy
- no Palworld systemd service installed on the host

The local compose file did carry `restart: unless-stopped`, so this sweep changed it to
`restart: "no"` to prevent accidental host auto-start if that compose file is used locally.

### `tunes-bot-js`

`tunes-bot-js` is a compact Discord music-link conversion bot with Spotify genre lookup and
channel routing.

**Harvest decision:** reference-only, no major ADR needed.

- Keep as a small pattern note: cross-service link reconciliation is useful beyond music.
- FERROS can reuse the concept later as a service-parity broker pattern where competing service
  outputs are normalized into one user-facing choice surface.
- No unique FERROS architecture must be ported from this repo as-is.

**Operational note:** local compose restart policy should be neutralized before archive if this
folder is retired.

### `home-browser`

`home-browser` contained no files at archive time and was removed from `/home/homelab001/apps`.

**Archive posture:** complete.

**Concept note preserved:** even though the legacy folder was empty, "home browser" remains a
useful future product metaphor for FERROS as a trusted, cross-device personal shell.

## Archive Recommendation

Archive confidence after this sweep:

| Folder | Confidence | Reason |
|--------|------------|--------|
| `botgen-rust` | High | ADR-018 already captures the reusable architecture shapes |
| `workpace-rust` | Medium-High | shell and IPC were captured; voting is now preserved in a research note |
| `workspace-old` | High | separate smart-home project, low current FERROS relevance |
| `sheetgen-rust` | High | ADR-020 now captures the remaining hierarchy delta |
| `home-assistant` | Medium-High | ADR-009 now captures the best shell-context idea; optional deeper spatial notes can wait |
| `palworld-server` | High | operationally neutralized and outside FERROS scope |
| `tunes-bot-js` | Medium-High | no unique architecture to port, but useful cross-service reconciliation idea preserved |
| `home-browser` | High | folder was empty and has been removed; concept retained as product language |

## Requested Follow-Up Research Time

None of the following block archive, but each is a valid targeted future dive if a nearby
stream needs it:

1. Voting and decision models beyond simple consent gates.
2. Layer toggles, context routing, and mobile reflow for spatial workbenches.
3. Blob-storage and undo or event groundwork from `sheetgen-rust` if `ferros-data` moves into
   those areas soon.
4. Cross-service parity brokering patterns derived from `tunes-bot-js` beyond music use cases.

## References

- [ADR-009](../adr/ADR-009-four-corner-docking-layout.md)
- [ADR-018](../adr/ADR-018-harvest-botgen.md)
- [ADR-019](../adr/ADR-019-harvest-workpace.md)
- [ADR-020](../adr/ADR-020-harvest-sheetgen.md)
- [RN-2026-05-voting-decision-models.md](../adr/_RESEARCH-NOTES/RN-2026-05-voting-decision-models.md)
- [ARCHIVE-MANIFEST-2026-05-02.md](./ARCHIVE-MANIFEST-2026-05-02.md)

## Archive Execution Note

On 2026-05-02, the retirement-target folders were moved from `/home/homelab001/apps` to:

`/home/homelab001/apps/_archive/2026-05-02-ferros-consolidation`

`home-assistant` and `ferros` were intentionally retained in place.