# Pack B Session 01 x86_64 ADR-025 Proof Note

Status: findings-backed proof note
Scope: x86_64 Pack B baseline only
Authority: ADR-022 plus completed hardware findings
Constraint: this note does not move D1, G4, Home Assistant proof, or separate-host hardware proof on its own

## Evidence inputs

- `docs/hardware/findings/FINDINGS-pack-b-session-01-profile-baseline.md`
- `docs/gates/D1.md`
- `docs/adr/_RESEARCH-NOTES/RN-2026-05-adr025-guardrail-scoreboard.md`
- `docs/adr/ADR-025-dual-root-hardware-runway.md`

## What this wave proves

- A real Pack B `x86_64` device under test exists now in the evidence set: `homelab001`.
- The FERROS binary executed on that physical host and completed `profile init` plus `profile show` against a fresh local profile path.
- The same session refreshed the local hub rehearsal artifact set and recorded a named stand-in summary on the DUT-side repo path.
- This is the first family-level operational proof that the `x86_64/Fastest` lane can execute the identify slice on real hardware without widening any Home Assistant or launch claim.

## ADR-025 mapping

| ADR-025 proof dimension | Result from Pack B session 01 | Bound imposed by this note |
|-------------------------|-------------------------------|----------------------------|
| Check 1 — lane sufficiency | Positive for one concrete `x86_64` baseline slice: the real Pack B DUT can run the local profile boundary and refresh the local artifact chain. | This is not full family closure, Pi proof, Jetson proof, ESP32 proof, or a statement that every required lane has passed. |
| Check 5 — control-plane witness boundary | All claims are source-attributed to `homelab001` local shell output and copied `.tmp/hub` artifacts from the same host. | No cross-board inference is allowed. This note does not elevate `x86_64` into proof for Pi, Jetson, ESP32, or a separate Pack C host. |
| Check 6 — claim-boundary enforcement | The completed findings stay at profile-baseline scope and carry forward the unresolved `ha-local-bridge` visibility mismatch as a gap, not a normalized success. | No D1, G4, HA, consent-accept, reboot-safe, or full power-cycle claim is added here. |

## D1 relationship

`docs/gates/D1.md` requires all evidence rows to be true at the same time before D1 can close. This Pack B baseline note supports only the first row, and even that row remains partial because the session captured `profile init` plus `profile show` on the target device but did not yet record a same-session shell re-open reload check.

This wave does not satisfy the remaining D1 requirements:

- no named Home Assistant entity or completed documented stand-in handoff through the D1 evidence table,
- no visible consent and deny flow on the operator surface,
- no reboot-safe FERROS-side state after a full power cycle,
- no D1 evidence-table write and therefore no D1 closure.

## Surviving non-claims

- No Home Assistant proof.
- No separate-host Pack C visibility proof.
- No reboot-safe or power-cycle-safe state proof.
- No G4 movement.
- No FERROS-native runtime claim.
- No claim that the bridge agent is correctly visible through `ferros agent list`; that mismatch remains open until a dedicated follow-up wave resolves it.

## Closure impact

- Safe closure movement earned now: none.
- Honest advancement earned now: the ADR-025 framework has its first findings-backed `x86_64` operational proof note tied to a real Pack B session.
- Next proof edges that can actually change closure posture: `HARDWARE-2026-04-30-06` for the handoff and reboot slice, `HARDWARE-2026-04-30-07` for separate-host HA visibility, and later non-`x86_64` family runs for broader ADR-025 family proof.