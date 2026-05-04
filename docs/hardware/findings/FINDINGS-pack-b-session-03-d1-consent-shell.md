# Findings — Pack B Session 03 D1 Consent Shell

> Filled from agent-executed commands on `homelab001` plus a live operator-visible localhost shell observation under explicit operator authorization from `Maangled`. Raw captures live under `.local-artifacts/pack-b-session-03-d1-closeout/`.

## Claim ceiling

- This findings packet captures only the D1 consent-flow visibility slice on the named Pack B device under test.
- It proves only that a capability-gated `ha-local-bridge` run was denied, that the denial was persisted in the local FERROS deny state, and that the same denied request was visible on the localhost shell surface.
- This findings packet does not authorize D1 closure, G4 closure, full DUT-only power-cycle survival, HA recovery, or a launch-grade Home Assistant bridge claim.

## Session header

| Field | Value |
|-------|-------|
| Date | `2026-05-04` |
| Operator | `Maangled` |
| Pack B DUT name | `homelab001` |
| Localhost shell route | `http://127.0.0.1:4317/` |
| Artifact path | `.local-artifacts/pack-b-session-03-d1-closeout/` |

## Command transcript

```text
cd /home/homelab001/apps/ferros
cargo run --target-dir target/copilot-shell -p ferros-node --bin ferros-node -- shell 4317
cargo run -p ferros-node --bin ferros -- agent describe ha-local-bridge | tee .local-artifacts/pack-b-session-03-d1-closeout/pre-power-agent-describe-ha-local-bridge.txt
cargo run -p ferros-node --bin ferros -- agent run ha-local-bridge | tee .local-artifacts/pack-b-session-03-d1-closeout/pre-power-denied-run.txt
cargo run -p ferros-node --bin ferros -- agent logs ha-local-bridge | tee .local-artifacts/pack-b-session-03-d1-closeout/pre-power-agent-logs-ha-local-bridge.txt
curl -s -X POST -H 'Content-Type: application/json' --data '{"jsonrpc":"2.0","id":"d1-deny","method":"agent.snapshot","params":{}}' http://127.0.0.1:4317/rpc | tee .local-artifacts/pack-b-session-03-d1-closeout/pre-power-agent-snapshot.json
```

## Bridge agent requirement reference

| Field | Value |
|-------|-------|
| Command used | `cargo run -p ferros-node --bin ferros -- agent describe ha-local-bridge` |
| Exit result | `0` |
| Required capability | `hub-local-bridge:bridge.observe` |
| Artifact reference | `.local-artifacts/pack-b-session-03-d1-closeout/pre-power-agent-describe-ha-local-bridge.txt` |

## Denied request result

| Field | Value |
|-------|-------|
| Command used | `cargo run -p ferros-node --bin ferros -- agent run ha-local-bridge` |
| Exit result | `1` |
| Observed deny note | `authorization denied: ha-local-bridge missing bridge.observe` |
| Artifact reference | `.local-artifacts/pack-b-session-03-d1-closeout/pre-power-denied-run.txt` |

## Persisted deny-log result

| Field | Value |
|-------|-------|
| Command used | `cargo run -p ferros-node --bin ferros -- agent logs ha-local-bridge` |
| Exit result | `0` |
| Observed deny-log note | `denied-start:ha-local-bridge missing bridge.observe` |
| Artifact reference | `.local-artifacts/pack-b-session-03-d1-closeout/pre-power-agent-logs-ha-local-bridge.txt` |

## Localhost shell observation

| Field | Value |
|-------|-------|
| Observation route | `http://127.0.0.1:4317/` |
| Backend artifact reference | `.local-artifacts/pack-b-session-03-d1-closeout/pre-power-agent-snapshot.json` |
| Grant rows visible | `0` |
| Deny entries visible | `2` |
| Latest deny event visible | `ha-local-bridge · ha-local-bridge missing bridge.observe` |
| Observation note | `The live localhost shell showed rpc live, denies 1 in the header summary, Deny entries 1 in the center panel before the second denied attempt, and Latest deny event: ha-local-bridge · ha-local-bridge missing bridge.observe in the consent/audit panel. The saved agent.snapshot artifact recorded an empty grant set plus denyLog entries for the denied ha-local-bridge start attempts.` |

## Remaining gaps

- This findings packet does not prove full DUT-only power-cycle survival; that still requires a separate hard power-cut observation.
- This findings packet does not widen the earlier Pack C stand-in HA proof into a launch-grade bridge claim.
- D1 remains open until the hard power-cycle row is observed and recorded.
- G4 remains open.

## Non-claims for this template

- No D1 or G4 closure.
- No full power-cycle survival.
- No Home Assistant recovery or independent install proof.