---
title: "Options"
description: "Options market data endpoints. Mirror methods cover historical bars and trades, latest quotes and trades, snapshots, chain lookups, and metadata endpoints. Convenience methods add:"
---

# Options

- Module path: `alpaca_data::options`
- Client type: `OptionsClient`
- docs.rs module: [https://docs.rs/alpaca-data/latest/alpaca_data/options/](https://docs.rs/alpaca-data/latest/alpaca_data/options/)
- Site rustdoc module: [https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/)
- Scope: Public API surface

Options market data endpoints. Mirror methods cover historical bars and trades, latest quotes and trades, snapshots, chain lookups, and metadata endpoints. Convenience methods add:

## Methods

| Method | Kind | Async | Request | Return | docs.rs | Site rustdoc |
| --- | --- | --- | --- | --- | --- | --- |
| `bars` | mirror | yes | `BarsRequest` | `Result<BarsResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/options/struct.OptionsClient.html#method.bars) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/struct.OptionsClient.html#method.bars) |
| `bars_all` | convenience | yes | `BarsRequest` | `Result<BarsResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/options/struct.OptionsClient.html#method.bars_all) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/struct.OptionsClient.html#method.bars_all) |
| `bars_stream` | convenience | no | `BarsRequest` | `ResponseStream<Result<BarsResponse, Error>>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/options/struct.OptionsClient.html#method.bars_stream) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/struct.OptionsClient.html#method.bars_stream) |
| `trades` | mirror | yes | `TradesRequest` | `Result<TradesResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/options/struct.OptionsClient.html#method.trades) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/struct.OptionsClient.html#method.trades) |
| `trades_all` | convenience | yes | `TradesRequest` | `Result<TradesResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/options/struct.OptionsClient.html#method.trades_all) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/struct.OptionsClient.html#method.trades_all) |
| `trades_stream` | convenience | no | `TradesRequest` | `ResponseStream<Result<TradesResponse, Error>>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/options/struct.OptionsClient.html#method.trades_stream) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/struct.OptionsClient.html#method.trades_stream) |
| `latest_quotes` | mirror | yes | `LatestQuotesRequest` | `Result<LatestQuotesResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/options/struct.OptionsClient.html#method.latest_quotes) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/struct.OptionsClient.html#method.latest_quotes) |
| `latest_trades` | mirror | yes | `LatestTradesRequest` | `Result<LatestTradesResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/options/struct.OptionsClient.html#method.latest_trades) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/struct.OptionsClient.html#method.latest_trades) |
| `snapshots` | mirror | yes | `SnapshotsRequest` | `Result<SnapshotsResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/options/struct.OptionsClient.html#method.snapshots) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/struct.OptionsClient.html#method.snapshots) |
| `snapshots_all` | convenience | yes | `SnapshotsRequest` | `Result<SnapshotsResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/options/struct.OptionsClient.html#method.snapshots_all) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/struct.OptionsClient.html#method.snapshots_all) |
| `snapshots_stream` | convenience | no | `SnapshotsRequest` | `ResponseStream<Result<SnapshotsResponse, Error>>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/options/struct.OptionsClient.html#method.snapshots_stream) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/struct.OptionsClient.html#method.snapshots_stream) |
| `chain` | mirror | yes | `ChainRequest` | `Result<ChainResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/options/struct.OptionsClient.html#method.chain) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/struct.OptionsClient.html#method.chain) |
| `chain_all` | convenience | yes | `ChainRequest` | `Result<ChainResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/options/struct.OptionsClient.html#method.chain_all) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/struct.OptionsClient.html#method.chain_all) |
| `chain_stream` | convenience | no | `ChainRequest` | `ResponseStream<Result<ChainResponse, Error>>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/options/struct.OptionsClient.html#method.chain_stream) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/struct.OptionsClient.html#method.chain_stream) |
| `exchange_codes` | mirror | yes | - | `Result<ExchangeCodesResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/options/struct.OptionsClient.html#method.exchange_codes) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/struct.OptionsClient.html#method.exchange_codes) |
| `condition_codes` | mirror | yes | `ConditionCodesRequest` | `Result<ConditionCodesResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/options/struct.OptionsClient.html#method.condition_codes) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/struct.OptionsClient.html#method.condition_codes) |

## Requests

### `BarsRequest`

- Kind: struct
- Summary: -
- docs.rs: [BarsRequest](https://docs.rs/alpaca-data/latest/alpaca_data/options/struct.BarsRequest.html)
- Site rustdoc: [BarsRequest](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/struct.BarsRequest.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `symbols` | `symbols` | `Vec<String>` | - |
| `timeframe` | `timeframe` | `TimeFrame` | - |
| `start` | `start` | `Option<String>` | - |
| `end` | `end` | `Option<String>` | - |
| `limit` | `limit` | `Option<u32>` | - |
| `sort` | `sort` | `Option<Sort>` | - |
| `page_token` | `page_token` | `Option<String>` | - |

### `ChainRequest`

- Kind: struct
- Summary: -
- docs.rs: [ChainRequest](https://docs.rs/alpaca-data/latest/alpaca_data/options/struct.ChainRequest.html)
- Site rustdoc: [ChainRequest](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/struct.ChainRequest.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `underlying_symbol` | `underlying_symbol` | `String` | - |
| `feed` | `feed` | `Option<OptionsFeed>` | - |
| `r#type` | `type` | `Option<ContractType>` | - |
| `strike_price_gte` | `strike_price_gte` | `Option<rust_decimal::Decimal>` | - |
| `strike_price_lte` | `strike_price_lte` | `Option<rust_decimal::Decimal>` | - |
| `expiration_date` | `expiration_date` | `Option<String>` | - |
| `expiration_date_gte` | `expiration_date_gte` | `Option<String>` | - |
| `expiration_date_lte` | `expiration_date_lte` | `Option<String>` | - |
| `root_symbol` | `root_symbol` | `Option<String>` | - |
| `updated_since` | `updated_since` | `Option<String>` | - |
| `limit` | `limit` | `Option<u32>` | - |
| `page_token` | `page_token` | `Option<String>` | - |

### `ConditionCodesRequest`

- Kind: struct
- Summary: -
- docs.rs: [ConditionCodesRequest](https://docs.rs/alpaca-data/latest/alpaca_data/options/struct.ConditionCodesRequest.html)
- Site rustdoc: [ConditionCodesRequest](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/struct.ConditionCodesRequest.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `ticktype` | `ticktype` | `TickType` | - |

### `LatestQuotesRequest`

- Kind: struct
- Summary: -
- docs.rs: [LatestQuotesRequest](https://docs.rs/alpaca-data/latest/alpaca_data/options/struct.LatestQuotesRequest.html)
- Site rustdoc: [LatestQuotesRequest](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/struct.LatestQuotesRequest.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `symbols` | `symbols` | `Vec<String>` | - |
| `feed` | `feed` | `Option<OptionsFeed>` | - |

### `LatestTradesRequest`

- Kind: struct
- Summary: -
- docs.rs: [LatestTradesRequest](https://docs.rs/alpaca-data/latest/alpaca_data/options/struct.LatestTradesRequest.html)
- Site rustdoc: [LatestTradesRequest](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/struct.LatestTradesRequest.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `symbols` | `symbols` | `Vec<String>` | - |
| `feed` | `feed` | `Option<OptionsFeed>` | - |

### `SnapshotsRequest`

- Kind: struct
- Summary: -
- docs.rs: [SnapshotsRequest](https://docs.rs/alpaca-data/latest/alpaca_data/options/struct.SnapshotsRequest.html)
- Site rustdoc: [SnapshotsRequest](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/struct.SnapshotsRequest.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `symbols` | `symbols` | `Vec<String>` | - |
| `feed` | `feed` | `Option<OptionsFeed>` | - |
| `limit` | `limit` | `Option<u32>` | - |
| `page_token` | `page_token` | `Option<String>` | - |

### `TradesRequest`

- Kind: struct
- Summary: -
- docs.rs: [TradesRequest](https://docs.rs/alpaca-data/latest/alpaca_data/options/struct.TradesRequest.html)
- Site rustdoc: [TradesRequest](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/struct.TradesRequest.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `symbols` | `symbols` | `Vec<String>` | - |
| `start` | `start` | `Option<String>` | - |
| `end` | `end` | `Option<String>` | - |
| `limit` | `limit` | `Option<u32>` | - |
| `sort` | `sort` | `Option<Sort>` | - |
| `page_token` | `page_token` | `Option<String>` | - |


## Responses

### `BarsResponse`

- Kind: struct
- Summary: -
- docs.rs: [BarsResponse](https://docs.rs/alpaca-data/latest/alpaca_data/options/struct.BarsResponse.html)
- Site rustdoc: [BarsResponse](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/struct.BarsResponse.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `bars` | `bars` | `HashMap<String, Vec<Bar>>` | - |
| `next_page_token` | `next_page_token` | `Option<String>` | - |

### `ChainResponse`

- Kind: struct
- Summary: -
- docs.rs: [ChainResponse](https://docs.rs/alpaca-data/latest/alpaca_data/options/struct.ChainResponse.html)
- Site rustdoc: [ChainResponse](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/struct.ChainResponse.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `snapshots` | `snapshots` | `HashMap<String, Snapshot>` | - |
| `next_page_token` | `next_page_token` | `Option<String>` | - |

### `ConditionCodesResponse`

- Kind: type
- Summary: -
- docs.rs: [ConditionCodesResponse](https://docs.rs/alpaca-data/latest/alpaca_data/options/type.ConditionCodesResponse.html)
- Site rustdoc: [ConditionCodesResponse](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/type.ConditionCodesResponse.html)
- Alias target: `HashMap<String, String>`

### `ExchangeCodesResponse`

- Kind: type
- Summary: -
- docs.rs: [ExchangeCodesResponse](https://docs.rs/alpaca-data/latest/alpaca_data/options/type.ExchangeCodesResponse.html)
- Site rustdoc: [ExchangeCodesResponse](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/type.ExchangeCodesResponse.html)
- Alias target: `HashMap<String, String>`

### `LatestQuotesResponse`

- Kind: struct
- Summary: -
- docs.rs: [LatestQuotesResponse](https://docs.rs/alpaca-data/latest/alpaca_data/options/struct.LatestQuotesResponse.html)
- Site rustdoc: [LatestQuotesResponse](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/struct.LatestQuotesResponse.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `quotes` | `quotes` | `HashMap<String, Quote>` | - |

### `LatestTradesResponse`

- Kind: struct
- Summary: -
- docs.rs: [LatestTradesResponse](https://docs.rs/alpaca-data/latest/alpaca_data/options/struct.LatestTradesResponse.html)
- Site rustdoc: [LatestTradesResponse](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/struct.LatestTradesResponse.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `trades` | `trades` | `HashMap<String, Trade>` | - |

### `SnapshotsResponse`

- Kind: struct
- Summary: -
- docs.rs: [SnapshotsResponse](https://docs.rs/alpaca-data/latest/alpaca_data/options/struct.SnapshotsResponse.html)
- Site rustdoc: [SnapshotsResponse](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/struct.SnapshotsResponse.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `snapshots` | `snapshots` | `HashMap<String, Snapshot>` | - |
| `next_page_token` | `next_page_token` | `Option<String>` | - |

### `TradesResponse`

- Kind: struct
- Summary: -
- docs.rs: [TradesResponse](https://docs.rs/alpaca-data/latest/alpaca_data/options/struct.TradesResponse.html)
- Site rustdoc: [TradesResponse](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/struct.TradesResponse.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `trades` | `trades` | `HashMap<String, Vec<Trade>>` | - |
| `next_page_token` | `next_page_token` | `Option<String>` | - |


## Models

### `Bar`

- Kind: struct
- Summary: -
- docs.rs: [Bar](https://docs.rs/alpaca-data/latest/alpaca_data/options/struct.Bar.html)
- Site rustdoc: [Bar](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/struct.Bar.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `t` | `t` | `Option<Timestamp>` | - |
| `o` | `o` | `Option<rust_decimal::Decimal>` | - |
| `h` | `h` | `Option<rust_decimal::Decimal>` | - |
| `l` | `l` | `Option<rust_decimal::Decimal>` | - |
| `c` | `c` | `Option<rust_decimal::Decimal>` | - |
| `v` | `v` | `Option<u64>` | - |
| `n` | `n` | `Option<u64>` | - |
| `vw` | `vw` | `Option<rust_decimal::Decimal>` | - |

### `Greeks`

- Kind: struct
- Summary: -
- docs.rs: [Greeks](https://docs.rs/alpaca-data/latest/alpaca_data/options/struct.Greeks.html)
- Site rustdoc: [Greeks](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/struct.Greeks.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `delta` | `delta` | `Option<rust_decimal::Decimal>` | - |
| `gamma` | `gamma` | `Option<rust_decimal::Decimal>` | - |
| `rho` | `rho` | `Option<rust_decimal::Decimal>` | - |
| `theta` | `theta` | `Option<rust_decimal::Decimal>` | - |
| `vega` | `vega` | `Option<rust_decimal::Decimal>` | - |

### `Quote`

- Kind: struct
- Summary: -
- docs.rs: [Quote](https://docs.rs/alpaca-data/latest/alpaca_data/options/struct.Quote.html)
- Site rustdoc: [Quote](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/struct.Quote.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `t` | `t` | `Option<Timestamp>` | - |
| `bx` | `bx` | `Option<String>` | - |
| `bp` | `bp` | `Option<rust_decimal::Decimal>` | - |
| `bs` | `bs` | `Option<u64>` | - |
| `ax` | `ax` | `Option<String>` | - |
| `ap` | `ap` | `Option<rust_decimal::Decimal>` | - |
| `r#as` | `as` | `Option<u64>` | - |
| `c` | `c` | `Option<String>` | - |

### `Snapshot`

- Kind: struct
- Summary: -
- docs.rs: [Snapshot](https://docs.rs/alpaca-data/latest/alpaca_data/options/struct.Snapshot.html)
- Site rustdoc: [Snapshot](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/struct.Snapshot.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `latestTrade` | `latestTrade` | `Option<Trade>` | - |
| `latestQuote` | `latestQuote` | `Option<Quote>` | - |
| `minuteBar` | `minuteBar` | `Option<Bar>` | - |
| `dailyBar` | `dailyBar` | `Option<Bar>` | - |
| `prevDailyBar` | `prevDailyBar` | `Option<Bar>` | - |
| `greeks` | `greeks` | `Option<Greeks>` | - |
| `impliedVolatility` | `impliedVolatility` | `Option<rust_decimal::Decimal>` | - |

### `Trade`

- Kind: struct
- Summary: -
- docs.rs: [Trade](https://docs.rs/alpaca-data/latest/alpaca_data/options/struct.Trade.html)
- Site rustdoc: [Trade](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/struct.Trade.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `t` | `t` | `Option<Timestamp>` | - |
| `x` | `x` | `Option<String>` | - |
| `p` | `p` | `Option<rust_decimal::Decimal>` | - |
| `s` | `s` | `Option<u64>` | - |
| `c` | `c` | `Option<String>` | - |


## Enums

### `ContractType`

- Kind: enum
- Summary: -
- docs.rs: [ContractType](https://docs.rs/alpaca-data/latest/alpaca_data/options/enum.ContractType.html)
- Site rustdoc: [ContractType](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/enum.ContractType.html)

| Variant | Official Value |
| --- | --- |
| `Call` | `call` |
| `Put` | `put` |

### `OptionsFeed`

- Kind: enum
- Summary: -
- docs.rs: [OptionsFeed](https://docs.rs/alpaca-data/latest/alpaca_data/options/enum.OptionsFeed.html)
- Site rustdoc: [OptionsFeed](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/enum.OptionsFeed.html)

| Variant | Official Value |
| --- | --- |
| `Opra` | `opra` |
| `Indicative` | `indicative` |

### `TickType`

- Kind: enum
- Summary: -
- docs.rs: [TickType](https://docs.rs/alpaca-data/latest/alpaca_data/options/enum.TickType.html)
- Site rustdoc: [TickType](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/enum.TickType.html)

| Variant | Official Value |
| --- | --- |
| `Trade` | `trade` |
| `Quote` | `quote` |

### `TimeFrame`

- Kind: struct
- Summary: -
- docs.rs: [TimeFrame](https://docs.rs/alpaca-data/latest/alpaca_data/options/struct.TimeFrame.html)
- Site rustdoc: [TimeFrame](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/options/struct.TimeFrame.html)
- Example constructors: `min_1 -> 1Min`, `day_1 -> 1Day`


## Related Repository Artifacts

- Examples: `examples/options_chain.rs`
- Tests: `tests/live_options_condition_codes.rs`, `tests/live_options_historical.rs`, `tests/live_options_latest_metadata.rs`, `tests/live_options_snapshots_chain.rs`, `tests/mock_options_errors.rs`
- Benchmarks: `benches/options.rs`

## Coverage Notes

- The strict endpoint parity ledger for this module lives in [API Coverage](../api-coverage.md).
- Generated reference pages mirror the shipped Rust surface and do not claim unimplemented Alpaca endpoints as available.
