# ADR-002: Smart Contract Boundaries — Where Contracts Fit in FERROS

## Status
Accepted

## Context
The FERROS architecture includes smart contract integration for governance and verification. Early design discussions revealed a common instinct to use smart contracts for data storage (e.g., storing user profiles as NFTs with encrypted JSON metadata). Analysis showed this creates unnecessary chain dependency, wallet friction, privacy exposure, and cost overhead without meaningful benefit over local storage.

We need a clear boundary defining what smart contracts ARE used for and what they ARE NOT used for.

## Decision

### Smart contracts ARE used for (4 approved use cases):

1. **Permission-gating policies**: Critical permission policies (e.g., "this device will never share health data without biometric confirmation") can be committed to a contract, making them publicly auditable, tamper-resistant, and enforceable even if local software is compromised. Modifications require a cooling-off period.

2. **Profile provenance anchoring**: When a user earns a credential or degree, the root hash of their progression chain is anchored to a ledger as a one-time write. This provides an immutable record of origin without exposing the underlying data. The anchor includes: root hash, credential type, timestamp, issuing system identifier.

3. **Cross-network credential verification**: External systems can verify a user's credential by checking the ledger anchor against a locally-provided proof. This enables trust between FERROS instances without requiring a central authority.

4. **Bounty settlement**: When real value (tokens, credits, currency) changes hands for completed work, the transaction is recorded on-chain for accountability and dispute resolution.

### Smart contracts ARE NOT used for:

1. **Data storage**: User profiles, progression data, task completions, and activity logs are stored locally. Chain storage is too expensive, too slow, too public, and creates unnecessary dependency.

2. **Daily authentication**: Users authenticate locally using keypairs stored in hardware-backed secure storage. The chain is not involved in routine auth flows.

3. **Task tracking**: Task completion, XP gains, and streak calculations run entirely locally. The seal chain provides integrity without chain dependency.

4. **Real-time operations**: Anything that needs to happen in under 1 second should not touch a smart contract. Block times and gas costs make this impractical.

5. **Configuration management**: System settings, UI preferences, assistance levels, and automation rules are local data, not chain data.

### The coupling constraint

Once permissions are controlled by a smart contract, the entire system must be accessible to that contract. This means:
- The contract interface must be minimal and stable (hard to upgrade)
- The local system must function fully even when the chain is unavailable
- Contract-controlled permissions are a SUBSET of all permissions (most permissions remain local-only)
- Users can opt out of contract-based permissions entirely with no loss of core functionality

## Consequences

### Positive
- Clear architectural boundary prevents "blockchain creep"
- System works 100% offline for all core functionality
- No wallet requirement for basic users
- Privacy preserved — only hashes and policy commitments go on-chain
- Reduced gas costs — only 4 transaction types, most are rare

### Negative
- Credential verification requires the verifier to understand the local proof format
- Permission-gating contracts add complexity to the governance layer
- Users may expect "everything on-chain" and need education about the local-first model

### Risks
- Scope creep: future features may attempt to add chain dependency. This ADR should be referenced as the governing boundary.
- Chain unavailability: the system must degrade gracefully. All chain-dependent features must have local fallbacks.

## Related
- [ADR-001: Progression-Lock Pattern](./ADR-001-progression-lock-pattern.md)
- FERROS Blueprint: Section 07 — Ledger & Smart Contract Coordination
- FERROS Blueprint: Section 05 — Security Model
