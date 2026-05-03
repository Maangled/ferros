# REENTRY-PHASE0 Dependency Drift Audit

Status: complete
Date: 2026-05-03
Mode: strict initiative policy

## Audit Scope
- Root workspace manifest: Cargo.toml
- Root lockfile: Cargo.lock
- Crate manifests under crates/*/Cargo.toml
- Crate lockfiles under crates/*/Cargo.lock
- npm manifests/lockfiles (package.json, package-lock.json, yarn.lock, pnpm-lock.yaml, npm-shrinkwrap.json)

## Inventory Found
Cargo manifests:
- Cargo.toml
- crates/ferros-core/Cargo.toml
- crates/ferros-runtime/Cargo.toml
- crates/ferros-profile/Cargo.toml
- crates/ferros-agents/Cargo.toml
- crates/ferros-node/Cargo.toml
- crates/ferros-data/Cargo.toml
- crates/ferros-hub/Cargo.toml
- xtask/Cargo.toml

Cargo lockfiles:
- Cargo.lock
- crates/ferros-data/Cargo.lock

npm manifests/lockfiles in ferros:
- None found

Workspace members declared in Cargo.toml:
- xtask
- crates/ferros-core
- crates/ferros-runtime
- crates/ferros-profile
- crates/ferros-agents
- crates/ferros-node
- crates/ferros-data
- crates/ferros-hub

## Compliance Result (Strict Mode)
- No npm manifest presence in ferros workspace: pass.
- No npm lockfile presence in ferros workspace: pass.
- No new workspace-member drift observed in current manifest set: pass.
- No dependency additions were made by this segment: pass.
- No lockfile edits were made by this segment: pass.

## Violations
- None detected in this segment.

## Severity and Action
- Severity: none
- Gate action: continue

## Claims Added
- Segment-level strict-mode compliance snapshot for current ferros repo state.

## Claims Explicitly Not Added
- No statement about historical dependency drift outside this segment.
- No claim about non-ferros repositories.
- No permission to relax strict mode.

## HANDOFF CARD
- Lane ID: D1
- Status: complete
- Files read: Cargo.toml; Cargo.lock; crates/*/Cargo.toml; crates/*/Cargo.lock; npm manifest patterns
- Files changed: docs/orchestration/REENTRY-PHASE0-DEPENDENCY-AUDIT.md
- Evidence produced: strict dependency audit report for phase 0
- Claims added: no drift introduced by this segment
- Claims explicitly not added: historical drift claims or policy relaxation
- Validation: manifest inventory and strict-mode checklist review
- Residual risks: future lanes must keep manifests unchanged unless explicitly approved
- Next safe follow-up, if any: run claim red-team against lane outputs
