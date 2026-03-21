# ADR-006: Level Zero — Adaptive Onboarding & Robot Assistant Architecture

## Status
Accepted

## Date
2026-03-21

## Context

FERROS uses a 4-level assist system (Levels 1–4) chosen during character creation that controls dashboard complexity. However, the initial onboarding experience — the very first page a new user sees — has no assisted mode. Users with difficulty paying attention (whether due to ADHD, age, anxiety, or any other condition) can get lost or overwhelmed before they even create a profile.

This is analogous to video game design: if a player keeps dying at a section, good game designers make that section easier or provide hints so the player keeps playing. If the difficulty wall is not adjusted, the player stops playing entirely. FERROS needs the same adaptive approach for its onboarding. The 4-level assist system (defined in prior ADRs) addresses this problem once a user has a profile — but nothing addresses the moment before the profile exists.

The trigger for this ADR is the completion of PRs #22–#27, which collectively implemented the Genesis onboarding page (`personal-profile.html`). During user testing, the "Begin Setup" button was unreachable due to scroll gating, the achievement hover tooltips dismissed before users could click them, and new users had no guided orientation before being asked to make decisions. This ADR captures the architectural response to those usability failures.

## Decision

### Level 0: The Robot Helper

We introduce **Level 0** as the default entry state for ALL new users. Level 0 is a guided, interactive onboarding experience delivered by a CSS-animated robot companion (the "FERROS Assistant").

