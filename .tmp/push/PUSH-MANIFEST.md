# FERROS Push Manifest

## Scope

- Mode: bounded local push execution
- Wave anchor: `WAVE-2026-04-28-01`
- Active gate: G4
- Runway context: D1 remains open

## Total artifacts

- Crates added: 0
- Files changed or added: 21
- Batch 1 implementation files: 10
- Batch digest files: 8
- Manifest files: 1
- Branches created: 0
- Draft PRs created: 0
- Current local LOC delta before final truth-sync: +1139 / -24 across landed Batch 1 implementation files

## Per-batch digests

- [Batch 1](batch-1-digest.md) — landed owner lanes
- [Batch 2](batch-2-digest.md) — seeded follow-up vectors
- [Batch 3](batch-3-digest.md) — seeded follow-up vectors
- [Batch 4](batch-4-digest.md) — seeded follow-up vectors
- [Batch 5](batch-5-digest.md) — seeded follow-up vectors
- [Batch 6](batch-6-digest.md) — seeded follow-up vectors
- [Batch 7](batch-7-digest.md) — seeded follow-up vectors
- [Batch 8](batch-8-digest.md) — seeded follow-up vectors

## Surfaced failures and open seams

- No compile or test failures remained in the executed Batch 1 validations.
- Open seam: `crates/ferros-node/src/lib.rs:1025` exposes `/runway-summary.json`, but the shell still reads the default local profile path only.
- Open seam: `crates/ferros-runtime/src/local_runway.rs:24` defines checkpoint transitions that are not yet consumed by `ferros-node`.
- Open seam: `schemas/local-push-audit-envelope.schema.json:1` is in repo, but no harness or codegen consumer validates it yet.
- Open seam: `site/agent-center-shell.html:1149` renders the runway route read-only; it is still an operator drill, not launch evidence.

## Recommended next-gate targets

1. Consume `LocalRunwayState` inside `ferros-node` and let S5 render checkpoint progress from the same summary.
2. Add a real local writer for `local-push-audit-envelope` so `.tmp/push/` output has a typed emitter, not just markdown.
3. Keep S8 serial and last: truth-sync `STATUS.md` only after one more owner lane actually changes gate-facing reality.

## Proposed STATUS.md patch

```diff
--- a/STATUS.md
+++ b/STATUS.md
@@
-| S4 Runtime / OS Core | 🟡 Post-G3 hardening | `ferros-runtime`, in-memory executor and bus, policy property tests, the `cargo run --bin ferros -- demo` path, the `ferros-core --no-default-features` compile slice, a local `thumbv7em-none-eabi --no-default-features` proof, and the narrow host/controller support for both the local-only `LocalAgentApi` seam and the first local-only lifecycle/write JSON-RPC slice are now in repo; CI is configured to enforce the same thumb-target check while broader `no_std` and host hardening remain | post-G3 |
+| S4 Runtime / OS Core | 🟡 Post-G3 hardening | Existing post-G3 runtime slices remain landed; this push adds an additive `ferros-runtime` local runway checkpoint scaffold plus a `ferros-node` local-only runway summary read surface for operator drills without changing gate truth, remote transport, or launch claims | post-G3 |
@@
-| S5 UX | 🟨 Phase A archive/link-hygiene landed; Phase B localhost observation slice landed; selected-agent shell-intent copy staged | real landing page and honest status banner shipped; the Phase A archive/link-hygiene pass and docs-root reference repairs are landed, the fixed-slot localhost shell reads live agent, grant-state, and deny-log data through `ferros-node`, operator-assisted localhost acceptance proves local `ferros agent run | stop` changes read back through the same `agent.snapshot` refresh seam, the upstream local-only `agent.run` / `agent.stop` JSON-RPC slice now exists on the localhost host, and the 2026-04-27 WAVE-2026-04-27-01 landing staged selected-agent lifecycle intent copy and read-only slot affordances on the live shell while the shell UI itself still remains observation-only | post-G3 |
+| S5 UX | 🟨 Phase A archive/link-hygiene landed; Phase B localhost observation slice landed; selected-agent shell-intent copy staged | Existing localhost shell and lifecycle-control surfaces remain observation-first; this push adds a read-only runway route backed by `/runway-summary.json` plus harness coverage for the new route while keeping browser writes, grant mutation, remote transport, and gate claims closed | post-G3 |
```

## Notes

- Branch and PR creation were requested by the directive but were not performed in this local chat environment.
- Batches 2 through 8 are recorded as seeded follow-up vectors, not as landed repo changes.