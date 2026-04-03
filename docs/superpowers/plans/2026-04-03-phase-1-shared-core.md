# Phase 1 Shared Core Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 把 `alpaca-data` 的共享基础层做成可复用、可测试、可扩展的 async HTTP 核心，并至少打通一个真实 Alpaca Market Data API happy-path 请求。

**Architecture:** 这一 phase 先不追求把所有资源域做完，而是先把 `ClientBuilder -> Auth -> Endpoint Routing -> Query Serialization -> Http Transport -> Error Mapping -> Pagination Helpers` 这条通路打通。共享层完成后，用 `crypto.latest_quotes` 作为无凭证真实 API canary endpoint 验证整条链路，再把通用分页能力留给 stocks/options/news/corporate_actions 复用。

**Tech Stack:** Rust 2024 edition, `reqwest` async client with `rustls`, `tokio`, `serde`/`serde_json`, `futures-util`, `wiremock` for exceptional-path tests only, `criterion` for local benchmark baselines

---

## File Structure

- `Cargo.toml`
  - 增加 async HTTP、序列化、测试、benchmark 依赖。
  - 注册 `criterion` benchmark target。
- `src/client.rs`
  - 管理 `ClientBuilder`、共享 `Inner` runtime state、默认 data base URL、资源 accessor。
- `src/auth.rs`
  - 管理 `api_key`/`secret_key` 成对校验、header 注入、public/private 请求判定。
- `src/error.rs`
  - 定义共享 typed error，区分 config、transport、timeout、status、rate limit、deserialize、pagination。
- `src/common/mod.rs`
  - 导出共享 query/response/time/enums 工具。
- `src/common/query.rs`
  - 新建 query writer；负责只写入非空参数、处理 `symbols` 逗号拼接、统一官方单词输出。
- `src/common/enums.rs`
  - 放共享的 enum string 映射辅助 trait。
- `src/common/response.rs`
  - 把响应 stream 从 `Empty<T>` 升级成可返回分页 page 的 boxed stream 类型。
- `src/transport/mod.rs`
  - 导出 endpoint/http/pagination/rate_limit/retry 模块。
- `src/transport/endpoint.rs`
  - 新建 endpoint routing；用资源域 + endpoint 类型拼出官方 Market Data API 路径前缀。
- `src/transport/http.rs`
  - 封装 `reqwest::Client`、header/query/path 拼装、JSON decode、status/error 映射。
- `src/transport/retry.rs`
  - 定义 429/5xx 的重试策略和 retryable 判定。
- `src/transport/rate_limit.rs`
  - 定义每 client 可选并发限制，避免单 client 无限并发。
- `src/transport/pagination.rs`
  - 定义分页 request/response trait，以及 `collect_all` / `stream_pages` 驱动。
- `src/crypto/client.rs`
  - 先实现 `latest_quotes` 作为共享层 canary endpoint。
- `src/crypto/request.rs`
  - 让 `LatestQuotesRequest` 可供共享 query writer 使用。
- `src/crypto/response.rs`
  - 让 `LatestQuotesResponse` 可供 `serde` decode。
- `src/crypto/model.rs`
  - 先把 `Quote` 最小 happy-path 需要的字段做成可反序列化。
- `tests/client_builder.rs`
  - 覆盖 builder、partial credentials、public crypto client 基线。
- `tests/live_crypto_latest_quotes_smoke.rs`
  - 真实 API smoke test；默认需要 `ALPACA_LIVE_TESTS=1` 才运行。
- `tests/mock_transport_errors.rs`
  - 只覆盖 429、5xx、timeout、malformed JSON 等异常路径。
- `benches/shared_core.rs`
  - 建立 query 编码和分页 merge 的 benchmark 基线。
- `memory/core/system-map.md`
  - 在 phase 完成后补共享层结构说明。

## Environment Conventions

- Live tests 开关：`ALPACA_LIVE_TESTS=1`
- 认证型 live tests 凭证：
  - `APCA_API_KEY_ID`
  - `APCA_API_SECRET_KEY`
- 可选 data URL 覆盖：`APCA_API_DATA_URL`
- 默认 data URL：`https://data.alpaca.markets`

## Phase Exit Targets

