# S7 Research Note — D1 Bring-Up Checklist

**Date:** 2026-04-27  
**Owning stream:** S7 primary  
**Output feeds:** HARDWARE-QUEUE UX session plan (HARDWARE-2026-04-27-03), firmware spike plan (HARDWARE-2026-04-27-02)  
**Status:** Research note — not a gate-close claim. D1 is still active runway.

---

## Purpose

This note synthesizes the D1 gate evidence requirements from `docs/gates/D1.md`, the Pack B bring-up worksheet (`docs/hub/pack-b-bring-up-worksheet.md`), and S7 runway docs into a single operator-facing checklist. It is intended for use when scripting the D1 UX session plan and when planning firmware spike milestones.

D1 does not require unattended operation. An x86_64 home-server or laptop can stand in for a Raspberry Pi for the first D1 proof run, provided the same FERROS binary is used and the evidence is documented accurately.

---

## D1 Evidence Requirements (from `docs/gates/D1.md`)

All four must be true simultaneously and documented in `docs/gates/D1.md` before D1 is declared closed.

| # | Evidence item | Definition |
|---|---|---|
| 1 | Profile creation / show | `ferros profile init` and `ferros profile show` run successfully on the target device using the real FERROS binary and the frozen `profile.v0.json` contract |
| 2 | HA entity registered (or named stand-in) | At least one Home Assistant entity registered through the planned bridge contract or a documented stand-in; the stand-in must be named; a pure mock without a defined bridge seam does not count |
| 3 | Consent flow visible | The consent/capability grant flow is visible to the operator; deny-by-default enforcement demonstrable (one denied request logged and visible) |
| 4 | Reboot-safe FERROS-side state | Device goes through one full power cycle; after reboot, the FERROS profile loads and at least one agent re-registers without manual intervention |

**HA re-registration after power cycle is NOT required for D1 (it is required for G4).**

---

## Evidence Item 1 — Profile Creation / Show

### FERROS binary commands

```
ferros profile init
ferros profile show
```

### What a passing result looks like

- `ferros profile init` completes without error and reports the profile path, e.g.:
  ```
  Profile initialized at /home/<user>/.ferros/profile.json
  ```
- `ferros profile show` prints the profile fields (id, display name, key hash) matching the `profile.v0.json` schema.
- Running `ferros profile show` a second time after reopening the shell produces the same output (persistence confirmed within the session).

### What a failing result looks like

- Exit code non-zero on `ferros profile init`.
- `ferros profile show` prints "No profile found" or returns an error.
- Profile fields differ between two `show` calls in the same session (write did not persist).

### Known unknowns / blockers

- On the chosen D1 target, the profile path may need to be explicitly set if the default `$HOME/.ferros/` path is not writable (e.g., read-only rootfs).
- If the target device runs as a different user, confirm the binary binary has write access to the profile directory.

---

## Evidence Item 2 — HA Entity Registered (or Named Stand-In)

### Context

The `ferros-hub` binary and the HA bridge implementation are not yet built. For D1, a **named stand-in** is acceptable. The stand-in must be documented in the D1 evidence section of `docs/gates/D1.md`.

### Acceptable stand-in for D1

A stand-in that satisfies D1 must:
1. Have a defined bridge seam — the stand-in represents the same data contract the real HA bridge will use.
2. Be named explicitly in the evidence (e.g., "Stand-in: local echo agent acting as HA bridge shim, registered as `ha-bridge` with capability `agent.ha-bridge`").
3. Demonstrate that the FERROS agent center can accept a third-party agent registration — even if that agent is the echo agent acting as a shim.

### FERROS binary commands for stand-in

```
ferros agent list                    # confirm echo/timer/ha-bridge-shim registered
ferros agent describe ha-bridge      # show manifest fields
ferros agent run ha-bridge           # start the shim (if it requires agent.ha-bridge grant)
```

### What a passing result looks like

- `ferros agent list` shows the stand-in agent name.
- `ferros agent describe <name>` shows its manifest with named capability requirements.
- The stand-in is documented by name in `docs/gates/D1.md` evidence table.

