# S2 Research Note — Profile Import/Export Round-Trip

**Date:** 2026-04-27  
**Owning stream:** S2 primary; S7 D1 planning  
**Output feeds:** D1 bring-up checklist (docs/research/S7-d1-bring-up-checklist.md), D1 evidence scripting  
**Status:** Research note — documents existing CLI commands and expected output for scripting D1 evidence.

---

## Purpose

This note provides the exact CLI commands, expected outputs, and error signatures for the `ferros profile` command lifecycle, framed as D1 evidence scripting. It draws from the frozen `profile.v0.json` schema contract and G2 closed CLI paths.

G2 is closed. This note does not reopen G2, add new CLI verbs, or modify any schema files.

---

## The Frozen Schema Contract

The profile data shape is governed by `schemas/profile.v0.json`. This schema is frozen as of G2 close. No field may be added, removed, or renamed without a schema evolution ADR.

**Key fields present in `profile.v0.json`:**
- `id` — the profile's stable identifier (e.g., a UUID or derived hash)
- `display_name` — human-readable name for the profile
- `created_at` — ISO-8601 timestamp

**Key fields NOT present:**
- Private keys or secrets — the profile record is not a keystore
- Capability grants — grants are tracked separately by the S4 runtime, not in the profile record

---

## CLI Command Lifecycle

### 1. Initialize a new profile

```bash
ferros profile init
```

**What happens:**
- Creates the profile file at the default path (`~/.ferros/profile.json` or platform equivalent).
- Generates a new profile id.
- Writes fields conforming to `profile.v0.json`.

**Expected successful output:**
```
Profile initialized at /home/<user>/.ferros/profile.json
```

**Expected error output (profile already exists):**
```
Error: profile already exists at /home/<user>/.ferros/profile.json
       Use `ferros profile show` to view the existing profile.
```

**D1 evidence note:** Run `ferros profile init` once per device. After initialization, do not re-run without a deliberate wipe — re-initialization would change the profile id and invalidate previously collected evidence.

---

### 2. Show the current profile

```bash
ferros profile show
```

**What happens:**
- Reads the profile from the default path.
- Prints all profile fields.

**Expected successful output:**
```
Profile
  id:           <profile-id>
  display_name: <name>
  created_at:   <ISO-8601 timestamp>
```

**Expected error output (no profile exists):**
```
Error: no profile found at /home/<user>/.ferros/profile.json
       Run `ferros profile init` to create one.
```

**D1 evidence note:** The output of `ferros profile show` before and after a power cycle must match (same `id`, `display_name`, `created_at`). Record the output in the D1 evidence transcript.

---

### 3. Export the profile to a file

```bash
ferros profile export <path>
```

**What happens:**
- Reads the profile from the default path.
- Writes it in JSON format to `<path>`, conforming to `profile.v0.json`.

**Expected successful output:**
```
Profile exported to <path>
```

**Expected error output (no profile):**
```
Error: no profile found. Run `ferros profile init` first.
```

**Expected error output (path not writable):**
```
Error: cannot write to <path>: permission denied
```

**Temp-file path convention:**  
For D1 evidence scripting, use a deterministic temp-file path so the exported file can be verified:
```bash
ferros profile export /tmp/ferros-profile-d1-export.json
cat /tmp/ferros-profile-d1-export.json
```

The `cat` output should match the fields printed by `ferros profile show`. This confirms the export is not lossy.

---

### 4. Import a profile from a file

```bash
ferros profile import <path>
```

**What happens:**
- Reads the JSON file at `<path>`.
- Validates it against the `profile.v0.json` schema.
- Writes it to the default profile path, replacing any existing profile.

**Expected successful output:**
```
Profile imported from <path>
  id:           <profile-id>
  display_name: <name>
  created_at:   <ISO-8601 timestamp>
```

**Expected error output (file does not exist):**
```
Error: cannot read <path>: no such file or directory
```

**Expected error output (schema validation failure):**
```
Error: profile import failed: invalid profile format (schema validation failed)
```

**D1 evidence scripting for round-trip proof:**
```bash
# Export
ferros profile export /tmp/ferros-profile-d1-export.json

# Wipe and re-import (only in scripted test, not in production D1 session)
rm ~/.ferros/profile.json
ferros profile import /tmp/ferros-profile-d1-export.json
ferros profile show
# Output must match the original `ferros profile show` output
```

---

## Grant / Revoke Commands

The profile record itself does not store capability grants. Grants are managed by the S4 runtime. These commands exist on the CLI but their outputs are separate from the profile round-trip:

```bash
ferros profile grant <capability> --profile-id <id>
# Grants capability to the specified profile. Recorded in the S4 grant store.

ferros profile revoke <capability> --profile-id <id>
# Revokes capability from the specified profile.
```

**Expected grant output:**
```
Capability <capability> granted to profile <id>
```

**Expected revoke output:**
```
Capability <capability> revoked from profile <id>
```

**D1 evidence note:** Grant/revoke is separate from the profile round-trip. For D1 consent-flow demonstration, issue a `ferros profile grant` for one capability and `ferros agent run` for an agent requiring a different (non-granted) capability to generate a deny-log entry.

---

## Full D1 Evidence Script

The following script can be run by the D1 operator to produce a transcript that covers all profile evidence requirements:

```bash
#!/bin/bash
# FERROS D1 Profile Evidence Script
# Run once on the target device. Save full terminal output as evidence.

echo "=== 1. Initialize profile ==="
ferros profile init

echo "=== 2. Show profile ==="
ferros profile show

echo "=== 3. Export profile ==="
ferros profile export /tmp/ferros-d1-profile.json

echo "=== 4. Verify exported JSON ==="
cat /tmp/ferros-d1-profile.json

echo "=== 5. Import profile (round-trip) ==="
ferros profile import /tmp/ferros-d1-profile.json

echo "=== 6. Show profile after import — must match step 2 output ==="
ferros profile show

echo "=== Done. Save this transcript to docs/gates/D1.md evidence section. ==="
```

**After power cycle:** Re-run only step 2 (`ferros profile show`) and confirm the output matches. Step 2 output before and after power cycle constitutes the reboot-safe profile persistence evidence.

---

## What NOT to Do

| Action | Why |
|---|---|
| Modify `schemas/profile.v0.json` | Schema is frozen; G2 is closed |
| Add new CLI verbs (`ferros profile <new-verb>`) | Requires reopening G2 scope via an S2 ADR |
| Record secrets or keys in the profile export | Profile is identity only, not a keystore |
| Use `ferros profile import` as a production restore path | Import is for D1 evidence scripting; production path is `ferros profile init` on the real device |

---

## Source Documents

- `schemas/profile.v0.json` — frozen profile schema (do not modify)
- `docs/gates/G2.md` — G2 closed gate evidence (do not reopen)
- `streams/S2-profile/README.md` — S2 stream scope
