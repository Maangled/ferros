# S4 Research Note — Policy Engine Invariants

**Date:** 2026-04-27  
**Owning stream:** S4 primary  
**Output feeds:** D1 bring-up checklist, S5 consent flow UX, S3 agent manifest catalog  
**Status:** Research note — catalogs existing test invariants. No new tests added; no crate modifications.

---

## Purpose

This note catalogs the `DenyByDefaultPolicy` invariants proven by the existing test suite, expressed in plain English. It is intended to help operators, contributors, and D1 demo planners understand what the policy engine guarantees without reading Rust source.

All invariants in this note correspond to real test functions in the repository. Test names are given so they can be verified with `cargo test`.

---

## Policy Engine Types (from `ferros-core`)

| Type | Role |
|---|---|
| `PolicyEngine` trait | Evaluates a `CapabilityRequest` against a grant set; returns `PolicyDecision` |
| `DenyByDefaultPolicy` | The only policy implementation today; denies anything not explicitly granted |
| `CapabilityRequest` | A request from a profile (`profile_id`) for a named `Capability` |
| `Capability` | A dot-scoped name string (e.g., `"consent.read"`, `"agent.echo"`); validated at construction |
| `CapabilityGrantView` | Trait over any grant record; exposes `profile_id()`, `capability()`, and `is_active()` |
| `PolicyDecision` | `Allowed` or `Denied(PolicyDenialReason)` |
| `PolicyDenialReason` | `NoGrantsPresented`, `ProfileNotGranted`, or `CapabilityNotGranted` |
| `CapabilityError` | `Empty` or `ContainsWhitespace` — returned when constructing an invalid `Capability` |
| `RequesterProfileIdError` | `Empty` or `ContainsWhitespace` — returned when constructing a `CapabilityRequest` with an invalid profile id |

---

## Invariants Catalog

### I-1: A capability name must not be empty

**Test:** `capability_rejects_empty_names` (`crates/ferros-core/tests/capability_policy.rs:119`)

`Capability::new("")` and `Capability::new("   \n")` both return `Err(CapabilityError::Empty)`.

A capability name must contain at least one non-whitespace character. Whitespace-only strings are treated as empty.

---

### I-2: A capability name must not contain whitespace

**Test:** `capability_rejects_whitespace_names` (`crates/ferros-core/tests/capability_policy.rs:125`)

`Capability::new("consent read")` returns `Err(CapabilityError::ContainsWhitespace)`.
`Capability::new("consent\tread")` returns `Err(CapabilityError::ContainsWhitespace)`.

Any internal whitespace (space, tab, newline) in a capability name is rejected. Dots and hyphens are allowed (e.g., `"consent.read"`, `"agent.echo"`).

---

### I-3: Dot-scoped names are valid capability names

**Test:** `capability_accepts_dot_scoped_name` (`crates/ferros-core/tests/capability_policy.rs:137`)

`Capability::new("consent.read")` succeeds and returns a `Capability` whose `as_str()` equals `"consent.read"`.

The canonical naming convention for FERROS capabilities is `<scope>.<verb>` (e.g., `agent.echo`, `consent.read`, `runtime.dispatch`).

---

### I-4: A request profile id must not be empty

**Test:** `request_rejects_empty_profile_ids` (`crates/ferros-core/tests/capability_policy.rs:144`)

`CapabilityRequest::new("", capability)` and `CapabilityRequest::new("   ", capability)` both return `Err(RequesterProfileIdError::Empty)`.

Profile ids follow the same whitespace rules as capability names.

---

### I-5: A request profile id must not contain whitespace

**Test:** `request_rejects_profile_ids_with_whitespace` (`crates/ferros-core/tests/capability_policy.rs`)

`CapabilityRequest::new("profile alpha", capability)` returns `Err(RequesterProfileIdError::ContainsWhitespace)`.

---

### I-6: An empty grant set always produces `NoGrantsPresented`

**Test:** `deny_when_no_grants_are_present` (`crates/ferros-core/tests/capability_policy.rs`)

When the grant set is empty (`&[]`), `DenyByDefaultPolicy::evaluate` returns `Denied(NoGrantsPresented)` regardless of the request.

**Operator meaning:** If no grants have been configured, every agent start attempt is denied with reason `NoGrantsPresented`. This is the initial state of a fresh FERROS install.

---

### I-7: An exact profile + capability match on an active grant produces `Allowed`

**Test:** `allow_when_profile_and_capability_match_exactly` (`crates/ferros-core/tests/capability_policy.rs`)

When the grant set contains an active grant with `profile_id == request.profile_id` and `capability == request.capability`, the decision is `Allowed`.

**Operator meaning:** An agent whose required capability is granted to the correct profile is permitted to start.

---

### I-8: Grant order does not affect the decision

**Test:** `allow_when_matching_grant_is_not_first` (`crates/ferros-core/tests/capability_policy.rs`)

The policy engine scans all grants. A matching grant that is not the first in the slice still produces `Allowed`. The engine does not short-circuit on non-matching grants in a way that would miss a later valid grant.

---

### I-9: An inactive grant does not authorize the capability

