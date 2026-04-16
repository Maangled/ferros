# Legacy Repo Reference

These predecessor projects informed FERROS's design. Patterns are ported per
[ADR-013](../adr/ADR-013-legacy-integration-strategy.md), aligned with the wave
structure.

| Repo | Description | Key Patterns for FERROS |
|------|-------------|------------------------|
| [botgen-rust](https://github.com/Maangled/botgen-rust) | Modular Discord bot platform (Rust) | Agent trait, command bus, service registry, work queues |
| [sheetgen-rust](https://github.com/Maangled/sheetgen-rust) | AI architectural documentation (Rust + GPT-4) | Schema-driven tests, three-layer decomposition, dependency resolution |
| [workpace-rust](https://github.com/Maangled/workpace-rust) | Prototype card website (Rust + WASM) | Module surface map, WASM pipeline, IndexedDB patterns |

## Integration Rules

1. **Port interfaces, not implementations** — extract patterns and API shapes only
2. **No server dependencies in Phase 0** — no Postgres, Redis, S3, or Docker
3. **Wave-aligned** — patterns are only ported when their target wave is active
4. **Schema-first** — all new data structures must update JSON schemas before code
5. **Anti-pattern: premature parallelization** — don't throw agents at shell surfaces
   before contracts stabilize
