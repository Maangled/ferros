# FERROS — Agent Working Guide

> READ THIS BEFORE TOUCHING ANY CODE.
> This document exists because agents have caused recurring bugs by not understanding
> the architecture constraints. Every section exists because something went wrong.

## Quick Context

FERROS is a Rust-native OS (long-term vision). The active development surface is currently
a self-contained HTML prototype: `docs/personal-profile.html`. This single file contains
all CSS, all JS, and all HTML — no build tools, no frameworks, no external dependencies.
It must work when opened directly from disk via the `file://` protocol.

Read `README.md` for the full project vision.
Read `docs/architecture-overview.md` for the system architecture.
Read `docs/adr/` for binding architectural decisions.

---

## Agent Roles

Different agents are assigned different roles. Understand your role before starting.

### Role: Feature Agent
You are implementing a new feature in `docs/personal-profile.html`.
- Read the full ADR set before starting. The ADRs are binding.
- Follow the four-stage flow. Do not skip stages or bypass the consent system.
- `saveProfile()` is the ONLY place localStorage is written. Do not write to localStorage anywhere else.
- Test your mental model: does this work if `crypto.subtle` is unavailable? (It must.)
- Do not add external dependencies. Not even a CDN link. Not even for a font.

### Role: Bug Fix Agent
You are fixing a bug in `docs/personal-profile.html`.
- Check the Bug Log section of this document FIRST. The bug may already be documented with a known fix.
- Do not change the hash implementation from djb2 to `crypto.subtle` unconditionally. This will break `file://`.
- Do not restructure the stage flow. It is load-bearing.
- Do not remove the `saveProfile()` guard. It is the security boundary for all four session modes.

### Role: Documentation Agent
You are updating or creating markdown docs.
- Do not touch `docs/personal-profile.html`.
- Ensure any feature you document is actually implemented (check the PR history).
- ADRs are append-only — do not edit or rewrite the body of an existing ADR. If a decision changes, create a new ADR that supersedes the old one and add a "Superseded by ADR-XXX" note as an addendum at the bottom of the old file.
- The ADR numbering convention: `ADR-0001` for meta/process ADRs (e.g., `ADR-0001-start-new-do-not-fork.md`); `ADR-001`, `ADR-002`, … for technical decisions (e.g., `ADR-001-progression-lock-pattern.md`).

### Role: Architecture Agent
You are making a structural decision that affects multiple files or future development.
- Write an ADR first. Do not implement without an ADR.
- Get the ADR merged before the implementation PR.
- ADR decisions are binding for all subsequent agents.

---

## Critical Architecture Constraints

These are not preferences. Violating them causes bugs.

### 1. `file://` Protocol Compatibility (CRITICAL)
The prototype MUST work when opened directly from disk. This has several implications:

**`crypto.subtle` is NOT available on `file://` in most browsers.**
The Web Crypto API requires a secure context (`https://` or `localhost`). `file://` is not a secure context in Chrome/Edge/Firefox.

**The fix (already implemented, DO NOT REVERT):**
```javascript
async function hashData(str) {
  // Try crypto.subtle first (https:// or localhost)
  if (window.crypto && window.crypto.subtle) {
    try {
      const encoded = new TextEncoder().encode(str);
      const buffer = await window.crypto.subtle.digest('SHA-256', encoded);
      return Array.from(new Uint8Array(buffer))
        .map(b => b.toString(16).padStart(2, '0')).join('');
    } catch (e) { /* fall through to djb2 */ }
  }
  // djb2 fallback for file:// protocol
  let hash = 5381;
  for (let i = 0; i < str.length; i++) {
    hash = ((hash << 5) + hash) + str.charCodeAt(i);
    hash = hash & hash; // force 32-bit integer
  }
  return Math.abs(hash).toString(16).padStart(8, '0');
}
```

**DO NOT replace this with unconditional `crypto.subtle`.**
**DO NOT remove the djb2 fallback.**
**DO NOT make `hashData` synchronous** — callers use `await`.

### 2. Zero External Dependencies (CRITICAL)
No CDN links. No `<script src="...">`. No `import`. No `fetch`. No service workers.
If you add an external dependency, the file stops working offline and on `file://`.

### 3. `saveProfile()` Is the Only localStorage Write Point
Do not write to `localStorage` anywhere except inside `saveProfile()`.
`saveProfile()` has a guard at the top:
```javascript
function saveProfile() {
  if (recoveryMode || sessionDeclined) return; // SECURITY BOUNDARY — DO NOT REMOVE
  localStorage.setItem('ferros_profile', JSON.stringify(profile));
  localStorage.setItem('ferros_seal_chain', JSON.stringify(sealChain));
}
```
If you bypass this, alias mode and recovery mode will leak data to the host device's localStorage. This breaks the privacy guarantee.

