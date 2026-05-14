# FERROS ADR Roadmap

This roadmap lists likely future decision and research topics without granting them authority before a record is created.

Numbering is assigned only when a real ADR or research note is added. This file is for navigation and anticipation, not for pre-approving outcomes.

---

## State Note - Operator administration control plane planning (as of 2026-05-13)

This note records the next local-agent-control decision after the first monitor wiring pass. It is additive and does not pre-approve the ACC translation before ADR-040 has implementation evidence and is promoted beyond Draft.

### Landed since last roadmap update

- **ADR-039** - Operator administration agent control plane. Draft. Defines the near-term bridge monitor as the local Administration surface for special-agent categories, escalation triage, packet lifecycle chat, guarded chat creation, and agent-directory/runtime-service separation.
- **ADR-040** - Agent Command Center administration translation reservation. Draft. Reserves the ACC translation scope for multi-operator Administration, lifecycle chat replay, permissioned controls, real status tracking, and hardened service contracts.

### Active follow-up scope

- **Four-corners monitor layout:** project the local monitor into ADR-009 zones: Agent Directory in top-left, Running Services and Console in top-right, Administration and Quick Actions in bottom-left, Packet Lifecycle Chat in bottom-right, Runway mindmap in the center viewport, category carousel on the top edge, Archive drawer on the left edge, and Deny Log drawer on the right edge.
- **Carousel attention model:** special-agent category controls should stay compact, show title plus quick-view icons only in collapsed/running buttons, and center the highest weighted attention score. Administration remains the tie-breaker until another category has strictly higher attention.
- **Runway mindmap:** replace the old checklist-first view with a lane/loop/work-order graph that shows current lane health, packet flow, blocked nodes, reports, escalations, and next actions. Checklist items can remain as node details or proof gates, not the primary mental model.
- **Archive / deny / console / quick-actions placement:** Archive becomes a left-edge drawer for closed chats and lifecycle history; Deny Log becomes a right-edge drawer near service/console context; Console belongs to Running Services; Quick Actions belong to Administration and must be bound to a selected packet, loop, service, or escalation.
- **Real status-tracking requirements:** replace provisional progress with packet-backed fields for `agentId`, `cycleId`, `workOrderId`, `escalationId`, `status`, `statusReason`, `statusDetail`, `startedAt`, `updatedAt`, `staleAfter`, optional `progress`, `sourceAgentId`, and `targetAgentId`.
- **Background-agent lifecycle contract:** every background cycle must finish through report, work order, escalation, denial, archive, or stopped reason. Silent stops become Administration attention.
- **Hierarchy-backed agent source proposal:** keep `.github/agents/*.agent.md` canonical until a generator proves a flat mirror from an outside source tree such as `agents/source/{ferros,coding,business}/`.
- **ACC translation bar:** ADR-040 must remain Draft until ACC has implementation evidence for multi-operator queues, permissioned controls, lifecycle chat replay, audit retention, service contracts, and migration from bridge-local monitor endpoints.

---

## State Note - Post-v0.2.0 operator-UX planning (as of 2026-05-04)

This note records the ADR backlog state after the `v0.2.0` closeout and the first interconnected operator-UX planning packet. It is additive.

### Landed since last roadmap update

- **ADR-028** - Core launch boundary and optional module lanes. Accepted.
- **ADR-029** - Human-operator session orchestration and evidence flow. Accepted. Formalizes the Human Test Backlog, coordinator triage, and evidence closure loop used by the next UX lane.
- **ADR-030 through ADR-035** - Draft scaffolds for proposed material lifecycle, evidence badges, tool-lane disclosures, home-topology lineage, Arena non-evidentiary boundaries, and research-only governance signals.
- **ADR-036** - Draft scaffold for Linux-host touchscreen pilot evidence and driver-lineage cards. Keeps current host-driver use explicit while deferring subcore driver integrity work and later card/deck audit automation.
- `docs/orchestration/INTERCONNECTED-OPERATOR-UX-PLAN.md` is now the active planning packet for the frontend-first operator UX program. Its PDF-derived motifs and quarantined ideas come from the external discovery packet at `/home/homelab001/apps/FERROS-PDF-DISCOVERY-NOTE.md`, which is planning input rather than ADR authority.
- `docs/surfaces/MODULE-AND-FORK-SAFETY.md` now carries the non-ADR architecture note for protected shell layers, safe fork points, and current surface-fork boundaries.
- `docs/operator-sessions/INSTRUCTION-TEMPLATE.md` now scaffolds named operator instruction packets for ready human-test items.

### Open backlog items (not yet an ADR)

- Ratify or rework ADR-030 through ADR-035 after the first implementation evidence exists.
- Module and fork-safety ADR follow-up, if the current architecture note hardens into a decision boundary.

---

## State Note — Post-BATCH-2026-04-27-C (as of 2026-04-27)

This note records the ADR backlog state after the BATCH-2026-04-27-C reconciliation pass. It is additive — no ADR bodies are modified here.

### Landed since last roadmap update

- **ADR-018, ADR-019, ADR-020** — Harvest trilogy (S6): `sheetgen-rust`, `botgen-rust`, and `workspace-rust` disposition decisions. All three Accepted.
- **ADR-021** — Dependency admission policy. Accepted.
- **ADR-022** — Decision program governance. Accepted. Establishes ADR evidence tiers and the research-note lane.
- **ADR-023** — Onramp policy. Accepted. Defines how discovered HA entities route through the S5 consent surface before becoming canonical FERROS state.
- **ADR-024** — Ledger / value-transfer posture. **Proposed only.** Pending counsel red-line and ratification. Do not promote.
- **ADR-027** — Service parity broker and assurance tiers. **Proposed.** Extends ADR-023 with reversible provider association and transaction-scoped trust requirements for high-impact actions.
- **RN-2026-05** — Voting and decision models. Active research note preserving `workpace-rust` vote taxonomy and tally patterns without promoting voting into the ADR lane yet.

### Open backlog items (not yet an ADR)

- Post-ratification ledger follow-up — scope and implementation path for ADR-024, after counsel review is complete.
- Card/deck game ADRs — ADR-015 and ADR-016 are referenced in the near-term FILLER lane; their exact scope is pending research-note completion.
- Multi-device coordination ADRs — pairing-flow contract, reboot-safe persistence guarantees, and target-platform cross-compilation posture (all awaiting `ferros-hub` scaffold work in S7).
- Module packaging and discovery UX — define how optional Home Assistant, local LLM runtime, and external LLM API modules are listed, installed, updated, disabled, and removed.
- Controlled test-home rollout criteria — define the handoff bar between coordinated lab installs and the later unmanaged independent-install phase.
- Interconnected operator UX follow-up ADR ratification — ADR-030 through ADR-035 now exist as drafts and need evidence-backed ratification or revision.

### Blocked

- **ADR-024** — blocked on counsel red-line. Do not advance to Accepted without ratification.

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
- Module-lane assurance and trust model once optional integrations start shipping to controlled test homes

### Hub and hardware

- Pairing-flow contract once S7 moves from runway docs into real scaffold work
- Reboot-safe persistence guarantees for `ferros-hub`
- Target-platform and cross-compilation posture for pre-launch hardware work

---

## Research-Heavy Topics That Should Stay Out of the ADR Lane Until Evidence Improves

- Voting and governance mechanism choice — active in [RN-2026-05-voting-decision-models.md](./_RESEARCH-NOTES/RN-2026-05-voting-decision-models.md); do not promote until FERROS has a concrete decision surface and evidence beyond prior art
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