**Test:** `inactive_grant_does_not_authorize_matching_capability` (`crates/ferros-core/tests/capability_policy.rs`)

If the matching grant has `is_active() == false`, it is not counted. The decision falls back to `Denied(NoGrantsPresented)` if no other active grants exist.

**Operator meaning:** Revoked grants (inactive) are not evaluated as if they were still active. A grant must be active to authorize a request.

---

### I-10: Profile present in grants but requesting a different capability produces `CapabilityNotGranted`

**Test:** `deny_when_profile_exists_but_capability_is_missing` (`crates/ferros-core/tests/capability_policy.rs`)

When the grant set has an active grant for the requesting profile but for a different capability, the decision is `Denied(CapabilityNotGranted)`.

**Operator meaning:** An agent that requires `agent.echo` will be denied if the profile only has `agent.timer` granted. The profile is recognized, but the specific capability is missing.

---

### I-11: The capability granted only to other profiles produces `ProfileNotGranted`

**Test:** `deny_when_request_profile_has_no_grants` (`crates/ferros-core/tests/capability_policy.rs`)

When active grants exist but none are for the requesting profile, the decision is `Denied(ProfileNotGranted)`.

**Operator meaning:** A second device profile trying to use a capability granted only to the first profile will be denied with reason `ProfileNotGranted`.

---

### I-12: Profile has grants, but the requested capability belongs only to another profile → `CapabilityNotGranted`

**Test:** `deny_when_target_capability_only_exists_for_other_profiles` (`crates/ferros-core/tests/capability_policy.rs`)

When profile A has a grant for `consent.write` and profile B has a grant for `consent.read`, a request from profile A for `consent.read` is `Denied(CapabilityNotGranted)` — the profile is recognized (A has grants), but the specific capability is not granted to A.

---

### I-13: Active exact-match allows regardless of neighbor grant ordering (property-based)

**Test:** `active_exact_match_allows_regardless_of_grant_order` (proptest, `crates/ferros-core/tests/capability_policy.rs`)

Proptest-verified over arbitrary token sequences and grant orderings: an active exact match always produces `Allowed`, regardless of what other grants surround it in the list. Non-matching grants do not suppress a valid grant.

---

### I-14: Mismatch grants with varied specs produce the correct denial reason (property-based)

**Test:** `mismatch_grant_specs_produce_correct_denial_reason` (proptest, `crates/ferros-core/tests/capability_policy.rs`)

The denial reason is determined by a priority rule:
1. If no active grants exist at all → `NoGrantsPresented`
2. If active grants exist and at least one matches the requesting profile → `CapabilityNotGranted`
3. If active grants exist but none matches the requesting profile → `ProfileNotGranted`

---

## Boundary-Layer Invariants (from `ferros-runtime`)

These tests are in `crates/ferros-runtime/tests/boundaries.rs` and cover the executor and message-bus layers that sit above the policy engine.

### B-1: Executor runs jobs in submission order (FIFO)

**Test:** `executor_runs_jobs_in_submission_order` (`boundaries.rs:48`)

A `StubExecutor` with two submitted jobs returns them in FIFO order. `pending_jobs()` decrements after each `pop_next()`.

---

### B-2: `InMemoryExecutor` preserves FIFO submission order

**Test:** `in_memory_executor_preserves_submission_order` (`boundaries.rs:71`)

The concrete `InMemoryExecutor` (the CI-validated executor implementation) preserves FIFO order across submit/pop cycles.

---

### B-3: Message bus routes messages to the correct recipient

**Test:** `message_bus_routes_messages_by_recipient` (`boundaries.rs:92`)

Messages sent to different recipients are queued independently. A `try_recv("agent.charlie")` only returns the message addressed to `agent.charlie`, not the one for `agent.bravo`. Message metadata (sender, recipient, capability, payload, nonce) is preserved verbatim.

---

### B-4: Message bus returns `None` for an unknown recipient

**Test:** `message_bus_reports_empty_queue_for_unknown_recipient` (`boundaries.rs:119`)

`try_recv("agent.charlie")` returns `Ok(None)` when no message has been sent to that recipient. It does not error; it reports an empty queue.

---

## Deny-Log Observability for D1

The policy invariants listed above are observable in the operator-facing deny-log slot. For D1 demo purposes:

| What the operator wants to show | Relevant invariant | How to trigger |
|---|---|---|
| Any denial at all (deny-by-default working) | I-6 | Attempt to start an agent before any grants are configured |
| Profile recognized but capability missing | I-10 | Grant `agent.timer` to the profile; attempt to run the echo agent (requires `agent.echo`) |
| Completely unknown profile | I-11 | Attempt an agent run from a profile with no grants at all |
| Inactive grant does not authorize | I-9 | Revoke a grant; re-attempt the agent run |

---

## Source Documents

- `crates/ferros-core/tests/capability_policy.rs` — primary test file (authoritative)
- `crates/ferros-runtime/tests/boundaries.rs` — runtime boundary tests
- `crates/ferros-core/src/capability.rs` — `Capability`, `DenyByDefaultPolicy`, policy types
- `crates/ferros-core/src/lib.rs` — public re-exports
