# ADR-005: Cross-Device Identity & Session Mode Architecture

## Status
Accepted

## Date
2026-03-20

## Context
FERROS is local-first. Profiles live in `localStorage` on a single device. But people move — they're at a friend's house, a coffee shop, a work machine, a shared hub. The question is: **how does your identity travel with you without requiring a server, an account system, or compromising the host machine?**

This ADR captures the four session modes implemented across PRs #7, #11, #12, #13, #16, and #17 as a unified architectural decision. The implementation was arrived at iteratively, but the underlying model is coherent and worth recording formally.

The core insight is: **a FERROS profile IS a cryptographic identity.** The genesis hash seeds the seal chain. All activity is signed under that hash. The profile doesn't need to be "logged into" — it needs to be *present* (in localStorage) or *borrowed* (in memory for the session). This means identity can be:

1. **Owned and permanent** — full profile in localStorage (home device)
2. **Declined entirely** — session mode, no identity, no persistence
3. **Borrowed from a public pool** — alias mode, bearer identity
4. **Recovered from backup** — recovery mode, real identity, session-scoped

The trigger for writing this ADR is the completion of recovery mode in PR #17, which closes the identity loop: you can now leave home with a USB key, be yourself on any machine, and come back with a signed log.

## Decision

### The Four Session Modes

#### Mode 0: Full Profile (Home Mode)
- **What:** Profile in `localStorage`. The normal state.
- **When:** User has completed setup on this device, or imported their profile permanently via the Portability panel.
- **Stored:** `ferros_profile` + `ferros_seal_chain` in `localStorage`. Persists across sessions.
- **Identity:** Real. Signed under the user's genesis hash.
- **Ends:** Never (until user resets or clears data).
- **Take with them:** Everything — it's already there.
- **Body class:** `body.level-N` (whichever assist level they chose)

