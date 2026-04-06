---
title: "Project Structure"
description: "Repository layout, module boundaries, examples, tests, and benchmarks."
sidebar_position: 4
---

# Project Structure

## Top-Level Directories

| Path | Responsibility |
| --- | --- |
| `.agents` | local automation and audit skills |
| `.github` | GitHub workflows for release and site automation |
| `benches` | Criterion micro-benchmarks |
| `docs` | public narrative and generated documentation |
| `examples` | runnable usage examples |
| `scripts` | public shell entry points |
| `src` | crate source code |
| `tests` | integration, live, and mock fault-injection tests |
| `tools` | public maintenance assets and doc generators |
| `website` | Docusaurus site shell for GitHub Pages |

## Source Tree

- `src/stocks/`
  - `client.rs`
  - `enums.rs`
  - `mod.rs`
  - `model.rs`
  - `request.rs`
  - `response.rs`
- `src/options/`
  - `client.rs`
  - `enums.rs`
  - `mod.rs`
  - `model.rs`
  - `request.rs`
  - `response.rs`
- `src/crypto/`
  - `client.rs`
  - `enums.rs`
  - `mod.rs`
  - `model.rs`
  - `request.rs`
  - `response.rs`
- `src/news/`
  - `client.rs`
  - `mod.rs`
  - `model.rs`
  - `request.rs`
  - `response.rs`
- `src/corporate_actions/`
  - `client.rs`
  - `mod.rs`
  - `model.rs`
  - `request.rs`
  - `response.rs`
- `src/common/`
  - `enums.rs`
  - `mod.rs`
  - `query.rs`
  - `response.rs`
  - `time.rs`
  - `validate.rs`
- `src/transport/`
  - `endpoint.rs`
  - `http.rs`
  - `meta.rs`
  - `mod.rs`
  - `observer.rs`
  - `pagination.rs`
  - `rate_limit.rs`
  - `retry.rs`

## Examples

| Example | Resource |
| --- | --- |
| `examples/client_builder.rs` | client |
| `examples/corporate_actions_list.rs` | corporate_actions |
| `examples/crypto_latest_quotes.rs` | crypto |
| `examples/news_list.rs` | news |
| `examples/options_chain.rs` | options |
| `examples/stocks_bars_all.rs` | stocks |
| `examples/stocks_latest_bar.rs` | stocks |

## Tests

| Test | Kind | Resource |
| --- | --- | --- |
| `tests/api_sync_scripts.rs` | integration | shared |
| `tests/client_builder.rs` | integration | client |
| `tests/decimal_precision.rs` | integration | shared |
| `tests/decimal_type_paths.rs` | integration | shared |
| `tests/doc_site_generation.rs` | integration | client |
| `tests/live_corporate_actions.rs` | live | corporate_actions |
| `tests/live_crypto_historical.rs` | live | crypto |
| `tests/live_crypto_latest.rs` | live | crypto |
| `tests/live_crypto_latest_quotes_smoke.rs` | live | crypto |
| `tests/live_crypto_loc_variants.rs` | live | crypto |
| `tests/live_crypto_snapshots.rs` | live | crypto |
| `tests/live_news.rs` | live | news |
| `tests/live_options_condition_codes.rs` | live | options |
| `tests/live_options_historical.rs` | live | options |
| `tests/live_options_latest_metadata.rs` | live | options |
| `tests/live_options_snapshots_chain.rs` | live | options |
| `tests/live_stocks_auctions.rs` | live | stocks |
| `tests/live_stocks_batch_historical.rs` | live | stocks |
| `tests/live_stocks_latest_snapshot.rs` | live | stocks |
| `tests/live_stocks_metadata.rs` | live | stocks |
| `tests/live_stocks_single_historical.rs` | live | stocks |
| `tests/mock_crypto_errors.rs` | mock | crypto |
| `tests/mock_news_corporate_actions_errors.rs` | mock | news |
| `tests/mock_options_errors.rs` | mock | options |
| `tests/mock_stocks_errors.rs` | mock | stocks |
| `tests/mock_transport_errors.rs` | mock | transport |
| `tests/public_api.rs` | integration | client |
| `tests/request_validation.rs` | integration | shared |

## Benchmarks

| Benchmark | Resource |
| --- | --- |
| `benches/crypto.rs` | crypto |
| `benches/news_corporate_actions.rs` | news |
| `benches/options.rs` | options |
| `benches/shared_core.rs` | transport |
| `benches/stocks.rs` | stocks |

## Documentation Pipeline

- `./tools/docs/generate-doc-site` regenerates the project structure page, API reference pages, sidebar schema, and the generated README documentation block.
- `website/` contains the Docusaurus site shell that consumes the committed docs under `docs/`.
- GitHub Pages builds the site and bundles fresh `cargo doc --no-deps` output under `/api/`.
