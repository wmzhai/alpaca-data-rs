---
title: "Crypto"
description: "Crypto market data endpoints. Mirror methods cover historical bars, quotes, trades, latest data, latest orderbooks, and snapshots. Convenience methods add:"
---

# Crypto

- Module path: `alpaca_data::crypto`
- Client type: `CryptoClient`
- docs.rs module: [https://docs.rs/alpaca-data/latest/alpaca_data/crypto/](https://docs.rs/alpaca-data/latest/alpaca_data/crypto/)
- Site rustdoc module: [https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/)
- Scope: Public API surface

Crypto market data endpoints. Mirror methods cover historical bars, quotes, trades, latest data, latest orderbooks, and snapshots. Convenience methods add:

## Methods

| Method | Kind | Async | Request | Return | docs.rs | Site rustdoc |
| --- | --- | --- | --- | --- | --- | --- |
| `bars` | mirror | yes | `BarsRequest` | `Result<BarsResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/crypto/struct.CryptoClient.html#method.bars) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/struct.CryptoClient.html#method.bars) |
| `bars_all` | convenience | yes | `BarsRequest` | `Result<BarsResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/crypto/struct.CryptoClient.html#method.bars_all) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/struct.CryptoClient.html#method.bars_all) |
| `bars_stream` | convenience | no | `BarsRequest` | `ResponseStream<Result<BarsResponse, Error>>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/crypto/struct.CryptoClient.html#method.bars_stream) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/struct.CryptoClient.html#method.bars_stream) |
| `quotes` | mirror | yes | `QuotesRequest` | `Result<QuotesResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/crypto/struct.CryptoClient.html#method.quotes) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/struct.CryptoClient.html#method.quotes) |
| `quotes_all` | convenience | yes | `QuotesRequest` | `Result<QuotesResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/crypto/struct.CryptoClient.html#method.quotes_all) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/struct.CryptoClient.html#method.quotes_all) |
| `quotes_stream` | convenience | no | `QuotesRequest` | `ResponseStream<Result<QuotesResponse, Error>>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/crypto/struct.CryptoClient.html#method.quotes_stream) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/struct.CryptoClient.html#method.quotes_stream) |
| `trades` | mirror | yes | `TradesRequest` | `Result<TradesResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/crypto/struct.CryptoClient.html#method.trades) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/struct.CryptoClient.html#method.trades) |
| `trades_all` | convenience | yes | `TradesRequest` | `Result<TradesResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/crypto/struct.CryptoClient.html#method.trades_all) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/struct.CryptoClient.html#method.trades_all) |
| `trades_stream` | convenience | no | `TradesRequest` | `ResponseStream<Result<TradesResponse, Error>>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/crypto/struct.CryptoClient.html#method.trades_stream) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/struct.CryptoClient.html#method.trades_stream) |
| `latest_bars` | mirror | yes | `LatestBarsRequest` | `Result<LatestBarsResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/crypto/struct.CryptoClient.html#method.latest_bars) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/struct.CryptoClient.html#method.latest_bars) |
| `latest_quotes` | mirror | yes | `LatestQuotesRequest` | `Result<LatestQuotesResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/crypto/struct.CryptoClient.html#method.latest_quotes) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/struct.CryptoClient.html#method.latest_quotes) |
| `latest_trades` | mirror | yes | `LatestTradesRequest` | `Result<LatestTradesResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/crypto/struct.CryptoClient.html#method.latest_trades) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/struct.CryptoClient.html#method.latest_trades) |
| `latest_orderbooks` | mirror | yes | `LatestOrderbooksRequest` | `Result<LatestOrderbooksResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/crypto/struct.CryptoClient.html#method.latest_orderbooks) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/struct.CryptoClient.html#method.latest_orderbooks) |
| `snapshots` | mirror | yes | `SnapshotsRequest` | `Result<SnapshotsResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/crypto/struct.CryptoClient.html#method.snapshots) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/struct.CryptoClient.html#method.snapshots) |

## Requests

### `BarsRequest`

- Kind: struct
- Summary: -
- docs.rs: [BarsRequest](https://docs.rs/alpaca-data/latest/alpaca_data/crypto/struct.BarsRequest.html)
- Site rustdoc: [BarsRequest](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/struct.BarsRequest.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `symbols` | `symbols` | `Vec<String>` | - |
| `timeframe` | `timeframe` | `TimeFrame` | - |
| `start` | `start` | `Option<String>` | - |
| `end` | `end` | `Option<String>` | - |
| `limit` | `limit` | `Option<u32>` | - |
| `sort` | `sort` | `Option<Sort>` | - |
| `loc` | `loc` | `Option<Loc>` | - |
| `page_token` | `page_token` | `Option<String>` | - |

### `LatestBarsRequest`

- Kind: struct
- Summary: -
- docs.rs: [LatestBarsRequest](https://docs.rs/alpaca-data/latest/alpaca_data/crypto/struct.LatestBarsRequest.html)
- Site rustdoc: [LatestBarsRequest](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/struct.LatestBarsRequest.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `symbols` | `symbols` | `Vec<String>` | - |
| `loc` | `loc` | `Option<Loc>` | - |

### `LatestOrderbooksRequest`

- Kind: struct
- Summary: -
- docs.rs: [LatestOrderbooksRequest](https://docs.rs/alpaca-data/latest/alpaca_data/crypto/struct.LatestOrderbooksRequest.html)
- Site rustdoc: [LatestOrderbooksRequest](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/struct.LatestOrderbooksRequest.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `symbols` | `symbols` | `Vec<String>` | - |
| `loc` | `loc` | `Option<Loc>` | - |

### `LatestQuotesRequest`

- Kind: struct
- Summary: -
- docs.rs: [LatestQuotesRequest](https://docs.rs/alpaca-data/latest/alpaca_data/crypto/struct.LatestQuotesRequest.html)
- Site rustdoc: [LatestQuotesRequest](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/struct.LatestQuotesRequest.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `symbols` | `symbols` | `Vec<String>` | - |
| `loc` | `loc` | `Option<Loc>` | - |

### `LatestTradesRequest`

- Kind: struct
- Summary: -
- docs.rs: [LatestTradesRequest](https://docs.rs/alpaca-data/latest/alpaca_data/crypto/struct.LatestTradesRequest.html)
- Site rustdoc: [LatestTradesRequest](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/struct.LatestTradesRequest.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `symbols` | `symbols` | `Vec<String>` | - |
| `loc` | `loc` | `Option<Loc>` | - |

### `QuotesRequest`

- Kind: struct
- Summary: -
- docs.rs: [QuotesRequest](https://docs.rs/alpaca-data/latest/alpaca_data/crypto/struct.QuotesRequest.html)
- Site rustdoc: [QuotesRequest](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/struct.QuotesRequest.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `symbols` | `symbols` | `Vec<String>` | - |
| `start` | `start` | `Option<String>` | - |
| `end` | `end` | `Option<String>` | - |
| `limit` | `limit` | `Option<u32>` | - |
| `sort` | `sort` | `Option<Sort>` | - |
| `loc` | `loc` | `Option<Loc>` | - |
| `page_token` | `page_token` | `Option<String>` | - |

### `SnapshotsRequest`

- Kind: struct
- Summary: -
- docs.rs: [SnapshotsRequest](https://docs.rs/alpaca-data/latest/alpaca_data/crypto/struct.SnapshotsRequest.html)
- Site rustdoc: [SnapshotsRequest](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/struct.SnapshotsRequest.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `symbols` | `symbols` | `Vec<String>` | - |
| `loc` | `loc` | `Option<Loc>` | - |

### `TradesRequest`

- Kind: struct
- Summary: -
- docs.rs: [TradesRequest](https://docs.rs/alpaca-data/latest/alpaca_data/crypto/struct.TradesRequest.html)
- Site rustdoc: [TradesRequest](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/struct.TradesRequest.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `symbols` | `symbols` | `Vec<String>` | - |
| `start` | `start` | `Option<String>` | - |
| `end` | `end` | `Option<String>` | - |
| `limit` | `limit` | `Option<u32>` | - |
| `sort` | `sort` | `Option<Sort>` | - |
| `loc` | `loc` | `Option<Loc>` | - |
| `page_token` | `page_token` | `Option<String>` | - |


## Responses

### `BarsResponse`

- Kind: struct
- Summary: -
- docs.rs: [BarsResponse](https://docs.rs/alpaca-data/latest/alpaca_data/crypto/struct.BarsResponse.html)
- Site rustdoc: [BarsResponse](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/struct.BarsResponse.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `bars` | `bars` | `HashMap<String, Vec<Bar>>` | - |
| `next_page_token` | `next_page_token` | `Option<String>` | - |

### `LatestBarsResponse`

- Kind: struct
- Summary: -
- docs.rs: [LatestBarsResponse](https://docs.rs/alpaca-data/latest/alpaca_data/crypto/struct.LatestBarsResponse.html)
- Site rustdoc: [LatestBarsResponse](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/struct.LatestBarsResponse.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `bars` | `bars` | `HashMap<String, Bar>` | - |

### `LatestOrderbooksResponse`

- Kind: struct
- Summary: -
- docs.rs: [LatestOrderbooksResponse](https://docs.rs/alpaca-data/latest/alpaca_data/crypto/struct.LatestOrderbooksResponse.html)
- Site rustdoc: [LatestOrderbooksResponse](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/struct.LatestOrderbooksResponse.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `orderbooks` | `orderbooks` | `HashMap<String, Orderbook>` | - |

### `LatestQuotesResponse`

- Kind: struct
- Summary: -
- docs.rs: [LatestQuotesResponse](https://docs.rs/alpaca-data/latest/alpaca_data/crypto/struct.LatestQuotesResponse.html)
- Site rustdoc: [LatestQuotesResponse](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/struct.LatestQuotesResponse.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `quotes` | `quotes` | `HashMap<String, Quote>` | - |

### `LatestTradesResponse`

- Kind: struct
- Summary: -
- docs.rs: [LatestTradesResponse](https://docs.rs/alpaca-data/latest/alpaca_data/crypto/struct.LatestTradesResponse.html)
- Site rustdoc: [LatestTradesResponse](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/struct.LatestTradesResponse.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `trades` | `trades` | `HashMap<String, Trade>` | - |

### `QuotesResponse`

- Kind: struct
- Summary: -
- docs.rs: [QuotesResponse](https://docs.rs/alpaca-data/latest/alpaca_data/crypto/struct.QuotesResponse.html)
- Site rustdoc: [QuotesResponse](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/struct.QuotesResponse.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `quotes` | `quotes` | `HashMap<String, Vec<Quote>>` | - |
| `next_page_token` | `next_page_token` | `Option<String>` | - |

### `SnapshotsResponse`

- Kind: struct
- Summary: -
- docs.rs: [SnapshotsResponse](https://docs.rs/alpaca-data/latest/alpaca_data/crypto/struct.SnapshotsResponse.html)
- Site rustdoc: [SnapshotsResponse](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/struct.SnapshotsResponse.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `snapshots` | `snapshots` | `HashMap<String, Snapshot>` | - |

### `TradesResponse`

- Kind: struct
- Summary: -
- docs.rs: [TradesResponse](https://docs.rs/alpaca-data/latest/alpaca_data/crypto/struct.TradesResponse.html)
- Site rustdoc: [TradesResponse](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/struct.TradesResponse.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `trades` | `trades` | `HashMap<String, Vec<Trade>>` | - |
| `next_page_token` | `next_page_token` | `Option<String>` | - |


## Models

### `Bar`

- Kind: struct
- Summary: -
- docs.rs: [Bar](https://docs.rs/alpaca-data/latest/alpaca_data/crypto/struct.Bar.html)
- Site rustdoc: [Bar](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/struct.Bar.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `t` | `t` | `Option<Timestamp>` | - |
| `o` | `o` | `Option<f64>` | - |
| `h` | `h` | `Option<f64>` | - |
| `l` | `l` | `Option<f64>` | - |
| `c` | `c` | `Option<f64>` | - |
| `v` | `v` | `Option<f64>` | - |
| `n` | `n` | `Option<u64>` | - |
| `vw` | `vw` | `Option<f64>` | - |

### `Orderbook`

- Kind: struct
- Summary: -
- docs.rs: [Orderbook](https://docs.rs/alpaca-data/latest/alpaca_data/crypto/struct.Orderbook.html)
- Site rustdoc: [Orderbook](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/struct.Orderbook.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `t` | `t` | `Option<Timestamp>` | - |
| `b` | `b` | `Option<Vec<OrderbookLevel>>` | - |
| `a` | `a` | `Option<Vec<OrderbookLevel>>` | - |

### `OrderbookLevel`

- Kind: struct
- Summary: -
- docs.rs: [OrderbookLevel](https://docs.rs/alpaca-data/latest/alpaca_data/crypto/struct.OrderbookLevel.html)
- Site rustdoc: [OrderbookLevel](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/struct.OrderbookLevel.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `p` | `p` | `Option<f64>` | - |
| `s` | `s` | `Option<f64>` | - |

### `Quote`

- Kind: struct
- Summary: -
- docs.rs: [Quote](https://docs.rs/alpaca-data/latest/alpaca_data/crypto/struct.Quote.html)
- Site rustdoc: [Quote](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/struct.Quote.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `t` | `t` | `Option<Timestamp>` | - |
| `bp` | `bp` | `Option<f64>` | - |
| `bs` | `bs` | `Option<f64>` | - |
| `ap` | `ap` | `Option<f64>` | - |
| `r#as` | `as` | `Option<f64>` | - |

### `Snapshot`

- Kind: struct
- Summary: -
- docs.rs: [Snapshot](https://docs.rs/alpaca-data/latest/alpaca_data/crypto/struct.Snapshot.html)
- Site rustdoc: [Snapshot](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/struct.Snapshot.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `latestTrade` | `latestTrade` | `Option<Trade>` | - |
| `latestQuote` | `latestQuote` | `Option<Quote>` | - |
| `minuteBar` | `minuteBar` | `Option<Bar>` | - |
| `dailyBar` | `dailyBar` | `Option<Bar>` | - |
| `prevDailyBar` | `prevDailyBar` | `Option<Bar>` | - |

### `Trade`

- Kind: struct
- Summary: -
- docs.rs: [Trade](https://docs.rs/alpaca-data/latest/alpaca_data/crypto/struct.Trade.html)
- Site rustdoc: [Trade](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/struct.Trade.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `t` | `t` | `Option<Timestamp>` | - |
| `p` | `p` | `Option<f64>` | - |
| `s` | `s` | `Option<f64>` | - |
| `i` | `i` | `Option<u64>` | - |
| `tks` | `tks` | `Option<String>` | - |


## Enums

### `Loc`

- Kind: enum
- Summary: -
- docs.rs: [Loc](https://docs.rs/alpaca-data/latest/alpaca_data/crypto/enum.Loc.html)
- Site rustdoc: [Loc](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/enum.Loc.html)

| Variant | Official Value |
| --- | --- |
| `Us` | `us` |
| `Us1` | `us-1` |
| `Us2` | `us-2` |
| `Eu1` | `eu-1` |
| `Bs1` | `bs-1` |

### `TimeFrame`

- Kind: struct
- Summary: -
- docs.rs: [TimeFrame](https://docs.rs/alpaca-data/latest/alpaca_data/crypto/struct.TimeFrame.html)
- Site rustdoc: [TimeFrame](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/crypto/struct.TimeFrame.html)
- Example constructors: `min_1 -> 1Min`, `day_1 -> 1Day`


## Related Repository Artifacts

- Examples: `examples/crypto_latest_quotes.rs`
- Tests: `tests/live_crypto_historical.rs`, `tests/live_crypto_latest.rs`, `tests/live_crypto_latest_quotes_smoke.rs`, `tests/live_crypto_loc_variants.rs`, `tests/live_crypto_snapshots.rs`, `tests/mock_crypto_errors.rs`
- Benchmarks: `benches/crypto.rs`

## Coverage Notes

- The strict endpoint parity ledger for this module lives in [API Coverage](../api-coverage.md).
- Generated reference pages mirror the shipped Rust surface and do not claim unimplemented Alpaca endpoints as available.
