# S8 Docs / Governance / Contributor Onboarding — Contracts

S8 produces documentation contracts — shared vocabulary, templates, and governance conventions. These are not code contracts, but they are binding conventions for all streams.

---

## Conventions owned by S8

| Convention | Location | Status |
|-----------|----------|--------|
| ADR process and template | `docs/adr/ADR-TEMPLATE.md` | ✅ Created |
| Stream planning format (README/PROGRESS/BACKLOG/CONTRACTS) | `streams/SN-*/` | ✅ Created |
| Gate document format | `docs/gates/G*.md` | ✅ Created |
| Status dashboard format | `STATUS.md` | ✅ Created |
| Contributor routing | `CONTRIBUTING.md` | ✅ Created |
| Contracts overview | `docs/contracts/CONTRACTS-OVERVIEW.md` | ✅ Created |

---

## Dependencies

None. S8 runs independently.

---

## Notes

S8 does not gate other streams. It is a continuous background stream that keeps documentation synchronized with reality. Any stream may update its own PROGRESS.md, BACKLOG.md, and CONTRACTS.md directly — S8 coordinates the format, not the content.
