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
| `NewsList` | `NewsList` |
| `OptionsBars` | `OptionsBars` |
| `OptionsTrades` | `OptionsTrades` |
| `OptionsLatestQuotes` | `OptionsLatestQuotes` |
| `OptionsLatestTrades` | `OptionsLatestTrades` |
| `OptionsSnapshots` | `OptionsSnapshots` |
| `OptionsExchangeCodes` | `OptionsExchangeCodes` |
| `StocksBars` | `StocksBars` |
| `StocksAuctions` | `StocksAuctions` |
| `StocksQuotes` | `StocksQuotes` |
| `StocksTrades` | `StocksTrades` |
| `StocksLatestBars` | `StocksLatestBars` |
| `StocksLatestQuotes` | `StocksLatestQuotes` |
| `StocksLatestTrades` | `StocksLatestTrades` |
| `StocksSnapshots` | `StocksSnapshots` |
| `StocksExchangeCodes` | `StocksExchangeCodes` |
| `CorporateActionsList` | `CorporateActionsList` |

### `HttpClient`

- Kind: struct
- Summary: -
- docs.rs: [HttpClient](https://docs.rs/alpaca-data/latest/alpaca_data/transport/struct.HttpClient.html)
- Site rustdoc: [HttpClient](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/transport/struct.HttpClient.html)

### `RateLimiter`

- Kind: struct
- Summary: -
- docs.rs: [RateLimiter](https://docs.rs/alpaca-data/latest/alpaca_data/transport/struct.RateLimiter.html)
- Site rustdoc: [RateLimiter](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/transport/struct.RateLimiter.html)

### `RetryPolicy`

- Kind: struct
- Summary: -
- docs.rs: [RetryPolicy](https://docs.rs/alpaca-data/latest/alpaca_data/transport/struct.RetryPolicy.html)
- Site rustdoc: [RetryPolicy](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/transport/struct.RetryPolicy.html)


## Related Repository Artifacts

- Examples: -
- Tests: `tests/mock_transport_errors.rs`
- Benchmarks: `benches/shared_core.rs`

## Coverage Notes

- The strict endpoint parity ledger for this module lives in [API Coverage](../api-coverage.md).
- Generated reference pages mirror the shipped Rust surface and do not claim unimplemented Alpaca endpoints as available.