### 4. Session Modes Are Mutually Exclusive
Four session modes exist (see [ADR-005](./adr/ADR-005-cross-device-identity-and-session-modes.md)):
- **Full Profile** — localStorage, normal operation
- **Session Mode** — no identity, no storage
- **Alias Mode** — sessionStorage only, pseudonymous
- **Recovery Mode** — JS module variables only, no storage at all

They cannot be active simultaneously. If you add a feature that writes to localStorage, guard it with the existing mode checks. If you add a new mode, update ADR-005.

### 5. Stage Flow Is Load-Bearing
The four stages (0→1→2→3) are not optional. Do not add shortcuts that bypass:
- The Trade Window consent dialog (Stage 0 entry)
- The character creation form (Stage 1)
- The first protocol seal (Stage 2)
Users who decline the Trade Window enter session mode — they still see the UI, they just don't persist.

### 6. Seal Chain Integrity
The `sealChain` array must never be modified outside of `addSeal()`. Do not push directly to `sealChain`. Do not sort or reorder it. The chain order is its integrity.

---

## Bug Log

Documented bugs, root causes, and their fixes. Check here before fixing anything — the bug may have been previously fixed and reverted.

### BUG-001: crypto.subtle Fails on file:// Protocol
**Symptom:** Stage 0→1 transition hangs or throws `TypeError: Cannot read properties of undefined (reading 'digest')` or similar. Profile creation fails silently.
**Root cause:** `crypto.subtle` requires a secure context. `file://` is not a secure context.
**Fixed in:** PR #6 (surgical bug fix batch)
**Fix:** djb2 fallback in `hashData()`. See constraint #1 above.
**DO NOT REVERT.** This fix has been accidentally broken twice by agents refactoring the hash function.

### BUG-002: Avatar Selection Parameter Mismatch
**Symptom:** Selected avatar does not appear on the dashboard after character creation. Default avatar shown instead.
**Root cause:** The `selectAvatar(el)` function was called with the element as argument but internally read `event.target` — mixing two calling conventions.
**Fixed in:** PR #6
**Fix:** `selectAvatar(el)` now uses the `el` parameter directly. Do not change the function signature without updating all call sites.

### BUG-003: Stage Visibility Toggling — All Stages Visible Simultaneously
**Symptom:** All four stages are visible at once on page load instead of only Stage 0.
**Root cause:** `showStage(n)` was toggling classes but the initial CSS state left all stages visible.
**Fixed in:** PR #6
**Fix:** CSS hides all `.stage` elements by default. `showStage(n)` adds `.stage-visible` to the target and removes it from all others. Do not add `display:block` or `visibility:visible` to any stage element in CSS without going through `showStage()`.

### BUG-004: Locked Level Cards Clickable via Keyboard
**Symptom:** Tab-focusing a locked level card and pressing Enter selected it, bypassing the lock.
**Root cause:** Click handler only, no keyboard guard.
**Fixed in:** PR #9
**Fix:** `selectAssistLevel(n)` checks `isLevelLocked(n)` at the top and returns early. Lock check must happen inside the function, not just in the click handler.

### BUG-005: Recovery Mode Writing to localStorage
**Symptom (theoretical, prevent it):** Profile data appearing in localStorage on a foreign device after a recovery session.
**Root cause:** Any code path that calls `saveProfile()` during recovery mode.
**Prevention:** The `if (recoveryMode || sessionDeclined) return;` guard at the top of `saveProfile()`. Do not add any code between `function saveProfile() {` and this guard line.

### BUG-006: Alias Session Surviving Tab Close Then Reopening localStorage
**Symptom (theoretical, prevent it):** Alias mode state persisting after tab close, allowing it to bleed into a full-profile session on the same device.
**Root cause:** If alias state were stored in localStorage instead of sessionStorage, it would persist.
**Prevention:** `ferros_alias_session` MUST use `sessionStorage`, never `localStorage`. Do not move it.

