# D1 Session 01 Script

> Plan-only operator script. This file is not evidence and does not claim D1 closure, G4 closure, physical-device evidence, Home Assistant proof, or full power-cycle survival.

## Scope

Use this script only after the named Pack B session details are filled in from `docs/hardware/pack-b-session-01-plan.md`. If any required placeholder remains blank, stop and fill the planning docs before execution.

This script covers the first operator-attended D1-style session flow only:

1. profile init or show on the target device,
2. agent-center read through the localhost shell or CLI,
3. one named stand-in or later real bridge-entity staging check,
4. one full DUT-only power cycle with confirmation that profile and agent state reload.

## Required before execution

- Pack B DUT name filled in: `[PLACEHOLDER - required before execution]`
- Pack C Home Assistant host name filled in: `[PLACEHOLDER - required before execution]`
- Operator station filled in: `[PLACEHOLDER - required before execution]`
- DUT repo path filled in: `[PLACEHOLDER - required before execution]`
- DUT-only power-cut method filled in: `[PLACEHOLDER - required before execution]`
- Planned findings target chosen:
  - `docs/hardware/findings/FINDINGS-pack-b-session-01-profile-baseline.md`
  - `docs/hardware/findings/FINDINGS-pack-b-session-02-handoff-mirror.md`
  - `docs/hardware/findings/FINDINGS-pack-c-session-01-ha-visibility.md`

## Session script

### 0. Open the session

- Confirm the operator, date, DUT name, HA host name, and working directory.
- Open the findings template that matches the intended session scope.
- Record that the run is operator-attended and future-facing until real outputs are captured.

### 1. Verify the profile boundary

Run on the target device:

```text
ferros profile init
ferros profile show
```

Record:

- whether `ferros profile init` succeeded,
- the reported profile path,
- whether `ferros profile show` returned the expected fields.

If `ferros profile init` or `ferros profile show` fails, stop and record the failure. Do not infer success from prior local rehearsal.

### 2. Verify the agent-center read path

Choose one observation path and record which one was used:

- localhost shell route on the DUT, or
- CLI reads such as `ferros agent list`, `ferros agent describe <name>`, or `ferros agent logs <name>`.

Record:

- the command or route used,
- whether the observation surface was readable,
- any named stand-in or bridge agent reference shown.

### 3. Verify one named stand-in or bridge staging check

Use either:

- a clearly named stand-in, or
- the future real bridge path once it exists.

Record only what was actually visible:

- agent name,
- proposal or decision reference if present,
- whether the item is still staged or proposed material only.

Do not record this step as Home Assistant proof unless a real Pack C host result exists and is captured separately.

### 4. Verify deny visibility

Use the planned deny observation path from the session plan.

Possible operator actions:

- observe the deny-log slot on the shell, or
- run `ferros agent logs <name>` after a denied request.

Record:

- what observation path stayed open,
- whether one denied request was visible,
- whether the operator could distinguish a deny from a generic system error.

### 5. Verify full DUT-only power-cycle observation

This D1-style session step is complete only after a full DUT-only power cycle. A clean reboot may be used as an optional pre-check, but it does not satisfy this session script on its own.

Required power-cycle reference:

```text
[PLACEHOLDER - exact DUT-only power-cut method required before execution]
```

After the device returns, rerun:

```text
ferros profile show
ferros agent list
```

Record only the observed result, including whether at least one agent returned in the post-power-cycle agent list.

### 6. Close the session

- Mark which findings template was updated.
- Record unresolved gaps.
- Restate the claim ceiling: no D1 closure, no G4 closure, no Home Assistant proof, and no physical-device evidence beyond the exact session output captured.

## Claim ceiling

- This script is a future operator procedure, not evidence.
- A named stand-in is not real Home Assistant proof.
- A clean reboot is not full power-cycle survival and does not satisfy the required D1-style power-cycle step in this script.
- This script does not authorize remote transport, canonical mutation, or accept or reject transport.