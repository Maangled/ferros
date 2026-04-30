# PowerShell Build Tools

## Requirement

FERROS uses two PowerShell scripts for code generation. These scripts generate deterministic output and must be re-run whenever their inputs change.

### Runtime

The scripts require **Windows PowerShell 5.1** (ships with Windows 10/11). They are also compatible with **PowerShell Core 7+** (`pwsh`), but `pwsh` may not be installed by default.

Execution command (Windows):
```
powershell -ExecutionPolicy Bypass -File tools\generate-harness-constants.ps1
powershell -ExecutionPolicy Bypass -File tools\generate-ferros-core.ps1
```

The `-ExecutionPolicy Bypass` flag is required because the default policy on Windows blocks unsigned scripts. This is a local development tool — the scripts do not run in production.

### Wave 1 Cross-Platform Alternative

For macOS/Linux contributors who cannot run PowerShell:

1. **Install PowerShell Core:** `brew install powershell` (macOS) or `sudo apt install powershell` (Debian/Ubuntu). Then run with `pwsh -File tools/generate-harness-constants.ps1`.

2. **Future: Node.js Port.** The generators are pure file-read + string-template + JSON operations. A Node.js port is straightforward and would eliminate the PowerShell dependency. This is tracked as a Wave 1 task.

---

## Scripts

### `run-stream-a-baseline.ps1`

**Purpose:** Runs Stream A baseline lock checks by executing both generators and failing if generated artifacts drift from committed output.

**When to run:** Before starting Stream A implementation work and after any schema/template edits.

**Usage:**
```
powershell -ExecutionPolicy Bypass -File tools\run-stream-a-baseline.ps1
```

**What it checks:**
- Runs `generate-harness-constants.ps1`
- Runs `generate-ferros-core.ps1`
- Verifies no drift in:
	- `harnesses/_constants.js`
	- `docs/assets/_core/ferros-core.js`

If drift is detected, the script exits non-zero and prints the files that must be reviewed.

---

### `generate-harness-constants.ps1`

**Purpose:** Reads all schemas (`schemas/*.schema.json`) and fixtures (`schemas/fixtures/*.json`), then outputs `harnesses/_constants.js` containing `var SCHEMA_*` and `var FIXTURE_*` declarations.

**When to run:** After any schema or fixture change.

**Usage:**
```
powershell -ExecutionPolicy Bypass -File tools\generate-harness-constants.ps1
```

**Optional parameters:**
- `-OutFile <path>` — Override output path (default: `harnesses/_constants.js`)
- `-RepoRoot <path>` — Override repo root (default: script's parent directory)

**Naming convention:**
- `profile.schema.json` → `SCHEMA_PROFILE`
- `full-profile-stage3.json` → `FIXTURE_FULL_PROFILE_STAGE3`
- `invalid-broken-seal-chain.json` → `FIXTURE_INVALID_BROKEN_SEAL_CHAIN`

**Drift rule:** Non-empty `git diff harnesses/_constants.js` after regeneration = stale harness.

---

### `generate-ferros-core.ps1`

**Purpose:** Reads `docs/assets/_core/templates.json`, validates each template against `schemas/template.schema.json`, and embeds the template array into `docs/assets/_core/ferros-core.js` at the `TEMPLATE_PROFILES` placeholder.

**When to run:** After any change to `templates.json` or `template.schema.json`.

**Usage:**
```
powershell -ExecutionPolicy Bypass -File tools\generate-ferros-core.ps1
```

**Optional parameters:**
- `-RepoRoot <path>` — Override repo root

**Validation:** The script performs basic field validation against the template schema (required fields, aliasClass enum, stream enum) and exits with error if any template is invalid.

---

## Verification

After running either generator, verify freshness:
```
git diff harnesses/_constants.js
git diff docs/assets/_core/ferros-core.js
```

An empty diff confirms the committed output matches the current inputs. A non-empty diff means either:
1. The generator was run after inputs changed (expected — commit the new output).
2. The committed output was hand-edited (not allowed — regenerate and commit).

---

## Local Hub Runway Helper

`cargo xtask hub-runway` is the local rehearsal helper for the published hub-owned restart, proposal, and decision seams.

By default it now does three things in one pass:
- validates the current restart-aware hub summary and the emitted local onramp proposal and decision artifacts
- prints a deterministic `.tmp/hub` artifact inventory in its stdout
- restores the four known `.tmp/hub` artifacts to their pre-run state after validation so the helper does not leave ambiguous generated local files behind

The helper treats the following four files as the only expected closeout inventory under `.tmp/hub`. If any other file remains there after the helper run, `cargo xtask hub-runway` now fails so the residue is surfaced explicitly instead of being silently ignored.

Usage:
```
cargo xtask hub-runway
cargo xtask hub-runway --keep-artifacts
```

Use `--keep-artifacts` only when you intentionally want the generated `.tmp/hub` files left in place for manual inspection. Without that flag, the helper restores or removes the known hub artifact files after it finishes.

The deterministic artifact set is:
- `.tmp/hub/simulated-local-bridge-artifact.json`
- `.tmp/hub/local-hub-state-snapshot.json`
- `.tmp/hub/local-onramp-proposal.json`
- `.tmp/hub/local-onramp-decision-receipt.json`
