#!/usr/bin/env bash
set -euo pipefail

echo "FERROS shell tooling placeholder"
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
