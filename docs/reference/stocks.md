---
title: "Stocks"
description: "Stock market data endpoints. Mirror methods cover historical batch and single-symbol endpoints, latest endpoints, auction history, snapshots, and metadata endpoints. Convenience methods add:"
---

# Stocks

- Module path: `alpaca_data::stocks`
- Client type: `StocksClient`
- docs.rs module: [https://docs.rs/alpaca-data/latest/alpaca_data/stocks/](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/)
- Site rustdoc module: [https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/)
- Scope: Public API surface

Stock market data endpoints. Mirror methods cover historical batch and single-symbol endpoints, latest endpoints, auction history, snapshots, and metadata endpoints. Convenience methods add:

## Methods

| Method | Kind | Async | Request | Return | docs.rs | Site rustdoc |
| --- | --- | --- | --- | --- | --- | --- |
| `bars` | mirror | yes | `BarsRequest` | `Result<BarsResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.StocksClient.html#method.bars) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.StocksClient.html#method.bars) |
| `auctions` | mirror | yes | `AuctionsRequest` | `Result<AuctionsResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.StocksClient.html#method.auctions) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.StocksClient.html#method.auctions) |
| `auctions_all` | convenience | yes | `AuctionsRequest` | `Result<AuctionsResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.StocksClient.html#method.auctions_all) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.StocksClient.html#method.auctions_all) |
| `auctions_single` | mirror | yes | `AuctionsSingleRequest` | `Result<AuctionsSingleResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.StocksClient.html#method.auctions_single) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.StocksClient.html#method.auctions_single) |
| `auctions_single_all` | convenience | yes | `AuctionsSingleRequest` | `Result<AuctionsSingleResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.StocksClient.html#method.auctions_single_all) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.StocksClient.html#method.auctions_single_all) |
| `auctions_stream` | convenience | no | `AuctionsRequest` | `ResponseStream<Result<AuctionsResponse, Error>>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.StocksClient.html#method.auctions_stream) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.StocksClient.html#method.auctions_stream) |
| `auctions_single_stream` | convenience | no | `AuctionsSingleRequest` | `ResponseStream<Result<AuctionsSingleResponse, Error>>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.StocksClient.html#method.auctions_single_stream) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.StocksClient.html#method.auctions_single_stream) |
| `bars_all` | convenience | yes | `BarsRequest` | `Result<BarsResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.StocksClient.html#method.bars_all) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.StocksClient.html#method.bars_all) |
| `bars_single` | mirror | yes | `BarsSingleRequest` | `Result<BarsSingleResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.StocksClient.html#method.bars_single) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.StocksClient.html#method.bars_single) |
| `bars_single_all` | convenience | yes | `BarsSingleRequest` | `Result<BarsSingleResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.StocksClient.html#method.bars_single_all) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.StocksClient.html#method.bars_single_all) |
| `bars_stream` | convenience | no | `BarsRequest` | `ResponseStream<Result<BarsResponse, Error>>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.StocksClient.html#method.bars_stream) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.StocksClient.html#method.bars_stream) |
| `bars_single_stream` | convenience | no | `BarsSingleRequest` | `ResponseStream<Result<BarsSingleResponse, Error>>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.StocksClient.html#method.bars_single_stream) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.StocksClient.html#method.bars_single_stream) |
| `quotes` | mirror | yes | `QuotesRequest` | `Result<QuotesResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.StocksClient.html#method.quotes) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.StocksClient.html#method.quotes) |
| `quotes_all` | convenience | yes | `QuotesRequest` | `Result<QuotesResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.StocksClient.html#method.quotes_all) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.StocksClient.html#method.quotes_all) |
| `quotes_single` | mirror | yes | `QuotesSingleRequest` | `Result<QuotesSingleResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.StocksClient.html#method.quotes_single) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.StocksClient.html#method.quotes_single) |
| `quotes_single_all` | convenience | yes | `QuotesSingleRequest` | `Result<QuotesSingleResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.StocksClient.html#method.quotes_single_all) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.StocksClient.html#method.quotes_single_all) |
| `quotes_stream` | convenience | no | `QuotesRequest` | `ResponseStream<Result<QuotesResponse, Error>>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.StocksClient.html#method.quotes_stream) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.StocksClient.html#method.quotes_stream) |
| `quotes_single_stream` | convenience | no | `QuotesSingleRequest` | `ResponseStream<Result<QuotesSingleResponse, Error>>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.StocksClient.html#method.quotes_single_stream) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.StocksClient.html#method.quotes_single_stream) |
| `trades` | mirror | yes | `TradesRequest` | `Result<TradesResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.StocksClient.html#method.trades) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.StocksClient.html#method.trades) |
| `trades_all` | convenience | yes | `TradesRequest` | `Result<TradesResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.StocksClient.html#method.trades_all) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.StocksClient.html#method.trades_all) |
| `trades_single` | mirror | yes | `TradesSingleRequest` | `Result<TradesSingleResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.StocksClient.html#method.trades_single) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.StocksClient.html#method.trades_single) |
| `trades_single_all` | convenience | yes | `TradesSingleRequest` | `Result<TradesSingleResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.StocksClient.html#method.trades_single_all) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.StocksClient.html#method.trades_single_all) |
| `trades_stream` | convenience | no | `TradesRequest` | `ResponseStream<Result<TradesResponse, Error>>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.StocksClient.html#method.trades_stream) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.StocksClient.html#method.trades_stream) |
| `trades_single_stream` | convenience | no | `TradesSingleRequest` | `ResponseStream<Result<TradesSingleResponse, Error>>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.StocksClient.html#method.trades_single_stream) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.StocksClient.html#method.trades_single_stream) |
| `latest_bars` | mirror | yes | `LatestBarsRequest` | `Result<LatestBarsResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.StocksClient.html#method.latest_bars) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.StocksClient.html#method.latest_bars) |
| `latest_bar` | mirror | yes | `LatestBarRequest` | `Result<LatestBarResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.StocksClient.html#method.latest_bar) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.StocksClient.html#method.latest_bar) |
| `latest_quotes` | mirror | yes | `LatestQuotesRequest` | `Result<LatestQuotesResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.StocksClient.html#method.latest_quotes) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.StocksClient.html#method.latest_quotes) |
| `latest_quote` | mirror | yes | `LatestQuoteRequest` | `Result<LatestQuoteResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.StocksClient.html#method.latest_quote) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.StocksClient.html#method.latest_quote) |
| `latest_trades` | mirror | yes | `LatestTradesRequest` | `Result<LatestTradesResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.StocksClient.html#method.latest_trades) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.StocksClient.html#method.latest_trades) |
| `latest_trade` | mirror | yes | `LatestTradeRequest` | `Result<LatestTradeResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.StocksClient.html#method.latest_trade) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.StocksClient.html#method.latest_trade) |
| `snapshots` | mirror | yes | `SnapshotsRequest` | `Result<SnapshotsResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.StocksClient.html#method.snapshots) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.StocksClient.html#method.snapshots) |
| `snapshot` | mirror | yes | `SnapshotRequest` | `Result<SnapshotResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.StocksClient.html#method.snapshot) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.StocksClient.html#method.snapshot) |
| `condition_codes` | mirror | yes | `ConditionCodesRequest` | `Result<ConditionCodesResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.StocksClient.html#method.condition_codes) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.StocksClient.html#method.condition_codes) |
| `exchange_codes` | mirror | yes | - | `Result<ExchangeCodesResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.StocksClient.html#method.exchange_codes) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.StocksClient.html#method.exchange_codes) |

## Requests

### `AuctionsRequest`

- Kind: struct
- Summary: -
- docs.rs: [AuctionsRequest](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.AuctionsRequest.html)
- Site rustdoc: [AuctionsRequest](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.AuctionsRequest.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `symbols` | `symbols` | `Vec<String>` | - |
| `start` | `start` | `Option<String>` | - |
| `end` | `end` | `Option<String>` | - |
| `limit` | `limit` | `Option<u32>` | - |
| `asof` | `asof` | `Option<String>` | - |
| `feed` | `feed` | `Option<AuctionFeed>` | - |
| `currency` | `currency` | `Option<Currency>` | - |
| `page_token` | `page_token` | `Option<String>` | - |
| `sort` | `sort` | `Option<Sort>` | - |

### `AuctionsSingleRequest`

- Kind: struct
- Summary: -
- docs.rs: [AuctionsSingleRequest](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.AuctionsSingleRequest.html)
- Site rustdoc: [AuctionsSingleRequest](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.AuctionsSingleRequest.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `symbol` | `symbol` | `String` | - |
| `start` | `start` | `Option<String>` | - |
| `end` | `end` | `Option<String>` | - |
| `limit` | `limit` | `Option<u32>` | - |
| `asof` | `asof` | `Option<String>` | - |
| `feed` | `feed` | `Option<AuctionFeed>` | - |
| `currency` | `currency` | `Option<Currency>` | - |
| `page_token` | `page_token` | `Option<String>` | - |
| `sort` | `sort` | `Option<Sort>` | - |

### `BarsRequest`

- Kind: struct
- Summary: -
- docs.rs: [BarsRequest](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.BarsRequest.html)
- Site rustdoc: [BarsRequest](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.BarsRequest.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `symbols` | `symbols` | `Vec<String>` | - |
| `timeframe` | `timeframe` | `TimeFrame` | - |
| `start` | `start` | `Option<String>` | - |
| `end` | `end` | `Option<String>` | - |
| `limit` | `limit` | `Option<u32>` | - |
| `adjustment` | `adjustment` | `Option<Adjustment>` | - |
| `feed` | `feed` | `Option<DataFeed>` | - |
| `sort` | `sort` | `Option<Sort>` | - |
| `asof` | `asof` | `Option<String>` | - |
| `currency` | `currency` | `Option<Currency>` | - |
| `page_token` | `page_token` | `Option<String>` | - |

### `BarsSingleRequest`

- Kind: struct
- Summary: -
- docs.rs: [BarsSingleRequest](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.BarsSingleRequest.html)
- Site rustdoc: [BarsSingleRequest](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.BarsSingleRequest.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `symbol` | `symbol` | `String` | - |
| `timeframe` | `timeframe` | `TimeFrame` | - |
| `start` | `start` | `Option<String>` | - |
| `end` | `end` | `Option<String>` | - |
| `limit` | `limit` | `Option<u32>` | - |
| `adjustment` | `adjustment` | `Option<Adjustment>` | - |
| `feed` | `feed` | `Option<DataFeed>` | - |
| `sort` | `sort` | `Option<Sort>` | - |
| `asof` | `asof` | `Option<String>` | - |
| `currency` | `currency` | `Option<Currency>` | - |
| `page_token` | `page_token` | `Option<String>` | - |

### `ConditionCodesRequest`

- Kind: struct
- Summary: -
- docs.rs: [ConditionCodesRequest](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.ConditionCodesRequest.html)
- Site rustdoc: [ConditionCodesRequest](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.ConditionCodesRequest.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `ticktype` | `ticktype` | `TickType` | - |
| `tape` | `tape` | `Tape` | - |

### `LatestBarRequest`

- Kind: struct
- Summary: -
- docs.rs: [LatestBarRequest](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.LatestBarRequest.html)
- Site rustdoc: [LatestBarRequest](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.LatestBarRequest.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `symbol` | `symbol` | `String` | - |
| `feed` | `feed` | `Option<DataFeed>` | - |
| `currency` | `currency` | `Option<Currency>` | - |

### `LatestBarsRequest`

- Kind: struct
- Summary: -
- docs.rs: [LatestBarsRequest](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.LatestBarsRequest.html)
- Site rustdoc: [LatestBarsRequest](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.LatestBarsRequest.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `symbols` | `symbols` | `Vec<String>` | - |
| `feed` | `feed` | `Option<DataFeed>` | - |
| `currency` | `currency` | `Option<Currency>` | - |

### `LatestQuoteRequest`

- Kind: struct
- Summary: -
- docs.rs: [LatestQuoteRequest](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.LatestQuoteRequest.html)
- Site rustdoc: [LatestQuoteRequest](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.LatestQuoteRequest.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `symbol` | `symbol` | `String` | - |
| `feed` | `feed` | `Option<DataFeed>` | - |
| `currency` | `currency` | `Option<Currency>` | - |

### `LatestQuotesRequest`

- Kind: struct
- Summary: -
- docs.rs: [LatestQuotesRequest](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.LatestQuotesRequest.html)
- Site rustdoc: [LatestQuotesRequest](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.LatestQuotesRequest.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `symbols` | `symbols` | `Vec<String>` | - |
| `feed` | `feed` | `Option<DataFeed>` | - |
| `currency` | `currency` | `Option<Currency>` | - |

### `LatestTradeRequest`

- Kind: struct
- Summary: -
- docs.rs: [LatestTradeRequest](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.LatestTradeRequest.html)
- Site rustdoc: [LatestTradeRequest](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.LatestTradeRequest.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `symbol` | `symbol` | `String` | - |
| `feed` | `feed` | `Option<DataFeed>` | - |
| `currency` | `currency` | `Option<Currency>` | - |

### `LatestTradesRequest`

- Kind: struct
- Summary: -
- docs.rs: [LatestTradesRequest](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.LatestTradesRequest.html)
- Site rustdoc: [LatestTradesRequest](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.LatestTradesRequest.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `symbols` | `symbols` | `Vec<String>` | - |
| `feed` | `feed` | `Option<DataFeed>` | - |
| `currency` | `currency` | `Option<Currency>` | - |

### `QuotesRequest`

- Kind: struct
- Summary: -
- docs.rs: [QuotesRequest](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.QuotesRequest.html)
- Site rustdoc: [QuotesRequest](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.QuotesRequest.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `symbols` | `symbols` | `Vec<String>` | - |
| `start` | `start` | `Option<String>` | - |
| `end` | `end` | `Option<String>` | - |
| `limit` | `limit` | `Option<u32>` | - |
| `feed` | `feed` | `Option<DataFeed>` | - |
| `sort` | `sort` | `Option<Sort>` | - |
| `asof` | `asof` | `Option<String>` | - |
| `currency` | `currency` | `Option<Currency>` | - |
| `page_token` | `page_token` | `Option<String>` | - |

### `QuotesSingleRequest`

- Kind: struct
- Summary: -
- docs.rs: [QuotesSingleRequest](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.QuotesSingleRequest.html)
- Site rustdoc: [QuotesSingleRequest](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.QuotesSingleRequest.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `symbol` | `symbol` | `String` | - |
| `start` | `start` | `Option<String>` | - |
| `end` | `end` | `Option<String>` | - |
| `limit` | `limit` | `Option<u32>` | - |
| `feed` | `feed` | `Option<DataFeed>` | - |
| `sort` | `sort` | `Option<Sort>` | - |
| `asof` | `asof` | `Option<String>` | - |
| `currency` | `currency` | `Option<Currency>` | - |
| `page_token` | `page_token` | `Option<String>` | - |

### `SnapshotRequest`

- Kind: struct
- Summary: -
- docs.rs: [SnapshotRequest](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.SnapshotRequest.html)
- Site rustdoc: [SnapshotRequest](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.SnapshotRequest.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `symbol` | `symbol` | `String` | - |
| `feed` | `feed` | `Option<DataFeed>` | - |
| `currency` | `currency` | `Option<Currency>` | - |

### `SnapshotsRequest`

- Kind: struct
- Summary: -
- docs.rs: [SnapshotsRequest](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.SnapshotsRequest.html)
- Site rustdoc: [SnapshotsRequest](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.SnapshotsRequest.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `symbols` | `symbols` | `Vec<String>` | - |
| `feed` | `feed` | `Option<DataFeed>` | - |
| `currency` | `currency` | `Option<Currency>` | - |

### `TradesRequest`

- Kind: struct
- Summary: -
- docs.rs: [TradesRequest](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.TradesRequest.html)
- Site rustdoc: [TradesRequest](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.TradesRequest.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `symbols` | `symbols` | `Vec<String>` | - |
| `start` | `start` | `Option<String>` | - |
| `end` | `end` | `Option<String>` | - |
| `limit` | `limit` | `Option<u32>` | - |
| `feed` | `feed` | `Option<DataFeed>` | - |
| `sort` | `sort` | `Option<Sort>` | - |
| `asof` | `asof` | `Option<String>` | - |
| `currency` | `currency` | `Option<Currency>` | - |
| `page_token` | `page_token` | `Option<String>` | - |

### `TradesSingleRequest`

- Kind: struct
- Summary: -
- docs.rs: [TradesSingleRequest](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.TradesSingleRequest.html)
- Site rustdoc: [TradesSingleRequest](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.TradesSingleRequest.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `symbol` | `symbol` | `String` | - |
| `start` | `start` | `Option<String>` | - |
| `end` | `end` | `Option<String>` | - |
| `limit` | `limit` | `Option<u32>` | - |
| `feed` | `feed` | `Option<DataFeed>` | - |
| `sort` | `sort` | `Option<Sort>` | - |
| `asof` | `asof` | `Option<String>` | - |
| `currency` | `currency` | `Option<Currency>` | - |
| `page_token` | `page_token` | `Option<String>` | - |


## Responses

### `AuctionsResponse`

- Kind: struct
- Summary: -
- docs.rs: [AuctionsResponse](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.AuctionsResponse.html)
- Site rustdoc: [AuctionsResponse](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.AuctionsResponse.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `auctions` | `auctions` | `HashMap<String, Vec<DailyAuction>>` | - |
| `next_page_token` | `next_page_token` | `Option<String>` | - |
| `currency` | `currency` | `Option<Currency>` | - |

### `AuctionsSingleResponse`

- Kind: struct
- Summary: -
- docs.rs: [AuctionsSingleResponse](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.AuctionsSingleResponse.html)
- Site rustdoc: [AuctionsSingleResponse](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.AuctionsSingleResponse.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `symbol` | `symbol` | `String` | - |
| `auctions` | `auctions` | `Vec<DailyAuction>` | - |
| `next_page_token` | `next_page_token` | `Option<String>` | - |
| `currency` | `currency` | `Option<Currency>` | - |

### `BarsResponse`

- Kind: struct
- Summary: -
- docs.rs: [BarsResponse](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.BarsResponse.html)
- Site rustdoc: [BarsResponse](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.BarsResponse.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `bars` | `bars` | `HashMap<String, Vec<Bar>>` | - |
| `next_page_token` | `next_page_token` | `Option<String>` | - |
| `currency` | `currency` | `Option<Currency>` | - |

### `BarsSingleResponse`

- Kind: struct
- Summary: -
- docs.rs: [BarsSingleResponse](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.BarsSingleResponse.html)
- Site rustdoc: [BarsSingleResponse](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.BarsSingleResponse.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `symbol` | `symbol` | `String` | - |
| `bars` | `bars` | `Vec<Bar>` | - |
| `next_page_token` | `next_page_token` | `Option<String>` | - |
| `currency` | `currency` | `Option<Currency>` | - |

### `ConditionCodesResponse`

- Kind: type
- Summary: -
- docs.rs: [ConditionCodesResponse](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/type.ConditionCodesResponse.html)
- Site rustdoc: [ConditionCodesResponse](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/type.ConditionCodesResponse.html)
- Alias target: `HashMap<String, String>`

### `ExchangeCodesResponse`

- Kind: type
- Summary: -
- docs.rs: [ExchangeCodesResponse](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/type.ExchangeCodesResponse.html)
- Site rustdoc: [ExchangeCodesResponse](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/type.ExchangeCodesResponse.html)
- Alias target: `HashMap<String, String>`

### `LatestBarResponse`

- Kind: struct
- Summary: -
- docs.rs: [LatestBarResponse](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.LatestBarResponse.html)
- Site rustdoc: [LatestBarResponse](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.LatestBarResponse.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `symbol` | `symbol` | `String` | - |
| `bar` | `bar` | `Bar` | - |
| `currency` | `currency` | `Option<Currency>` | - |

### `LatestBarsResponse`

- Kind: struct
- Summary: -
- docs.rs: [LatestBarsResponse](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.LatestBarsResponse.html)
- Site rustdoc: [LatestBarsResponse](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.LatestBarsResponse.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `bars` | `bars` | `HashMap<String, Bar>` | - |
| `currency` | `currency` | `Option<Currency>` | - |

### `LatestQuoteResponse`

- Kind: struct
- Summary: -
- docs.rs: [LatestQuoteResponse](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.LatestQuoteResponse.html)
- Site rustdoc: [LatestQuoteResponse](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.LatestQuoteResponse.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `symbol` | `symbol` | `String` | - |
| `quote` | `quote` | `Quote` | - |
| `currency` | `currency` | `Option<Currency>` | - |

### `LatestQuotesResponse`

- Kind: struct
- Summary: -
- docs.rs: [LatestQuotesResponse](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.LatestQuotesResponse.html)
- Site rustdoc: [LatestQuotesResponse](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.LatestQuotesResponse.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `quotes` | `quotes` | `HashMap<String, Quote>` | - |
| `currency` | `currency` | `Option<Currency>` | - |

### `LatestTradeResponse`

- Kind: struct
- Summary: -
- docs.rs: [LatestTradeResponse](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.LatestTradeResponse.html)
- Site rustdoc: [LatestTradeResponse](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.LatestTradeResponse.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `symbol` | `symbol` | `String` | - |
| `trade` | `trade` | `Trade` | - |
| `currency` | `currency` | `Option<Currency>` | - |

### `LatestTradesResponse`

- Kind: struct
- Summary: -
- docs.rs: [LatestTradesResponse](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.LatestTradesResponse.html)
- Site rustdoc: [LatestTradesResponse](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.LatestTradesResponse.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `trades` | `trades` | `HashMap<String, Trade>` | - |
| `currency` | `currency` | `Option<Currency>` | - |

### `QuotesResponse`

- Kind: struct
- Summary: -
- docs.rs: [QuotesResponse](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.QuotesResponse.html)
- Site rustdoc: [QuotesResponse](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.QuotesResponse.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `quotes` | `quotes` | `HashMap<String, Vec<Quote>>` | - |
| `next_page_token` | `next_page_token` | `Option<String>` | - |
| `currency` | `currency` | `Option<Currency>` | - |

### `QuotesSingleResponse`

- Kind: struct
- Summary: -
- docs.rs: [QuotesSingleResponse](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.QuotesSingleResponse.html)
- Site rustdoc: [QuotesSingleResponse](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.QuotesSingleResponse.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `symbol` | `symbol` | `String` | - |
| `quotes` | `quotes` | `Vec<Quote>` | - |
| `next_page_token` | `next_page_token` | `Option<String>` | - |
| `currency` | `currency` | `Option<Currency>` | - |

### `SnapshotResponse`

- Kind: struct
- Summary: -
- docs.rs: [SnapshotResponse](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.SnapshotResponse.html)
- Site rustdoc: [SnapshotResponse](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.SnapshotResponse.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `symbol` | `symbol` | `String` | - |
| `currency` | `currency` | `Option<Currency>` | - |
| `latestTrade` | `latestTrade` | `Option<Trade>` | - |
| `latestQuote` | `latestQuote` | `Option<Quote>` | - |
| `minuteBar` | `minuteBar` | `Option<Bar>` | - |
| `dailyBar` | `dailyBar` | `Option<Bar>` | - |
| `prevDailyBar` | `prevDailyBar` | `Option<Bar>` | - |

### `SnapshotsResponse`

- Kind: type
- Summary: -
- docs.rs: [SnapshotsResponse](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/type.SnapshotsResponse.html)
- Site rustdoc: [SnapshotsResponse](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/type.SnapshotsResponse.html)
- Alias target: `HashMap<String, Snapshot>`

### `TradesResponse`

- Kind: struct
- Summary: -
- docs.rs: [TradesResponse](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.TradesResponse.html)
- Site rustdoc: [TradesResponse](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.TradesResponse.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `trades` | `trades` | `HashMap<String, Vec<Trade>>` | - |
| `next_page_token` | `next_page_token` | `Option<String>` | - |
| `currency` | `currency` | `Option<Currency>` | - |

### `TradesSingleResponse`

- Kind: struct
- Summary: -
- docs.rs: [TradesSingleResponse](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.TradesSingleResponse.html)
- Site rustdoc: [TradesSingleResponse](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.TradesSingleResponse.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `symbol` | `symbol` | `String` | - |
| `trades` | `trades` | `Vec<Trade>` | - |
| `next_page_token` | `next_page_token` | `Option<String>` | - |
| `currency` | `currency` | `Option<Currency>` | - |


## Models

### `Auction`

- Kind: struct
- Summary: -
- docs.rs: [Auction](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.Auction.html)
- Site rustdoc: [Auction](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.Auction.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `t` | `t` | `Option<Timestamp>` | - |
| `x` | `x` | `Option<String>` | - |
| `p` | `p` | `Option<Decimal>` | - |
| `s` | `s` | `Option<u64>` | - |
| `c` | `c` | `Option<String>` | - |

### `Bar`

- Kind: struct
- Summary: -
- docs.rs: [Bar](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.Bar.html)
- Site rustdoc: [Bar](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.Bar.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `t` | `t` | `Option<Timestamp>` | - |
| `o` | `o` | `Option<Decimal>` | - |
| `h` | `h` | `Option<Decimal>` | - |
| `l` | `l` | `Option<Decimal>` | - |
| `c` | `c` | `Option<Decimal>` | - |
| `v` | `v` | `Option<u64>` | - |
| `n` | `n` | `Option<u64>` | - |
| `vw` | `vw` | `Option<Decimal>` | - |

### `DailyAuction`

- Kind: struct
- Summary: -
- docs.rs: [DailyAuction](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.DailyAuction.html)
- Site rustdoc: [DailyAuction](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.DailyAuction.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `d` | `d` | `Option<String>` | - |
| `o` | `o` | `Vec<Auction>` | - |
| `c` | `c` | `Vec<Auction>` | - |

### `Quote`

- Kind: struct
- Summary: -
- docs.rs: [Quote](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.Quote.html)
- Site rustdoc: [Quote](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.Quote.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `t` | `t` | `Option<Timestamp>` | - |
| `bx` | `bx` | `Option<String>` | - |
| `bp` | `bp` | `Option<Decimal>` | - |
| `bs` | `bs` | `Option<u64>` | - |
| `ax` | `ax` | `Option<String>` | - |
| `ap` | `ap` | `Option<Decimal>` | - |
| `r#as` | `as` | `Option<u64>` | - |
| `c` | `c` | `Option<Vec<String>>` | - |
| `z` | `z` | `Option<String>` | - |

### `Snapshot`

- Kind: struct
- Summary: -
- docs.rs: [Snapshot](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.Snapshot.html)
- Site rustdoc: [Snapshot](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.Snapshot.html)

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
- docs.rs: [Trade](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.Trade.html)
- Site rustdoc: [Trade](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.Trade.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `t` | `t` | `Option<Timestamp>` | - |
| `x` | `x` | `Option<String>` | - |
| `p` | `p` | `Option<Decimal>` | - |
| `s` | `s` | `Option<u64>` | - |
| `i` | `i` | `Option<u64>` | - |
| `c` | `c` | `Option<Vec<String>>` | - |
| `z` | `z` | `Option<String>` | - |
| `u` | `u` | `Option<String>` | - |


## Enums

### `Adjustment`

- Kind: struct
- Summary: -
- docs.rs: [Adjustment](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.Adjustment.html)
- Site rustdoc: [Adjustment](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.Adjustment.html)
- Example constructors: `raw -> raw`, `split -> split`, `dividend -> dividend`, `spin_off -> spin-off`, `all -> all`

### `AuctionFeed`

- Kind: enum
- Summary: -
- docs.rs: [AuctionFeed](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/enum.AuctionFeed.html)
- Site rustdoc: [AuctionFeed](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/enum.AuctionFeed.html)

| Variant | Official Value |
| --- | --- |
| `Sip` | `sip` |

### `DataFeed`

- Kind: enum
- Summary: -
- docs.rs: [DataFeed](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/enum.DataFeed.html)
- Site rustdoc: [DataFeed](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/enum.DataFeed.html)

| Variant | Official Value |
| --- | --- |
| `DelayedSip` | `delayed_sip` |
| `Iex` | `iex` |
| `Otc` | `otc` |
| `Sip` | `sip` |
| `Boats` | `boats` |
| `Overnight` | `overnight` |

### `Tape`

- Kind: enum
- Summary: -
- docs.rs: [Tape](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/enum.Tape.html)
- Site rustdoc: [Tape](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/enum.Tape.html)

| Variant | Official Value |
| --- | --- |
| `A` | `A` |
| `B` | `B` |
| `C` | `C` |

### `TickType`

- Kind: enum
- Summary: -
- docs.rs: [TickType](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/enum.TickType.html)
- Site rustdoc: [TickType](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/enum.TickType.html)

| Variant | Official Value |
| --- | --- |
| `Trade` | `trade` |
| `Quote` | `quote` |

### `TimeFrame`

- Kind: struct
- Summary: -
- docs.rs: [TimeFrame](https://docs.rs/alpaca-data/latest/alpaca_data/stocks/struct.TimeFrame.html)
- Site rustdoc: [TimeFrame](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/stocks/struct.TimeFrame.html)
- Example constructors: `min_1 -> 1Min`, `day_1 -> 1Day`


## Related Repository Artifacts

- Examples: `examples/stocks_bars_all.rs`, `examples/stocks_latest_bar.rs`
- Tests: `tests/live_stocks_auctions.rs`, `tests/live_stocks_batch_historical.rs`, `tests/live_stocks_latest_snapshot.rs`, `tests/live_stocks_metadata.rs`, `tests/live_stocks_single_historical.rs`, `tests/mock_stocks_errors.rs`
- Benchmarks: `benches/stocks.rs`

## Coverage Notes

- The strict endpoint parity ledger for this module lives in [API Coverage](../api-coverage.md).
- Generated reference pages mirror the shipped Rust surface and do not claim unimplemented Alpaca endpoints as available.
