# S8 Research Note - Contributor Onboarding Checklist

**Date:** 2026-04-28
**Owning stream:** S8 primary
**Output feeds:** contributor readiness; Batch K release/install readiness
**Status:** Checklist draft only. No issue seeding or governance authority change is made.

---

## Purpose

This checklist gives a new contributor a short, repo-truth-backed path from "I opened the repository" to "I can choose a scoped task." It complements `CONTRIBUTING.md` without replacing it.

---

## Read First

1. `STATUS.md` - current gate and stream state.
2. `CONTRIBUTING.md` - stream map and PR conventions.
3. The stream README for the work you want to touch.
4. The stream BACKLOG and PROGRESS files.
5. Relevant contract docs if your work crosses a stream boundary.
6. `docs/orchestration/WAVE-QUEUE.md` if you are picking queued work.
7. `docs/gates/D1.md` and `docs/gates/G4.md` if your task mentions demo, launch, hardware, or Home Assistant.

---

## Pick the Right Stream

| If the work is about... | Start here |
|---|---|
| Workspace, CI, release hygiene | S1 |
| Profile, keys, grants, schemas | S2 |
| Agent registry, local API, JSON/RPC | S3 |
| Runtime, policy, executor, reload | S4 |
| Site, localhost shell, WASM UX | S5 |
| Prior-art harvest and data extraction | S6 |
| Hub, hardware, Home Assistant | S7 |
| Docs, ADRs, governance, contributor flow | S8 |

If a task touches more than one stream, name the primary owner first and keep consumer-awareness edits narrow.

---

## Gate Awareness

| Gate | Current state for contributors |
|---|---|
| G1 | Closed. Do not reopen closed evidence wording. |
| G2 | Closed. Do not mutate frozen schemas in place. |
| G3 | Closed. Current work is post-G3 hardening. |
| D1 | Active demo runway. Do not claim closed evidence. |
| G4 | Active launch gate. Hardware and HA evidence are still missing. |

The most common mistake is treating D1 rehearsal docs as launch readiness. D1 is a demo gate; G4 is launch.

---

## Queue Discipline

Before editing:

- Check whether your task is already in a queue.
- Confirm the wave's anchor files.
- Stay inside anchor files unless the wave explicitly permits operational bookkeeping.
- Do not edit shared truth surfaces casually.
- If a frozen schema, gate-close row, CI workflow, or root manifest is involved, ask for a dedicated scope.

After editing:

- Update the owning stream `PROGRESS.md` only when the wave calls for it or when the change materially advances that stream.
- Keep run-log and queue edits in the orchestration closeout path.
- Record validation honestly.

---

## Validation Checklist

| Change type | Minimum validation |
|---|---|
| Docs-only | Editor diagnostics or `get_errors` when available; direct readback |
| Rust code | Focused Cargo test/check for touched crates |
| `ferros-core` no_std | `cargo check -p ferros-core --no-default-features` and target check if relevant |
| Shell/HTML | Harness or browser path that matches the claim |
| Gate/evidence | Exact evidence artifact and date |
| Hardware | Human/device findings file; no reconstructed evidence |

If a validation tool is unavailable in your environment, say so. Do not claim a check you did not run.

---

## Hard Stop Lines

Do not:

- edit `schemas/profile.v0.json`;
- edit `schemas/capability-grant.v0.json`;
- change G1/G2/G3 closed evidence wording;
- promote ADR-024 from Proposed;
- claim D1 or G4 evidence from planning docs;
- widen remote transport or privileged browser control without code-backed scope;
- turn a hardware plan into a hardware finding.

---

## PR Shape

Use a small PR with:

- title prefix `[S#]`;
- one primary stream;
- clear anchor files;
- validation command output or validation limitation;
- no unrelated formatting churn;
- explicit note if D1/G4 are mentioned but not moved.

Good contributor work in FERROS is often a narrow improvement that preserves truth. The project rewards boring precision.

