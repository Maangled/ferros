# FERROS ADR Roadmap

This roadmap lists likely future decision and research topics without granting them authority before a record is created.

Numbering is assigned only when a real ADR or research note is added. This file is for navigation and anticipation, not for pre-approving outcomes.

---

## Immediate Structural Priorities

- ADR program governance and evidence tiers — landed in [ADR-022-decision-program-governance.md](./ADR-022-decision-program-governance.md)
- Root doctrine and evaluation rubric — landed in [../../DOCTRINE.md](../../DOCTRINE.md)
- ACC object-to-card projection note — active in the research lane
- ADR index, roadmap, and research-note lane — active supporting surfaces

---

## Near-Term Candidate Topics

### Profile, identity, and contract truth

- Signed profile envelope disposition: publish as a schema contract or keep Rust-local
- Profile freeze and CLI closeout policy for the remaining `ferros profile` verbs
- Audit-log ownership, retention, and signing posture

### Agent center and shell

- First S3-owned remote contract for the local agent center shell
- Agent-manifest schema publication, if it graduates from Rust-only shape to published contract
- ACC visual and mechanical handoff model after the current research note becomes stable

### Governance and launch credibility

- Threat-model structure once S2 and S4 stabilize enough for a useful skeleton
- Governance document posture for decision ownership, approvals, and release sign-off
- Launch-facing provenance story before any stronger trustless-proof or on-chain claims

### Hub and hardware

- Pairing-flow contract once S7 moves from runway docs into real scaffold work
- Reboot-safe persistence guarantees for `ferros-hub`
- Target-platform and cross-compilation posture for pre-launch hardware work

---

## Research-Heavy Topics That Should Stay Out of the ADR Lane Until Evidence Improves

- Voting and governance mechanism choice
- Treasury or asset tokenization design
- Post-quantum migration posture beyond high-level threat awareness
- Biomedical, education, robotics, or other vertical-domain application commitments
- Full trustless-proof or decentralized verification posture beyond current signed local evidence

These topics belong in research notes first unless the repo reaches a concrete implementation decision with named evidence.

---

## Roadmap Use Rules

1. Do not treat this file as an approval surface.
2. Add a topic here only if it is plausible enough to matter to stream planning or contributor navigation.
3. Remove or rewrite topics when the real ADR or research note lands.
4. Prefer a research note over a full ADR when the main job is to gather proof instead of freezing a decision.