- `ClientBuilder` 可生成无凭证 public crypto client，也可生成带凭证 client。
- 共享 `HttpClient` 已支持 query、auth header、JSON decode、typed error。
- `collect_all` / `stream_pages` 已可被后续资源模块复用。
- `crypto.latest_quotes` 已跑通真实 API smoke test。
- 异常路径测试仅使用 mock。
- `criterion` benchmark baseline 已建立。

### Task 1: Client Runtime and Auth Validation

**Files:**
- Modify: `Cargo.toml`
- Modify: `src/client.rs`
- Modify: `src/auth.rs`
- Modify: `src/error.rs`
- Test: `tests/client_builder.rs`

- [x] **Step 1: Write the failing builder/auth tests**

```rust
use alpaca_data::{Client, Error};

#[test]
fn builder_allows_public_crypto_only_clients() {
    let client = Client::builder().build().expect("public crypto client should build");
    let _ = client.crypto();
}

#[test]
fn builder_rejects_partial_credentials() {
    let error = Client::builder()
        .api_key("key-only")
        .build()
        .expect_err("partial credentials must fail");

    assert!(matches!(
        error,
        Error::InvalidConfiguration(message)
            if message.contains("api_key") && message.contains("secret_key")
    ));
}

#[test]
fn builder_accepts_explicit_shared_runtime_settings() {
    let client = Client::builder()
        .api_key("key")
        .secret_key("secret")
        .base_url("https://data.alpaca.markets")
        .timeout(std::time::Duration::from_secs(5))
        .max_retries(2)
        .max_in_flight(32)
        .build()
        .expect("configured client should build");

    let _ = client.stocks();
}
```

- [x] **Step 2: Run test to verify it fails**

Run: `cargo test --test client_builder -v`
Expected: FAIL because `timeout`, `max_retries`, `max_in_flight`, and `InvalidConfiguration` do not exist yet.

- [x] **Step 3: Write the minimal builder/auth implementation**

```rust
#[derive(Clone, Debug)]
pub struct ClientBuilder {
    api_key: Option<String>,
    secret_key: Option<String>,
    base_url: Option<String>,
    timeout: std::time::Duration,
    max_retries: u32,
    max_in_flight: Option<usize>,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            api_key: None,
            secret_key: None,
            base_url: None,
            timeout: std::time::Duration::from_secs(10),
            max_retries: 3,
            max_in_flight: None,
        }
    }
}

impl ClientBuilder {
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    pub fn max_in_flight(mut self, max_in_flight: usize) -> Self {
        self.max_in_flight = Some(max_in_flight);
        self
    }

    pub fn build(self) -> Result<Client, Error> {
        let auth = Auth::new(self.api_key, self.secret_key)?;
        let base_url = self
            .base_url
            .unwrap_or_else(|| "https://data.alpaca.markets".to_string());

        Ok(Client::from_parts(auth, base_url, self.timeout, self.max_retries, self.max_in_flight))
    }
}

impl Auth {
    pub(crate) fn new(
        api_key: Option<String>,
        secret_key: Option<String>,
    ) -> Result<Self, Error> {
        match (api_key, secret_key) {
            (Some(api_key), Some(secret_key)) => Ok(Self {
                api_key: Some(api_key),
                secret_key: Some(secret_key),
            }),
            (None, None) => Ok(Self::default()),
            _ => Err(Error::InvalidConfiguration(
                "api_key and secret_key must be paired".into(),
            )),
        }
    }
}
```

- [x] **Step 4: Run test to verify it passes**

Run: `cargo test --test client_builder -v`
Expected: PASS with 3 passed tests.

- [x] **Step 5: Commit**

```bash
git add Cargo.toml src/client.rs src/auth.rs src/error.rs tests/client_builder.rs
git commit -m "feat: add shared client builder runtime config"
```

### Task 2: Query Serialization and Endpoint Routing

**Files:**
- Modify: `src/common/mod.rs`
- Modify: `src/common/enums.rs`
- Modify: `src/transport/mod.rs`
- Create: `src/common/query.rs`
- Create: `src/transport/endpoint.rs`
- Test: `src/common/query.rs`
- Test: `src/transport/endpoint.rs`