### What a failing result looks like

- No named stand-in (a "pure mock" with no defined bridge seam does not count).
- The agent list does not show the stand-in after init.

---

## Evidence Item 3 — Consent Flow Visible

### What the operator must demonstrate

- The deny-log is visible in the localhost shell UI or via `ferros agent logs`.
- At least one denied request is in the log (pre-seeded via a local CLI lifecycle attempt against an agent whose capability is not granted).
- The operator can point to the deny-log slot and say "this is deny-by-default enforcement."

### FERROS binary commands

```
# Pre-seed a denial:
ferros agent run <agent-without-grant>
# Observe in shell: deny-log slot shows denied-start entry
# Or via CLI:
ferros agent logs <agent-name>
```

### What a passing result looks like

- Shell deny-log slot shows at least one `denied-start:` entry.
- `ferros agent logs <name>` output contains a denial line.
- The operator can demonstrate the deny is from capability enforcement, not a system error.

### What a failing result looks like

- Deny log is empty even after a failed `agent run` attempt.
- The denial is displayed as a generic error rather than a capability-enforcement event.

### Current state

The deny-by-default enforcement is code-backed and tested (`test agent_cli_` in `ferros-node`). The localhost shell renders the deny-log through the `denyLog.list` JSON/RPC endpoint. The consent-flow copy spec (from WAVE-2026-04-27-08) defines draft language for the shell slots, pending counsel red-line.

---

## Evidence Item 4 — Reboot-Safe FERROS-Side State

### What the operator must demonstrate

1. Note the current profile state (`ferros profile show`).
2. Execute a full power cycle (power off, power on — not just a process restart).
3. After reboot, run `ferros profile show` — it must return the same profile without manual intervention.
4. Run `ferros agent list` — at least one agent must be registered (re-registers on startup without manual prompt).

### FERROS binary commands

```
# Before power cycle:
ferros profile show      # note profile id
ferros agent list        # note registered agents

# After power cycle (no manual profile init):
ferros profile show      # must match pre-cycle output
ferros agent list        # at least one agent must be registered
```

### What a passing result looks like

- Profile id matches after power cycle.
- Agent list is non-empty without running `ferros profile init` again.

### What a failing result looks like

- `ferros profile show` returns "No profile found" after power cycle.
- `ferros agent list` is empty after power cycle and requires manual re-registration.

### Current state

The reload-boundary tests (`reload_boundary_runtime_with_state_`, `reload_boundary_load_local_profile`) prove that the runtime can reload state from a local path. The reboot-safe claim requires that the profile path survives a power cycle — this depends on the target device's filesystem (not volatile RAM, not tmpfs).

---

## Firmware Spike Milestones (HARDWARE-2026-04-27-02 input)

For planning the firmware spike on the chosen D1 device, the four milestones below map directly to the D1 evidence items:

| Milestone | Maps to D1 evidence | Definition |
|---|---|---|
| Boot | Pre-condition | FERROS binary starts on the target device without crash |
| Identify | Evidence item 1 | `ferros profile init` and `ferros profile show` succeed |
| Accept-grant | Evidence item 3 | At least one capability grant issued; deny-by-default demonstrable |
| Report-state | Evidence item 4 | State survives power cycle |

---

## Not in Scope for D1

- `ferros-hub` binary (not yet built)
- Real HA bridge implementation (stand-in is acceptable for D1)
- Multi-device operation (G4 requirement, not D1)
- Unattended operation (G4 requirement, not D1)
- Independent private-beta install (G4 requirement, not D1)

---

## Source Documents

- `docs/gates/D1.md` — authoritative evidence requirements
- `docs/gates/G4.md` — launch criteria (not D1)
- `docs/hub/pack-b-bring-up-worksheet.md` — operator evidence surface source
- `docs/hub/reference-hardware.md` — hardware recipe authority
- `streams/S7-hub/README.md` — operator evidence surface definition
