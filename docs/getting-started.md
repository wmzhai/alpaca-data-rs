# Getting Started

## Current State

Install `alpaca-data` from crates.io:

```toml
[dependencies]
alpaca-data = "0.10.1"
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

The retry builder defaults stay conservative: 5xx retries remain enabled within the retry budget, while 429 retries and `Retry-After` handling stay opt-in until you enable them explicitly.

Use `retry_jitter(...)` to add a bounded random delay on top of each computed retry wait so concurrent callers are less likely to retry in lockstep.

Use `total_retry_budget(...)` to bound the retry loop for one request against elapsed time. After the recent retry hardening, the remaining budget also caps each scheduled retry wait, including `Retry-After`-driven waits and waits with jitter enabled.

Use `reqwest_client(...)` when a service integration needs to own reqwest-level settings such as connection pooling, default headers, or timeout behavior. When you inject a custom client, configure those reqwest-level knobs on the injected client instead of combining them with `timeout(...)` on `ClientBuilder`.

Use `credentials_from_env()?` or `credentials_from_env_names(...)?` only as optional ergonomics when your runtime already manages paired environment variables. The primary credential path remains explicit `api_key(...)` plus `secret_key(...)`.

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
