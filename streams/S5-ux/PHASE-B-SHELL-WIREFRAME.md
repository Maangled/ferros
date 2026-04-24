# S5 UX — Phase B Shell Wireframe

**Status:** Active pre-G3 design artifact  
**Date:** 2026-04-23  
**Applies to:** Local web shell served on `localhost` by the future Phase B implementation

---

## Purpose

This note turns the surface-first shell direction into a concrete shell shape that can be implemented in boring HTML, CSS, and vanilla JS once S3's JSON/RPC surface is ready.

It is grounded in:

- `forge-workbench.html` for slot layout, collapse anchors, and focus-mode behavior
- `agent-command-center.html` for agent-center subject matter
- [SURFACE-FIRST-SHELL.md](./SURFACE-FIRST-SHELL.md) for the fixed-slot and six-degree rules

This is a shell artifact, not a data-contract change. S5 is still consuming S3 and S4 boundaries.

---

## Slot map

| Slot | Primary role | Default surface |
|------|--------------|-----------------|
| Top edge | Identity and transport chrome | Shell identity, localhost target, RPC health, consent state |
| Top-left | Route and registry slot | Route list plus agent registry |
| Center | Primary focus slot | Selected agent summary, grant review, or deny-log table |
| Top-right | Inspector slot | Selected object detail, capability state, metadata |
| Bottom-left | Tools slot | Filters, quick actions, route-scoped controls |
| Bottom-right | Consent and audit slot | Consent prompts, audit handoff, assistant guidance |
| Bottom edge | Status rail | Active route, latency, recent errors, deny counters |

Collapsed surfaces remain visible as anchors in their home slots. Focus mode enlarges the current center or inspector surface without hiding shell identity.

---

## Default desktop wireframe

```text
+--------------------------------------------------------------------------------------------------+
| FERROS LOCAL SHELL | localhost:PORT | rpc: ready/degraded/offline | consent: idle/armed | user |
+------------------------------+------------------------------------------+----------------------+
| ROUTES / REGISTRY            | PRIMARY FOCUS                            | INSPECTOR            |
| - Agents                     | Selected route view                      | Selected agent       |
| - Grants                     |                                          | capability summary   |
| - Deny log                   | Agents: status overview                  | object metadata      |
| agents list / filters        | Grants: review queue                     | recent activity      |
| collapsed anchor when hidden | Deny log: table or focused event         | collapsed anchor     |
+------------------------------+------------------------------------------+----------------------+
| TOOLS / FILTERS              | STATUS RAIL                              | CONSENT / AUDIT      |
| route-scoped actions         | route | rpc latency | errors | denies    | privileged action    |
| quick filters                | recent shell notices                     | confirm / cancel     |
| focus toggle                 | collapsed anchor when hidden             | audit handoff note   |
+------------------------------+------------------------------------------+----------------------+
```

---

## Narrow-layout fallback

The first HTML shell should still be desktop-first, but the topology should degrade cleanly below a narrow breakpoint.

- Top edge and bottom edge remain persistent.
- `Top-left` and `Top-right` become slide-in drawers with visible anchors.
- `Bottom-left` and `Bottom-right` collapse into tabs above the status rail.
- The degree budget does not change; drawers count the same as revealing a collapsed surface.

---

## Workflow frames

### Inspect agent status

```text
1. Open or reveal ROUTES / REGISTRY.
2. Select an agent from the registry list.
3. Center shows status summary; INSPECTOR shows capability and metadata detail.
```

Target budget: 3 degrees

### Grant capability

```text
1. Select agent.
2. Center switches to grant review for that agent.
3. Choose capability action.
4. Arm privileged action.
5. CONSENT / AUDIT slot opens the confirmation surface.
6. Confirm or cancel.
```

Target budget: 6 degrees

### Review deny log

```text
1. Switch route or slot to Deny log.
2. Select or filter an event.
3. INSPECTOR shows the full deny event context.
```

Target budget: 3 degrees

---

## Surface behavior rules

- The shell chooses slot placement; child surfaces do not move themselves.
- Registry, inspector, tools, and consent surfaces may collapse, but their anchors stay visible.
- Consent is route-scoped, not a free-floating modal stack.
- Deny-log review should reuse the inspector and center slots before introducing new layers.
- Empty, loading, degraded, and offline states must appear in-slot rather than as detached overlays.

---

## Minimal shell intent vocabulary

These are the smallest typed intents needed to preserve shell control without pushing layout decisions into child surfaces.

```ts
type ShellSlot =
  | 'topEdge'
  | 'topLeft'
  | 'center'
  | 'topRight'
  | 'bottomLeft'
  | 'bottomRight'
  | 'bottomEdge';

type RouteId = 'agents' | 'grants' | 'denyLog';

type SurfaceId =
  | 'registry'
  | 'agentSummary'
  | 'grantReview'
  | 'denyLog'
  | 'inspector'
  | 'tools'
  | 'consent'
  | 'audit';

type SurfaceMode = 'pinned' | 'collapsed' | 'focus';

type ShellIntent =
  | {
      type: 'shell.route.select';
      route: RouteId;
      origin: 'user' | 'deepLink' | 'rpc';
    }
  | {
      type: 'shell.surface.present';
      slot: ShellSlot;
      surface: SurfaceId;
      mode: SurfaceMode;
    }
  | {
      type: 'shell.object.select';
      objectType: 'agent' | 'grant' | 'denyEvent';
      objectId: string;
    }
  | {
      type: 'shell.focus.enter';
      surface: SurfaceId;
      objectId?: string;
    }
  | {
      type: 'shell.focus.exit';
      surface: SurfaceId;
    }
  | {
      type: 'shell.privileged.arm';
      action: 'grant.create' | 'grant.revoke';
      requestId: string;
      agentId: string;
      capability: string;
    }
  | {
      type: 'shell.consent.resolve';
      requestId: string;
      decision: 'confirm' | 'cancel';
    }
  | {
      type: 'shell.audit.handoff';
      requestId: string;
      denyEventId?: string;
    }
  | {
      type: 'shell.connection.update';
      state: 'connecting' | 'ready' | 'degraded' | 'offline';
      latencyMs?: number;
    };
```

### Intent boundaries

- `shell.route.select` changes the active route, not the shell layout rules.
- `shell.surface.present` allows the shell to swap slot contents without giving children layout control.
- `shell.object.select` is the common bridge from registry, grant list, and deny log into focus or inspector detail.
- `shell.privileged.arm` and `shell.consent.resolve` keep privileged actions explicit and auditable.
- `shell.audit.handoff` hands a privileged event into the audit surface without inventing another modal layer.

---

## Build-facing notes

- The first HTML shell can be built as fixed CSS grid regions with slot-specific `data-slot` attributes.
- The center surface should be the only region that routinely swaps full route content.
- Inspector and consent surfaces should be reusable across all three core workflows.
- The current read-first S3 contract now exposes `agent.list`, `agent.describe`, `grant.list`, and `denyLog.list`. Phase B should consume those routes directly before asking S3 for privileged writes or richer transport features.
- No mock-only features should be added just to fill the shell; empty and loading states are acceptable pre-G3.

---

## References

- [SURFACE-FIRST-SHELL.md](./SURFACE-FIRST-SHELL.md)
- [DOCS-HTML-PROTOTYPE-AUDIT.md](./DOCS-HTML-PROTOTYPE-AUDIT.md)
- [README.md](./README.md)
- [../../docs/forge-workbench.html](../../docs/forge-workbench.html)
- [../../docs/agent-command-center.html](../../docs/agent-command-center.html)