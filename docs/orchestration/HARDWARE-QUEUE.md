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

### HARDWARE-2026-04-30-05

- Title: First Pack B physical ferros-hub run with profile init/show
- Status: ready
- Priority: P1
- Gate: D1 physical-device baseline
- Owning streams: S7 primary; S2 and S4 consumer awareness
- Goal: Run the first real Pack B x86_64 device-under-test session with the FERROS binary on physical hardware, capture the exact build or run command used for `ferros-hub` or the target-side host path, and record target-side `ferros profile init` plus `ferros profile show` on the named DUT. The output should establish the first honest physical-device baseline without claiming HA entity visibility, full power-cycle survival, independent install proof, D1 closure, or G4 movement.
- Anchor files: `docs/hardware/findings/FINDINGS-pack-b-session-01-profile-baseline.md`
- Validation: `get_errors` clean on `docs/hardware/findings/FINDINGS-pack-b-session-01-profile-baseline.md`; findings record the named DUT, physical session date, exact command refs, profile init and show result, and operator-attended note.
- Constraints: Physical-device execution is expected in this future wave, but keep the claim ceiling at profile baseline only. Do not claim HA entity visibility, deny visibility, restart-safe state, full power-cycle survival, D1 closure, or G4 movement. Run this lane only after the named session plan exists.
- Last update: 2026-04-30
- size: L
- serial-after: HARDWARE-2026-04-30-04D
- solo: true
- track: hardware

### HARDWARE-2026-04-30-06

- Title: Mirror the local handoff artifact chain on the named DUT
- Status: ready
- Priority: P1
- Gate: D1 on-device handoff mirror
- Owning streams: S7 primary; S2, S3, and S4 consumer awareness
- Goal: On the named Pack B device under test, mirror the already-landed local code-runway artifact chain by capturing a bridge artifact or named stand-in output, proposal and decision fields, runway shell or log observation, deny visibility, and a clean-reboot reload observation that matches the local handoff map. Record the operator-attended session honestly as DUT-side rehearsal output without claiming real HA entity visibility, full power-cycle survival, consent acceptance, D1 closure, or G4 movement.
- Anchor files: `docs/hardware/findings/FINDINGS-pack-b-session-02-handoff-mirror.md`
- Validation: `get_errors` clean on `docs/hardware/findings/FINDINGS-pack-b-session-02-handoff-mirror.md`; findings record the named stand-in or bridge artifact reference, proposal and decision field refs, shell or log evidence, deny-log evidence, reboot note, and operator-attended note.
- Constraints: Keep all wording aligned with the current local-only handoff surfaces. Do not turn a named stand-in into real HA proof. Do not treat clean reboot as full power-cycle survival. Do not claim consent acceptance, D1 closure, or G4 movement. Run this lane only after the first physical baseline exists.
- Last update: 2026-04-30
- size: L
- serial-after: HARDWARE-2026-04-30-05
- solo: true
- track: hardware

### HARDWARE-2026-04-30-07

- Title: Separate Pack C HA visibility and recovery proof after DUT rehearsal
- Status: ready
- Priority: P1
- Gate: G4 separate-host HA lab proof prep
- Owning streams: S7 primary; S3 and S4 consumer awareness
- Goal: Run the first separate-host Pack C Home Assistant lab proof after the Pack B DUT rehearsal exists, capturing one real HA entity visibility result plus any later HA-side recovery observation while the DUT restart or cold-boot path is exercised. Record the separate-host topology, evidence references, and remaining gaps without claiming independent install proof, D1 closure, or G4 movement.
- Anchor files: `docs/hardware/findings/FINDINGS-pack-c-session-01-ha-visibility.md`
- Validation: `get_errors` clean on `docs/hardware/findings/FINDINGS-pack-c-session-01-ha-visibility.md`; findings record the named Pack B DUT, named Pack C HA host, HA entity or dashboard evidence reference, recovery observation if attempted, and claim ceiling.
- Constraints: Keep this as the first real HA-proof step and do not pull it earlier into the D1-style DUT rehearsal lanes. Do not claim independent install proof, private-beta readiness, G4 closure, or any result not captured on real hardware with a separate HA host. Run this lane only after the DUT-side handoff mirror exists.
- Last update: 2026-04-30
- size: L
- serial-after: HARDWARE-2026-04-30-06
- solo: true
- track: hardware

