# Changelog

All notable changes to this project are documented in this file.

## v0.9.2

### Changed

- Consolidated release verification and documentation deployment into the `github-pages` workflow.
- Restricted release automation to pushed release tags such as `vX.Y.Z`.
- Updated the `github-pages` environment policy to allow only `v*.*.*` tags to deploy.
- Updated the retained repository docs to describe the tag-only release automation policy.

## v0.9.1

### Fixed

- Removed the `rg` runtime dependency from `./scripts/api-sync-audit` so the audit entry point and its integration tests run on the default GitHub-hosted Linux runner.

## v0.9.0

### Added

- Added stock auction mirror coverage with `auctions`, `auctions_single`, `auctions_all`, `auctions_stream`, `auctions_single_all`, and `auctions_single_stream`.
- Added option condition-code mirror coverage with `options().condition_codes`.
- Added dual-license files `LICENSE-MIT` and `LICENSE-APACHE`.
- Added live coverage for stock auctions, option condition codes, and the new crypto `loc` variants.

### Changed

- Extended `crypto::Loc` to include the official `us-2` and `bs-1` values.
- Updated the API coverage contract and public docs to reflect full coverage of the currently adopted Market Data resource families.
- Aligned installation and publication docs with the crates.io package and the live GitHub Pages documentation site.

## v0.8.0

### Added

- Added a Docusaurus documentation site under `website/` with a custom landing page, generated sidebar config, and GitHub Pages deployment workflow.
- Added `tools/docs/generate-doc-site` to generate `docs/index.md`, `docs/project-structure.md`, `docs/reference/`, `docs/generated/`, and the README documentation block from the current repository state.
- Added `tests/doc_site_generation.rs` to verify the documentation generator writes the expected public artifacts.

### Changed

- Enriched the public docs with a generated project-structure page, generated module reference pages, and local docs-site usage guidance.
- Added the GitHub Pages homepage metadata to `Cargo.toml` and excluded `website/` and `tools/docs/` from the published crate artifact.
- Split repository automation responsibilities so release CI remains tag-triggered while the documentation site deploys separately from `main`.

## v0.7.1

### Changed

- Removed the last retained references to the deleted internal planning directories from the kept public docs and changelog history.
- Kept `.gitignore` rules for local recreations of the removed internal directories while clearing those names from the retained repository text.

## v0.7.0

### Changed

- Removed the tracked internal planning and working-note directories from the repository as part of the final public-repository cleanup.
- Added `.gitignore` rules so local recreations of those removed internal directories stay out of git.
- Updated the retained docs and internal audit skill to stop referring to those removed directories as part of the live repository state.
- Simplified package-boundary notes now that those internal directories are no longer tracked.
- Kept `docs.rs` as the primary documentation host and did not add a GitHub Pages site.

## v0.6.3

### Added

- Added parameter-signature and response-field-signature contracts to `tools/api-coverage/market-data-api.json`.
- Added audit regression coverage for rejected CLI arguments and for parameter and response drift reporting in `tests/api_sync_scripts.rs`.

### Changed

- Consolidated the public API audit workflow into `scripts/api-sync-audit` as the single entry point.
- Made `scripts/api-sync-audit` run the full report by default, reject CLI arguments, and exit non-zero on blocking drift.
- Extended the audit report to cover mirror-path drift, parameter drift, response-field drift, enum gaps, and convenience-helper re-validation notes.
- Updated the retained docs and the internal audit skill to describe the single-script audit workflow.

## v0.6.2

### Added

- Added `scripts/api-sync-openapi` as a local entry point for fetching the latest official Market Data OpenAPI summary or raw JSON.
- Added `scripts/api-sync-audit` as a read-only local parity audit entry point that prints findings and recommended follow-up changes to the terminal.
- Added `tests/api_sync_scripts.rs` to verify script help output and the `crypto::Loc` schema-ref audit regression.

### Changed

- Updated the README and retained public docs to document the script-based API audit workflow.
- Changed the audit script to resolve `crypto::Loc` enum values through OpenAPI schema references and report adopted-family mirror gaps dynamically from the local manifest.