#### Mode 1: Session Mode (No Consent)
- **What:** User declined the Trade Window. No localStorage writes. No identity.
- **When:** User clicks "Decline" on the MMO Trade Window permission dialog (PR #7).
- **Stored:** `ferros_session_declined = true` in `sessionStorage` only. Nothing in `localStorage`.
- **Identity:** None. Anonymous browser session.
- **Ends:** Tab close / page refresh. Session mode flag is gone.
- **Take with them:** Nothing.
- **Body class:** None specific; persistent amber `#session-banner` shown.
- **Key constraint:** `saveProfile()` is a no-op in session mode.

#### Mode 2: Alias Mode (Borrowed Identity)
- **What:** User borrows a public profile from the gallery to log activity under a pseudonymous identity.
- **When:** User picks a template profile (e.g. "Nikola Tesla", "Philip J. Fry") from the Profile Gallery and clicks "Use as Alias" (PR #13, #14).
- **Stored:** `ferros_alias_session` in `sessionStorage` only. Contains: `aliasCode`, `aliasName`, `startedAt`, `logs[]`.
- **Identity:** Pseudonymous. Signed under the alias's public genesis key stub. `attribution: "unlinked"` until claimed.
- **Ends:** User clicks "Exit Alias Mode" or closes the tab.
- **Take with them:** A `.ferros-log` file (`logType: "alias-session"`) downloaded on exit. Contains all logs signed under the alias code.
- **Claim flow:** On home device, import the `.ferros-log` via Dashboard → Portability → Claim Alias Log. Entries are verified and merged into the real profile with `linkedTo` updated to the user's genesis hash (PR #16).
- **Body class:** `body.alias-mode`

#### Mode 3: Recovery Mode (Real Identity, Foreign Device)
- **What:** User loads their exported profile backup on a foreign device. Real identity, constrained to the session.
- **When:** User clicks "Recover Profile" on Genesis, selects their `.json` backup file, and confirms (PR #17).
- **Stored:** All state in module-level JS variables only. `localStorage` is never written during recovery mode. `saveProfile()` is guarded to return immediately if `recoveryMode === true`.
- **Identity:** Real. Signed under the user's own genesis hash (`attribution: "self"`). Not pseudonymous.
- **Ends:** User clicks "Exit" from the recovery banner. Prompted to download log if entries exist.
- **Take with them:** A `.ferros-log` file (`logType: "recovery-session"`) downloaded on exit. Signed under the user's own genesis hash.
- **Claim flow:** Same import path as alias logs, but `processAliasLogClaim()` recognizes `logType: "recovery-session"` and uses different claim text + achievement (`🔑 Recovery Claimed`).
- **Body class:** `body.recovery-mode`

### Session Mode Comparison Table

| Mode | Identity | localStorage writes | sessionStorage | Exit artifact | Claimable | Body class |
|---|---|---|---|---|---|---|
| Full Profile | Real | ✅ Yes | — | — | N/A | `body.level-N` |
| Session Mode | None | ❌ Never | `session_declined` | None | No | — |
| Alias Mode | Pseudonymous | ❌ Never | `alias_session` | `.ferros-log` (alias) | ✅ Yes (PR #16) | `body.alias-mode` |
| Recovery Mode | Real (own) | ❌ Never | — (JS vars only) | `.ferros-log` (recovery) | ✅ Yes (PR #17) | `body.recovery-mode` |

### The Portable Log Artifact (`.ferros-log`)

A `.ferros-log` file is a JSON artifact that carries activity logs across devices. It is the bridge between all session modes and the home profile.

**Schema** (values shown as `"one" | "other"` indicate allowed alternatives, not literal JSON):
```json
{
  "ferrosVersion": "1.0",
  "logType": "alias-session | recovery-session",
  "alias": {
    "id": "string",
    "name": "string",
    "aliasCode": "string",
    "attribution": "unlinked"
  },
  "profile": {
    "id": "string",
    "genesisHash": "string",
    "attribution": "self"
  },
  "sessionStart": "ISO timestamp",
  "sessionEnd": "ISO timestamp",
  "entries": [
    {
      "ts": "ISO timestamp",
      "text": "string",
      "type": "activity | journal",
      "seal": "djb2 hash of content+timestamp"
    }
  ],
  "claimInstructions": "string"
}
```

The `alias` field is present for `alias-session` logs; the `profile` field is present for `recovery-session` logs.

**Filename conventions:**
- Alias logs: `ferros-alias-log-[aliasCode]-[YYYY-MM-DD].json`
- Recovery logs: `ferros-recovery-[sanitized-name]-[YYYY-MM-DD].json`

**Integrity:** Each entry is sealed with a djb2 hash of its content + timestamp. The `verifyChain()` function checks seal integrity on import. Broken chains are flagged with `meta.sealBroken = true` (alias claim) or shown as an inline warning (recovery load), but import is never blocked outright — privacy-first, the user decides.

### Mutual Exclusivity Rules

The four modes are mutually exclusive at the implementation level:

- Entering **alias mode** clears any active recovery mode (`_doExitRecovery()`) and ignores session mode.
- Entering **recovery mode** clears any active alias mode (`_doExitAlias()`) and ignores session mode.
- Entering **session mode** (Trade Window decline) prevents localStorage writes globally but does not block alias or recovery mode entry — those operate in their own memory anyway.
- **Full profile** mode is the default when `ferros_profile` exists in localStorage. If a full profile is present, the "Recover Profile" entry point on Genesis redirects to the Portability panel instead of opening the recovery panel.

### Accountability Model

The alias and recovery systems form a pseudonymous accountability layer:

**Alias logs are bearer instruments.** Like cash — anyone can use an alias code. The log is valid and permanently attributable to that alias code on-ledger. It is `unlinked` until the true user claims it. If they never claim it, it remains attributed to the alias forever.

**Privacy is the default, not erasure.** An unclaimed alias log is not deleted. It exists in the chain attributed to the alias code. The user who created it has a receipt (the `.ferros-log` file). The connection between "person who used bernie-4f2a" and their real genesis hash exists only in the `.ferros-log` file — which only they hold.

**Fraudulent use creates an immutable receipt.** If someone uses an alias to commit fraud and later claims those logs to their real profile, the claim itself establishes the link. The connection is then permanent and reviewable. If they never claim, the fraud is attributed to the alias but the personal connection is unprovable without the `.ferros-log` file.

**Recovery logs are self-attributed.** `attribution: "self"` — there is no pseudonymity. The log is signed under the user's own genesis hash. Claiming it simply merges entries into the home profile with a clear provenance trail.

**Future governance layer (out of scope for this ADR):** A federated AI governance network (democratic, open-source, peer-elected) could subpoena alias-to-real-profile claim connections if evidence of predatory fraud crosses a threshold. This is a future layer. Until then, privacy is preserved by cryptographic default, not policy.

## Consequences

### Decisions Made

**Decision: sessionStorage for alias/session mode flags, JS module vars for recovery mode.**
- Alias sessions use `sessionStorage` because the alias identity should not outlive the browser tab.
- Recovery mode uses JS module variables (not even `sessionStorage`) because nothing — not even a flag — should be written to the foreign device's storage.
- Rejected: Writing recovery flags to `sessionStorage`. Even a sessionStorage write feels wrong on a machine that isn't yours. The amber banner and `body.recovery-mode` class are the only indicators.

**Decision: `saveProfile()` guard as the security boundary.**
- A single `if (recoveryMode || sessionMode) return;` at the top of `saveProfile()` is the enforcement point.
- All other code can call `saveProfile()` freely — the guard silently no-ops it.
- Rejected: Patching every call site individually. Too fragile, too easy to miss.

**Decision: Identical claim flow for alias and recovery logs.**
- `processAliasLogClaim()` handles both `logType: "alias-session"` and `logType: "recovery-session"`.
- The difference is cosmetic: different confirmation text, different achievement ID, `profile` field vs `alias` field.
- Rejected: Two separate import functions. Code duplication, inconsistent validation.

**Decision: Broken seal chains are flagged, not blocked.**
- A tampered or incomplete seal chain shows a warning but doesn't prevent import.
- Rationale: The user may have partially-completed logs, or the djb2 hash function may have minor implementation drift between versions. Blocking import would lose data.
- The `meta.sealBroken = true` badge on the character sheet makes the integrity state visible permanently.

### Positive
- Cross-device identity without a server, an account system, or any cloud dependency.
- No real profile data is ever written to a foreign device in any session mode.
- A single `saveProfile()` guard enforces the storage boundary — no per-call patching required.
- Alias and recovery logs are portable, claimable, and cryptographically receipted.
- Privacy-by-default: unclaimed alias logs are pseudonymous; the user controls whether to link them.

### Negative
- Recovery mode state lives only in JS module variables — a page refresh or crash loses unsaved entries (mitigated by a download prompt on exit).
- Alias impersonation is possible by design (same as a pen name). The alias code system makes this explicit.
- sessionStorage is tab-scoped — closing the tab without downloading the alias log loses the session.

### Out of Scope (Future ADRs)
- **Ledger anchoring of alias codes** — when alias codes go on-chain as provenance anchors (ADR-002 use case 2).
- **Key pair generation** — personal alias recovery keys. A future ADR will define the key derivation scheme for user-generated alias keys that aren't template profiles.
- **Seeding bounties** — the distributed storage economy where profile fragments are seeded by nodes and bounty-rewarded (ADR-002 use case 4).
- **Federated governance** — the democratic oversight layer for fraud subpoenas.
- **NFT-adjacent profile ownership** — fractional profile ownership, seeding dividends, provably scarce identity artifacts. Out of scope until Layer 4 infrastructure exists.

## Related
- [ADR-001: Progression-Lock Pattern](./ADR-001-progression-lock-pattern.md) — The genesis hash from ADR-001 is the cryptographic identity that alias and recovery modes borrow or carry. The seal chain format is shared.
- [ADR-002: Smart Contract Boundaries](./ADR-002-smart-contract-boundaries.md) — The claim flow (when a `.ferros-log` is merged into a real profile) is a candidate for provenance anchoring on-chain — one of ADR-002's 4 approved use cases. Not implemented yet; the current implementation is local-only.
- [ADR-003: Alias System](./ADR-003-alias-system.md) — This ADR supersedes ADR-003's session mode documentation with a unified four-mode model. ADR-003 remains the canonical reference for alias identity specifically.
- [ADR-004: Template Profile Specification](./ADR-004-template-profile-specification.md) — The `TEMPLATE_PROFILES` constant provides the alias pool. The `aliasCode` field in each template stub is the session identifier used in alias mode.
- PR #7: MMO Trade Window (session mode)
- PR #11: Resume Banner + Session Mode
- PR #13: Alias Mode
- PR #16: Alias Log Claim Flow
- PR #17: Key Recovery / Cross-Device Login
