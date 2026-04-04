# alpaca-data

`alpaca-data` is an async Rust client for the Alpaca Market Data HTTP API.

The crate is built around two constraints:

- The official Alpaca Market Data HTTP API is the only semantic source of truth.
- Public Rust naming is idiomatic Rust, while request and response field words stay aligned with the official API.

## Status

- Current branch baseline: `v0.5.5`
- Implemented resource families: `stocks`, `options`, `crypto`, `news`, `corporate_actions`
- Current phase: `Phase 6: Release Hardening`
- This repository does not cover Trading API, Broker API, WebSocket, or SSE
- This crate is async-only

`Phase 6` is release preparation only. It improves the public docs surface, runnable examples, rustdoc, API coverage documentation, the internal API sync workflow, package metadata, and pre-release verification. Final internal-doc cleanup and any real crates.io publication decision are deferred to `Phase 7: Release`.

## Design Contract

### Official HTTP API first

- Endpoint semantics follow the official Alpaca Market Data API.
- Request fields and response fields use the original official words.
- Rust-specific adaptation is kept minimal and only used where the language requires it, such as `r#type`.

### Idiomatic Rust public API

- Crate name: `alpaca_data`
- Root client: `alpaca_data::Client`
- Resource accessors: `stocks()`, `options()`, `crypto()`, `news()`, `corporate_actions()`
- Modules are lowercase, types are `PascalCase`, methods and fields are `snake_case`

### Two API layers

The crate exposes two layers:

- Mirror layer: direct Rust wrappers for the official HTTP endpoints
- Convenience layer: `*_all` and `*_stream` helpers on top of official paginated endpoints

The convenience layer never changes the official payload words. It only automates pagination.

## Coverage Summary

### Stocks

- Historical batch: `bars`, `quotes`, `trades`
- Historical single-symbol: `bars_single`, `quotes_single`, `trades_single`
- Convenience: `bars_all`, `bars_stream`, `quotes_all`, `quotes_stream`, `trades_all`, `trades_stream`
- Single-symbol convenience: `bars_single_all`, `bars_single_stream`, `quotes_single_all`, `quotes_single_stream`, `trades_single_all`, `trades_single_stream`
- Latest: `latest_bars`, `latest_bar`, `latest_quotes`, `latest_quote`, `latest_trades`, `latest_trade`
- Snapshots and metadata: `snapshots`, `snapshot`, `condition_codes`, `exchange_codes`

### Options

- Historical: `bars`, `trades`
- Convenience: `bars_all`, `bars_stream`, `trades_all`, `trades_stream`
- Latest: `latest_quotes`, `latest_trades`
- Snapshot family: `snapshots`, `snapshots_all`, `snapshots_stream`, `chain`, `chain_all`, `chain_stream`
- Metadata: `exchange_codes`

### Crypto

- Historical: `bars`, `quotes`, `trades`
- Convenience: `bars_all`, `bars_stream`, `quotes_all`, `quotes_stream`, `trades_all`, `trades_stream`
- Latest: `latest_bars`, `latest_quotes`, `latest_trades`, `latest_orderbooks`
- Snapshots: `snapshots`

### News

- `list`, `list_all`, `list_stream`

### Corporate Actions

- `list`, `list_all`, `list_stream`

## Quick Start

The crate has not been published to crates.io yet. Until the release phase is complete, use a git dependency.

```toml
[dependencies]
alpaca-data = { git = "https://github.com/wmzhai/alpaca-data-rs.git" }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Build a client:

```rust
use alpaca_data::Client;

let client = Client::new();
let client = Client::builder().build()?;
# Ok::<(), alpaca_data::Error>(())
```

Fetch stock latest bars:

```rust
use alpaca_data::{Client, stocks};

#[tokio::main]
async fn main() -> Result<(), alpaca_data::Error> {
    let client = Client::builder()
        .api_key(std::env::var("APCA_API_KEY_ID").expect("APCA_API_KEY_ID is required"))
        .secret_key(std::env::var("APCA_API_SECRET_KEY").expect("APCA_API_SECRET_KEY is required"))
        .build()?;

    let response = client
        .stocks()
        .latest_bars(stocks::LatestBarsRequest {
            symbols: vec!["AAPL".into()],
            feed: None,
            currency: None,
        })
        .await?;

    println!("{response:?}");
    Ok(())
}
```

Collect all pages from a paginated endpoint:

```rust
use alpaca_data::{Client, news};

