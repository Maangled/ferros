# S1 Research Note - Boot Sequence for the D1 Target

**Date:** 2026-04-28
**Owning stream:** S1 primary; S7 consumer awareness
**Output feeds:** S7 power-cycle recovery protocol; D1 device/session planning
**Status:** Research note only. This does not claim D1 or G4 evidence.

---

## Purpose

This note names the minimum boot-sequence expectations for the first D1 target path. It keeps the S1 foundation boundary honest: S1 can say the workspace, CI foundation, toolchain pin, and local binary paths exist, but the actual target-device boot evidence belongs to a human/device session.

D1 is a demo gate. It can use an x86_64 stand-in for the first proof run if the real FERROS binary runs there and the evidence is recorded accurately. G4 remains stricter and requires launch hardware plus Home Assistant evidence.

---

## Current Repo-Backed Inputs

| Input | Current truth | Evidence source |
|---|---|---|
| Workspace foundation | Cargo workspace, toolchain pin, CI workflow, and xtask runner exist | `streams/S1-foundation/README.md`; `STATUS.md` |
| G1 state | Closed from hosted CI run #24812246339 | `STATUS.md`; `docs/gates/G1.md` |
| Binary path | `ferros` binary is in `crates/ferros-node` and has profile/agent/demo commands | S2/S3/S4 progress docs |
| D1 profile script | Existing profile init/show/export/import command sequence is documented | `docs/research/S2-profile-import-export-round-trip.md` |
| D1 target runway | Pack B x86_64 is the observability-first target class | `docs/hub/reference-hardware.md` |

S1 does not own profile semantics, runtime reload, HA registration, or consent enforcement. This note only frames the boot path that must be boring before those downstream checks matter.

---

## Boot-Sequence Checkpoints

| Checkpoint | Operator checks | Owner |
|---|---|---|
| Toolchain present | `rustc --version`, `cargo --version`, and the repo root can run Cargo commands | S1/session |
| Workspace builds or binary is available | Either `cargo run -p ferros-node --bin ferros -- demo` runs on target, or a prebuilt binary is present and versioned in the session notes | S1/S4 |
| Persistent state path exists | A writable, non-temp directory is chosen for profile and agent state | S2/S7 |
| Profile bootstrap starts cleanly | `ferros profile init` succeeds once | S2 |
| Profile reads back | `ferros profile show` returns the same profile in a reopened shell | S2 |
| Agent shell/read path starts | `ferros agent list` or the localhost shell can read registry state | S3/S5 |
| Deny path can be observed | A denied lifecycle attempt can be logged and viewed | S3/S4/S5 |
| Reboot handoff is ready | The operator knows the clean reboot and cold-boot method before collecting power-cycle evidence | S7 |

Only the first two checkpoints are S1-facing foundation checks. The remaining rows are downstream checks that depend on S2/S3/S4/S5/S7 surfaces.

---

## Minimal Boot Transcript Shape

This is a transcript shape for a D1 operator script. It is not evidence until run on a named target device.

```bash
rustc --version
cargo --version
cargo run -p ferros-node --bin ferros -- demo

ferros profile init
ferros profile show

ferros agent list
ferros agent logs echo
```

If the device uses a prebuilt `ferros` binary instead of `cargo run`, the session notes must record:

- binary source or build commit
- target architecture and OS
- profile state path
- operator name and date
- whether the device is Pack B x86_64, Pack A aarch64, or another named stand-in

---

## What Is Already Strong Enough

- The workspace foundation is not the bottleneck for D1 planning.
- Pack B x86_64 is a valid first D1 stand-in if the real FERROS binary runs there.
- The boot transcript can begin with the existing `ferros` binary and current profile/agent commands.
- D1 does not need branch protection, release tagging, or an installer to be rehearsed.

---

## What Remains Session-Owned

| Gap | Why it is not S1 evidence |
|---|---|
| Target device selection | Hardware queue remains parked until the human names a device/session window |
| Persistent profile path | Depends on the chosen device filesystem |
| Startup supervision | No service manager or launch installer exists yet |
| Power-cycle recovery | Requires a real reboot or power cut |
| HA entity registration | Belongs to S7 bridge runway and D1 stand-in rules |

---

## Stop Lines

Do not use this note to claim:

- D1 is closed.
- G4 evidence exists.
- `ferros-hub` exists.
- A production supervisor or install service exists.
- `v0.0.1-foundation` has been tagged.
- Branch protection is verified.

---

## Downstream Handoff

S7 can consume this note as the boot-sequence input to the power-cycle recovery protocol. That protocol should treat this note as "pre-reboot baseline collection," not as evidence that reboot-safe behavior has been proven.

