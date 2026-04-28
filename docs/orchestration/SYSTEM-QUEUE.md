# FERROS System Queue

This queue feeds the local driver for **system-track** work: legal/T&C, ledger/chain ADR, asset library, character/profile templates, card/deck game/trade UX, smart-contract drafts, and onramp ADR work. Consumed only by Batch Mode runs scoped to `track: system`.

## Queue item schema (same shape as WAVE-QUEUE.md)

Required fields: `Title`, `Status`, `Priority`, `Gate`, `Owning streams`, `Goal`, `Anchor files`, `Validation`, `Constraints`, `Last update`

Optional fields (additive, do not break existing item order):
- `size: S | L` — S means ≤3 anchor files, single stream, docs-only. L means multi-stream or schema-touching. Batch Mode default consumes only S.
- `parallel-safe-with: [WAVE-IDs]` — explicit non-overlap declarations.
- `serial-after: WAVE-ID` — must wait for a prior wave.
- `solo: true | false` — must run alone (truth-sync, gate close, schema freeze, shared truth surfaces).
- `track: code | system | hardware` — which queue this belongs to.

---

## Ready

None. System queue empty.

---

## In Progress

None.

---

## In Progress

None.

---

## Blocked

None.

---

## Done

### SYSTEM-2026-04-27-02

- Title: Draft ADR — Ledger/chain substrate comparison and recommendation
- Status: done
- Priority: P1
- Gate: pre-G4 policy runway
- Owning streams: S8 primary; S6 consumer awareness
- Goal: Author a comparison ADR evaluating Solana, EVM L2, Cosmos app-chain, and non-chain signed-ledger options against FERROS invariants (locally sovereign, consent-first, deliverable on flashdrive, signed grants exist). The ADR produces a recommendation but not a commitment. It cross-references existing prior-art mentions in the repo ADR set.
- Anchor files: `docs/adr/ADR-024-ledger-substrate.md`
- Validation: `get_errors` clean on `docs/adr/ADR-024-ledger-substrate.md` and `docs/adr/_INDEX.md`.
- Constraints: Docs-only. Recommendation only, not a binding commitment. Do not reopen G1–G3. Do not mutate frozen schemas.
- Last update: 2026-04-27
- size: S
- track: system

### SYSTEM-2026-04-27-03

- Title: Legal/T&C scaffold — Terms, licensing posture, and consent language
- Status: done
- Priority: P2
- Gate: pre-G4 policy runway
- Owning streams: S8 primary
- Goal: Scaffold `docs/legal/` with three placeholder files: `TERMS-OF-USE.md`, `LICENSING-POSTURE.md`, and `CONSENT-LANGUAGE.md`. Each file states the FERROS posture in plain English and explicitly marks itself as draft awaiting counsel red-line. No legal advice is given or implied; these are structured placeholders so counsel has a clear surface to red-line rather than starting from scratch.
- Anchor files: `docs/legal/TERMS-OF-USE.md`, `docs/legal/LICENSING-POSTURE.md`, `docs/legal/CONSENT-LANGUAGE.md`
- Validation: `get_errors` clean on the three new files.
- Constraints: Docs-only. Mark every file as draft. Do not claim these constitute legal advice or final terms.
- Last update: 2026-04-27
- size: S
- track: system

### SYSTEM-2026-04-27-01

- Title: Draft ADR — External systems are onramps, not identity truth
- Status: done
- Priority: P1
- Gate: pre-G4 policy runway
- Owning streams: S8 primary; S7 consumer awareness; S2 consumer awareness
- Goal: Author the onramp policy ADR covering Home Assistant, calendar import, social-graph import (LinkedIn/Facebook style), and bundle/migration pipelines. The ADR frames all imported data as proposed FERROS material requiring explicit consent before becoming canonical state. It does not constrain the HA bridge implementation details (those remain S7-owned) but does establish the data-direction invariant so future S7 implementation work can reference a decided policy rather than re-litigating it mid-flight. Cross-check against `docs/adr/ADR-021-dependency-admission-policy.md` for consistency.
- Anchor files: `docs/adr/ADR-023-onramp-policy.md`
- Validation: `get_errors` clean on `docs/adr/ADR-023-onramp-policy.md` and `docs/adr/_INDEX.md`.
- Constraints: Docs-only. Do not claim HA bridge implementation details, pairing handshake order, or consent UI internals. Do not reopen G1–G3. Do not mutate frozen schemas.
- Last update: 2026-04-27
- size: S
- track: system

None yet. System queue established in WAVE-2026-04-27-03.
