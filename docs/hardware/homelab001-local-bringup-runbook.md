# homelab001 Phase 0 Local Bring-up Runbook

Status: Draft
Date: 2026-05-03
Scope: local bring-up on `homelab001`

## Purpose
Use the current repo checkout on `homelab001` to prove the local `ferros` profile and `ferros-hub` readout path before any separate-host Home Assistant validation.

## Claim Ceiling
- Local bring-up only.
- Same-machine Home Assistant, if used, is optional local rehearsal only.
- No separate-host Home Assistant proof.
- No D1 closure.
- No G4 closure.
- No launch-readiness claim.

## Known Local Facts
- Operator: `Maangled`
- Repo path: `/home/homelab001/apps/ferros`
- Local state path: `/home/homelab001/apps/ferros/.local-state`
- Local artifact path: `/home/homelab001/apps/ferros/.local-artifacts`
- Temporary Home Assistant mode: co-located on `homelab001` if used

## Pre-Run Setup

```bash
cd /home/homelab001/apps/ferros
mkdir -p .local-state .local-artifacts/reentry-homehub-local-01
```

## Step 1. Refresh The Local Hub Rehearsal Chain

Use `--keep-artifacts` so the known `.tmp/hub` files remain available for inspection and copying.

```bash
cd /home/homelab001/apps/ferros
cargo xtask hub-runway --keep-artifacts \
  | tee .local-artifacts/reentry-homehub-local-01/xtask-hub-runway.txt
cp -f .tmp/hub/*.json .local-artifacts/reentry-homehub-local-01/
```

Expected local artifact set:
- `.tmp/hub/simulated-local-bridge-artifact.json`
- `.tmp/hub/local-hub-state-snapshot.json`
- `.tmp/hub/local-onramp-proposal.json`
- `.tmp/hub/local-onramp-decision-receipt.json`

## Step 2. Initialize A Local Profile In Repo-Owned State

```bash
cd /home/homelab001/apps/ferros
cargo run -p ferros-node --bin ferros -- \
  profile init /home/homelab001/apps/ferros/.local-state/homelab001-profile.json \
  | tee .local-artifacts/reentry-homehub-local-01/profile-init.txt
```

## Step 3. Read The Local Profile Back

```bash
cd /home/homelab001/apps/ferros
cargo run -p ferros-node --bin ferros -- \
  profile show /home/homelab001/apps/ferros/.local-state/homelab001-profile.json \
  | tee .local-artifacts/reentry-homehub-local-01/profile-show.txt
```

## Step 4. Read The Local Hub Summary

```bash
cd /home/homelab001/apps/ferros
cargo run -p ferros-hub -- summary \
  | tee .local-artifacts/reentry-homehub-local-01/hub-summary.txt
```

## Step 5. Record The Local Bridge Rehearsal Output

This is still local rehearsal only.

```bash
cd /home/homelab001/apps/ferros
cargo run -p ferros-hub -- prove-bridge \
  | tee .local-artifacts/reentry-homehub-local-01/hub-prove-bridge.txt
```

## Step 6. Inspect Agent Visibility

```bash
cd /home/homelab001/apps/ferros
cargo run -p ferros-node --bin ferros -- agent list \
  | tee .local-artifacts/reentry-homehub-local-01/agent-list.txt
```

The current validated local bridge agent name is `ha-local-bridge`. If `agent list` shows a different local name, use the observed name and record the deviation in findings.

```bash
cd /home/homelab001/apps/ferros
cargo run -p ferros-node --bin ferros -- agent describe ha-local-bridge \
  | tee .local-artifacts/reentry-homehub-local-01/agent-describe.txt
```

## Step 7. Optional Deny Visibility Check

```bash
cd /home/homelab001/apps/ferros
cargo run -p ferros-hub -- deny-demo \
  | tee .local-artifacts/reentry-homehub-local-01/hub-deny-demo.txt
```

## Step 8. Optional Co-Located Home Assistant Note

If Home Assistant is running on `homelab001`, record only whether it was co-located during the session. Do not claim separate-host proof, dashboard visibility, or recovery proof from this note alone.

## Capture Targets

Fill [ferros/docs/hardware/findings/FINDINGS-homelab001-local-bringup.md](ferros/docs/hardware/findings/FINDINGS-homelab001-local-bringup.md) with:
- date
- operator
- exact commands used
- profile init result
- profile show result
- hub summary result
- hub prove-bridge result
- agent list result
- copied artifact paths
- optional co-located Home Assistant note
- optional LAN-device observation note
- failures and remaining gaps

## Non-Claims
- Do not claim separate-host Home Assistant proof.
- Do not claim device control.
- Do not claim Matter support.
- Do not claim network telemetry beyond the captured local outputs and notes.
- Do not claim D1 or G4 closure.

## HANDOFF CARD
- Lane ID: A1
- Status: complete
- Files read: docs/hardware/pack-b-session-01-command-map.md; docs/hardware/pack-b-session-01-plan.md; docs/orchestration/REENTRY-PHASE0-HARDWARE-READINESS-AUDIT.md; crates/ferros-hub/src/main.rs; crates/ferros-node/src/bin/ferros.rs; docs/hub/local-code-runway-inventory.md; tools/README.md; xtask/src/main.rs
- Files changed: docs/hardware/homelab001-local-bringup-runbook.md
- Evidence produced: repo-backed local bring-up runbook for homelab001
- Claims added: exact local command sequence and capture targets for profile and hub bring-up
- Claims explicitly not added: separate-host Home Assistant proof, D1 closure, G4 closure, Matter support, device control
- Validation: command-form audit against current CLI sources and helper docs
- Residual risks: real session outputs still need operator capture in findings
- Next safe follow-up, if any: fill the local findings template after commands are run on homelab001