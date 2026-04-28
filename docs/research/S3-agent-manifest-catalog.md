# S3 Research Note — Agent Manifest Catalog

**Date:** 2026-04-27  
**Owning stream:** S3 primary; S7 D1 planning  
**Output feeds:** D1 bring-up checklist (docs/research/S7-d1-bring-up-checklist.md), S5 consent flow UX  
**Status:** Research note — catalogs existing manifests and documents the HA bridge shim placeholder. No new implementation.

---

## Purpose

This note catalogs the agent manifests defined in the `ferros-agents` crate, documents the `AgentManifest` struct schema for contributors and operators, and frames what a minimal HA-bridge shim agent would declare for D1 purposes.

This note does not implement the HA bridge, define the HA entity schema, or claim any protocol details beyond what is documented in S7 runway notes.

---

## `AgentManifest` Struct Schema

All agents in the FERROS system are described by an `AgentManifest`. The struct is defined in `crates/ferros-agents/src/manifest.rs`.

```
AgentManifest {
    name:                 AgentName,               // unique identifier, token-valid string
    version:              String,                   // semver string (e.g., "0.1.0")
    required_capabilities: Vec<CapabilityRequirement>,  // list of capability requirements
}
```

### `AgentName`

A validated string that uniquely identifies an agent in the registry. Token-valid: no whitespace, no reserved characters.

### `CapabilityRequirement`

```
CapabilityRequirement {
    profile_id:  ProfileId,  // the profile that must hold the grant
    capability:  String,     // the dot-scoped capability name (e.g., "agent.echo")
}
```

`CapabilityRequirement::is_satisfied_by(&[CapabilityGrant]) -> bool` — returns `true` if any active grant in the slice matches `(profile_id, capability)`.

### `AgentManifest::authorization(&[CapabilityGrant]) -> AuthorizationDecision`

```
AuthorizationDecision::Authorized
AuthorizationDecision::Denied { missing: Vec<CapabilityRequirement> }
```

All required capabilities must be satisfied for `Authorized`. Any unsatisfied requirement is listed in the `Denied::missing` vector.

---

## Registered Agent Catalog

### Agent 1 — Echo Agent

**Purpose:** Reference implementation. Demonstrates the minimal agent lifecycle (register → run → status).

| Field | Value |
|---|---|
| `name` | `"echo"` |
| `version` | `"0.1.0"` |
| `required_capabilities` | `[{ capability: "agent.echo" }]` |
| Initial status | `AgentStatus::Registered` |
| Constructor | `EchoAgent::new(profile_id)` |

**What it does:** Responds to incoming messages by echoing them back to the sender. Used in CI to verify that an agent can be registered, run, and communicate through the message bus.

**Capability requirement in plain English:** The profile associated with this agent must have an active grant for `"agent.echo"`. Without this grant, `DenyByDefaultPolicy` will deny any `CapabilityRequest` for `"agent.echo"` and the agent start attempt is logged as a denial.

---

### Agent 2 — Timer Agent

**Purpose:** Reference implementation. Demonstrates a time-driven agent lifecycle.

| Field | Value |
|---|---|
| `name` | `"timer"` |
| `version` | `"0.1.0"` |
| `required_capabilities` | `[{ capability: "agent.timer" }]` |
| Initial status | `AgentStatus::Registered` |
| Constructor | `TimerAgent::new(profile_id)` |

**What it does:** Fires at configured intervals. Demonstrates that the runtime executor can schedule time-driven jobs alongside event-driven ones.

**Capability requirement in plain English:** The profile associated with this agent must have an active grant for `"agent.timer"`.

---

## HA Bridge Shim Placeholder (D1 Stand-In)

The real HA bridge agent (`ferros-hub` / `ha-bridge`) is not yet implemented. For D1, a named stand-in is acceptable under the D1 gate evidence rules.

