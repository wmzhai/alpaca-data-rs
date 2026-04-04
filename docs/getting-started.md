# Getting Started

## Current State

Install `alpaca-data` from crates.io:

```toml
[dependencies]
alpaca-data = "0.9.1"
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
- `base_url(...)`
- `timeout(...)`
- `max_retries(...)`
- `max_in_flight(...)`

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
