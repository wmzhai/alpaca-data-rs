# AGENTS.md

## Read First

Start new sessions with:

1. `README.md`
2. Relevant files under `docs/`
3. Relevant source files under `src/`, `tests/`, `examples/`, or `benches/`

## Scope

- This repository is a high-performance Rust client for the Alpaca Market Data HTTP API.
- Market Data HTTP API is in scope.
- Trading API, Broker API, WebSocket, and SSE are out of scope.
- The crates.io package name is `alpaca-data`.
- The Rust import path is `alpaca_data`.

## API Contract

- The official Alpaca Market Data HTTP API is the only semantic source of truth.
- Request field names and response field names must use the official words exactly.
- Rust-specific adaptation must stay minimal and only happen when the language requires it, such as `r#type`.
- Public Rust naming must stay idiomatic: modules lowercase, types `PascalCase`, methods and fields `snake_case`.
- The public root entry point is `alpaca_data::Client`.
- The public resource accessors are `stocks()`, `options()`, `crypto()`, `news()`, and `corporate_actions()`.
- The mirror layer must preserve the official endpoint shape.
- The convenience layer may only add `*_all` and `*_stream`.
- `*_all` must return the same response type with `next_page_token = None` after aggregation.
- `*_stream` must yield page-shaped responses, not flattened items.
- Type names should stay short and avoid repeating the resource prefix.

## Runtime and Performance

- The library is async-only. Do not add sync wrappers.
- Do not use `serde_json::Value` as the core hot-path data model when typed structs are available.
- Do not add implicit caching.
- Do not add derived Greeks, IV, or other synthetic calculations in the core client.

## Testing

- Successful-path tests must use the real Alpaca API whenever credentials can cover the scenario.
- Mocks are only allowed for fault injection such as 429, 5xx, timeout, malformed JSON, connection interruption, or invalid pagination tokens.
- Do not use mocks to prove primary correctness.
- Keep live tests gated by environment variables.

## Repository Rules

- Do not use `.worktrees/` or git worktrees for this repository.
- If sub-agents are used, they must use `gpt-5.4` only.
- If no audited MSRV is maintained, do not declare `rust-version` in `Cargo.toml`.
- GitHub CI must use the floating `stable` toolchain.
- Release CI must only run on pushed release tags such as `vX.Y.Z`.
- GitHub Pages documentation deployment must only run on pushed release tags such as `vX.Y.Z`.
- Keep internal workflow material out of the published crate artifact.
- Before release-readiness or API-parity work, run `.agents/skills/alpaca-market-data-sync/SKILL.md`.
- If the audit finds mirror drift, fix the mirror layer before re-validating convenience helpers.

## Documentation Rules

- `README.md` is the public design and usage contract.
- `./tools/docs/generate-doc-site` is the source of truth for generated docs under `docs/generated/`, `docs/reference/`, `docs/project-structure.md`, `docs/index.md`, and `website/sidebars.ts`.
- When the public API, repository structure, or documentation navigation changes, rerun `./tools/docs/generate-doc-site` before committing.
- `CHANGELOG.md` must stay in English and record factual release changes only.
- Do not write process history or planning narrative into retained public docs.
- Before any commit, align code, tests, `README.md`, `AGENTS.md`, relevant `docs/`, and `CHANGELOG.md`.
- When there is no implemented code fact, write it as a constraint or planned behavior, not as shipped behavior.

## Commit Rules

- Auto-generated commit messages must be in English.
- Commit titles must use Conventional Commits: `<type>: <summary>`.
- Preferred types are `feat`, `fix`, `chore`, `refactor`, and `docs`.
- If useful, add a short English body describing what the commit contains.
- Keep the version format at `MAJOR.MINOR.PATCH`.
- Versioned commits should use `<type>: <summary> (vX.Y.Z)`.
- Update `CHANGELOG.md` for every versioned commit.
- Use `chore: bump version and changelog (vX.Y.Z)` for the final release-changelog bump commit.
- Prefer `git merge --ff-only` when integrating linear release work.

## Do Not Do

- Do not mix Trading API or Broker API code into this crate.
- Do not rename official request or response words for SDK taste.
- Do not add mocks to normal successful-path tests.
