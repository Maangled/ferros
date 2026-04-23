# ADR-019 — Harvest workpace-rust for Shell Delivery, Typed IPC, and UX Surface Patterns

**Status:** Accepted  
**Date:** 2026-04-23  
**Stream:** S6  
**Deciders:** FERROS stream coordination / S6 harvest lane

---

## Context

`Maangled/workpace-rust` is not a workspace manager in the window-manager sense. The audit in `.tmp/w-r.md` shows a browser-native shell with a signed module envelope, typed WebSocket protocol, iframe isolation, HUD chrome, client cache verification, and capability-scoped `postMessage` routing. Those patterns overlap most strongly with FERROS S5 and, to a lesser extent, later S3 message surfaces. They do not provide a direct implementation path for FERROS runtime or workspace-state persistence because the repo is deeply coupled to WASM-in-browser execution, DOM APIs, blob URLs, IndexedDB, and actix-web session handling.

---

## Decision

**FERROS will harvest workpace-rust as post-G3 prior art for shell composition, typed IPC, signed delivery envelopes, and capability-scoped UI messaging. FERROS will not treat it as a source of workspace-state, browser mechanics, or direct implementation code.**

This ADR constrains the workpace input to architecture and UX-shell patterns that FERROS can re-express in its own host and rendering model.

---

## Rationale

The audit shows real value in workpace-rust's typed command protocol, its split transport topology, and its insistence on signed module payloads with explicit delivery envelopes. It also shows sharp boundaries: the repository does not actually implement persistent workspace context, multi-workspace coordination, or a reusable module-runtime abstraction outside the browser. Translating only the durable architectural ideas avoids importing browser-specific assumptions into FERROS.

### Options considered

| Option | Summary | Reason not chosen (if rejected) |
|--------|---------|--------------------------------|
| Harvest protocol and shell patterns only (chosen) | Use workpace-rust as a typed IPC, signed-envelope, and shell-composition reference | — |
| Treat workpace-rust as the basis for FERROS shell implementation | Port browser shell runtime, iframe isolation, cache, and session model directly | Rejected because the runtime is tightly coupled to `web_sys`, IndexedDB, blob URLs, iframe CSP, and actix-web server assumptions |
| Ignore workpace-rust completely | Reject it as too browser-specific to be useful | Rejected because the audit surfaces several strong transport, integrity, and least-privilege messaging patterns worth preserving |

---

## Consequences

**Positive:**
- S5 gets a concrete prior-art source for shell layering, slot composition, focus-mode chrome behavior, and transport discipline.
- S3 can later borrow the typed command and response envelope discipline without importing the browser runtime.
- S6 now has an explicit boundary that prevents browser-specific workpace mechanisms from leaking into FERROS core architecture.

**Negative / trade-offs:**
- FERROS still needs to define its own render boundary, cache store, and shell runtime instead of porting the browser implementation.
- The repo name may keep misleading contributors into assuming it contains workspace persistence; this ADR explicitly says it does not.
- Some attractive security patterns, especially iframe and CSP isolation, remain inspiration-only because FERROS may target non-browser hosts.

---

## Adopt / Adapt / Reference / Discard

### Adopt

- Typed command and response enums: adopt the discipline of a serializable `command` / `response` protocol with exhaustive handling.
- Dispatcher plus split-pipe topology: adopt the idea of one inbound command dispatcher with dedicated transport or service pipes behind it.
- Signed payload verification: adopt the Blake3 plus Ed25519 integrity pattern as a FERROS delivery and verification primitive where signed artifacts matter.
- Capability-scoped UI message ACLs: adopt the principle that child surfaces only emit commands they are explicitly allowed to send.

### Adapt

- PageData delivery envelope: adapt the outer envelope shape of `module_id`, `version`, content payload, state payload, and integrity metadata, but replace browser-specific asset fields with FERROS-owned render or module descriptors.
- PageState carrier: adapt the `user_prefs`, `form_data`, and `custom_state` shape into a FERROS workspace-context model only after FERROS defines actual pane, session, and topology semantics.
- Version-verified cache handshake: adapt the client-version to server-verified refresh pattern, but back it with a FERROS-native store rather than IndexedDB.
- HUD slot composition: adapt the named-slot shell composition model for future FERROS local UI surfaces.

### Reference

- Iframe sandbox plus CSP isolation: reference only as a statement of boundary intent for untrusted render surfaces.
- Five-layer shell stack and focus-mode HUD transitions: reference as S5 UX guidance, not implementation law.
- HTML comments and layout intent embedded in `os.html`: reference as design reasoning for future shell docs.
- Boot-chain sequencing from init page to shell runtime: reference as a staging example for multi-phase shell startup.

### Discard

- IndexedDB module storage as an implementation requirement.
- Blob URL asset-delivery mechanics.
- Actix-session cookie middleware and HTTP session specifics.
- Hard-coded module routing tables.
- Development-mode signing-key lifecycle that writes keys to `/tmp`.
- Browser-specific `eval(import(...))` and DOM mutation plumbing.
- String-template HTML component generation as an architectural pattern.

---

## Downstream Implications

### S5 UX

- S5 should treat this ADR as the prior-art basis for Phase B shell composition, typed transport contracts, and future focus-mode UI behavior.
- S5 should not read `workpace-rust` directly for implementation once this ADR exists unless a new unresolved question appears.

### S3 Agent Center

- S3 may borrow the typed IPC and dispatcher discipline from this ADR when designing its JSON or RPC surfaces.
- S3 should not inherit the browser runtime, shared broadcast response model, or module-routing assumptions.

### S6 Ecosystem Harvest

- This ADR closes the first workpace-rust audit pass.
- Later revisits should only happen if FERROS needs a more detailed shell-boundary or message-bus study.

---

## Compliance

- If FERROS chooses a browser host as a first-class launch vehicle, revisit the reference-only classifications here.
- If FERROS adopts a concrete workspace-state persistence model, revisit whether the adapted `PageState` concept needs a stronger contract.
- If any implementation stream starts copying browser-specific workpace code directly, revisit this ADR rather than bypassing the harvest lane.

---

## References

- `.tmp/w-r.md`
- [ADR-013](c:\Users\mosle\OneDrive\Desktop\GPTs\ferros\ferros\docs\adr\ADR-013-legacy-integration-strategy.md)
- [streams/S5-ux/README.md](c:\Users\mosle\OneDrive\Desktop\GPTs\ferros\ferros\streams\S5-ux\README.md)
- [streams/S6-harvest/README.md](c:\Users\mosle\OneDrive\Desktop\GPTs\ferros\ferros\streams\S6-harvest\README.md)