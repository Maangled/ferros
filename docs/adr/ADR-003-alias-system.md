# ADR-003: The Alias System

## Status
Accepted

## Context
FERROS profiles are cryptographic identities — the genesis hash seeds everything downstream. A user at a friend's house, a coffee shop, or any foreign device shouldn't have to create a full profile just to log an activity. They need a lightweight "alias session" — borrowing a public identity key to sign logs — without compromising the host profile or their own.

Aliases are **bearer instruments**: like cash, anyone can use them. The logs signed under an alias are valid and permanently recorded in the chain, but they remain `unlinked` until the true owner claims them.

## Decision

### 1. Alias Identity
- An alias is a **public genesis key stub** — a small JSON object (see ADR-004 for the full template stub schema).
- Aliases are identified by a short **alias code** (e.g. `nikola-50a9`) — a slug derived from the template name + first 4 chars of the template's genesis hash stub.
- Well-known aliases (famous/template profiles from the gallery) are **hardcoded into the HTML** as JSON stubs. User-created aliases eventually live on the ledger (future: ADR-002 provenance anchoring).

### 2. Alias Session
- An alias session is a **lightweight, non-destructive mode** distinct from both full profile login and session mode (trade dialog decline).
- On session start, a `ferros_alias_session` key is written to `sessionStorage` (NOT localStorage) containing:
  ```json
  {
    "aliasCode": "nikola-50a9",
    "aliasName": "Nikola Tesla",
    "startedAt": "ISO timestamp",
    "logs": []
  }
  ```
- No `ferros_profile` localStorage writes occur during an alias session.
- The alias session persists for the browser tab lifetime only (sessionStorage).

### 3. Log Artifacts
- Every activity logged during an alias session is appended to `ferros_alias_session.logs` as a **portable log entry**:
  ```json
  {
    "logId": "uuid-or-hash",
    "aliasCode": "nikola-50a9",
    "action": "habit_logged",
    "data": { "habit": "Morning coffee", "duration": 15 },
    "signedUnder": "alias:nikola-50a9",
    "linkedTo": null,
    "timestamp": "ISO timestamp",
    "seal": "local-hash-or-sha256"
  }
  ```
- `linkedTo: null` means **unlinked** — the log is attributed to the alias but not tied to any real profile.
- At session end, the user is prompted to **download their alias log** as `ferros-alias-log-[aliasCode]-[date].json` (a portable `.ferros-log` artifact — the `.json` extension ensures the file opens in any text editor without extra tooling).

### 4. Claim Flow (future: PR 9)
- On their home device, a user imports a `.ferros-log` file.
- The system verifies the seal chain of the log entries.
- If valid, each log entry's `linkedTo` field is updated to the user's own genesis hash.
- The logs are merged into the real profile's seal chain and journal.
- A "Claimed from alias: nikola-50a9" journal entry is created.
- If the alias log's seal chain is broken or tampered, a ⚠️ "Seal broken — log imported with integrity warning" badge is shown but import still proceeds.

### 5. Accountability Boundary
- Alias sessions are **pseudonymous, not anonymous** — the seal chain exists and provides a receipt.
- Fraudulent use creates an immutable log under the alias code.
- If a user later claims that alias log to their real profile, the connection is established.
- Unclaimed alias logs remain publicly attributed to the alias (e.g. `nikola-50a9`) forever on the ledger — the unlinked state is the default, not erasure.
- Federated governance (future layer) can subpoena claim connections if fraud is established — but privacy is preserved by default until evidence threshold is crossed.

### 6. UI Entry Points (for PR 6 + PR 7)
- **Profile Gallery** (Genesis page): A modal carousel showing famous/template profiles. Prominent "← Back to Setup" exit. Subtle "Use as Alias" button per profile card.
- **Alias code search**: A text input in the gallery modal where a user can enter a known alias code directly to skip browsing.
- The "Use as Alias" button triggers alias session initialization and skips all setup stages, landing directly on a simplified dashboard labeled `[ALIAS MODE] Nikola Tesla`.

## Consequences

### Positive
- Users can log activities on foreign devices without setup friction.
- Logs are portable, claimable, and cryptographically receipted.
- No real profile data is written to a host device.
- Privacy-by-default: unclaimed logs are pseudonymous.

### Negative
- Alias impersonation is possible (by design — same as a pen name or pseudonym). The alias code system makes this explicit.
- Claim flow (PR 9) is required to get XP/achievements credited to a real profile.
- sessionStorage is tab-scoped — closing the tab without downloading the alias log loses the session.

### Out of Scope
- Real-time alias lookup from ledger (future).
- Alias cost / bounty mechanics (future — referenced in design notes but not implemented here).
- Key-pair based alias signing (future — currently alias "signing" uses the same djb2/SHA-256 fallback hash as the main profile).
- Account recovery keys (future — see design notes).

## Related
- [ADR-001: Progression-Lock Pattern](./ADR-001-progression-lock-pattern.md)
- [ADR-002: Smart Contract Boundaries](./ADR-002-smart-contract-boundaries.md)
- [ADR-004: Template Profile Specification](./ADR-004-template-profile-specification.md)
- FERROS Blueprint: Identity & Permission Layer
- PR 6: Alias Mode
- PR 9: Claim Flow (future)
