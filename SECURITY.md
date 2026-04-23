# Security Policy

FERROS is pre-1.0 and its security posture is still being formalized. This file is intentionally minimal: it defines the current reporting path without claiming production readiness.

---

## Supported branch

Only `main` is considered supported right now. There are no long-term-supported release branches yet.

## Reporting a vulnerability

Do not file public issues for suspected vulnerabilities.

1. Prefer GitHub private vulnerability reporting or a private security advisory if that surface is available for this repo.
2. If private GitHub reporting is not available, contact the maintainers privately using the ownership information in `CODEOWNERS` and request a private channel before sharing details.
3. Include the affected paths, expected impact, reproduction steps or proof-of-concept details, and the commit or branch if the report depends on work that is not on `main`.

## Response expectations

FERROS is maintained by a small team. There is no formal response SLA yet. The intent is to acknowledge good-faith reports promptly and coordinate remediation before public disclosure.

## Scope

This policy applies to security-relevant issues in:

- Rust crates under `crates/`
- schemas and contracts under `schemas/` and `docs/contracts/`
- runtime and harness surfaces under `harnesses/`, `site/`, and `docs/`
- automation and release surfaces under `.github/` and `tools/`

## Safe harbor

Good-faith research is welcome. Please avoid actions that would:

- access data that is not yours
- modify or destroy data
- degrade service availability
- retain persistent access after confirming an issue
- publish exploit details before maintainers have had a reasonable chance to assess the report