- [ ] **Step 1: Write failing unit tests for query writing and path routing**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_writer_joins_symbols_and_skips_none() {
        let mut query = QueryWriter::default();
        query.push_csv("symbols", ["AAPL", "MSFT"]);
        query.push_opt("limit", Some(100u32));
        query.push_opt::<u32>("page_token", None);

        assert_eq!(
            query.finish(),
            vec![
                ("symbols".to_string(), "AAPL,MSFT".to_string()),
                ("limit".to_string(), "100".to_string()),
            ]
        );
    }

    #[test]
    fn endpoint_routes_crypto_latest_quotes_to_official_path() {
        let endpoint = Endpoint::crypto_latest_quotes(Loc::Us);
        assert_eq!(endpoint.path(), "/v1beta3/crypto/us/latest/quotes");
        assert!(!endpoint.requires_auth());
    }
}
```

- [ ] **Step 2: Run unit tests to verify they fail**

Run: `cargo test --lib -- --nocapture`
Expected: FAIL because `QueryWriter` and `Endpoint` are not implemented yet.

- [ ] **Step 3: Write the minimal query/endpoint implementation**

```rust
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct QueryWriter {
    pairs: Vec<(String, String)>,
}

impl QueryWriter {
    pub(crate) fn push_csv<I, S>(&mut self, key: &'static str, values: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = values
            .into_iter()
            .map(|value| value.as_ref().to_string())
            .collect::<Vec<_>>()
            .join(",");

        if !joined.is_empty() {
            self.pairs.push((key.to_string(), joined));
        }
    }