## v0.6.1

### Changed

- Removed the manifest-level `rust-version` declaration until the crate adopts an audited MSRV policy.
- Pinned the fault-injection-only `wiremock` dev-dependency to `0.6.4`.
- Changed GitHub Actions to run on pushed `vX.Y.Z` tags and use the floating `stable` Rust toolchain.
- Aligned repository documentation with the current Rust toolchain and CI policy.

## v0.6.0

### Added

- Added public English repository docs under `docs/`.
- Added runnable examples under `examples/`.
- Added crate-level, client-level, and module-level rustdoc.
- Added the public API coverage contract in `docs/api-coverage.md`.
- Added the machine-readable API coverage manifest at `tools/api-coverage/market-data-api.json`.
- Added the internal Alpaca Market Data sync skill at `.agents/skills/alpaca-market-data-sync/SKILL.md`.

### Changed

- Added release metadata, package-boundary controls, and pre-publication verification guidance for the crate.
- Aligned the README, release checklist, and repository maintenance docs with the shipped API surface.

## v0.5.5

### Added

- Added `.github/workflows/ci.yml` as the repository CI workflow for formatting, tests, examples, docs, and packaging checks.

### Changed

- Added `repository`, `documentation`, `license`, `keywords`, and `categories` metadata to `Cargo.toml`.
- Tightened package exclusions so internal workflow material is not shipped in the published crate.

## v0.5.4

### Added

- Added the internal API audit workflow at `.agents/skills/alpaca-market-data-sync/SKILL.md`.

### Changed

- Updated the API coverage and release docs so release-readiness checks require an official-vs-local API audit first.

## v0.5.3

### Added

- Added `docs/api-coverage.md` as the public endpoint-to-method mapping.
- Added `tools/api-coverage/market-data-api.json` as the machine-readable API coverage manifest.

### Changed

- Recorded the known adopted-family gaps for `StockAuctions`, `StockAuctionSingle`, and `OptionMetaConditions`.
- Recorded the known crypto `loc` parity gap for `us-2` and `bs-1`.

## v0.5.2

### Added

- Added crate-level rustdoc to `src/lib.rs`.
- Added public rustdoc for `Client`, `ClientBuilder`, and the root resource accessors.
- Added module-level rustdoc for `stocks`, `options`, `crypto`, `news`, and `corporate_actions`.
- Added runnable examples for `client_builder`, `stocks_latest_bar`, `stocks_bars_all`, `options_chain`, `crypto_latest_quotes`, `news_list`, and `corporate_actions_list`.

### Changed

- Updated the README and examples docs to point to the runnable examples directory.

## v0.5.1

### Added

- Added the first public English documentation set: `getting-started`, `authentication`, `layers`, `examples`, and `release-checklist`.
- Rewrote the public README in English to describe the shipped API surface and repository usage.

### Changed

- Synchronized repository documentation around the public release surface and packaging rules.

## v0.5.0

### Changed

- Completed the `news` and `corporate_actions` resource families with official mirror endpoints, convenience helpers, live coverage, fault-injection coverage, and benchmark support.

## v0.4.3

### Added

- Added `tests/mock_news_corporate_actions_errors.rs` for malformed JSON, merge behavior, and repeated `next_page_token` regressions.
- Added `benches/news_corporate_actions.rs` for `news.list` and `corporate_actions.list`.
- Added the `news_corporate_actions` bench target to `Cargo.toml`.

### Changed

- Expanded fault-injection coverage for `news` and `corporate_actions`.

## v0.4.2

### Added

- Added `tests/live_corporate_actions.rs` for `corporate_actions.list`, `list_all`, and `list_stream`.
- Added unit and public-API coverage for the `corporate_actions` request, response, route, and typed-family behavior.
- Added a repeated `next_page_token` regression test for shared pagination helpers.

### Changed

