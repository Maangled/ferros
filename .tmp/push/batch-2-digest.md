# PUSH Batch 2 Digest

- Status: seeded only
- Seed source: `batch-1-digest.md`

## Candidate Lanes

| Lane | Focus |
|---|---|
| L1 | S4 consume `LocalRunwayState` inside `ferros-node` runway builders. |
| L2 | S3 expose a local typed reader for `/runway-summary.json` without widening JSON-RPC. |
| L3 | S2 project `LocalConsentSnapshot` into the current shell-ready read model. |
| L4 | S5 let the shell choose between default and explicit profile-path runway reads. |
| L5 | S6 add a local writer for `local-push-audit-envelope` records under `.tmp/push/`. |
| L6 | S1 extend `cargo xtask burst` with file-level stats and digest emission. |
| L7 | S7 map stand-in agent, deny visibility, and reboot pending state into one operator drill. |
| L8 | S8 keep `STATUS.md` and gate truth serial until one of the owner lanes above lands. |

## Guardrails

- Frozen `profile.v0.json` and `capability-grant.v0.json` stay untouched.
- Shared truth stays serial.