# Operator Session Log

Append-only log for named operator sessions.

Use one row per session window or clearly grouped session packet.

| Session ID | Date | Operator | Human Test Backlog item | Instruction packet | Findings or evidence | Status | Coordinator decision | Follow-up |
|------------|------|----------|-------------------------|--------------------|----------------------|--------|----------------------|-----------|
| TEMPLATE | YYYY-MM-DD | name | HTB-000 | path/to/instruction.md | path/to/findings.md | Ready / In Session / Closed | Close / Hotfix / Agent Backlog / Hardware Queue / Escalate | optional queue or ADR note |