- Implemented `GET /v1/corporate-actions` with `list_all` and `list_stream`.
- Expanded `corporate_actions::CorporateActionType` to cover the full live API query set.
- Kept the official bucketed `corporate_actions` wrapper while providing typed arrays for documented families and fallbacks for unknown buckets.
- Made shared pagination fail fast on repeated page tokens.

## v0.4.1

### Added

- Added `tests/live_news.rs` for `news.list`, `news.list_all`, and `news.list_stream`.
- Added request, response, route, and public-API coverage for `news`.

### Changed

- Implemented `GET /v1beta1/news` with `list_all` and `list_stream`.
- Modeled `symbols` as `Option<Vec<String>>` while preserving the official CSV query word.
- Completed the typed `news` models for `author`, `created_at`, `updated_at`, `summary`, `content`, `url`, `images`, `symbols`, and `source`.

## v0.4.0

### Added

- Added `benches/crypto.rs` for `crypto.snapshots`.
- Added the `crypto` bench target to `Cargo.toml`.

### Changed

- Completed the `crypto` resource family with historical endpoints, latest endpoints, snapshots, convenience helpers, live coverage, fault coverage, and benchmark support.

## v0.3.3

### Added

- Added `tests/live_crypto_snapshots.rs` for `crypto.snapshots`.
- Added `tests/mock_crypto_errors.rs` for malformed snapshot and orderbook responses.
- Added route, request, response, and public-API coverage for `crypto.snapshots`.

### Changed

- Implemented `GET /v1beta3/crypto/{loc}/snapshots`.
- Updated `crypto::Snapshot` to preserve the official camelCase fields such as `latestTrade`, `latestQuote`, `minuteBar`, `dailyBar`, and `prevDailyBar`.
- Changed `SnapshotsResponse` to preserve the official top-level `snapshots` map.

## v0.3.2

### Added

- Added `tests/live_crypto_latest.rs` for `latest_bars`, `latest_quotes`, `latest_trades`, and `latest_orderbooks`.
- Added route, request, response, and public-API coverage for the crypto latest endpoint family.

### Changed

- Implemented `GET /v1beta3/crypto/{loc}/latest/bars`, `/latest/quotes`, `/latest/trades`, and `/latest/orderbooks`.
- Changed latest crypto responses to preserve the official top-level `bars`, `quotes`, `trades`, and `orderbooks` maps.
- Added the public `crypto::Orderbook` and `crypto::OrderbookLevel` typed models.

## v0.3.1

### Added

- Added `tests/live_crypto_historical.rs` for `crypto.bars`, `crypto.quotes`, `crypto.trades`, and `crypto.bars_all`.
- Added route, request, response, and pagination coverage for crypto historical endpoints.

### Changed

- Implemented `GET /v1beta3/crypto/{loc}/bars`, `/quotes`, and `/trades`.
- Added `bars_all`, `bars_stream`, `quotes_all`, `quotes_stream`, `trades_all`, and `trades_stream`.
- Modeled `crypto::Loc` and `crypto::TimeFrame` with official string values and preserved decimal trade and quote shapes.
- Fixed crypto endpoint routing to use the official hyphenated location words.

## v0.3.0

### Added

- Added `benches/options.rs` for `options.chain`.
- Added the `options` bench target to `Cargo.toml`.

### Changed

- Completed the `options` resource family with historical endpoints, latest endpoints, snapshots, chain, convenience helpers, live coverage, fault coverage, and benchmark support.

## v0.2.4

### Added

- Added `tests/live_options_snapshots_chain.rs` for `options.snapshots`, `options.chain`, `snapshots_all`, `snapshots_stream`, `chain_all`, and `chain_stream`.
- Added `tests/mock_options_errors.rs` for malformed JSON and duplicate-symbol pagination failures.
- Added request, response, route, and public-API coverage for option snapshots and chain endpoints.

### Changed

- Implemented `GET /v1beta1/options/snapshots` and `GET /v1beta1/options/snapshots/{underlying_symbol}`.
- Added `snapshots_all`, `snapshots_stream`, `chain_all`, and `chain_stream`.
- Added typed `greeks` and `impliedVolatility` support to `options::Snapshot`.
- Made snapshot and chain aggregation reject duplicate symbols across pages.

