# Release Checklist

This checklist describes the release-preparation bar for the repository.

The current branch baseline is `v0.9.0`.

## Public Surface

- README reflects the real public API
- public docs under `docs/` are in English
- the GitHub Pages site under `website/` builds from the committed docs tree without broken links
- `docs.rs` is treated as the primary API-reference host
- GitHub Pages hosts the public docs and the rustdoc bundle under `/api/`
- `Cargo.toml` carries release metadata for repository, documentation, keywords, categories, and license
- `Cargo.toml` carries the GitHub Pages `homepage`
- `Cargo.toml` intentionally omits `rust-version` until the project adopts an audited MSRV policy
- examples and rustdoc point to the same API shape
- API coverage documentation matches the codebase
- `docs/api-coverage.md` matches `tools/api-coverage/market-data-api.json`
- `docs/index.md`, `docs/project-structure.md`, `docs/reference/`, `docs/generated/`, `website/sidebars.ts`, and the README docs block match the latest `./tools/docs/generate-doc-site` output
- `./scripts/api-sync-audit` has been run against the intended release baseline and reports no blocking drift
- any detected mirror drift is resolved before convenience-layer compatibility is treated as valid again
- release CI is a tag-triggered guardrail and runs only when a `vX.Y.Z` tag is pushed
- GitHub Pages deployment is a separate workflow that builds on `main`

## Verification Targets

The release verification run should cover:

```bash
cargo fmt --check
cargo test
cargo check --examples
cargo test --doc
cargo doc --no-deps
npm run build --prefix website
cargo bench --no-run
cargo package --list
cargo package
cargo publish --dry-run
```

Required live tests must also be run with real Alpaca credentials when the scenario is expected to succeed against the official API.

## Packaging Expectations

The published crate should be clean:

- public docs are present
- internal workflow material is not shipped in the package artifact
- `.agents/`, `.github/`, `AGENTS.md`, `tools/docs/`, and `website/` are excluded from the package contents
