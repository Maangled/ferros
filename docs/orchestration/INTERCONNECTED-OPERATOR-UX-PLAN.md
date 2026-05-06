# Interconnected Operator UX Plan

**Status:** Active planning packet  
**Date:** 2026-05-05  
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

Current repo status on 2026-05-05:

- ADR-029 is now in repo as the accepted operator-session loop record.
- `docs/orchestration/OPERATOR-SESSION-PATTERN.md`, `docs/backlogs/HUMAN-TEST-BACKLOG.md`, `docs/operator-sessions/SESSION-LOG.md`, and `docs/operator-sessions/INSTRUCTION-TEMPLATE.md` now scaffold the first human-test lane.
- ADR-030 through ADR-035 now exist as draft scaffolds and should be treated as follow-on decision work, not ratified doctrine.
- `docs/surfaces/MODULE-AND-FORK-SAFETY.md` is now the temporary architecture note for base-module protection and safe surface-fork posture.
- The first seeded shared-module batch is now landed on the current localhost shell and H9 proof path: `ProposedMaterialCard`, `ReceiptStrip`, `EvidenceBadge`, `SourceLineageCard`, `OperatorStepCard`, and `ToolLaneCard` now render the live runway and route-scope surfaces on `site/agent-center-shell.html` without widening transport, grant mutation, or onramp accept/reject behavior.
- The first shell-frame extraction seam is now landed on that same shell: protected-chrome markers are live, route metadata is single-homed, the top edge plus bottom status rails and side-panel headers now render through shared shell renderers, and `LifecycleControlCard` plus `LifecycleOutcomeCard` now carry the current lifecycle surfaces. Focused `ferros-node` tests and live same-origin H9 validation passed for this wave. The shell still behaves as one effective browser-writer lane: selected-agent lifecycle only.
- The current runway path now also carries `ConsentBoundaryCard` and `RecoveryStateCard`, keeping route-local artifacts, blocked canonical state, and the next honest operator recovery move explicit on the same read-only `/runway-summary.json` route. Focused `ferros-node` tests and live H9 validation passed for this follow-up at `83 passed, 0 failed, 2 skipped`.
- The first touch posture slice is now landed on the current shell: `TouchAnchorStrip` is emitted through the protected top chrome, coarse-pointer tap targets are enlarged, and top/bottom context stays sticky at narrow or coarse-pointer posture. This is a Linux-host pilot surface only; it does not claim subcore-owned display/input drivers or a sealed driver evidence chain.
- The current shell now also exposes `Home-Hub`, `Forge`, and `Arena` as bounded routes on the same state it already owns: `Home-Hub` reads topology and lineage posture from the loaded runway summary, `Forge` stages preview-only bundle and artifact posture from the current `/profile` result plus route-local artifacts, and `Arena` stages preview-only runtime posture from the selected agent plus runway recovery state. Focused `ferros-node` shell and harness tests pass for this route expansion.
- Grouped human-session packets now exist for the next non-hardware waves: H1 current shell read surfaces, H2 profile round trip, H3 Home-Hub read, and H4 preview surfaces are all ready to issue. Touch remains hardware-gated through HTB-013 rather than being falsely promoted to a pure browser-only pass.

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

Current implementation note:

- All six seeded shared modules above are now landed on the current localhost shell as the first shared-module batch. They currently cover the runway summary path plus route-scope and audit copy surfaces.
- Base-shell extraction is no longer only future work: the current shell now emits stable protected-chrome markers, single-homes route metadata, and renders the top edge, bottom status rails, and side-panel headers through shared shell renderers while preserving the current behavior.
- The current lifecycle control and outcome surfaces now render through `LifecycleControlCard` and `LifecycleOutcomeCard`, and the runway route now stages blocked consent plus operator recovery through `ConsentBoundaryCard` and `RecoveryStateCard`. The shell remains intentionally narrow: onramp accept/reject, grant mutation, remote transport, and broader parallel writer lanes remain follow-on work.
- The current shell now also exposes a dedicated touch-navigation layer through `TouchAnchorStrip` and coarse-pointer posture rules, making the narrow layout usable on a dedicated operator screen while keeping the current four-corner grammar intact.
- The current shell now also exposes `Home-Hub`, `Forge`, and `Arena` as bounded read or preview routes on the same shell: topology and lineage stay tied to the loaded runway summary, preview bundle posture stays tied to `/profile`, and runtime preview stays tied to the selected-agent plus runway recovery state. None of these routes widens writes or claims canonical authority.
- The next evidence step for touch is not more CSS; it is a real Linux-host touchscreen session that records connector path, host display/input drivers, and the evidence ceiling before later subcore driver modularization or audit work exists.

