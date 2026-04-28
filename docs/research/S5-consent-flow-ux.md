# S5 Research Note — Consent Flow UX

**Date:** 2026-04-27  
**Owning stream:** S5 primary; S4 policy-engine runtime; S7 D1 planning  
**Output feeds:** D1 bring-up checklist (docs/research/S7-d1-bring-up-checklist.md), consent copy spec  
**Status:** Research note — describes existing behavior only. Does not add shell features or modify consent copy.

---

## Purpose

This note documents what the operator sees today in the FERROS localhost shell with respect to consent and capability enforcement. It is intended to:
1. Support D1 demo scripting (the operator must demonstrate consent flow visibility).
2. Record the current shell slot structure for S5 UX continuity.
3. Identify gaps between the current read-only shell and the full consent flow a D1 demo requires.

---

## What the Localhost Shell Shows Today

### Shell endpoint

The localhost shell is served by `ferros-node` at `http://localhost:<port>/agent-center-shell.html`. It is static HTML that polls the local JSON/RPC backend.

**Source asset:** `site/agent-center-shell.html`  
**Backend:** `crates/ferros-node/` (`/rpc` route)

### Visible slots (Phase B — landed)

| Slot | What it shows | JSON/RPC endpoint |
|---|---|---|
| Agent list | All registered agents with name and status | `agent.list` or `agent.snapshot` |
| Agent detail | Per-agent manifest: name, version, required capabilities | `agent.describe` |
| Capability grant state | Active grants for the selected agent | `agent.snapshot` |
| Deny log | Log of capability-denied events (deny-by-default enforcement visible) | `denyLog.list` |
| Selected-agent intent copy | Shell copy staging intent to run/stop the selected agent (read-only affordances, no browser-issued write RPC) | None — intent staging only |

### What is NOT in the shell today

- Browser-issued `agent.run` or `agent.stop` write RPC — the shell stages intent copy and affordances but does not send writes.
- Grant/revoke actions — operator cannot grant or revoke capability grants through the shell.
- Consent resolution (accept/reject) surface — S5 Phase B defers this until a code-backed follow-up.
- Remote transport — the shell is localhost-only; no off-device access.
- Live deny generation — denials can only be observed when pre-seeded through the existing local lifecycle/log seam.

---

## Deny-by-Default Enforcement — What the Operator Sees

### Mechanism

The `DenyByDefaultPolicy` (from `ferros-core`) is the only policy in force today. Every capability request is evaluated against the current grant set. If no matching active grant exists for the requester profile + capability pair, the request is denied and a `PolicyDenialReason` is recorded.

The three denial reasons (from `PolicyDenialReason`):

| Reason | Meaning |
|---|---|
| `NoGrantsPresented` | No active grants in the grant set at all (or all grants are inactive) |
| `ProfileNotGranted` | The requesting profile is not listed in any active grant |
| `CapabilityNotGranted` | The requesting profile has active grants, but not for the requested capability |

### How the operator observes a denial

1. **Localhost shell deny-log slot** (`denyLog.list`): Each entry shows the agent name, the denied capability, and the denial reason. The slot updates on next shell refresh.
2. **CLI** (`ferros agent logs <name>`): The CLI output includes denial events from the agent's log history.

### Pre-seeding a denial for D1 demo

The shell does not generate denials autonomously. To demonstrate a denial for a D1 evidence session:

1. Start an agent that requires a capability not yet granted:
   ```
   ferros agent run <agent-name>
   ```
   — if the agent's required capability is not in the grant set, the start attempt is denied.
2. Refresh the shell deny-log slot — the denial should appear.
3. Alternatively, use `ferros agent logs <agent-name>` from the CLI.

**Note:** The denial must be from capability enforcement (reason logged), not a process error. A crash or binary-not-found is not an acceptable D1 consent-flow demonstration.

---

## Consent Language (Draft)

The consent language for the shell deny-log and capability slots is in draft as of the WAVE-2026-04-27-08 output. The draft copy is recorded in `docs/legal/CONSENT-LANGUAGE.md` (pending counsel red-line).

**This note does NOT copy or modify any sections of CONSENT-LANGUAGE.md.** The draft copy is authored by S5 and is subject to legal review.

### What the D1 operator needs to see (from the copy spec)

The D1 consent-flow demonstration requires the operator to point to language that:
1. Explains what "deny-by-default" means in plain terms (not an error, a policy decision).
2. Shows which agent made the request and what capability was denied.
3. Does not use internal type names like `NoGrantsPresented` in operator-visible UI text — the shell renders a human-readable label.

**Current status:** The shell renders denial reason labels via the deny-log slot. The exact copy is the CONSENT-LANGUAGE.md draft (not yet finalized). For D1 demo purposes, the operator can point to the deny-log slot and read the label; the label need not match the final production copy.

---

## Gap Analysis: Current Shell vs D1 Demo Requirements

| D1 requirement | Current shell state | Gap |
|---|---|---|
| Deny log visible | ✅ denyLog.list slot renders | None |
| At least one denial present | ⬜ Must be pre-seeded | Operator must pre-seed denial before demo session |
| Grant state visible | ✅ agent.snapshot renders grant list | None |
| Consent copy visible | 🟡 Draft copy in CONSENT-LANGUAGE.md | Copy not yet finalized; labels are functional but may change post-counsel |
| Operator can point to enforcement slot | ✅ Deny-log slot in shell | None |
| Browser can issue grant/revoke | ❌ Not in shell | Not a D1 requirement; G4 requirement |

### Summary

The current shell satisfies the D1 consent-flow visibility requirement, provided the operator pre-seeds at least one denial. The only known gap is that the consent copy is not yet finalized; for D1, functional labels are acceptable. No new shell features are needed for D1.

---

## Phase B Onramp Surface (ADR-023)

Under ADR-023 (Onramp Policy), entities discovered through the HA bridge arrive as proposed FERROS material. The S5 Phase B shell does not yet have an onramp surface (accept/reject affordances). This is a Phase B follow-up item, not a D1 blocker.

**What will be needed for G4 (not D1):**
- An explicit operator accept action through the S5 onramp surface.
- The accept event auditable through the S3/S4 audit-log surface.

---

## Source Documents

- `site/agent-center-shell.html` — localhost shell asset
- `streams/S5-ux/README.md` — Phase B definition (authoritative for current scope)
- `docs/legal/CONSENT-LANGUAGE.md` — draft consent copy (do not copy sections; pending counsel)
- `crates/ferros-core/src/capability.rs` — `DenyByDefaultPolicy`, `PolicyDenialReason` types
- `docs/research/S4-policy-engine-invariants.md` — policy invariant catalog
