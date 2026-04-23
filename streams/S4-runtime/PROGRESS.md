# S4 Runtime / OS Core — Progress

Reverse-chronological. Append a dated entry at the top per session.

---

## 2026-04-23 — First ferros-core capability/policy slice landed

- Added `Capability`, `CapabilityRequest`, `CapabilityGrantView`, `PolicyDecision`, and `DenyByDefaultPolicy` to `crates/ferros-core/`.
- Kept the grant boundary FERROS-owned by aligning the adapter trait to S2's current minimal `CapabilityGrant { profile_id, capability }` shape without introducing a crate cycle.
- Added 10 focused grant/deny and validation tests to prove deny-by-default behavior before any runtime or message-bus work.

## 2026-04-21 — Stream scaffolded

- Stream directory and planning documents created.
- Blocked on G1 (S1 Foundation).
- Stub capability types can be sketched before G2; will be replaced by S2 types at G2.
- Policy engine design can begin independently.
