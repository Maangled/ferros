# Interconnected Operator UX Plan

**Status:** Active planning packet  
**Date:** 2026-05-04  
**Streams:** S5 / S7 / S8 / Cross-cutting

This document is the working orchestration plan for the next FERROS UX lane. It treats Profile, ACC, Forge, Arena, and Home-Hub as harmonized surface modules built on a protected base module, while letting missing backend seams become explicit Agent Backlog items instead of vague blockers.

The legacy paper scrape is complete enough for planning. The paper is a motif and concept source, not a binding technical specification.

---

## Planning inputs

- Durable discovery packet: `/home/homelab001/apps/FERROS-PDF-DISCOVERY-NOTE.md`
- Current shell stream: `streams/S5-ux/README.md`
- Current operator loop substrate: `docs/orchestration/LOCAL-DRIVER.md`
- Current hardware/operator precedent: `docs/orchestration/HARDWARE-QUEUE.md`
- Current launch-facing onramp boundary: `docs/adr/ADR-023-onramp-policy.md`

Do not re-import or commit the original PDF as implementation input. Use the discovery note as the planning packet and keep the paper archival.

---

## Discovery stance

The paper contributes three useful inputs:

1. Visual and interaction motifs: HUD layers, cards, vault-like protected slots, topology maps, contract-like flows, mobile-first stacks, and status rails.
2. Local-sovereignty concepts: local profiles, consented data movement, quarantined material, source lineage, user-controlled tool selection, and evidence-like receipts.
3. Research backlog prompts: governance, merit, simulation, provenance, local AI, Home-Hub topology, and safety questions that may later become ADRs.

The paper does **not** override FERROS doctrine or onramp invariants. It does not authorize identity policy, biometrics, surveillance, public-chain authority, vague security ratings, or unsupported legal or policing claims.

---

## Translation rules

Use these translations when moving discovery-note material into FERROS UX, docs, or ADRs:

| Paper language or motif | FERROS-safe translation |
|-------------------------|-------------------------|
| Hero HUD | Base Shell / operator HUD grammar |
| Contracts | Capability-bound operation or consent gate |
| Home / nHomes | Home-Hub local node and topology surface |
| Bank / vault | Local store, staged artifact, or protected evidence slot |
| Matter / Energy / Data card | Proposed-material card, capability summary, data preview, or receipt |
| MED / Satori / Hero card | Research-only knowledge or progression signal |
| Token / NFT / mint | Artifact, receipt, provenance record, or export bundle |
| Brain / AI tool selector | Local or external tool lane selector with declared data access |
| Sandbox / simulation | Arena runtime preview and non-evidentiary result staging |
| Security rating | Evidence badge backed by named proof only |
| Trade | Explicit accept or reject, arm or confirm, or consent receipt |

---

## Phase 0 - Paper scrape status

Phase 0 is complete enough for planning and partially complete for later visual-reference work.

Completed:

1. `Paper Scrape Agent` produced the discovery packet.
2. `PDF Visual Review Agent` extracted representative motifs: HUD panels, mobile surfaces, card flows, node and topology diagrams, consent-like transaction flows, and local-home server diagrams.
3. `Paper-to-FERROS Mapper` routed discovery-note material into Profile, ACC, Forge, Arena, Home-Hub, shared modules, operator sessions, ADR candidates, backlog, and rejection.
4. `Paper Review Auditor` quarantined dated assumptions, unsafe claims, speculative governance, biometrics or surveillance, NFT authority, and legal or product-copy risks.

Still optional later:

1. Render and archive selected page thumbnails outside the repo for visual reference.
2. Convert selected motifs into wireframe descriptions.
3. Cross-check any technical claim before it becomes ADR rationale.

---

## Phase 1 - Orchestration docs and ADRs

Create the operating substrate before implementation begins.

Current repo status on 2026-05-04:

- ADR-029 is now in repo as the accepted operator-session loop record.
- `docs/orchestration/OPERATOR-SESSION-PATTERN.md`, `docs/backlogs/HUMAN-TEST-BACKLOG.md`, `docs/operator-sessions/SESSION-LOG.md`, and `docs/operator-sessions/INSTRUCTION-TEMPLATE.md` now scaffold the first human-test lane.
- ADR-030 through ADR-035 now exist as draft scaffolds and should be treated as follow-on decision work, not ratified doctrine.
- `docs/surfaces/MODULE-AND-FORK-SAFETY.md` is now the temporary architecture note for base-module protection and safe surface-fork posture.

Required docs and ADRs:

