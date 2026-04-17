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

---

## Wave 0 Closure Addendum (2026-04-17)

**Added:** PR 6 — Docs/ADR reconciliation

### Baseline established

Wave 0 PRs 1–5 collectively established the hardened contract closure baseline:

- **PR 1 (#41):** Deterministic generators (`generate-harness-constants.ps1`, `generate-ferros-core.ps1`) and named inventory arrays in `_constants.js`. The L1 drift detection requirement from the legacy integration table is satisfied: regenerating constants produces an empty diff.
- **PR 2 (#42):** Manifest hardening and fixture truth — `docs/contracts/manifest.json` maps all C1–C10 contracts to their schemas, fixtures, and harnesses.
- **PR 3 (#43):** H1 (`ferros-contract-validator.html`) expanded to the full C1–C7 contract matrix.
- **PR 4 (#44):** C8–C10 harness gaps closed — H3 nonce handshake, H2 true round-trip, H4 deny-code probes.
- **PR 5 (#45):** Supporting harness alignment and shared-core boundary cleanup.

### Harness numbering

| Harness | File | Role | Gate level |
|---------|------|------|------------|
| H1 | `harnesses/ferros-contract-validator.html` | C1–C7 schema + fixture validation | Gate (Wave 0 exit) |
| H2 | `harnesses/round-trip-harness.html` | C9 storage rules — import/export round-trip | Gate (Wave 0 exit) |
| H3 | `harnesses/runtime-harness.html` | C8 runtime host — nonce handshake lifecycle | Gate (Wave 0 exit) |
| H4 | `harnesses/negative-harness.html` | C10 permission model — deny probes | Gate (Wave 0 exit) |
| H5 | `harnesses/acceptance-harness.html` | Black-box acceptance (Journey 1) | Supporting |
| H6 | `harnesses/write-path-harness.html` | Durable write-path / storage boundary | Supporting |
| H7 | `harnesses/semantic-fixture-linter.html` | Cross-fixture semantic consistency | Supporting |
| H8 | `harnesses/ui-acceptance-harness.html` | UI-facing acceptance (DOM / localStorage only) | Supporting |

**H1–H4 are the Wave 0 gate harnesses** (must be green before Wave 0 is declared closed). H5–H8 are supporting proof harnesses that reinforce the gate harnesses but are not individually required for closure.

### Deterministic freshness

The deterministic generator requirement is enforced by: running `generate-harness-constants.ps1` and `generate-ferros-core.ps1` against the current schemas + templates and verifying the output diff is empty. This is the L1 drift detection check from the legacy integration table in ADR-013.
