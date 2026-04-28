# PUSH Batch 3 Digest

- Status: seeded only
- Seed source: `batch-2-digest.md`

## Candidate Lanes

| Lane | Focus |
|---|---|
| L1 | S4 add checkpoint-to-checklist mapping between `LocalRunwayState` and runway summary items. |
| L2 | S3 add a stable local serializer for shell/export consumers of runway summaries. |
| L3 | S2 expose revoked-grant summaries as explicit local-only shell-facing copy. |
| L4 | S5 stage a runway inspector card that reads from the local-only route only. |
| L5 | S6 add fixture examples for `local-push-audit-envelope.schema.json`. |
| L6 | S1 emit a dry-run burst inventory from xtask without touching CI or workflows. |
| L7 | S7 document reboot and re-registration hooks as separate proofs from local restart. |
| L8 | S8 seed closure-evidence intake lines for future failing seams only after they exist. |

## Guardrails

- No remote transport.
- No HA bridge implementation claims.