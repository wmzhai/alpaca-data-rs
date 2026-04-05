# Authentication

## Credential Pairing

`ClientBuilder` enforces paired credentials:

- both `api_key` and `secret_key` must be present
- or both must be omitted

Supplying only one side is an invalid configuration error.

The primary application path is explicit builder credentials:

```rust
use alpaca_data::Client;

let client = Client::builder()
    .api_key(std::env::var("APCA_API_KEY_ID").expect("APCA_API_KEY_ID is required"))
    .secret_key(std::env::var("APCA_API_SECRET_KEY").expect("APCA_API_SECRET_KEY is required"))
    .build()?;
# let _ = client;
# Ok::<(), alpaca_data::Error>(())
```

## Optional Environment Helpers

When a service already injects credentials through process environment, `ClientBuilder` can load the pair directly:

```rust
use alpaca_data::Client;

let client = Client::builder()
    .credentials_from_env()?
    .build()?;
# let _ = client;
# Ok::<(), alpaca_data::Error>(())
```

Use `credentials_from_env_names(...)` when the surrounding runtime uses different variable names:

```rust
use alpaca_data::Client;

let client = Client::builder()
    .credentials_from_env_names("ALPACA_DATA_API_KEY", "ALPACA_DATA_SECRET_KEY")?
    .build()?;
# let _ = client;
# Ok::<(), alpaca_data::Error>(())
```

Environment helpers follow the same pairing rule:

- both variables present -> credentials are loaded onto the builder
- both variables absent -> the builder is left unchanged
- only one variable present -> `Error::InvalidConfiguration`

## Current Auth Rules

The current implementation requires credentials for:

- `stocks`
- `options`
- `news`
- `corporate_actions`

The currently implemented `crypto` HTTP endpoints can run without credentials.

This matches the current endpoint auth rules in the codebase and should be treated as an implementation fact, not a promise about future Alpaca policy changes.

## Environment Variables Used By Live Tests

Repository live tests expect:

- `APCA_API_KEY_ID`
- `APCA_API_SECRET_KEY`
- `ALPACA_LIVE_TESTS=1`

Optional override:

- `APCA_API_DATA_URL`

Example:

```bash
export APCA_API_KEY_ID="your-key"
export APCA_API_SECRET_KEY="your-secret"
export ALPACA_LIVE_TESTS=1
```

## Local `.env` Workflow

If your local `.env` stores credentials under names such as:

- `ALPACA_DATA_API_KEY`
- `ALPACA_DATA_SECRET_KEY`

Run the following exports before running live tests or examples:

```bash
export APCA_API_KEY_ID="$ALPACA_DATA_API_KEY"
export APCA_API_SECRET_KEY="$ALPACA_DATA_SECRET_KEY"
```

## Base URL Override

The client defaults to:

```text
https://data.alpaca.markets
```

Use `base_url(...)` only when you need a local test server or a different Alpaca-compatible endpoint.
