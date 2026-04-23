# ADR-021 — Dependency Admission Policy for Browser Surfaces and Rust Substrate

**Status:** Accepted  
**Date:** 2026-04-23  
**Stream:** Cross-cutting  
**Deciders:** FERROS stream coordination / S1 foundation / S5 UX / S8 docs

---

## Context

FERROS already operates with a restrictive dependency posture, but that rule is currently spread across implementation reality and a few adjacent documents instead of one governing ADR. Browser-facing artifacts are expected to run offline and on `file://`, which is why `docs/AGENT_GUIDE.md` forbids external libraries, CDN scripts, `import`, `fetch`, and service-worker assumptions. The current Rust workspace also reflects a deliberately small substrate: `xtask` is std-only, `ferros-core` has no third-party crates, and `ferros-profile` uses a narrow set of dependencies centered on serialization and schema validation. Without an explicit policy ADR, later streams could still drift into ambient dependency culture by normalizing frameworks, package-manager pipelines, or convenience crates as the default answer.

FERROS needs one decision that makes the current philosophy explicit before more browser surfaces, runtime crates, and tooling layers appear.

---

## Decision

**FERROS will keep browser surfaces framework-free and package-free, keep the Rust substrate minimal, and admit third-party crates only when they materially reduce risk or complexity inside narrow audited domains such as serialization, cryptography, encoding, or schema validation. Convenience, familiarity, or ecosystem fashion are not sufficient reasons to add a dependency.**

Additional policy rules:

- Browser-delivered FERROS surfaces in `docs/`, `site/`, `harnesses/`, and equivalent offline artifacts must not depend on npm packages, bundled framework runtimes, CDN scripts, remote modules, or other network-loaded code.
- Rust workspace crates should default to std/core plus FERROS-owned code and other local workspace crates.
- A third-party Rust crate is acceptable only when the alternative is materially riskier or more complex than using a small, well-understood external library in a narrow boundary.
- Dependency review includes transitive cost. A crate that appears small but imports a wide or unstable tree does not satisfy this policy automatically.
- If a capability appears to require a broad dependency stack, FERROS should first isolate that need behind an owned boundary, defer it to a later layer, or prove why the larger stack is safer than the local alternative.

---

## Rationale

This policy preserves three FERROS priorities that are already visible in the repository.

First, browser surfaces are review artifacts and conformance fixtures as much as they are UI experiments. Keeping them framework-free and package-free ensures they open directly from disk, remain inspectable in one pass, and do not inherit a build or hosting dependency just to render core behavior.

Second, the Rust layer is meant to become the durable substrate, not a dumping ground for ambient ecosystem habits. A small manifest surface makes audits, upgrades, and ownership boundaries easier to reason about as S2 through S7 expand the workspace.

Third, there are domains where "do it ourselves" is the riskier choice. Serialization, cryptography, and similar correctness-sensitive primitives are often safer when delegated to narrowly scoped, established libraries than when recreated ad hoc inside FERROS. The point of this ADR is not zero dependencies at any cost. It is disciplined dependency admission instead of default dependency accumulation.

### Options considered

| Option | Summary | Reason not chosen (if rejected) |
|--------|---------|--------------------------------|
| Restrictive admission policy with narrow audited exceptions (chosen) | Keep browser surfaces package-free, keep Rust minimal, and allow small external crates only where they reduce real risk or complexity | — |
| Framework and package-manager baseline for browser surfaces | Build browser UX around a standard JavaScript framework and dependency pipeline | Rejected because FERROS's current browser surfaces are explicitly offline and `file://`-safe review artifacts; a framework baseline would add operational and audit overhead immediately |
| Normal ecosystem-driven crate adoption for Rust | Treat third-party crates as the default implementation path unless a problem appears later | Rejected because it encourages ambient dependency growth, obscures audits with transitive trees, and weakens FERROS's ownership of its core substrate |

---

## Consequences

**Positive:**
- Offline browser surfaces remain portable, inspectable, and truthful to the no-build delivery contract.
- Contributors get a clear default: write the small local implementation unless a dependency clearly reduces risk in a narrow domain.
- Cargo manifests stay legible as new crates land.
- Dependency audits stay bounded and reviewable instead of becoming background maintenance debt.

**Negative / trade-offs:**
- Some capabilities will take longer to land because convenience libraries and framework shortcuts are not acceptable by default.
- Contributors adding a new crate need to justify scope, transitive cost, and why a local implementation is worse.
- Some features may need to wait for a better boundary instead of being forced into the baseline surface or substrate immediately.

---

## Compliance

- Treat any browser-surface addition of CDN scripts, npm-managed runtime dependencies, remote modules, bundlers as a hard requirement for baseline operation, or framework bootstraps as non-compliant with this ADR.
- Treat any Rust dependency added primarily for convenience, fashion, or speculative future use as non-compliant with this ADR.
- A compliant third-party crate addition must state the narrow domain it serves, why the local alternative is riskier or more complex, and why the dependency's transitive footprint is acceptable.
- Revisit this ADR if FERROS changes the baseline browser-surface delivery model away from offline/package-free artifacts, or if future substrate requirements cannot be met without a broader explicitly bounded dependency strategy.

---

## References

- [docs/AGENT_GUIDE.md](../AGENT_GUIDE.md)
- [ADR-015](./ADR-015-universal-parametric-authoring-workbench.md)
- [ADR-017](./ADR-017-html-surface-incubation-strategy.md)
- [Cargo.toml](../../Cargo.toml)
- [crates/ferros-core/Cargo.toml](../../crates/ferros-core/Cargo.toml)
- [crates/ferros-profile/Cargo.toml](../../crates/ferros-profile/Cargo.toml)
- [xtask/Cargo.toml](../../xtask/Cargo.toml)