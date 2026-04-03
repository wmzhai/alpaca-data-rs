# Phase 2 Stocks Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 将 `stocks` 资源域完整落地为官方 Alpaca Market Data API HTTP `stocks` 模块，覆盖 batch / single 官方 endpoint、分页便利层、真实 API 测试、异常路径测试和 benchmark 基线。

**Architecture:** 这一 phase 在现有 shared core 基础上，把 `stocks` 做成第一个完整资源模板。公开 API 同时保留 batch 与 single 官方 endpoint，内部共享 endpoint routing、query serialization、typed decode 和 pagination helper。实现顺序按 public surface -> historical batch -> historical single -> latest/snapshot -> metadata/model completion -> benchmark/docs/phase completion 推进。

**Tech Stack:** Rust 2024 edition, `reqwest` async client with `rustls`, `tokio`, `serde`/`serde_json`, shared pagination helpers, real Alpaca Market Data API integration tests, limited `wiremock` for exceptional-path tests only, `criterion` for local benchmark baselines

---

## Execution Constraints

- 遵守 `AGENTS.md`：不要使用 `.worktrees/`；直接在普通 git 分支上执行
- 每个 task 完成前先跑对应验证，再同步 `CHANGELOG.md`、受影响文档和版本号
- 每个 task 完成后做一次 patch 版本提交
- `Phase 2` 收尾时执行一次 MINOR 版本升级，并使用 `chore: bump version and changelog (v0.2.0)` 完成 phase 提交
- 正常成功路径测试严禁 mock；尽量使用真实 Alpaca API

## File Structure

### Existing files to modify

- `Cargo.toml`
- `README.md`
- `CHANGELOG.md`
- `src/transport/endpoint.rs`
- `src/stocks/mod.rs`
- `src/stocks/client.rs`
- `src/stocks/enums.rs`
- `src/stocks/request.rs`
- `src/stocks/response.rs`
- `src/stocks/model.rs`
- `tests/public_api.rs`
- `memory/README.md`
- `memory/api/README.md`
- `memory/core/system-map.md`
- `memory/core/workflows.md`
- `docs/superpowers/plans/2026-04-03-full-project-roadmap.md`

### New files to create

- `tests/live_stocks_batch_historical.rs`
- `tests/live_stocks_single_historical.rs`
- `tests/live_stocks_latest_snapshot.rs`
- `tests/live_stocks_metadata.rs`
- `tests/mock_stocks_errors.rs`
- `benches/stocks.rs`

## Public API To Deliver

### Batch methods

- `bars`
- `quotes`
- `trades`
- `latest_bars`
- `latest_quotes`
- `latest_trades`
- `snapshots`
- `condition_codes`
- `exchange_codes`

### Single methods

- `bars_single`
- `quotes_single`
- `trades_single`
- `latest_bar`
- `latest_quote`
- `latest_trade`
- `snapshot`

### Convenience methods

- `bars_all` / `bars_stream`
- `quotes_all` / `quotes_stream`
- `trades_all` / `trades_stream`
- `bars_single_all` / `bars_single_stream`
- `quotes_single_all` / `quotes_single_stream`
- `trades_single_all` / `trades_single_stream`

## Type Matrix To Deliver

### Requests

- `BarsRequest` / `BarsSingleRequest`
- `QuotesRequest` / `QuotesSingleRequest`
- `TradesRequest` / `TradesSingleRequest`
- `LatestBarsRequest` / `LatestBarRequest`
- `LatestQuotesRequest` / `LatestQuoteRequest`
- `LatestTradesRequest` / `LatestTradeRequest`
- `SnapshotsRequest` / `SnapshotRequest`
- `ConditionCodesRequest`

### Responses

- `BarsResponse` / `BarsSingleResponse`
- `QuotesResponse` / `QuotesSingleResponse`
- `TradesResponse` / `TradesSingleResponse`
- `LatestBarsResponse` / `LatestBarResponse`
- `LatestQuotesResponse` / `LatestQuoteResponse`
- `LatestTradesResponse` / `LatestTradeResponse`
- `SnapshotsResponse` / `SnapshotResponse`
- `ConditionCodesResponse`
- `ExchangeCodesResponse`

### Models

- `Bar`
- `Quote`
- `Trade`
- `Snapshot`
- `ConditionCode`
- `ExchangeCode`

### Enum coverage

- `TimeFrame`
- `Adjustment`
- `DataFeed`
- `Sort`
- `Currency`
- `TickType` (for `ConditionCodesRequest`)

### Task 1: Public Surface, Routing, and Request/Response Skeletons