#[tokio::main]
async fn main() -> Result<(), alpaca_data::Error> {
    let client = Client::builder()
        .api_key(std::env::var("APCA_API_KEY_ID").expect("APCA_API_KEY_ID is required"))
        .secret_key(std::env::var("APCA_API_SECRET_KEY").expect("APCA_API_SECRET_KEY is required"))
        .build()?;

    let response = client
        .news()
        .list_all(news::ListRequest {
            start: Some("2026-04-01T00:00:00Z".into()),
            end: Some("2026-04-04T00:00:00Z".into()),
            sort: Some(news::Sort::Desc),
            symbols: Some(vec!["AAPL".into()]),
            limit: Some(50),
            include_content: Some(false),
            exclude_contentless: Some(true),
            page_token: None,
        })
        .await?;

    println!("{}", response.news.len());
    Ok(())
}
```

## Authentication

Authentication behavior follows the implemented endpoint rules:

- `stocks`, `options`, `news`, and `corporate_actions` require credentials
- The currently implemented `crypto` HTTP endpoints can run without credentials

Live tests in this repository use:

- `APCA_API_KEY_ID`
- `APCA_API_SECRET_KEY`
- `ALPACA_LIVE_TESTS=1`

If you keep local secrets in a `.env` file under different names such as `ALPACA_DATA_API_KEY` and `ALPACA_DATA_SECRET_KEY`, export them into the `APCA_*` names before running live tests or examples.

See [docs/authentication.md](docs/authentication.md) for the current auth contract.

## Documentation Map

The intended API-reference host is `docs.rs` once the crate is published. Repository docs under `docs/` are the narrative companion.
Runnable examples live under [`examples/`](examples/) and are indexed in [docs/examples.md](docs/examples.md).
The package metadata targets the source repository at `https://github.com/wmzhai/alpaca-data-rs` and the published API docs at `https://docs.rs/alpaca-data`.
The release metadata now uses the dual-license expression `MIT OR Apache-2.0`.

- [Getting started](docs/getting-started.md)
- [Authentication](docs/authentication.md)
- [Layers](docs/layers.md)
- [API coverage](docs/api-coverage.md)
- [Examples](docs/examples.md)
- [Release checklist](docs/release-checklist.md)

The formal endpoint-to-method mapping lives in [docs/api-coverage.md](docs/api-coverage.md).

## Testing

Default checks:

```bash
cargo test
```

Enable live tests:

```bash
ALPACA_LIVE_TESTS=1 cargo test --test live_crypto_latest_quotes_smoke -- --nocapture
ALPACA_LIVE_TESTS=1 cargo test --test live_stocks_batch_historical -- --nocapture
ALPACA_LIVE_TESTS=1 cargo test --test live_stocks_single_historical -- --nocapture
ALPACA_LIVE_TESTS=1 cargo test --test live_stocks_latest_snapshot -- --nocapture
ALPACA_LIVE_TESTS=1 cargo test --test live_stocks_metadata -- --nocapture
ALPACA_LIVE_TESTS=1 cargo test --test live_options_historical -- --nocapture
ALPACA_LIVE_TESTS=1 cargo test --test live_options_latest_metadata -- --nocapture
ALPACA_LIVE_TESTS=1 cargo test --test live_options_snapshots_chain -- --nocapture
ALPACA_LIVE_TESTS=1 cargo test --test live_news -- --nocapture
ALPACA_LIVE_TESTS=1 cargo test --test live_corporate_actions -- --nocapture
```

Normal success-path coverage must use the real Alpaca API whenever credentials can cover the scenario. Mocks are reserved for fault injection paths such as malformed JSON, repeated pagination tokens, timeouts, and HTTP failures.

## Benchmarks

Local micro-benchmark baselines currently live in:

- `benches/shared_core.rs`
- `benches/stocks.rs`
- `benches/options.rs`
- `benches/crypto.rs`
- `benches/news_corporate_actions.rs`

Compile benchmark targets without running a full sample:

```bash
cargo bench --no-run
```

## Release Plan

- `Phase 6: Release Hardening` prepares the crate for publication
- `Phase 7: Release` handles internal-doc cleanup, final repository cleanup, and the final publication decision

The published crate is expected to exclude internal workflow material such as `.agents/`, `.github/`, `AGENTS.md`, `docs/superpowers/`, and `memory/`, while CI verifies formatting, tests, examples, docs, and package creation.

No automatic crates.io publication happens during `Phase 6`.