## v0.2.3

### Added

- Added `tests/live_options_latest_metadata.rs` for `options.latest_quotes`, `options.latest_trades`, and `options.exchange_codes`.
- Added route, request, and response coverage for option latest endpoints and metadata.

### Changed

- Implemented `GET /v1beta1/options/quotes/latest`, `GET /v1beta1/options/trades/latest`, and `GET /v1beta1/options/meta/exchanges`.
- Changed latest option responses to preserve the official top-level `quotes` and `trades` maps.
- Changed `ExchangeCodesResponse` to use the official top-level map shape instead of a custom wrapper.

## v0.2.2

### Added

- Added `tests/live_options_historical.rs` for `options.bars`, `options.trades`, `options.bars_all`, and `options.trades_stream`.
- Added request, response, and route coverage for option historical endpoints.

### Changed

- Implemented `GET /v1beta1/options/bars` and `GET /v1beta1/options/trades`.
- Added `bars_all`, `bars_stream`, `trades_all`, and `trades_stream`.
- Modeled `options::TimeFrame`, `options::OptionsFeed`, and `options::ContractType` with official string values.
- Replaced placeholder option historical models with typed `Bar` and `Trade` models.

## v0.2.1

### Changed

- Restricted delegated sub-agents to `gpt-5.4`.
- Synchronized repository rules and status documentation after the initial options work landed.

## v0.2.0

### Added

- Added `benches/stocks.rs` for `stocks.latest_quote`.
- Added the `stocks` bench target to `Cargo.toml`.

### Changed

- Completed the `stocks` resource family with batch historical endpoints, single historical endpoints, latest endpoints, snapshots, metadata endpoints, convenience helpers, live coverage, fault coverage, and benchmark support.
- Tightened live stock historical test parameters so they remain stable under the client timeout budget.

## v0.1.6

### Added

- Added live coverage for `bars_all`, `bars_stream`, `quotes_all`, `quotes_stream`, `trades_all`, and `trades_stream` in `tests/live_stocks_batch_historical.rs`.
- Added response tests and fault-injection tests for batch historical pagination merges and currency mismatches.

### Changed

- Implemented batch historical convenience helpers for `stocks`.
- Reused shared pagination helpers for batch stock historical endpoints.
- Tightened live-test pagination parameters to keep the real API happy-path stable.

## v0.1.5

### Added

- Added `tests/live_stocks_metadata.rs` for `stocks.condition_codes` and `stocks.exchange_codes`.
- Added request, response, and fault-injection coverage for stock metadata endpoints.
- Added the public `stocks::Tape` enum for the official metadata query word.

### Changed

- Implemented `GET /v2/stocks/meta/conditions/{ticktype}?tape=...` and `GET /v2/stocks/meta/exchanges`.
- Preserved the official metadata map shapes instead of custom wrapper objects.
- Removed placeholder metadata models that did not match the official payload.

## v0.1.4

### Added

- Added typed request and response support for stock latest and snapshot endpoints.
- Added `tests/live_stocks_latest_snapshot.rs` for latest and snapshot coverage across batch and single endpoints.

### Changed

- Implemented batch latest, single latest, batch snapshots, and single snapshot endpoints for `stocks`.
- Added `delayed_sip` to `stocks::DataFeed`.
- Preserved the official snapshot body shapes for both batch and single responses.

## v0.1.3

### Added

- Added single-historical stock request and response types for quotes and trades.
- Added `tests/live_stocks_single_historical.rs` for stock single-historical endpoints and convenience helpers.
- Added `tests/mock_stocks_errors.rs` for malformed JSON and pagination mismatches.

### Changed

- Implemented `GET /v2/stocks/{symbol}/bars`, `/quotes`, and `/trades`.
- Added `bars_single_all`, `bars_single_stream`, `quotes_single_all`, `quotes_single_stream`, `trades_single_all`, and `trades_single_stream`.
- Enforced pagination validation for mismatched `symbol` or `currency` across pages.

