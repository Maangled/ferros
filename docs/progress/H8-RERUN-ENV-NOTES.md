# H8 Clean Rerun — Environmental Notes

**Date:** 2026-04-21  
**Harness:** `harnesses/ui-acceptance-harness.html`  
**Result:** PASS — 17/17 (verified in two independent browser sessions)  
**PR:** PR 8 — H8 clean rerun + environmental notes

---

## What changed to enable the clean pass

The previous "🔧 Needs clean rerun" flag was caused by two issues in the harness, not by any underlying product regression.

### Issue 1 — genesis-card onboarding lock (blocker)

Since the onboarding UX overhaul (robot-tour / discovery chain), the genesis card that contains the **Begin Setup →** button starts with the CSS class `onboarding-locked` (`display:none !important`). The class is removed only when the user engages with the featured-profiles gallery (clicking **✕ Skip**, **Build Own**, or **Use This Template**).

The harness was written before this mechanism existed. It would find `begin-btn` via `getElementById` but see `offsetParent === null` (hidden ancestor), then fall through to a weaker "completion path available" branch — never clicking the button, never creating the genesis seal.

**Fix (minimal):** After accepting the trade window, the harness now checks whether `genesis-card` is still locked and, if so, clicks `.gallery-dismiss-btn` via a normal DOM click. That fires `dismissFeaturedProfiles()` → `unlockGenesisCard()`, making `begin-btn` visible and accessible. No `contentWindow` JS calls — stays black-box.

### Issue 2 — completion-button click guard (logic error)

The original harness used `clickVisibleButtonByText` as the primary click mechanism (which clicks internally before returning). A guard `if (completeBtn.id !== 'begin-btn')` was then used to prevent a double-click when `begin-btn` was returned by the text search. This guard inadvertently suppressed the click when `begin-btn` was found **directly** via `getElementById` while visible — the correct, fast path introduced by the Issue 1 fix.

**Fix (minimal):** Replaced the one-size-fits-all search+guard pattern with an explicit two-branch approach:  
- If `begin-btn` or `stage1-btn` is found **and visible** → click it directly.  
- Otherwise → delegate to `clickVisibleButtonByText` (which clicks internally).

### Issue 3 — incorrect post-condition assertion

The harness checked `profile.identity.name` after stage-0 completion, but `identity.name` is set in `completeStage1()`, not `completeStage0()`. The stage-0 journey ends with the profile at `meta.stage = 1` — that is the correct post-condition.

**Fix (minimal):** Changed the assertion from `'Profile has identity.name'` to `'Profile advanced to stage 1'` (`stored.meta.stage === 1`). The total test count is unchanged (17).

---

## Stable rerun environment requirements

| Requirement | Notes |
|---|---|
| Fresh localStorage | Harness clears `localStorage` at journey start — no manual clearing needed |
| No stale profile from other tabs | A profile open in another tab would cause the page to skip stage 0 entirely; close all `personal-profile.html` tabs first |
| `file://` or same-origin HTTP | Harness and monolith must share the same origin so `localStorage` is shared; `file://` always works; HTTP requires `localhost` (not cross-host) |
| `#testmode` URL fragment | Already hardcoded in the harness; skips the 10-second boot animation, making the run predictable |
| No browser extensions that intercept `localStorage` | Extensions that intercept or block storage writes (cookie blockers, privacy badger, etc.) will cause the seal-creation wait to time out |

---

## Test groups and counts (17/17)

| Group | Tests | Result |
|---|---|---|
| Setup | 1 | PASS |
| A: Pre-Consent | 2 | PASS |
| B: Trade Window | 2 | PASS |
| C: Stage 0 | 4 | PASS |
| D: Persistence | 4 | PASS |
| E: FerrosCore | 1 | PASS |
| F: Seal Metadata | 3 | PASS |

---

## What was NOT changed

- No changes to `docs/personal-profile.html` (the monolith under test)
- No changes to `docs/assets/_core/ferros-core.js`
- No changes to `docs/contracts/manifest.json`
- No changes to any schema or fixture
- The H8 harness scope (DOM + localStorage only, no `contentWindow` JS calls) is preserved