**Files:**
- Modify: `src/transport/endpoint.rs`
- Modify: `src/stocks/mod.rs`
- Modify: `src/stocks/client.rs`
- Modify: `src/stocks/request.rs`
- Modify: `src/stocks/response.rs`
- Modify: `tests/public_api.rs`
- Modify: `Cargo.toml`
- Modify: `CHANGELOG.md`

- [ ] **Step 1: Write the failing public API and routing tests**

```rust
use alpaca_data::{Client, stocks};

#[test]
fn stocks_module_exposes_batch_and_single_type_names() {
    let _ = stocks::BarsRequest::default();
    let _ = stocks::BarsSingleRequest::default();
    let _ = stocks::LatestQuoteRequest::default();
    let _ = stocks::SnapshotRequest::default();
    let _ = stocks::ConditionCodesRequest::default();
    let _ = stocks::BarsSingleResponse::default();
    let _ = stocks::LatestBarResponse::default();
    let _ = stocks::SnapshotResponse::default();
}

#[test]
fn stocks_client_exposes_batch_and_single_method_names() {
    let client = Client::builder()
        .api_key("key")
        .secret_key("secret")
        .build()
        .expect("client should build");

    let _ = client.stocks().bars(stocks::BarsRequest::default());
    let _ = client.stocks().bars_single(stocks::BarsSingleRequest::default());
    let _ = client.stocks().latest_quotes(stocks::LatestQuotesRequest::default());
    let _ = client.stocks().latest_quote(stocks::LatestQuoteRequest::default());
    let _ = client.stocks().snapshots(stocks::SnapshotsRequest::default());
    let _ = client.stocks().snapshot(stocks::SnapshotRequest::default());
    let _ = client
        .stocks()
        .condition_codes(stocks::ConditionCodesRequest::default());
    let _ = client.stocks().exchange_codes();
}
```

```rust
#[cfg(test)]
mod tests {
    use super::Endpoint;

    #[test]
    fn endpoint_routes_stocks_batch_and_single_paths() {
        assert_eq!(Endpoint::stocks_bars().path(), "/v2/stocks/bars");
        assert_eq!(
            Endpoint::stocks_bars_single("AAPL").path(),
            "/v2/stocks/AAPL/bars"
        );
        assert_eq!(
            Endpoint::stocks_latest_quote("AAPL").path(),
            "/v2/stocks/AAPL/quotes/latest"
        );
        assert_eq!(
            Endpoint::stocks_snapshot("AAPL").path(),
            "/v2/stocks/AAPL/snapshot"
        );
    }
}
```

- [ ] **Step 2: Run the targeted tests to verify they fail**

Run: `cargo test --test public_api stocks_module_exposes_batch_and_single_type_names -- --nocapture`
Expected: FAIL because the new request/response types and methods do not exist yet.

Run: `cargo test endpoint_routes_stocks_batch_and_single_paths --lib -- --nocapture`
Expected: FAIL because the `Endpoint` variants do not exist yet.

- [ ] **Step 3: Add the public surface and skeleton routing**

```rust
#[derive(Clone, Debug, Default)]
pub struct BarsSingleRequest {
    pub symbol: String,
    pub timeframe: TimeFrame,
    pub start: Option<String>,
    pub end: Option<String>,
    pub limit: Option<u32>,
    pub adjustment: Option<Adjustment>,
    pub feed: Option<DataFeed>,
    pub sort: Option<Sort>,
    pub asof: Option<String>,
    pub currency: Option<Currency>,
    pub page_token: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct LatestQuoteRequest {
    pub symbol: String,
    pub feed: Option<DataFeed>,
    pub currency: Option<Currency>,
}

#[derive(Clone, Debug, Default)]
pub struct SnapshotRequest {
    pub symbol: String,
    pub feed: Option<DataFeed>,
    pub currency: Option<Currency>,
}

#[derive(Clone, Debug, Default)]
pub struct ConditionCodesRequest {
    pub ticktype: TickType,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TickType {
    #[default]
    Trade,
    Quote,
}
```

```rust
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BarsSingleResponse {
    pub bars: Vec<Bar>,
    pub next_page_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LatestQuoteResponse {
    pub quote: Quote,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SnapshotResponse {
    pub snapshot: Snapshot,
}
```

```rust
pub async fn bars_single(&self, _request: BarsSingleRequest) -> Result<BarsSingleResponse, Error> {
    self.ensure_credentials()?;
    Err(Error::NotImplemented {
        operation: "stocks.bars_single",
    })
}

pub async fn latest_quote(
    &self,
    _request: LatestQuoteRequest,
) -> Result<LatestQuoteResponse, Error> {
    self.ensure_credentials()?;
    Err(Error::NotImplemented {
        operation: "stocks.latest_quote",
    })
}

pub async fn snapshot(&self, _request: SnapshotRequest) -> Result<SnapshotResponse, Error> {
    self.ensure_credentials()?;
    Err(Error::NotImplemented {
        operation: "stocks.snapshot",
    })
}
```