    pub(crate) fn push_opt<T>(&mut self, key: &'static str, value: Option<T>)
    where
        T: ToString,
    {
        if let Some(value) = value {
            self.pairs.push((key.to_string(), value.to_string()));
        }
    }

    pub(crate) fn finish(self) -> Vec<(String, String)> {
        self.pairs
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Endpoint {
    CryptoLatestQuotes { loc: Loc },
}

impl Endpoint {
    pub(crate) fn crypto_latest_quotes(loc: Loc) -> Self {
        Self::CryptoLatestQuotes { loc }
    }

    pub(crate) fn path(&self) -> &'static str {
        match self {
            Self::CryptoLatestQuotes { loc: Loc::Us } => "/v1beta3/crypto/us/latest/quotes",
            Self::CryptoLatestQuotes { loc: Loc::Us1 } => "/v1beta3/crypto/us1/latest/quotes",
        }
    }

    pub(crate) fn requires_auth(&self) -> bool {
        match self {
            Self::CryptoLatestQuotes { .. } => false,
        }
    }
}
```

- [ ] **Step 4: Run unit tests to verify they pass**

Run: `cargo test --lib -- --nocapture`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/common/mod.rs src/common/enums.rs src/common/query.rs src/transport/mod.rs src/transport/endpoint.rs
git commit -m "feat: add shared query writer and endpoint routing"
```

### Task 3: Async HTTP Transport, Retry, and Typed Error Mapping

**Files:**
- Modify: `Cargo.toml`
- Modify: `src/client.rs`
- Modify: `src/auth.rs`
- Modify: `src/error.rs`
- Modify: `src/transport/http.rs`
- Modify: `src/transport/retry.rs`
- Modify: `src/transport/rate_limit.rs`
- Test: `tests/mock_transport_errors.rs`

- [ ] **Step 1: Write failing exceptional-path tests with mock HTTP responses**

```rust
use alpaca_data::{Client, Error, crypto};
use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::{method, path};

#[tokio::test]
async fn rate_limit_maps_retry_after_header() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1beta3/crypto/us/latest/quotes"))
        .respond_with(
            ResponseTemplate::new(429)
                .insert_header("retry-after", "3")
                .set_body_string("too many requests"),
        )
        .mount(&server)
        .await;

    let error = Client::builder()
        .base_url(server.uri())
        .build()
        .expect("client should build")
        .crypto()
        .latest_quotes(crypto::LatestQuotesRequest {
            symbols: vec!["BTC/USD".into()],
            loc: Some(crypto::Loc::Us),
        })
        .await
        .expect_err("request should fail");

    assert!(matches!(
        error,
        Error::RateLimited {
            retry_after: Some(3),
            ..
        }
    ));
}

#[tokio::test]
async fn malformed_json_maps_deserialize_error() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1beta3/crypto/us/latest/quotes"))
        .respond_with(ResponseTemplate::new(200).set_body_raw("not-json", "application/json"))
        .mount(&server)
        .await;

    let error = Client::builder()
        .base_url(server.uri())
        .build()
        .expect("client should build")
        .crypto()
        .latest_quotes(crypto::LatestQuotesRequest {
            symbols: vec!["BTC/USD".into()],
            loc: Some(crypto::Loc::Us),
        })
        .await
        .expect_err("request should fail");

    assert!(matches!(error, Error::Deserialize(_)));
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test --test mock_transport_errors -v`
Expected: FAIL because `wiremock` is not configured and `crypto.latest_quotes` still returns `NotImplemented`.

- [ ] **Step 3: Implement `reqwest` transport, retry, and error mapping**

```rust
pub(crate) struct HttpClient {
    client: reqwest::Client,
    retry_policy: RetryPolicy,
    rate_limiter: RateLimiter,
}

impl HttpClient {
    pub(crate) async fn get_json<T>(
        &self,
        base_url: &str,
        endpoint: Endpoint,
        auth: &Auth,
        query: Vec<(String, String)>,
    ) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        let permit = self.rate_limiter.acquire().await?;
        let _permit = permit;

        let url = format!("{}{}", base_url.trim_end_matches('/'), endpoint.path());
        let mut attempt = 0;

        loop {
            let mut request = self.client.get(&url).query(&query);
            request = auth.apply(request, endpoint.requires_auth())?;

            let response = request.send().await.map_err(Error::from_reqwest)?;
            let status = response.status();
            let retry_after = parse_retry_after(response.headers());
            let body = response.text().await.map_err(Error::from_reqwest)?;

            if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                return Err(Error::RateLimited {
                    retry_after,
                    body: Some(body),
                });
            }

            if status.is_server_error() && self.retry_policy.should_retry(status, attempt) {
                attempt += 1;
                tokio::time::sleep(self.retry_policy.backoff(attempt)).await;
                continue;
            }

            if !status.is_success() {
                return Err(Error::HttpStatus {
                    status: status.as_u16(),
                    body: Some(body),
                });
            }

            return serde_json::from_str(&body).map_err(|error| Error::Deserialize(error.to_string()));
        }
    }
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test --test mock_transport_errors -v`
Expected: PASS with mock exceptional-path coverage only.

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml src/client.rs src/auth.rs src/error.rs src/transport/http.rs src/transport/retry.rs src/transport/rate_limit.rs tests/mock_transport_errors.rs
git commit -m "feat: add shared async transport and error mapping"
```

### Task 4: Pagination Collection and Page Streaming Helpers

**Files:**
- Modify: `src/common/response.rs`
- Modify: `src/transport/pagination.rs`
- Test: `src/transport/pagination.rs`

- [ ] **Step 1: Write failing pagination helper tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, Default, PartialEq)]
    struct FakeRequest {
        page_token: Option<String>,
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    struct FakeResponse {
        values: Vec<u32>,
        next_page_token: Option<String>,
    }

    #[tokio::test]
    async fn collect_all_merges_pages_and_clears_next_page_token() {
        let first = FakeResponse {
            values: vec![1, 2],
            next_page_token: Some("page-2".into()),
        };
        let second = FakeResponse {
            values: vec![3],
            next_page_token: None,
        };

        let response = collect_all(FakeRequest::default(), |request| async move {
            if request.page_token.as_deref() == Some("page-2") {
                Ok(second.clone())
            } else {
                Ok(first.clone())
            }
        })
        .await
        .expect("pagination should succeed");

        assert_eq!(response.values, vec![1, 2, 3]);
        assert_eq!(response.next_page_token, None);
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test collect_all_merges_pages_and_clears_next_page_token --lib -- --nocapture`
Expected: FAIL because pagination traits/helpers do not exist yet.

- [ ] **Step 3: Implement generic pagination traits and stream alias**

```rust
pub type ResponseStream<T> = std::pin::Pin<Box<dyn futures_util::Stream<Item = T> + Send>>;

pub(crate) trait PaginatedRequest: Clone {
    fn with_page_token(&self, page_token: Option<String>) -> Self;
}

pub(crate) trait PaginatedResponse: Sized {
    fn next_page_token(&self) -> Option<&str>;
    fn merge_page(&mut self, next: Self) -> Result<(), Error>;
    fn clear_next_page_token(&mut self);
}

pub(crate) async fn collect_all<Request, Response, Fetch, Fut>(
    request: Request,
    mut fetch: Fetch,
) -> Result<Response, Error>
where
    Request: PaginatedRequest,
    Response: PaginatedResponse,
    Fetch: FnMut(Request) -> Fut,
    Fut: std::future::Future<Output = Result<Response, Error>>,
{
    let mut current_request = request;
    let mut combined = fetch(current_request.clone()).await?;

    while let Some(next_page_token) = combined.next_page_token().map(str::to_owned) {
        current_request = current_request.with_page_token(Some(next_page_token));
        let page = fetch(current_request.clone()).await?;
        combined.merge_page(page)?;
    }

    combined.clear_next_page_token();
    Ok(combined)
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test collect_all_merges_pages_and_clears_next_page_token --lib -- --nocapture`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/common/response.rs src/transport/pagination.rs
git commit -m "feat: add shared pagination helpers"
```

### Task 5: Real-API Canary Endpoint via `crypto.latest_quotes`

**Files:**
- Modify: `src/crypto/client.rs`
- Modify: `src/crypto/request.rs`
- Modify: `src/crypto/response.rs`
- Modify: `src/crypto/model.rs`
- Modify: `tests/public_api.rs`
- Create: `tests/live_crypto_latest_quotes_smoke.rs`

- [ ] **Step 1: Write the failing public/live tests**

```rust
use alpaca_data::{Client, crypto};

#[tokio::test]
async fn crypto_latest_quotes_smoke_uses_real_api() {
    if std::env::var("ALPACA_LIVE_TESTS").as_deref() != Ok("1") {
        eprintln!("skipping live test; set ALPACA_LIVE_TESTS=1 to enable");
        return;
    }

    let response = Client::new()
        .crypto()
        .latest_quotes(crypto::LatestQuotesRequest {
            symbols: vec!["BTC/USD".into()],
            loc: Some(crypto::Loc::Us),
        })
        .await
        .expect("real Alpaca crypto latest quotes request should succeed");

    assert!(response.quotes.contains_key("BTC/USD"));
}
```

```rust
#[test]
fn crypto_client_exposes_latest_quotes_method() {
    let client = Client::new();
    let _ = client.crypto().latest_quotes(crypto::LatestQuotesRequest::default());
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test crypto_client_exposes_latest_quotes_method --test public_api -v`
Expected: FAIL if the public API test is added before implementation is wired.

Run: `ALPACA_LIVE_TESTS=1 cargo test --test live_crypto_latest_quotes_smoke -- --nocapture`
Expected: FAIL because `latest_quotes` still returns `NotImplemented`.

- [ ] **Step 3: Implement the minimal happy-path endpoint on top of shared transport**

```rust
impl CryptoClient {
    pub async fn latest_quotes(
        &self,
        request: LatestQuotesRequest,
    ) -> Result<LatestQuotesResponse, Error> {
        let endpoint = Endpoint::crypto_latest_quotes(request.loc.unwrap_or_default());
        let mut query = QueryWriter::default();
        query.push_csv("symbols", &request.symbols);

        self.inner
            .http
            .get_json(
                &self.inner.base_url,
                endpoint,
                &self.inner.auth,
                query.finish(),
            )
            .await
    }
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct LatestQuotesResponse {
    pub quotes: std::collections::HashMap<String, Quote>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct Quote {
    pub t: Option<Timestamp>,
    pub bp: Option<f64>,
    pub bs: Option<f64>,
    pub ap: Option<f64>,
    #[serde(rename = "as")]
    pub r#as: Option<f64>,
}
```

在这个实现步骤里，使用 raw identifier `r#as` 保持和官方字段 `as` 一一对应。

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test crypto_client_exposes_latest_quotes_method --test public_api -v`
Expected: PASS.

Run: `ALPACA_LIVE_TESTS=1 cargo test --test live_crypto_latest_quotes_smoke -- --nocapture`
Expected: PASS and the response contains `BTC/USD`.

- [ ] **Step 5: Commit**

```bash
git add src/crypto/client.rs src/crypto/request.rs src/crypto/response.rs src/crypto/model.rs tests/public_api.rs tests/live_crypto_latest_quotes_smoke.rs
git commit -m "feat: add crypto latest quotes canary endpoint"
```

### Task 6: Benchmark Baseline and Documentation Sync

**Files:**
- Modify: `Cargo.toml`
- Create: `benches/shared_core.rs`
- Modify: `README.md`
- Modify: `memory/core/system-map.md`
- Modify: `memory/core/workflows.md`

- [ ] **Step 1: Write the failing benchmark and doc TODO checklist**

```rust
use alpaca_data::{Client, crypto};
use criterion::{Criterion, criterion_group, criterion_main};
use tokio::runtime::Runtime;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn bench_crypto_latest_quotes_local(c: &mut Criterion) {
    let runtime = Runtime::new().expect("runtime should build");
    let server = runtime.block_on(async {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/v1beta3/crypto/us/latest/quotes"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{"quotes":{"BTC/USD":{"t":"2026-04-03T12:00:00Z","bp":1.0,"bs":1.0,"ap":1.1,"as":1.0}}}"#,
                "application/json",
            ))
            .mount(&server)
            .await;
        server
    });

    c.bench_function("shared_core/crypto_latest_quotes_local", |b| {
        b.to_async(&runtime).iter(|| async {
            let client = Client::builder()
                .base_url(server.uri())
                .build()
                .expect("client should build");

            let _ = client
                .crypto()
                .latest_quotes(crypto::LatestQuotesRequest {
                    symbols: vec!["BTC/USD".into()],
                    loc: Some(crypto::Loc::Us),
                })
                .await
                .expect("request should succeed");
        })
    });
}

criterion_group!(shared_core, bench_crypto_latest_quotes_local);
criterion_main!(shared_core);
```

文档检查清单：

```text
- README 补 shared core 当前能力和 live test 启用方法
- memory/core/system-map.md 补 transport/query/pagination 结构图
- memory/core/workflows.md 补 live test 与 mock test 的使用边界
```

- [ ] **Step 2: Run benchmark/doc commands to verify missing pieces**

Run: `cargo bench --bench shared_core --no-run`
Expected: FAIL before `criterion` target and benchmark file are wired.

- [ ] **Step 3: Implement the benchmark target and sync docs**

```toml
[dev-dependencies]
criterion = { version = "0.5", default-features = false, features = ["cargo_bench_support", "async_tokio"] }
wiremock = "0.6"
tokio = { version = "1", features = ["macros", "rt-multi-thread", "time", "sync"] }

[[bench]]
name = "shared_core"
harness = false
```

```markdown
## Current Shared Core Status

- `ClientBuilder` supports base URL, timeout, retry, and optional concurrency limits
- `crypto.latest_quotes` is the Phase 1 canary endpoint
- Happy-path tests use the real Alpaca API when `ALPACA_LIVE_TESTS=1`
- Mock tests are reserved for exceptional transport failures only
```

- [ ] **Step 4: Run full verification**

Run: `cargo fmt --check`
Expected: PASS.

Run: `cargo test`
Expected: PASS, with live tests skipped unless `ALPACA_LIVE_TESTS=1`.

Run: `cargo bench --bench shared_core --no-run`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml benches/shared_core.rs README.md memory/core/system-map.md memory/core/workflows.md
git commit -m "docs: record shared core workflow and benchmarks"
```

## Final Verification Checklist

- [ ] `cargo fmt --check`
- [ ] `cargo test`
- [ ] `ALPACA_LIVE_TESTS=1 cargo test --test live_crypto_latest_quotes_smoke -- --nocapture`
- [ ] `cargo bench --bench shared_core --no-run`
- [ ] 确认 mock 测试只覆盖异常路径
- [ ] 确认公开字段名继续保持官方原始单词
- [ ] 确认 `bars_all` / `bars_stream` 共用 helper 已可被后续资源模块复用

## Handoff Notes

- 本 phase 只把 `crypto.latest_quotes` 作为共享层 canary endpoint，避免过早把整块 `crypto` 域做完。
- `stocks` / `options` / `news` / `corporate_actions` 的实际 endpoint 批量实现放到后续 phase。
- 如果 `crypto.latest_quotes` 的官方返回结构与当前最小 `Quote` 模型不一致，优先扩字段，不要引入自定义字段名。
