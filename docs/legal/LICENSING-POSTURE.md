# FERROS Licensing Posture

> **Status: DRAFT — Awaiting counsel red-line. This document does not constitute legal advice or a final licensing statement. It is a structured placeholder for legal review.**

**Last updated:** 2026-04-27  
**Version:** 0.1-draft

---

## Purpose

This document describes the FERROS project's intended licensing posture: which license applies to which artifact class, what the intent is for dual-licensing or contribution agreements, and what questions remain open for counsel resolution.

---

## Current License Coverage

| Artifact class | License | File |
|----------------|---------|------|
| Rust source code (`crates/**`, `xtask/**`) | See `LICENSE` | [`/LICENSE`](../../LICENSE) |
| Documentation (`docs/**`, `README.md`, `*.md` root files) | See `LICENSE-DOCS` | [`/LICENSE-DOCS`](../../LICENSE-DOCS) |
| Schemas (`schemas/**`) | See `LICENSE` | [`/LICENSE`](../../LICENSE) |
| HTML harnesses (`harnesses/**`, `site/**`) | See `LICENSE` | [`/LICENSE`](../../LICENSE) |
| Stream planning docs (`streams/**`) | See `LICENSE-DOCS` | [`/LICENSE-DOCS`](../../LICENSE-DOCS) |

> _Red-line target: confirm whether the two-license split (code vs. docs) is intentional and whether all artifact classes are correctly attributed. Confirm whether schemas are code or documentation for licensing purposes._

---

## Dual-Licensing Intent (Open Question)

FERROS is built with a long-term commercialization path that includes a marketplace, hosted services, and potential enterprise integrations. The current license may not be compatible with all commercial use cases.

Possible future posture: dual-license the core Rust substrate under an open-source license and a commercial license. Counsel should advise on:

- Whether the current license permits the commercial paths described in `ROADMAP.md`.
- Whether a Contributor License Agreement (CLA) is required before accepting external contributions.
- Whether a dual-license structure would conflict with existing dependencies' licenses.

> _Red-line target: assess dual-licensing viability against current dependency manifest (see ADR-021). Flag any dependencies whose licenses constrain FERROS's commercialization options._

---

## Contribution Posture

Currently there is no formal CLA. The `CONTRIBUTING.md` file at the repository root describes the contribution process. Any contribution is assumed to be made under the same license as the file being contributed to.

> _Red-line target: determine whether a CLA is required before the v0.1.0 launch tag. If required, draft CLA language._

---

## Trademark Posture

The name "FERROS" is not formally registered as a trademark. Use of the name by third parties for derivative or competing products is not explicitly addressed.

> _Red-line target: advise on trademark registration. Recommend whether to file before or after the v0.1.0 launch tag._

---

## Schema and Data Format Licensing

`schemas/profile.v0.json` and `schemas/capability-grant.v0.json` are the frozen v0 schemas. Third-party tools that implement FERROS profile compatibility would need to reference these schemas. The licensing of schema files — particularly whether compatibility implementations require a license — is an open question.

> _Red-line target: determine whether schema-file licensing requires separate treatment from software licensing. Consider whether a schema compatibility covenant or patent non-assertion covenant is appropriate._

---

_This document is a structured placeholder. It maps the current repo artifact structure to license files and surfaces open questions for counsel. It does not constitute a legal document, legal advice, or a binding licensing commitment._