```rust
pub(crate) enum Endpoint {
    CryptoLatestQuotes { loc: Loc },
    StocksBars,
    StocksQuotes,
    StocksTrades,
    StocksBarsSingle { symbol: String },
    StocksQuotesSingle { symbol: String },
    StocksTradesSingle { symbol: String },
    StocksLatestBars,
    StocksLatestQuotes,
    StocksLatestTrades,
    StocksLatestBar { symbol: String },
    StocksLatestQuote { symbol: String },
    StocksLatestTrade { symbol: String },
    StocksSnapshots,
    StocksSnapshot { symbol: String },
    StocksConditionCodes { ticktype: &'static str },
    StocksExchangeCodes,
}
```

- [ ] **Step 4: Re-run the targeted tests**

Run: `cargo test --test public_api stocks_module_exposes_batch_and_single_type_names -- --nocapture`
Expected: PASS.

Run: `cargo test endpoint_routes_stocks_batch_and_single_paths --lib -- --nocapture`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml src/transport/endpoint.rs src/stocks/mod.rs src/stocks/client.rs src/stocks/request.rs src/stocks/response.rs tests/public_api.rs CHANGELOG.md
git commit -m "feat: add stocks endpoint surface and routing (v0.1.1)"
```

### Task 2: Historical Batch Endpoints and Real API Baseline

**Files:**
- Modify: `src/common/enums.rs`
- Modify: `src/stocks/enums.rs`
- Modify: `src/stocks/request.rs`
- Modify: `src/stocks/response.rs`
- Modify: `src/stocks/model.rs`
- Modify: `src/stocks/client.rs`
- Modify: `src/transport/endpoint.rs`
- Create: `tests/live_stocks_batch_historical.rs`
- Modify: `CHANGELOG.md`

- [ ] **Step 1: Write failing live and unit tests for batch historical endpoints**

```rust
use alpaca_data::{Client, stocks};

#[tokio::test]
async fn stocks_bars_quotes_and_trades_use_real_api() {
    if std::env::var("ALPACA_LIVE_TESTS").as_deref() != Ok("1") {
        eprintln!("skipping live test; set ALPACA_LIVE_TESTS=1 to enable");
        return;
    }

    let client = Client::builder()
        .api_key(std::env::var("APCA_API_KEY_ID").expect("key required"))
        .secret_key(std::env::var("APCA_API_SECRET_KEY").expect("secret required"))
        .base_url(
            std::env::var("APCA_API_DATA_URL")
                .unwrap_or_else(|_| "https://data.alpaca.markets".into()),
        )
        .build()
        .expect("client should build");

    let bars = client
        .stocks()
        .bars(stocks::BarsRequest {
            symbols: vec!["AAPL".into(), "MSFT".into()],
            timeframe: stocks::TimeFrame::Day1,
            start: Some("2026-03-20T00:00:00Z".into()),
            end: Some("2026-03-25T00:00:00Z".into()),
            limit: Some(10),
            adjustment: Some(stocks::Adjustment::Raw),
            feed: Some(stocks::DataFeed::Iex),
            sort: Some(stocks::Sort::Asc),
            asof: None,
            currency: Some(stocks::Currency::Usd),
            page_token: None,
        })
        .await
        .expect("bars should succeed");

    assert!(bars.bars.contains_key("AAPL"));

    let quotes = client
        .stocks()
        .quotes(stocks::QuotesRequest {
            symbols: vec!["AAPL".into()],
            start: Some("2026-03-25T13:30:00Z".into()),
            end: Some("2026-03-25T13:35:00Z".into()),
            limit: Some(100),
            feed: Some(stocks::DataFeed::Iex),
            sort: Some(stocks::Sort::Asc),
            asof: None,
            currency: Some(stocks::Currency::Usd),
            page_token: None,
        })
        .await
        .expect("quotes should succeed");

    assert!(quotes.quotes.contains_key("AAPL"));
}
```

```rust
#[test]
fn stocks_data_feed_serializes_to_official_strings() {
    assert_eq!(stocks::DataFeed::Sip.as_str(), "sip");
    assert_eq!(stocks::DataFeed::Iex.as_str(), "iex");
    assert_eq!(stocks::Adjustment::All.as_str(), "all");
    assert_eq!(stocks::TimeFrame::Day1.as_str(), "1Day");
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test stocks_data_feed_serializes_to_official_strings --lib -- --nocapture`
Expected: FAIL because the enum string mapping helper does not exist yet.

Run: `ALPACA_LIVE_TESTS=1 cargo test --test live_stocks_batch_historical -- --nocapture`
Expected: FAIL because `bars`, `quotes`, and `trades` are not implemented yet.

- [ ] **Step 3: Implement batch historical requests, wrappers, models, and fetchers**

```rust
pub trait ApiStr {
    fn as_str(&self) -> &'static str;
}

impl ApiStr for TimeFrame {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Min1 => "1Min",
            Self::Day1 => "1Day",
        }
    }
}

