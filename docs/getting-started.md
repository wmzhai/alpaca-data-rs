# Getting Started

## Current State

Install `alpaca-data` from crates.io:

```toml
[dependencies]
alpaca-data = "0.11.0"
rust_decimal = "1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## Create a Client

```rust
use alpaca_data::Client;

let client = Client::new();
let client = Client::builder().build()?;
# Ok::<(), alpaca_data::Error>(())
```

Use `Client::builder()` when you need explicit configuration:

- `api_key(...)`
- `secret_key(...)`
- `credentials_from_env()?`
- `credentials_from_env_names(...)?`
- `base_url(...)`
- `timeout(...)`
- `reqwest_client(...)`
- `observer(...)`
- `max_retries(...)`
- `retry_on_429(...)`
- `respect_retry_after(...)`
- `base_backoff(...)`
- `max_backoff(...)`
- `retry_jitter(...)`
- `total_retry_budget(...)`
- `max_in_flight(...)`

`ClientBuilder`, `Client`, and the resource clients redact configured
credentials in their `Debug` output by default. If `base_url(...)` includes URL
userinfo such as `https://user:pass@example.test`, `Debug` output removes that
userinfo and shows only the sanitized URL.

The retry builder defaults stay conservative: 5xx retries remain enabled within the retry budget, while 429 retries and `Retry-After` handling stay opt-in until you enable them explicitly.

Use `retry_jitter(...)` to add a bounded random delay on top of each computed retry wait so concurrent callers are less likely to retry in lockstep.

Use `total_retry_budget(...)` to bound the retry loop for one request against elapsed time. After the recent retry hardening, the remaining budget also caps each scheduled retry wait, including `Retry-After`-driven waits and waits with jitter enabled.

Use `reqwest_client(...)` when a service integration needs to own reqwest-level settings such as connection pooling, default headers, or timeout behavior. When you inject a custom client, configure those reqwest-level knobs on the injected client instead of combining them with `timeout(...)` on `ClientBuilder`.

Use `credentials_from_env()?` or `credentials_from_env_names(...)?` only as optional ergonomics when your runtime already manages paired environment variables. The primary credential path remains explicit `api_key(...)` plus `secret_key(...)`.

Whenever you provide credentials explicitly or through the env helpers, both
`api_key` and `secret_key` must be present or both omitted.
`ClientBuilder::build()` rejects blank, whitespace-only, and HTTP
header-invalid values before constructing the client.

Use `observer(...)` when you want successful-response metadata for logging or metrics. The observer sees endpoint name, URL, status, request ID, retry attempt count, and elapsed time, but it does not change request or response semantics.

## Choose a Resource

The root client exposes five resource entrypoints:

- `client.stocks()`
- `client.options()`
- `client.crypto()`
- `client.news()`
- `client.corporate_actions()`

## Understand the Two Layers

- Mirror layer: direct endpoint wrappers such as `stocks().bars(...)` or `news().list(...)`
- Convenience layer: pagination helpers such as `bars_all`, `bars_stream`, `list_all`, and `list_stream`

The convenience layer does not rename official fields. It only automates pagination.

Typed numeric public API fields and request filters use exact `rust_decimal::Decimal` values so price, strike, ratio, and rate values keep the precision and scale returned by Alpaca.

## Build Decimal-Based Requests

Use `rust_decimal::Decimal` for numeric request fields such as options chain strike filters:

```rust
use std::str::FromStr;

use alpaca_data::{Client, options};
use rust_decimal::Decimal;

# async fn example(client: &Client) -> Result<(), alpaca_data::Error> {
let response = client
    .options()
    .chain(options::ChainRequest {
        underlying_symbol: "AAPL".into(),
        feed: None,
        r#type: Some(options::ContractType::Call),
        strike_price_gte: Some(
            Decimal::from_str("180.0").expect("decimal literal should parse"),
        ),
        strike_price_lte: Some(
            Decimal::from_str("220.0").expect("decimal literal should parse"),
        ),
        expiration_date: None,
        expiration_date_gte: Some("2026-04-01".into()),
        expiration_date_lte: Some("2026-06-30".into()),
        root_symbol: None,
        updated_since: None,
        limit: Some(5),
        page_token: None,
    })
    .await?;
# let _ = response;
# Ok(())
# }
```

## Run Local Checks

```bash
cargo test
cargo bench --no-run
```

For live Alpaca integration tests, see [authentication.md](authentication.md) and [release-checklist.md](release-checklist.md).

## Browse The Documentation Site Locally

Regenerate the committed documentation artifacts:

```bash
./tools/docs/generate-doc-site
```

Install the Docusaurus dependencies once:

```bash
npm install --prefix website
```

Start the local docs site:

```bash
npm run start --prefix website
```

Build the production docs bundle:

```bash
npm run build --prefix website
```
