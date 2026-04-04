# Release Checklist

This checklist describes the release-preparation bar for the repository.

## Phase Boundary

`Phase 6` prepares the crate for publication but does not publish it.

`Phase 7` handles:

- internal working-doc cleanup
- final public-repo cleanup
- the final publication decision

## Public Surface

- README reflects the real public API
- public docs under `docs/` are in English
- `docs.rs` is treated as the primary API-reference host
- examples and rustdoc point to the same API shape
- API coverage documentation matches the codebase
- `docs/api-coverage.md` matches `tools/api-coverage/market-data-api.json`

## Verification Targets

The final `Phase 6` verification run must cover:

```bash
cargo fmt --check
cargo test
cargo check --examples
cargo test --doc
cargo doc --no-deps
cargo bench --no-run
cargo package
cargo publish --dry-run
```

Required live tests must also be run with real Alpaca credentials when the scenario is expected to succeed against the official API.

## Packaging Expectations

The published crate should be clean:

- public docs are present
- internal working directories are not shipped in the package artifact

In `Phase 6`, this is achieved through package-boundary configuration rather than by deleting the internal directories from git.
