# ADR-0001 — Start New: Do Not Fork Athena OS or AEGIS-OS

**Status:** Accepted  
**Date:** 2026  
**Context:** FERROS founding architecture decision

---

## Context

Two existing open-source projects were evaluated as potential bases for FERROS:

1. **Athena OS / aegis installer** — A NixOS-based security-focused distribution with
   excellent declarative state management, reproducible builds, atomic upgrades, and an
   operator-friendly installation flow.

2. **Galactic-Code-Developers AEGIS-OS** — A mobile-first OS targeting ARMv9 with
   owner-controlled verified boot, hardware-rooted isolation, tight trust boundaries, and
   a minimal early attack surface. Intentionally network-free.

The question: should FERROS fork one of these as its main trunk?

---

## Decision

**Start new. Do not fork either Athena OS or AEGIS-OS as the main trunk.**

Borrow architectural ideas selectively and credit them explicitly.

---

## Rationale

### Against forking Athena OS
- Athena OS is a Linux distribution. FERROS is explicitly designed to replace Linux.
  Building on a Linux base would inherit the very C-codebase legacy and unsafe assumptions
  that FERROS is architected to avoid.
- The declarative state management and atomic upgrade ideas from Athena OS are worth
  borrowing, but they do not require the Linux kernel or the NixOS toolchain.

### Against forking AEGIS-OS
- AEGIS-OS is intentionally scoped to mobile/ARMv9 and has no networking stack. FERROS
  targets x86_64/QEMU as a primary development and validation environment and requires
  networking from Phase 3 onward.
- The ARMv9-specific isolation mechanisms and mobile hardware assumptions would require
  significant rework that is equivalent in effort to building the relevant subsystems
  from scratch in Rust.

### For starting new
- FERROS is written in Rust from the ground up. Neither Athena OS nor AEGIS-OS provides
  a Rust-native kernel or bootloader codebase.
- Starting new allows FERROS to own its entire dependency graph, enforce its own
  capability model from layer zero, and make architectural decisions without being
  constrained by the accumulated decisions of an inherited codebase.
- The founding specification (ferros-blueprint.html) defines the exact architecture.
  Forking introduces a gap between the inherited design and the specified design that
  must be closed incrementally — a source of technical debt from day one.

---

## Consequences

- FERROS borrows **ideas** from Athena OS (declarative state, reproducible builds,
  atomic upgrades) and from AEGIS-OS (verified boot, hardware-rooted isolation, minimal
  trust boundaries), but implements them independently in Rust.
- Both projects are credited as design inspirations in the founding specification and
  in this ADR.
- The FERROS team is responsible for the full implementation of all subsystems with no
  inherited upstream to pull security patches from. This is a deliberate trade-off in
  favour of architectural coherence and memory safety.

---

## Alternatives Considered

| Alternative | Reason Rejected |
|-------------|-----------------|
| Fork Athena OS | Linux-based; inherits C legacy; contradicts FERROS mission |
| Fork AEGIS-OS | ARMv9/mobile-only; no networking; not Rust-native kernel |
| Use seL4 as kernel base | seL4 is C + formal proofs; Rust-native kernel preferred |
| Use Redox OS as base | Closest match, but architecture diverges significantly from FERROS spec; evaluated separately |
