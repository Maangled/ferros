# FERROS Threat Model

> Status: living skeleton for a pre-1.0 system.
>
> This file records the current security posture without pretending the platform is fully stabilized. It should become more precise as S2, S3, S4, and S7 contracts harden.

---

## Purpose

FERROS is a local-first system that aims to increase user ownership over identity, automation, and device behavior. The threat model therefore focuses on the ways that user sovereignty could be weakened through key compromise, capability escalation, opaque dependencies, contract drift, or misleading operational claims.

This document is a planning and review surface, not a certification claim.

Primary related references:

- [DOCTRINE.md](./DOCTRINE.md)
- [SECURITY.md](./SECURITY.md)
- [docs/adr/ADR-002-smart-contract-boundaries.md](docs/adr/ADR-002-smart-contract-boundaries.md)
- [docs/adr/ADR-021-dependency-admission-policy.md](docs/adr/ADR-021-dependency-admission-policy.md)
- [docs/gates/G2.md](docs/gates/G2.md)
- [docs/gates/G3.md](docs/gates/G3.md)
- [docs/gates/G4.md](docs/gates/G4.md)

---

## Current System Scope

The current in-repo scope covered by this threat model is:

- local profile and grant material under S2
- runtime policy and consent enforcement under S4
- local agent registration and execution surfaces under S3
- schemas, fixtures, and contract documents that shape cross-stream behavior
- site, harness, and documentation surfaces where misleading authority could cause unsafe implementation or user assumptions

Out of scope for now:

- a completed hardware trust and boot chain model
- production network perimeter design
- marketplace or multi-tenant agent distribution
- a finalized distributed or on-chain verification architecture

---

## Security Goals

FERROS should preserve the following properties as the platform evolves:

1. Users retain local ownership of their profile, grants, and meaningful execution state.
2. Privileged actions require explicit, auditable consent.
3. Capability checks fail closed when the system cannot prove a grant.
4. Cross-stream contracts stay explicit and versioned.
5. Documentation and UI surfaces do not overstate what is implemented or trustworthy.
6. Dependency and custody assumptions remain narrow and reviewable.

---

## High-Value Assets

| Asset | Why it matters | Current status |
|------|----------------|----------------|
| Profile key material | Root of identity and grant signing | Active S2 surface; still pre-1.0 |
| Signed capability grants | Permission boundary for privileged actions | Active S2 surface; evidence landed |
| Runtime consent policy | Enforces deny-by-default behavior | Active S4 surface; property tests landed |
| Agent manifests and lifecycle state | Controls what can run and how it is described | Partial S3 surface; remote contract not yet frozen |
| Schemas and fixtures | Prevent silent contract drift | Active and versioned, but still evolving pre-freeze |
| Launch and governance claims | Prevent false trust and premature authority | Active S8 responsibility |

---

## Trust Boundaries

| Boundary | What crosses it | Current posture |
|---------|------------------|-----------------|
| User intent -> capability execution | Consent, grants, runtime decisions | Must stay explicit and auditable |
| Profile crate -> runtime / agent layers | Identity and grant material | Still tightening through G2 and G3 |
| Published schema -> implementation | Fixtures, validators, Rust types, docs | Versioning exists; freeze discipline remains critical |
| Prototype / docs -> product authority | Contributor interpretation and UX direction | High documentation risk if truth surfaces drift |
| Local system -> future hub / HA integration | Device control and persisted automation | Mostly future-facing; not launch-ready yet |

---

## Threat Actors

| Actor | Example concern |
|------|-----------------|
| Curious local attacker | Reads local files, tries to extract profile or grant material |
| Malicious agent author | Requests excessive capabilities or exploits weak manifest boundaries |
| Supply-chain attacker | Introduces dependency risk or hidden behavior through admitted packages |
| Careless contributor | Mutates a contract, gate claim, or docs surface without updating owning truth |
| Overclaiming operator or maintainer | Treats a prototype, CI run, or draft doc as stronger proof than it is |