## v0.1.2

### Added

- Added `tests/live_stocks_batch_historical.rs` for stock batch historical endpoints.
- Added query-serialization coverage for stock batch historical request words.

### Changed

- Implemented `stocks.bars`, `stocks.quotes`, and `stocks.trades`.
- Modeled stock request enums with official string values, including `TimeFrame`, `Adjustment`, `Currency`, `DataFeed`, and `Sort`.
- Added typed stock historical response models with official wrappers and pagination fields.

## v0.1.1

### Added

- Added the initial public stock skeleton types for single historical, latest quote, snapshot, and condition-code requests and responses.
- Added stock batch and single route variants to `Endpoint`.

### Changed

- Added stock skeleton methods that returned `Error::NotImplemented` while the API surface was still being built.
- Kept static endpoint paths borrowed and only allocated for dynamic stock routes.

## v0.1.0

### Added

- Added `benches/shared_core.rs` for the shared `crypto.latest_quotes` hot path.
- Added repository rules for versioned release commits and full pre-commit alignment checks.

### Changed

- Established the first shared-core baseline for transport, auth, query, errors, pagination, and live verification.
- Reused a single `Client` inside the shared benchmark to avoid mixing builder overhead into the hot path.

## v0.0.6

### Added

- Added `tests/live_crypto_latest_quotes_smoke.rs` for real API validation of `crypto.latest_quotes`.
- Added public API coverage for `crypto.latest_quotes`.

### Changed

- Added the `ALPACA_LIVE_TESTS=1` gate for live tests.
- Added `APCA_API_DATA_URL` as a base-URL override for live smoke tests.

## v0.0.5

### Added

- Added `PaginatedRequest`, `PaginatedResponse`, `collect_all`, and `stream_pages`.
- Added unit tests for merged pagination and streamed pagination behavior.

### Changed

- Changed `common::response::ResponseStream` to a boxed async stream type.
- Changed `empty_stream()` to return a boxed empty stream.

## v0.0.4

### Added

- Added `reqwest`, `serde`, `serde_json`, `tokio`, and `wiremock` for transport and fault-injection testing.
- Added `tests/mock_transport_errors.rs` for 429 retry-after handling and malformed JSON mapping.
- Added request-header injection to `Auth`.
- Added async JSON GET support to `HttpClient`.

### Changed

- Moved `Client` onto a shared `HttpClient` configured by builder runtime settings.
- Added working retry and rate-limit behavior instead of placeholders.
- Extended `Error::RateLimited` to keep both `retry_after` and `body`.
- Routed `crypto.latest_quotes` through the shared transport path.

## v0.0.3

### Added

- Added `src/common/query.rs` with `QueryWriter`.
- Added `src/transport/endpoint.rs` with `Endpoint`.
- Added unit tests for query serialization and endpoint routing.

### Changed

- Exported the new `query` and `endpoint` modules from the shared module tree.

## v0.0.2

### Added

- Added `tests/client_builder.rs` for public crypto clients, partial credentials, and explicit runtime configuration.
- Added `timeout`, `max_retries`, and `max_in_flight` to `ClientBuilder`.
- Added `Error::InvalidConfiguration` for configuration failures.

### Changed

- Set `ClientBuilder::build()` to default to `https://data.alpaca.markets`.
- Required `api_key` and `secret_key` to be configured together.
- Tightened repository rules around code, test, and documentation alignment before commits.

## v0.0.1

### Added

- Bootstrapped the `alpaca-data` Rust library crate.
- Added the root `alpaca_data::Client` entry point.
- Added `stocks`, `options`, `crypto`, `news`, and `corporate_actions` module skeletons.
- Added the initial `transport` and `common` layers.
- Added `tests/public_api.rs` to verify the public API surface.
- Added `AGENTS.md` and the initial repository operating docs.

### Changed

- Set the crate version to `0.0.1`.
- Standardized the published crate name as `alpaca-data` and the Rust import path as `alpaca_data`.
- Standardized commit messages on English Conventional Commits.