impl ApiStr for DataFeed {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Sip => "sip",
            Self::Iex => "iex",
            Self::Boats => "boats",
            Self::Overnight => "overnight",
        }
    }
}
```

```rust
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct Bar {
    pub t: Option<Timestamp>,
    pub o: Option<f64>,
    pub h: Option<f64>,
    pub l: Option<f64>,
    pub c: Option<f64>,
    pub v: Option<f64>,
    pub n: Option<u64>,
    pub vw: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct Quote {
    pub t: Option<Timestamp>,
    pub ax: Option<String>,
    pub ap: Option<f64>,
    pub r#as: Option<f64>,
    pub bx: Option<String>,
    pub bp: Option<f64>,
    pub bs: Option<f64>,
    pub c: Option<Vec<String>>,
    pub z: Option<String>,
}
```

```rust
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct BarsResponse {
    pub bars: std::collections::HashMap<String, Vec<Bar>>,
    pub next_page_token: Option<String>,
}
```

```rust
pub async fn bars(&self, request: BarsRequest) -> Result<BarsResponse, Error> {
    self.ensure_credentials()?;
    let endpoint = Endpoint::stocks_bars();
    let mut query = QueryWriter::default();
    query.push_csv("symbols", &request.symbols);
    query.push_opt("timeframe", Some(request.timeframe.as_str()));
    query.push_opt("start", request.start.as_deref());
    query.push_opt("end", request.end.as_deref());
    query.push_opt("limit", request.limit);
    query.push_opt("adjustment", request.adjustment.map(|value| value.as_str()));
    query.push_opt("feed", request.feed.map(|value| value.as_str()));
    query.push_opt("sort", request.sort.map(|value| value.as_str()));
    query.push_opt("asof", request.asof.as_deref());
    query.push_opt("currency", request.currency.map(|value| value.as_str()));
    query.push_opt("page_token", request.page_token.as_deref());
    self.inner
        .http
        .get_json(&self.inner.base_url, endpoint, &self.inner.auth, query.finish())
        .await
}
```

- [ ] **Step 4: Re-run the batch historical tests**

Run: `cargo test stocks_data_feed_serializes_to_official_strings --lib -- --nocapture`
Expected: PASS.

Run: `ALPACA_LIVE_TESTS=1 cargo test --test live_stocks_batch_historical -- --nocapture`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/common/enums.rs src/stocks/enums.rs src/stocks/request.rs src/stocks/response.rs src/stocks/model.rs src/stocks/client.rs src/transport/endpoint.rs tests/live_stocks_batch_historical.rs CHANGELOG.md
git commit -m "feat: add stocks batch historical endpoints (v0.1.2)"
```

### Task 3: Historical Single Endpoints and Pagination Convenience

**Files:**
- Modify: `src/stocks/request.rs`
- Modify: `src/stocks/response.rs`
- Modify: `src/stocks/client.rs`
- Modify: `src/transport/endpoint.rs`
- Modify: `src/transport/pagination.rs`
- Create: `tests/live_stocks_single_historical.rs`
- Create: `tests/mock_stocks_errors.rs`
- Modify: `CHANGELOG.md`

- [ ] **Step 1: Write failing tests for single historical endpoints and convenience helpers**

```rust
use alpaca_data::{Client, stocks};
use futures_util::StreamExt;

#[tokio::test]
async fn stocks_single_historical_and_pagination_helpers_use_real_api() {
    if std::env::var("ALPACA_LIVE_TESTS").as_deref() != Ok("1") {
        eprintln!("skipping live test; set ALPACA_LIVE_TESTS=1 to enable");
        return;
    }

    let client = Client::builder()
        .api_key(std::env::var("APCA_API_KEY_ID").expect("key required"))
        .secret_key(std::env::var("APCA_API_SECRET_KEY").expect("secret required"))
        .build()
        .expect("client should build");

    let request = stocks::BarsSingleRequest {
        symbol: "AAPL".into(),
        timeframe: stocks::TimeFrame::Min1,
        start: Some("2026-03-25T13:30:00Z".into()),
        end: Some("2026-03-25T13:40:00Z".into()),
        limit: Some(2),
        adjustment: Some(stocks::Adjustment::Raw),
        feed: Some(stocks::DataFeed::Iex),
        sort: Some(stocks::Sort::Asc),
        asof: None,
        currency: Some(stocks::Currency::Usd),
        page_token: None,
    };

    let page = client
        .stocks()
        .bars_single(request.clone())
        .await
        .expect("single bars should succeed");
    assert!(!page.bars.is_empty());

    let all = client
        .stocks()
        .bars_single_all(request.clone())
        .await
        .expect("single bars_all should succeed");
    assert!(!all.bars.is_empty());
    assert_eq!(all.next_page_token, None);

    let pages = client
        .stocks()
        .bars_single_stream(request)
        .collect::<Vec<_>>()
        .await;
    assert!(!pages.is_empty());
}
```

```rust
#[tokio::test]
async fn malformed_single_historical_json_maps_to_deserialize_error() {
    let server = wiremock::MockServer::start().await;
    wiremock::Mock::given(wiremock::matchers::method("GET"))
        .and(wiremock::matchers::path("/v2/stocks/AAPL/bars"))
        .respond_with(wiremock::ResponseTemplate::new(200).set_body_raw(
            "not-json",
            "application/json",
        ))
        .mount(&server)
        .await;

    let error = Client::builder()
        .api_key("key")
        .secret_key("secret")
        .base_url(server.uri())
        .build()
        .expect("client should build")
        .stocks()
        .bars_single(stocks::BarsSingleRequest {
            symbol: "AAPL".into(),
            timeframe: stocks::TimeFrame::Min1,
            start: None,
            end: None,
            limit: Some(1),
            adjustment: None,
            feed: None,
            sort: None,
            asof: None,
            currency: None,
            page_token: None,
        })
        .await
        .expect_err("request should fail");

    assert!(matches!(error, alpaca_data::Error::Deserialize(_)));
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test --test mock_stocks_errors malformed_single_historical_json_maps_to_deserialize_error -- --nocapture`
Expected: FAIL because `bars_single` is not implemented yet.

Run: `ALPACA_LIVE_TESTS=1 cargo test --test live_stocks_single_historical -- --nocapture`
Expected: FAIL because single historical methods and `*_single_all` / `*_single_stream` do not exist yet.

- [ ] **Step 3: Implement single historical endpoints and pagination helpers**

```rust
impl crate::transport::pagination::PaginatedRequest for BarsSingleRequest {
    fn with_page_token(&self, page_token: Option<String>) -> Self {
        let mut next = self.clone();
        next.page_token = page_token;
        next
    }
}

impl crate::transport::pagination::PaginatedResponse for BarsSingleResponse {
    fn next_page_token(&self) -> Option<&str> {
        self.next_page_token.as_deref()
    }

    fn merge_page(&mut self, next: Self) -> Result<(), Error> {
        self.bars.extend(next.bars);
        self.next_page_token = next.next_page_token;
        Ok(())
    }

    fn clear_next_page_token(&mut self) {
        self.next_page_token = None;
    }
}
```

```rust
pub async fn bars_single_all(
    &self,
    request: BarsSingleRequest,
) -> Result<BarsSingleResponse, Error> {
    self.ensure_credentials()?;
    crate::transport::pagination::collect_all(request, |request| {
        let client = self.clone();
        async move { client.bars_single(request).await }
    })
    .await
}

pub fn bars_single_stream(
    &self,
    request: BarsSingleRequest,
) -> ResponseStream<Result<BarsSingleResponse, Error>> {
    let client = self.clone();
    crate::transport::pagination::stream_pages(request, move |request| {
        let client = client.clone();
        async move { client.bars_single(request).await }
    })
}
```

- [ ] **Step 4: Re-run the single historical tests**

Run: `cargo test --test mock_stocks_errors malformed_single_historical_json_maps_to_deserialize_error -- --nocapture`
Expected: PASS.

Run: `ALPACA_LIVE_TESTS=1 cargo test --test live_stocks_single_historical -- --nocapture`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/stocks/request.rs src/stocks/response.rs src/stocks/client.rs src/transport/endpoint.rs src/transport/pagination.rs tests/live_stocks_single_historical.rs tests/mock_stocks_errors.rs CHANGELOG.md
git commit -m "feat: add stocks single historical pagination (v0.1.3)"
```

### Task 4: Latest and Snapshot Endpoints for Batch and Single

**Files:**
- Modify: `src/stocks/request.rs`
- Modify: `src/stocks/response.rs`
- Modify: `src/stocks/model.rs`
- Modify: `src/stocks/client.rs`
- Modify: `src/transport/endpoint.rs`
- Create: `tests/live_stocks_latest_snapshot.rs`
- Modify: `tests/public_api.rs`
- Modify: `CHANGELOG.md`

- [ ] **Step 1: Write failing live tests for latest and snapshot endpoints**

```rust
use alpaca_data::{Client, stocks};

#[tokio::test]
async fn stocks_latest_and_snapshot_endpoints_use_real_api() {
    if std::env::var("ALPACA_LIVE_TESTS").as_deref() != Ok("1") {
        eprintln!("skipping live test; set ALPACA_LIVE_TESTS=1 to enable");
        return;
    }

    let client = Client::builder()
        .api_key(std::env::var("APCA_API_KEY_ID").expect("key required"))
        .secret_key(std::env::var("APCA_API_SECRET_KEY").expect("secret required"))
        .build()
        .expect("client should build");

    let latest_quotes = client
        .stocks()
        .latest_quotes(stocks::LatestQuotesRequest {
            symbols: vec!["AAPL".into(), "MSFT".into()],
            feed: Some(stocks::DataFeed::Iex),
            currency: Some(stocks::Currency::Usd),
        })
        .await
        .expect("latest quotes should succeed");
    assert!(latest_quotes.quotes.contains_key("AAPL"));

    let latest_quote = client
        .stocks()
        .latest_quote(stocks::LatestQuoteRequest {
            symbol: "AAPL".into(),
            feed: Some(stocks::DataFeed::Iex),
            currency: Some(stocks::Currency::Usd),
        })
        .await
        .expect("latest quote should succeed");
    assert!(latest_quote.quote.t.is_some());

    let snapshots = client
        .stocks()
        .snapshots(stocks::SnapshotsRequest {
            symbols: vec!["AAPL".into(), "MSFT".into()],
            feed: Some(stocks::DataFeed::Iex),
            currency: Some(stocks::Currency::Usd),
        })
        .await
        .expect("snapshots should succeed");
    assert!(snapshots.snapshots.contains_key("AAPL"));

    let snapshot = client
        .stocks()
        .snapshot(stocks::SnapshotRequest {
            symbol: "AAPL".into(),
            feed: Some(stocks::DataFeed::Iex),
            currency: Some(stocks::Currency::Usd),
        })
        .await
        .expect("snapshot should succeed");
    assert!(snapshot.snapshot.latest_trade.is_some());
}
```

- [ ] **Step 2: Run the live tests to verify they fail**

Run: `ALPACA_LIVE_TESTS=1 cargo test --test live_stocks_latest_snapshot -- --nocapture`
Expected: FAIL because latest/snapshot endpoints are not implemented yet.

- [ ] **Step 3: Implement latest/snapshot batch and single fetchers**

```rust
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct LatestQuoteResponse {
    pub quote: Quote,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct SnapshotResponse {
    #[serde(flatten)]
    pub snapshot: Snapshot,
}
```

```rust
pub async fn latest_quotes(
    &self,
    request: LatestQuotesRequest,
) -> Result<LatestQuotesResponse, Error> {
    self.ensure_credentials()?;
    let mut query = QueryWriter::default();
    query.push_csv("symbols", &request.symbols);
    query.push_opt("feed", request.feed.map(|value| value.as_str()));
    query.push_opt("currency", request.currency.map(|value| value.as_str()));
    self.inner
        .http
        .get_json(
            &self.inner.base_url,
            Endpoint::stocks_latest_quotes(),
            &self.inner.auth,
            query.finish(),
        )
        .await
}

pub async fn latest_quote(
    &self,
    request: LatestQuoteRequest,
) -> Result<LatestQuoteResponse, Error> {
    self.ensure_credentials()?;
    let mut query = QueryWriter::default();
    query.push_opt("feed", request.feed.map(|value| value.as_str()));
    query.push_opt("currency", request.currency.map(|value| value.as_str()));
    self.inner
        .http
        .get_json(
            &self.inner.base_url,
            Endpoint::stocks_latest_quote(&request.symbol),
            &self.inner.auth,
            query.finish(),
        )
        .await
}
```

- [ ] **Step 4: Re-run the live tests**

Run: `ALPACA_LIVE_TESTS=1 cargo test --test live_stocks_latest_snapshot -- --nocapture`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/stocks/request.rs src/stocks/response.rs src/stocks/model.rs src/stocks/client.rs src/transport/endpoint.rs tests/live_stocks_latest_snapshot.rs tests/public_api.rs CHANGELOG.md
git commit -m "feat: add stocks latest and snapshot endpoints (v0.1.4)"
```

### Task 5: Metadata Endpoints and Model Completion

**Files:**
- Modify: `src/stocks/enums.rs`
- Modify: `src/stocks/request.rs`
- Modify: `src/stocks/response.rs`
- Modify: `src/stocks/model.rs`
- Modify: `src/stocks/client.rs`
- Modify: `src/transport/endpoint.rs`
- Create: `tests/live_stocks_metadata.rs`
- Modify: `tests/mock_stocks_errors.rs`
- Modify: `CHANGELOG.md`

- [ ] **Step 1: Write failing tests for metadata endpoints and model completeness**

```rust
use alpaca_data::{Client, stocks};

#[tokio::test]
async fn stocks_metadata_endpoints_use_real_api() {
    if std::env::var("ALPACA_LIVE_TESTS").as_deref() != Ok("1") {
        eprintln!("skipping live test; set ALPACA_LIVE_TESTS=1 to enable");
        return;
    }

    let client = Client::builder()
        .api_key(std::env::var("APCA_API_KEY_ID").expect("key required"))
        .secret_key(std::env::var("APCA_API_SECRET_KEY").expect("secret required"))
        .build()
        .expect("client should build");

    let condition_codes = client
        .stocks()
        .condition_codes(stocks::ConditionCodesRequest {
            ticktype: stocks::TickType::Trade,
        })
        .await
        .expect("condition codes should succeed");
    assert!(!condition_codes.condition_codes.is_empty());

    let exchange_codes = client
        .stocks()
        .exchange_codes()
        .await
        .expect("exchange codes should succeed");
    assert!(!exchange_codes.exchange_codes.is_empty());
}
```

```rust
#[test]
fn tick_type_serializes_to_official_strings() {
    assert_eq!(stocks::TickType::Trade.as_str(), "trades");
    assert_eq!(stocks::TickType::Quote.as_str(), "quotes");
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test tick_type_serializes_to_official_strings --lib -- --nocapture`
Expected: FAIL because `TickType` does not exist yet.

Run: `ALPACA_LIVE_TESTS=1 cargo test --test live_stocks_metadata -- --nocapture`
Expected: FAIL because metadata endpoints are not implemented yet.

- [ ] **Step 3: Implement metadata requests/models and finish official field coverage**

```rust
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TickType {
    #[default]
    Trade,
    Quote,
}

impl crate::common::enums::ApiStr for TickType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Trade => "trades",
            Self::Quote => "quotes",
        }
    }
}
```

```rust
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct ConditionCode {
    pub code: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct ExchangeCode {
    pub id: Option<u64>,
    pub code: String,
    pub name: String,
    pub tape: Option<String>,
}
```

```rust
pub async fn condition_codes(
    &self,
    request: ConditionCodesRequest,
) -> Result<ConditionCodesResponse, Error> {
    self.ensure_credentials()?;
    self.inner
        .http
        .get_json(
            &self.inner.base_url,
            Endpoint::stocks_condition_codes(request.ticktype.as_str()),
            &self.inner.auth,
            Vec::new(),
        )
        .await
}
```

- [ ] **Step 4: Re-run the metadata tests**

Run: `cargo test tick_type_serializes_to_official_strings --lib -- --nocapture`
Expected: PASS.

Run: `ALPACA_LIVE_TESTS=1 cargo test --test live_stocks_metadata -- --nocapture`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/stocks/enums.rs src/stocks/request.rs src/stocks/response.rs src/stocks/model.rs src/stocks/client.rs src/transport/endpoint.rs tests/live_stocks_metadata.rs tests/mock_stocks_errors.rs CHANGELOG.md
git commit -m "feat: add stocks metadata and complete models (v0.1.5)"
```

### Task 6: Benchmark, Docs, Verification, and Phase Completion

**Files:**
- Create: `benches/stocks.rs`
- Modify: `README.md`
- Modify: `AGENTS.md` (only if new stable rule appears)
- Modify: `memory/README.md`
- Modify: `memory/api/README.md`
- Modify: `memory/core/system-map.md`
- Modify: `memory/core/workflows.md`
- Modify: `docs/superpowers/plans/2026-04-03-full-project-roadmap.md`
- Modify: `CHANGELOG.md`
- Modify: `Cargo.toml`

- [ ] **Step 1: Write failing benchmark and doc sync checklist**

```rust
use alpaca_data::{Client, stocks};
use criterion::{Criterion, criterion_group, criterion_main};
use tokio::runtime::Runtime;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn bench_stocks_latest_quote_local(c: &mut Criterion) {
    let runtime = Runtime::new().expect("runtime should build");
    let server = runtime.block_on(async {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/v2/stocks/AAPL/quotes/latest"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{"quote":{"t":"2026-04-03T12:00:00Z","ax":"V","ap":200.1,"as":1,"bx":"V","bp":200.0,"bs":1}}"#,
                "application/json",
            ))
            .mount(&server)
            .await;
        server
    });

    let client = Client::builder()
        .api_key("key")
        .secret_key("secret")
        .base_url(server.uri())
        .build()
        .expect("client should build");
    let stocks_client = client.stocks();

    c.bench_function("stocks/latest_quote_local", |b| {
        b.to_async(&runtime).iter(|| {
            let stocks_client = stocks_client.clone();
            async move {
                let _ = stocks_client
                    .latest_quote(stocks::LatestQuoteRequest {
                        symbol: "AAPL".into(),
                        feed: None,
                        currency: None,
                    })
                    .await
                    .expect("request should succeed");
            }
        })
    });
}