### BUG-007: Begin Setup Button Had Wrong Glow Animation
**Symptom:** "Begin Setup →" button in the genesis card was pulsing with `beginPulse` glow animation, making it appear to be the primary CTA when it is a secondary path.
**Root cause:** PR #23 added `beginPulse` to `.begin-btn` in CSS (and PR A re-applied it in `_postBootReveal()`). The glow belongs ONLY on the "Get Started" button inside the robot's speech bubble dialog, after ~60 seconds of user inactivity.
**Fixed in:** PR D (ADR-006 implementation)
**Fix:** Removed `animation: beginPulse …` from the `.begin-btn` CSS rule. The Begin Setup button is now a static button with no glow. The adaptive glow (`getStartedPulse`) is applied only to `#get-started-btn` inside the robot dialog after a 60-second inactivity timeout.
**Button differentiation (per ADR-006):**
- **"Get Started"** → inside robot speech bubble, opens Trade Window, glows after ~60s inactivity
- **"Begin Setup →"** → in "Welcome to Your Progression System" box, proceeds to Stage 1, **no glow**
- **"🍴 Fork this Profile"** → on featured profile cards, starts alias mode, **no glow**

### BUG-008: Achievement Tooltips Dismissed Before Clicking Buttons Inside
**Symptom:** Flowchart/diagram popups on achievement cards dismissed when moving cursor from the card to the popup button inside it, because the `:hover` state was on the card and moving to the popup broke the hover chain.
**Root cause:** CSS `:hover`-only tooltip pattern: as soon as the pointer left the `.genesis-ach-card` boundary (even to enter the `.ach-hover-diagram` child), the tooltip collapsed.
**Fixed in:** PR A (click-to-toggle pattern) — click card to toggle `.ach-active` class; click outside to dismiss.
**Note:** The click-to-toggle pattern is already implemented. Do not revert to `:hover`-only.

### BUG-009: Scroll Gating Clipped Begin Button Below Fold
**Symptom:** On viewports shorter than the total hero content height, the "Begin Setup →" button was unreachable because `body.scroll-gated { height: 100vh; overflow: hidden }` prevented scrolling past the fold.
**Root cause:** PR #24's scroll gating used `overflow: hidden` on `body` without allowing internal scroll on `#stage-0`.
**Fixed in:** PR #26 — `body.scroll-gated #stage-0 { max-height: 100vh; overflow-y: auto }` so the stage can scroll internally while the body is gated.
**Do not remove** the `#stage-0` internal scroll rule when modifying scroll gating CSS.

---

## What Not To Do (Anti-Patterns)

These have been attempted or will be attempted by future agents. Avoid them.

| Anti-pattern | Why it breaks things |
|---|---|
| Replace `djb2` with `crypto.subtle` only | Breaks `file://` — BUG-001 |
| Add a `<script src="...">` CDN link | Breaks offline / `file://` usage |
| Write to `localStorage` outside `saveProfile()` | Breaks session mode privacy |
| Make `hashData()` synchronous | Breaks all `await addSeal(...)` call sites |
| Push directly to `sealChain[]` | Breaks chain integrity |
| Skip the Trade Window on first visit | Breaks consent model |
| Use `fetch()` for anything | Not available on `file://` |
| Use ES modules (`import/export`) | Not supported on `file://` without a server |
| Add `.gitignore`-ing the HTML file | The HTML file IS the product |
| Rewrite ADRs in place | ADRs are append-only — supersede with a new ADR, add a note to the old one |

---

## File Map

| File | What it is | Edit? |
|---|---|---|
| `docs/personal-profile.html` | Primary prototype — single HTML, all CSS+JS embedded | ✅ Main target |
| `docs/architecture-overview.md` | Prose architecture summary — OS + prototype layer | ✅ Keep current |
| `docs/adr/ADR-001-*.md` | Progression lock / seal chain | ✅ Read before touching hashing |
| `docs/adr/ADR-002-*.md` | Smart contract boundaries | ✅ Read before any chain/ledger work |
| `docs/adr/ADR-003-*.md` | Alias system | ✅ Read before alias/recovery work |
| `docs/adr/ADR-004-*.md` | Template profile spec / stub schema | ✅ Read before gallery/template work |
| `docs/adr/ADR-005-*.md` | Cross-device identity & session modes | ✅ Read before any session/storage work |
| `docs/AGENT_GUIDE.md` | This file | ✅ Update when new bugs found |
| `ferros-blueprint.html` | Founding spec (Phase 0 conformance test) | ⚠️ Read-only — do not modify |
| `docs/deployment-roadmap.html` | Phase roadmap visualization | ⚠️ Only update with major phase changes |
| `docs/ferros-showcase.html` | Public landing page | ⚠️ Separate concern from profile |
| `docs/agent-command-center.html` | Agent task UI | ⚠️ Separate concern |
| `docs/home-hud-dashboard.html` | Smart home HUD prototype | ⚠️ Separate concern |
| `docs/schedule-ledger.html` | Schedule/habit ledger | ⚠️ Separate concern |
| `README.md` | Repo overview | ✅ Update when major features land |

