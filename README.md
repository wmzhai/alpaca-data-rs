# alpaca-data

`alpaca-data` is an async Rust client for the Alpaca Market Data HTTP API.

The crate is built around two constraints:

- The official Alpaca Market Data HTTP API is the only semantic source of truth.
- Public Rust naming is idiomatic Rust, while request and response field words use the official API terms.

## Status

- Current branch baseline: `v0.9.2`
- Implemented resource families: `stocks`, `options`, `crypto`, `news`, `corporate_actions`
- This repository does not cover Trading API, Broker API, WebSocket, or SSE
- This crate is async-only
- crates.io package: `alpaca-data`
- Public docs include a GitHub Pages site, generated API reference pages, rustdoc links, API coverage docs, and a tag-triggered release workflow

Release automation is intentionally tag-triggered only and follows GitHub-hosted `stable`. The `github-pages` workflow runs the standard Rust validation steps and the documentation build only on pushed release tags such as `vX.Y.Z`. The manifest intentionally omits `rust-version` until an audited MSRV policy exists.

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

- Historical batch: `auctions`, `bars`, `quotes`, `trades`
- Historical single-symbol: `auctions_single`, `bars_single`, `quotes_single`, `trades_single`
- Convenience: `auctions_all`, `auctions_stream`, `bars_all`, `bars_stream`, `quotes_all`, `quotes_stream`, `trades_all`, `trades_stream`
- Single-symbol convenience: `auctions_single_all`, `auctions_single_stream`, `bars_single_all`, `bars_single_stream`, `quotes_single_all`, `quotes_single_stream`, `trades_single_all`, `trades_single_stream`
- Latest: `latest_bars`, `latest_bar`, `latest_quotes`, `latest_quote`, `latest_trades`, `latest_trade`
- Snapshots and metadata: `snapshots`, `snapshot`, `condition_codes`, `exchange_codes`

### Options

- Historical: `bars`, `trades`
- Convenience: `bars_all`, `bars_stream`, `trades_all`, `trades_stream`
- Latest: `latest_quotes`, `latest_trades`
- Snapshot family: `snapshots`, `snapshots_all`, `snapshots_stream`, `chain`, `chain_all`, `chain_stream`
- Metadata: `condition_codes`, `exchange_codes`

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

Install from crates.io:

```toml
[dependencies]
alpaca-data = "0.9.2"
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

<!-- docs-site:start -->
- Documentation site: https://wmzhai.github.io/alpaca-data-rs/
- Primary API host after publish: https://docs.rs/alpaca-data
- Project Pages route: `https://wmzhai.github.io/alpaca-data-rs/`
- Local generation: `./tools/docs/generate-doc-site`
- Local site install: `npm install --prefix website`
- Local site start: `npm run start --prefix website`
- Local site build: `npm run build --prefix website`
- API reference sections:
  - [Documentation](docs/index.md)
  - [Getting Started](docs/getting-started.md)
  - [Authentication](docs/authentication.md)
  - [Layers](docs/layers.md)
  - [Project Structure](docs/project-structure.md)
  - [API Reference](docs/reference/index.md)
  - [API Coverage](docs/api-coverage.md)
  - [Examples](docs/examples.md)
  - [Release Checklist](docs/release-checklist.md)
- Generated module references: `docs/reference/stocks.md`, `docs/reference/options.md`, `docs/reference/crypto.md`, `docs/reference/news.md`, `docs/reference/corporate-actions.md`, `docs/reference/common.md`, `docs/reference/transport.md`
<!-- docs-site:end -->


## API Audit Script

The repository includes one local entry point for official Market Data API audit work.

Requirements:

- `bash`
- `curl`
- `jq`

Run the read-only parity audit against the local coverage manifest and source tree:

```bash
./scripts/api-sync-audit
```

The script accepts no command-line arguments. It prints the complete audit report directly to the terminal and exits non-zero when blocking drift is detected. The report covers mirror-path coverage, parameter signatures, response-field signatures, enum gaps, and convenience helpers that require re-validation after mirror changes.

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

## Publication Notes

The published crate excludes internal workflow material such as `.agents/`, `.github/`, and `AGENTS.md`, while the tag-triggered release workflow verifies formatting, tests, examples, docs, package creation, and documentation-site builds on pushed release tags.

The public documentation site is published at `https://wmzhai.github.io/alpaca-data-rs/`. `docs.rs` remains the primary rustdoc host, and the repository docs under `docs/` remain the narrative documentation surface.

No automatic crates.io publication is performed by repository CI.