criterion_group!(stocks_benches, bench_stocks_latest_quote_local);
criterion_main!(stocks_benches);
```

文档检查清单：

```text
- README 补 stocks batch/single 方法矩阵与 phase 2 当前状态
- memory/api/README.md 补 stocks single endpoint 公开约定
- memory/core/system-map.md 补 stocks 真实实现与 test/bench 文件
- roadmap 把 Phase 2 标记为完成并把下一步推进到 Phase 3
- CHANGELOG 记录所有对外接口、测试、benchmark、文档和内部实现变化
```

- [ ] **Step 2: Run the final verification commands before docs sync**

Run: `cargo fmt --check`
Expected: PASS.

Run: `cargo test`
Expected: PASS, with live tests skipped unless `ALPACA_LIVE_TESTS=1`.

Run: `set -a && source .env && set +a && cargo test --test live_stocks_batch_historical -- --nocapture`
Expected: PASS.

Run: `set -a && source .env && set +a && cargo test --test live_stocks_single_historical -- --nocapture`
Expected: PASS.

Run: `set -a && source .env && set +a && cargo test --test live_stocks_latest_snapshot -- --nocapture`
Expected: PASS.

Run: `set -a && source .env && set +a && cargo test --test live_stocks_metadata -- --nocapture`
Expected: PASS.

Run: `cargo bench --bench stocks --no-run`
Expected: FAIL before benchmark target is wired.

- [ ] **Step 3: Add the benchmark and sync phase-completion docs**

```toml
[[bench]]
name = "stocks"
harness = false
```

```markdown
## Current Phase Status

