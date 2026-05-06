# Operator Session Log

Append-only log for named operator sessions.

Use one row per session window or clearly grouped session packet.

| Session ID | Date | Operator | Human Test Backlog item | Instruction packet | Findings or evidence | Status | Coordinator decision | Follow-up |
|------------|------|----------|-------------------------|--------------------|----------------------|--------|----------------------|-----------|
| OPS-2026-05-05-H1 | 2026-05-05 | TBD at issue time | HTB-001, HTB-002, HTB-008, HTB-009, HTB-011, HTB-012 | docs/operator-sessions/OPS-2026-05-05-H1-current-shell-read-surfaces.md | docs/operator-sessions/findings/OPS-2026-05-05-H1-current-shell-read-surfaces.md | Ready | Issue when operator is available | Covers the current shell read-surface wave after the new route and badge proof landed |
| OPS-2026-05-05-H2 | 2026-05-05 | TBD at issue time | HTB-003 | docs/operator-sessions/OPS-2026-05-05-H2-profile-round-trip.md | docs/operator-sessions/findings/OPS-2026-05-05-H2-profile-round-trip.md | Ready | Issue when operator is available | Use after the bounded profile route proof; grant and revoke remain closed |
| OPS-2026-05-05-H3 | 2026-05-05 | TBD at issue time | HTB-010 | docs/operator-sessions/OPS-2026-05-05-H3-home-hub-read.md | docs/operator-sessions/findings/OPS-2026-05-05-H3-home-hub-read.md | Ready | Issue when operator is available | Uses the new Home-Hub route as a read-only topology and lineage surface |
| OPS-2026-05-05-H4 | 2026-05-05 | TBD at issue time | HTB-005, HTB-006 | docs/operator-sessions/OPS-2026-05-05-H4-preview-surfaces.md | docs/operator-sessions/findings/OPS-2026-05-05-H4-preview-surfaces.md | Ready | Issue when operator is available | Uses the new Forge and Arena preview-only routes without implying authority or promotion |
| OPS-2026-05-05-HTB-004 | 2026-05-05 | TBD at issue time | HTB-004 | docs/operator-sessions/OPS-2026-05-05-HTB-004-onramp-review.md | docs/operator-sessions/findings/OPS-2026-05-05-HTB-004-onramp-review.md | Ready | Issue when operator is available | Use the current runway boundary/recovery slice as the first human-test checkpoint |
| OPS-2026-05-05-HTB-013 | 2026-05-05 | TBD at issue time | HTB-013 | docs/operator-sessions/OPS-2026-05-05-HTB-013-host-touchscreen-pilot.md | docs/operator-sessions/findings/OPS-2026-05-05-HTB-013-host-touchscreen-pilot.md | Waiting On Hardware Queue | Issue after the touchscreen is connected and host capture tools are available | Use after HTB-004 or alongside it as the first hardware-backed touch pilot |
| TEMPLATE | YYYY-MM-DD | name | HTB-000 | path/to/instruction.md | path/to/findings.md | Ready / In Session / Closed | Close / Hotfix / Agent Backlog / Hardware Queue / Escalate | optional queue or ADR note |