---

## Priority Threats Right Now

### 1. Key or grant compromise

If profile key material or signed grant artifacts are exposed or mishandled, the user loses meaningful control over identity and delegated capability.

Current controls:

- Ed25519-based identity and signed grant work exists in S2.
- G2 remains open until the profile contract and CLI story are more fully stabilized.
- The doctrine explicitly prefers explicit consent and auditable state transitions.

Current gaps:

- storage hardening and operating-system-level protections are not yet mature
- the full lifecycle for import/export/grant/revoke is still incomplete

### 2. Capability escalation or consent bypass

If agents can execute privileged actions without a valid grant or if deny paths are not durable, FERROS fails its core sovereignty claim.

Current controls:

- deny-by-default runtime posture
- property tests for policy behavior
- runnable `ferros demo` path that exercises deny behavior

Current gaps:

- richer host hardening beyond the in-memory local demo
- future remote contract and shell surfaces still need to preserve the same fail-closed model

### 3. Contract drift between schemas, docs, and implementations

If published schemas, fixtures, CLI behavior, and Rust types diverge, contributors and users act on false interfaces.

Current controls:

- schema versioning discipline
- fixtures and harnesses
- contract overview and gate docs

Current gaps:

- G2 freeze is not finished
- S3 JSON/RPC and manifest publication are still open questions

### 4. Documentation or prototype authority drift

If a prototype or stale document is treated as authoritative, the system can inherit unsafe behavior or false claims without code ever changing.

Current controls:

- honest status dashboard
- ADR doctrine and governance baseline
- `docs/ORCHESTRATION.md` explicitly downgraded to historical context

Current gaps:

- truth-sync remains ongoing work across streams
- HTML incubation surfaces are still more concrete than some live implementation slices

### 5. Dependency and build-chain expansion without review

Unreviewed dependencies can widen the attack surface, undermine portability, or silently import authority the project does not control.

Current controls:

- ADR-021 dependency policy
- small-team review culture
- workspace and CI baselines under S1

Current gaps:

- no dedicated security review checklist for dependency admission yet
- no release-signing or provenance program yet

---

## Existing Defenses and Evidence

- Signed grants and signed-profile envelope work in S2
- Profile fixture validation and contract-first schema discipline
- Deny-by-default runtime posture with property tests in S4
- Local demo and CI proof for the current S3/S4 convergence slice
- SECURITY reporting path in [SECURITY.md](./SECURITY.md)
- Dependency admission policy in [docs/adr/ADR-021-dependency-admission-policy.md](docs/adr/ADR-021-dependency-admission-policy.md)
- Governance taxonomy and evidence basis in [docs/adr/ADR-022-decision-program-governance.md](docs/adr/ADR-022-decision-program-governance.md)

---

## Known Weaknesses and Open Work

- G2 is still open, so profile and grant surfaces are not yet fully frozen.
- G3 is not closed, so the runtime and agent-center boundary is still converging.
- S5 shell work has not yet proven that browser-facing control surfaces preserve the same consent posture.
- S7 launch surfaces have not reached real-hardware evidence yet.
- The project does not yet claim hardened local secret storage, formal sandboxing, or verified update provenance.

---

## Immediate Security Priorities

1. Finish G2 truth and profile-freeze work so identity contracts stop shifting underneath downstream streams.
2. Preserve fail-closed consent semantics as S3 JSON/RPC and S5 shell work begin.
3. Keep documentation and status surfaces honest as implementation moves.
4. Add security review prompts to future dependency and release workflows.
5. Expand this file once S7 real-hardware work creates a concrete device and network threat surface.

---

## Review Rule

When a contributor proposes a change that affects identity, consent, execution, persistence, or launch claims, review it against this question:

**Does the change increase or reduce local user sovereignty, explicit consent, contract clarity, and auditability?**

If the answer is unclear, the change likely needs either stronger evidence or a clearer boundary record before it lands.