- `Phase 2: Stocks` 已完成 batch / single 官方 endpoint 对应
- stocks historical batch / single 都已支持 `*_all` / `*_stream`
- stocks happy-path tests 以真实 Alpaca API 为主
- `benches/stocks.rs` 提供本地 micro-benchmark baseline
```

- [ ] **Step 4: Run the complete phase verification**

Run: `cargo fmt --check`
Expected: PASS.

Run: `cargo test`
Expected: PASS.

Run: `set -a && source .env && set +a && cargo test --test live_stocks_batch_historical -- --nocapture`
Expected: PASS.

Run: `set -a && source .env && set +a && cargo test --test live_stocks_single_historical -- --nocapture`
Expected: PASS.

Run: `set -a && source .env && set +a && cargo test --test live_stocks_latest_snapshot -- --nocapture`
Expected: PASS.

Run: `set -a && source .env && set +a && cargo test --test live_stocks_metadata -- --nocapture`
Expected: PASS.

Run: `cargo bench --bench stocks --no-run`
Expected: PASS.

Run: `git diff --stat main...HEAD`
Expected: show only Phase 2 stocks work.

- [ ] **Step 5: Commit the phase completion release**

```bash
git add Cargo.toml README.md AGENTS.md memory/README.md memory/api/README.md memory/core/system-map.md memory/core/workflows.md docs/superpowers/plans/2026-04-03-full-project-roadmap.md CHANGELOG.md benches/stocks.rs
git commit -m "chore: bump version and changelog (v0.2.0)"
```

## Final Verification Checklist

- [ ] `cargo fmt --check`
- [ ] `cargo test`
- [ ] `set -a && source .env && set +a && cargo test --test live_stocks_batch_historical -- --nocapture`
- [ ] `set -a && source .env && set +a && cargo test --test live_stocks_single_historical -- --nocapture`
- [ ] `set -a && source .env && set +a && cargo test --test live_stocks_latest_snapshot -- --nocapture`
- [ ] `set -a && source .env && set +a && cargo test --test live_stocks_metadata -- --nocapture`
- [ ] `cargo bench --bench stocks --no-run`
- [ ] 确认 mock 测试只覆盖异常路径
- [ ] 确认 batch / single endpoint 的请求字段与响应字段继续保持官方原词
- [ ] 确认 `Phase 2` 收尾版本提升为 `v0.2.0`
- [ ] 确认 phase 完成后合并 `main`、推送远端并删除当前开发分支

## Handoff Notes

- `stocks` 是后续资源域的模板模块，优先保持结构清晰和命名稳定
- 如果真实 API 响应和参考文档之间出现差异，以真实 API 返回为准，并同步修正文档
- `options` 和 `crypto` 后续 phase 应尽量复用本 phase 沉淀下来的 batch/single 命名模式、分页 helper 用法和 live test 结构
