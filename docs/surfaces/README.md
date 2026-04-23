# HTML Surfaces

This directory is the start of the FERROS surface reorganization effort.

## What belongs here

- Archived or superseded HTML surface variants.
- Future active surface groupings as they are moved out of the `docs/` top layer.
- Surface-specific notes that do not need to live beside contracts, ADRs, or gate docs.

## Why this exists

FERROS began with editable HTML surfaces because that was the fastest way to iterate on UX and front-end behavior without repeating the slower Rust-plus-WASM editing loop used in another workspace.

Those surfaces still matter:

- They are aspirational pathways for future product lanes.
- They drive the final front end by making UX and state boundaries concrete early.
- They now coexist with stream-based backend and contract work, allowing progress from both directions.

See `docs/adr/ADR-017-html-surface-incubation-strategy.md` for the policy decision behind this structure.