The stand-in must:
1. Have a defined bridge seam (not a pure mock).
2. Be named explicitly in the D1 evidence.
3. Demonstrate that the agent center can accept a third-party agent registration.

### Minimal HA bridge shim manifest (placeholder)

The echo agent can serve as the D1 stand-in by being registered under a distinct name representing the bridge intent. Alternatively, a minimal shim manifest can be constructed as follows:

| Field | Placeholder value |
|---|---|
| `name` | `"ha-bridge"` (provisional) |
| `version` | `"0.1.0"` |
| `required_capabilities` | `[{ capability: "agent.ha-bridge" }]` (provisional) |

**Notes:**
- The capability name `"agent.ha-bridge"` is provisional. The real bridge will define its capabilities based on what HA entity discovery and FERROS consent enforcement require.
- The bridge shim manifest must be registered in the agent registry before the D1 evidence session so that `ferros agent list` shows it.
- Bridge protocol details (pairing handshake, HA entity schema, HA wire format) are S7-owned and not constrained by this note.

### What a D1 stand-in registration looks like

```bash
# ferros agent register is not yet a CLI verb — registration happens at startup
# The bridge shim is either compiled in as a reference agent or registered via
# the JSON/RPC agent.register endpoint.
ferros agent list
# Expected output should include: ha-bridge  0.1.0  Registered
```

---

## Registry Contract (`AgentRegistry` trait)

The in-memory registry (`InMemoryAgentRegistry`) implements `AgentRegistry`:

| Method | Behavior |
|---|---|
| `register(manifest)` | Registers the agent; errors if the name is already taken (`RegistryError::AlreadyRegistered`) |
| `deregister(name)` | Removes the agent; returns the manifest if found, `None` if not |
| `list()` | Returns `Vec<AgentSummary>` (name + version for each registered agent) |
| `describe(name)` | Returns the full `AgentManifest` for the named agent, or `None` if not found |

**Uniqueness:** Two agents cannot share the same `AgentName`. Attempting to register a second agent with the same name returns `Err(RegistryError::AlreadyRegistered)`.

**Ordering:** `InMemoryAgentRegistry` uses `BTreeMap` internally, so `list()` returns agents sorted by name.

---

## Agent Lifecycle and Consent Gate

The consent gate applies at every `agent.run` attempt, not at registration time. An agent can be registered without its required capability being granted. The denial only fires when the agent is started:

```
ferros agent register ha-bridge   → AgentStatus::Registered (no capability check)
ferros agent run ha-bridge        → DenyByDefaultPolicy evaluates agent.ha-bridge grant
                                    → Denied(NoGrantsPresented) if no active grants
```

This means:
1. An operator can pre-populate the agent registry at startup without any grants configured.
2. The deny-log will only accumulate entries once `agent.run` is attempted.
3. For D1 consent-flow demonstration, pre-register the stand-in, then attempt `ferros agent run <stand-in>` without the matching grant to generate a denial entry.

---

## What This Note Does NOT Define

- HA entity schema or HA wire format (S7-owned)
- Bridge pairing handshake order (S7-owned)
- Additional agent capabilities beyond `agent.echo`, `agent.timer`, and the provisional `agent.ha-bridge`
- Agent shutdown/restart semantics (S4-owned beyond basic start/stop)

---

## Source Documents

- `crates/ferros-agents/src/manifest.rs` — `AgentManifest`, `CapabilityRequirement`, `AuthorizationDecision` types
- `crates/ferros-agents/src/reference.rs` — `EchoAgent`, `TimerAgent` constructors
- `crates/ferros-agents/src/registry.rs` — `AgentRegistry` trait, `InMemoryAgentRegistry`, `RegistryError`
- `streams/S3-agent-center/README.md` — S3 stream scope
- `streams/S7-hub/README.md` — HA bridge onramp mapping (ADR-023)
- `docs/research/S7-d1-bring-up-checklist.md` — D1 stand-in requirements
