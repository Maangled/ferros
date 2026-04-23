# FERROS Licensing

FERROS currently uses a split licensing model:

- Code in this repository is licensed under GNU GPL v3.0 only. See `LICENSE`.
- Documentation content is licensed separately under CC BY 4.0. See `LICENSE-DOCS`.

## Cargo workspace alignment

The Rust workspace inherits `GPL-3.0-only` from the root `Cargo.toml` so the published crate metadata matches the repository's code license.

## Scope guidance

- Rust crates, shell scripts, generators, and other executable source files follow the code license.
- Markdown, HTML architecture docs, ADRs, and other written documentation follow the documentation license unless a file states otherwise.

If FERROS adopts a different or dual-license model later, update `LICENSE`, `LICENSE-DOCS`, `Cargo.toml`, and this file together in the same change set.
