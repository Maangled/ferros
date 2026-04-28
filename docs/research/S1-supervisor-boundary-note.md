# S1 Research Note - Supervisor Boundary Note

**Date:** 2026-04-28
**Owning stream:** S1 primary; S4 consumer awareness
**Output feeds:** Runtime hardening and future install/service planning
**Status:** Boundary note only. No service manager or launch installer is claimed.

---

## Purpose

This note keeps the word "supervisor" from drifting ahead of repo truth. FERROS currently has a local binary, CI workflows, and a localhost shell host. It does not yet have a production service supervisor, reboot-start unit, installer, or launch package.

The boundary matters because D1 and G4 both talk about restart and power-cycle behavior. Restart evidence must not silently imply that S1 has already shipped a daemon model.

---

## Current Supervisory Surfaces

| Surface | What it currently provides | What it does not provide |
|---|---|---|
| `.github/workflows/ci.yml` | Hosted validation for build/test/fmt/clippy and selected runtime checks | Runtime supervision on a user device |
| `.github/workflows/release.yml` | Manual source bundle workflow and artifact manifest | A device install, tag creation, or service unit |
| `cargo xtask ci` | Local developer verification path | Runtime process monitoring |
| `ferros-node shell [port]` | Localhost HTTP shell host while the process is running | Auto-start, restart-on-failure, auth, TLS, or remote serving |
| `ferros` CLI | Profile and agent commands | Long-running hub process ownership |

S1 owns the foundation and packaging discipline. S4/S7 own runtime and hub semantics that may later need a supervisor.

---

## Current Honest Language

Use these phrases:

- "local process"
- "localhost shell host"
- "manual run"
- "operator-started session"
- "future service boundary"
- "supervisor not yet published"

Avoid these phrases until implementation exists:

- "daemon"
- "installed service"
- "systemd unit"
- "launch agent"
- "auto-restart"
- "production supervisor"
- "unattended hub"

---

## Future Supervisor Entry Bar

A future S1/S4/S7 supervisor wave should not be called landed until it names all of the following:

| Requirement | Minimum honest detail |
|---|---|
| Process owner | Which binary is supervised and under which user account |
| Startup command | Exact command and working directory |
| State paths | Profile, grant, agent state, and logs live on persistent storage |
| Restart policy | What happens on clean exit, crash, reboot, and full power loss |
| Logs | Where stdout/stderr and FERROS event logs are collected |
| Network binding | Localhost-only vs LAN-visible behavior |
| Validation | Focused test or session proof for the selected platform |

Until those rows exist, D1 scripts should say "operator starts FERROS manually" and G4 should remain open.

---

## D1 Relationship

D1 does not require a production supervisor. D1 can be operator-attended and may start FERROS manually after boot if the evidence table records the operator steps honestly.

However, the D1 power-cycle requirement still says FERROS-side state must survive a full power cycle. That is a persistence claim, not a supervisor claim. The difference is important:

| Claim | D1 can accept? | Notes |
|---|---|---|
| Profile persists after reboot | Yes | Proven with `ferros profile show` after power cycle |
| At least one agent re-registers without manual intervention | Yes | Depends on current startup path and session script |
| FERROS starts as a production service | No | Future install/supervisor work |
| HA re-registration after power cycle | No | G4 requirement, not D1 |

---

## G4 Relationship

G4 eventually needs a launch-grade hub path. A manual `ferros-node shell` process is not enough for launch evidence unless the launch criteria are explicitly satisfied by the tested hardware path and documented install steps.

Before G4, S7 should either:

- publish a real hub process that owns restart behavior, or
- explicitly record why the chosen launch package satisfies equivalent supervision requirements.

No such claim exists today.

---

## Handoff

S4 can use this note when describing restart/reload boundaries: reload helpers prove local state handling, not service supervision. S7 can use this note when scripting D1 power-cycle rehearsal so the operator transcript separates "state survived" from "process was supervised."

