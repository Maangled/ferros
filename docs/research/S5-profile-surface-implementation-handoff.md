# S5 Profile Surface Implementation Handoff

Status: Batch F research handoff. This defines the next implementation boundary for a browser-visible profile surface without reopening the frozen S2 contract.

## Fixed Inputs

- S2 owns `ProfileId`, `CapabilityGrant`, `schemas/profile.v0.json`, and `schemas/capability-grant.v0.json`.
- G2 is closed. The published v0 schemas are frozen and must not be mutated for this surface.
- The real `ferros` binary already supports `profile init`, `show`, `export`, and `import`.
- S5 is a consumer. It may expose local profile state through the localhost shell, but it does not define profile identity semantics.

## Minimum Surface

The first browser profile surface should expose only these four operator intents:

| Intent | S5 display | Backing constraint |
|--------|------------|--------------------|
| `init` | Create a local profile at an operator-selected local path | Must refuse overwrite unless the backing local command/API already refuses overwrite |
| `show` | Render the unsigned `profile.v0.json` consumer view | Must not show signed-profile internals as a published browser contract |
| `export` | Produce a local bundle through the existing S2 path | Must not claim multi-device sync or key-wrap support |
| `import` | Accept a local bundle through the existing rollback-safe import path | Must surface rejection without leaving ambiguous partial state in the UI |

Grant creation, grant revocation, passphrase wrapping, cloud sync, and remote profile access are out of scope for this first surface.

## Implementation Boundary For The Next Code Wave

The next code wave should choose an explicitly scoped localhost adapter for the four intents above. That adapter may be CLI-backed or API-backed, but it must satisfy these rules:

- It remains local-only and same-origin with the current shell host.
- It preserves S2's overwrite, parse, verify, and rollback behavior instead of reimplementing those rules in browser JavaScript.
- It returns structured success/error information that S5 can render without inventing a new profile schema.
- It has focused tests against temp-file-backed local state.
- It does not add grant/revoke mutation as an incidental convenience.

If the adapter requires new JSON-RPC methods, that must be the explicit scope of that later wave because the current read-first S3 contract is load-bearing.

## UX Slots

| Slot | First useful content |
|------|----------------------|
| Profile path | Local path entry plus current load status |
| Identity summary | `ProfileId`, device label, created/updated fields when present in the frozen profile view |
| Import/export status | Last local operation result, including rollback-safe rejection copy |
| Consent boundary | Clear note that grants are visible elsewhere but not mutated by this surface |

## Stop Lines

- Do not edit `schemas/profile.v0.json` or `schemas/capability-grant.v0.json`.
- Do not reopen G2 evidence wording.
- Do not add browser grant/revoke actions.
- Do not publish remote profile access.
- Do not treat the surface as D1 evidence until a separate evidence wave records an operator session.
