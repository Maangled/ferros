# S5 Research Note - Shell Navigation Depth Audit

**Date:** 2026-04-28
**Owning stream:** S5 primary
**Output feeds:** D1 operator script; Batch F consent/control definition
**Status:** Audit note only. No shell HTML or harness behavior is changed.

---

## Purpose

This note audits the current localhost shell against the S5 six-degree reach rule from `streams/S5-ux/SURFACE-FIRST-SHELL.md`.

The current shell is read-first and stages lifecycle intent copy. It does not send browser-issued writes. The audit therefore separates "reachable observation tasks" from "future privileged tasks."

---

## Current Shell Surfaces

| Zone | Current role |
|---|---|
| Top identity/status | Shell identity, backend state, route, agent/deny counts |
| Left navigation | Agents, grants, and deny-log route buttons plus registry list |
| Center | Agents table, grants table, or deny-log table |
| Inspector | Selected agent, grant, or deny-event detail |
| Tools | Profile path input, agent filter, staged lifecycle method copy |
| Consent/audit | Selected-agent lifecycle intent note, future grant/revoke home, latest deny event |
| Bottom status | Route, selected item, profile path, refresh/error state |

---

## Interaction Budget Results

| Workflow | Degree count | Current result |
|---|---:|---|
| Inspect an agent status | 2 | Select `Agents`, then select agent row |
| Review current grants | 1 | Select `Grants` route |
| Review deny log | 1 | Select `Deny log` route |
| Inspect a deny event | 2 | Select `Deny log`, then select event |
| Filter agent registry | 1 | Type in agent filter already visible in tools |
| Change profile read path | 1 | Type path in profile input, then refresh observation |
| Stage lifecycle intent copy | 2 | Select agent; audit/tools slots show `agent.run` or `agent.stop` intent |
| Observe local run/stop after CLI | 3 | Run CLI out-of-band, refresh shell, inspect selected agent |

All current observation tasks are within six degrees.

---

## Future Task Budgets

These tasks are not wired yet, but their eventual designs should stay within the same limit.

| Future workflow | Target budget | Current prerequisite |
|---|---:|---|
| Browser-issued `agent.run` / `agent.stop` | 4 or fewer | Consent/audit gate must fire before write RPC |
| Capability grant | 6 or fewer | Grant-write surface must exist and legal copy must be cleared |
| Profile import/export through shell | 4 or fewer | S2-local adapter and frozen-boundary harness |
| Onramp accept/reject | 5 or fewer | Proposed-item staging and audit-log accept event |
| Profile recovery after failed import | 5 or fewer | Profile recovery UX and rollback messaging |

If a future implementation needs more degrees than this, S5 should promote the action into an existing home slot instead of adding modal chains.

---

## D1 Operator Tasks

| D1 task | In current shell? | Notes |
|---|---|---|
| View agent registry | Yes | Agents route |
| View selected agent detail | Yes | Inspector |
| View grant state | Yes | Grants route / snapshot |
| View deny log | Yes | Deny-log route and latest deny copy |
| Explain lifecycle intent | Yes | Read-only tools/audit copy |
| Send lifecycle write from browser | No | Future control bar |
| Grant/revoke from browser | No | Future privileged surface |
| Accept HA onramp item | No | Future onramp surface |

D1 can use the shell for observation and consent-flow visibility. It should not claim browser control.

---

## Audit Findings

1. The shell topology still follows the surface-first rule. It uses fixed slots, not draggable windows.
2. Current D1 observation workflows fit comfortably under six degrees.
3. The weakest point is profile-path refresh: it is visible, but the operator must understand that the shell is reading a local profile path, not editing it.
4. The consent/audit slot is correctly reserved for future privileged work without pretending that writes are available.
5. The next browser-issued lifecycle control wave should add controls in the existing tools/audit slots instead of creating a new modal stack.

---

## Batch F Input

Batch F should use this audit when defining:

- deny-log UX wording;
- lifecycle control entry bar;
- profile surface wireframe;
- onramp consent slot placement.

The target is not a bigger shell. The target is to make the existing slots do real work without breaking the six-degree reach rule.

