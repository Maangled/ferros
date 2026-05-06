# HTB-013 Host Touchscreen Pilot

Session ID: OPS-2026-05-05-HTB-013
Date: 2026-05-05
Operator: TBD at session start
Backlog item: HTB-013

Goal:

Exercise the current localhost shell on the USB-C touchscreen attached to this Linux host, record the connector and driver chain honestly, and verify that the touch posture remains usable without hover-only interactions.

Commands or routes to use:

1. Connect the touchscreen. Prefer the single-cable motherboard USB-C path if available; otherwise record the explicit fallback path.
2. From the repo root, start the shell with `cargo run -p ferros-node --bin ferros-node -- shell 4326`.
3. Open `http://127.0.0.1:4326/` on the touchscreen-driven display.
4. Record the host stack using the commands that work on this machine:
   - `lsusb`
   - `lsmod | rg -i 'nvidia|drm|hid|i2c|usb'`
   - `libinput list-devices`
   - `xrandr --listactivemonitors`
5. In the shell, use the touch anchors to move between Routes, Focus, Inspector, Tools, and Audit.
6. Run the current human-facing shell route check on the touchscreen:
   - set Local profile path to `/definitely/missing/profile.json`
   - open the `Runway` route
   - confirm the consent boundary and recovery posture remain visible and readable

Expected observation:

1. The shell remains usable by touch: the top touch anchors are tappable, primary actions do not depend on hover, and top/bottom context remains visible.
2. The operator can move across Routes, Focus, Inspector, Tools, and Audit without losing track of the active route.
3. The connector path and host display/input driver stack are recorded explicitly in the findings.
4. The findings note clearly states that the host driver chain is observed external infrastructure, not sealed FERROS-managed driver evidence.

Evidence capture path:

`docs/operator-sessions/findings/OPS-2026-05-05-HTB-013-host-touchscreen-pilot.md`

Rollback path:

1. Stop the shell server.
2. Disconnect the touchscreen if the session is complete.
3. If the touch path is unusable, record the failure and route it to the coordinator rather than improvising undocumented environment changes.

Stop criteria:

1. The touchscreen does not enumerate or remains unusable on this host.
2. The shell requires hover-only behavior or fine-pointer precision to proceed.
3. The connector or driver path cannot be identified well enough to write an honest evidence note.
4. The shell implies a stronger driver-integrity claim than the host session can support.

Immediate-task comments:

Record whether touch anchors, tap targets, and sticky context were sufficient, and note any spot where the shell still feels desktop-first.

Meta comments for coordinator:

Route broader comments about cardizing drivers, ACC audit assignment, or `Autosave` interaction capture to the coordinator as planning or ADR follow-up rather than folding them into a pass/fail call.