---

## In Progress

None.

---

## Blocked

None.

---

## Done

### HARDWARE-2026-04-27-01

- Title: D1 device-target inventory
- Status: done
- Priority: P1
- Gate: pre-D1 device selection
- Owning streams: S7 primary; S8 consumer awareness
- Goal: Document the candidate demo devices the project has access to and pick a primary D1 target. For each candidate: note the form factor, OS/firmware baseline, display capability, and known bring-up effort. Identify which device is the primary D1 target and why. Record any known blockers or required tools. This is a docs-only research and selection wave; no firmware or code lands.
- Anchor files: `docs/hardware/d1-target-inventory.md`
- Validation: `get_errors` clean on `docs/hardware/d1-target-inventory.md`.
- Constraints: Docs-only. Do not claim D1 evidence (D1 is being defined, not closed). Do not invent bridge protocol or HA fork internals.
- Last update: 2026-04-30
- size: S
- track: hardware

### HARDWARE-2026-04-27-03

- Title: First real-world UX session plan for D1
- Status: done
- Priority: P1
- Gate: pre-D1 UX planning
- Owning streams: S7 primary; S5 consumer awareness
- Goal: Define a 60-minute real-world UX session script for D1. The script must cover: (1) profile init via `ferros profile init` on the target device, (2) agent-center read via the localhost shell or CLI, (3) one HA entity registered through the planned bridge contract or documented stand-in, (4) one power cycle with confirmation that profile and agent state reload. Output is a session script the human can run verbatim; findings from the actual session get filed under `docs/hardware/findings/`. This wave produces the script only, not the findings.
- Anchor files: `docs/hardware/ux-sessions/d1-session-01-script.md`
- Validation: `get_errors` clean on the session script.
- Constraints: Docs-only. Do not claim D1 or G4 evidence. Do not invent bridge protocol internals. `parallel-safe-with: [HARDWARE-2026-04-27-01]`.
- Last update: 2026-04-30
- size: S
- parallel-safe-with: [HARDWARE-2026-04-27-01]
- track: hardware

### HARDWARE-2026-04-30-04

- Title: Name the first Pack B bring-up session from the local handoff
- Status: done
- Priority: P1
- Gate: pre-D1 named DUT session prep
- Owning streams: S7 primary; S8 consumer awareness
- Goal: Turn the local code-runway handoff into a named first hardware session by choosing the Pack B x86_64 device under test, the separate Pack C Home Assistant host, the operator station, and the storage, network, and DUT-only power-cut arrangement that will be used later. Capture the worksheet header, topology confirmation, pre-run checks, and operator rehearsal prep in one plan note without claiming any physical-device evidence, Home Assistant proof, D1 closure, or G4 movement.
- Anchor files: `docs/hardware/pack-b-session-01-plan.md`
- Validation: `get_errors` clean on `docs/hardware/pack-b-session-01-plan.md`; the plan names the DUT, HA host, operator station, storage path, network note, and DUT-only power-cut method or clearly marks each one as a required-before-execution placeholder; wording stays prep-only and future-facing.
- Constraints: Plan only. Do not execute hardware work in this wave. Preserve Pack B as the first DUT lane and Pack C as the separate HA host. Do not claim any physical-device evidence, HA dashboard proof, D1 closure, or G4 movement.
- Last update: 2026-04-30
- size: S
- serial-after: HARDWARE-2026-04-27-03
- track: hardware

### HARDWARE-2026-04-30-04A

