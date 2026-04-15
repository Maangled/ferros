# Contract: Permission Model

**ID:** C10
**Version:** 1.0
**Status:** Active
**Last updated:** 2026-04-13
**Depends on:** ADR-002 (Smart Contract Boundaries), ADR-005 (Session Modes), ADR-001 (Seal Chain)
**Enforced by:** H4 (negative-harness.html)

---

## Purpose

This contract defines who can do what in the FERROS system, how consent is captured, what happens when permission is denied, and what audit trail is emitted. It is **enforced behavior**, not documentation. Wave 0 scope covers local-first permissions only — on-chain and DAO governance are deferred (ADR-002).

---

## Core Model

Permission is modeled as a 4-tuple:

```
(subject, actor, action, resource) → decision (allow | deny) + audit emission
```

- **Subject:** The identity asserting the permission. One of: `full-profile`, `alias`, `recovery`, `session`, `agent`.
- **Actor:** The code path making the request. One of: `user`, `system`, `agent`.
- **Action:** What is being attempted. See action registry below.
- **Resource:** What the action targets. One of: `localStorage`, `sessionStorage`, `profile`, `sealChain`, `aliasLog`, `recoveryLog`, `exportFile`, `card`, `deck`.

---

## Action Registry (Wave 0)

| Action | Description |
|---|---|
| `profile.write` | Write profile data to `localStorage` |
| `profile.read` | Read profile data from `localStorage` |
| `profile.export` | Serialize profile to downloadable `.json` |
| `profile.import` | Deserialize and load a `.json` file into profile state |
| `sealChain.append` | Add a seal to the seal chain via `addSeal()` |
| `sealChain.verify` | Read and verify the seal chain (non-mutating) |
| `aliasLog.write` | Write an entry to the alias session log |
| `aliasLog.export` | Download the `.ferros-log` file |
| `recoveryLog.write` | Write an entry to the recovery session log |
| `recoveryLog.export` | Download the `.ferros-log` file |
| `card.save` | Persist card state locally |
| `card.export` | Export a card to a file |
| `storage.clear` | Clear `localStorage` keys |

---

## Permission Matrix

| Action | full-profile | alias | recovery | session | agent |
|---|---|---|---|---|---|
| `profile.write` | ✅ Allow | ❌ Deny + audit | ❌ Deny + audit | ❌ Deny + audit | 🔒 Gated (Wave 3) |
| `profile.read` | ✅ Allow | ❌ Deny | ❌ Deny | ❌ Deny | 🔒 Gated (Wave 3) |
| `profile.export` | ✅ Allow | ❌ Deny | ❌ Deny | ❌ Deny | 🔒 Gated (Wave 3) |
| `profile.import` | ✅ Allow | ❌ Deny | ✅ Allow (read-only, no persist) | ❌ Deny | 🔒 Gated (Wave 3) |
| `sealChain.append` | ✅ Allow | ❌ Deny + audit | ❌ Deny + audit | ❌ Deny + audit | 🔒 Gated (Wave 3) |
| `sealChain.verify` | ✅ Allow | ✅ Allow (own log only) | ✅ Allow (own log only) | ❌ Deny | 🔒 Gated (Wave 3) |
| `aliasLog.write` | ❌ Deny | ✅ Allow | ❌ Deny | ❌ Deny | 🔒 Gated (Wave 3) |
| `aliasLog.export` | ❌ Deny | ✅ Allow | ❌ Deny | ❌ Deny | 🔒 Gated (Wave 3) |
| `recoveryLog.write` | ❌ Deny | ❌ Deny | ✅ Allow (JS vars only, no storage) | ❌ Deny | 🔒 Gated (Wave 3) |
| `recoveryLog.export` | ❌ Deny | ❌ Deny | ✅ Allow | ❌ Deny | 🔒 Gated (Wave 3) |
| `card.save` | ✅ Allow | ❌ Deny | ❌ Deny | ❌ Deny | 🔒 Gated (Wave 3) |
| `card.export` | ✅ Allow | ❌ Deny | ❌ Deny | ❌ Deny | 🔒 Gated (Wave 3) |
| `storage.clear` | ✅ Allow (self only) | ❌ Deny | ❌ Deny | ❌ Deny | ❌ Deny |

**Note:** "Gated (Wave 3)" means the action is undefined in Wave 0. Any agent attempting this action in Wave 0 must receive `PERMISSION_NOT_YET_DEFINED` and the attempt must be logged.