Next batch order:

1. Issue the first operator-ready runway review packet for HTB-004 against the current boundary/recovery slice.
2. Run the first Linux-host touchscreen pilot and capture the driver/connector chain explicitly so HTB-007 and the later ACC audit surface can work from named host modules instead of vague environment assumptions.
3. Prove the first operator-session loops on the current shell without widening writes beyond selected-agent lifecycle control.
4. Keep actual onramp accept/reject wiring queued behind the audit-log consent seam rather than faking it in the browser.

### Batched autopilot run

Use this as the default execution posture for the rest of the interconnected operator UX plan:

1. Default to Batch Mode for execution work. If the next lane is already implied by this packet and does not require new user authority, continue without pausing between implementation slices.
2. Parallelize only the lanes whose anchors do not overlap. Shell contracts, shared truth docs, backlog status, session packets, and ADR indexes remain serial-only.
3. Close each batch with local proof before widening scope: focused Rust tests, shell or harness checks, touched-doc validation, and explicit backlog or session-state updates.
4. Treat human execution, hardware availability, policy escalation, and failed local proof as the only valid stop points. Do not stop at a planning-only boundary once the next batch is already named.

Continuous batch recursion rule:

1. After each closed batch, immediately spawn the next recursion cycle on the same plan: `build -> self-review -> cross-review -> harness proof -> doc and backlog sync -> next slice`.
2. If self-review or cross-review finds a defect, route to a same-batch hotfix branch and re-run that cycle before promoting any new scope.
3. If proof passes and no escalation trigger exists, continue automatically into the next named slice without waiting for a new planning checkpoint.
4. Always write one bounded delta note per cycle (what changed, what failed, what is next) so recursive runs stay inspectable.

Parallel module train rules:

1. Keep one serial control lane for shared anchors: shell contracts, route metadata, backlog state, session log, and ADR index updates.
2. Run module construction in parallel trains when they do not edit the same anchor file:
	- Train M1: shared utility modules and renderer extraction.
	- Train M2: Profile and ACC bounded surfaces.
	- Train M3: Home-Hub read surfaces and source-lineage views.
	- Train M4: Forge and Arena preview surfaces.
3. Each train must pass its local tests before merge; cross-train integration runs only after all touched trains settle.
4. If two trains collide on one anchor, downgrade to serial merge order instead of forcing concurrent edits.

Autopilot batch map:

| Batch | Agent lanes | Scope | Exit gate | Human-test targets |
|-------|-------------|-------|-----------|--------------------|
| A0 - Current shell closeout | Shared Module + Testing + Docs/ADR | Close the current runway and touch substrate, keep HTB-004 issued, and keep HTB-013 prepared as the first hardware-backed touch packet | Current `ferros-node` shell and harness checks pass; HTB-004 packet stays ready; HTB-013 packet, findings stub, and session-log row exist | HTB-004, HTB-013 |
| A1 - Current shell read surfaces | ACC + Shared Module + Testing + Docs/ADR | Harden the current shell and ACC-adjacent read surfaces without widening writes: inspect-agent clarity, deny recovery, touch posture, proposed-material inspection, receipt readback, tool disclosure, and evidence-badge honesty | Slice-level shell or harness proof lands, instruction packets exist, and operator evidence language stays honest about blocked or proposed state | HTB-001, HTB-002, HTB-007 via HTB-013, HTB-008, HTB-009, HTB-011, HTB-012 |
| A2 - Profile lane | Profile UX + Shared Module + Testing + Docs/ADR | Promote Profile to a first-class module, but keep scope bounded to init, show, export, import, and explicit proposed-material framing | Profile-focused proof passes and docs still state that grant or revoke mutation remains out of scope | HTB-003 |
| A3 - Home-Hub read lane | Home-Hub + Testing + Docs/ADR + Discovery Carryover | Land topology, onramp proposal, and source-lineage read surfaces without treating Home Assistant or host state as canonical identity truth | Source-lineage and blocked-state proof lands; operator packet exists; docs keep Home-Hub evidence boundaries explicit | HTB-010 |
| A4 - Preview surfaces | Forge + Arena + Testing + Docs/ADR | Land preview-only authoring and runtime surfaces, provenance displays, and non-evidentiary result staging | Preview proof lands and docs still reject silent promotion into canonical profile or progression | HTB-005, HTB-006 |

Human campaign order after the implementation batches:

1. H0: issue HTB-004 first, then run HTB-013 as soon as the touchscreen and host capture tools are available.
2. H1: run the current-shell read-surface items together once A1 closes cleanly: HTB-001, HTB-002, HTB-008, HTB-009, HTB-011, and HTB-012. HTB-007 stays coupled to HTB-013 because it still needs the physical touchscreen pass.
3. H2: run HTB-003 after the bounded Profile lane closes.
4. H3: run HTB-010 after the Home-Hub read lane closes.
5. H4: run HTB-005 and HTB-006 after the preview-only Forge and Arena lanes close.

If any human wave finds a defect that blocks the next meaningful operator step, route it to a hotfix lane that returns to the owning batch above instead of widening the queue ad hoc.

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

### Continuous self-review cadence

Apply this cadence to every active module train, not just final handoff:

1. Slice plan: name the smallest shippable delta for one module or seam.
2. Build pass: land code and local harness updates for that slice.
3. Self-review pass: builder agent critiques its own change for regressions, edge cases, and doctrine drift.
4. Cross-review pass: a second agent checks overlap seams (Profile, ACC, Forge, Arena, Home-Hub, and shared shell anchors).
5. Proof pass: run focused tests plus live shell or harness checks tied to the changed seam.
6. Truth-sync pass: update backlog, operator packets, and orchestration notes only with proven outcomes.
7. Recurse: open the next slice immediately unless a stop condition is hit.

Mandatory stop conditions:

1. Human authority required.
2. Hardware evidence required.
3. Policy or ADR escalation required.
4. Local proof fails three consecutive attempts on the same seam.

Everything else defaults to continue.

### Parallel construction matrix for site modules

Use this matrix to keep module work concurrent while preserving shared-shell integrity:

| Lane | Primary modules | Can run in parallel with | Must stay serial with |
|------|-----------------|--------------------------|------------------------|
| L0 | Base shell anchors and route registry | none | all lanes |
| L1 | Utility cards, badges, strips, and chips | L2, L3, L4, L5 (if no anchor overlap) | L0 |
| L2 | Profile and ACC surfaces | L1, L3, L4, L5 | L0 |
| L3 | Home-Hub read and lineage surfaces | L1, L2, L4, L5 | L0 |
| L4 | Forge preview authoring surfaces | L1, L2, L3, L5 | L0 |
| L5 | Arena runtime preview surfaces | L1, L2, L3, L4 | L0 |
| L6 | Harness and test contracts | L1, L2, L3, L4, L5 | final integration gate |
| L7 | Docs, backlog, and operator packets | after proof per lane | L0 truth anchors |

Parallel merge protocol:

1. L0 changes open and close first.
2. L1-L5 run concurrently in bounded slices.
3. L6 validates each lane slice and then validates the combined surface.
4. L7 updates only proven state; never pre-promote readiness.

---

## Phase 5 - Surface build order

1. Continue refactoring the current shell behind a base module without widening behavior.
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