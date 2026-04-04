# Authentication

## Credential Pairing

`ClientBuilder` enforces paired credentials:

- both `api_key` and `secret_key` must be present
- or both must be omitted

Supplying only one side is an invalid configuration error.

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

export them before running live tests or examples:

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