1. `docs/adr/ADR-029-human-operator-session-orchestration-and-evidence-flow.md` - landed
2. `docs/orchestration/OPERATOR-SESSION-PATTERN.md` - landed
3. `docs/backlogs/HUMAN-TEST-BACKLOG.md` - landed
4. `docs/operator-sessions/SESSION-LOG.md` - landed
5. `docs/operator-sessions/INSTRUCTION-TEMPLATE.md` - landed
6. `docs/surfaces/MODULE-AND-FORK-SAFETY.md` - landed as an architecture note
7. `docs/adr/ADR-030-proposed-material-receipts-and-canonical-state.md` - draft scaffold
8. `docs/adr/ADR-031-evidence-badges-and-security-rating-claims.md` - draft scaffold
9. `docs/adr/ADR-032-local-tool-and-ai-tool-selector-boundary.md` - draft scaffold
10. `docs/adr/ADR-033-home-topology-and-onramp-source-lineage.md` - draft scaffold
11. `docs/adr/ADR-034-arena-non-evidentiary-runtime-boundary.md` - draft scaffold
12. `docs/adr/ADR-035-governance-and-merit-signals-are-research-only.md` - draft scaffold

Target operating loop:

`Human Test Backlog -> Agent instructs human -> Results + Human Comments -> close item or route to Agent Backlog / hotfix / coordinator -> completed agent items return to Human Test Backlog -> next human instruction`

---

## Phase 2 - Shared module system

Use **modules**, not templates.

1. **Base Shell Module**: protected chrome, route anchors, status rail, layout rules, accessibility, focus behavior, and translated HUD grammar.
2. **Shared Utility Modules**: panel headers, badges, pills, drawers, consent affordances, safe rendering helpers, source-lineage chips, receipt strips, evidence badges, and local-only badges.
3. **Surface Modules**: Profile, ACC, Forge, Arena, Home-Hub, and future variants.
4. **Data Adapter Modules**: `/rpc`, `/profile`, `/runway-summary.json`, local artifacts, schemas, and future module-lane endpoints.
5. **Capability and Consent Gate Modules**: local-only scope, explicit arm and confirm, ADR-023 quarantine, and no hidden writes.
6. **Harness Modules**: base stability, surface behavior, adapter contracts, touch readiness, and consent invariants.

The base module is protected and swappable only through a layered safety model. Surface modules can fork more freely. Base-shell changes require stricter review and harness proof.

### Shared modules seeded from discovery

1. `ProposedMaterialCard`
2. `ReceiptStrip`
3. `EvidenceBadge`
4. `SourceLineageCard`
5. `ToolLaneCard`
6. `OperatorStepCard`

---

## Phase 3 - Category agents

Run category agents in parallel when anchors do not overlap.

1. `Profile UX Agent`: profile init, show, export, import, session modes, seal-chain display, no grant or revoke leak
2. `ACC Agent`: registry, lifecycle, grants readout, deny log, and audit view
3. `Forge Agent`: authoring, card, deck, and module creation, preview handoff, and provenance
4. `Arena Agent`: runtime preview, result staging, and Profile progression proposals
5. `Home-Hub Agent`: onramp proposals, bridge status, and HA/local-LLM/external-LLM module lanes
6. `Shared Module Agent`: base shell, utilities, accessibility, route and drawer behavior, and shell grammar
7. `Testing Agent`: harnesses, preflight gates, touch checks, and operator readiness
8. `Docs/ADR Agent`: ADR-029, module layering, operator-session docs, and backlog schema
9. `Discovery Carryover Agent`: keeps the discovery note mapped into backlog and ADR candidates without importing quarantined material

### Discovery Carryover Agent rules

May promote:

- HUD and chrome motifs
- Card, deck, and module vocabulary where it maps to artifacts
- Source-lineage displays
- Consent receipts
- Proposed-material surfaces
- Local topology and status diagrams
- Sandbox and runtime staging language

Must quarantine:

- Biometric identity flows
- Government-ID onboarding
- NFT or public-chain authority claims
- Automatic sync into canonical profile or grants
- Surveillance, policing, or AI jury flows
- Anti-cheat or root-access framing
- Legal allegations as product copy
- Merit or governance systems without separate ADR review

---

## Phase 4 - Recursive review

Use recursive subagents, but cap the depth.

1. Builder Agent produces a lane slice.
2. Local Reviewer checks the slice.
3. Cross-Stream Seam Auditor checks Profile, ACC, Arena, and Home-Hub boundaries.
4. Consent and Boundary Validator checks frozen schemas, local-only transport, no hidden mutation, and no consent bypass.
5. Discovery Quarantine Reviewer checks that discovery-note material was translated safely.
6. Gatekeeper Review Agent returns `continue`, `stop-clean`, or `stop-escalate`.