---

## PR History (What Has Been Merged)

Understanding this prevents re-implementing things or reverting fixes.

| PR | Title | Key changes |
|---|---|---|
| #3 | Add personal progression profile and deployment roadmap HTML docs | Initial `personal-profile.html` and `deployment-roadmap.html` |
| #4 | Implement progression-lock pattern, seal chain, and smart contract boundary ADRs | ADR-001, ADR-002 |
| #5 | [WIP] Fix Begin Setup button — CLOSED, NOT MERGED | Timed out — no code landed |
| #6 | Fix broken stage transitions | BUG-001 (crypto.subtle), BUG-002 (avatar), BUG-003 (stage visibility) |
| #7 | MMO Trade Window consent dialog + session mode | Trade Window UI, session mode, `ferros_trade_accepted` |
| #8 | Genesis hype page | Hero banner, locked achievement cards, feature pills |
| #9 | Lock Levels 2–4 with progression-gated unlocks | BUG-004 (keyboard bypass), `applyLevelLocks()`, `isLevelLocked()` |
| #10 | ADR-003 (Alias System) + ADR-004 (Template Profile Specification) | Architecture docs |
| #11 | Resume banner + session mode integration | Returning user detection, `#welcome-back-banner`, "Clear data" button |
| #12 | Export/Import Profile — JSON Portability Panel | `exportProfile()`, `importProfile()`, `#ps-portability`, coming-soon sync stub |
| #13 | Alias Mode | `ferros_alias_session` in sessionStorage, alias dashboard, `.ferros-log` export |
| #14 | Profile Gallery — Genesis page browse & preview modal | `TEMPLATE_PROFILES` constant, gallery modal, "Use as Alias" button |
| #15 | Template Schedules in Character Creation | Schedule picker in Stage 1, schedule seeding in Stage 2 |
| #16 | Alias Log Claim Flow | Import `.ferros-log`, verify seal chain, merge into real profile |
| #17 | Key Recovery / Cross-Device Login | `recoveryMode` flag, recovery panel on Genesis, recovery session export |
| #18 | ADR-005 Cross-Device Identity & Session Modes | Architecture doc for all four session modes |

---

## Upcoming Work (Next PRs)

These are known upcoming features. Do not implement them without a task assignment.

- **Bug sweep** — systematic review of the full `personal-profile.html` for UI/UX bugs, broken flows, edge cases
- **ADR-006** — Personal alias keys (account-recovery-style user-generated keys, not template codes)
- **ADR-007** — Ledger anchoring specification (when and how root hashes go on-chain)
- **Template expansion** — More professions/archetypes in the gallery (beyond the initial 8)
- **Achievements audit** — Ensure all defined achievements have trigger logic in the code
- **Level unlock thresholds** — Currently hardcoded; consider making them configurable

---

## Terminology Glossary

| Term | Meaning |
|---|---|
| **Seal** | A hash of task data + previous seal + timestamp + nonce. The unit of tamper-evident progression. |
| **Seal chain** | The ordered array of seals representing a user's complete history. |
| **Genesis hash** | `seal_0` — the first seal, seeded from the user's identity data. The cryptographic root of the profile. |
| **Protocol** | A FERROS task unit — a named set of actions a user commits to completing. |
| **Stream** | A, B, or C — the focus category for an XP attribute (A=Deep Work/Focus, B=Social/Service, C=Physical/Care). |
| **Assist level** | 1–4, chosen during character creation. Controls dashboard complexity (Level 1 = simplified, Level 4 = full director mode). |
| **Trade Window** | The MMO-style consent dialog that appears before any localStorage writes. |
| **Session mode** | User declined the Trade Window. No storage. Persistent amber banner. |
| **Alias mode** | User borrowed a template profile identity. SessionStorage only. Pseudonymous. |
| **Recovery mode** | User loaded their own profile backup on a foreign device. JS vars only. No storage. |
| **.ferros-log** | Portable JSON artifact exported from alias or recovery sessions. Claimable on home device. |
| **djb2** | The hash fallback function used when `crypto.subtle` is unavailable (file:// protocol). |
| **ledgerPointer** | Field in template stubs. `null` until the FERROS distributed ledger exists. Migration hook. |
| **ADR** | Architecture Decision Record. Binding. Append-only. |
