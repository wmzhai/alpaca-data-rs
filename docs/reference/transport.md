---
title: "Transport"
description: "Shared HTTP transport, endpoint routing, pagination, retry, and rate-limit infrastructure."
---

# Transport

- Module path: `alpaca_data::transport`
- Client type: internal-only module
- docs.rs module: [https://docs.rs/alpaca-data/latest/alpaca_data/transport/](https://docs.rs/alpaca-data/latest/alpaca_data/transport/)
- Site rustdoc module: [https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/transport/](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/transport/)
- Scope: Internal architecture support only

Shared HTTP transport, endpoint routing, pagination, retry, and rate-limit infrastructure.

## Shared Types

### `Endpoint`

- Kind: enum
- Summary: -
- docs.rs: [Endpoint](https://docs.rs/alpaca-data/latest/alpaca_data/transport/enum.Endpoint.html)
- Site rustdoc: [Endpoint](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/transport/enum.Endpoint.html)

| Variant | Official Value |
| --- | --- |
| `NewsList` | `news.list` |
| `OptionsBars` | `options.bars` |
| `OptionsTrades` | `options.trades` |
| `OptionsLatestQuotes` | `options.latest_quotes` |
| `OptionsLatestTrades` | `options.latest_trades` |
| `OptionsSnapshots` | `options.snapshots` |
| `OptionsExchangeCodes` | `options.exchange_codes` |
| `StocksBars` | `stocks.bars` |
| `StocksAuctions` | `stocks.auctions` |
| `StocksQuotes` | `stocks.quotes` |
| `StocksTrades` | `stocks.trades` |
| `StocksLatestBars` | `stocks.latest_bars` |
| `StocksLatestQuotes` | `stocks.latest_quotes` |
| `StocksLatestTrades` | `stocks.latest_trades` |
| `StocksSnapshots` | `stocks.snapshots` |
| `StocksExchangeCodes` | `stocks.exchange_codes` |
| `CorporateActionsList` | `corporate_actions.list` |

### `HttpClient`

- Kind: struct
- Summary: -
- docs.rs: [HttpClient](https://docs.rs/alpaca-data/latest/alpaca_data/transport/struct.HttpClient.html)
- Site rustdoc: [HttpClient](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/transport/struct.HttpClient.html)

### `ObservedResponseMeta`

- Kind: struct
- Summary: Read-only metadata emitted to [`TransportObserver`] implementations.
- docs.rs: [ObservedResponseMeta](https://docs.rs/alpaca-data/latest/alpaca_data/transport/struct.ObservedResponseMeta.html)
- Site rustdoc: [ObservedResponseMeta](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/transport/struct.ObservedResponseMeta.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `endpoint_name` | `endpoint_name` | `&'static str` | Stable endpoint name such as `stocks.latest_bars`. |
| `url` | `url` | `String` | Fully resolved request URL. |
| `status` | `status` | `u16` | Final HTTP status code. |
| `request_id` | `request_id` | `Option<String>` | Alpaca request identifier when the server returned one. |
| `attempt_count` | `attempt_count` | `u32` | Number of retry attempts that happened before this terminal response. |
| `elapsed` | `elapsed` | `Duration` | Total elapsed request time across retries. |

### `ObserverHandle`

- Kind: struct
- Summary: -
- docs.rs: [ObserverHandle](https://docs.rs/alpaca-data/latest/alpaca_data/transport/struct.ObserverHandle.html)
- Site rustdoc: [ObserverHandle](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/transport/struct.ObserverHandle.html)

### `RateLimiter`

- Kind: struct
- Summary: -
- docs.rs: [RateLimiter](https://docs.rs/alpaca-data/latest/alpaca_data/transport/struct.RateLimiter.html)
- Site rustdoc: [RateLimiter](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/transport/struct.RateLimiter.html)

### `ResponseMeta`

- Kind: struct
- Summary: -
- docs.rs: [ResponseMeta](https://docs.rs/alpaca-data/latest/alpaca_data/transport/struct.ResponseMeta.html)
- Site rustdoc: [ResponseMeta](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/transport/struct.ResponseMeta.html)

### `RetryConfig`

- Kind: struct
- Summary: -
- docs.rs: [RetryConfig](https://docs.rs/alpaca-data/latest/alpaca_data/transport/struct.RetryConfig.html)
- Site rustdoc: [RetryConfig](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/transport/struct.RetryConfig.html)

### `RetryDecision`

- Kind: enum
- Summary: -
- docs.rs: [RetryDecision](https://docs.rs/alpaca-data/latest/alpaca_data/transport/enum.RetryDecision.html)
- Site rustdoc: [RetryDecision](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/transport/enum.RetryDecision.html)

| Variant | Official Value |
| --- | --- |
| `DoNotRetry` | `DoNotRetry` |

### `TransportErrorMeta`

- Kind: struct
- Summary: -
- docs.rs: [TransportErrorMeta](https://docs.rs/alpaca-data/latest/alpaca_data/transport/struct.TransportErrorMeta.html)
- Site rustdoc: [TransportErrorMeta](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/transport/struct.TransportErrorMeta.html)


## Related Repository Artifacts

- Examples: -
- Tests: `tests/mock_transport_errors.rs`
- Benchmarks: `benches/shared_core.rs`

## Coverage Notes

- The strict endpoint parity ledger for this module lives in [API Coverage](../api-coverage.md).
- Generated reference pages mirror the shipped Rust surface and do not claim unimplemented Alpaca endpoints as available.
