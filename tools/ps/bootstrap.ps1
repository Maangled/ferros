Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

Write-Host "FERROS PowerShell tooling placeholder"
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
