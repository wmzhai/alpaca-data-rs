# Release Checklist

This checklist describes the release-preparation bar for the repository.

The current branch baseline is `v0.6.2`.

## Public Surface

- README reflects the real public API
- public docs under `docs/` are in English
- `docs.rs` is treated as the primary API-reference host
- `Cargo.toml` carries release metadata for repository, documentation, keywords, categories, and license
- `Cargo.toml` intentionally omits `rust-version` until the project adopts an audited MSRV policy
- examples and rustdoc point to the same API shape
- API coverage documentation matches the codebase
- `docs/api-coverage.md` matches `tools/api-coverage/market-data-api.json`
- `./scripts/api-sync-audit` has been run against the intended release baseline
- any detected mirror drift is resolved before convenience-layer compatibility is treated as valid again
- repository CI is a tag-triggered release guardrail and runs only when a `vX.Y.Z` tag is pushed

## Verification Targets

The release verification run should cover:

```bash
cargo fmt --check
cargo test
cargo check --examples
cargo test --doc
cargo doc --no-deps
cargo bench --no-run
cargo package --list
cargo package
cargo publish --dry-run
```

Required live tests must also be run with real Alpaca credentials when the scenario is expected to succeed against the official API.

## Packaging Expectations

The published crate should be clean:

- public docs are present
- internal working directories are not shipped in the package artifact
- `.agents/`, `.github/`, `AGENTS.md`, `docs/superpowers/`, and `memory/` are excluded from the package contents

This is currently achieved through package-boundary configuration rather than by deleting internal directories from git.
