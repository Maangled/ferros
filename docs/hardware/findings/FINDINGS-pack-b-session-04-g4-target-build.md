# Findings — Pack B Session 04 G4 Target Build

> Filled from agent-executed commands on the physical host `homelab001` under explicit operator authorization from `Maangled`. Raw captures live under `.local-artifacts/pack-b-session-04-g4-target-build/`.

## Scope

This findings packet captures only the Pack B `x86_64` target-build proof for `ferros-hub` on the chosen core launch lane.

## Claim ceiling

- This packet proves that `cargo build -p ferros-hub --target x86_64-unknown-linux-gnu` completed successfully on the named Pack B DUT `homelab001`.
- This packet also proves that the expected target binary exists at `target/x86_64-unknown-linux-gnu/debug/ferros-hub` after the build.
- This packet does not authorize G4 closure by itself, physical-device runtime proof by itself, or any optional module-lane claim.

## Session header

| Field | Value |
|-------|-------|
| Date | `2026-05-04` |
| Operator | `Maangled` |
| Pack B DUT name | `homelab001` |
| Target triple | `x86_64-unknown-linux-gnu` |
| Artifact path | `.local-artifacts/pack-b-session-04-g4-target-build/` |

## Command transcript

```text
cd /home/homelab001/apps/ferros
mkdir -p .local-artifacts/pack-b-session-04-g4-target-build
cargo build -p ferros-hub --target x86_64-unknown-linux-gnu | tee .local-artifacts/pack-b-session-04-g4-target-build/ferros-hub-target-build.txt
ls -l target/x86_64-unknown-linux-gnu/debug/ferros-hub | tee .local-artifacts/pack-b-session-04-g4-target-build/ferros-hub-target-binary.txt
```

## Target build result

| Field | Value |
|-------|-------|
| Command used | `cargo build -p ferros-hub --target x86_64-unknown-linux-gnu` |
| Exit result | `0` |
| Cargo note | `Finished dev profile [unoptimized + debuginfo] target(s) in 0.05s` |
| Artifact reference | `.local-artifacts/pack-b-session-04-g4-target-build/ferros-hub-target-build.txt` |

## Built binary reference

| Field | Value |
|-------|-------|
| Binary path | `target/x86_64-unknown-linux-gnu/debug/ferros-hub` |
| Observed size | `37265096` bytes |
| Artifact reference | `.local-artifacts/pack-b-session-04-g4-target-build/ferros-hub-target-binary.txt` |

## Remaining gaps

- This packet does not prove the full G4 core runtime path by itself; it only covers the target build slice.
- This packet does not claim release tagging or G4 closure by itself.

## Non-claims for this template

- No G4 closure by itself.
- No optional module-lane proof.