- Every new user starts at Level 0 — there is no opt-in requirement
- Level 0 is active only during the Genesis/Stage 0 onboarding flow
- The robot acts as "your computer" — a friendly guide that walks you through the system
- Users naturally graduate out of Level 0 when they complete onboarding and enter their dashboard (Levels 1–4 selection happens during Stage 1 character creation)
- Users can exit or dismiss the robot helper at any time — we never force someone to interact with an AI when they can control their own life
- The existing Levels 1–4 (from PR #9) remain unchanged and are selected during Stage 1 character creation

This maps the full assist continuum as follows:

| Level | Mode | When Active | Robot |
|-------|------|-------------|-------|
| 0 | Guided onboarding | Genesis/Stage 0 only — all new users | Always present |
| 1 | Simplified dashboard | Post-setup, chosen at character creation | Available on request |
| 2 | Standard dashboard | Post-setup, chosen at character creation | For complex tasks |
| 3 | Advanced dashboard | Post-setup, chosen at character creation | For automation |
| 4 | Director mode | Post-setup, chosen at character creation | Equal collaborator |

### The Boot Sequence

When a new user opens `personal-profile.html` with no existing profile in `localStorage`, the page runs the following cinematic boot sequence before displaying any interactive content:

1. **Black screen** — page starts dark, no content visible
2. **FERROS title sweep** — "FERROS" appears centered with a 3D light/shine CSS animation sweeping across the letters from left to right
3. **Subtitle typewriter** — "Personal Progression System — Genesis Block" types out underneath with a smaller version of the shine effect active while each character is written
4. **Shrink + reveal** — title and subtitle animate to their normal banner size and position; the banner bar drops down from above; the hero section ("Level up your life. Track your growth. Earn real skills.") grows into view from opacity/scale zero
5. **Robot entrance** — the CSS android companion climbs out of a horizontal line at the bottom of the hero area, casts a glow shadow on the imaginary floor, and pushes the hero content box smaller as if making physical room

Returning users (profile exists in `localStorage`) skip the boot sequence entirely and see the page in its normal state.

A **Skip** link is available in a fixed corner position for users who do not want to watch the sequence. The Skip link appears after the first 2 seconds so it does not flash on screen before the animation even starts.

All boot sequence timing is achieved with CSS `@keyframes` and sequential `animation-delay` values. Minimal JS orchestration handles the `localStorage` check to determine new vs. returning user status and removes the boot-sequence overlay class on completion.

### Interactive Onboarding Flow (Phase 1–4)

After the boot sequence completes, the robot begins guiding the user through the first page in four phases:

#### Phase 1 — Robot Teaches the System

The robot speech bubble reads:
> "Hover over each of the icons to see more information"

The three hero feature pills each gain hover explainer tooltips:

| Pill | Tooltip Content |
|------|-----------------|
| **Cryptographic Seals** | Explains that every action you log is sealed with a cryptographic hash, creating a tamper-evident chain of your progress |
| **Achievement System** | Explains that completing milestones earns Seal Points (SP) and unlocks new stages of your profile |
| **100% Local / No Servers** | Explains that nothing leaves your device — your data lives in your browser's localStorage, not a cloud |

The robot animates to hover over each of the three pills in sequence, briefly triggering each tooltip before returning to its home position. This teaches the interaction pattern before requiring the user to do it themselves.

#### Phase 2 — Get Started Call-to-Action

When the robot returns to its home position, the speech bubble updates to:
> "FERROS is free, for you, for life. Click this button to get started."

A **"Get Started"** button appears **inside the robot's speech bubble dialog**. This is the primary call-to-action for new users.

After approximately 60 seconds of user inactivity (no click, no scroll), the Get Started button begins glowing progressively brighter. This adaptive hint is borrowed directly from video game design: if the player has not found the path forward, make the path more visible. The glow does not start immediately — only after a delay that accounts for users who are reading the tooltips or taking their time. The glow is never forced; it is a suggestion.

#### Phase 3 — Trade Window + First Achievement

When the user clicks Get Started:

1. The MMO Trade Window consent dialog appears (the existing Stage 0 flow — this is never bypassed)
2. The user accepts the Trade Window
3. A 🚀 rocket emoji animation fires upward from the button position, travels in an arc, and lands on the "Genesis Pioneer" achievement card
4. The Achievements section appears and/or scrolls into view
5. The Genesis Pioneer achievement unlocks with a gold shimmer animation and screen-wide fireworks
6. "+25 SP" floats up from the achievement card
7. The robot slides out the right side of the hero section and re-enters from the left side of the Achievements section, animated with excitement
8. The robot speech bubble reads: "Congrats! You got your first achievement!"

All achievements **except "Discover Profiles"** are shown in a greyed-out locked state with a lock icon overlay. Locked achievements are still hoverable — users can read the unlock requirements for each one without being able to claim them. This teaches the progression system without overwhelming the user.

#### Phase 4 — Second Achievement / Branching Paths

After the first achievement, the user has two paths:

- **Browse Profiles** — the robot follows the user as they scroll or navigate to the Featured Profiles section
- **Start from Scratch** — clicking this button scrolls the user down to the "Welcome to Your Progression System" box

The second achievement or seal in the chain — **"Discover Profiles"** — is triggered by whichever action the user takes first: forking a profile card (🍴 Fork this Profile) or clicking the Begin Setup → button in the Welcome box. The specific SP value and unlock criteria for Discover Profiles are defined in the achievement implementation; this ADR binds the trigger conditions only. The robot moves with the user as they scroll through this section.

### Button Differentiation

Three distinct buttons exist on the Genesis page and must not be confused:

| Button | Location | Purpose | Glow Behavior |
|--------|----------|---------|---------------|
| **Get Started** | Inside robot's speech bubble | Primary CTA — opens Trade Window | Glows progressively after ~60s inactivity |
| **Begin Setup →** | Welcome to Your Progression System box | Start-from-scratch path | No glow — static button |
| **🍴 Fork this Profile** | Featured profile cards | Fork a template profile | No glow — static button |

The "Begin Setup →" button must **not** glow. It is a secondary path, not the primary call-to-action. Any existing pulse or glow animation on this button (added by earlier PRs) must be removed.

### The Adaptive Difficulty Philosophy

Level 0 onboarding is explicitly designed for people who have difficulty paying attention, whether due to:
- ADHD or other attention disorders
- Age (young children or elderly users)
- Anxiety, depression, or other mental health conditions
- Simply being new to systems like this

The core principle, borrowed from video game design:
> If the player keeps dying, make it easier for them to get past that point so they keep playing.

In FERROS terms:
- The robot provides visual guidance and reduces cognitive load before asking the user to make decisions
- Hover explainers teach the system concepts (cryptographic seals, achievement system, local storage model) before the user commits to anything
- The Get Started button glows brighter over time if the user has not acted — adaptive difficulty hint, not a forced interaction
- Achievements provide immediate positive feedback (dopamine loop) at each step
- The fireworks and celebration animations reinforce that progress is being made — the user is winning
- Locked-but-hoverable achievements show the path forward without overwhelming with choice

Level 0 is not limited to onboarding. The robot can reappear at any future difficulty zone or stopping point in the application — any place where a meaningful fraction of users stop making progress. When that happens in the future, a new ADR should capture the specific intervention.

### Future Vision: AI Agent Assistant

The robot helper in Level 0 is a static CSS animation today. In the future, the robot becomes an **AI agent** powered by the Agent Command Center (see `docs/agent-command-center.html`):

- Users can watch the robot "run around their profile" and fix things in real time
- Users can direct the robot as tasks occur to them
- The robot can surface conflicts that the human may not foresee
- The robot works off the alpha tester profile for initial development
- The robot's actions are logged as achievements and seals in the chain
- Users can dismiss or disable the robot at any time — autonomy is always preserved

The progression path for the robot companion:

| Level | Robot Role |
|-------|-----------|
| 0 | Guides you through everything — hands-on onboarding |
| 1 | Available on request — simplified dashboard, robot for orientation |
| 2 | Standard dashboard — robot for complex or unfamiliar tasks |
| 3 | Advanced dashboard — robot for automation and batch operations |
| 4 | Director mode — robot as an equal collaborator, taking direction |

This is a future ADR item. The current implementation commits only to the CSS-animated robot with static speech bubbles and the onboarding phase orchestration described above.

### Implementation Constraints

All existing architecture constraints from `docs/AGENT_GUIDE.md` and prior ADRs apply:

- `file://` protocol compatibility — no external dependencies, no CDN links
- All animations are CSS `@keyframes` with minimal JS orchestration for sequencing
- `saveProfile()` remains the only `localStorage` write point — the boot sequence and robot add no new write paths
- The boot sequence and robot are a **layer on top of** Stage 0, not a replacement — the four-stage flow is load-bearing and must not be restructured
- The Trade Window consent flow is never bypassed — "Get Started" opens the Trade Window, not the profile setup directly
- All four session modes (Full, Session, Alias, Recovery) are unaffected by Level 0 — the robot does not appear in alias or recovery mode
- The `localStorage` check for new vs. returning user determines boot sequence playback — the check runs before any rendering and uses the same `ferros_profile` key as the rest of the system

## Consequences

### Decisions Made

**Decision: Level 0 is always-on for new users, never opt-in.**
- Requiring opt-in defeats the purpose — users who need guidance most are the least likely to seek it out.
- Users can always skip or dismiss — the decision is about the default, not a forced interaction.
- Rejected: showing Level 0 only when the user explicitly asks for help.

**Decision: Get Started button lives inside the robot speech bubble, not as a standalone page element.**
- Collocating the CTA with the robot creates a single focal point. The user does not need to search the page.
- The robot's speech bubble is already the attention anchor — putting the button there is the lowest-friction path.
- Rejected: a separate large CTA button below the hero text. This competes visually with the robot.

**Decision: Begin Setup → button does not glow.**
- The Begin Setup → button is a secondary path (start from scratch). Glowing it implies it is the primary action, which it is not.
- Glowing Get Started (after 60s inactivity) is the adaptive difficulty hint. A second glowing button dilutes the signal.
- Any existing glow/pulse on Begin Setup → from earlier PRs must be removed.

**Decision: Achievement hover tooltips must persist through pointer movement.**
- The bug where hover tooltips dismissed before the user could click buttons inside them is a blocking usability failure.
- Fix: switch from CSS `:hover`-only to a click-to-toggle or hover-bridge pattern (sufficient padding/nesting so the pointer does not leave the hover target when moving to the button).
- Rejected: removing buttons from inside tooltips. The buttons are load-bearing (Discover Profiles unlock flow).

**Decision: Locked achievements are visible but greyed-out, not hidden.**
- Hiding locked achievements removes the sense of progression scope — users do not know what is ahead.
- Showing them greyed with a lock icon and hoverable tooltips teaches the system while maintaining achievement integrity.
- Rejected: hiding locked achievements entirely until unlocked.

**Decision: Genesis Pioneer unlocks on Trade Window acceptance, not on page load.**
- Achievement unlocks must be tied to user actions, not automatic page events.
- Genesis Pioneer is the reward for consenting to the Trade Window — it is earned, not given.
- Rejected: auto-unlocking Genesis Pioneer on first load. This trains users to expect achievements without action.

### Positive
- All new users receive guided orientation — the hardest part of any system is starting
- The robot pattern is extensible to future difficulty zones without redesigning the system
- Adaptive difficulty (glowing button after 60s) is non-intrusive — it appears only when needed
- Achievements tied to actions reinforce the core loop immediately
- CSS-only animations maintain `file://` compatibility
- The four session modes are unaffected — Level 0 composes cleanly with existing architecture

### Negative
- The boot sequence adds approximately 8–10 seconds to the first-load experience for new users (mitigated by the Skip link)
- Returning users see a momentary flash before the `localStorage` check completes (mitigated by running the check synchronously before any render)
- The robot speech bubble text is static — it cannot respond to user questions (mitigated by the future AI agent path)
- Maintaining distinct visual identities for three buttons (Get Started, Begin Setup →, Fork this Profile) requires ongoing design discipline

### Out of Scope (Future ADRs)
- **AI agent integration** — the robot as a live AI assistant powered by the Agent Command Center
- **Level 0 at post-onboarding difficulty zones** — when and how the robot reappears after Genesis
- **Personalized adaptive difficulty** — using session history to calibrate the glow timer and hint frequency per user
- **Robot action logging as achievements** — when AI agent actions write to the seal chain

## Related
- [ADR-001: Progression-Lock Pattern](./ADR-001-progression-lock-pattern.md) — The four progression stages; Level 0 sits before Stage 0 and does not alter the stage structure
- [ADR-002: Smart Contract Boundaries](./ADR-002-smart-contract-boundaries.md) — Genesis Pioneer and chain achievements are seal-chain events consistent with ADR-002's on-chain anchoring model
- [ADR-005: Cross-Device Identity & Session Modes](./ADR-005-cross-device-identity-and-session-modes.md) — Level 0 is active only in Full Profile mode (new user); it does not appear in alias, recovery, or session mode
- PR #22: Acronym fix, cloud sync placeholder, architecture TODO comments
- PR #23: CSS android companion, speech bubbles, Genesis Pioneer achievement, achievement hover tooltips
- PR #24: Featured profiles revamp, Fork this Profile CTA, scroll gating (scroll gating bug documented here)
- PR #25 / PR A: Boot sequence, Begin button fix, achievement hover fix
- PR #26 / PR B: Rocket animation, screen-wide fireworks, enhanced unlock sequence
