# FERROS Hardware Queue

This queue feeds the local driver for **hardware-track** work: firmware spikes per demo device, touchscreen bring-up, SoC ML probe, docking-station bring-up, and real-world UX session plans. Consumed only by Batch Mode runs scoped to `track: hardware`.

The output of every hardware wave is a `docs/hardware/findings/FINDINGS-<device>-<date>.md` file (for waves that produce physical evidence) or a plan document under `docs/hardware/` (for pre-run planning waves).

## Queue item schema (same shape as WAVE-QUEUE.md)

Required fields: `Title`, `Status`, `Priority`, `Gate`, `Owning streams`, `Goal`, `Anchor files`, `Validation`, `Constraints`, `Last update`

Optional fields (additive, do not break existing item order):
- `size: S | L` — S means ≤3 anchor files, single stream, docs-only. L means multi-stream or schema-touching. Batch Mode default consumes only S.
- `parallel-safe-with: [WAVE-IDs]` — explicit non-overlap declarations.
- `serial-after: WAVE-ID` — must wait for a prior wave.
- `solo: true | false` — must run alone.
- `track: code | system | hardware` — which queue this belongs to.

---

## Ready

### HARDWARE-2026-04-27-01

- Title: D1 device-target inventory
- Status: ready
- Priority: P1
- Gate: pre-D1 device selection
- Owning streams: S7 primary; S8 consumer awareness
- Goal: Document the candidate demo devices the project has access to and pick a primary D1 target. For each candidate: note the form factor, OS/firmware baseline, display capability, and known bring-up effort. Identify which device is the primary D1 target and why. Record any known blockers or required tools. This is a docs-only research and selection wave; no firmware or code lands.
- Anchor files: `docs/hardware/d1-target-inventory.md`
- Validation: `get_errors` clean on `docs/hardware/d1-target-inventory.md`.
- Constraints: Docs-only. Do not claim D1 evidence (D1 is being defined, not closed). Do not invent bridge protocol or HA fork internals.
- Last update: 2026-04-27
- size: S
- track: hardware

### HARDWARE-2026-04-27-02

- Title: First firmware-spike target plan for the chosen D1 device
- Status: ready
- Priority: P1
- Gate: pre-D1 firmware prep
- Owning streams: S7 primary
- Goal: Docs-only spike plan for the chosen D1 device (identified in HARDWARE-2026-04-27-01). Cover the four milestones: boot, identify (device profile visible via `ferros profile init`), accept-grant (at least one capability grant issued), report-state (one HA-facing state report visible). For each milestone: note the expected deliverable, the toolchain or wiring needed, and any known unknowns. No firmware code lands in this wave.
- Anchor files: `docs/hardware/firmware-spikes/<target>/README.md`
- Validation: `get_errors` clean on the spike plan README.
- Constraints: Docs-only. No firmware code. Do not claim D1 evidence. `serial-after: HARDWARE-2026-04-27-01` (device target must be chosen first).
- Last update: 2026-04-27
- size: S
- serial-after: HARDWARE-2026-04-27-01
- track: hardware

### HARDWARE-2026-04-27-03

- Title: First real-world UX session plan for D1
- Status: ready
- Priority: P1
- Gate: pre-D1 UX planning
- Owning streams: S7 primary; S5 consumer awareness
- Goal: Define a 60-minute real-world UX session script for D1. The script must cover: (1) profile init via `ferros profile init` on the target device, (2) agent-center read via the localhost shell or CLI, (3) one HA entity registered through the planned bridge contract or documented stand-in, (4) one power cycle with confirmation that profile and agent state reload. Output is a session script the human can run verbatim; findings from the actual session get filed under `docs/hardware/findings/`. This wave produces the script only, not the findings.
- Anchor files: `docs/hardware/ux-sessions/d1-session-01-script.md`
- Validation: `get_errors` clean on the session script.
- Constraints: Docs-only. Do not claim D1 or G4 evidence. Do not invent bridge protocol internals. `parallel-safe-with: [HARDWARE-2026-04-27-01]`.
- Last update: 2026-04-27
- size: S
- parallel-safe-with: [HARDWARE-2026-04-27-01]
- track: hardware

---

## In Progress

None.

---

## Blocked

None.

---

## Done

None yet. Hardware queue established in WAVE-2026-04-27-03.