Recursion depth max: `2`. Shared truth surfaces stay serial-only.

---

## Phase 5 - Surface build order

1. Refactor the current shell behind a base module without behavior change.
2. Split ACC and local shell into surface modules.
3. Promote Profile to a first-class module.
4. Add Home-Hub onramp proposal and decision surfaces.
5. Add Forge as visible authoring and export module.
6. Add Arena as visible runtime and result-staging module.
7. Harmonize all surfaces around one shell grammar, one status rail, one consent and audit slot, evidence badges, source-lineage cards, receipt strips, and touch posture.

### Surface-specific incorporation

**Base Shell**

- Use now: persistent chrome, status rail, route anchors, consent and audit slot, visible backout states, local or runway or blocked badges.
- Avoid: game or fantasy copy unless intentionally adopted later.

**Profile**

- Use now: local profile, import/export, proposed-material queue, progression receipt lane.
- Avoid: biometrics, government-ID verification, Hero alignment, or public-chain profile authority.

**ACC**

- Use now: contract-like capability cards, grants, required capabilities, deny log, audit view, explicit arm and confirm.
- Avoid: hidden grant or revoke, browser privilege widening, or automatic external authority.

**Home-Hub**

- Use now or soon: local topology card, proposed HA entity card, source lineage, restart and power-cycle evidence slots, display-only runway state until backend accept or reject is real.
- Avoid: treating Home Assistant as identity truth or silently registering entity state as profile state.

**Forge**

- Use later: card, deck, and module authoring, export/provenance preview, required capability summary, artifact receipt preview.
- Avoid: NFT, mint, or coin authority language.

**Arena**

- Use later: sandbox or runtime preview, simulation result staging, non-evidentiary badge, explicit handoff proposal to Profile.
- Avoid: silently promoting simulation output to Profile or progression.

---

## Backend queues expected

Frontend-first work should generate explicit backend queues for:

1. Profile-agent binding
2. Onramp accept or reject endpoint
3. Arena result staging
4. Forge export contract and provenance
5. Home-Hub command bridge
6. Grant or revoke staging flow
7. Optional module registry
8. Receipt persistence and read seam
9. Proposed-material staging read seam by source
10. Evidence-badge source mapping
11. Tool-lane declaration endpoint or manifest
12. Source-lineage metadata for local artifacts and onramp proposals

---

## Pre-operator verification

Before the first human operator session:

1. Relevant Rust tests pass.
2. The existing shell acceptance harness passes.
3. Consent-invariant review passes.
4. Touch checks pass: no hover-only primary actions, persistent route and status context, visible backout.
5. Operator instruction file exists.
6. Findings template exists.
7. Rollback and stop criteria are written.
8. Discovery quarantine check passes.
9. Evidence badges are backed by named seams or marked as placeholders.
10. Proposed material is visibly non-canonical until accepted.
11. Receipt surfaces distinguish rehearsal or runway receipts from canonical evidence.

---

## First Human Test Backlog items

1. Inspect Agent
2. Recover From Deny
3. Profile Round Trip
4. Onramp Review
5. Forge Preview
6. Arena Preview
7. Touch Posture
8. Proposed Material Inspection
9. Receipt Readback
10. Home-Hub Topology Read
11. Tool Lane Disclosure
12. Evidence Badge Sanity Check

---

## Key decision

FERROS can override current readiness **only** in the sense that it can design and build the whole interconnected UX shape now, including honest blocked states and generated backend queues.

FERROS must **not** override safety boundaries:

- no frozen-schema mutation
- no fake consent
- no remote-transport claim
- no hidden grant or revoke
- no canonical mutation before explicit accept
- no unsupported security rating
- no identity truth from external systems
- no discovery-note authority without FERROS ADR review
- no automatic sync into profile, grants, or progression
- no surveillance, biometric, policing, or governance authority surfaces without separate research and ADR approval

---

## Updated working posture

The discovery note now acts as a motif and research source for:

1. shared shell grammar
2. proposed-material cards
3. evidence and receipt language
4. source-lineage display
5. Home-Hub topology
6. Forge and Arena artifact grammar
7. ADR candidates
8. quarantine lists

Implementation should use the discovery note as input, but the repo's accepted ADRs and current FERROS docs remain the authority for consent, canonical state, and implementation boundaries.