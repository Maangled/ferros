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
| HTB-001 | Inspect Agent | Planned | S5 / S3 | Base-shell modularization | Validate selected-agent detail, grants, deny visibility, and status rail clarity |
| HTB-002 | Recover From Deny | Planned | S5 / S3 / S4 | Touch-safe deny path and operator copy | Deny must remain visible and actionable without widening privileges |
| HTB-003 | Profile Round Trip | Planned | S5 / S2 | Profile slice stays at init/show/export/import | Grant and revoke remain out of scope |
| HTB-004 | Onramp Review | Planned | S5 / S7 | Proposed-material route and honest blocked-state language | Accept/reject may still be display-only if the backend seam is not ready |
| HTB-005 | Forge Preview | Planned | S5 / forge lane | First Forge surface module and export preview | Non-authoritative artifact grammar only |
| HTB-006 | Arena Preview | Planned | S5 / arena lane | Arena runtime preview and result-staging surface | Results must remain non-evidentiary until accepted |
| HTB-007 | Touch Posture | Planned | S5 | Touch-safe shell posture | No hover-only primary actions |
| HTB-008 | Proposed Material Inspection | Planned | S5 / S7 | `ProposedMaterialCard` and source-lineage display | Verify non-canonical language before accept |
| HTB-009 | Receipt Readback | Planned | S5 / S7 | `ReceiptStrip` and source seam metadata | Distinguish rehearsal, runway, and canonical receipts |
| HTB-010 | Home-Hub Topology Read | Planned | S7 | Topology and source-lineage surfaces | Distinguish local runway from real-device evidence |
| HTB-011 | Tool Lane Disclosure | Planned | S5 / S7 | Tool-lane module and disclosure fields | Local vs external scope must be explicit |
| HTB-012 | Evidence Badge Sanity Check | Planned | S5 / S8 | Evidence badge module and named backing seams | No unsupported security or verification claims |

---

## Queue rules

1. Move an item to `Ready` only after preflight gates pass.
2. Every `Ready` item should have a named operator instruction packet.
3. If the operator finds a blocking local defect, the coordinator may route it to `Hotfix`.
4. If a comment is meta or architectural, route it through the coordinator instead of rewriting the active task ad hoc.
5. Closed items should have a session-log row and a findings or evidence reference.