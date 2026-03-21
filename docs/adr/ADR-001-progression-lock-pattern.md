# ADR-001: Progression-Lock Pattern for Local-Sovereign User Onboarding

## Status
Accepted

## Context
FERROS needs an onboarding and progression system that:
- Works entirely locally with no server or chain dependency for daily operation
- Is tamper-evident without requiring blockchain for every interaction
- Supports ADA-conscious assistance levels that genuinely change the interface
- Produces verifiable credentials that can optionally be anchored to a distributed ledger
- Makes the setup process itself the first "quest" — no separate onboarding flow

## Decision
We adopt the **progression-lock pattern**: a hash-chain-based progression system where:

1. The HTML document starts as a blank slate with only the first setup step visible
2. Completing a task generates a **sealed data package** — a SHA-256 hash incorporating the task data, the previous seal, a timestamp, and a nonce
3. Each seal becomes the **unlock key** for the next progression stage
4. The ordered chain of seals forms a tamper-evident history of the user's progression
5. The entire chain is stored locally (localStorage or local file)
6. The final seal (root hash) can optionally be **anchored to a distributed ledger** as a one-time provenance record
7. Verification of the chain requires no external service — it can be validated locally by re-computing each hash

### Seal Chain Structure

```
seal_0 = hash(genesis_data + 'genesis' + timestamp + nonce)
seal_1 = hash(task_1_data + seal_0 + timestamp + nonce)
seal_2 = hash(task_2_data + seal_1 + timestamp + nonce)
...
seal_N = hash(task_N_data + seal_{N-1} + timestamp + nonce)
```

### Implementation

The Web Crypto API (`crypto.subtle.digest('SHA-256', ...)`) is used for hashing. This runs in any modern browser with no external dependencies:

```javascript
async function sealTask(taskId, taskData, previousSeal) {
  const payload = JSON.stringify({
    taskId,
    data: taskData,
    previousSeal: previousSeal || 'genesis',
    timestamp: new Date().toISOString(),
    nonce: crypto.getRandomValues(new Uint32Array(1))[0]
  });
  const encoded = new TextEncoder().encode(payload);
  const hashBuffer = await crypto.subtle.digest('SHA-256', encoded);
  const hashArray = Array.from(new Uint8Array(hashBuffer));
  return hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
}
```

### Known Implementation Constraint: file:// Protocol

`crypto.subtle` requires a secure context (`https://` or `localhost`). When
`docs/personal-profile.html` is opened directly from disk via `file://`, the
Web Crypto API is unavailable in Chrome, Edge, and Firefox.

**The deployed implementation uses a two-path approach:**

1. Attempt `crypto.subtle.digest('SHA-256', ...)` — succeeds in browser/server context
2. Fall back to djb2 hash — used on `file://` protocol

```javascript
async function hashData(str) {
  if (window.crypto && window.crypto.subtle) {
    try {
      const encoded = new TextEncoder().encode(str);
      const buf = await window.crypto.subtle.digest('SHA-256', encoded);
      return Array.from(new Uint8Array(buf))
        .map(b => b.toString(16).padStart(2,'0')).join('');
    } catch(e) { /* fall through */ }
  }
  // djb2 fallback
  let h = 5381;
  for (let i = 0; i < str.length; i++) {
    h = ((h << 5) + h) + str.charCodeAt(i);
    h = h & h;
  }
  return Math.abs(h).toString(16).padStart(8, '0');
}
```

**Agents: DO NOT replace this with unconditional `crypto.subtle`. DO NOT remove the
djb2 fallback. This is a permanent constraint of the `file://` deployment model.**

The djb2 hash is not cryptographically strong (it is not SHA-256). It is sufficient for
local tamper-evidence and progression locking. It is **NOT** sufficient for on-chain
anchoring — when anchoring to a ledger, use the SHA-256 path (which requires a server
context, not `file://`).

### Integrity Properties
- **Forward integrity**: Each seal depends on all previous seals (any tampering breaks the chain)
- **Local verifiability**: Chain can be verified by re-computing hashes from genesis
- **Optional anchoring**: The root hash can be published to a ledger for external verification
- **Privacy preserving**: Only the hash is anchored, not the underlying data

### Progression Stages

| Stage | Name | Trigger | XP Awarded |
|-------|------|---------|-----------|
| 0 | Genesis | First load | — |
| 1 | Character Creation | Complete Stage 0 | 50 XP (stream affinity) |
| 2 | First Protocol | Complete Stage 1 | 100 XP (Discipline) |
| 3 | Main Dashboard | Complete Stage 2 | Full profile unlocked |
| 4+ | Progressive Unlocks | XP thresholds / achievements | Ongoing |

## Consequences

### Positive
- Zero chain dependency for daily operation
- No wallet friction during onboarding
- Tamper-evident without blockchain overhead
- Natural gamification through progressive reveals
- ADA-conscious: onboarding IS the first quest, complexity increases gradually
- Verifiable credentials without centralized issuer

### Negative
- localStorage is not durable across browser clears (mitigated by export/import)
- Hash chain doesn't prevent someone from generating a fake chain from scratch (mitigated by optional ledger anchoring)
- Nonce adds complexity (necessary for preventing replay/precomputation)

### Risks
- Browser crypto API availability (mitigated: supported in all modern browsers, graceful fallback)
- Profile portability between devices (mitigated: JSON export/import with chain verification)

## Related
- [ADR-002: Smart Contract Boundaries](./ADR-002-smart-contract-boundaries.md)
- FERROS Blueprint: Consent Engine design
- FERROS Blueprint: Identity & Permission Layer