- Title: Pack B command and artifact rehearsal map
- Status: done
- Priority: P1
- Gate: pre-D1 Pack B command rehearsal prep
- Owning streams: S7 primary; S8 consumer awareness
- Goal: Add a plan-only command and artifact rehearsal map for the future Pack B session that ties the existing local-code-runway commands, profile init and show references, and expected `.tmp/hub` artifact names to future DUT-side operator steps without claiming that any physical session has occurred.
- Anchor files: `docs/hardware/pack-b-session-01-command-map.md`
- Validation: `get_errors` clean on `docs/hardware/pack-b-session-01-command-map.md`; wording stays future-facing and operator-run only
- Constraints: Plan only. Run only after `HARDWARE-2026-04-30-04`. Do not record observed results, timestamps, screenshots, or evidence. Do not claim physical-device evidence, Home Assistant proof, D1 closure, or G4 movement.
- Last update: 2026-04-30
- size: S
- serial-after: HARDWARE-2026-04-30-04
- track: hardware

### HARDWARE-2026-04-30-04B

- Title: Pack B findings template
- Status: done
- Priority: P1
- Gate: pre-D1 Pack B baseline template prep
- Owning streams: S7 primary; S8 consumer awareness
- Goal: Create a blank findings template for the first future Pack B physical baseline wave so the eventual operator session can record DUT name, operator, command transcript, profile init and show results, local artifact references, failure notes, and the claim ceiling without inventing any physical result in advance.
- Anchor files: `docs/hardware/findings/FINDINGS-pack-b-session-01-profile-baseline.md`
- Validation: `get_errors` clean on `docs/hardware/findings/FINDINGS-pack-b-session-01-profile-baseline.md`; the template is explicitly blank and non-evidentiary until a real session fills it
- Constraints: Template only. Run only after `HARDWARE-2026-04-30-04A`. Do not insert any physical result, transcript, timestamp, operator identity, or evidence claim. Do not claim D1 closure, G4 movement, or Home Assistant proof.
- Last update: 2026-04-30
- size: S
- serial-after: HARDWARE-2026-04-30-04A
- track: hardware

### HARDWARE-2026-04-30-04C

- Title: Pack B handoff mirror template
- Status: done
- Priority: P1
- Gate: pre-D1 Pack B handoff mirror template prep
- Owning streams: S7 primary; S8 consumer awareness
- Goal: Create a blank findings template for the future DUT-side handoff mirror wave so the eventual operator session can record bridge artifact or stand-in output, proposal and decision fields, shell or log observation, deny visibility, reboot observation, and remaining gaps while clearly distinguishing clean reboot observation from full power-cycle survival.
- Anchor files: `docs/hardware/findings/FINDINGS-pack-b-session-02-handoff-mirror.md`
- Validation: `get_errors` clean on `docs/hardware/findings/FINDINGS-pack-b-session-02-handoff-mirror.md`; the template stays blank, future-facing, and explicit about clean reboot versus full power-cycle survival
- Constraints: Template only. Run only after `HARDWARE-2026-04-30-04B`. Do not insert any HA result, physical evidence, or executed consent claim. Do not claim D1 closure, G4 movement, or full power-cycle proof.
- Last update: 2026-04-30
- size: S
- serial-after: HARDWARE-2026-04-30-04B
- track: hardware

### HARDWARE-2026-04-30-04D

- Title: Pack C HA visibility template
- Status: done
- Priority: P1
- Gate: pre-G4 Pack C visibility template prep
- Owning streams: S7 primary; S8 consumer awareness
- Goal: Create a blank future Pack C Home Assistant visibility findings template that stays separate from the Pack B local rehearsal surfaces and can later capture the named Pack B DUT, named Pack C HA host, entity or dashboard reference, recovery observation, and claim ceiling without asserting any real HA proof in advance.
- Anchor files: `docs/hardware/findings/FINDINGS-pack-c-session-01-ha-visibility.md`
- Validation: `get_errors` clean on `docs/hardware/findings/FINDINGS-pack-c-session-01-ha-visibility.md`; the template stays blank and explicitly non-evidentiary until a real session fills it
- Constraints: Template only. Run only after `HARDWARE-2026-04-30-04C`. Keep Pack C visibility separate from Pack B local rehearsal. Do not claim real Home Assistant proof, physical-device evidence, D1 closure, G4 movement, or independent install evidence.
- Last update: 2026-04-30
- size: S
- serial-after: HARDWARE-2026-04-30-04C
- track: hardware
