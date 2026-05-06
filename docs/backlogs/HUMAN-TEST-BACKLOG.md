# Human Test Backlog

This file is the ordered queue of operator-facing test items.

Use it when a feature slice is ready for a human to inspect, operate, or validate. Do not use it for pure implementation work.

---

## Status values

- `Planned` - defined, but not yet ready for operator execution
- `Ready` - preflight gates passed and an instruction packet can be issued
- `In Session` - operator is actively executing it
- `Coordinator Review` - result captured and waiting for triage
- `Hotfix` - blocked on a front-of-queue repair
- `Agent Backlog` - blocked on broader implementation or docs work
- `Hardware Queue` - blocked on hardware or environment work
- `Closed` - completed with acceptable evidence

---

## Seed queue

| ID | Item | Status | Primary owner | Depends on | Notes |
|----|------|--------|---------------|------------|-------|
| HTB-001 | Inspect Agent | Ready | S5 / S3 | Base-shell modularization | Ready packet: `docs/operator-sessions/OPS-2026-05-05-H1-current-shell-read-surfaces.md`. Validate selected-agent detail, grants, deny visibility, and status rail clarity |
| HTB-002 | Recover From Deny | Ready | S5 / S3 / S4 | Touch-safe deny path and operator copy | Ready packet: `docs/operator-sessions/OPS-2026-05-05-H1-current-shell-read-surfaces.md`. Recovery posture is landed on the runway route; inspect deny visibility and the next honest move without implying hidden writes |
| HTB-003 | Profile Round Trip | Ready | S5 / S2 | Profile slice stays at init/show/export/import | Ready packet: `docs/operator-sessions/OPS-2026-05-05-H2-profile-round-trip.md`. Grant and revoke remain out of scope |
| HTB-004 | Onramp Review | Ready | S5 / S7 | Proposed-material route and honest blocked-state language | Ready packet: `docs/operator-sessions/OPS-2026-05-05-HTB-004-onramp-review.md`. Accept/reject remains display-only; verify the consent boundary, route-local artifact, and recovery posture without inferring canonical state |
| HTB-005 | Forge Preview | Ready | S5 / forge lane | First Forge surface module and export preview | Ready packet: `docs/operator-sessions/OPS-2026-05-05-H4-preview-surfaces.md`. Non-authoritative artifact grammar only |
| HTB-006 | Arena Preview | Ready | S5 / arena lane | Arena runtime preview and result-staging surface | Ready packet: `docs/operator-sessions/OPS-2026-05-05-H4-preview-surfaces.md`. Results must remain non-evidentiary until accepted |
| HTB-007 | Touch Posture | Hardware Queue | S5 | Touch-safe shell posture | Touch posture is implemented in the shell, but the real touchscreen pass now rides the hardware-backed HTB-013 packet and findings flow |
| HTB-008 | Proposed Material Inspection | Ready | S5 / S7 | `ProposedMaterialCard` and source-lineage display | Ready packet: `docs/operator-sessions/OPS-2026-05-05-H1-current-shell-read-surfaces.md`. Verify non-canonical language before accept |
| HTB-009 | Receipt Readback | Ready | S5 / S7 | `ReceiptStrip` and source seam metadata | Ready packet: `docs/operator-sessions/OPS-2026-05-05-H1-current-shell-read-surfaces.md`. Distinguish rehearsal, runway, and canonical receipts |
| HTB-010 | Home-Hub Topology Read | Ready | S7 | Topology and source-lineage surfaces | Ready packet: `docs/operator-sessions/OPS-2026-05-05-H3-home-hub-read.md`. Distinguish local runway from real-device evidence |
| HTB-011 | Tool Lane Disclosure | Ready | S5 / S7 | Tool-lane module and disclosure fields | Ready packet: `docs/operator-sessions/OPS-2026-05-05-H1-current-shell-read-surfaces.md`. Local vs external scope must be explicit |
| HTB-012 | Evidence Badge Sanity Check | Ready | S5 / S8 | Evidence badge module and named backing seams | Ready packet: `docs/operator-sessions/OPS-2026-05-05-H1-current-shell-read-surfaces.md`. No unsupported security or verification claims |
| HTB-013 | Host Touchscreen Pilot | Hardware Queue | S5 / S8 | HTB-007 plus host driver capture packet | Packet: `docs/operator-sessions/OPS-2026-05-05-HTB-013-host-touchscreen-pilot.md`. Issue after the touchscreen is connected and the host capture commands are available |

---

## Campaign order

1. H0 - Ready or hardware-gated now: HTB-004 first, then HTB-013 once the touchscreen is connected and the host capture commands can run.
2. H1 - Current shell and ACC read surfaces: HTB-001, HTB-002, HTB-008, HTB-009, HTB-011, and HTB-012 after the A1 autopilot batch closes. HTB-007 rides the same shell posture work but remains hardware-gated through HTB-013.
3. H2 - Profile lane: HTB-003 after init, show, export, and import stay bounded and validated.
4. H3 - Home-Hub read lane: HTB-010 after topology and source-lineage read surfaces land.
5. H4 - Preview-only surfaces: HTB-005 and HTB-006 after Forge and Arena preview proof lands.

If a campaign wave finds a blocking local defect, move it to `Hotfix` and return the repair to the owning implementation batch instead of reordering the whole queue informally.

---

## Queue rules

1. Move an item to `Ready` only after preflight gates pass.
2. Every `Ready` item should have a named operator instruction packet.
3. If the operator finds a blocking local defect, the coordinator may route it to `Hotfix`.
4. If a comment is meta or architectural, route it through the coordinator instead of rewriting the active task ad hoc.
5. Closed items should have a session-log row and a findings or evidence reference.