---

## Deny Behavior

When a permission is denied:
1. The action **must not proceed** — not even partially.
2. A `PERMISSION_DENIED` audit entry must be emitted to the active log (see Audit Emission below).
3. A human-readable error must be surfaced to the UI (not silently swallowed).
4. No data must be written to any storage as a side effect of the denied action.

---

## Consent Capture

Consent for `localStorage` writes is gated by the **Trade Window** (Stage 0). The consent model is:

| Event | Consent State | Effect |
|---|---|---|
| User opens app (no prior profile) | Unconsented | Trade Window shown; no storage writes allowed |
| User accepts Trade Window | Consented | Full Profile mode activated; `localStorage` writes permitted |
| User declines Trade Window | Declined | Session Mode activated; `sessionStorage.setItem('session_declined', 'true')` is the only permitted write |
| User closes tab | Consent not persisted | Session Mode state is gone; next visit starts fresh |

**Immutable consent rule:** Once the Trade Window has been accepted and a genesis seal created, the consent decision is embedded in the seal chain. It cannot be revoked without clearing all storage. This is by design.

---

## Audit Emission

Audit events are emitted to the **in-memory audit trail** in Wave 0. Persistence to disk (`.ferros-log`) is only for alias and recovery session exports.

### Audit Entry Shape (Wave 0)

```json
{
  "ts": "<ISO timestamp>",
  "subject": "<full-profile | alias | recovery | session | agent>",
  "action": "<action string>",
  "resource": "<resource string>",
  "decision": "<allow | deny>",
  "reason": "<optional human-readable string>"
}
```

### Events That Must Emit Audit Entries

| Event | Decision | When |
|---|---|---|
| alias session attempts `profile.write` | deny | Alias code calls `saveProfile()` |
| recovery mode attempts `sealChain.append` | deny | Recovery mode calls `addSeal()` |
| session mode attempts `profile.write` | deny | Session-declined user triggers `saveProfile()` |
| profile imported successfully | allow | Import completes without error |
| profile import rejected (any I-x rule) | deny | Import fails validation |
| seal added to chain | allow | `addSeal()` completes |

---

## Storage Policy Cross-Reference (from storage-rules.md)

This contract's reject decisions map directly to C9 storage rules:

- `profile.write` / `sealChain.append` mutation paths are gated by the unified durable-write predicate (`canMutateDurableState`) rather than per-call ad hoc checks.
- The predicate requires Trade Window consent accepted and denies durable writes in `sessionMode`, `aliasMode`, and `recoveryMode`.
- Import rejection codes `STORAGE_*` from C9 are treated as upstream validation errors, not permission decisions — they happen before mutation is evaluated.

---

## Wave 0 Enforcement Points

| Rule | Enforced By |
|---|---|
| Alias mode denied `profile.write` emits audit entry | H4 negative-harness.html |
| Recovery mode denied `sealChain.append` emits audit entry | H4 negative-harness.html |
| Session mode denied `profile.write` — nothing in localStorage | H4 negative-harness.html |
| Trade Window consent gate prevents all pre-consent writes | H5 acceptance-harness.html |

---

## Deferred (Wave 3+)

- Agent action permission gates (all `🔒 Gated` actions above)
- DAO-governed permission policies (ADR-002)
- Cooling-off periods for permission changes
- Multi-subject consent (joint actions)
- On-chain audit anchoring

---

## Audit Record Retention

In-memory audit entries are bounded by a **ring buffer** capped at **1000 entries**. When the buffer is full, the oldest entry is evicted (FIFO). This prevents unbounded memory growth in long-running sessions.

- The 1000-entry cap is the Phase 0 ceiling. Post-Phase-0 may persist audit records to disk or chain.
- Eviction is silent — no notification to the user.
- Exported `.ferros-log` files contain only the entries accumulated during that specific session, not the ring buffer history.

---

## Claim Uniqueness

When a user claims an alias or recovery log via the Portability panel:

- The `sessionId` from the `.ferros-log` envelope is checked against `profile.meta.claimedAliasSessions`.
- If the `sessionId` is already present → reject with `CLAIM_DUPLICATE_SESSION`. The entries are not merged.
- If the `sessionId` is new → append to `claimedAliasSessions` and merge entries into the profile journal.
- This ensures idempotent claims: importing the same log file twice has no effect.
