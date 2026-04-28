# S3 Research Note - Remote Transport Boundary

**Date:** 2026-04-28
**Owning stream:** S3 primary; S5 and S7 consumer awareness
**Output feeds:** Batch F control definitions; later remote/hub hardening
**Status:** Boundary note only. No JSON/RPC method, transport, auth model, or bridge protocol is added.

---

## Purpose

S3 now has a useful localhost JSON/RPC host and a local-only lifecycle/write slice. That can sound like a remote-control surface if the docs get sloppy. This note draws the line: what is local and published today, what is not remote, and what must exist before any broader transport claim is honest.

---

## Current Published Surface

| Surface | Current status |
|---|---|
| Local shell host | `ferros-node shell [port]` serves the current shell on localhost |
| Read JSON/RPC | `agent.list`, `agent.describe`, `agent.snapshot`, `grant.list`, `denyLog.list` |
| Local write JSON/RPC | `agent.run`, `agent.stop`, routed through `LocalAgentApi` on the same local state path |
| Observation path | `agent.snapshot`, `agent.describe`, and `denyLog.list` after local state changes |
| Deny behavior | Denied writes return local authorization errors and persist stable deny-log summaries |

This is enough for the current S5 localhost shell and for local D1 rehearsal planning.

---

## Not Published

| Not published | Why it matters |
|---|---|
| LAN or internet binding | No remote listener contract, access policy, or threat model is published |
| Authentication model | No token, session, pairing, mTLS, or browser auth boundary exists |
| Transport adapters | No Unix socket, named pipe, WebSocket, SSE, health endpoint, or subscription contract is published |
| Remote privileged writes | `agent.run` and `agent.stop` are localhost-only; grant writes are not in this surface |
| HA-facing transport | Home Assistant bridge transport remains S7-owned and unimplemented |
| Hub remote observation | No log streaming, remote inspection, or multi-device control exists |

---

## Localhost Contract Rules

1. A method being JSON/RPC does not make it remote-safe.
2. `agent.run` and `agent.stop` are local-only lifecycle methods on the current shell host.
3. Read-after-write observation stays on existing read methods; no subscription or push state is implied.
4. Grant creation and revoke stay out of this S3 surface.
5. S5 browser controls may consume this local surface only after their consent/audit gate is wired.
6. S7 may plan around registry and local inspection surfaces, but not around a remote hub transport.

---

## Requirements Before Remote Transport Can Be Claimed

| Requirement | Owner |
|---|---|
| Explicit bind address and allowed client model | S3/S4 |
| Authentication and consent boundary | S3/S4/S5 |
| Threat-model update for remote control | S8/S4 |
| Transport-specific tests | S3/S4 |
| Clear denial/error envelope for remote callers | S3 |
| Operator UX that distinguishes local from remote control | S5 |
| S7 handoff for HA bridge, if applicable | S7 |

Until these exist, docs should say "localhost shell host" or "local JSON/RPC" rather than "remote transport."

---

## Consumer Guidance

| Consumer | May rely on today | Must not infer |
|---|---|---|
| S5 | Local shell can read `agent.snapshot` and stage lifecycle intent | Browser-issued control is already safe |
| S7 | Registry/list/log surfaces exist for runway planning | HA bridge transport exists |
| S4 | Local writes route through current policy/deny path | Remote runtime host is published |
| S8 | Contributor docs can describe localhost controls as future work | Launch or remote-control readiness |

---

## Batch F Input

Batch F can use this note to define deny-log UX, lifecycle-control entry bars, and HA bridge seam catalogs without accidentally widening S3. Any implementation wave after Batch F should choose either:

- a browser-local control slice that remains localhost-only, or
- a real remote transport wave with auth, threat model, and tests in scope.

